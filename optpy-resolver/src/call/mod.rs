mod functiontree;
mod referencestore;

use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::{
    Assign, BinaryOperation, BoolOperation, CallFunction, CallMethod, Compare, Dict, Expr, Func,
    If, Index, Statement, UnaryOperation, While,
};

use self::{functiontree::FunctionTree, referencestore::ReferenceStore};

pub(super) fn resolve_function_calls(
    statements: &[Statement],
) -> (Vec<Statement>, BTreeMap<String, BTreeSet<String>>) {
    let mut store = ReferenceStore::default();
    let mut tree = FunctionTree::default();
    list_variable_contexts(statements, "$", &mut store, &mut tree);

    let variables = store.list_variables();
    for variable in variables {
        let functions = store.list_by_variable(&variable);
        for function in functions.iter() {
            let mut path = tree.path(function);
            while let Some(top) = path.pop() {
                if functions.contains(&top) {
                    path.push(top);
                    break;
                }
            }

            for function in path {
                store.record(&variable, &function);
            }
        }
    }

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
        Statement::If(If { test, body, orelse }) => {
            let test = resolve_expr(test, extensions);
            let body = resolve_statements(body, extensions);
            let orelse = resolve_statements(orelse, extensions);
            Statement::If(If { test, body, orelse })
        }
        Statement::Func(Func { name, args, body }) => {
            let variables = extensions.get(name).expect("invalid");
            let mut args = args.clone();
            args.extend(variables.clone());
            let body = resolve_statements(body, extensions);
            Statement::Func(Func {
                name: name.to_string(),
                args,
                body,
            })
        }
        Statement::Return(expr) => {
            Statement::Return(expr.as_ref().map(|e| resolve_expr(e, extensions)))
        }
        Statement::While(While { test, body }) => {
            let test = resolve_expr(test, extensions);
            let body = resolve_statements(body, extensions);
            Statement::While(While { test, body })
        }
        Statement::Break => Statement::Break,
        Statement::Continue => Statement::Continue,
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
        Expr::CallFunction(CallFunction { name, args }) => {
            let variables = match extensions.get(name) {
                Some(v) => v,
                None => {
                    return Expr::CallFunction(CallFunction {
                        name: name.to_string(),
                        args: resolve_exprs(args, extensions),
                    })
                }
            };
            let mut args = resolve_exprs(args, extensions);
            args.extend(
                variables
                    .iter()
                    .map(|name| Expr::VariableName(name.to_string())),
            );
            Expr::CallFunction(CallFunction {
                name: name.to_string(),
                args,
            })
        }
        Expr::CallMethod(CallMethod { value, name, args }) => {
            let value = resolve_expr(value, extensions);
            let args = resolve_exprs(args, extensions);
            Expr::CallMethod(CallMethod {
                value: Box::new(value),
                name: name.to_string(),
                args,
            })
        }
        Expr::Tuple(exprs) => Expr::Tuple(resolve_exprs(exprs, extensions)),
        Expr::BoolOperation(BoolOperation { op, conditions }) => {
            let conditions = resolve_exprs(conditions, extensions);
            Expr::BoolOperation(BoolOperation {
                op: *op,
                conditions,
            })
        }
        Expr::Compare(Compare { left, right, op }) => {
            let left = resolve_expr(left, extensions);
            let right = resolve_expr(right, extensions);
            Expr::Compare(Compare {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            })
        }
        Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
            let left = resolve_expr(left, extensions);
            let right = resolve_expr(right, extensions);
            Expr::BinaryOperation(BinaryOperation {
                left: Box::new(left),
                right: Box::new(right),
                op: *op,
            })
        }
        Expr::ConstantString(_)
        | Expr::ConstantBoolean(_)
        | Expr::ConstantNumber(_)
        | Expr::VariableName(_) => expr.clone(),
        Expr::UnaryOperation(UnaryOperation { value, op }) => {
            let value = Box::new(resolve_expr(value, extensions));
            Expr::UnaryOperation(UnaryOperation { value, op: *op })
        }
        Expr::Index(Index { value, index }) => {
            let value = Box::new(resolve_expr(value, extensions));
            let index = Box::new(resolve_expr(index, extensions));
            Expr::Index(Index { value, index })
        }
        Expr::List(list) => {
            let list = resolve_exprs(list, extensions);
            Expr::List(list)
        }
        Expr::Dict(Dict { pairs }) => {
            let pairs = pairs
                .iter()
                .map(|(key, value)| {
                    (
                        resolve_expr(key, extensions),
                        resolve_expr(value, extensions),
                    )
                })
                .collect();
            Expr::Dict(Dict { pairs })
        }
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
            Statement::Func(Func { name, args, body }) => {
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
                    assert!(
                        internal.remove(arg),
                        "no arg={} in internal={:?} in function={}",
                        arg,
                        internal,
                        name
                    );
                }
                definitions.insert(name.clone(), internal);
                extensions.insert(name.clone(), external);
            }
            Statement::If(If { body, orelse, .. }) => {
                collect_extension(body, store, definitions, extensions);
                collect_extension(orelse, store, definitions, extensions);
            }
            Statement::While(While { body, .. }) => {
                collect_extension(body, store, definitions, extensions);
            }
            Statement::Break
            | Statement::Continue
            | Statement::Assign(_)
            | Statement::Expression(_)
            | Statement::Return(_) => {}
        }
    }
}

fn list_variable_contexts(
    statements: &[Statement],
    function_name: &str,
    store: &mut ReferenceStore,
    tree: &mut FunctionTree,
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
            Statement::If(If { test, body, orelse }) => {
                list_from_expr(test, function_name, store);
                list_variable_contexts(body, function_name, store, tree);
                list_variable_contexts(orelse, function_name, store, tree);
            }
            Statement::Func(Func { name, args, body }) => {
                tree.add_edge(function_name, &name);
                list_variable_contexts(body, name, store, tree);
                for arg in args {
                    store.record(arg, name);
                }
            }
            Statement::While(While { test, body }) => {
                list_from_expr(test, function_name, store);
                list_variable_contexts(body, function_name, store, tree);
            }
            Statement::Break | Statement::Continue => {}
        }
    }
}

fn list_from_expr(expr: &Expr, function_name: &str, store: &mut ReferenceStore) {
    match expr {
        Expr::CallFunction(CallFunction { name: _, args }) => {
            list_from_exprs(args, function_name, store);
        }
        Expr::CallMethod(CallMethod {
            value,
            name: _,
            args,
        }) => {
            list_from_expr(value, function_name, store);
            list_from_exprs(args, function_name, store);
        }
        Expr::Tuple(values) => {
            list_from_exprs(values, function_name, store);
        }
        Expr::VariableName(name) => {
            store.record(name, function_name);
        }
        Expr::BoolOperation(BoolOperation { op: _, conditions }) => {
            list_from_exprs(conditions, function_name, store);
        }
        Expr::Compare(Compare { left, right, op: _ }) => {
            list_from_expr(left, function_name, store);
            list_from_expr(right, function_name, store);
        }
        Expr::BinaryOperation(BinaryOperation { left, right, op: _ }) => {
            list_from_expr(left, function_name, store);
            list_from_expr(right, function_name, store);
        }
        Expr::Index(Index { value, index }) => {
            list_from_expr(value, function_name, store);
            list_from_expr(index, function_name, store);
        }
        Expr::List(list) => {
            list_from_exprs(list, function_name, store);
        }
        Expr::Dict(Dict { pairs }) => {
            for (key, value) in pairs {
                list_from_expr(key, function_name, store);
                list_from_expr(value, function_name, store);
            }
        }
        Expr::ConstantNumber(_) | Expr::ConstantString(_) | Expr::ConstantBoolean(_) => {}
        Expr::UnaryOperation(UnaryOperation { value, op: _ }) => {
            list_from_expr(value, function_name, store);
        }
    }
}
fn list_from_exprs(exprs: &[Expr], function_name: &str, store: &mut ReferenceStore) {
    for expr in exprs {
        list_from_expr(expr, function_name, store);
    }
}

#[cfg(test)]
mod tests {
    use optpy_parser::{parse, CallFunction, Number};

    use crate::{resolve, util::StripMargin};

    use super::*;

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
                Statement::Func(Func {
                    name: "__f0".into(),
                    args: vec![],
                    body: vec![Statement::Return(Some(Expr::ConstantNumber(Number::Int(
                        "1".into()
                    ))))]
                }),
                Statement::Expression(Expr::CallFunction(CallFunction {
                    name: "print__macro__".into(),
                    args: vec![Expr::CallFunction(CallFunction {
                        name: "__f0".into(),
                        args: vec![]
                    })]
                }))
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
