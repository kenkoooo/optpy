use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::{
    Assign, BinaryOperation, BinaryOperator, BoolOperation, BoolOperator, CallFunction, CallMethod,
    Compare, CompareOperator, Dict, Expr, Func, If, Index, Number, Statement, UnaryOperation,
    UnaryOperator, While,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

pub struct CodeGenerator {
    pub function_formatter: fn(&str, args: &[String], body: TokenStream) -> TokenStream,
    pub constant_boolean_formatter: fn(bool) -> TokenStream,
    pub declaration_formatter: fn(&str) -> TokenStream,
    pub tuple_formatter: fn(&[TokenStream]) -> TokenStream,
    pub list_formatter: fn(&[TokenStream]) -> TokenStream,
    pub string_formatter: fn(&str) -> TokenStream,
    pub dict_formatter: fn(&[(TokenStream, TokenStream)]) -> TokenStream,
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self {
            function_formatter: format_function,
            constant_boolean_formatter: format_constant_boolean,
            declaration_formatter: format_declaration,
            tuple_formatter: format_tuple,
            list_formatter: format_list,
            string_formatter: format_string,
            dict_formatter: format_dict,
        }
    }
}

fn format_function(name: &str, args: &[String], body: TokenStream) -> TokenStream {
    let args = args
        .iter()
        .map(|arg| format_ident!("{}", arg))
        .collect::<Vec<_>>();
    let name = format_ident!("{}", name);
    quote! {
        fn #name(#(mut #args: Value),*) -> Value {
            #body
            return Default::default();
        }
    }
}

fn format_constant_boolean(b: bool) -> TokenStream {
    if b {
        quote! {
            Value::from(true)
        }
    } else {
        quote! {
            Value::from(false)
        }
    }
}

fn format_declaration(variable: &str) -> TokenStream {
    let variable = format_ident!("{}", variable);
    quote! {
        let mut #variable = Value::default();
    }
}

fn format_tuple(tuple: &[TokenStream]) -> TokenStream {
    let tuple = tuple.iter().map(format_arg_expr).collect::<Vec<_>>();
    quote! {
       Value::from(vec![ #(Value::from(#tuple)),* ])
    }
}
fn format_list(list: &[TokenStream]) -> TokenStream {
    let list = list.iter().map(format_arg_expr).collect::<Vec<_>>();
    quote! {
       Value::from(vec![ #(Value::from(#list)),* ])
    }
}
fn format_string(s: &str) -> TokenStream {
    quote! {
        Value::from(#s)
    }
}
fn format_arg_expr(expr: &TokenStream) -> TokenStream {
    fn is_ident(token: &TokenStream) -> bool {
        token
            .to_string()
            .chars()
            .all(|c| matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '_'))
    }

    if is_ident(expr) {
        quote! {
            #expr.__shallow_copy()
        }
    } else {
        quote! {
            #expr
        }
    }
}

fn format_dict(pairs: &[(TokenStream, TokenStream)]) -> TokenStream {
    let pairs = pairs
        .iter()
        .map(|(key, value)| {
            let key = format_arg_expr(key);
            let value = format_arg_expr(value);
            quote! {
                (Value::from(#key), Value::from(#value))
            }
        })
        .collect::<Vec<_>>();

    quote! {
        Value::dict(vec![ #(#pairs),* ])
    }
}

impl CodeGenerator {
    pub fn generate_function_body(
        &self,
        body: &[Statement],
        function_name: &str,
        definitions: &BTreeMap<String, BTreeSet<String>>,
    ) -> TokenStream {
        let mut result = TokenStream::new();
        if let Some(definitions) = definitions.get(function_name) {
            for variable in definitions {
                result.append_all((self.declaration_formatter)(variable));
            }
        }
        for statement in body {
            let statement = self.format_statement(statement, definitions);
            result.append_all(statement);
        }
        result
    }

    fn format_statement(
        &self,
        statement: &Statement,
        definitions: &BTreeMap<String, BTreeSet<String>>,
    ) -> TokenStream {
        match statement {
            Statement::Assign(Assign { target, value }) => {
                let target = self.format_expr(target, true);
                let value = self.format_expr(value, false);
                let value = format_arg_expr(&value);
                quote! {
                    #target.assign(#value);
                }
            }
            Statement::Expression(expr) => {
                let value = self.format_expr(expr, false);
                quote! {
                    #value;
                }
            }
            Statement::If(If { test, body, orelse }) => {
                let test = self.format_expr(test, false);
                let body = body
                    .iter()
                    .map(|s| self.format_statement(s, definitions))
                    .collect::<Vec<_>>();
                let orelse = orelse
                    .iter()
                    .map(|s| self.format_statement(s, definitions))
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
                let body = self.generate_function_body(body, name, definitions);
                (self.function_formatter)(name, args, body)
            }
            Statement::Return(value) => match value {
                Some(value) => {
                    let value = self.format_expr(value, false);
                    quote! {
                        return #value;
                    }
                }
                None => {
                    quote! {
                        return Default::default();
                    }
                }
            },
            Statement::While(While { test, body }) => {
                let test = self.format_expr(test, false);
                let body = body
                    .iter()
                    .map(|s| self.format_statement(s, definitions))
                    .collect::<Vec<_>>();
                quote! {
                    while (#test).test() {
                        #(#body);*
                    }
                }
            }
            Statement::Break => quote! { break; },
            Statement::Continue => quote! { continue; },
            Statement::Import(_) | Statement::FromImport(_) => unreachable!(),
        }
    }

    fn format_expr(&self, expr: &Expr, assign_lhs: bool) -> TokenStream {
        match expr {
            Expr::CallFunction(CallFunction { name, args }) => {
                let args = self.format_exprs(args);
                let args = args.iter().map(format_arg_expr).collect::<Vec<_>>();
                if let Some(macro_name) = name.strip_suffix("__macro__") {
                    let name = format_ident!("{}", macro_name);
                    quote! {
                        #name !( #(#args),* )
                    }
                } else {
                    let name = format_ident!("{}", name);
                    quote! {
                        #name ( #(#args),* )
                    }
                }
            }
            Expr::CallMethod(CallMethod { value, name, args }) => {
                let value = self.format_expr(value, false);
                let name = format_ident!("{}", name);
                let args = self.format_exprs(args);
                let args = args.iter().map(format_arg_expr).collect::<Vec<_>>();
                quote! {
                    #value . #name ( #(#args),* )
                }
            }
            Expr::Tuple(values) => {
                let list = self.format_exprs(values);
                (self.tuple_formatter)(&list)
            }
            Expr::VariableName(name) => {
                let name = format_ident!("{}", name);
                quote! {
                    #name
                }
            }
            Expr::BoolOperation(BoolOperation { op, conditions }) => {
                let op = self.format_boolean_operation(op);
                let conditions = self.format_exprs(conditions);

                let mut result = TokenStream::new();
                for (i, condition) in conditions.iter().enumerate() {
                    if i > 0 {
                        result.append_all(quote! { # op });
                    }
                    result.append_all(quote! { #condition .test() });
                }
                quote! { Value::from(#result) }
            }
            Expr::Compare(Compare { left, right, op }) => {
                let left = self.format_expr(left, false);
                let right = self.format_expr(right, false);
                let right = format_arg_expr(&right);
                let op = self.format_compare_ident(op);
                quote! { #left . #op (#right) }
            }
            Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
                let left = self.format_expr(left, false);
                let right = self.format_expr(right, false);
                let right = format_arg_expr(&right);
                let op = self.format_binary_ident(op);
                quote! { #left . #op (#right) }
            }
            Expr::ConstantNumber(number) => self.format_number(number),
            Expr::None => {
                quote! {
                    Value::default()
                }
            }
            Expr::Index(Index { value, index }) => {
                let value = self.format_expr(value, assign_lhs);
                let index = self.format_expr(index, false);
                let index = format_arg_expr(&index);
                if assign_lhs {
                    quote! {
                        #value .__index_ref(#index)
                    }
                } else {
                    quote! {
                        #value .__index_value(#index)
                    }
                }
            }
            Expr::List(list) => {
                let list = self.format_exprs(list);
                (self.list_formatter)(&list)
            }
            Expr::Dict(Dict { pairs }) => {
                let pairs = pairs
                    .iter()
                    .map(|(key, value)| {
                        let key = self.format_expr(key, false);
                        let value = self.format_expr(value, false);
                        (key, value)
                    })
                    .collect::<Vec<_>>();
                (self.dict_formatter)(&pairs)
            }
            Expr::ConstantString(value) => (self.string_formatter)(value),
            Expr::ConstantBoolean(b) => (self.constant_boolean_formatter)(*b),
            Expr::UnaryOperation(UnaryOperation { value, op }) => {
                let value = self.format_expr(value, false);
                let op = self.format_unary_ident(op);
                quote! {
                    #value . #op ()
                }
            }
        }
    }

    fn format_exprs(&self, exprs: &[Expr]) -> Vec<TokenStream> {
        exprs.iter().map(|e| self.format_expr(e, false)).collect()
    }

    fn format_boolean_operation(&self, op: &BoolOperator) -> TokenStream {
        match op {
            BoolOperator::And => quote! { && },
            BoolOperator::Or => quote! { || },
        }
    }
    fn format_compare_ident(&self, op: &CompareOperator) -> Ident {
        match op {
            CompareOperator::Less => format_ident!("__lt"),
            CompareOperator::LessOrEqual => format_ident!("__le"),
            CompareOperator::Greater => format_ident!("__gt"),
            CompareOperator::GreaterOrEqual => format_ident!("__ge"),
            CompareOperator::Equal => format_ident!("__eq"),
            CompareOperator::NotEqual => format_ident!("__ne"),
            CompareOperator::In => format_ident!("__in"),
            CompareOperator::NotIn => format_ident!("__not_in"),
        }
    }
    fn format_binary_ident(&self, op: &BinaryOperator) -> Ident {
        match op {
            BinaryOperator::Add => format_ident!("__add"),
            BinaryOperator::Sub => format_ident!("__sub"),
            BinaryOperator::Mul => format_ident!("__mul"),
            BinaryOperator::Div => format_ident!("__div"),
            BinaryOperator::Mod => format_ident!("__rem"),
            BinaryOperator::FloorDiv => format_ident!("__floor_div"),
            BinaryOperator::Pow => format_ident!("__pow"),
            BinaryOperator::BitAnd => format_ident!("__bit_and"),
            BinaryOperator::LeftShift => format_ident!("__left_shift"),
            BinaryOperator::RightShift => format_ident!("__right_shift"),
        }
    }
    fn format_unary_ident(&self, op: &UnaryOperator) -> Ident {
        match op {
            UnaryOperator::Add => format_ident!("__unary_add"),
            UnaryOperator::Sub => format_ident!("__unary_sub"),
            UnaryOperator::Not => format_ident!("__unary_not"),
        }
    }

    fn format_number(&self, number: &Number) -> TokenStream {
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
}
