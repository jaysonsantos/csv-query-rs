extern crate csv;
#[macro_use]
extern crate error_chain;
extern crate sqlite;

mod errors;
mod executor;

use executor::Executor;
use std::env;
use std::io;

fn process() -> errors::Result<()> {
    let executor = Executor::with_csv(io::stdin())?;
    executor.print_results(&env::args().nth(1).expect("Specify a query to run"))?;
    Ok(())
}

fn main() {
    process().unwrap();
}
