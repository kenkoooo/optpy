use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::{Assign, Expr, Statement};

pub(super) fn resolve_function_calls(
    statements: &[Statement],
) -> (Vec<Statement>, BTreeMap<String, BTreeSet<String>>) {
    let mut store = ReferenceStore::default();
    list_variable_contexts(statements, "$", &mut store);

    let mut definitions = BTreeMap::new();
    let mut extensions = BTreeMap::new();
    collect_extension(statements, &mut store, &mut definitions, &mut extensions);

    let root_variables = store.list_by_function("$");
    for v in root_variables.iter() {
        let functions = store.list_by_variable(v);
        assert_eq!(functions.len(), 1);
    }

    definitions.insert(String::new(), root_variables);
    let statements = resolve_statements(statements, &extensions);
    (statements, definitions)
}
fn resolve_statement(
    statement: &Statement,
    extensions: &BTreeMap<String, BTreeSet<String>>,
) -> Statement {
    match statement {
        Statement::Assign(Assign { target, value }) => {
            let target = resolve_expr(target, extensions);
            let value = resolve_expr(value, extensions);
            Statement::Assign(Assign { target, value })
        }
        Statement::Expression(expr) => Statement::Expression(resolve_expr(expr, extensions)),
        Statement::If { test, body, orelse } => {
            let test = resolve_expr(test, extensions);
            let body = resolve_statements(body, extensions);
            let orelse = resolve_statements(orelse, extensions);
            Statement::If { test, body, orelse }
        }
        Statement::Func { name, args, body } => {
            let variables = extensions.get(name).expect("invalid");
            let mut args = args.clone();
            args.extend(variables.clone());
            let body = resolve_statements(body, extensions);
            Statement::Func {
                name: name.to_string(),
                args,
                body,
            }
        }
        Statement::Return(expr) => {
            Statement::Return(expr.as_ref().map(|e| resolve_expr(e, extensions)))
        }
        Statement::While { test, body } => {
            let test = resolve_expr(test, extensions);
            let body = resolve_statements(body, extensions);
            Statement::While { test, body }
        }
        Statement::Break => Statement::Break,
        statement => unreachable!("{:?}", statement),
    }
}

fn resolve_statements(
    statements: &[Statement],
    extensions: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<Statement> {
    statements
        .iter()
        .map(|s| resolve_statement(s, extensions))
        .collect()
}

fn resolve_expr(expr: &Expr, extensions: &BTreeMap<String, BTreeSet<String>>) -> Expr {
    match expr {
        Expr::CallFunction { name, args } => {
            let variables = match extensions.get(name) {
                Some(v) => v,
                None => {
                    return Expr::CallFunction {
                        name: name.to_string(),
                        args: resolve_exprs(args, extensions),
                    }
                }
            };
            let mut args = resolve_exprs(args, extensions);
            args.extend(
                variables
                    .iter()
                    .map(|name| Expr::VariableName(name.to_string())),
            );
            Expr::CallFunction {
                name: name.to_string(),
                args,
            }
        }
        Expr::CallMethod { value, name, args } => {
            let value = resolve_expr(value, extensions);
            let args = resolve_exprs(args, extensions);
            Expr::CallMethod {
                value: Box::new(value),
                name: name.to_string(),
                args,
            }
        }
        Expr::Tuple(exprs) => Expr::Tuple(resolve_exprs(exprs, extensions)),
        Expr::BoolOperation { op, conditions } => {
            let conditions = resolve_exprs(conditions, extensions);
            Expr::BoolOperation {
                op: *op,
                conditions,
            }
        }
        Expr::Compare { left, right, op } => {
            let left = resolve_expr(left, extensions);
            let right = resolve_expr(right, extensions);
            Expr::Compare {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            }
        }
        Expr::BinaryOperation { left, right, op } => {
            let left = resolve_expr(left, extensions);
            let right = resolve_expr(right, extensions);
            Expr::BinaryOperation {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            }
        }
        expr => expr.clone(),
    }
}

fn resolve_exprs(exprs: &[Expr], extensions: &BTreeMap<String, BTreeSet<String>>) -> Vec<Expr> {
    exprs.iter().map(|e| resolve_expr(e, extensions)).collect()
}

fn collect_extension(
    statements: &[Statement],
    store: &mut ReferenceStore,
    definitions: &mut BTreeMap<String, BTreeSet<String>>,
    extensions: &mut BTreeMap<String, BTreeSet<String>>,
) {
    for statement in statements {
        match statement {
            Statement::Func { name, args, body } => {
                collect_extension(body, store, definitions, extensions);
                let mut external = BTreeSet::new();
                let mut internal = BTreeSet::new();
                let variables = store.list_by_function(name);
                for v in variables {
                    let functions = store.list_by_variable(&v);
                    if functions.len() == 1 {
                        internal.insert(v);
                    } else {
                        external.insert(v);
                    }
                }
                store.remove_function(name);
                for arg in args {
                    assert!(internal.remove(arg));
                }
                definitions.insert(name.clone(), internal);
                extensions.insert(name.clone(), external);
            }
            _ => {}
        }
    }
}

fn list_variable_contexts(
    statements: &[Statement],
    function_name: &str,
    store: &mut ReferenceStore,
) {
    for statement in statements {
        match statement {
            Statement::Assign(Assign { target, value }) => {
                list_from_expr(target, function_name, store);
                list_from_expr(value, function_name, store);
            }
            Statement::Expression(expr) => {
                list_from_expr(expr, function_name, store);
            }
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    list_from_expr(expr, function_name, store);
                }
            }
            Statement::If { test, body, orelse } => {
                list_from_expr(test, function_name, store);
                list_variable_contexts(body, function_name, store);
                list_variable_contexts(orelse, function_name, store);
            }
            Statement::Func { name, args, body } => {
                list_variable_contexts(body, name, store);
                for arg in args {
                    store.record(arg, name);
                }
            }
            Statement::While { test, body } => {
                list_from_expr(test, function_name, store);
                list_variable_contexts(body, function_name, store);
            }
            Statement::Break => continue,
            statement => unreachable!("{:?}", statement),
        }
    }
}

fn list_from_expr(expr: &Expr, function_name: &str, store: &mut ReferenceStore) {
    match expr {
        Expr::CallFunction { name: _, args } => {
            list_from_exprs(args, function_name, store);
        }
        Expr::CallMethod {
            value,
            name: _,
            args,
        } => {
            list_from_expr(value, function_name, store);
            list_from_exprs(args, function_name, store);
        }
        Expr::Tuple(values) => {
            list_from_exprs(values, function_name, store);
        }
        Expr::VariableName(name) => {
            store.record(name, function_name);
        }
        Expr::BoolOperation { op: _, conditions } => {
            list_from_exprs(conditions, function_name, store);
        }
        Expr::Compare { left, right, op: _ } => {
            list_from_expr(left, function_name, store);
            list_from_expr(right, function_name, store);
        }
        Expr::BinaryOperation { left, right, op: _ } => {
            list_from_expr(left, function_name, store);
            list_from_expr(right, function_name, store);
        }
        Expr::Index { value, index } => {
            list_from_expr(value, function_name, store);
            list_from_expr(index, function_name, store);
        }
        Expr::List(list) => {
            list_from_exprs(list, function_name, store);
        }
        Expr::ConstantNumber(_) | Expr::ConstantString(_) | Expr::ConstantBoolean(_) => {}
        Expr::UnaryOperation { value, op: _ } => {
            list_from_expr(value, function_name, store);
        }
        expr => unreachable!("{:?}", expr),
    }
}
fn list_from_exprs(exprs: &[Expr], function_name: &str, store: &mut ReferenceStore) {
    for expr in exprs {
        list_from_expr(expr, function_name, store);
    }
}

#[derive(Default, Debug)]
struct ReferenceStore {
    variable_functions: BTreeMap<String, BTreeSet<String>>,
    function_variables: BTreeMap<String, BTreeSet<String>>,
}

impl ReferenceStore {
    fn record(&mut self, variable_name: &str, function_name: &str) {
        self.variable_functions
            .entry(variable_name.to_string())
            .or_default()
            .insert(function_name.to_string());
        self.function_variables
            .entry(function_name.to_string())
            .or_default()
            .insert(variable_name.to_string());
    }

    fn list_by_function(&self, function_name: &str) -> BTreeSet<String> {
        self.function_variables
            .get(function_name)
            .cloned()
            .unwrap_or_default()
    }

    fn list_by_variable(&self, variable_name: &str) -> BTreeSet<String> {
        self.variable_functions
            .get(variable_name)
            .cloned()
            .unwrap_or_default()
    }

    fn remove_function(&mut self, function_name: &str) {
        let variables = self
            .function_variables
            .remove(function_name)
            .unwrap_or_default();
        for variable in variables {
            assert!(self
                .variable_functions
                .get_mut(&variable)
                .expect("invalid")
                .remove(function_name));
        }
    }
}

#[cfg(test)]
mod tests {
    use optpy_parser::{parse, Number};

    use crate::{resolve, util::StripMargin};

    use super::*;

    #[test]
    fn test_call_resolver() {
        let code = r"
            |__v0, __v1 = map(int, input().split())
            |__v2 = __v0 + __v1
            |def __f0(__v3):
            |    def __f1(__v4):
            |        __v5 = __v1 + __v2
            |        return __v4 + __v5
            |    return __f1(__v1) + __v3
            |__v6 = __f0(__v0 + __v1 + __v2)
            |print(__v6)"
            .strip_margin();

        let expected = r"
            |__v0 = map_int(input().split())
            |__v1 = __v0[0]
            |__v2 = __v0[1]
            |__v3 = __v1 + __v2
            |def __f0(__v4, __v2):
            |    def __f1(__v5, __v2, __v3):
            |        __v6 = __v2 + __v3
            |        return __v5 + __v6
            |    return __f1(__v2, __v2, __v3) + __v4
            |__v7 = __f0(__v1 + __v2 + __v3, __v2)
            |print__macro__(__v7)"
            .strip_margin();

        let ast = parse(code).unwrap();
        let (statements, definitions) = resolve(&ast);
        assert_eq!(statements, parse(expected).unwrap());
        assert_eq!(
            definitions,
            BTreeMap::from([
                (
                    "".into(),
                    BTreeSet::from([
                        "__v0".into(),
                        "__v1".into(),
                        "__v2".into(),
                        "__v3".into(),
                        "__v7".into(),
                    ])
                ),
                ("__f0".into(), BTreeSet::from([])),
                ("__f1".into(), BTreeSet::from(["__v6".into()]))
            ])
        );
    }

    #[test]
    fn test_non_variable_function() {
        let code = r"
            |def f():
            |   return 1
            |print(f())"
            .strip_margin();

        let ast = parse(code).unwrap();
        let (statements, definitions) = resolve(&ast);
        assert_eq!(
            statements,
            [
                Statement::Func {
                    name: "__f0".into(),
                    args: vec![],
                    body: vec![Statement::Return(Some(Expr::ConstantNumber(Number::Int(
                        "1".into()
                    ))))]
                },
                Statement::Expression(Expr::CallFunction {
                    name: "print__macro__".into(),
                    args: vec![Expr::CallFunction {
                        name: "__f0".into(),
                        args: vec![]
                    }]
                })
            ]
        );
        assert_eq!(
            definitions,
            BTreeMap::from([
                ("".into(), BTreeSet::from([])),
                ("__f0".into(), BTreeSet::from([])),
            ])
        );
    }
}
