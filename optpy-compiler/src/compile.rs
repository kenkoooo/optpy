use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{expression::Expression, statement::Statement};

pub fn compile_code(code: &str) -> Result<String> {
    let ast = rustpython_parser::parser::parse_program(code)?;
    let mut statements = vec![];
    for statement in ast.statements {
        let statement = Statement::interpret(&statement)?;
        statements.push(statement);
    }
    let code = generate_code(&statements)?;
    Ok(code.to_string())
}

fn generate_code(statements: &[Statement]) -> Result<TokenStream> {
    let mut result = vec![];
    for statement in statements {
        match statement {
            Statement::Expression { inner } => {
                let expr = generate_expr(inner);
                result.push(quote! { #expr; })
            }
            Statement::Assign { targets, value } => {
                if targets.len() != 1 {
                    return Err(anyhow!("unsupporeted non-single target assignment"));
                }
                let target = generate_expr(&targets[0]);
                let value = generate_expr(value);
                result.push(quote! {
                    let #target = #value;
                });
            }
        }
    }

    Ok(TokenStream::from_iter(result.into_iter()))
}

fn generate_expr(expr: &Expression) -> TokenStream {
    match expr {
        Expression::Identifier { name } => {
            let ident = format_ident!("{}", name);
            quote! { #ident }
        }
        Expression::Call { function, args } => {
            let function = generate_expr(function);
            let args = args.iter().map(|a| generate_expr(a)).collect::<Vec<_>>();
            quote! {
                #function( #(#args),* )
            }
        }
        Expression::Binop { a, b, op } => {
            let op = TokenStream::from(op);
            let a = generate_expr(a);
            let b = generate_expr(b);
            quote! { #a #op #b }
        }
    }
}
