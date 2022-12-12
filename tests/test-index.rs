use optpy_runtime::Value;
use optpy_test_macro::python_function;

#[test]
fn test_list_index() {
    python_function! {r"
def test(x):
    a = [1, 2, 3]
    return a.index(x)"}

    assert_eq!(test(Value::from(1)), Value::from(0));
    assert_eq!(test(Value::from(3)), Value::from(2));
}
