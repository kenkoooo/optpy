use crate::{statement::Assign, Expr, Number, Statement};

pub(crate) fn simplify_tuple_assignments(stmts: Vec<Statement>) -> Vec<Statement> {
    stmts.into_iter().flat_map(simplify_stmt).collect()
}

fn simplify_stmt(stmt: Statement) -> Vec<Statement> {
    match stmt {
        Statement::Assign(Assign { target, value }) => {
            if let Expr::Tuple(targets) = target {
                let tmp_target = Expr::VariableName("__tmp_for_tuple".into());
                let mut result = vec![Statement::Assign(Assign {
                    target: tmp_target.clone(),
                    value,
                })];
                for (i, target) in targets.into_iter().enumerate() {
                    result.push(Statement::Assign(Assign {
                        target,
                        value: Expr::Index {
                            value: Box::new(tmp_target.clone()),
                            index: Box::new(Expr::ConstantNumber(Number::Int(i.to_string()))),
                        },
                    }))
                }
                result
            } else {
                vec![Statement::Assign(Assign { target, value })]
            }
        }
        Statement::If { test, body, orelse } => {
            let body = simplify_tuple_assignments(body);
            let orelse = simplify_tuple_assignments(orelse);
            vec![Statement::If { test, body, orelse }]
        }
        Statement::Func { name, args, body } => {
            let body = simplify_tuple_assignments(body);
            vec![Statement::Func { name, args, body }]
        }
        Statement::While { test, body } => {
            let body = simplify_tuple_assignments(body);
            vec![Statement::While { test, body }]
        }
        Statement::For { target, iter, body } => {
            let body = simplify_tuple_assignments(body);
            vec![Statement::For { target, iter, body }]
        }
        Statement::Return(_) | Statement::Expression(_) | Statement::Break => vec![stmt],
    }
}
