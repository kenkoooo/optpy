use optpy_runtime::Value;
use optpy_test_macro::python_function;

#[test]
fn test_heapify() {
    python_function! {r"
def test():
    import heapq
    a = [1, 6, 8, 0, -1]
    heapq.heapify(a)
    return a"}

    assert_eq!(
        test(),
        Value::from(vec![
            Value::from(-1),
            Value::from(0),
            Value::from(8),
            Value::from(1),
            Value::from(6)
        ])
    );
}
