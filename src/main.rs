extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate csv_query;

use std::fs::File;
use std::io;

use clap::{App, Arg};

use csv_query::errors::{Result, ResultExt};
use csv_query::Executor;

const PROGRAM: &str = "csv-query";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn process() -> Result<()> {
    let matches = App::new(PROGRAM)
        .version(VERSION)
        .arg(
            Arg::with_name("query")
                .long("query")
                .short("q")
                .takes_value(true)
                .required(true)
                .help("Query to run over CSV file(s)"),
        )
        .arg(
            Arg::with_name("files")
                .short("f")
                .long("files")
                .takes_value(true)
                .required(true)
                .multiple(true)
                .help("CSV files to work with"),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .takes_value(true)
                .default_value(";")
                .help("Delimiter used in your CSV"),
        )
        .arg(
            Arg::with_name("insert-batch-size")
                .short("b")
                .long("insert-batch-size")
                .takes_value(true)
                .default_value("1000")
                .help("How many line to buffer before inserting them into the database"),
        )
        .get_matches();

    let mut input_buffers: Vec<io::BufReader<File>> = vec![];

    for f in matches
        .values_of("files")
        .unwrap()
        .map(|f| File::open(f).chain_err(|| format!("Opening file: {:?}", f)))
    {
        input_buffers.push(io::BufReader::new(f?));
    }

    let delimiter = {
        let delimiter = matches.value_of("delimiter").unwrap();
        if delimiter.len() > 1 {
            bail!("Invalid delimiter {:?}", delimiter);
        }
        delimiter.as_bytes()[0]
    };

    let mut executor = Executor::new(
        input_buffers,
        io::BufWriter::new(io::stdout()),
        delimiter,
        matches
            .value_of("insert-batch-size")
            .unwrap()
            .parse()
            .chain_err(|| "Batch size is not a valid integer")?,
    )?;
    let query = matches.value_of("query").unwrap();
    executor.write_query_results(query)?;
    Ok(())
}

quick_main!(process);
