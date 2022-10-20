use anyhow::{anyhow, Result};
use rustpython_parser::ast::{ExpressionType, StatementType};

use crate::expression::{number::Number, Expression};

#[derive(Debug)]
pub(crate) enum Statement {
    Initialize {
        variables: Vec<String>,
    },
    Expression {
        inner: Expression,
    },
    Assign {
        target: Expression,
        value: Expression,
    },
    If {
        test: Expression,
        body: Vec<Statement>,
        orelse: Option<Vec<Statement>>,
    },
}

impl Statement {
    pub(crate) fn interpret(statement: &rustpython_parser::ast::Statement) -> Result<Vec<Self>> {
        match &statement.node {
            StatementType::Expression { expression } => {
                let expression = Expression::interpret(&expression)?;
                Ok(vec![Statement::Expression { inner: expression }])
            }
            StatementType::Assign { targets, value } => {
                assert_eq!(targets.len(), 1);
                let value = Expression::interpret(value)?;
                match &targets[0].node {
                    ExpressionType::Tuple { elements } => {
                        const TMP_VARIABLE_NAME: &str = "__short_live_tmp";
                        let tmp = Expression::Identifier {
                            name: TMP_VARIABLE_NAME.into(),
                        };
                        let mut statements = vec![Statement::Assign {
                            target: tmp.clone(),
                            value,
                        }];
                        for (i, element) in elements.iter().enumerate() {
                            statements.push(Statement::Assign {
                                target: Expression::interpret(element)?,
                                value: Expression::Subscript {
                                    a: Box::new(tmp.clone()),
                                    b: Box::new(Expression::Number {
                                        value: Number::Integer {
                                            value: i.to_string(),
                                        },
                                    }),
                                },
                            });
                        }

                        Ok(statements)
                    }
                    _ => {
                        let target = Expression::interpret(&targets[0])?;
                        Ok(vec![Statement::Assign { target, value }])
                    }
                }
            }
            StatementType::If { test, body, orelse } => {
                let test = Expression::interpret(test)?;
                let body = body
                    .iter()
                    .map(|s| Statement::interpret(s))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect();
                let orelse = match orelse {
                    Some(orelse) => Some(
                        orelse
                            .iter()
                            .map(|s| Statement::interpret(s))
                            .collect::<Result<Vec<_>>>()?
                            .into_iter()
                            .flatten()
                            .collect(),
                    ),
                    None => None,
                };
                Ok(vec![Statement::If { test, body, orelse }])
            }
            _ => Err(anyhow!("unimplemented statement: {:?}", statement.node)),
        }
    }
}
