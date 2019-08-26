use std::fs::create_dir_all;
use std::io::Write;

use directories::ProjectDirs;

use error_chain::ChainedError;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use csv_query::errors::{Result, ResultExt};
use csv_query::Executor;

pub(crate) fn run_interactive<W>(mut executor: Executor<W>) -> Result<()>
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
                // Always add to history so user can fix mistakes
                rl.add_history_entry(&query);
                if let Err(e) = executor.write_query_results(&query) {
                    eprintln!("Error running query {}", e.display_chain());
                }
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
