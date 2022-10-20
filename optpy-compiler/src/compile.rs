use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::{
    context::{collect_declared_variables, resolve_variable_names, IdentifierStore},
    expression::Expression,
    statement::Statement,
};

pub fn compile_code(code: &str) -> Result<TokenStream> {
    let ast = rustpython_parser::parser::parse_program(code)?;
    let mut statements = vec![];
    for statement in ast.statements {
        let statement = Statement::interpret(&statement)?;
        statements.extend(statement);
    }

    let mut store = IdentifierStore::new();
    let mut ctx = vec![];
    collect_declared_variables(&statements, &mut ctx, &mut store);
    let statements = resolve_variable_names(&statements, &mut ctx, &store)?;
    let code = statements
        .iter()
        .map(|s| s.to_token_stream())
        .collect::<TokenStream>();
    Ok(quote! {
        fn main() {
            #code
        }
    })
}

impl ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Statement::Expression { inner } => tokens.append_all(quote! { #inner; }),
            Statement::Assign { target, value } => {
                tokens.append_all(quote! {
                    #target = #value;
                });
            }
            Statement::If { test, body, orelse } => {
                tokens.append_all(quote! {
                    if #test {
                        #(#body);*
                    }
                });
                if let Some(statements) = orelse {
                    tokens.append_all(quote! {
                        else {
                            #(#statements);*
                        }
                    });
                }
            }
            Statement::Initialize { variables } => {
                for v in variables {
                    let v = format_ident!("{}", v);
                    tokens.append_all(quote! {
                        let mut #v = Value::None;
                    });
                }
            }
        }
    }
}

impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Expression::Identifier { name } => {
                let ident = format_ident!("{}", name);
                tokens.append_all(quote! { #ident });
            }
            Expression::Call { function, args } => {
                let args = args.iter().map(|a| a.to_token_stream()).collect::<Vec<_>>();
                tokens.append_all(quote! {
                    #function( #(#args),* )
                });
            }
            Expression::Binop { a, b, op } => {
                tokens.append_all(quote! { #a #op #b });
            }
            Expression::Tuple { elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_token_stream())
                    .collect::<Vec<_>>();
                tokens.append_all(quote! {
                    [
                        #(#elements),*
                    ]
                });
            }
            Expression::Attribute { value, name } => {
                let name = format_ident!("{}", name);
                tokens.append_all(quote! {
                    #value.#name
                });
            }
            Expression::Compare { values, ops } => {
                let n = ops.len();
                assert_eq!(n + 1, values.len());

                let mut compares = Vec::with_capacity(n);
                for i in 0..n {
                    let left = &values[i];
                    let right = &values[i + 1];
                    let op = ops[i];
                    compares.push(quote! {
                        #left
                        #op
                        #right
                    });
                }
                tokens.append_all(quote! {
                    #(#compares)&&*
                });
            }
            Expression::Number { value } => {
                tokens.append_all(quote! { #value });
            }
            Expression::Subscript { a, b } => {
                tokens.append_all(quote! {
                    #a.index(#b)
                });
            }
        }
    }
}
