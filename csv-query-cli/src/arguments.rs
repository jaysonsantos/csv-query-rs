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
                .required_unless("output")
                .conflicts_with("output")
                .help("Query to run over CSV file(s)"),
        ).arg(
            Arg::with_name("interactive")
                .long("interactive")
                .short("i")
                .takes_value(false)
                .required_unless("query")
                .conflicts_with("query")
                .required_unless("output")
                .conflicts_with("output")
                .help("Open an interactive console to run and print out queries in CSV format"),
        ).arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .required_unless("query")
                .conflicts_with("query")
                .required_unless("interactive")
                .conflicts_with("interactive")
                .help("Dump the sqlite database into a fie"),
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
