use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use optpy::{compile, typed_compile};
use optpy_dump::DumpPython;
use optpy_parser::parse;
use optpy_resolver::resolve;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate a Rust file from a Python file
    Compile {
        /// Input Python file
        input: PathBuf,

        /// Path to output Rust file
        output: Option<PathBuf>,
    },
    /// Dump internal Python statements
    Dump {
        /// Input Python file
        input: PathBuf,
    },
    /// Compile with type inference (experimental)
    Typed {
        /// Input Python file
        input: PathBuf,

        /// Path to output Rust file
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Command::Compile { input, output } => {
            let code = read_to_string(&input)?;
            let result = compile(code)?;

            let output = match output {
                Some(output) => output,
                None => input.with_extension("rs"),
            };
            write(&output, result)?;
            log::info!("Generated {:?}", output);
        }
        Command::Dump { input } => {
            let code = read_to_string(&input)?;
            let ast = parse(code)?;
            let (ast, _) = resolve(&ast);
            let python_code = ast.to_python_code();
            println!("{}", python_code);
        }
        Command::Typed { input, output } => {
            let code = read_to_string(&input)?;
            let result = typed_compile(code)?;

            let output = match output {
                Some(output) => output,
                None => input.with_extension("rs"),
            };
            write(&output, result)?;
            log::info!("Generated {:?}", output);
        }
    }

    Ok(())
}
