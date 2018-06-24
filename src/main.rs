extern crate clap;
extern crate csv;
#[macro_use]
extern crate error_chain;
extern crate sqlite;

mod csv_utils;
mod db_utils;
mod errors;
mod executor;

use std::fs::File;
use std::io;

use clap::{App, Arg, SubCommand};

use errors::{Result, ResultExt};
use executor::Executor;

const PROGRAM: &'static str = "csv_query";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn process() -> Result<()> {
    let matches = App::new(PROGRAM)
        .version(VERSION)
        .subcommand(
            SubCommand::with_name("run_query")
                .about("Run queries over CSV files")
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
                ),
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

    let delimiter = matches
        .value_of("delimiter")
        .and_then(|delimiter| {
            if delimiter.len() > 1 {
                bail!("Invalid delimiter {:?}", delimiter);
            }
            Ok(delimiter)
        })
        .and_then(|delimiter| Ok(delimiter.as_bytes()[0]));

    let mut executor = Executor::with_csv(
        input_buffers,
        io::BufWriter::new(io::stdout()),
        delimiter.unwrap(),
    )?;
    let query = matches.value_of("query").unwrap();
    executor.print_results(query)?;
    Ok(())
}

quick_main!(process);
