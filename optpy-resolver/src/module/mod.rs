mod module_map;
pub(crate) use module_map::ModuleMap;

use std::collections::BTreeMap;

use optpy_parser::{
    Assign, BinaryOperation, BoolOperation, CallFunction, CallMethod, Compare, Dict, Expr,
    FromImport, Func, If, Import, Index, Statement, UnaryOperation, While,
};

pub(super) fn resolve_modules(
    statements: Vec<Statement>,
    module_map: &ModuleMap,
) -> Vec<Statement> {
    let mut modules = EnabledModules {
        layers: vec![],
        module_map,
    };

    modules.push_layer();
    let result = resolve_statements(statements, &mut modules);
    modules.pop_layer();
    result
}

fn resolve_statements<'a>(
    statements: Vec<Statement>,
    modules: &mut EnabledModules<'a>,
) -> Vec<Statement> {
    let mut result = vec![];
    for statement in statements {
        match statement {
            Statement::Assign(Assign { target, value }) => result.push(Statement::Assign(Assign {
                target: resolve_expr(target, modules),
                value: resolve_expr(value, modules),
            })),
            Statement::Expression(expr) => {
                result.push(Statement::Expression(resolve_expr(expr, modules)))
            }
            Statement::If(If { test, body, orelse }) => result.push(Statement::If(If {
                test: resolve_expr(test, modules),
                body: resolve_statements(body, modules),
                orelse: resolve_statements(orelse, modules),
            })),
            Statement::Func(Func { name, args, body }) => {
                modules.push_layer();
                let body = resolve_statements(body, modules);
                modules.pop_layer();
                result.push(Statement::Func(Func { name, args, body }));
            }
            Statement::Return(ret) => {
                result.push(Statement::Return(ret.map(|r| resolve_expr(r, modules))))
            }
            Statement::While(While { test, body }) => result.push(Statement::While(While {
                test: resolve_expr(test, modules),
                body: resolve_statements(body, modules),
            })),
            Statement::Break => result.push(Statement::Break),
            Statement::Continue => result.push(Statement::Continue),
            Statement::Import(import) => {
                modules.declare_import(&import);
            }
            Statement::FromImport(import) => {
                modules.declare_from_import(&import);
            }
        }
    }
    result
}

fn resolve_expr<'a>(expr: Expr, modules: &mut EnabledModules<'a>) -> Expr {
    match expr {
        Expr::CallFunction(CallFunction { name, args }) => {
            let expr = Expr::CallFunction(CallFunction {
                name,
                args: exprs(args, modules),
            });
            modules.query(&expr).unwrap_or(expr)
        }
        Expr::CallMethod(CallMethod { value, name, args }) => {
            let expr = Expr::CallMethod(CallMethod {
                value: Box::new(resolve_expr(*value, modules)),
                name,
                args: exprs(args, modules),
            });
            modules.query(&expr).unwrap_or(expr)
        }
        Expr::Tuple(tuple) => Expr::Tuple(exprs(tuple, modules)),
        Expr::BoolOperation(BoolOperation { op, conditions }) => {
            Expr::BoolOperation(BoolOperation {
                op,
                conditions: exprs(conditions, modules),
            })
        }
        Expr::Compare(Compare { left, right, op }) => Expr::Compare(Compare {
            left: Box::new(resolve_expr(*left, modules)),
            right: Box::new(resolve_expr(*right, modules)),
            op,
        }),
        Expr::UnaryOperation(UnaryOperation { value, op }) => {
            Expr::UnaryOperation(UnaryOperation {
                value: Box::new(resolve_expr(*value, modules)),
                op,
            })
        }
        Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
            Expr::BinaryOperation(BinaryOperation {
                left: Box::new(resolve_expr(*left, modules)),
                right: Box::new(resolve_expr(*right, modules)),
                op,
            })
        }
        Expr::Index(Index { value, index }) => Expr::Index(Index {
            value: Box::new(resolve_expr(*value, modules)),
            index: Box::new(resolve_expr(*index, modules)),
        }),
        Expr::ConstantNumber(_)
        | Expr::ConstantString(_)
        | Expr::ConstantBoolean(_)
        | Expr::None
        | Expr::VariableName(_) => expr,
        Expr::List(list) => Expr::List(exprs(list, modules)),
        Expr::Dict(Dict { pairs }) => {
            let (keys, values) = pairs.into_iter().unzip();
            let keys = exprs(keys, modules);
            let values = exprs(values, modules);
            let pairs = keys.into_iter().zip(values).collect();
            Expr::Dict(Dict { pairs })
        }
    }
}
fn exprs<'a>(exprs: Vec<Expr>, modules: &mut EnabledModules<'a>) -> Vec<Expr> {
    exprs
        .into_iter()
        .map(|expr| resolve_expr(expr, modules))
        .collect()
}

struct EnabledModules<'a> {
    layers: Vec<BTreeMap<String, String>>,
    module_map: &'a ModuleMap,
}

impl<'a> EnabledModules<'a> {
    fn push_layer(&mut self) {
        self.layers.push(BTreeMap::new());
    }
    fn pop_layer(&mut self) {
        self.layers.pop().expect("there's no layer");
    }
    fn declare_import(&mut self, import: &Import) {
        let layer = self.layers.last_mut().expect("layers should not be empty.");
        let imported = self.module_map.find_children(&import.import);
        for (python_function, actual_function) in imported {
            layer.insert(
                format!("{}.{python_function}", import.alias),
                actual_function.to_string(),
            );
        }
    }
    fn declare_from_import(&mut self, import: &FromImport) {
        let layer = self.layers.last_mut().expect("layers should not be empty.");
        if import.import == "*" {
            let imported = self.module_map.find_children(&import.from);
            for (python_function, actual_function) in imported {
                layer.insert(python_function.to_string(), actual_function.to_string());
            }
        } else {
            let ident = format!("{}.{}", import.from, import.import);
            if let Some(actual_function) = self.module_map.find_match(&ident) {
                layer.insert(import.alias.to_string(), actual_function.to_string());
            }
        }
    }

    fn query(&self, expr: &Expr) -> Option<Expr> {
        match expr {
            Expr::CallFunction(CallFunction { name, args }) => {
                let replaced = self.find(name)?;
                Some(Expr::CallFunction(CallFunction {
                    name: replaced.to_string(),
                    args: args.clone(),
                }))
            }
            Expr::CallMethod(CallMethod { value, name, args }) => {
                let ident = format_value_chain(value)?;
                let ident = format!("{}.{}", ident, name);
                let replaced = self.find(&ident)?;
                Some(Expr::CallFunction(CallFunction {
                    name: replaced.to_string(),
                    args: args.clone(),
                }))
            }
            _ => unreachable!(),
        }
    }

    fn find(&self, ident: &str) -> Option<&String> {
        for layer in self.layers.iter().rev() {
            if let Some(ident) = layer.get(ident) {
                return Some(ident);
            }
        }
        None
    }
}

// TODO attribute
fn format_value_chain(value: &Expr) -> Option<String> {
    match value {
        Expr::VariableName(name) => Some(name.to_string()),
        _ => None,
    }
}
