use optpy_runtime::ToValue;
use optpy_test_macro::python_function;

#[test]
fn test_gcd() {
    python_function! {r#"
def test(a, b):
    import math
    return math.gcd(a, b)"#}

    assert_eq!(test(&(10.to_value()), &15.to_value(),), 5.to_value());
}

#[test]
fn test_sys_setrecursionlimit() {
    python_function! {r"
def test():
    import sys
    sys.setrecursionlimit(1)
    return"}
    test();
}
