use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use clap::Parser;
use optpy_generator::generate_code;
use optpy_parser::parse;
use optpy_resolver::resolve;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const STD: &str = include_str!("../optpy-std/src/lib.rs");

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,

    #[clap(default_value = "./a.rs")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let code = read_to_string(args.input)?;
    let ast = parse(code)?;
    let (ast, definitions) = resolve(&ast);
    let code = generate_code(&ast, &definitions);

    let mut result = STD.to_string();
    result += &code.to_string();

    write(args.output, result)?;
    Ok(())
}
