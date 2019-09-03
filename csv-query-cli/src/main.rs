use error_chain::quick_main;

mod arguments;
mod interactive;
mod process;

const PROGRAM: &str = "csv-query";
const VERSION: &str = env!("CARGO_PKG_VERSION");

quick_main!(process::process);
