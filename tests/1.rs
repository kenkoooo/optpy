use anyhow::Result;
use optpy::transpile;

#[test]
fn test_1() -> Result<()> {
    let python_code = r"
a = int(input())
b = int(input())
answer = b - a
print(answer)
";
    let rust_code = transpile(python_code)?;
    let output = optpy_test_tools::execute(&rust_code, "150\n155\n")?;
    assert_eq!(output, "5\n");

    Ok(())
}

#[test]
fn test_2() -> Result<()> {
    let python_code = r"
a, b, c = map(int, input().split())
if a <= c < b:
    answer = 1
else:
    answer = 0
print(answer)
";
    let rust_code = transpile(python_code)?;
    let output = optpy_test_tools::execute(&rust_code, "10 15 12\n")?;
    assert_eq!(output, "1\n");

    Ok(())
}
