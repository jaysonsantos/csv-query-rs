use std::fs::File;
use std::io;

use error_chain::bail;

use csv_query::errors::{Result, ResultExt};
use csv_query::Executor;

use crate::arguments;
use crate::interactive::run_interactive;

pub(crate) fn process() -> Result<()> {
    let matches = arguments::build().get_matches();
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
    } else if matches.is_present("interactive") {
        run_interactive(executor)?;
    } else if let Some(output_file) = matches.value_of("output") {
        executor.dump_database(output_file)?;
    }
    Ok(())
}
