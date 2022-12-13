use crate::{
    statement::{Assign, FromImport, RawStmt},
    unixtime_nano, CallFunction, Expr, For, Func, If, Import, Statement, While,
};

pub(crate) fn simplify_for_loops(stmts: Vec<RawStmt<Expr>>) -> Vec<Statement> {
    stmts.into_iter().flat_map(simplify_statement).collect()
}

fn simplify_statement(stmt: RawStmt<Expr>) -> Vec<Statement> {
    match stmt {
        RawStmt::Assign(Assign { target, value }) => {
            vec![Statement::Assign(Assign { target, value })]
        }
        RawStmt::Expression(e) => vec![Statement::Expression(e)],
        RawStmt::If(If { test, body, orelse }) => {
            let body = simplify_for_loops(body);
            let orelse = simplify_for_loops(orelse);
            vec![Statement::If(If { test, body, orelse })]
        }
        RawStmt::Func(Func { name, args, body }) => {
            let body = simplify_for_loops(body);
            vec![Statement::Func(Func { name, args, body })]
        }
        RawStmt::Return(e) => vec![Statement::Return(e)],
        RawStmt::While(While { test, body }) => {
            let body = simplify_for_loops(body);
            vec![Statement::While(While { test, body })]
        }
        RawStmt::Break => vec![Statement::Break],
        RawStmt::Continue => vec![Statement::Continue],
        RawStmt::For(For { target, iter, body }) => {
            let tmp_iter = Expr::VariableName(format!("__tmp_for_loop_iter_{}", unixtime_nano()));

            let mut while_body = vec![Statement::Assign(Assign {
                target,
                value: Expr::CallFunction(CallFunction {
                    name: "next".into(),
                    args: vec![tmp_iter.clone()],
                }),
            })];
            while_body.extend(simplify_for_loops(body));

            vec![
                Statement::Assign(Assign {
                    target: tmp_iter.clone(),
                    value: Expr::CallFunction(CallFunction {
                        name: "iter".into(),
                        args: vec![iter],
                    }),
                }),
                Statement::While(While {
                    test: Expr::CallFunction(CallFunction {
                        name: "__has_next".into(),
                        args: vec![tmp_iter],
                    }),
                    body: while_body,
                }),
            ]
        }
        RawStmt::Import(Import { import, alias }) => {
            vec![Statement::Import(Import { import, alias })]
        }
        RawStmt::FromImport(FromImport {
            import,
            alias,
            from,
        }) => {
            vec![Statement::FromImport(FromImport {
                import,
                alias,
                from,
            })]
        }
    }
}
