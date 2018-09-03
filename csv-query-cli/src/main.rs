extern crate clap;
extern crate directories;
#[macro_use]
extern crate error_chain;
extern crate rustyline;

extern crate csv_query;

mod arguments;
mod interactive;
mod process;

const PROGRAM: &str = "csv-query";
const VERSION: &str = env!("CARGO_PKG_VERSION");

quick_main!(process::process);
