use optpy_compiler::compile_code;
use quote::quote;

#[test]
fn test() {
    let code = include_str!("./assets/1.py");
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
