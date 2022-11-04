mod test_env;
use test_env::execute;

#[test]
fn test_if_integration_test() -> anyhow::Result<()> {
    let code = include_str!("./assets/if.py");

    assert_eq!(execute(code, "3 4\n")?, "Even\n");
    assert_eq!(execute(code, "3 5\n")?, "Odd\n");

    Ok(())
}
