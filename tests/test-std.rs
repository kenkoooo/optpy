use optpy_std::Value;
use optpy_test_macro::python_function;

#[test]
fn test_sorted() {
    python_function! {r"
def test():
    x = [2, 1]
    x = sorted(x)
    return x"}

    assert_eq!(test(), Value::from(vec![Value::from(1), Value::from(2)]));
}

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
fn test_split() {
    python_function! {r#"
def test():
    x = "abc efg"
    return x.split()"#}

    assert_eq!(
        test(),
        Value::from(vec![Value::from("abc"), Value::from("efg")])
    );
}

#[test]
fn test_assign() {
    python_function! {r"
def test():
    x = [0]
    x[0] = x[0]
    return x[0]"}

    assert_eq!(test(), Value::from(0));

    python_function! {r"
def test2():
    x = [1]
    x[0] = 2
    return x"}

    assert_eq!(test2(), Value::from(vec![Value::from(2)]));
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

#[test]
fn test_len() {
    python_function! {r"
def test(a):
    return len(a)"
    }

    assert_eq!(test(&Value::from("abcdef")), Value::from(6));
    assert_eq!(test(&Value::from("あいうえお")), Value::from(5));
    assert_eq!(
        test(&Value::from(vec![Value::from(1), Value::from(2)])),
        Value::from(2)
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
        Value::dict(vec![
            (Value::from(1), Value::from(2)),
            (Value::from("a"), Value::from(3))
        ])
    );

    python_function! {r#"
def test2():
    x = {"a": 2}
    return x["a"]"#}
    assert_eq!(test2(), Value::from(2));

    python_function! {r#"
def test3():
    x = {"a": 2}
    x["a"] = 1
    return x["a"]"#}
    assert_eq!(test3(), Value::from(1));

    python_function! {r#"
def test4():
    x = {}
    x["a"] = 3
    return x["a"]"#}
    assert_eq!(test4(), Value::from(3));

    python_function! {r#"
def test5():
    x = dict()
    x["a"] = 3
    return x["a"]"#}
    assert_eq!(test5(), Value::from(3));
}

#[test]
fn test_mutable_index_reference() {
    python_function! {r#"
def test():
    def f(x):
        x.pop()
        return [1, 2, 3]


    x = [[1, 2, 3]]
    if x[0] == f(x):
        return "eq"
    else:
        return "neq""#}
    assert_eq!(test(), Value::from("eq"));
}

#[test]
fn test_in() {
    {
        python_function! {r#"
def test(key):
    a = {key: 10}
    return "a" in a"#};

        assert_eq!(test(&Value::from("b")), Value::from(false));
        assert_eq!(test(&Value::from("a")), Value::from(true));
    }
    {
        python_function! {r#"
def test(key):
    a = {key: 10}
    return "a" not in a"#};

        assert_eq!(test(&Value::from("b")), Value::from(true));
        assert_eq!(test(&Value::from("a")), Value::from(false));
    }
}

#[test]
fn test_pow() {
    {
        python_function! {r#"
def test(x, e, m):
    return pow(x, e, m)"#}

        assert_eq!(
            test(&Value::from(2), &Value::from(3), &Value::from(10)),
            Value::from(8)
        );
        assert_eq!(
            test(&Value::from(2), &Value::from(4), &Value::from(10)),
            Value::from(6)
        );
    }

    {
        python_function! {r#"
def test(x, e):
    return x ** e"#}

        assert_eq!(test(&Value::from(2), &Value::from(3)), Value::from(8));
        assert_eq!(test(&Value::from(2), &Value::from(4)), Value::from(16));
    }
}

#[test]
fn test_range() {
    {
        python_function! {r#"
def test():
    result = []
    for i in range(5):
        result.append(i)
    return result"#}

        assert_eq!(
            test(),
            Value::from(vec![
                Value::from(0),
                Value::from(1),
                Value::from(2),
                Value::from(3),
                Value::from(4),
            ])
        );
    }
    {
        python_function! {r#"
def test():
    result = []
    for i in range(2, 5):
        result.append(i)
    return result"#}

        assert_eq!(
            test(),
            Value::from(vec![Value::from(2), Value::from(3), Value::from(4),])
        );
    }
}

#[test]
fn test_function_call_index() {
    python_function! {r"
def test():
    i = 0
    def f():
        return i
    a = [0, 1, 2]
    return a[f()]"}
    assert_eq!(test(), Value::from(0));
}

#[test]
fn test_set() {
    {
        python_function! {r"
def test():
    a = [1, 2, 3, 2, 1]
    return len(set(a))"}
        assert_eq!(test(), Value::from(3));
    }
    {
        python_function! {r"
def test(a):
    a = set(a)
    return 1 in a"}
        assert_eq!(
            test(&Value::from(vec![
                Value::from(1),
                Value::from(2),
                Value::from(3)
            ])),
            Value::from(true)
        );
    }
    {
        python_function! {r"
def test():
    a = set()
    a.add(1)
    return 1 in a"}
        assert_eq!(test(), Value::from(true));
    }
}

#[test]
fn test_strip() {
    python_function! {r"
def test(a):
    return a.strip()"}
    assert_eq!(test(&Value::from("   aaa   ")), Value::from("aaa"));
}

#[test]
fn test_compile_exit() {
    python_function! {r"
def test():
    exit()
    exit(1)
    return 1"}
}

#[test]
fn test_str() {
    python_function! {r"
def test(a):
    return str(a)"}
    assert_eq!(test(&Value::from(1)), Value::from("1"));
    assert_eq!(test(&Value::from(0.5)), Value::from("0.5"));
    assert_eq!(test(&Value::from("string")), Value::from("string"));
}

#[test]
fn test_bool_and_assign() {
    python_function! {r"
def test(a, b):
    a &= b
    return a"}
    assert_eq!(
        test(&Value::from(true), &Value::from(false)),
        Value::from(false)
    );
    assert_eq!(
        test(&Value::from(true), &Value::from(true)),
        Value::from(true)
    );
}

#[test]
fn test_unary_not() {
    python_function! {r"
def test(a):
    return not a"}
    assert_eq!(test(&Value::from(true)), Value::from(false));
    assert_eq!(test(&Value::from(false)), Value::from(true));
}

#[test]
fn test_any() {
    python_function! {r"
def test(s):
    return any(si in [1, 2, 3] for si in s)"}

    assert_eq!(
        test(&Value::from(vec![Value::from(1), Value::from(4)])),
        Value::from(true)
    );
    assert_eq!(
        test(&Value::from(vec![Value::from(5), Value::from(4)])),
        Value::from(false)
    );
}

#[test]
fn test_all() {
    python_function! {r"
def test(s):
    return all(si in [1, 2, 3] for si in s)"}

    assert_eq!(
        test(&Value::from(vec![Value::from(1), Value::from(3)])),
        Value::from(true)
    );
    assert_eq!(
        test(&Value::from(vec![Value::from(1), Value::from(4)])),
        Value::from(false)
    );
}

#[test]
fn test_count() {
    python_function! {r"
def test(s, t):
    return s.count(t)"}
    assert_eq!(
        test(&Value::from("abcdabc"), &Value::from("abc")),
        Value::from(2)
    );
    assert_eq!(
        test(&Value::from("abcdabc"), &Value::from("abcd")),
        Value::from(1)
    );
    assert_eq!(
        test(
            &Value::from(vec![Value::from(1), Value::from(2), Value::from(2)]),
            &Value::from(2)
        ),
        Value::from(2)
    );
}

#[test]
fn test_min_max() {
    {
        python_function! {r"
def test(a, b):
    return [min(a,b), max(a,b)]"}
        assert_eq!(
            test(&Value::from(1), &Value::from(2)),
            Value::from(vec![Value::from(1), Value::from(2)])
        );
    }
    {
        python_function! {r"
def test(a, b, c):
    return [min(a,b,c), max(a,b,c)]"}
        assert_eq!(
            test(&Value::from(1), &Value::from(3), &Value::from(2)),
            Value::from(vec![Value::from(1), Value::from(3)])
        );
    }
    {
        python_function! {r"
def test(arr):
    return [min(arr), max(arr)]"}
        assert_eq!(
            test(&Value::from(vec![
                Value::from(1),
                Value::from(3),
                Value::from(2)
            ])),
            Value::from(vec![Value::from(1), Value::from(3)])
        );
    }
}

#[test]
fn test_sum() {
    {
        python_function! {r"
def test():
    return sum(1, 2)
    "}
        assert_eq!(test(), Value::from(3));
    }

    {
        python_function! {r"
def test():
    return sum(1, 2, 3)
    "}
        assert_eq!(test(), Value::from(6));
    }

    {
        python_function! {r"
def test():
    return sum([1, 2])
    "}
        assert_eq!(test(), Value::from(3));
    }
}

#[test]
fn test_negative_index() {
    python_function! {r"
def test():
    a = [1, 2, 3]
    return [a[-1], a[-2], a[-3]]
    "}
    assert_eq!(
        test(),
        Value::from(vec![Value::from(3), Value::from(2), Value::from(1)])
    );
}
#[test]
fn test_del_list() {
    python_function! {r"
def test():
    a = [1, 2, 3]
    del a[0]
    return a
    "}
    assert_eq!(test(), Value::from(vec![Value::from(2), Value::from(3)]));
}

#[test]
fn test_enumerate() {
    python_function! {r#"
def test():
    a = ["a", "b", "c"]
    return enumerate(a)"#}
    assert_eq!(
        test(),
        Value::from(vec![
            Value::from(vec![Value::from(0), Value::from("a")]),
            Value::from(vec![Value::from(1), Value::from("b")]),
            Value::from(vec![Value::from(2), Value::from("c")]),
        ])
    );
}

#[test]
fn test_tuple() {
    python_function! {r#"
def test():
    a = ["a", "b", "c"]
    return tuple(a)"#}
    assert_eq!(
        test(),
        Value::from(vec![Value::from("a"), Value::from("b"), Value::from("c"),])
    );
}

#[test]
fn test_next() {
    python_function! {r"
def test():
    x = (i for i in range(3))
    return [next(x), next(x), next(x)]"}
    assert_eq!(
        test(),
        Value::from(vec![Value::from(0), Value::from(1), Value::from(2)])
    );
}

#[test]
fn test_multiply_list() {
    python_function! {r"
def test():
    x = [1, 2, 3]
    return x * 3"}
    assert_eq!(
        test(),
        Value::from(vec![
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from(1),
            Value::from(2),
            Value::from(3)
        ])
    );
}
