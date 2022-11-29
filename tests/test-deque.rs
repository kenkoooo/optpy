use optpy_runtime::Value;
use optpy_test_macro::python_function;

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
#[test]
fn test_appendleft() {
    python_function! {r"
def test():
    import collections
    d = collections.deque()
    d.append(1)
    d.appendleft(2)
    return d.popleft()"}
    assert_eq!(test(), Value::from(2));
}

#[test]
fn test_bool() {
    python_function! {r"
def test(a):
    from collections import deque
    d = deque(a)
    if d:
        return 1
    else:
        return 2"}
    assert_eq!(test(&Value::from(vec![Value::from(0)])), Value::from(1));
    assert_eq!(test(&Value::from(vec![])), Value::from(2));
}
