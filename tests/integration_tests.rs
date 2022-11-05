mod test_env;

#[macro_export]
macro_rules! optpy_integration_test {
    ($name:ident, $code:expr, $(($input:expr, $output:expr)),+) => {
        #[test]
        fn $name() {
            let code = $code;
            let tests = [$(($input, $output)),+];
            for (input, output) in tests {
                assert_eq!(test_env::execute(code, input).unwrap(), output);
            }
        }
    };
}

optpy_integration_test! {
test_if_statement,
r#"
a, b = map(int, input().split())
ans = a * b
if ans % 2 == 0:
    print("Even")
else:
    print("Odd")
"#,
("3 4\n", "Even\n"),
("3 5\n", "Odd\n")
}
