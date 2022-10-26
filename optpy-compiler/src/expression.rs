mod comparison;
pub(crate) mod number;
mod operator;

use anyhow::{anyhow, Result};
use rustpython_parser::ast::ExpressionType;

use self::{comparison::Comparison, number::Number, operator::Operator};

#[derive(Debug, Clone)]
pub(crate) enum OptpyExpression {
    Identifier {
        name: String,
    },
    Call {
        function: Box<OptpyExpression>,
        args: Vec<OptpyExpression>,
    },
    Binop {
        a: Box<OptpyExpression>,
        b: Box<OptpyExpression>,
        op: Operator,
    },
    Tuple {
        elements: Vec<OptpyExpression>,
    },
    Attribute {
        value: Box<OptpyExpression>,
        name: String,
    },
    Compare {
        values: Vec<OptpyExpression>,
        ops: Vec<Comparison>,
    },
    Number {
        value: Number,
    },
    Subscript {
        a: Box<OptpyExpression>,
        b: Box<OptpyExpression>,
    },
}

impl OptpyExpression {
    pub(crate) fn load(expression: &rustpython_parser::ast::Expression) -> Result<Self> {
        match &expression.node {
            ExpressionType::Identifier { name } => {
                Ok(OptpyExpression::Identifier { name: name.into() })
            }
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                if !keywords.is_empty() {
                    return Err(anyhow!("unimplemented keywords: {:?}", keywords));
                }
                let function = Box::new(OptpyExpression::load(function)?);
                let args = args
                    .iter()
                    .map(|e| OptpyExpression::load(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(OptpyExpression::Call { function, args })
            }
            ExpressionType::Binop { a, op, b } => {
                let a = Box::new(OptpyExpression::load(a)?);
                let b = Box::new(OptpyExpression::load(b)?);
                let op = Operator::from(op);
                Ok(OptpyExpression::Binop { a, b, op })
            }
            ExpressionType::Tuple { elements } => {
                let elements = elements
                    .iter()
                    .map(|e| OptpyExpression::load(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(OptpyExpression::Tuple { elements })
            }
            ExpressionType::Attribute { value, name } => {
                let value = Box::new(OptpyExpression::load(value)?);
                Ok(OptpyExpression::Attribute {
                    value,
                    name: name.into(),
                })
            }
            ExpressionType::Compare { vals, ops } => {
                let values = vals
                    .iter()
                    .map(|e| OptpyExpression::load(e))
                    .collect::<Result<Vec<_>>>()?;
                let ops = ops.iter().map(|c| Comparison::from(c)).collect::<Vec<_>>();
                Ok(OptpyExpression::Compare { values, ops })
            }
            ExpressionType::Number { value } => Ok(OptpyExpression::Number {
                value: Number::from(value),
            }),
            ExpressionType::Subscript { a, b } => {
                let a = Box::new(OptpyExpression::load(a)?);
                let b = Box::new(OptpyExpression::load(b)?);
                Ok(OptpyExpression::Subscript { a, b })
            }
            _ => Err(anyhow!("unimplemented expression: {:?}", expression.node)),
        }
    }
}
