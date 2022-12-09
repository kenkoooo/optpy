use optpy_runtime::ToValue;
use optpy_test_macro::python_function;

#[test]
fn test_bool() {
    python_function! {r#"
def test(x):
    if x:
        return "OK"
    else:
        return "NO"
"#}

    assert_eq!(test(&1.to_value()), "OK".to_value());
    assert_eq!(test(&0.to_value()), "NO".to_value());
    assert_eq!(test(&(-1).to_value()), "OK".to_value());
    assert_eq!(test(&0.1.to_value()), "OK".to_value());
    assert_eq!(test(&0.0.to_value()), "NO".to_value());
    assert_eq!(test(&(-0.1).to_value()), "OK".to_value());
}
