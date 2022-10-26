use std::collections::BTreeMap;

use anyhow::{anyhow, Result};

use crate::{expression::OptpyExpression, statement::OptpyStatement};

pub(crate) fn collect_declared_variables(
    statements: &[OptpyStatement],
    ctx: &mut Vec<String>,
    store: &mut IdentifierStore,
) {
    for statement in statements {
        match statement {
            OptpyStatement::Assign { target, .. } => {
                collect_from_tuple(target, ctx, store);
            }
            OptpyStatement::If { body, orelse, .. } => {
                collect_declared_variables(body, ctx, store);
                if let Some(orelse) = orelse {
                    collect_declared_variables(orelse, ctx, store);
                }
            }
            OptpyStatement::For { target, body, .. } => {
                collect_from_tuple(target, ctx, store);
                collect_declared_variables(body, ctx, store);
            }
            OptpyStatement::FunctionDef { name, body, args } => {
                let name = store.declare(ctx, name);
                ctx.push(name.to_string());
                for arg in args {
                    store.declare(ctx, arg);
                }
                collect_declared_variables(body, ctx, store);
                assert_eq!(ctx.pop(), Some(name.to_string()));
            }
            OptpyStatement::Return { .. }
            | OptpyStatement::Initialize { .. }
            | OptpyStatement::Expression { .. } => continue,
        }
    }
}

fn collect_from_tuple(expr: &OptpyExpression, ctx: &Vec<String>, store: &mut IdentifierStore) {
    match expr {
        OptpyExpression::Identifier { name } => {
            store.declare(ctx, name);
        }
        OptpyExpression::Tuple { elements } => {
            for e in elements {
                collect_from_tuple(e, ctx, store);
            }
        }
        _ => {}
    }
}

pub(crate) fn resolve_variable_names(
    statements: &[OptpyStatement],
    ctx: &mut Vec<String>,
    store: &IdentifierStore,
) -> Result<Vec<OptpyStatement>> {
    let mut statements = resolve_statements(statements, ctx, store)?;
    let mut variables = store.list_declared_variables(ctx);
    variables.sort();
    statements.insert(0, OptpyStatement::Initialize { variables });
    Ok(statements)
}
fn resolve_statements(
    statements: &[OptpyStatement],
    ctx: &mut Vec<String>,
    store: &IdentifierStore,
) -> Result<Vec<OptpyStatement>> {
    let statements = statements
        .iter()
        .map(|s| resolve_statement(s, ctx, store))
        .collect::<Result<_>>()?;
    Ok(statements)
}

fn resolve_statement(
    statement: &OptpyStatement,
    ctx: &mut Vec<String>,
    store: &IdentifierStore,
) -> Result<OptpyStatement> {
    match statement {
        OptpyStatement::Initialize { .. } => unreachable!(),
        OptpyStatement::Expression { inner } => {
            let inner = resolve_expr(inner, ctx, store)?;
            Ok(OptpyStatement::Expression { inner })
        }
        OptpyStatement::Assign { target, value } => {
            let target = resolve_expr(target, ctx, store)?;
            let value = resolve_expr(value, ctx, store)?;
            Ok(OptpyStatement::Assign { target, value })
        }
        OptpyStatement::If { test, body, orelse } => {
            let test = resolve_expr(test, ctx, store)?;
            let body = resolve_statements(body, ctx, store)?;
            let orelse = match orelse {
                Some(orelse) => Some(resolve_statements(orelse, ctx, store)?),
                None => None,
            };
            Ok(OptpyStatement::If { test, body, orelse })
        }
        OptpyStatement::For { target, iter, body } => {
            let target = resolve_expr(target, ctx, store)?;
            let iter = resolve_expr(iter, ctx, store)?;
            let body = resolve_statements(body, ctx, store)?;
            Ok(OptpyStatement::For { target, iter, body })
        }
        OptpyStatement::FunctionDef { name, body, args } => {
            let (name, user_defined) = store.fetch(ctx, name)?;
            assert!(user_defined);
            ctx.push(name.clone());
            let args = args
                .iter()
                .map(|arg| store.fetch(ctx, arg))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|(name, user_defined)| {
                    assert!(user_defined);
                    name
                })
                .collect::<Vec<_>>();
            let body = resolve_statements(body, ctx, store)?;
            assert_eq!(ctx.pop(), Some(name.clone()));
            Ok(OptpyStatement::FunctionDef { name, body, args })
        }
        OptpyStatement::Return { value } => {
            let value = match value {
                Some(e) => Some(resolve_expr(e, ctx, store)?),
                None => None,
            };
            Ok(OptpyStatement::Return { value })
        }
    }
}

fn resolve_expr(
    expr: &OptpyExpression,
    ctx: &Vec<String>,
    store: &IdentifierStore,
) -> Result<OptpyExpression> {
    match expr {
        OptpyExpression::Identifier { name } => {
            let (name, user_defined) = store.fetch(ctx, name)?;
            if user_defined {
                Ok(OptpyExpression::Attribute {
                    value: Box::new(OptpyExpression::Identifier { name: "ctx".into() }),
                    name,
                })
            } else {
                Ok(OptpyExpression::Identifier { name })
            }
        }
        OptpyExpression::Call { function, args } => {
            let function = Box::new(resolve_expr(function, ctx, store)?);
            let args = resolve_expressions(args, ctx, store)?;
            Ok(OptpyExpression::Call { function, args })
        }
        OptpyExpression::Binop { a, b, op } => {
            let a = Box::new(resolve_expr(a, ctx, store)?);
            let b = Box::new(resolve_expr(b, ctx, store)?);
            Ok(OptpyExpression::Binop { a, b, op: *op })
        }
        OptpyExpression::Tuple { elements } => {
            let elements = resolve_expressions(elements, ctx, store)?;
            Ok(OptpyExpression::Tuple { elements })
        }
        OptpyExpression::Attribute { value, name } => {
            let value = Box::new(resolve_expr(value, ctx, store)?);
            Ok(OptpyExpression::Attribute {
                value,
                name: name.clone(),
            })
        }
        OptpyExpression::Compare { values, ops } => {
            let values = resolve_expressions(values, ctx, store)?;
            Ok(OptpyExpression::Compare {
                values,
                ops: ops.clone(),
            })
        }
        OptpyExpression::Number { value } => Ok(OptpyExpression::Number {
            value: value.clone(),
        }),
        OptpyExpression::Subscript { a, b } => {
            let a = Box::new(resolve_expr(a, ctx, store)?);
            let b = Box::new(resolve_expr(b, ctx, store)?);
            Ok(OptpyExpression::Subscript { a, b })
        }
    }
}
fn resolve_expressions(
    expressions: &[OptpyExpression],
    ctx: &Vec<String>,
    store: &IdentifierStore,
) -> Result<Vec<OptpyExpression>> {
    let expressions = expressions
        .iter()
        .map(|e| resolve_expr(e, ctx, store))
        .collect::<Result<_>>()?;
    Ok(expressions)
}

const BUILTIN_FUNCTIONS: [(&str, &str); 5] = [
    ("int", "int"),
    ("input", "input"),
    ("print", "print"),
    ("map", "map"),
    ("range", "range!"),
];
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

    fn fetch(&self, ctx: &[String], name: &str) -> Result<(String, bool)> {
        if let Some(name) = self
            .inner
            .get(ctx)
            .and_then(|m| m.get(name))
            .map(|name| name.clone())
        {
            return Ok((name, true));
        }

        if let Some((_, call)) = BUILTIN_FUNCTIONS.iter().find(|(n, _)| *n == name) {
            return Ok((call.to_string(), false));
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
