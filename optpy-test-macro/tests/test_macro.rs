use optpy_std::Value;
use optpy_test_macro::test_python;

#[test]
fn test() {
    let result = test_python!(
        r"
a = []
return a
"
    );
    assert!(matches!(result, Value::List(_)));
}
