use anyhow::{anyhow, Result};
use rustpython_parser::ast::StatementType;

use crate::expression::Expression;

#[derive(Debug)]
pub(crate) enum Statement {
    Expression {
        inner: Expression,
    },
    Assign {
        targets: Vec<Expression>,
        value: Expression,
    },
}

impl Statement {
    pub(crate) fn interpret(statement: &rustpython_parser::ast::Statement) -> Result<Self> {
        match &statement.node {
            StatementType::Expression { expression } => {
                let expression = Expression::interpret(&expression)?;
                Ok(Statement::Expression { inner: expression })
            }
            StatementType::Assign { targets, value } => {
                let targets = targets
                    .iter()
                    .map(|e| Expression::interpret(e))
                    .collect::<Result<Vec<_>>>()?;
                let value = Expression::interpret(value)?;
                Ok(Statement::Assign { targets, value })
            }
            _ => Err(anyhow!("unimplemented: {:?}", statement.node)),
        }
    }
}
