use std::{
    fs::write,
    io::{Read, Write},
    process::{Command, Stdio},
};

use anyhow::Result;
use tempfile::tempdir;

use optpy::compile;

pub fn execute(code: &str, input: &str) -> Result<String> {
    let dir = tempdir()?;
    let file = dir.path().join("a.rs");
    let path = dir.path().join("a");
    let code = compile(code)?;

    write(&file, code)?;

    Command::new("rustfmt").args([&file]).output()?;
    let output = Command::new("rustc")
        .arg("-o")
        .arg(&path)
        .arg(&file)
        .output()?;

    assert!(
        path.exists(),
        "{}",
        String::from_utf8(output.stderr).unwrap()
    );
    let process = Command::new(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    process.stdin.unwrap().write_all(input.as_bytes())?;

    let mut output = String::new();
    process.stdout.unwrap().read_to_string(&mut output)?;

    Ok(output)
}
