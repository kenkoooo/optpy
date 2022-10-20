use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use rustpython_parser::ast::ExpressionType;

#[derive(Debug, Clone)]
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
    Tuple {
        elements: Vec<Expression>,
    },
    Attribute {
        value: Box<Expression>,
        name: String,
    },
    Compare {
        values: Vec<Expression>,
        ops: Vec<Comparison>,
    },
    Number {
        value: Number,
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
            ExpressionType::Tuple { elements } => {
                let elements = elements
                    .iter()
                    .map(|e| Expression::interpret(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Expression::Tuple { elements })
            }
            ExpressionType::Attribute { value, name } => {
                let value = Box::new(Expression::interpret(value)?);
                Ok(Expression::Attribute {
                    value,
                    name: name.into(),
                })
            }
            ExpressionType::Compare { vals, ops } => {
                let values = vals
                    .iter()
                    .map(|e| Expression::interpret(e))
                    .collect::<Result<Vec<_>>>()?;
                let ops = ops.iter().map(|c| Comparison::from(c)).collect::<Vec<_>>();
                Ok(Expression::Compare { values, ops })
            }
            ExpressionType::Number { value } => Ok(Expression::Number {
                value: Number::from(value),
            }),
            _ => Err(anyhow!("unimplemented expression: {:?}", expression.node)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

impl ToTokens for Operator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Operator::Add => tokens.append_all(quote! { + }),
            Operator::Sub => tokens.append_all(quote! { - }),
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum Comparison {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    In,
    NotIn,
    Is,
    IsNot,
}

impl From<&rustpython_parser::ast::Comparison> for Comparison {
    fn from(c: &rustpython_parser::ast::Comparison) -> Self {
        match c {
            rustpython_parser::ast::Comparison::Equal => Comparison::Equal,
            rustpython_parser::ast::Comparison::NotEqual => Comparison::NotEqual,
            rustpython_parser::ast::Comparison::Less => Comparison::Less,
            rustpython_parser::ast::Comparison::LessOrEqual => Comparison::LessOrEqual,
            rustpython_parser::ast::Comparison::Greater => Comparison::Greater,
            rustpython_parser::ast::Comparison::GreaterOrEqual => Comparison::GreaterOrEqual,
            rustpython_parser::ast::Comparison::In => Comparison::In,
            rustpython_parser::ast::Comparison::NotIn => Comparison::NotIn,
            rustpython_parser::ast::Comparison::Is => Comparison::Is,
            rustpython_parser::ast::Comparison::IsNot => Comparison::IsNot,
        }
    }
}

impl ToTokens for Comparison {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Comparison::Equal => todo!(),
            Comparison::NotEqual => todo!(),
            Comparison::Less => tokens.append_all(quote! { < }),
            Comparison::LessOrEqual => tokens.append_all(quote! { <= }),
            Comparison::Greater => todo!(),
            Comparison::GreaterOrEqual => todo!(),
            Comparison::In => todo!(),
            Comparison::NotIn => todo!(),
            Comparison::Is => todo!(),
            Comparison::IsNot => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Number {
    Integer { value: String },
    Float { value: f64 },
}

impl From<&rustpython_parser::ast::Number> for Number {
    fn from(v: &rustpython_parser::ast::Number) -> Self {
        match v {
            rustpython_parser::ast::Number::Integer { value } => Number::Integer {
                value: value.to_string(),
            },
            rustpython_parser::ast::Number::Float { value } => Number::Float { value: *value },
            rustpython_parser::ast::Number::Complex { .. } => todo!(),
        }
    }
}

impl ToTokens for Number {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Number::Integer { value } => {
                let value = value.to_string();
                tokens.append_all(quote! {
                    Value::integer(#value)
                });
            }
            Number::Float { value } => tokens.append_all(quote! { Value::float(#value) }),
        }
    }
}
