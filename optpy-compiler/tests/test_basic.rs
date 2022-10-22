use optpy_compiler::compile_code;
use quote::quote;

#[test]
fn test_assign() {
    let code = r"
a = int(input())
b = int(input())
answer = b - a
print(answer)
";
    let result = compile_code(code).unwrap();
    assert_eq!(
        result.to_string(),
        quote! {
            fn main() {
                let mut __v0 = Value::None;
                let mut __v1 = Value::None;
                let mut __v2 = Value::None;

                __v0 = int(input());
                __v1 = int(input());
                __v2 = __v1 - __v0;
                print(__v2);
            }
        }
        .to_string()
    );
}

#[test]
fn test_if_statement() {
    let code = r"
a, b, c = map(int, input().split())
if a <= c < b:
    answer = 1
else:
    answer = 0
print(answer)
";
    let result = compile_code(code).unwrap();
    assert_eq!(
        result.to_string(),
        quote! {
            fn main() {
                let mut __v0 = Value::None;
                let mut __v1 = Value::None;
                let mut __v2 = Value::None;
                let mut __v3 = Value::None;
                let mut __v4 = Value::None;

                __v0 = map(int, input().split());
                __v1 = __v0.index(Value::i64(0i64));
                __v2 = __v0.index(Value::i64(1i64));
                __v3 = __v0.index(Value::i64(2i64));

                if __v1 <= __v3 && __v3 < __v2 {
                    __v4 = Value::i64(1i64);
                } else {
                    __v4 = Value::i64(0i64);
                }

                print(__v4);
            }
        }
        .to_string()
    );
}

#[test]
fn test_for_loop() {
    let code = r"
n=int(input())
ans=1
for i in range(1,n+1):
    ans = ans * i
print(ans)
";
    let result = compile_code(code).unwrap();
    assert_eq!(
        result.to_string(),
        quote! {
            fn main() {
                let mut __v0 = Value::None;
                let mut __v1 = Value::None;
                let mut __v2 = Value::None;
                __v0 = int(input());
                __v1 = Value::i64(1i64);
                for __for_tmp_v in range!(Value::i64(1i64), __v0 + Value::i64(1i64)) {
                    __v2 = Value::from(__for_tmp_v);
                    __v1 = __v1 * __v2;
                }
                print(__v1);
            }
        }
        .to_string()
    );
}
