use crate::{
    hash,
    statement::{Assign, FromImport, RawStmt},
    CallFunction, CallMethod, Compare, CompareOperator, Expr, For, Func, If, Import, Number,
    Statement, While,
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
            let hash = hash(&body);
            let tmp_target = Expr::VariableName(format!("__tmp_for_loop_iter_{}", hash));

            let mut while_body = vec![Statement::Assign(Assign {
                target,
                value: Expr::CallMethod(CallMethod {
                    value: Box::new(tmp_target.clone()),
                    name: "pop".into(),
                    args: vec![],
                }),
            })];
            while_body.extend(simplify_for_loops(body));

            vec![
                Statement::Assign(Assign {
                    target: tmp_target.clone(),
                    value: Expr::CallFunction(CallFunction {
                        name: "list".into(),
                        args: vec![iter],
                    }),
                }),
                Statement::Expression(Expr::CallMethod(CallMethod {
                    value: Box::new(tmp_target.clone()),
                    name: "reverse".into(),
                    args: vec![],
                })),
                Statement::While(While {
                    test: Expr::Compare(Compare {
                        left: Box::new(Expr::CallFunction(CallFunction {
                            name: "len".into(),
                            args: vec![tmp_target.clone()],
                        })),
                        op: CompareOperator::Greater,
                        right: Box::new(Expr::ConstantNumber(Number::Int("0".into()))),
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
