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

#[test]
fn test_heap_push_pop() {
    python_function! {r"
def test():
    from heapq import heapify, heappop, heappush
    a = [1, 4, 7]
    heapify(a)
    heappush(a, 2)
    heappush(a, 6)
    heappush(a, 5)
    heappush(a, 3)
    x = []
    for _ in range(7):
        x.append(heappop(a))
    return x"}

    assert_eq!(
        test(),
        Value::from(vec![
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from(4),
            Value::from(5),
            Value::from(6),
            Value::from(7)
        ]),
    );
}
