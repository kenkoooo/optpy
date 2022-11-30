use optpy_generator::CodeGenerator;
use optpy_parser::{parse, Func};
use optpy_resolver::resolve;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    LitStr, Result,
};

#[proc_macro]
pub fn python_function(tokens: TokenStream) -> TokenStream {
    let input: PythonTestInput = syn::parse(tokens).unwrap();
    let code = input.python_code.value();

    let ast = parse(code).unwrap();
    assert_eq!(ast.len(), 1);
    let function_name = match &ast[0] {
        optpy_parser::Statement::Func(Func { name, .. }) => name.clone(),
        _ => panic!(),
    };
    let (ast, definitions) = resolve(&ast);
    let (name, args) = match &ast[0] {
        optpy_parser::Statement::Func(Func { name, args, .. }) => (name.clone(), args.clone()),
        _ => panic!(),
    };

    let generator = CodeGenerator::default();
    let code = generator.generate_function_body(&ast, "", &definitions);
    let function_name = format_ident!("{}", function_name);
    let args = args
        .into_iter()
        .map(|arg| format_ident!("{}", arg))
        .collect::<Vec<_>>();
    let resolved_name = format_ident!("{}", name);

    let result = quote! {
        #[allow(unreachable_code)]
        fn #function_name(#(#args: &optpy_runtime::Value),*) -> optpy_runtime::Value {
            use optpy_runtime::*;
            #code

            #resolved_name( #(#args),* )
        }
    };

    result.into()
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
