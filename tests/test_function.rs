use anyhow::Result;
use optpy::transpile;

#[test]
fn test_function() -> Result<()> {
    let code = r"
def my_function(a, b, c):
    return a + b + c

a, b, c = map(int, input().split())
print(my_function(a, b, c))
";
    let rust_code = transpile(code)?;
    let output = optpy_test_tools::execute(&rust_code, "1 2 3\n")?;
    assert_eq!(output, "6\n");

    Ok(())
}
