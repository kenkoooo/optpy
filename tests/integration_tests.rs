use anyhow::Result;
use optpy::transpile;

#[test]
#[ignore]
fn test_abc274_b() -> Result<()> {
    let code = include_str!("./integration_tests/abc274_b/main.py");
    let input = include_str!("./integration_tests/abc274_b/1.in");
    let expected = include_str!("./integration_tests/abc274_b/1.out");

    let rust_code = transpile(code)?;
    let output = optpy_test_tools::execute(&rust_code, input)?;
    assert_eq!(output, expected);

    Ok(())
}
