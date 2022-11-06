mod test_env;

macro_rules! optpy_integration_test {
    ($name:ident, $code:expr, $(($input:expr, $output:expr)),+) => {
        #[test]
        fn $name() {
            let code = $code;
            let tests = [$(($input, $output)),+];
            for (input, output) in tests {
                assert_eq!(test_env::execute(code, input).unwrap(), output);
            }
        }
    };
    (ignore, $name:ident, $code:expr, $(($input:expr, $output:expr)),+) => {
        #[test]
        #[ignore]
        fn $name() {
            let code = $code;
            let tests = [$(($input, $output)),+];
            for (input, output) in tests {
                assert_eq!(test_env::execute(code, input).unwrap(), output);
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
ignore,
test_while_loop,
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
("101\n", "2\n"),
("000\n", "0\n")
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
