use optpy_std::typed_object::Object;
use optpy_test_macro::typed_python_function;

macro_rules! assert_typed_eq {
    ($left:expr, $right:expr $(,)?) => {
        assert_eq!(format!("{:?}", $left), format!("{:?}", $right))
    };
}

#[test]
fn test_sorted() {
    typed_python_function! {r"
def test():
    x = [2, 1]
    x = sorted(x)
    return x"}
    assert_typed_eq!(test(), Object::from(vec![Object::from(1), Object::from(2)]));
}
