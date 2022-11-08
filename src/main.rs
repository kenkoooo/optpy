use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use optpy::compile;

#[derive(Parser, Debug)]
struct Args {
    /// Input Python file
    input: PathBuf,

    /// Path to output Rust file
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let code = read_to_string(&args.input)?;
    let result = compile(code)?;

    let output = match args.output.as_ref() {
        Some(output) => output.clone(),
        None => args.input.with_extension("rs"),
    };
    write(&output, result)?;
    log::info!("Generated {:?}", output);

    Ok(())
}
