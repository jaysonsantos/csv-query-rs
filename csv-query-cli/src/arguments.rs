use super::{PROGRAM, VERSION};

use clap::{App, Arg};

pub(crate) fn build<'a, 'b>() -> App<'a, 'b> {
    App::new(PROGRAM)
        .version(VERSION)
        .arg(
            Arg::with_name("query")
                .long("query")
                .short("q")
                .takes_value(true)
                .required_unless("interactive")
                .conflicts_with("interactive")
                .help("Query to run over CSV file(s)"),
        ).arg(
            Arg::with_name("interactive")
                .long("interactive")
                .short("i")
                .takes_value(false)
                .required_unless("query")
                .conflicts_with("query")
                .help("Open an interactive console to run and print out queries in CSV format"),
        ).arg(
            Arg::with_name("files")
                .short("f")
                .long("files")
                .takes_value(true)
                .required(true)
                .multiple(true)
                .help("CSV files to work with"),
        ).arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .takes_value(true)
                .default_value(";")
                .help("Delimiter used in your CSV"),
        ).arg(
            Arg::with_name("insert-batch-size")
                .short("b")
                .long("insert-batch-size")
                .takes_value(true)
                .default_value("1000")
                .help("How many line to buffer before inserting them into the database"),
        )
}
