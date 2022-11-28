use optpy_runtime::Value;
use optpy_test_macro::python_function;

#[test]
fn test_gcd() {
    python_function! {r#"
def test(a, b):
    import math
    return math.gcd(a, b)"#}

    assert_eq!(test(&Value::from(10), &Value::from(15),), Value::from(5));
}
