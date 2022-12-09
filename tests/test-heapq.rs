use optpy_runtime::ToValue;
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
        vec![
            (-1).to_value(),
            0.to_value(),
            8.to_value(),
            1.to_value(),
            6.to_value()
        ]
        .to_value()
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
        vec![
            1.to_value(),
            2.to_value(),
            3.to_value(),
            4.to_value(),
            5.to_value(),
            6.to_value(),
            7.to_value()
        ]
        .to_value(),
    );
}
