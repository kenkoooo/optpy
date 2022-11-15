use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{statement::Assign, BoolOperator, Expr, For, If, Statement, While};

pub(crate) fn simplify_list_comprehensions(stmts: Vec<Statement>) -> Vec<Statement> {
    stmts.into_iter().flat_map(stmt).collect()
}

fn stmt(stmt: Statement) -> Vec<Statement> {
    match stmt {
        Statement::Assign(Assign { target, value }) => {
            let (target, mut s1) = eval_expr(target);
            let (value, s2) = eval_expr(value);
            s1.extend(s2);
            s1.push(Statement::Assign(Assign { target, value }));
            s1
        }
        Statement::Expression(e) => {
            let (e, mut s) = eval_expr(e);
            s.push(Statement::Expression(e));
            s
        }
        Statement::If(If { test, body, orelse }) => {
            let (test, mut s) = eval_expr(test);
            let body = simplify_list_comprehensions(body);
            let orelse = simplify_list_comprehensions(orelse);
            s.push(Statement::If(If { test, body, orelse }));
            s
        }
        Statement::Func { name, args, body } => {
            let body = simplify_list_comprehensions(body);
            vec![Statement::Func { name, args, body }]
        }
        Statement::Return(None) => {
            vec![Statement::Return(None)]
        }
        Statement::Return(Some(r)) => {
            let (r, mut s) = eval_expr(r);
            s.push(Statement::Return(Some(r)));
            s
        }
        Statement::While(While { test, body }) => {
            let (test, mut s) = eval_expr(test);
            let body = simplify_list_comprehensions(body);
            s.push(Statement::While(While { test, body }));
            s
        }
        Statement::Break => vec![Statement::Break],
        Statement::For(For { target, iter, body }) => {
            let (iter, mut s) = eval_expr(iter);
            let body = simplify_list_comprehensions(body);
            s.push(Statement::For(For { target, iter, body }));
            s
        }
    }
}

fn exprs(exprs: Vec<Expr>) -> (Vec<Expr>, Vec<Statement>) {
    let (exprs, stmts): (Vec<_>, Vec<_>) = exprs.into_iter().map(eval_expr).unzip();
    (exprs, stmts.into_iter().flatten().collect())
}

fn eval_expr(expr: Expr) -> (Expr, Vec<Statement>) {
    match expr {
        Expr::CallFunction { name, args } => {
            let (args, s) = exprs(args);
            (Expr::CallFunction { name, args }, s)
        }
        Expr::CallMethod { value, name, args } => {
            let (value, mut s1) = eval_expr(*value);
            let (args, s2) = exprs(args);
            s1.extend(s2);
            (
                Expr::CallMethod {
                    value: Box::new(value),
                    name,
                    args,
                },
                s1,
            )
        }
        Expr::Tuple(tuple) => {
            let (tuple, s) = exprs(tuple);
            (Expr::Tuple(tuple), s)
        }
        Expr::VariableName(_)
        | Expr::ConstantNumber(_)
        | Expr::ConstantString(_)
        | Expr::ConstantBoolean(_) => (expr, vec![]),
        Expr::BoolOperation { op, conditions } => {
            let (conditions, s) = exprs(conditions);
            (Expr::BoolOperation { op, conditions }, s)
        }
        Expr::Compare { left, right, op } => {
            let (left, mut s1) = eval_expr(*left);
            let (right, s2) = eval_expr(*right);
            s1.extend(s2);
            (
                Expr::Compare {
                    left: Box::new(left),
                    right: Box::new(right),
                    op,
                },
                s1,
            )
        }
        Expr::BinaryOperation { left, right, op } => {
            let (left, mut s1) = eval_expr(*left);
            let (right, s2) = eval_expr(*right);
            s1.extend(s2);
            (
                Expr::BinaryOperation {
                    left: Box::new(left),
                    right: Box::new(right),
                    op,
                },
                s1,
            )
        }
        Expr::Index { value, index } => {
            let (value, mut s1) = eval_expr(*value);
            let (index, s2) = eval_expr(*index);
            s1.extend(s2);
            (
                Expr::Index {
                    value: Box::new(value),
                    index: Box::new(index),
                },
                s1,
            )
        }
        Expr::List(list) => {
            let (list, s) = exprs(list);
            (Expr::List(list), s)
        }
        Expr::ListComprehension { value, generators } => {
            let tmp_list = Expr::VariableName("__result".into());
            let (value, mut generation_body) = eval_expr(*value);
            generation_body.push(Statement::Expression(Expr::CallMethod {
                value: Box::new(tmp_list.clone()),
                name: "append".into(),
                args: vec![value],
            }));

            for generator in generators {
                let ifs = generator.ifs;
                if !ifs.is_empty() {
                    generation_body = vec![Statement::If(If {
                        test: Expr::BoolOperation {
                            op: BoolOperator::And,
                            conditions: ifs,
                        },
                        body: generation_body,
                        orelse: vec![],
                    })];
                }
                let (iter, mut new_generation_body) = eval_expr(*generator.iter);
                let target = generator.target;
                new_generation_body.push(Statement::For(For {
                    target: *target,
                    iter: iter,
                    body: generation_body,
                }));
                generation_body = new_generation_body;
            }

            let mut function_body = vec![Statement::Assign(Assign {
                target: tmp_list.clone(),
                value: Expr::List(vec![]),
            })];
            function_body.extend(generation_body);
            function_body.push(Statement::Return(Some(tmp_list)));

            let mut hasher = DefaultHasher::new();
            function_body.hash(&mut hasher);
            let hash = hasher.finish();
            let function_name = format!("__f{}", hash);

            (
                Expr::CallFunction {
                    name: function_name.clone(),
                    args: vec![],
                },
                vec![Statement::Func {
                    name: function_name,
                    args: vec![],
                    body: function_body,
                }],
            )
        }
        Expr::UnaryOperation { value, op } => {
            let (value, s) = eval_expr(*value);
            (
                Expr::UnaryOperation {
                    value: Box::new(value),
                    op,
                },
                s,
            )
        }
    }
}
