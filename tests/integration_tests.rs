mod test_env;

macro_rules! optpy_integration_test {
    ($name:ident, $code:expr, $(($input:expr, $output:expr)),+) => {
        #[test]
        fn $name() {
            let code = $code;
            let tests = [$(($input, $output)),+];
            for (input, expected) in tests {
                let (output, code) = test_env::execute(code, input).unwrap();
                assert_eq!(output, expected, "{}", code);
            }
        }
    };
    (ignore, $name:ident, $code:expr, $(($input:expr, $output:expr)),+) => {
        #[test]
        #[ignore]
        fn $name() {
            let code = $code;
            let tests = [$(($input, $output)),+];
            for (input, expected) in tests {
                let (output, code) = test_env::execute(code, input).unwrap();
                assert_eq!(output, expected, "{}", code);
            }
        }
    };
}

optpy_integration_test! {
test_if_statement,
r#"
a, b = map(int, input().split())
ans = a * b
if ans % 2 == 0:
    print("Even")
else:
    print("Odd")
"#,
("3 4\n", "Even\n"),
("3 5\n", "Odd\n")
}

optpy_integration_test! {
test_multiple_if_conditions,
r#"
a, b, c = map(int, input().split())
ans = a * b
if a <= b < c:
    print("IN")
else:
    print("OUT")
"#,
("3 4 5\n", "IN\n"),
("3 5 4\n", "OUT\n")
}

optpy_integration_test! {
test_count,
r#"
s = input()
print(s.count('1'))
"#,
("101\n", "2\n"),
("000\n", "0\n")
}

optpy_integration_test! {
test_list_add_assign,
r"
A = list(map(int, input().split()))
A[0] += 1
print(A[0])
",
("1 2 3\n", "2\n")
}

optpy_integration_test! {
test_solve_abc081_b,
r#"
N = int(input())
A = list(map(int, input().split()))

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
print(count)
"#,
("3\n8 12 40\n", "2\n"),
("4\n5 6 8 10\n", "0\n")
}

optpy_integration_test! {
test_for_loop,
r#"
N = int(input())
ans = 0
for i in range(N):
    ans += i
print(ans)
"#,
("5\n", "10\n"),
("10\n", "45\n")
}

optpy_integration_test! {
test_ops,
r#"
N, M = map(int, input().split())
print(N + M)
print(N * M)
print(N - M)
print(N / M)
print(N // M)
"#,
("4 2\n", "6\n8\n2\n2\n2\n"),
("1 2\n", "3\n2\n-1\n0.5\n0\n")
}

optpy_integration_test! {
test_recursive_fibonacci,
r#"
def fib(n):
    if n == 1 or n == 0:
        return 1
    return fib(n - 1) + fib(n - 2)
n = int(input())
n = fib(n)
print(n)
"#,
("0\n", "1\n"),
("1\n", "1\n"),
("2\n", "2\n"),
("3\n", "3\n"),
("4\n", "5\n")
}

optpy_integration_test! {
test_multiple_print,
r#"
A = 1
B = 2
S = "hello"
print(A+B, S)
"#,
("", "3 hello\n")
}

optpy_integration_test! {
test_list_initialization,
r#"
A = []
A.append("A")
A.append("B")
print(A[0], A[1])
"#,
("", "A B\n")
}
