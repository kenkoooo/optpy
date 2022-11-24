use optpy_std::object::Object;
use optpy_test_macro::python_function;

#[test]
fn test_if_statement() {
    python_function! {r#"
def test(a, b):
    ans = a * b
    if ans % 2 == 0:
        return "Even"
    else:
        return "Odd"
    "#}

    let result = test(&Object::from(3), &Object::from(4));
    assert_eq!(result, Object::from("Even"));

    let result = test(&Object::from(3), &Object::from(5));
    assert_eq!(result, Object::from("Odd"));
}

#[test]
fn test_multiple_if_conditions() {
    python_function! {r#"
def test(a, b, c):
    ans = a * b
    if a <= b < c:
        return "IN"
    else:
        return "OUT"
    "#}

    let result = test(&Object::from(3), &Object::from(4), &Object::from(5));
    assert_eq!(result, Object::from("IN"));

    let result = test(&Object::from(3), &Object::from(5), &Object::from(4));
    assert_eq!(result, Object::from("OUT"));
}

#[test]
fn test_list_add_assign() {
    python_function! {r"
def test(A):
    A[0] += 1
    return A[0]
    "}
    assert_eq!(
        test(&Object::from(vec![
            Object::from(1),
            Object::from(2),
            Object::from(3)
        ])),
        Object::from(2)
    );
}

#[test]
fn test_solve_abc081_b() {
    python_function! {r"
def solve(N, A):    
    flag = 0
    count = 0
    
    while True:
        for i in range(N):
            if A[i] % 2 != 0:
                flag = 1
        if flag == 1:
            break
        for i in range(N):
            A[i] = A[i]//2
        count += 1
    return count
"}

    let result = solve(
        &Object::from(3),
        &Object::from(vec![Object::from(8), Object::from(12), Object::from(40)]),
    );
    assert_eq!(result, Object::from(2));

    let result = solve(
        &Object::from(4),
        &Object::from(vec![
            Object::from(5),
            Object::from(6),
            Object::from(8),
            Object::from(10),
        ]),
    );
    assert_eq!(result, Object::from(0));
}

#[test]
fn test_for_loop() {
    python_function! {r#"
def test(N):
    ans = 0
    for i in range(N):
        ans += i
    return ans
    "#}

    let result = test(&Object::from(5));
    assert_eq!(result, Object::from(10));

    let result = test(&Object::from(10));
    assert_eq!(result, Object::from(45));
}

#[test]
fn test_recursive_fibonacci() {
    python_function! {r#"
def test(n):
    def fib(n):
        if n == 1 or n == 0:
            return 1
        return fib(n - 1) + fib(n - 2)
    n = fib(n)
    return n
    "#}

    assert_eq!(test(&Object::from(0)), Object::from(1));
    assert_eq!(test(&Object::from(1)), Object::from(1));
    assert_eq!(test(&Object::from(2)), Object::from(2));
    assert_eq!(test(&Object::from(3)), Object::from(3));
    assert_eq!(test(&Object::from(4)), Object::from(5));
}

#[test]
fn test_list_initialization() {
    python_function! {r#"
def test():
    A = []
    A.append("A")
    A.append("B")
    return [A[0], A[1]]
    "#}
    assert_eq!(
        test(),
        Object::from(vec![Object::from("A"), Object::from("B")])
    );
}

#[test]
fn test_tuple_for_loop_target() {
    python_function! {r#"
def test():
    A = [["A", "B"] , ["C", "D"]]
    result = []
    for a, b in A:
        result.append([b, a])
    return result
    "#}

    assert_eq!(
        test(),
        Object::from(vec![
            Object::from(vec![Object::from("B"), Object::from("A")]),
            Object::from(vec![Object::from("D"), Object::from("C")])
        ])
    );
}

#[test]
fn test_assign_self() {
    python_function! {r#"
def test():
    x = [0]
    x[0] = x[0]
    return x[0]
    "#}
    assert_eq!(test(), Object::from(0));
}

#[test]
fn test_mut_mutated_self() {
    python_function! {r#"
def test():
    x = [0]
    x.append(x.pop()+200)
    return x[0]
    "#}
    assert_eq!(test(), Object::from(200));
}

#[test]
fn test_assign_in_loop() {
    python_function! {r#"
def test():
    for i in [0, 1, 2]:
        x = i
    return x
    "#}
    assert_eq!(test(), Object::from(2));
}

#[test]
fn test_mutate_argument() {
    python_function! {r"
def test_mutate_argument():
    def f(arr):
        arr[0] = 200
    arr = [0]
    f(arr)
    return arr[0]
"};
    let result = test_mutate_argument();
    assert_eq!(result, Object::from(200));
}

#[test]
fn test_short_circuit_evaluation() {
    python_function! {r#"
def test_short_circuit_evaluation(N):
    eval = []
    def a():
        eval.append(1)
        return True
    if N == 1 and a():
        return eval
    else:
        return eval
    "#};
    let result = test_short_circuit_evaluation(&Object::from(0));
    assert_eq!(result, Object::from(vec![Object::from(1)]));

    let result = test_short_circuit_evaluation(&Object::from(1));
    assert_eq!(result, Object::from(vec![]));
}

#[test]
fn test_array_assignment() {
    python_function! {
        r"
def test_array_assignment():
    a = [0, 1, 2]
    a[0] = a[1]
    a[1] = a[2]
    return [a[0], a[1], a[2]]
    "
    };
    let result = test_array_assignment();
    assert_eq!(
        result,
        Object::from(vec![Object::from(1), Object::from(2), Object::from(2)])
    );
}
#[test]
fn test_return_list_ref() {
    python_function! {
        r"
def test_return_list_ref():
    def f():
        a = [0, 1, 2]
        return a[1]
    return f() + 1
    "
    };
    let result = test_return_list_ref();
    assert_eq!(result, Object::from(2));
}

#[test]
fn test_continue() {
    python_function! {r#"
def test():
    res = []
    for i in range(5):
        if i%2==1:
            continue
        res.append(i)
    return res
"#}
    assert_eq!(
        test(),
        Object::from(vec![Object::from(0), Object::from(2), Object::from(4)])
    );
}

#[test]
fn test_list_comprehension() {
    python_function! {r#"
def test(N, M):
    a = [[i*j for j in range(M)] for i in range(N)]
    return a
    "#}

    assert_eq!(
        test(&Object::from(3), &Object::from(2)),
        Object::from(vec![
            Object::from(vec![Object::from(0), Object::from(0)]),
            Object::from(vec![Object::from(0), Object::from(1)]),
            Object::from(vec![Object::from(0), Object::from(2)])
        ])
    );
}

#[test]
fn test_multiple_target_assign() {
    python_function! {r"
def test():
    x = [1, 2, 3]
    def f():
        return x.pop()
    a = b = c = f()
    return [a, b, c]
"}
    assert_eq!(
        test(),
        Object::from(vec![Object::from(3), Object::from(3), Object::from(3)])
    );
}

#[test]
fn test_list_move() {
    {
        python_function! {r"
def test():
    a = 1
    b = 2
    x = []
    x.append((a, b))
    return [x, a, b]"}
    }
    {
        python_function! {r"
def test():
    a = 1
    b = 2
    x = []
    x.append([a, b])
    return [x, a, b]"}
    }
    {
        python_function! {r"
def test():
    a = 1
    b = 2
    x = {a: b}
    return [x, a, b]"}
    }
}

#[test]
fn test_none() {
    python_function! {
        r"
def test():
    a = None
    return a"
    }
    assert_eq!(test(), Object::none());
}

#[test]
fn test_del() {
    python_function! {
        r"
def test():
    a = 1
    del a
    return a"
    }
    assert_eq!(test(), Object::none());
}

#[test]
fn test_set() {
    python_function! {r"
def test(b):
    a = {b}
    return 1 in a"}
    assert_eq!(test(&Object::from("a")), Object::from(false));
    assert_eq!(test(&Object::from(1)), Object::from(true));
}
