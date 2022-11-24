use optpy_std::typed_object::Object;
use optpy_test_macro::typed_python_function;

macro_rules! assert_typed_eq {
    ($left:expr, $right:expr $(,)?) => {
        assert_eq!(format!("{:?}", $left), format!("{:?}", $right))
    };
}

#[test]
fn test_if_statement() {
    typed_python_function! {r#"
def test(a, b):
    ans = a * b
    if ans % 2 == 0:
        return "Even"
    else:
        return "Odd"
    "#}

    let result = test(&Object::from(3), &Object::from(4));
    assert_typed_eq!(result, Object::from("Even"));

    let result = test(&Object::from(3), &Object::from(5));
    assert_typed_eq!(result, Object::from("Odd"));
}
