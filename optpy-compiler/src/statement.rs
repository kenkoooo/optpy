use anyhow::{anyhow, Result};
use rustpython_parser::ast::StatementType;

use crate::expression::OptpyExpression;

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
    FunctionDef {
        name: String,
        body: Vec<OptpyStatement>,
        args: Vec<String>,
    },
    Return {
        value: Option<OptpyExpression>,
    },
}

impl OptpyStatement {
    pub(crate) fn load(statement: &rustpython_parser::ast::Statement) -> Result<Vec<Self>> {
        match &statement.node {
            StatementType::Expression { expression } => {
                let expression = OptpyExpression::load(&expression)?;
                Ok(vec![OptpyStatement::Expression { inner: expression }])
            }
            StatementType::Assign { targets, value } => {
                assert_eq!(targets.len(), 1);
                let value = OptpyExpression::load(value)?;
                let target = OptpyExpression::load(&targets[0])?;
                Ok(vec![OptpyStatement::Assign { target, value }])
            }
            StatementType::If { test, body, orelse } => {
                let test = OptpyExpression::load(test)?;
                let body = load_statements(body)?;
                let orelse = match orelse {
                    Some(orelse) => Some(load_statements(orelse)?),
                    None => None,
                };
                Ok(vec![OptpyStatement::If { test, body, orelse }])
            }
            StatementType::For {
                is_async: _,
                orelse: _,
                target,
                iter,
                body,
            } => {
                let iter = OptpyExpression::load(iter)?;
                let body = load_statements(body)?;
                let target = OptpyExpression::load(target)?;
                Ok(vec![OptpyStatement::For { target, iter, body }])
            }
            StatementType::FunctionDef {
                is_async: _,
                decorator_list: _,
                returns: _,
                name,
                args,
                body,
            } => {
                let body = load_statements(body)?;
                let args = args.args.iter().map(|a| a.arg.clone()).collect::<Vec<_>>();
                Ok(vec![OptpyStatement::FunctionDef {
                    name: name.to_string(),
                    body,
                    args,
                }])
            }
            StatementType::Return { value } => {
                let value = match value {
                    Some(v) => Some(OptpyExpression::load(v)?),
                    None => None,
                };
                Ok(vec![OptpyStatement::Return { value }])
            }
            _ => Err(anyhow!("unimplemented statement: {:?}", statement.node)),
        }
    }
}

fn load_statements(
    statements: &[rustpython_parser::ast::Statement],
) -> Result<Vec<OptpyStatement>> {
    Ok(statements
        .iter()
        .map(|s| OptpyStatement::load(s))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}
