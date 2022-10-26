use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::{
    context::{collect_declared_variables, resolve_variable_names, IdentifierStore},
    expression::OptpyExpression,
    statement::OptpyStatement,
};

pub fn compile_code(code: &str) -> Result<TokenStream> {
    let ast = rustpython_parser::parser::parse_program(code)?;
    let statements = load_statements(&ast)?;
    let statements = resolve_variables(&statements)?;
    Ok(quote! {
        fn main() {
            #(#statements)*
        }
    })
}

fn load_statements(ast: &rustpython_parser::ast::Program) -> Result<Vec<OptpyStatement>> {
    Ok(ast
        .statements
        .iter()
        .map(|s| OptpyStatement::load(s))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}

fn resolve_variables(statements: &[OptpyStatement]) -> Result<Vec<OptpyStatement>> {
    let mut store = IdentifierStore::new();
    let mut ctx = vec![];
    collect_declared_variables(&statements, &mut ctx, &mut store);
    let statements = resolve_variable_names(&statements, &mut ctx, &store)?;
    Ok(statements)
}

impl ToTokens for OptpyStatement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            OptpyStatement::Expression { inner } => tokens.append_all(quote! { #inner; }),
            OptpyStatement::Assign { target, value } => match target {
                OptpyExpression::Tuple { elements } => {
                    tokens.append_all(quote! {
                        let __tmp = #value;
                    });
                    for (i, element) in elements.iter().enumerate() {
                        tokens.append_all(quote! {
                            #element = __tmp.index(Value::from(#i));
                        });
                    }
                }
                _ => {
                    tokens.append_all(quote! {
                        #target = #value;
                    });
                }
            },
            OptpyStatement::If { test, body, orelse } => {
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
            OptpyStatement::Initialize { variables } => {
                let v = variables
                    .iter()
                    .map(|v| format_ident!("{}", v))
                    .collect::<Vec<_>>();
                tokens.append_all(quote! {
                    struct Context {
                        #(#v:Value,)*
                    }
                    let mut ctx = Context {
                        #(#v: Value::None,)*
                    };
                });
            }
            OptpyStatement::For { target, iter, body } => {
                let tmp = OptpyExpression::Identifier {
                    name: "__for_tmp_v".to_string(),
                };
                let assign = OptpyStatement::Assign {
                    target: target.clone(),
                    value: tmp.clone(),
                };
                tokens.append_all(quote! {
                    for #tmp in #iter {
                        let #tmp = Value::from(#tmp);
                        #assign
                        #(#body);*
                    }
                });
            }
            OptpyStatement::FunctionDef { name, body, args } => {
                let args = args
                    .iter()
                    .map(|arg| format_ident!("{}", arg))
                    .collect::<Vec<_>>();
                let name = format_ident!("{}", name);
                tokens.append_all(quote! {
                    fn #name(
                        #(#args: Value,)*
                    ) -> Value {
                        struct Context {
                            #(#args: Value,)*
                        }
                        let mut ctx = Context {
                            #(#args),*
                        };
                        #(#body);*
                    }
                });
            }
            OptpyStatement::Return { value } => match value {
                Some(value) => {
                    tokens.append_all(quote! {
                        return #value;
                    });
                }
                None => {
                    tokens.append_all(quote! {
                        return Value::None;
                    });
                }
            },
        }
    }
}

impl ToTokens for OptpyExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            OptpyExpression::Identifier { name } => {
                if let Some(name) = name.strip_suffix('!') {
                    let name = format_ident!("{}", name);
                    tokens.append_all(quote! { #name! });
                } else {
                    let name = format_ident!("{}", name);
                    tokens.append_all(quote! { #name });
                }
            }
            OptpyExpression::Call { function, args } => {
                tokens.append_all(quote! {
                    #function( #(#args),* )
                });
            }
            OptpyExpression::Binop { a, b, op } => {
                tokens.append_all(quote! { #a #op #b });
            }
            OptpyExpression::Tuple { .. } => todo!(),
            OptpyExpression::Attribute { value, name } => {
                let name = format_ident!("{}", name);
                tokens.append_all(quote! {
                    #value.#name
                });
            }
            OptpyExpression::Compare { values, ops } => {
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
            OptpyExpression::Number { value } => {
                tokens.append_all(quote! { #value });
            }
            OptpyExpression::Subscript { a, b } => {
                tokens.append_all(quote! {
                    #a.index(#b)
                });
            }
        }
    }
}
