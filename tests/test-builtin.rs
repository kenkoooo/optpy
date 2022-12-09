use optpy_runtime::{ToValue, Value};
use optpy_test_macro::python_function;

#[test]
fn test_list() {
    python_function! {r"
def test():
    x = [1, 2, 3]
    y = list(x)
    x[0] = 200
    return y"}

    assert_eq!(test(), v(vec![v(1), v(2), v(3)]));
}

#[test]
fn test_sorted() {
    python_function! {r"
def test():
    x = [2, 1]
    x = sorted(x)
    return x"}

    assert_eq!(test(), v(vec![v(1), v(2)]));

    python_function! {r"
def test2():
    x = [2, 1]
    y = sorted(x)
    return x"}

    assert_eq!(test2(), v(vec![v(2), v(1)]));
}

#[test]
fn test_map_int() {
    python_function! {r"
def test(s):
    a = map(int, s.split())
    return a
"}

    assert_eq!(test(&v("1 2")), v(vec![v(1), v(2)]));
}

#[test]
fn test_split() {
    python_function! {r#"
def test():
    x = "abc efg"
    return x.split()"#}

    assert_eq!(test(), v(vec![v("abc"), v("efg")]));
}

#[test]
fn test_assign() {
    python_function! {r"
def test():
    x = [0]
    x[0] = x[0]
    return x[0]"}

    assert_eq!(test(), v(0));

    python_function! {r"
def test2():
    x = [1]
    x[0] = 2
    return x"}

    assert_eq!(test2(), v(vec![v(2)]));
}

#[test]
fn test_ops() {
    python_function! {
        r"
def test_ops(N, M):
    return [N + M, N * M, N - M, N / M, N // M]"
    };
    let result = test_ops(&v(4), &v(2));
    assert_eq!(result, v(vec![v(6), v(8), v(2), v(2), v(2),]));

    let result = test_ops(&v(1), &v(2));
    assert_eq!(result, v(vec![v(3), v(2), v(-1), v(0.5), v(0),]));
}

#[test]
fn test_unary_ops() {
    python_function! {r"
def test(a):
    return [+a, -a]"
    }

    assert_eq!(test(&v(4)), v(vec![v(4), v(-4)]))
}

#[test]
fn test_len() {
    python_function! {r"
def test(a):
    return len(a)"
    }

    assert_eq!(test(&v("abcdef")), v(6));
    assert_eq!(test(&v("あいうえお")), v(5));
    assert_eq!(test(&v(vec![v(1), v(2)])), v(2));
}

#[test]
fn test_dict() {
    python_function! {r#"
def test():
    x = {1:2, "a":3}
    return x"#
    }

    assert_eq!(test(), Value::dict(vec![(v(1), v(2)), (v("a"), v(3))]));

    python_function! {r#"
def test2():
    x = {"a": 2}
    return x["a"]"#}
    assert_eq!(test2(), v(2));

    python_function! {r#"
def test3():
    x = {"a": 2}
    x["a"] = 1
    return x["a"]"#}
    assert_eq!(test3(), v(1));

    python_function! {r#"
def test4():
    x = {}
    x["a"] = 3
    return x["a"]"#}
    assert_eq!(test4(), v(3));

    python_function! {r#"
def test5():
    x = dict()
    x["a"] = 3
    return x["a"]"#}
    assert_eq!(test5(), v(3));
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
    assert_eq!(test(), v("eq"));
}

#[test]
fn test_in() {
    {
        python_function! {r#"
def test(key):
    a = {key: 10}
    return "a" in a"#};

        assert_eq!(test(&v("b")), v(false));
        assert_eq!(test(&v("a")), v(true));
    }
    {
        python_function! {r#"
def test(key):
    a = {key: 10}
    return "a" not in a"#};

        assert_eq!(test(&v("b")), v(true));
        assert_eq!(test(&v("a")), v(false));
    }
}

#[test]
fn test_pow() {
    {
        python_function! {r#"
def test(x, e, m):
    return pow(x, e, m)"#}

        assert_eq!(test(&v(2), &v(3), &v(10)), v(8));
        assert_eq!(test(&v(2), &v(4), &v(10)), v(6));
    }

    {
        python_function! {r#"
def test(x, e):
    return x ** e"#}

        assert_eq!(test(&v(2), &v(3)), v(8));
        assert_eq!(test(&v(2), &v(4)), v(16));
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

        assert_eq!(test(), v(vec![v(0), v(1), v(2), v(3), v(4),]));
    }
    {
        python_function! {r#"
def test():
    result = []
    for i in range(2, 5):
        result.append(i)
    return result"#}

        assert_eq!(test(), v(vec![v(2), v(3), v(4),]));
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
    assert_eq!(test(), v(0));
}

#[test]
fn test_set() {
    {
        python_function! {r"
def test():
    a = [1, 2, 3, 2, 1]
    return len(set(a))"}
        assert_eq!(test(), v(3));
    }
    {
        python_function! {r"
def test(a):
    a = set(a)
    return 1 in a"}
        assert_eq!(test(&v(vec![v(1), v(2), v(3)])), v(true));
    }
    {
        python_function! {r"
def test():
    a = set()
    a.add(1)
    return 1 in a"}
        assert_eq!(test(), v(true));
    }
}

#[test]
fn test_strip() {
    python_function! {r"
def test(a):
    return a.strip()"}
    assert_eq!(test(&v("   aaa   ")), v("aaa"));
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
    assert_eq!(test(&v(1)), v("1"));
    assert_eq!(test(&v(0.5)), v("0.5"));
    assert_eq!(test(&v("string")), v("string"));
}

#[test]
fn test_bool_and_assign() {
    python_function! {r"
def test(a, b):
    a &= b
    return a"}
    assert_eq!(test(&v(true), &v(false)), v(false));
    assert_eq!(test(&v(true), &v(true)), v(true));
}

#[test]
fn test_unary_not() {
    python_function! {r"
def test(a):
    return not a"}
    assert_eq!(test(&v(true)), v(false));
    assert_eq!(test(&v(false)), v(true));
}

#[test]
fn test_any() {
    python_function! {r"
def test(s):
    return any(si in [1, 2, 3] for si in s)"}

    assert_eq!(test(&v(vec![v(1), v(4)])), v(true));
    assert_eq!(test(&v(vec![v(5), v(4)])), v(false));
}

#[test]
fn test_all() {
    python_function! {r"
def test(s):
    return all(si in [1, 2, 3] for si in s)"}

    assert_eq!(test(&v(vec![v(1), v(3)])), v(true));
    assert_eq!(test(&v(vec![v(1), v(4)])), v(false));
}

#[test]
fn test_count() {
    python_function! {r"
def test(s, t):
    return s.count(t)"}
    assert_eq!(test(&v("abcdabc"), &v("abc")), v(2));
    assert_eq!(test(&v("abcdabc"), &v("abcd")), v(1));
    assert_eq!(test(&v(vec![v(1), v(2), v(2)]), &v(2)), v(2));
}

#[test]
fn test_min_max() {
    {
        python_function! {r"
def test(a, b):
    return [min(a,b), max(a,b)]"}
        assert_eq!(test(&v(1), &v(2)), v(vec![v(1), v(2)]));
    }
    {
        python_function! {r"
def test(a, b, c):
    return [min(a,b,c), max(a,b,c)]"}
        assert_eq!(test(&v(1), &v(3), &v(2)), v(vec![v(1), v(3)]));
    }
    {
        python_function! {r"
def test(arr):
    return [min(arr), max(arr)]"}
        assert_eq!(test(&v(vec![v(1), v(3), v(2)])), v(vec![v(1), v(3)]));
    }
}

#[test]
fn test_sum() {
    {
        python_function! {r"
def test():
    return sum(1, 2)
    "}
        assert_eq!(test(), v(3));
    }

    {
        python_function! {r"
def test():
    return sum(1, 2, 3)
    "}
        assert_eq!(test(), v(6));
    }

    {
        python_function! {r"
def test():
    return sum([1, 2])
    "}
        assert_eq!(test(), v(3));
    }
}

#[test]
fn test_negative_index() {
    python_function! {r"
def test():
    a = [1, 2, 3]
    return [a[-1], a[-2], a[-3]]
    "}
    assert_eq!(test(), v(vec![v(3), v(2), v(1)]));
}
#[test]
fn test_del_list() {
    python_function! {r"
def test():
    a = [1, 2, 3]
    del a[0]
    return a
    "}
    assert_eq!(test(), v(vec![v(2), v(3)]));
}

#[test]
fn test_enumerate() {
    python_function! {r#"
def test():
    a = ["a", "b", "c"]
    return enumerate(a)"#}
    assert_eq!(
        test(),
        v(vec![
            v(vec![v(0), v("a")]),
            v(vec![v(1), v("b")]),
            v(vec![v(2), v("c")]),
        ])
    );
}

#[test]
fn test_tuple() {
    python_function! {r#"
def test():
    a = ["a", "b", "c"]
    return tuple(a)"#}
    assert_eq!(test(), v(vec![v("a"), v("b"), v("c"),]));
}

#[test]
fn test_next() {
    python_function! {r"
def test():
    x = (i for i in range(3))
    return [next(x), next(x), next(x)]"}
    assert_eq!(test(), v(vec![v(0), v(1), v(2)]));
}

#[test]
fn test_multiply_list() {
    python_function! {r"
def test():
    x = [1, 2, 3]
    return x * 3"}
    assert_eq!(
        test(),
        v(vec![v(1), v(2), v(3), v(1), v(2), v(3), v(1), v(2), v(3)])
    );
}

#[test]
fn test_abs() {
    python_function! {r"
def test(a):
    return abs(a)"}
    assert_eq!(test(&v(1)), v(1));
    assert_eq!(test(&v(-1)), v(1));
    assert_eq!(test(&v(1.5)), v(1.5));
}

#[test]
fn test_dict_keys() {
    python_function! {r#"
def test():
    a={}
    a["a"] = 1
    a["b"] = 2
    return sorted(list(a.keys()))"#}

    assert_eq!(test(), v(vec![v("a"), v("b")]));
}

#[test]
fn test_setdefault() {
    python_function! {r#"
def test():
    a = {}
    a.setdefault("b", [])
    a["b"].append(1)
    a.setdefault("b", [])
    a["b"].append(2)
    return a["b"]"#}

    assert_eq!(test(), v(vec![v(1), v(2)]));
}

#[test]
fn test_float() {
    python_function! {r"
def test():
    a = float('inf')
    return a"
    }
    assert_eq!(test(), v(f64::INFINITY))
}

#[test]
fn test_bool() {
    python_function! {r#"
def test(x):
    if x:
        return "OK"
    else:
        return "NO"
"#}

    assert_eq!(test(&v("")), v("NO"));
    assert_eq!(test(&v("s")), v("OK"));

    assert_eq!(test(&v(vec![])), v("NO"));
    assert_eq!(test(&v(vec![v("a")])), v("OK"));
}

#[test]
fn test_list_iteration() {
    python_function! {r#"
def test():
    a = [[0, 1]]
    result = []
    for i in a[0]:
        result.append(i)
    for i in a[0]:
        result.append(i)
    return result
    "#}

    assert_eq!(test(), v(vec![v(0), v(1), v(0), v(1)]));
}

#[test]
fn test_compare_tuple() {
    python_function! {r"
def test(a, b):
    return a < b"}
    assert_eq!(test(&v(vec![v(1), v(3)]), &v(vec![v(2), v(1)])), v(true))
}

fn v<T: ToValue>(t: T) -> Value {
    t.to_value()
}
