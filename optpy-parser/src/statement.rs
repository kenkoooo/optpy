use rustpython_parser::ast::{ExprKind, Stmt, StmtKind};

use crate::{expression::Expr, BinaryOperator, BoolOperator, CompareOperator, Number};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Assign {
        target: Expr,
        value: Expr,
    },
    Expression(Expr),
    If {
        test: Expr,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
    },
    Func {
        name: String,
        args: Vec<String>,
        body: Vec<Statement>,
    },
    Return(Option<Expr>),
    While {
        test: Expr,
        body: Vec<Statement>,
    },
    Break,
}

impl Statement {
    pub fn parse(statement: &StmtKind) -> Vec<Self> {
        match statement {
            StmtKind::Assign {
                targets,
                value,
                type_comment: _,
            } => {
                assert_eq!(targets.len(), 1);
                parse_assignment(&targets[0].node, Expr::parse(&value.node))
            }
            StmtKind::Expr { value } => {
                vec![Self::Expression(Expr::parse(&value.node))]
            }
            StmtKind::If { test, body, orelse } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                let orelse = parse_statements(orelse);
                vec![Self::If { test, body, orelse }]
            }
            StmtKind::FunctionDef {
                decorator_list: _,
                returns: _,
                name,
                args,
                body,
                type_comment: _,
            } => {
                let name = name.to_string();
                let args = args.args.iter().map(|arg| arg.node.arg.clone()).collect();
                let body = parse_statements(body);
                vec![Self::Func { name, args, body }]
            }
            StmtKind::Return { value } => {
                let value = value.as_ref().map(|value| Expr::parse(&value.node));
                vec![Self::Return(value)]
            }
            StmtKind::While {
                test,
                body,
                orelse: _,
            } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                vec![Statement::While { test, body }]
            }
            StmtKind::For {
                target,
                iter,
                body,
                orelse: _,
                type_comment: _,
            } => {
                // __tmp_variable_for_for_loop_ = list(iter)
                // __tmp_variable_for_for_loop_.reverse()
                // while len(__tmp_variable_for_for_loop_) > 0:
                //     target = __tmp_variable_for_for_loop_.pop()
                //     ..
                let tmp_variable = Expr::VariableName(format!(
                    "__tmp_variable_for_for_loop_{}_{}",
                    iter.location.row(),
                    iter.location.column()
                ));
                let iter = Expr::parse(&iter.node);
                let mut while_body = parse_assignment(
                    &target.node,
                    Expr::CallMethod {
                        value: Box::new(tmp_variable.clone()),
                        name: "pop".into(),
                        args: vec![],
                    },
                );
                while_body.extend(parse_statements(body));

                vec![
                    Statement::Assign {
                        target: tmp_variable.clone(),
                        value: Expr::CallFunction {
                            name: "list".to_string(),
                            args: vec![iter],
                        },
                    },
                    Statement::Expression(Expr::CallMethod {
                        value: Box::new(tmp_variable.clone()),
                        name: "reverse".into(),
                        args: vec![],
                    }),
                    Statement::While {
                        test: Expr::BoolOperation {
                            op: BoolOperator::And,
                            conditions: vec![Expr::Compare {
                                left: Box::new(Expr::CallFunction {
                                    name: "len".into(),
                                    args: vec![tmp_variable.clone()],
                                }),
                                right: Box::new(Expr::ConstantNumber(Number::Int("0".into()))),
                                op: CompareOperator::Greater,
                            }],
                        },
                        body: while_body,
                    },
                ]
            }
            StmtKind::Break => vec![Statement::Break],
            StmtKind::AugAssign { target, op, value } => {
                let target = Expr::parse(&target.node);
                let value = Expr::parse(&value.node);
                vec![Statement::Assign {
                    target: target.clone(),
                    value: Expr::BinaryOperation {
                        left: Box::new(target),
                        right: Box::new(value),
                        op: BinaryOperator::parse(op),
                    },
                }]
            }
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[Stmt]) -> Vec<Statement> {
    statements
        .iter()
        .flat_map(|s| Statement::parse(&s.node))
        .collect()
}

fn parse_assignment(target: &ExprKind, value: Expr) -> Vec<Statement> {
    match target {
        ExprKind::Tuple { elts, ctx: _ } => {
            let mut result = vec![];
            let tmp_target = Expr::VariableName("__tmp_for_tuple".into());
            result.push(Statement::Assign {
                target: tmp_target.clone(),
                value,
            });

            for (i, element) in elts.iter().enumerate() {
                result.push(Statement::Assign {
                    target: Expr::parse(&element.node),
                    value: Expr::Index {
                        value: Box::new(tmp_target.clone()),
                        index: Box::new(Expr::ConstantNumber(Number::Int(i.to_string()))),
                    },
                });
            }
            result
        }
        target => {
            let target = Expr::parse(target);
            vec![Statement::Assign { target, value }]
        }
    }
}
