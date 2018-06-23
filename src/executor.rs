use std::io::{BufRead, Read, Write};

use csv;
use sqlite;

use db_utils::{escape_columns, escape_values};
use errors::{Result, ResultExt};

pub struct Executor<W: Write> {
    // columns: Vec<String>,
    conn: sqlite::Connection,
    output: W,
}

impl<W> Executor<W>
where
    W: Write,
{
    pub fn with_csv<R>(readers: Vec<R>, output: W) -> Result<Executor<W>>
    where
        R: BufRead,
    {
        let conn = Self::create_database()?;
        for (i, reader) in readers.into_iter().enumerate() {
            let table_number = i + 1;
            let mut csv_readr = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_reader(reader);

            let columns = {
                csv_readr
                    .headers()
                    .chain_err(|| "Error reading headers")?
                    .clone()
            };
            Self::create_table(&conn, &columns, table_number)?;
            Self::fill_data(&conn, &columns, table_number, csv_readr)?;
        }
        Ok(Executor { conn, output })
    }

    fn create_database() -> Result<sqlite::Connection> {
        Ok(sqlite::open(":memory:").chain_err(|| "Error opening memory database.")?)
    }

    fn create_table(
        conn: &sqlite::Connection,
        columns: &csv::StringRecord,
        table_number: usize,
    ) -> Result<()> {
        let quoted_columns: Vec<String> = columns
            .iter()
            .map(|c| format!("\"{}\" VARCHAR NULL", c))
            .collect();
        let create_query = format!(
            "CREATE TABLE table{} ({})",
            table_number,
            quoted_columns.join(", ")
        );
        conn.execute(&create_query)
            .chain_err(|| format!("Error creating the database. Used query {}", create_query))?;
        Ok(())
    }

    fn fill_data<R>(
        conn: &sqlite::Connection,
        columns: &csv::StringRecord,
        table_number: usize,
        mut reader: csv::Reader<R>,
    ) -> Result<()>
    where
        R: Read,
    {
        let quoted_columns = escape_columns(columns);
        let insert = format!(
            "INSERT INTO table{} ({}) VALUES\n",
            table_number,
            quoted_columns.join(", ")
        );
        let mut rows: Vec<String> = vec![];
        for row in reader.records() {
            let row = row.chain_err(|| "Error reading row")?;
            let db_row = escape_values(&row);
            rows.push(format!("({})", db_row.join(", ")));
        }
        let final_query = format!("{}{}", insert, rows.join(",\n"));
        conn.execute(&final_query)
            .chain_err(|| "Error running insert query.")?;
        Ok(())
    }

    pub fn print_results(&mut self, query: &str) -> Result<()> {
        let prepared = self
            .conn
            .prepare(query)
            .chain_err(|| format!("Error preparing query: {}", query))?;
        let output_error = "Error writing on selected output";
        writeln!(
            self.output,
            "{}",
            &prepared
                .column_names()
                .iter()
                .map(|c| format!("\"{}\"", c))
                .collect::<Vec<String>>()
                .join(";")
        ).chain_err(|| output_error)?;
        let mut cursor = prepared.cursor();
        while let Some(row) = cursor.next().chain_err(|| "Error reading results")? {
            writeln!(
                self.output,
                "{}",
                row.iter()
                    .map(|e| format!(
                        "\"{}\"",
                        match e.kind() {
                            sqlite::Type::Float => e.as_float().unwrap().to_string(),
                            sqlite::Type::String => e.as_string().unwrap().to_owned(),
                            sqlite::Type::Integer => e.as_integer().unwrap().to_string(),
                            sqlite::Type::Null => "".to_owned(),
                            _ => "Cannot parse binary".to_owned(),
                        }
                    ))
                    .collect::<Vec<String>>()
                    .join(";")
            ).chain_err(|| output_error)?;
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_nothing() {
//         let input = BufReader::new();
//         let mut output = BufWriter::new();
//         let mut executor = Executor::with_csv(reader, output);
//     }
// }
