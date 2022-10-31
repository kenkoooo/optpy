use std::collections::HashMap;

use optpy_parser::{Expr, Statement};

use crate::error::{Error, Result};

pub fn resolve_names(statements: &[Statement]) -> Result<Vec<Statement>> {
    let mut variables = NameStore::new("__v");
    let mut functions = NameStore::new("__f");
    let ctx = ContextPath::default();
    collect_names(statements, &mut variables, &mut functions, &ctx);
    let statements = resolve_statements(statements, &variables, &functions, &ctx)?;
    Ok(statements)
}

fn collect_names(
    statements: &[Statement],
    variables: &mut NameStore,
    functions: &mut NameStore,
    ctx: &ContextPath,
) {
    for statement in statements {
        match statement {
            Statement::Assign { target, .. } => collect_variable_names(target, variables, ctx),
            Statement::If { body, orelse, .. } => {
                collect_names(body, variables, functions, ctx);
                collect_names(orelse, variables, functions, ctx);
            }
            Statement::Func { name, args, body } => {
                functions.declare(name, ctx);
                let ctx = ctx.join(name);
                for arg in args {
                    variables.declare(arg, &ctx);
                }
                collect_names(body, variables, functions, &ctx);
            }
            Statement::Return(_) | Statement::Expression(_) => continue,
        }
    }
}

fn collect_variable_names(expr: &Expr, variables: &mut NameStore, ctx: &ContextPath) {
    match expr {
        Expr::Tuple(tuple) => {
            for variable in tuple {
                collect_variable_names(variable, variables, ctx);
            }
        }
        Expr::Ident(name) => {
            variables.declare(name, ctx);
        }
        _ => todo!(),
    }
}

fn resolve_statements(
    statements: &[Statement],
    variables: &NameStore,
    functions: &NameStore,
    ctx: &ContextPath,
) -> Result<Vec<Statement>> {
    statements
        .iter()
        .map(|s| match s {
            Statement::Assign { target, value } => {
                let target = resolve_expr(target, variables, functions, ctx)?;
                let value = resolve_expr(value, variables, functions, ctx)?;
                Ok(Statement::Assign { target, value })
            }
            Statement::Expression(expr) => Ok(Statement::Expression(resolve_expr(
                expr, variables, functions, ctx,
            )?)),
            Statement::If { test, body, orelse } => {
                let test = resolve_expr(test, variables, functions, ctx)?;
                let body = resolve_statements(body, variables, functions, ctx)?;
                let orelse = resolve_statements(orelse, variables, functions, ctx)?;
                Ok(Statement::If { test, body, orelse })
            }
            Statement::Func { name, args, body } => {
                let resolved_name = functions.resolve(name, ctx).ok_or_else(|| {
                    Error::Unresolved(format!("name {} couldn't be resolved in {:?}", name, ctx))
                })?;
                let ctx = ctx.join(name);
                let args = args
                    .iter()
                    .map(|arg| {
                        variables.resolve(arg, &ctx).ok_or_else(|| {
                            Error::Unresolved(format!(
                                "name {} couldn't be resolved in {:?}",
                                arg, ctx
                            ))
                        })
                    })
                    .collect::<Result<_>>()?;
                let body = resolve_statements(body, variables, functions, &ctx)?;
                Ok(Statement::Func {
                    name: resolved_name,
                    args,
                    body,
                })
            }
            Statement::Return(expr) => match expr {
                Some(expr) => Ok(Statement::Return(Some(resolve_expr(
                    expr, variables, functions, ctx,
                )?))),
                None => Ok(Statement::Return(None)),
            },
        })
        .collect()
}

fn resolve_expr(
    expr: &Expr,
    variables: &NameStore,
    functions: &NameStore,
    ctx: &ContextPath,
) -> Result<Expr> {
    match expr {
        Expr::CallFunction { name, args } => {
            let name = match functions.resolve(name, ctx) {
                Some(name) => name,
                None => {
                    // built-in function
                    name.to_string()
                }
            };
            let args = resolve_exprs(args, variables, functions, ctx)?;
            Ok(Expr::CallFunction { name, args })
        }
        Expr::CallMethod { value, name, args } => {
            let value = resolve_expr(value, variables, functions, ctx)?;
            let args = resolve_exprs(args, variables, functions, ctx)?;
            Ok(Expr::CallMethod {
                value: Box::new(value),
                name: name.clone(),
                args,
            })
        }
        Expr::Tuple(exprs) => {
            let exprs = resolve_exprs(exprs, variables, functions, ctx)?;
            Ok(Expr::Tuple(exprs))
        }
        Expr::Ident(name) => {
            let name = match variables.resolve(name, ctx) {
                Some(name) => name,
                None => {
                    // built-in variable
                    name.to_string()
                }
            };
            Ok(Expr::Ident(name))
        }
        Expr::BoolOperation { op, conditions } => {
            let conditions = resolve_exprs(conditions, variables, functions, ctx)?;
            Ok(Expr::BoolOperation {
                op: *op,
                conditions,
            })
        }
        Expr::Compare { left, right, op } => {
            let left = resolve_expr(left, variables, functions, ctx)?;
            let right = resolve_expr(right, variables, functions, ctx)?;
            Ok(Expr::Compare {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            })
        }
        Expr::BinaryOperation { left, right, op } => {
            let left = resolve_expr(left, variables, functions, ctx)?;
            let right = resolve_expr(right, variables, functions, ctx)?;
            Ok(Expr::BinaryOperation {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            })
        }
        Expr::Number(number) => Ok(Expr::Number(number.clone())),
    }
}

fn resolve_exprs(
    exprs: &[Expr],
    variables: &NameStore,
    functions: &NameStore,
    ctx: &ContextPath,
) -> Result<Vec<Expr>> {
    exprs
        .iter()
        .map(|expr| resolve_expr(expr, variables, functions, ctx))
        .collect::<Result<Vec<_>>>()
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct ContextPath(Vec<String>);

impl ContextPath {
    fn join(&self, name: &str) -> Self {
        let mut path = self.0.clone();
        path.push(name.to_string());
        Self(path)
    }
    fn pop(&self) -> Option<Self> {
        let mut next = self.0.clone();
        match next.pop() {
            Some(_) => Some(Self(next)),
            None => None,
        }
    }
}

impl Default for ContextPath {
    fn default() -> Self {
        Self(Default::default())
    }
}

struct NameStore {
    prefix: String,
    map: HashMap<ContextPath, HashMap<String, String>>,
    global_counter: usize,
}

impl NameStore {
    fn new<S: AsRef<str>>(prefix: S) -> Self {
        Self {
            prefix: prefix.as_ref().into(),
            map: Default::default(),
            global_counter: 0,
        }
    }
    fn declare(&mut self, name: &str, ctx: &ContextPath) {
        let map = self.map.entry(ctx.clone()).or_default();
        map.entry(name.to_string()).or_insert_with(|| {
            let result = format!("{}{}", self.prefix, self.global_counter);
            self.global_counter += 1;
            result
        });
    }

    fn resolve(&self, name: &str, ctx: &ContextPath) -> Option<String> {
        let mut ctx = ctx.clone();
        loop {
            if let Some(name) = self.map.get(&ctx).and_then(|m| m.get(name)) {
                return Some(name.clone());
            }

            match ctx.pop() {
                Some(next) => {
                    ctx = next;
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use optpy_parser::parse;
    use optpy_test_helper::{assign, call_fn, call_method, expr, ident, tuple, StripMargin};

    use super::*;

    #[test]
    fn test_basic_resolver() {
        let code = r"
            |a, b = map(int, input().split())
            |print(a)
            |"
        .strip_margin();
        let ast = parse(code).unwrap();
        let resolved = resolve_names(&ast).unwrap();
        assert_eq!(
            resolved,
            vec![
                assign(
                    tuple!(ident("__v0"), ident("__v1")),
                    call_fn!(
                        "map",
                        ident("int"),
                        call_method!(call_fn!("input"), "split")
                    )
                ),
                expr(call_fn!("print", ident("__v0")))
            ]
        );
    }

    #[test]
    fn test_function_resolver() {
        let code = r"
            |a, b = map(int, input().split())
            |def func(a):
            |    return a + b
            |c = func(a)
            |print(c)
            |"
        .strip_margin();
        let ast = parse(code).unwrap();
        let resolved = resolve_names(&ast).unwrap();

        let expected = r"
            |__v0, __v1 = map(int, input().split())
            |def __f0(__v2):
            |    return __v2 + __v1
            |__v3 = __f0(__v0)
            |print(__v3)
            |"
        .strip_margin();
        assert_eq!(resolved, parse(expected).unwrap());
    }
}
