use anyhow::{anyhow, Result};
use rustpython_parser::ast::StatementType;

use crate::expression::Expression;

#[derive(Debug)]
pub(crate) enum Statement {
    Initialize {
        variables: Vec<String>,
    },
    Expression {
        inner: Expression,
    },
    Assign {
        targets: Vec<Expression>,
        value: Expression,
    },
    If {
        test: Expression,
        body: Vec<Statement>,
        orelse: Option<Vec<Statement>>,
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
            StatementType::If { test, body, orelse } => {
                let test = Expression::interpret(test)?;
                let body = body
                    .iter()
                    .map(|s| Statement::interpret(s))
                    .collect::<Result<Vec<_>>>()?;
                let orelse = match orelse {
                    Some(orelse) => Some(
                        orelse
                            .iter()
                            .map(|s| Statement::interpret(s))
                            .collect::<Result<Vec<_>>>()?,
                    ),
                    None => None,
                };
                Ok(Statement::If { test, body, orelse })
            }
            _ => Err(anyhow!("unimplemented: {:?}", statement.node)),
        }
    }
}
