use std::collections::HashMap;

use optpy_parser::{
    Assign, BinaryOperation, BoolOperation, CallFunction, CallMethod, Compare, Dict, Expr, Func,
    If, Index, Statement, UnaryOperation, While,
};

pub(super) fn resolve_names(statements: &[Statement]) -> Vec<Statement> {
    let mut variables = NameStore::new("__v");
    let mut functions = NameStore::new("__f");
    let ctx = ContextPath::default();
    collect_declarations(statements, &mut variables, &mut functions, &ctx);
    let statements = resolve_statements(statements, &variables, &functions, &ctx);
    statements
}

fn collect_declarations(
    statements: &[Statement],
    variables: &mut NameStore,
    functions: &mut NameStore,
    ctx: &ContextPath,
) {
    for statement in statements {
        match statement {
            Statement::Assign(Assign { target, .. }) => {
                collect_variable_names(target, variables, ctx)
            }
            Statement::If(If { body, orelse, .. }) => {
                collect_declarations(body, variables, functions, ctx);
                collect_declarations(orelse, variables, functions, ctx);
            }
            Statement::Func(Func { name, args, body }) => {
                functions.declare(name, ctx);
                let ctx = ctx.join(name);
                for arg in args {
                    variables.declare(arg, &ctx);
                }
                collect_declarations(body, variables, functions, &ctx);
            }
            Statement::While(While { body, .. }) => {
                collect_declarations(body, variables, functions, ctx);
            }
            Statement::Return(_)
            | Statement::Expression(_)
            | Statement::Break
            | Statement::Continue
            | Statement::Import(_)
            | Statement::FromImport(_) => continue,
        }
    }
}

fn collect_variable_names(expr: &Expr, variables: &mut NameStore, ctx: &ContextPath) {
    match expr {
        Expr::VariableName(name) => {
            variables.declare(name, ctx);
        }
        Expr::Index(Index { .. }) => {}
        expr => unreachable!("{:?}", expr),
    }
}

fn resolve_statements(
    statements: &[Statement],
    variables: &NameStore,
    functions: &NameStore,
    ctx: &ContextPath,
) -> Vec<Statement> {
    statements
        .iter()
        .map(|s| match s {
            Statement::Assign(Assign { target, value }) => {
                let target = resolve_expr(target, variables, functions, ctx);
                let value = resolve_expr(value, variables, functions, ctx);
                Statement::Assign(Assign { target, value })
            }
            Statement::Expression(expr) => {
                Statement::Expression(resolve_expr(expr, variables, functions, ctx))
            }
            Statement::If(If { test, body, orelse }) => {
                let test = resolve_expr(test, variables, functions, ctx);
                let body = resolve_statements(body, variables, functions, ctx);
                let orelse = resolve_statements(orelse, variables, functions, ctx);
                Statement::If(If { test, body, orelse })
            }
            Statement::Func(Func { name, args, body }) => {
                let resolved_name = functions.resolve(name, ctx).expect("invalid");
                let ctx = ctx.join(name);
                let args = args
                    .iter()
                    .map(|arg| variables.resolve(arg, &ctx).expect("invalid"))
                    .collect::<Vec<_>>();
                let body = resolve_statements(body, variables, functions, &ctx);
                Statement::Func(Func {
                    name: resolved_name,
                    args,
                    body,
                })
            }
            Statement::Return(expr) => match expr {
                Some(expr) => {
                    Statement::Return(Some(resolve_expr(expr, variables, functions, ctx)))
                }
                None => Statement::Return(None),
            },
            Statement::While(While { test, body }) => {
                let test = resolve_expr(test, variables, functions, ctx);
                let body = resolve_statements(body, variables, functions, ctx);
                Statement::While(While { test, body })
            }
            Statement::Break
            | Statement::Continue
            | Statement::Import(_)
            | Statement::FromImport(_) => s.clone(),
        })
        .collect()
}

fn resolve_expr(
    expr: &Expr,
    variables: &NameStore,
    functions: &NameStore,
    ctx: &ContextPath,
) -> Expr {
    match expr {
        Expr::CallFunction(CallFunction { name, args }) => {
            let name = match functions.resolve(name, ctx) {
                Some(name) => name,
                None => {
                    // built-in function
                    name.to_string()
                }
            };
            let args = resolve_exprs(args, variables, functions, ctx);
            Expr::CallFunction(CallFunction { name, args })
        }
        Expr::CallMethod(CallMethod { value, name, args }) => {
            let value = resolve_expr(value, variables, functions, ctx);
            let args = resolve_exprs(args, variables, functions, ctx);
            Expr::CallMethod(CallMethod {
                value: Box::new(value),
                name: name.clone(),
                args,
            })
        }
        Expr::Tuple(exprs) => {
            let exprs = resolve_exprs(exprs, variables, functions, ctx);
            Expr::Tuple(exprs)
        }
        Expr::VariableName(name) => {
            let name = match variables.resolve(name, ctx) {
                Some(name) => name,
                None => {
                    // built-in variable
                    name.to_string()
                }
            };
            Expr::VariableName(name)
        }
        Expr::BoolOperation(BoolOperation { op, conditions }) => {
            let conditions = resolve_exprs(conditions, variables, functions, ctx);
            Expr::BoolOperation(BoolOperation {
                op: *op,
                conditions,
            })
        }
        Expr::Compare(Compare { left, right, op }) => {
            let left = resolve_expr(left, variables, functions, ctx);
            let right = resolve_expr(right, variables, functions, ctx);
            Expr::Compare(Compare {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            })
        }
        Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
            let left = resolve_expr(left, variables, functions, ctx);
            let right = resolve_expr(right, variables, functions, ctx);
            Expr::BinaryOperation(BinaryOperation {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            })
        }
        Expr::Index(Index { value, index }) => {
            let value = resolve_expr(value, variables, functions, ctx);
            let index = resolve_expr(index, variables, functions, ctx);
            Expr::Index(Index {
                value: Box::new(value),
                index: Box::new(index),
            })
        }
        Expr::List(list) => {
            let list = resolve_exprs(list, variables, functions, ctx);
            Expr::List(list)
        }
        Expr::ConstantString(_)
        | Expr::ConstantNumber(_)
        | Expr::ConstantBoolean(_)
        | Expr::None => expr.clone(),
        Expr::UnaryOperation(UnaryOperation { value, op }) => {
            let value = resolve_expr(value, variables, functions, ctx);
            Expr::UnaryOperation(UnaryOperation {
                value: Box::new(value),
                op: *op,
            })
        }
        Expr::Dict(Dict { pairs }) => {
            let pairs = pairs
                .iter()
                .map(|(key, value)| {
                    let key = resolve_expr(key, variables, functions, ctx);
                    let value = resolve_expr(value, variables, functions, ctx);
                    (key, value)
                })
                .collect();
            Expr::Dict(Dict { pairs })
        }
    }
}

fn resolve_exprs(
    exprs: &[Expr],
    variables: &NameStore,
    functions: &NameStore,
    ctx: &ContextPath,
) -> Vec<Expr> {
    exprs
        .iter()
        .map(|expr| resolve_expr(expr, variables, functions, ctx))
        .collect::<Vec<_>>()
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

    use crate::util::StripMargin;

    use super::*;

    #[test]
    fn test_basic_resolver() {
        let code = r"
            |a, b = map(int, input().split())
            |print(a)
            |"
        .strip_margin();
        let ast = parse(code).unwrap();
        let resolved = resolve_names(&ast);

        let expected = r"
            |__v0 = iter(map(int, input().split()))
            |__v1 = next(__v0)
            |__v2 = next(__v0)
            |print(__v1)"
            .strip_margin();
        assert_eq!(resolved, parse(expected).unwrap());
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
        let resolved = resolve_names(&ast);

        let expected = r"
            |__v0 = iter(map(int, input().split()))
            |__v1 = next(__v0)
            |__v2 = next(__v0)
            |def __f0(__v3):
            |    return __v3 + __v2
            |__v4 = __f0(__v1)
            |print(__v4)"
            .strip_margin();
        assert_eq!(resolved, parse(expected).unwrap());

        let code = r"
            |a, b = map(int, input().split())
            |c = a + b
            |def f(a):
            |   def g(a):
            |       return a + b + c
            |   return g(b) + a
            |d = f(a + b + c)
            |print(d)
        "
        .strip_margin();

        let expected = r"
            |__v0 = iter(map(int, input().split()))
            |__v1 = next(__v0)
            |__v2 = next(__v0)
            |__v3 = __v1 + __v2
            |def __f0(__v4):
            |    def __f1(__v5):
            |        return __v5 + __v2 + __v3
            |    return __f1(__v2) + __v4
            |__v6 = __f0(__v1 + __v2 + __v3)
            |print(__v6)"
            .strip_margin();

        assert_eq!(
            resolve_names(&parse(code).unwrap()),
            parse(expected).unwrap()
        );

        let code = r"
            |a, b = map(int, input().split())
            |c = a + b
            |def f(a):
            |    def g(a):
            |        d = b + c
            |        return a + d
            |    return g(b) + a
            |d = f(a + b + c)
            |print(d)
        "
        .strip_margin();

        let expected = r"
            |__v0 = iter(map(int, input().split()))
            |__v1 = next(__v0)
            |__v2 = next(__v0)
            |__v3 = __v1 + __v2
            |def __f0(__v4):
            |    def __f1(__v5):
            |        __v6 = __v2 + __v3
            |        return __v5 + __v6
            |    return __f1(__v2) + __v4
            |__v7 = __f0(__v1 + __v2 + __v3)
            |print(__v7)"
            .strip_margin();

        assert_eq!(
            resolve_names(&parse(code).unwrap()),
            parse(expected).unwrap()
        );
    }
}
