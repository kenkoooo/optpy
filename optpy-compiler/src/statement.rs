use anyhow::{anyhow, Result};
use rustpython_parser::ast::{ExpressionType, StatementType};

use crate::expression::{number::Number, OptpyExpression};

#[derive(Debug)]
pub(crate) enum OptpyStatement {
    Initialize {
        variables: Vec<String>,
    },
    Expression {
        inner: OptpyExpression,
    },
    Assign {
        target: OptpyExpression,
        value: OptpyExpression,
    },
    If {
        test: OptpyExpression,
        body: Vec<OptpyStatement>,
        orelse: Option<Vec<OptpyStatement>>,
    },
    For {
        target: OptpyExpression,
        iter: OptpyExpression,
        body: Vec<OptpyStatement>,
    },
}

impl OptpyStatement {
    pub(crate) fn interpret(statement: &rustpython_parser::ast::Statement) -> Result<Vec<Self>> {
        match &statement.node {
            StatementType::Expression { expression } => {
                let expression = OptpyExpression::interpret(&expression)?;
                Ok(vec![OptpyStatement::Expression { inner: expression }])
            }
            StatementType::Assign { targets, value } => {
                assert_eq!(targets.len(), 1);
                let value = OptpyExpression::interpret(value)?;
                match &targets[0].node {
                    ExpressionType::Tuple { elements } => {
                        const TMP_VARIABLE_NAME: &str = "__short_live_tmp";
                        let tmp = OptpyExpression::Identifier {
                            name: TMP_VARIABLE_NAME.into(),
                        };
                        let mut statements = vec![OptpyStatement::Assign {
                            target: tmp.clone(),
                            value,
                        }];
                        for (i, element) in elements.iter().enumerate() {
                            statements.push(OptpyStatement::Assign {
                                target: OptpyExpression::interpret(element)?,
                                value: OptpyExpression::Subscript {
                                    a: Box::new(tmp.clone()),
                                    b: Box::new(OptpyExpression::Number {
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
                        let target = OptpyExpression::interpret(&targets[0])?;
                        Ok(vec![OptpyStatement::Assign { target, value }])
                    }
                }
            }
            StatementType::If { test, body, orelse } => {
                let test = OptpyExpression::interpret(test)?;
                let body = interpret_statements(body)?;
                let orelse = match orelse {
                    Some(orelse) => Some(interpret_statements(orelse)?),
                    None => None,
                };
                Ok(vec![OptpyStatement::If { test, body, orelse }])
            }
            StatementType::For {
                is_async,
                target,
                iter,
                body,
                orelse,
            } => {
                if *is_async {
                    return Err(anyhow!("async-for is not supported"));
                }
                if orelse.is_some() {
                    return Err(anyhow!("for-else is not supported"));
                }

                let target = OptpyExpression::interpret(target)?;
                let iter = OptpyExpression::interpret(iter)?;
                let body = interpret_statements(body)?;
                Ok(vec![OptpyStatement::For { target, iter, body }])
            }
            _ => Err(anyhow!("unimplemented statement: {:?}", statement.node)),
        }
    }
}

fn interpret_statements(
    statements: &[rustpython_parser::ast::Statement],
) -> Result<Vec<OptpyStatement>> {
    Ok(statements
        .iter()
        .map(|s| OptpyStatement::interpret(s))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}
