use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use optpy::compile;

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,

    #[clap(default_value = "./a.rs")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let code = read_to_string(args.input)?;
    let result = compile(code)?;

    write(args.output, result)?;
    Ok(())
}
