use optpy_std::Object;
use optpy_test_macro::python_function;

#[test]
fn test_sorted() {
    python_function! {r"
def test():
    x = [2, 1]
    x = sorted(x)
    return x"}

    assert_eq!(test(), Object::from(vec![Object::from(1), Object::from(2)]));
}

#[test]
fn test_map_int() {
    python_function! {r"
def test(s):
    a = map(int, s.split())
    return a
"}

    assert_eq!(
        test(&Object::from("1 2")),
        Object::from(vec![Object::from(1), Object::from(2)])
    );
}

#[test]
fn test_split() {
    python_function! {r#"
def test():
    x = "abc efg"
    return x.split()"#}

    assert_eq!(
        test(),
        Object::from(vec![Object::from("abc"), Object::from("efg")])
    );
}

#[test]
fn test_assign() {
    python_function! {r"
def test():
    x = [0]
    x[0] = x[0]
    return x[0]"}

    assert_eq!(test(), Object::from(0));

    python_function! {r"
def test2():
    x = [1]
    x[0] = 2
    return x"}

    assert_eq!(test2(), Object::from(vec![Object::from(2)]));
}

#[test]
fn test_ops() {
    python_function! {
        r"
def test_ops(N, M):
    return [N + M, N * M, N - M, N / M, N // M]"
    };
    let result = test_ops(&Object::from(4), &Object::from(2));
    assert_eq!(
        result,
        Object::from(vec![
            Object::from(6),
            Object::from(8),
            Object::from(2),
            Object::from(2),
            Object::from(2),
        ])
    );

    let result = test_ops(&Object::from(1), &Object::from(2));
    assert_eq!(
        result,
        Object::from(vec![
            Object::from(3),
            Object::from(2),
            Object::from(-1),
            Object::from(0.5),
            Object::from(0),
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
        test(&Object::from(4)),
        Object::from(vec![Object::from(4), Object::from(-4)])
    )
}

#[test]
fn test_len() {
    python_function! {r"
def test(a):
    return len(a)"
    }

    assert_eq!(test(&Object::from("abcdef")), Object::from(6));
    assert_eq!(test(&Object::from("あいうえお")), Object::from(5));
    assert_eq!(
        test(&Object::from(vec![Object::from(1), Object::from(2)])),
        Object::from(2)
    );
}

#[test]
fn test_dict() {
    python_function! {r#"
def test():
    x = {1:2, "a":3}
    return x"#
    }

    assert_eq!(
        test(),
        Object::dict(vec![
            (Object::from(1), Object::from(2)),
            (Object::from("a"), Object::from(3))
        ])
    );

    python_function! {r#"
def test2():
    x = {"a": 2}
    return x["a"]"#}
    assert_eq!(test2(), Object::from(2));

    python_function! {r#"
def test3():
    x = {"a": 2}
    x["a"] = 1
    return x["a"]"#}
    assert_eq!(test3(), Object::from(1));

    python_function! {r#"
def test4():
    x = {}
    x["a"] = 3
    return x["a"]"#}
    assert_eq!(test4(), Object::from(3));
}
