use optpy_runtime::Value;
use optpy_test_macro::python_function;

#[test]
fn test_same_list_comprehension() {
    python_function! {r"
def test(a):
    x = [[] for _ in range(a)]
    y = [[] for _ in range(a)]
    return x, y"}

    assert_eq!(
        test(&Value::from(2)),
        Value::from(vec![
            Value::from(vec![Value::from(vec![]), Value::from(vec![])]),
            Value::from(vec![Value::from(vec![]), Value::from(vec![])])
        ])
    )
}
