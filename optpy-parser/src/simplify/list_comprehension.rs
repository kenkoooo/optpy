use crate::{
    expression::{Dict, ListComprehension, RawExpr},
    statement::{Assign, FromImport, RawStmt},
    unixtime_nano, BinaryOperation, BoolOperation, BoolOperator, CallFunction, CallMethod, Compare,
    Expr, For, Func, If, Import, Index, UnaryOperation, While,
};

pub(crate) fn simplify_list_comprehensions(stmts: Vec<RawStmt<RawExpr>>) -> Vec<RawStmt<Expr>> {
    stmts.into_iter().flat_map(stmt).collect()
}

fn stmt(stmt: RawStmt<RawExpr>) -> Vec<RawStmt<Expr>> {
    match stmt {
        RawStmt::Assign(Assign { target, value }) => {
            let (target, mut s1) = eval_expr(target);
            let (value, s2) = eval_expr(value);
            s1.extend(s2);
            s1.push(RawStmt::Assign(Assign { target, value }));
            s1
        }
        RawStmt::Expression(e) => {
            let (e, mut s) = eval_expr(e);
            s.push(RawStmt::Expression(e));
            s
        }
        RawStmt::If(If { test, body, orelse }) => {
            let (test, mut s) = eval_expr(test);
            let body = simplify_list_comprehensions(body);
            let orelse = simplify_list_comprehensions(orelse);
            s.push(RawStmt::If(If { test, body, orelse }));
            s
        }
        RawStmt::Func(Func { name, args, body }) => {
            let body = simplify_list_comprehensions(body);
            vec![RawStmt::Func(Func { name, args, body })]
        }
        RawStmt::Return(None) => {
            vec![RawStmt::Return(None)]
        }
        RawStmt::Return(Some(r)) => {
            let (r, mut s) = eval_expr(r);
            s.push(RawStmt::Return(Some(r)));
            s
        }
        RawStmt::While(While { test, body }) => {
            let (test, mut s) = eval_expr(test);
            let body = simplify_list_comprehensions(body);
            s.push(RawStmt::While(While { test, body }));
            s
        }
        RawStmt::Break => vec![RawStmt::Break],
        RawStmt::Continue => vec![RawStmt::Continue],
        RawStmt::For(For { target, iter, body }) => {
            let (target, s) = eval_expr(target);
            assert!(s.is_empty(), "target contains list comprehension");
            let (iter, mut s) = eval_expr(iter);
            let body = simplify_list_comprehensions(body);
            s.push(RawStmt::For(For { target, iter, body }));
            s
        }
        RawStmt::Import(Import { import, alias }) => {
            vec![RawStmt::Import(Import { import, alias })]
        }
        RawStmt::FromImport(FromImport {
            from,
            import,
            alias,
        }) => {
            vec![RawStmt::FromImport(FromImport {
                from,
                import,
                alias,
            })]
        }
    }
}

fn exprs(exprs: Vec<RawExpr>) -> (Vec<Expr>, Vec<RawStmt<Expr>>) {
    let (exprs, stmts): (Vec<_>, Vec<_>) = exprs.into_iter().map(eval_expr).unzip();
    (exprs, stmts.into_iter().flatten().collect())
}

fn eval_expr(expr: RawExpr) -> (Expr, Vec<RawStmt<Expr>>) {
    match expr {
        RawExpr::CallFunction(CallFunction { name, args }) => {
            let (args, s) = exprs(args);
            (Expr::CallFunction(CallFunction { name, args }), s)
        }
        RawExpr::CallMethod(CallMethod { value, name, args }) => {
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
        RawExpr::Tuple(tuple) => {
            let (tuple, s) = exprs(tuple);
            (Expr::Tuple(tuple), s)
        }
        RawExpr::VariableName(v) => (Expr::VariableName(v), vec![]),
        RawExpr::ConstantNumber(v) => (Expr::ConstantNumber(v), vec![]),
        RawExpr::ConstantString(v) => (Expr::ConstantString(v), vec![]),
        RawExpr::ConstantBoolean(v) => (Expr::ConstantBoolean(v), vec![]),
        RawExpr::None => (Expr::None, vec![]),
        RawExpr::BoolOperation(BoolOperation { op, conditions }) => {
            let (conditions, s) = exprs(conditions);
            (Expr::BoolOperation(BoolOperation { op, conditions }), s)
        }
        RawExpr::Compare(Compare { left, right, op }) => {
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
        RawExpr::BinaryOperation(BinaryOperation { left, right, op }) => {
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
        RawExpr::Index(Index { value, index }) => {
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
        RawExpr::List(list) => {
            let (list, s) = exprs(list);
            (Expr::List(list), s)
        }
        RawExpr::Dict(Dict { pairs }) => {
            let mut statements = vec![];
            let mut p = vec![];
            for (key, value) in pairs {
                let (key, s) = eval_expr(key);
                statements.extend(s);
                let (value, s) = eval_expr(value);
                statements.extend(s);
                p.push((key, value));
            }
            (Expr::Dict(Dict { pairs: p }), statements)
        }
        RawExpr::ListComprehension(ListComprehension { value, generators }) => {
            let tmp_list = Expr::VariableName("__result".into());
            let (value, mut generation_body) = eval_expr(*value);
            generation_body.push(RawStmt::Expression(Expr::CallMethod(CallMethod {
                value: Box::new(tmp_list.clone()),
                name: "append".into(),
                args: vec![value],
            })));

            for generator in generators {
                let (ifs, s) = exprs(generator.ifs);
                assert!(
                    s.is_empty(),
                    "filter statement in a list comprehension also contains list comprehension"
                );
                if !ifs.is_empty() {
                    generation_body = vec![RawStmt::If(If {
                        test: Expr::BoolOperation(BoolOperation {
                            op: BoolOperator::And,
                            conditions: ifs,
                        }),
                        body: generation_body,
                        orelse: vec![],
                    })];
                }
                let (iter, mut new_generation_body) = eval_expr(*generator.iter);
                let (target, s) = eval_expr(*generator.target);
                assert!(
                    s.is_empty(),
                    "list generator target contains list comprehension"
                );
                new_generation_body.push(RawStmt::For(For {
                    target,
                    iter,
                    body: generation_body,
                }));
                generation_body = new_generation_body;
            }

            let mut function_body = vec![RawStmt::Assign(Assign {
                target: tmp_list.clone(),
                value: Expr::List(vec![]),
            })];
            function_body.extend(generation_body);
            function_body.push(RawStmt::Return(Some(tmp_list)));

            let function_name = format!("__f{}", unixtime_nano());

            (
                Expr::CallFunction(CallFunction {
                    name: function_name.clone(),
                    args: vec![],
                }),
                vec![RawStmt::Func(Func {
                    name: function_name,
                    args: vec![],
                    body: function_body,
                })],
            )
        }
        RawExpr::UnaryOperation(UnaryOperation { value, op }) => {
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
