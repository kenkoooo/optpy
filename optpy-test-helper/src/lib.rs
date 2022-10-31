mod string;
pub use string::StripMargin;

use optpy_parser::{BinaryOperator, Expr, Statement};

pub fn assign(target: Expr, value: Expr) -> Statement {
    Statement::Assign { target, value }
}

pub fn ident<S: AsRef<str>>(ident: S) -> Expr {
    Expr::Ident(ident.as_ref().into())
}
pub fn expr(expr: Expr) -> Statement {
    Statement::Expression(expr)
}

pub fn func<S: AsRef<str>>(name: S, args: Vec<S>, body: Vec<Statement>) -> Statement {
    Statement::Func {
        name: name.as_ref().into(),
        args: args.into_iter().map(|s| s.as_ref().into()).collect(),
        body,
    }
}

pub fn bin_op(left: Expr, op: BinaryOperator, right: Expr) -> Expr {
    Expr::BinaryOperation {
        left: Box::new(left),
        right: Box::new(right),
        op,
    }
}

#[macro_export]
macro_rules! tuple {
    ($($x:expr),+) => {
        optpy_parser::Expr::Tuple(vec![$($x),+])
    };
}

#[macro_export]
macro_rules! call_fn {
    ($name:expr, $($x:expr),+) => {
        optpy_parser::Expr::CallFunction {
            name: ($name).into(),
            args: vec![$($x),+],
        }
    };
    ($name:expr) => {
        optpy_parser::Expr::CallFunction {
            name: ($name).into(),
            args: vec![],
        }
    };
}

#[macro_export]
macro_rules! call_method {
    ($value:expr, $name:expr, $($x:expr),+) => {
        optpy_parser::Expr::CallMethod {
            value: Box::new($value),
            name: ($name).into(),
            args: vec![$($x),+],
        }
    };
    ($value:expr, $name:expr) => {
        optpy_parser::Expr::CallMethod {
            value: Box::new($value),
            name: ($name).into(),
            args: vec![],
        }
    };
}

#[macro_export]
macro_rules! returns {
    ($value:expr) => {
        optpy_parser::Statement::Return(Some($value))
    };
    () => {
        optpy_parser::Statement::Return(None)
    };
}
