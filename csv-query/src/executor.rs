use std::io::{BufRead, Read, Write};
use std::path::Path;

use csv;
use rusqlite;

use csv_utils::string_to_csv_output;
use db_utils::{escape_columns, escape_values, AllString};
use errors::{Result, ResultExt};

/// Main struct that parses the CSV and put the data into a SQLite
pub struct Executor<W: Write> {
    conn: rusqlite::Connection,
    output: W,
    delimiter: u8,
}

impl<W> Executor<W>
where
    W: Write,
{
    pub fn new<R>(
        readers: Vec<R>,
        output: W,
        delimiter: u8,
        batch_insert_number: usize,
    ) -> Result<Executor<W>>
    where
        R: BufRead,
    {
        let conn = Self::create_database()?;
        Self::process_csv_files(readers, delimiter, batch_insert_number, &conn)?;
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
        batch_insert_number: usize,
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
            Self::fill_data(
                &conn,
                &columns,
                table_number,
                batch_insert_number,
                csv_reader,
            )?;
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
        batch_insert_number: usize,
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
        for (i, row) in reader.records().enumerate() {
            let row = row.chain_err(|| "Error reading row")?;
            let db_row = escape_values(&row);
            rows.push(format!("({})", db_row.join(", ")));
            if i % batch_insert_number == 0 {
                Self::batch_insert(&conn, &insert, &mut rows)?;
            }
        }
        Self::batch_insert(&conn, &insert, &mut rows)?;
        Ok(())
    }

    /// Consume rows vector and write them into sqlite
    fn batch_insert(
        conn: &rusqlite::Connection,
        insert: &str,
        rows: &mut Vec<String>,
    ) -> Result<()> {
        let mut batch = insert.to_owned();
        batch.push_str(&rows.join(",\n"));
        conn.execute(&batch, &[])
            .chain_err(|| "Error running insert query.")?;
        rows.clear();
        Ok(())
    }

    fn delimiter_to_string(&self) -> String {
        let mut delimiter = String::new();
        delimiter.push(self.delimiter as char);
        delimiter
    }

    /// Run the query and write its result as CSV into the specified output stream
    pub fn write_query_results(&mut self, query: &str) -> Result<()> {
        let delimiter = self.delimiter_to_string();
        let mut prepared = Self::prepare_query(&self.conn, query)?;
        let output_error = "Error writing on selected output";
        Self::write_headers(&prepared, &mut self.output, &output_error, &delimiter)?;
        let mut rows = prepared
            .query(&[])
            .chain_err(|| "Error binding parameters")?;
        Self::write_rows(&mut rows, &mut self.output, &output_error, &delimiter)?;
        self.output.flush().chain_err(|| "Error writing results")?;
        Ok(())
    }

    fn prepare_query<'a>(
        conn: &'a rusqlite::Connection,
        query: &str,
    ) -> Result<rusqlite::Statement<'a>> {
        Ok(conn
            .prepare(query)
            .chain_err(|| format!("Error preparing query: {}", query))?)
    }

    fn write_headers(
        prepared: &rusqlite::Statement,
        output: &mut W,
        output_error: &str,
        delimiter: &str,
    ) -> Result<()> {
        let columns_names = prepared
            .column_names()
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect::<Vec<String>>()
            .join(&delimiter);
        writeln!(output, "{}", columns_names).chain_err(|| output_error)?;
        Ok(())
    }

    fn write_rows(
        rows: &mut rusqlite::Rows,
        output: &mut W,
        output_error: &str,
        delimiter: &str,
    ) -> Result<()> {
        while let Some(row) = rows.next() {
            let row = row.chain_err(|| "Error reading results")?;
            let output_rows = (0..row.column_count())
                .map(|r| row.get::<usize, AllString>(r).into())
                .map(|r| string_to_csv_output(&r))
                .collect::<Vec<String>>()
                .join(&delimiter);
            writeln!(output, "{}", output_rows).chain_err(|| output_error)?;
        }

        Ok(())
    }

    pub fn dump_database<P>(&self, output: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.conn
            .backup(rusqlite::DatabaseName::Main, output, None)
            .chain_err(|| "Failed to dump database")?;
        Ok(())
    }
}
