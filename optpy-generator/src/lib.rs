use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::{BinaryOperator, BoolOperator, CompareOperator, Expr, Number, Statement};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

pub fn generate_code(
    statements: &[Statement],
    definitions: &BTreeMap<String, BTreeSet<String>>,
) -> TokenStream {
    let body = generate_function_body(statements, "", definitions);
    quote! {
        fn main() {
            #body
        }
    }
}

fn generate_function_body(
    body: &[Statement],
    function_name: &str,
    definitions: &BTreeMap<String, BTreeSet<String>>,
) -> TokenStream {
    let mut result = TokenStream::new();
    if let Some(definitions) = definitions.get(function_name) {
        for variable in definitions {
            let variable = format_ident!("{}", variable);
            result.append_all(quote! {
                let mut #variable = Value::none();
            });
        }
    }
    for statement in body {
        let statement = format_statement(statement, definitions);
        result.append_all(statement);
    }
    result
}

fn format_statement(
    statement: &Statement,
    definitions: &BTreeMap<String, BTreeSet<String>>,
) -> TokenStream {
    match statement {
        Statement::Assign { target, value } => {
            let target = format_expr(target);
            let value = format_expr(value);
            quote! {
                #target.assign(#value);
            }
        }
        Statement::Expression(expr) => {
            let value = format_expr(expr);
            quote! {
                #value;
            }
        }
        Statement::If { test, body, orelse } => {
            let test = format_expr(test);
            let body = body
                .iter()
                .map(|s| format_statement(s, definitions))
                .collect::<Vec<_>>();
            let orelse = orelse
                .iter()
                .map(|s| format_statement(s, definitions))
                .collect::<Vec<_>>();
            quote! {
                if (#test).test() {
                    #(#body);*
                } else {
                    #(#orelse);*
                }
            }
        }
        Statement::Func { name, args, body } => {
            let args = args
                .iter()
                .map(|arg| format_ident!("{}", arg))
                .collect::<Vec<_>>();
            let body = generate_function_body(body, name, definitions);
            let name = format_ident!("{}", name);
            quote! {
                fn #name( #(#args: Value),*  ) -> Value {
                    #body
                    return Value::none();
                }
            }
        }
        Statement::Return(value) => match value {
            Some(value) => {
                let value = format_expr(value);
                quote! {
                    return #value;
                }
            }
            None => {
                quote! {
                    return Value::none();
                }
            }
        },
        Statement::While { test, body } => {
            let test = format_expr(test);
            let body = body
                .iter()
                .map(|s| format_statement(s, definitions))
                .collect::<Vec<_>>();
            quote! {
                while (#test).test() {
                    #(#body);*
                }
            }
        }
        Statement::Break => quote! { break; },
    }
}

fn format_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::CallFunction { name, args } => {
            let args = format_exprs(args);
            if let Some(macro_name) = name.strip_suffix("__macro__") {
                let name = format_ident!("{}", macro_name);
                quote! {
                    #name !( #(#args .shallow_copy()),* )
                }
            } else {
                let name = format_ident!("{}", name);
                quote! {
                    #name ( #(#args .shallow_copy()),* )
                }
            }
        }
        Expr::CallMethod { value, name, args } => {
            let value = format_expr(value);
            let name = format_ident!("{}", name);
            let args = format_exprs(args);
            quote! {
                #value . #name ( #(#args .shallow_copy()),* )
            }
        }
        Expr::Tuple(values) => {
            let values = format_exprs(values);
            quote! {
               Value::from(&[ #(#values),* ])
            }
        }
        Expr::VariableName(name) => {
            let name = format_ident!("{}", name);
            quote! {
                #name
            }
        }
        Expr::BoolOperation { op, conditions } => {
            let op = format_boolean_operation(op);
            let conditions = format_exprs(conditions);
            let head = &conditions[0];
            let mut result = quote! { #head };
            for condition in conditions.iter().skip(1) {
                result = quote! { #condition . #op (#result .shallow_copy()) };
            }
            result
        }
        Expr::Compare { left, right, op } => {
            let left = format_expr(left);
            let right = format_expr(right);
            let op = format_compare_ident(op);
            quote! { #left . #op (#right.shallow_copy()) }
        }
        Expr::BinaryOperation { left, right, op } => {
            let left = format_expr(left);
            let right = format_expr(right);
            let op = format_binary_ident(op);
            quote! { #left . #op (#right.shallow_copy()) }
        }
        Expr::ConstantNumber(number) => format_number(number),
        Expr::Index { value, index } => {
            let value = format_expr(value);
            let index = format_expr(index);
            quote! {
                #value .index( #index .shallow_copy() )
            }
        }
        Expr::List(list) => {
            let list = format_exprs(list);
            quote! {
                Value::from(vec![#(#list),*])
            }
        }
        Expr::ConstantString(value) => {
            quote! {
                Value::from(#value)
            }
        }
        Expr::ConstantBoolean(b) => {
            if *b {
                quote! {
                    Value::from(true)
                }
            } else {
                quote! {
                    Value::from(false)
                }
            }
        }
    }
}

fn format_exprs(exprs: &[Expr]) -> Vec<TokenStream> {
    exprs.iter().map(|e| format_expr(e)).collect()
}

fn format_boolean_operation(op: &BoolOperator) -> Ident {
    match op {
        BoolOperator::And => format_ident!("__bool_and"),
        BoolOperator::Or => format_ident!("__bool_or"),
    }
}
fn format_compare_ident(op: &CompareOperator) -> Ident {
    match op {
        CompareOperator::Less => format_ident!("is_lt"),
        CompareOperator::LessOrEqual => format_ident!("is_le"),
        CompareOperator::Equal => format_ident!("is_eq"),
        CompareOperator::Greater => format_ident!("is_gt"),
        CompareOperator::NotEqual => format_ident!("is_ne"),
    }
}
fn format_binary_ident(op: &BinaryOperator) -> Ident {
    match op {
        BinaryOperator::Add => format_ident!("__add"),
        BinaryOperator::Sub => format_ident!("__sub"),
        BinaryOperator::Mul => format_ident!("__mul"),
        BinaryOperator::Div => format_ident!("__div"),
        BinaryOperator::Mod => format_ident!("__mod"),
        BinaryOperator::FloorDiv => format_ident!("__floor_div"),
    }
}

fn format_number(number: &Number) -> TokenStream {
    match number {
        Number::Int(int) => match int.parse::<i64>() {
            Ok(int) => {
                quote! {
                    Value::from(#int)
                }
            }
            Err(_) => {
                todo!("bigint is not supported");
            }
        },
        Number::Float(float) => match float.parse::<f64>() {
            Ok(float) => {
                quote! {
                    Value::from(#float)
                }
            }
            Err(e) => {
                panic!("unsupported float value: {} {:?}", float, e);
            }
        },
    }
}
