use optpy_runtime::Value;
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

    assert_eq!(test(Value::from(1)), Value::from("OK"));
    assert_eq!(test(Value::from(0)), Value::from("NO"));
    assert_eq!(test(Value::from(-1)), Value::from("OK"));
    assert_eq!(test(Value::from(0.1)), Value::from("OK"));
    assert_eq!(test(Value::from(0.0)), Value::from("NO"));
    assert_eq!(test(Value::from(-0.1)), Value::from("OK"));
}

#[test]
fn test_shift() {
    python_function! {r"
def test():
    return [1 << 2, 4 >> 1]"
    }

    assert_eq!(test(), Value::from(vec![Value::from(4), Value::from(2)]))
}
