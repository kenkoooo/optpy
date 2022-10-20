mod comparison;
pub(crate) mod number;
mod operator;

use anyhow::{anyhow, Result};
use rustpython_parser::ast::ExpressionType;

use self::{comparison::Comparison, number::Number, operator::Operator};

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
    Subscript {
        a: Box<Expression>,
        b: Box<Expression>,
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
            ExpressionType::Subscript { a, b } => {
                let a = Box::new(Expression::interpret(a)?);
                let b = Box::new(Expression::interpret(b)?);
                Ok(Expression::Subscript { a, b })
            }
            _ => Err(anyhow!("unimplemented expression: {:?}", expression.node)),
        }
    }
}
