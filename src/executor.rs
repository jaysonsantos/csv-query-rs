use std::io::{BufRead, Read, Write};

use csv;
use rusqlite;

use csv_utils::string_to_csv_output;
use db_utils::{escape_columns, escape_values, AllString};
use errors::{Result, ResultExt};

pub struct Executor<W: Write> {
    // columns: Vec<String>,
    conn: rusqlite::Connection,
    output: W,
    delimiter: u8,
}

impl<W> Executor<W>
where
    W: Write,
{
    pub fn new<R>(readers: Vec<R>, output: W, delimiter: u8) -> Result<Executor<W>>
    where
        R: BufRead,
    {
        let conn = Self::create_database()?;
        Self::process_csv_files(readers, delimiter, &conn)?;
        Ok(Executor {
            conn,
            output,
            delimiter,
        })
    }

    fn create_database() -> Result<rusqlite::Connection> {
        Ok(rusqlite::Connection::open_in_memory().chain_err(|| "Opening memory database.")?)
    }

    fn process_csv_files<R>(
        readers: Vec<R>,
        delimiter: u8,
        conn: &rusqlite::Connection,
    ) -> Result<()>
    where
        R: Read,
    {
        for (i, reader) in readers.into_iter().enumerate() {
            let table_number = i + 1;
            let mut csv_reader = csv::ReaderBuilder::new()
                .delimiter(delimiter)
                .from_reader(reader);

            let columns = Self::get_csv_columns(&mut csv_reader)?;
            Self::create_table(&conn, &columns, table_number)?;
            Self::fill_data(&conn, &columns, table_number, csv_reader)?;
        }
        Ok(())
    }

    fn get_csv_columns<R>(csv_reader: &mut csv::Reader<R>) -> Result<csv::StringRecord>
    where
        R: Read,
    {
        Ok(csv_reader
            .headers()
            .chain_err(|| "Reading headers")?
            .clone())
    }

    fn create_table(
        conn: &rusqlite::Connection,
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
        conn.execute(&create_query, &[])
            .chain_err(|| format!("Error creating the database. Used query {}", create_query))?;
        Ok(())
    }

    fn fill_data<R>(
        conn: &rusqlite::Connection,
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
        conn.execute(&final_query, &[])
            .chain_err(|| "Error running insert query.")?;
        Ok(())
    }

    fn delimiter_to_string(&self) -> String {
        let mut delimiter = String::new();
        delimiter.push(self.delimiter as char);
        delimiter
    }

    pub fn print_results(&mut self, query: &str) -> Result<()> {
        let mut prepared = self
            .conn
            .prepare(query)
            .chain_err(|| format!("Error preparing query: {}", query))?;
        let delimiter = self.delimiter_to_string();
        let output_error = "Error writing on selected output";
        writeln!(
            self.output,
            "{}",
            &prepared
                .column_names()
                .iter()
                .map(|c| format!("\"{}\"", c))
                .collect::<Vec<String>>()
                .join(&delimiter)
        ).chain_err(|| output_error)?;
        let mut rows = prepared
            .query(&[])
            .chain_err(|| "Error binding parameters")?;
        while let Some(row) = rows.next() {
            let row = row.chain_err(|| "Error reading results")?;
            writeln!(
                self.output,
                "{}",
                (0..row.column_count())
                    .map(|r| row.get::<i32, AllString>(r).into())
                    .map(string_to_csv_output)
                    .collect::<Vec<String>>()
                    .join(&delimiter)
            ).chain_err(|| output_error)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Executor;
    use std::io::{BufReader, Cursor, Write};
    const USER_CSV: &'static str = "\"user\";\"age\"\n\
                                    \"user1\";\"10\"\n\
                                    \"user2\";\"4\"\n";
    const ORDER_CSV: &'static str = "user;price\n\
                                     user1;10\n\
                                     user2;30\n\
                                     user1;50\n";

    #[test]
    fn test_nothing() {
        let input = vec![BufReader::new(Cursor::new(&USER_CSV))];
        let output = vec![];
        let mut output_buffer = Cursor::new(output);
        {
            let buf = output_buffer.by_ref();
            let mut executor = Executor::new(input, buf, b';').unwrap();
            executor
                .print_results("select user, age from table1")
                .unwrap();
        }
        let output = output_buffer.into_inner();
        assert_eq!(String::from_utf8(output).unwrap(), USER_CSV);
    }

    #[test]
    fn test_join() {
        let input = vec![
            BufReader::new(Cursor::new(&USER_CSV)),
            BufReader::new(Cursor::new(&ORDER_CSV)),
        ];
        let output = vec![];
        let mut output_buffer = Cursor::new(output);
        {
            let buf = output_buffer.by_ref();
            let mut executor = Executor::new(input, buf, b';').unwrap();
            executor
                .print_results(
                    "select u.user, sum(price)
                    from table1 u
                    join table2 o
                        on u.user = o.user
                    group by u.user
                    having sum(price) > 50",
                )
                .unwrap();
        }
        let output = output_buffer.into_inner();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "\"user\";\"sum(price)\"\n\
             \"user1\";\"60\"\n"
        );
    }
}
