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

#[test]
fn test_deque() {
    {
        python_function! {r"
def test():
    from collections import deque
    x = deque()
    x.append(1)
    x.append(2)
    return [x.popleft(), x.popleft()]
    "}
        assert_eq!(test(), Value::from(vec![Value::from(1), Value::from(2)]))
    }
    {
        python_function! {r"
def test():
    import collections
    x = collections.deque([1, 2])
    return [x.popleft(), x.popleft()]
    "}
        assert_eq!(test(), Value::from(vec![Value::from(1), Value::from(2)]))
    }
}
