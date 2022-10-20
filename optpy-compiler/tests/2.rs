use optpy_compiler::compile_code;
use quote::quote;

#[test]
fn test() {
    let code = include_str!("./assets/2.py");
    let result = compile_code(code).unwrap();
    assert_eq!(
        result.to_string(),
        quote! {
            let mut __v0 = Value::None;
            let mut __v1 = Value::None;
            let mut __v2 = Value::None;
            let mut __v3 = Value::None;

            [__v0, __v1, __v2] = map(int, input().split());
            if __v0 <= __v2 && __v2 < __v1 {
                __v3 = Value::integer("1");
            } else {
                __v3 = Value::integer("0");
            }

            print(__v3);
        }
        .to_string()
    );
}
