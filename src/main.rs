extern crate clap;
extern crate directories;
#[macro_use]
extern crate error_chain;
extern crate rustyline;

extern crate csv_query;

use std::fs::{create_dir_all, File};
use std::io::{self, Write};

use clap::{App, Arg};

use directories::ProjectDirs;

use error_chain::ChainedError;

use rustyline::error::ReadlineError;
use rustyline::Editor;

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
                .required_unless("interactive")
                .conflicts_with("interactive")
                .help("Query to run over CSV file(s)"),
        )
        .arg(
            Arg::with_name("interactive")
                .long("interactive")
                .short("i")
                .takes_value(false)
                .required_unless("query")
                .conflicts_with("query")
                .help("Open an interactive console to run and print out queries in CSV format"),
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
    if let Some(query) = matches.value_of("query") {
        executor.write_query_results(query)?;
    }
    if matches.is_present("interactive") {
        run_interactive(executor)?;
    }
    Ok(())
}

fn run_interactive<W>(mut executor: Executor<W>) -> Result<()>
where
    W: Write,
{
    let history_file = ProjectDirs::from("io.github", "jaysonsantos", "csv-query").and_then(|p| {
        let mut p = p.data_dir().to_path_buf();
        p.push("history.txt");
        Some(p)
    });
    let mut rl = Editor::<()>::new();
    if let Some(ref history_file) = history_file.as_ref() {
        eprintln!("Loading history from {:?}", history_file,);
        if rl.load_history(&history_file).is_err() {
            eprintln!("No previous history.");
        }
    } else {
        eprintln!("Could not find data dir");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(query) => {
                if query.trim().is_empty() {
                    continue;
                }
                if let Err(e) = executor.write_query_results(&query) {
                    eprintln!("Error running query {}", e.display_chain());
                    continue;
                }
                rl.add_history_entry(query.as_ref());
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            e @ Err(_) => {
                e.chain_err(|| "Error running")?;
            }
        }
    }
    if let Some(ref history_file) = history_file.as_ref() {
        create_dir_all(history_file.parent().unwrap())
            .chain_err(|| "Failed to create data directory")?;
        rl.save_history(&history_file)
            .chain_err(|| "Error writing history file")?;
    }
    Ok(())
}

quick_main!(process);
