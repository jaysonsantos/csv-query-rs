extern crate csv;
#[macro_use]
extern crate error_chain;
extern crate sqlite;

mod db_utils;
mod errors;
mod executor;

use executor::Executor;
use std::env;
use std::io;

fn process() -> errors::Result<()> {
    eprintln!("Reading data from stdin");
    let mut executor = Executor::with_csv(
        io::BufReader::new(io::stdin()),
        io::BufWriter::new(io::stdout()),
    )?;
    executor.print_results(&env::args().nth(1).expect("Specify a query to run"))?;
    Ok(())
}

fn main() {
    process().expect("Error running processor");
}
