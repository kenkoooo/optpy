use optpy_std::Value;
use optpy_test_macro::python_function;

#[test]
fn test_map_int() {
    python_function! {r"
def test(s):
    a = map(int, s.split())
    return a
"}

    assert_eq!(
        test(&Value::from("1 2")),
        Value::from(vec![Value::from(1), Value::from(2)])
    );
}
#[test]
fn test_ops() {
    python_function! {
        r"
def test_ops(N, M):
    return [N + M, N * M, N - M, N / M, N // M]"
    };
    let result = test_ops(&Value::from(4), &Value::from(2));
    assert_eq!(
        result,
        Value::from(vec![
            Value::from(6),
            Value::from(8),
            Value::from(2),
            Value::from(2),
            Value::from(2),
        ])
    );

    let result = test_ops(&Value::from(1), &Value::from(2));
    assert_eq!(
        result,
        Value::from(vec![
            Value::from(3),
            Value::from(2),
            Value::from(-1),
            Value::from(0.5),
            Value::from(0),
        ])
    );
}

#[test]
fn test_unary_ops() {
    python_function! {r"
def test(a):
    return [+a, -a]"
    }

    assert_eq!(
        test(&Value::from(4)),
        Value::from(vec![Value::from(4), Value::from(-4)])
    )
}
