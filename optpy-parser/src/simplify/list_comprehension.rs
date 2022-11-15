use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    statement::{Assign, RawStatement},
    BinaryOperation, BoolOperation, BoolOperator, CallFunction, CallMethod, Compare, Expr, For,
    Func, If, Index, ListComprehension, UnaryOperation, While,
};

pub(crate) fn simplify_list_comprehensions(stmts: Vec<RawStatement>) -> Vec<RawStatement> {
    stmts.into_iter().flat_map(stmt).collect()
}

fn stmt(stmt: RawStatement) -> Vec<RawStatement> {
    match stmt {
        RawStatement::Assign(Assign { target, value }) => {
            let (target, mut s1) = eval_expr(target);
            let (value, s2) = eval_expr(value);
            s1.extend(s2);
            s1.push(RawStatement::Assign(Assign { target, value }));
            s1
        }
        RawStatement::Expression(e) => {
            let (e, mut s) = eval_expr(e);
            s.push(RawStatement::Expression(e));
            s
        }
        RawStatement::If(If { test, body, orelse }) => {
            let (test, mut s) = eval_expr(test);
            let body = simplify_list_comprehensions(body);
            let orelse = simplify_list_comprehensions(orelse);
            s.push(RawStatement::If(If { test, body, orelse }));
            s
        }
        RawStatement::Func(Func { name, args, body }) => {
            let body = simplify_list_comprehensions(body);
            vec![RawStatement::Func(Func { name, args, body })]
        }
        RawStatement::Return(None) => {
            vec![RawStatement::Return(None)]
        }
        RawStatement::Return(Some(r)) => {
            let (r, mut s) = eval_expr(r);
            s.push(RawStatement::Return(Some(r)));
            s
        }
        RawStatement::While(While { test, body }) => {
            let (test, mut s) = eval_expr(test);
            let body = simplify_list_comprehensions(body);
            s.push(RawStatement::While(While { test, body }));
            s
        }
        RawStatement::Break => vec![RawStatement::Break],
        RawStatement::For(For { target, iter, body }) => {
            let (iter, mut s) = eval_expr(iter);
            let body = simplify_list_comprehensions(body);
            s.push(RawStatement::For(For { target, iter, body }));
            s
        }
    }
}

fn exprs(exprs: Vec<Expr>) -> (Vec<Expr>, Vec<RawStatement>) {
    let (exprs, stmts): (Vec<_>, Vec<_>) = exprs.into_iter().map(eval_expr).unzip();
    (exprs, stmts.into_iter().flatten().collect())
}

fn eval_expr(expr: Expr) -> (Expr, Vec<RawStatement>) {
    match expr {
        Expr::CallFunction(CallFunction { name, args }) => {
            let (args, s) = exprs(args);
            (Expr::CallFunction(CallFunction { name, args }), s)
        }
        Expr::CallMethod(CallMethod { value, name, args }) => {
            let (value, mut s1) = eval_expr(*value);
            let (args, s2) = exprs(args);
            s1.extend(s2);
            (
                Expr::CallMethod(CallMethod {
                    value: Box::new(value),
                    name,
                    args,
                }),
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
        Expr::BoolOperation(BoolOperation { op, conditions }) => {
            let (conditions, s) = exprs(conditions);
            (Expr::BoolOperation(BoolOperation { op, conditions }), s)
        }
        Expr::Compare(Compare { left, right, op }) => {
            let (left, mut s1) = eval_expr(*left);
            let (right, s2) = eval_expr(*right);
            s1.extend(s2);
            (
                Expr::Compare(Compare {
                    left: Box::new(left),
                    right: Box::new(right),
                    op,
                }),
                s1,
            )
        }
        Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
            let (left, mut s1) = eval_expr(*left);
            let (right, s2) = eval_expr(*right);
            s1.extend(s2);
            (
                Expr::BinaryOperation(BinaryOperation {
                    left: Box::new(left),
                    right: Box::new(right),
                    op,
                }),
                s1,
            )
        }
        Expr::Index(Index { value, index }) => {
            let (value, mut s1) = eval_expr(*value);
            let (index, s2) = eval_expr(*index);
            s1.extend(s2);
            (
                Expr::Index(Index {
                    value: Box::new(value),
                    index: Box::new(index),
                }),
                s1,
            )
        }
        Expr::List(list) => {
            let (list, s) = exprs(list);
            (Expr::List(list), s)
        }
        Expr::ListComprehension(ListComprehension { value, generators }) => {
            let tmp_list = Expr::VariableName("__result".into());
            let (value, mut generation_body) = eval_expr(*value);
            generation_body.push(RawStatement::Expression(Expr::CallMethod(CallMethod {
                value: Box::new(tmp_list.clone()),
                name: "append".into(),
                args: vec![value],
            })));

            for generator in generators {
                let ifs = generator.ifs;
                if !ifs.is_empty() {
                    generation_body = vec![RawStatement::If(If {
                        test: Expr::BoolOperation(BoolOperation {
                            op: BoolOperator::And,
                            conditions: ifs,
                        }),
                        body: generation_body,
                        orelse: vec![],
                    })];
                }
                let (iter, mut new_generation_body) = eval_expr(*generator.iter);
                let target = generator.target;
                new_generation_body.push(RawStatement::For(For {
                    target: *target,
                    iter: iter,
                    body: generation_body,
                }));
                generation_body = new_generation_body;
            }

            let mut function_body = vec![RawStatement::Assign(Assign {
                target: tmp_list.clone(),
                value: Expr::List(vec![]),
            })];
            function_body.extend(generation_body);
            function_body.push(RawStatement::Return(Some(tmp_list)));

            let mut hasher = DefaultHasher::new();
            function_body.hash(&mut hasher);
            let hash = hasher.finish();
            let function_name = format!("__f{}", hash);

            (
                Expr::CallFunction(CallFunction {
                    name: function_name.clone(),
                    args: vec![],
                }),
                vec![RawStatement::Func(Func {
                    name: function_name,
                    args: vec![],
                    body: function_body,
                })],
            )
        }
        Expr::UnaryOperation(UnaryOperation { value, op }) => {
            let (value, s) = eval_expr(*value);
            (
                Expr::UnaryOperation(UnaryOperation {
                    value: Box::new(value),
                    op,
                }),
                s,
            )
        }
    }
}
