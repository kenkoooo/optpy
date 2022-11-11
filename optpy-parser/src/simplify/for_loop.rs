use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{BoolOperator, CompareOperator, Expr, Number, Statement};

pub(crate) fn simplify_for_loops(stmts: Vec<Statement>) -> Vec<Statement> {
    stmts.into_iter().flat_map(simplify_statement).collect()
}

fn simplify_statement(stmt: Statement) -> Vec<Statement> {
    match stmt {
        Statement::Assign { target, value } => {
            vec![Statement::Assign { target, value }]
        }
        Statement::Expression(e) => vec![Statement::Expression(e)],
        Statement::If { test, body, orelse } => {
            let body = simplify_for_loops(body);
            let orelse = simplify_for_loops(orelse);
            vec![Statement::If { test, body, orelse }]
        }
        Statement::Func { name, args, body } => {
            let body = simplify_for_loops(body);
            vec![Statement::Func { name, args, body }]
        }
        Statement::Return(e) => vec![Statement::Return(e)],
        Statement::While { test, body } => {
            let body = simplify_for_loops(body);
            vec![Statement::While { test, body }]
        }
        Statement::Break => vec![Statement::Break],
        Statement::For { target, iter, body } => {
            let mut hasher = DefaultHasher::new();
            body.hash(&mut hasher);
            let hash = hasher.finish();
            let tmp_target = Expr::VariableName(format!("__tmp_for_loop_iter_{}", hash));

            let mut while_body = vec![Statement::Assign {
                target,
                value: Expr::CallMethod {
                    value: Box::new(tmp_target.clone()),
                    name: "pop".into(),
                    args: vec![],
                },
            }];
            while_body.extend(simplify_for_loops(body));

            vec![
                Statement::Assign {
                    target: tmp_target.clone(),
                    value: Expr::CallFunction {
                        name: "list".into(),
                        args: vec![iter],
                    },
                },
                Statement::Expression(Expr::CallMethod {
                    value: Box::new(tmp_target.clone()),
                    name: "reverse".into(),
                    args: vec![],
                }),
                Statement::While {
                    test: Expr::BoolOperation {
                        op: BoolOperator::And,
                        conditions: vec![Expr::Compare {
                            left: Box::new(Expr::CallFunction {
                                name: "len".into(),
                                args: vec![tmp_target.clone()],
                            }),
                            op: CompareOperator::Greater,
                            right: Box::new(Expr::ConstantNumber(Number::Int("0".into()))),
                        }],
                    },
                    body: while_body,
                },
            ]
        }
    }
}