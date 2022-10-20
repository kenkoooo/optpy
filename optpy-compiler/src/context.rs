use std::collections::BTreeMap;

use anyhow::{anyhow, Result};

use crate::{expression::Expression, statement::Statement};

pub(crate) fn collect_declared_variables(
    statements: &[Statement],
    ctx: &mut Vec<String>,
    store: &mut IdentifierStore,
) {
    for statement in statements {
        match statement {
            Statement::Assign { target, .. } => {
                collect_from_tuple(target, ctx, store);
            }
            Statement::If { body, orelse, .. } => {
                collect_declared_variables(body, ctx, store);
                if let Some(orelse) = orelse {
                    collect_declared_variables(orelse, ctx, store);
                }
            }
            _ => {
                continue;
            }
        }
    }
}

fn collect_from_tuple(expr: &Expression, ctx: &Vec<String>, store: &mut IdentifierStore) {
    match expr {
        Expression::Identifier { name } => {
            store.declare(ctx, name);
        }
        Expression::Tuple { elements } => {
            for e in elements {
                collect_from_tuple(e, ctx, store);
            }
        }
        _ => {}
    }
}

pub(crate) fn resolve_variable_names(
    statements: &[Statement],
    ctx: &mut Vec<String>,
    store: &IdentifierStore,
) -> Result<Vec<Statement>> {
    let mut statements = resolve_statements(statements, ctx, store)?;
    let mut variables = store.list_declared_variables(ctx);
    variables.sort();
    statements.insert(0, Statement::Initialize { variables });
    Ok(statements)
}
fn resolve_statements(
    statements: &[Statement],
    ctx: &mut Vec<String>,
    store: &IdentifierStore,
) -> Result<Vec<Statement>> {
    let statements = statements
        .iter()
        .map(|s| resolve_statement(s, ctx, store))
        .collect::<Result<_>>()?;
    Ok(statements)
}

fn resolve_statement(
    statement: &Statement,
    ctx: &mut Vec<String>,
    store: &IdentifierStore,
) -> Result<Statement> {
    match statement {
        Statement::Initialize { .. } => unreachable!(),
        Statement::Expression { inner } => {
            let inner = resolve_expr(inner, ctx, store)?;
            Ok(Statement::Expression { inner })
        }
        Statement::Assign { target, value } => {
            let target = resolve_expr(target, ctx, store)?;
            let value = resolve_expr(value, ctx, store)?;
            Ok(Statement::Assign { target, value })
        }
        Statement::If { test, body, orelse } => {
            let test = resolve_expr(test, ctx, store)?;
            let body = resolve_statements(body, ctx, store)?;
            let orelse = match orelse {
                Some(orelse) => Some(resolve_statements(orelse, ctx, store)?),
                None => None,
            };
            Ok(Statement::If { test, body, orelse })
        }
    }
}

fn resolve_expr(
    expr: &Expression,
    ctx: &Vec<String>,
    store: &IdentifierStore,
) -> Result<Expression> {
    match expr {
        Expression::Identifier { name } => {
            let name = store.fetch(ctx, name)?;
            Ok(Expression::Identifier { name })
        }
        Expression::Call { function, args } => {
            let function = Box::new(resolve_expr(function, ctx, store)?);
            let args = resolve_expressions(args, ctx, store)?;
            Ok(Expression::Call { function, args })
        }
        Expression::Binop { a, b, op } => {
            let a = Box::new(resolve_expr(a, ctx, store)?);
            let b = Box::new(resolve_expr(b, ctx, store)?);
            Ok(Expression::Binop { a, b, op: *op })
        }
        Expression::Tuple { elements } => {
            let elements = resolve_expressions(elements, ctx, store)?;
            Ok(Expression::Tuple { elements })
        }
        Expression::Attribute { value, name } => {
            let value = Box::new(resolve_expr(value, ctx, store)?);
            Ok(Expression::Attribute {
                value,
                name: name.clone(),
            })
        }
        Expression::Compare { values, ops } => {
            let values = resolve_expressions(values, ctx, store)?;
            Ok(Expression::Compare {
                values,
                ops: ops.clone(),
            })
        }
        Expression::Number { value } => Ok(Expression::Number {
            value: value.clone(),
        }),
        Expression::Subscript { a, b } => {
            let a = Box::new(resolve_expr(a, ctx, store)?);
            let b = Box::new(resolve_expr(b, ctx, store)?);
            Ok(Expression::Subscript { a, b })
        }
    }
}
fn resolve_expressions(
    expressions: &[Expression],
    ctx: &Vec<String>,
    store: &IdentifierStore,
) -> Result<Vec<Expression>> {
    let expressions = expressions
        .iter()
        .map(|e| resolve_expr(e, ctx, store))
        .collect::<Result<_>>()?;
    Ok(expressions)
}

const BUILTIN_FUNCTIONS: [&str; 4] = ["int", "input", "print", "map"];
pub(crate) struct IdentifierStore {
    inner: BTreeMap<Vec<String>, BTreeMap<String, String>>,
    count: usize,
}

impl IdentifierStore {
    pub(crate) fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
            count: 0,
        }
    }
    fn declare(&mut self, ctx: &[String], name: &str) -> String {
        match self.inner.get(ctx).and_then(|map| map.get(name)) {
            Some(name) => name.clone(),
            None => {
                let variable_name = format_variable(self.count);
                self.inner
                    .entry(ctx.to_vec())
                    .or_default()
                    .insert(name.to_string(), variable_name.clone());
                self.count += 1;
                variable_name
            }
        }
    }

    fn fetch(&self, ctx: &[String], name: &str) -> Result<String> {
        if let Some(name) = self
            .inner
            .get(ctx)
            .and_then(|m| m.get(name))
            .map(|name| name.clone())
        {
            return Ok(name);
        }
        if BUILTIN_FUNCTIONS.contains(&name) {
            return Ok(name.to_string());
        }
        Err(anyhow!("undeclared variable: ctx={:?} name={}", ctx, name))
    }

    fn list_declared_variables(&self, ctx: &[String]) -> Vec<String> {
        self.inner
            .get(ctx)
            .map(|m| m.values().map(|name| name.clone()).collect())
            .unwrap_or_default()
    }
}

fn format_variable(id: usize) -> String {
    format!("__v{id}")
}
