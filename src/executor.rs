use std::io::Read;

use csv;
use errors;
use sqlite;

pub struct Executor {
    columns: Vec<String>,
    conn: sqlite::Connection,
}

impl Executor {
    pub fn with_csv<R>(reader: R) -> errors::Result<Executor>
    where
        R: Read,
    {
        let mut csv_readr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(reader);

        let columns = csv_readr
            .headers()
            .unwrap()
            .iter()
            .map(|e| e.into())
            .collect();
        let conn = Self::create_database(&columns)?;
        Self::fill_data(&conn, &columns, csv_readr)?;
        Ok(Executor { columns, conn })
    }

    fn create_database(columns: &Vec<String>) -> errors::Result<sqlite::Connection> {
        let conn = sqlite::open(":memory:").unwrap();
        let quoted_columns: Vec<String> = columns
            .iter()
            .map(|c| format!("\"{}\" VARCHAR NULL", c))
            .collect();
        let create_query = format!("CREATE TABLE table1 ({})", quoted_columns.join(", "));
        conn.execute(create_query).unwrap();
        Ok(conn)
    }

    fn fill_data<R>(
        conn: &sqlite::Connection,
        columns: &Vec<String>,
        mut reader: csv::Reader<R>,
    ) -> errors::Result<()>
    where
        R: Read,
    {
        let quoted_columns: Vec<String> = columns.iter().map(|c| format!("\"{}\"", c)).collect();
        let insert = format!(
            "INSERT INTO table1 ({}) VALUES\n",
            quoted_columns.join(", ")
        );
        let mut rows: Vec<String> = vec![];
        for row in reader.records() {
            let row = row.unwrap();
            let db_row: Vec<String> = row
                .iter()
                .map(|c| c.replace("'", "''"))
                .map(|c| format!("'{}'", c))
                .collect();
            rows.push(format!("({})", db_row.join(", ")));
        }
        let final_query = format!("{}{}", insert, rows.join(",\n"));
        conn.execute(final_query).unwrap();
        Ok(())
    }

    pub fn print_results(self, query: &str) -> errors::Result<()> {
        let mut cursor = self.conn.prepare(query).unwrap().cursor();

        while let Some(row) = cursor.next().unwrap() {
            println!("{:?}", row);
        }
        Ok(())
    }
}
