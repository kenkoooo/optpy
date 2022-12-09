use optpy_runtime::ToValue;
use optpy_test_macro::python_function;

#[test]
fn test_same_list_comprehension() {
    python_function! {r"
def test(a):
    x = [[] for _ in range(a)]
    y = [[] for _ in range(a)]
    return x, y"}

    assert_eq!(
        test(&2.to_value()),
        vec![
            vec![vec![].to_value(), vec![].to_value()].to_value(),
            vec![vec![].to_value(), vec![].to_value()].to_value()
        ]
        .to_value()
    )
}
