use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::{
    Assign, BinaryOperation, BinaryOperator, BoolOperation, BoolOperator, CallFunction, CallMethod,
    Compare, CompareOperator, Dict, Expr, Func, If, Index, Number, Statement, UnaryOperation,
    UnaryOperator, While,
};
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

pub fn generate_function_body(
    body: &[Statement],
    function_name: &str,
    definitions: &BTreeMap<String, BTreeSet<String>>,
) -> TokenStream {
    let mut result = TokenStream::new();
    if let Some(definitions) = definitions.get(function_name) {
        for variable in definitions {
            let variable = format_ident!("{}", variable);
            result.append_all(quote! {
                let mut #variable = Object::none();
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
        Statement::Assign(Assign { target, value }) => {
            let target = format_expr(target);
            let value = format_expr(value);
            quote! {
                #target.assign(& #value);
            }
        }
        Statement::Expression(expr) => {
            let value = format_expr(expr);
            quote! {
                #value;
            }
        }
        Statement::If(If { test, body, orelse }) => {
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
        Statement::Func(Func { name, args, body }) => {
            let args = args
                .iter()
                .map(|arg| format_ident!("{}", arg))
                .collect::<Vec<_>>();
            let body = generate_function_body(body, name, definitions);
            let name = format_ident!("{}", name);
            quote! {
                fn #name( #(#args: &Object),*  ) -> Object {
                    #(let mut #args = #args.__shallow_copy();)*
                    #body
                    return Object::none();
                }
            }
        }
        Statement::Return(value) => match value {
            Some(value) => {
                let value = format_expr(value);
                quote! {
                    return Object::from(#value);
                }
            }
            None => {
                quote! {
                    return Object::none();
                }
            }
        },
        Statement::While(While { test, body }) => {
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
        Statement::Continue => quote! { continue; },
    }
}

fn format_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::CallFunction(CallFunction { name, args }) => {
            let args = format_exprs(args);
            if let Some(macro_name) = name.strip_suffix("__macro__") {
                let name = format_ident!("{}", macro_name);
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
        Expr::CallMethod(CallMethod { value, name, args }) => {
            let value = format_expr(value);
            let name = format_ident!("{}", name);
            let args = format_exprs(args);
            quote! {
                #value . #name ( #(&#args),* )
            }
        }
        Expr::Tuple(values) => {
            let values = format_exprs(values);
            quote! {
               Object::from(&[ #(#values),* ])
            }
        }
        Expr::VariableName(name) => {
            let name = format_ident!("{}", name);
            quote! {
                #name
            }
        }
        Expr::BoolOperation(BoolOperation { op, conditions }) => {
            let op = format_boolean_operation(op);
            let conditions = format_exprs(conditions);

            let mut result = TokenStream::new();
            for (i, condition) in conditions.iter().enumerate() {
                if i > 0 {
                    result.append_all(quote! { # op });
                }
                result.append_all(quote! { #condition .test() });
            }
            quote! { Object::from(#result) }
        }
        Expr::Compare(Compare { left, right, op }) => {
            let left = format_expr(left);
            let right = format_expr(right);
            let op = format_compare_ident(op);
            quote! { #left . #op (&#right) }
        }
        Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
            let left = format_expr(left);
            let right = format_expr(right);
            let op = format_binary_ident(op);
            quote! { #left . #op (&#right) }
        }
        Expr::ConstantNumber(number) => format_number(number),
        Expr::Index(Index { value, index }) => {
            let value = format_expr(value);
            let index = format_expr(index);
            quote! {
                #value .index(& #index )
            }
        }
        Expr::List(list) => {
            let list = format_exprs(list);
            quote! {
                Object::from(vec![#(Object::from(&#list)),*])
            }
        }
        Expr::Dict(Dict { pairs }) => {
            let pairs = pairs
                .iter()
                .map(|(key, value)| {
                    let key = format_expr(key);
                    let value = format_expr(value);
                    quote! {
                        (#key, #value)
                    }
                })
                .collect::<Vec<_>>();
            quote! {
                Object::dict(vec![ #(#pairs),* ])
            }
        }
        Expr::ConstantString(value) => {
            quote! {
                Object::from(#value)
            }
        }
        Expr::ConstantBoolean(b) => {
            if *b {
                quote! {
                    Object::from(true)
                }
            } else {
                quote! {
                    Object::from(false)
                }
            }
        }
        Expr::UnaryOperation(UnaryOperation { value, op }) => {
            let value = format_expr(value);
            let op = format_unary_ident(op);
            quote! {
                #value . #op ()
            }
        }
    }
}

fn format_exprs(exprs: &[Expr]) -> Vec<TokenStream> {
    exprs.iter().map(|e| format_expr(e)).collect()
}

fn format_boolean_operation(op: &BoolOperator) -> TokenStream {
    match op {
        BoolOperator::And => quote! { && },
        BoolOperator::Or => quote! { || },
    }
}
fn format_compare_ident(op: &CompareOperator) -> Ident {
    match op {
        CompareOperator::Less => format_ident!("__lt"),
        CompareOperator::LessOrEqual => format_ident!("__le"),
        CompareOperator::Greater => format_ident!("__gt"),
        CompareOperator::GreaterOrEqual => format_ident!("__ge"),
        CompareOperator::Equal => format_ident!("__eq"),
        CompareOperator::NotEqual => format_ident!("__ne"),
    }
}
fn format_binary_ident(op: &BinaryOperator) -> Ident {
    match op {
        BinaryOperator::Add => format_ident!("__add"),
        BinaryOperator::Sub => format_ident!("__sub"),
        BinaryOperator::Mul => format_ident!("__mul"),
        BinaryOperator::Div => format_ident!("__div"),
        BinaryOperator::Mod => format_ident!("__rem"),
        BinaryOperator::FloorDiv => format_ident!("__floor_div"),
    }
}
fn format_unary_ident(op: &UnaryOperator) -> Ident {
    match op {
        UnaryOperator::Add => format_ident!("__unary_add"),
        UnaryOperator::Sub => format_ident!("__unary_sub"),
    }
}

fn format_number(number: &Number) -> TokenStream {
    match number {
        Number::Int(int) => match int.parse::<i64>() {
            Ok(int) => {
                quote! {
                    Object::from(#int)
                }
            }
            Err(_) => {
                todo!("bigint is not supported");
            }
        },
        Number::Float(float) => match float.parse::<f64>() {
            Ok(float) => {
                quote! {
                    Object::from(#float)
                }
            }
            Err(e) => {
                panic!("unsupported float value: {} {:?}", float, e);
            }
        },
    }
}
