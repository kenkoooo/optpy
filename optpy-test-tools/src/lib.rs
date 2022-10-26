use std::{
    fs::{create_dir_all, write},
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Result};
use tempfile::{tempdir, TempDir};

fn create_tmp_cargo_workspace(main: &str) -> Result<TempDir> {
    const OPTPY_STD_CARGO_TOML: &str = include_str!("../../optpy-std/Cargo.toml");

    let dir = tempdir()?;
    create_dir_all(dir.path().join("src"))?;
    write(dir.path().join("src").join("main.rs"), main)?;
    write(dir.path().join("Cargo.toml"), OPTPY_STD_CARGO_TOML)?;

    Ok(dir)
}

pub fn execute(main: &str, input: &str) -> Result<String> {
    let cargo = create_tmp_cargo_workspace(main)?;
    let fmt = Command::new("cargo").arg("fmt").status()?;
    if !fmt.success() {
        return Err(anyhow!("failed to format the code"));
    }

    let mut child = Command::new("cargo")
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .current_dir(cargo.path())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let output = String::from_utf8(output.stdout.to_vec())?;
    Ok(output)
}
