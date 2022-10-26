use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;

#[test]
fn test() -> Result<()> {
    let code = r"
def my_function(a, b, c):
    return a + b + c

a, b, c = map(int, input().split())
print(my_function(a, b, c))
";
    let code = optpy_compiler::compile_code(code)?;
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(code.to_string().as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let output = String::from_utf8(output.stdout.to_vec())?;
    eprintln!("{}", output);
    Ok(())
}
