use std::{env::args, fs::read_to_string};

use anyhow::{anyhow, Result};
use optpy::transpile;

fn main() -> Result<()> {
    let path = args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow!("Please specify the Python file"))?;
    let content = read_to_string(path)?;
    let rust = transpile(&content)?;
    println!("{}", rust);
    Ok(())
}
