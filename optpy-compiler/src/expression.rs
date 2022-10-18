use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::ExpressionType;

#[derive(Debug)]
pub(crate) enum Expression {
    Identifier {
        name: String,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    Binop {
        a: Box<Expression>,
        b: Box<Expression>,
        op: Operator,
    },
}

impl Expression {
    pub(crate) fn interpret(expression: &rustpython_parser::ast::Expression) -> Result<Self> {
        match &expression.node {
            ExpressionType::Identifier { name } => Ok(Expression::Identifier { name: name.into() }),
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                if !keywords.is_empty() {
                    return Err(anyhow!("unimplemented keywords: {:?}", keywords));
                }
                let function = Box::new(Expression::interpret(function)?);
                let args = args
                    .iter()
                    .map(|e| Expression::interpret(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Expression::Call { function, args })
            }
            ExpressionType::Binop { a, op, b } => {
                let a = Box::new(Expression::interpret(a)?);
                let b = Box::new(Expression::interpret(b)?);
                let op = Operator::from(op);
                Ok(Expression::Binop { a, b, op })
            }
            _ => Err(anyhow!("unimplemented expression: {:?}", expression.node)),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

impl From<&rustpython_parser::ast::Operator> for Operator {
    fn from(op: &rustpython_parser::ast::Operator) -> Self {
        match op {
            rustpython_parser::ast::Operator::Add => Operator::Add,
            rustpython_parser::ast::Operator::Sub => Operator::Sub,
            rustpython_parser::ast::Operator::Mult => Operator::Mult,
            rustpython_parser::ast::Operator::MatMult => Operator::MatMult,
            rustpython_parser::ast::Operator::Div => Operator::Div,
            rustpython_parser::ast::Operator::Mod => Operator::Mod,
            rustpython_parser::ast::Operator::Pow => Operator::Pow,
            rustpython_parser::ast::Operator::LShift => Operator::LShift,
            rustpython_parser::ast::Operator::RShift => Operator::RShift,
            rustpython_parser::ast::Operator::BitOr => Operator::BitOr,
            rustpython_parser::ast::Operator::BitXor => Operator::BitXor,
            rustpython_parser::ast::Operator::BitAnd => Operator::BitAnd,
            rustpython_parser::ast::Operator::FloorDiv => Operator::FloorDiv,
        }
    }
}

impl From<&Operator> for TokenStream {
    fn from(op: &Operator) -> Self {
        match op {
            Operator::Add => quote! { + },
            Operator::Sub => quote! { - },
            Operator::Mult => todo!(),
            Operator::MatMult => todo!(),
            Operator::Div => todo!(),
            Operator::Mod => todo!(),
            Operator::Pow => todo!(),
            Operator::LShift => todo!(),
            Operator::RShift => todo!(),
            Operator::BitOr => todo!(),
            Operator::BitXor => todo!(),
            Operator::BitAnd => todo!(),
            Operator::FloorDiv => todo!(),
        }
    }
}
