use optpy_generator::generate_function_body;
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

    let code = generate_function_body(&ast, "", &definitions, false);
    let function_name = format_ident!("{}", function_name);
    let args = args
        .into_iter()
        .map(|arg| format_ident!("{}", arg))
        .collect::<Vec<_>>();
    let resolved_name = format_ident!("{}", name);

    let result = quote! {
        #[allow(unreachable_code)]
        fn #function_name(#(#args: &optpy_std::object::Object),*) -> optpy_std::object::Object {
            use optpy_std::object::*;
            use optpy_std::builtin::*;
            use optpy_std::*;
            #code

            #resolved_name( #(#args),* )
        }
    };

    result.into()
}
#[proc_macro]
pub fn typed_python_function(tokens: TokenStream) -> TokenStream {
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

    let code = generate_function_body(&ast, "", &definitions, true);
    let function_name = format_ident!("{}", function_name);
    let interface = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let arg = format_ident!("{}", arg);
            let param = format_ident!("T{}", i);
            quote! {
                #arg: &optpy_std::typed_object::Object<#param>
            }
        })
        .collect::<Vec<_>>();
    let type_params = (0..interface.len())
        .map(|i| {
            let t = format_ident!("T{}", i);
            quote! {
                #t: optpy_std::typed_object::Value
            }
        })
        .collect::<Vec<_>>();
    let args = args
        .into_iter()
        .map(|arg| format_ident!("{}", arg))
        .collect::<Vec<_>>();
    let resolved_name = format_ident!("{}", name);

    let result = quote! {
        #[allow(unreachable_code)]
        fn #function_name<#(#type_params),*>(#(#interface),*) -> optpy_std::typed_object::Object<impl optpy_std::typed_object::Value> {
            use optpy_std::typed_object::*;
            use optpy_std::typed_builtin::*;
            use optpy_std::*;
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
