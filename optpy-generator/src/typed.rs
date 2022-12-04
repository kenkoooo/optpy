use optpy_parser::Number;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::CodeGenerator;

impl CodeGenerator {
    pub fn typed() -> Self {
        Self {
            function_formatter: format_function,
            number_formatter: format_number,
            function_call_formatter: format_function_call,
            declaration_formatter: format_declaration,
            list_formatter: format_list,
            tuple_formatter: format_tuple,
            boolean_formatter: format_constant_boolean,
            ..Default::default()
        }
    }
}

fn format_function(name: &str, args: &[String], body: TokenStream) -> TokenStream {
    let args = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let arg = format_ident!("{}", arg);
            let type_param = format_ident!("T{}", i);
            (arg, type_param)
        })
        .collect::<Vec<_>>();
    let names = args.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let name = format_ident!("{}", name);
    quote! {
        let #name = |#(#names),*| {
            #(let mut #names = #names.__shallow_copy();)*
            #body
            return Default::default();
        };
    }
}

fn format_number(number: &Number) -> TokenStream {
    match number {
        Number::Int(int) => match int.parse::<i64>() {
            Ok(int) => {
                quote! {
                    Number::from(#int)
                }
            }
            Err(_) => {
                todo!("bigint is not supported");
            }
        },
        Number::Float(float) => match float.parse::<f64>() {
            Ok(float) => {
                quote! {
                    Number::from(#float)
                }
            }
            Err(e) => {
                panic!("unsupported float value: {} {:?}", float, e);
            }
        },
    }
}

fn format_function_call(args: &[TokenStream], name: &str) -> TokenStream {
    if let Some(macro_name) = name.strip_suffix("__macro__") {
        let name = format_ident!("typed_{}", macro_name);
        quote! {
            #name !( #(&#args),* )
        }
    } else {
        let name = format_ident!("{}", name);
        quote! {
            #name ( #(&#args),* )
        }
    }
}

fn format_declaration(variable: &str) -> TokenStream {
    let variable = format_ident!("{}", variable);
    quote! {
        let mut #variable = Default::default();
    }
}

fn format_tuple(tuple: &[TokenStream]) -> TokenStream {
    quote! {
       TypedList::from(vec![ #((#tuple).__shallow_copy()),* ])
    }
}
fn format_list(tuple: &[TokenStream]) -> TokenStream {
    quote! {
        TypedList::from(vec![ #((#tuple).__shallow_copy()),* ])
    }
}

fn format_constant_boolean(b: TokenStream) -> TokenStream {
    quote! {
        Bool::from(#b)
    }
}
