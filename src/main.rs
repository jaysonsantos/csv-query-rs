extern crate clap;
extern crate csv;
#[macro_use]
extern crate error_chain;
extern crate sqlite;

mod db_utils;
mod errors;
mod executor;

use std::fs::File;
use std::io;

use clap::{App, Arg};

use errors::{Result, ResultExt};
use executor::Executor;

const PROGRAM: &'static str = "csv-query";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
        .get_matches();

    let input_buffers: Vec<io::BufReader<File>> = matches
        .values_of("files")
        .unwrap()
        .map(|f| File::open(f).chain_err(|| format!("Error opening file: {}", f)))
        .map(|f| io::BufReader::new(f.unwrap()))
        .collect();

    let mut executor = Executor::with_csv(input_buffers, io::BufWriter::new(io::stdout()))?;
    let query = matches.value_of("query").unwrap();
    executor.print_results(query)?;
    Ok(())
}

quick_main!(process);
