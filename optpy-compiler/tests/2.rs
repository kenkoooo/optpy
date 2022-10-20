use optpy_compiler::compile_code;
use quote::quote;

#[test]
fn test() {
    let code = include_str!("./assets/2.py");
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
