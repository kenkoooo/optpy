use optpy_generator::generate_function_body;
use optpy_parser::parse;
use optpy_resolver::resolve;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    LitStr, Result,
};

#[proc_macro]
pub fn test_python(tokens: TokenStream) -> TokenStream {
    let input: PythonTestInput = syn::parse(tokens).unwrap();
    let code = input.python_code.value();

    let ast = parse(code).unwrap();
    let (ast, definitions) = resolve(&ast);
    let code = generate_function_body(&ast, "", &definitions);

    quote! {
        {
            use optpy_std::*;
            fn test_function() -> Value {
                #code
            }
            test_function()
        }
    }
    .into()
}

struct PythonTestInput {
    python_code: LitStr,
}

impl Parse for PythonTestInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let python_code = input.parse()?;
        Ok(Self { python_code })
    }
}
