use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::{BinaryOperator, BoolOperator, CompareOperator, Expr, Number, Statement};
use proc_macro2::TokenStream;
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
                let mut #variable = Value::None;
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
                #target .assign( #value );
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
                if #test {
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
                    return Value::None;
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
                    return Value::None;
                }
            }
        },
    }
}

fn format_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::CallFunction { name, args } => {
            let name = format_ident!("{}", name);
            let args = format_exprs(args);
            quote! {
                #name ( #(#args),* )
            }
        }
        Expr::CallMethod { value, name, args } => {
            let value = format_expr(value);
            let name = format_ident!("{}", name);
            let args = format_exprs(args);
            quote! {
                #value . #name ( #(#args),* )
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
            let op = format_boolean_operator(op);
            let conditions = format_exprs(conditions);
            let mut result = TokenStream::new();
            for (i, condition) in conditions.iter().enumerate() {
                if i > 0 {
                    result.append_all(quote! { #op });
                }
                result.append_all(quote! { #condition })
            }
            result
        }
        Expr::Compare { left, right, op } => {
            let left = format_expr(left);
            let right = format_expr(right);
            let op = format_compare_operator(op);
            quote! { #left #op #right }
        }
        Expr::BinaryOperation { left, right, op } => {
            let left = format_expr(left);
            let right = format_expr(right);
            let op = format_binary_operator(op);
            quote! { #left #op #right }
        }
        Expr::Number(number) => format_number(number),
        Expr::Index { value, index } => {
            let value = format_expr(value);
            let index = format_expr(index);
            quote! {
                #value .index( #index )
            }
        }
    }
}

fn format_exprs(exprs: &[Expr]) -> Vec<TokenStream> {
    exprs.iter().map(|e| format_expr(e)).collect()
}

fn format_boolean_operator(op: &BoolOperator) -> TokenStream {
    match op {
        BoolOperator::And => quote! { && },
        BoolOperator::Or => quote! { || },
    }
}
fn format_compare_operator(op: &CompareOperator) -> TokenStream {
    match op {
        CompareOperator::Less => quote! { < },
        CompareOperator::LessOrEqual => quote! { <= },
    }
}
fn format_binary_operator(op: &BinaryOperator) -> TokenStream {
    match op {
        BinaryOperator::Add => quote! { + },
    }
}

fn format_number(number: &Number) -> TokenStream {
    match number {
        Number::Int(int) => match int.parse::<i64>() {
            Ok(int) => {
                quote! {
                    #int
                }
            }
            Err(_) => {
                todo!("bigint is not supported");
            }
        },
        Number::Float(float) => match float.parse::<f64>() {
            Ok(float) => {
                quote! {
                    #float
                }
            }
            Err(e) => {
                panic!("unsupported float value: {} {:?}", float, e);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use optpy_parser::parse;
    use optpy_resolver::{resolve_function_calls, resolve_names};
    use optpy_test_helper::StripMargin;

    use super::*;

    #[test]
    #[ignore]
    fn test_aa() {
        let code = r"
            |__v0, __v1 = map(int, input().split())
            |__v2 = __v0 + __v1
            |def __f0(__v3, __v1):
            |    def __f1(__v4, __v1, __v2):
            |        __v5 = __v1 + __v2
            |        return __v4 + __v5
            |    return __f1(__v1, __v1, __v2) + __v3
            |__v6 = __f0(__v0 + __v1 + __v2, __v1)
            |print(__v6)"
            .strip_margin();
        let ast = parse(code).unwrap();
        let ast = resolve_names(&ast);
        let (ast, definitions) = resolve_function_calls(&ast);
        let code = generate_code(&ast, &definitions);
        assert_eq!(
            code.to_string(),
            quote! {
                fn main() {
                    let mut __v0 = Value::None;
                    let mut __v1 = Value::None;
                    let mut __v2 = Value::None;
                    let mut __v9 = Value::None;
                    let mut int = Value::None;
                    Value::tuple(&[__v0, __v1]).assign(map(int, input().split()));
                    __v2.assign(__v0 + __v1);
                    fn __f0(__v3: Value, __v4: Value, __v2: Value) -> Value {
                        fn __f1(__v5: Value, __v6: Value, __v7: Value) -> Value {
                            let mut __v8 = Value::None;
                            __v8.assign(__v6 + __v7);
                            return __v5 + __v8;
                            return Value::None;
                        }
                        return __f1(__v4, __v4, __v2) + __v3;
                        return Value::None;
                    }
                    __v9.assign(__f0(__v0 + __v1 + __v2, __v1, __v2));
                    print(__v9);
                }
            }
            .to_string()
        );
    }
}
