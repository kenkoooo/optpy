use rustpython_parser::ast::StatementType;

use crate::{expression::Expr, BoolOperator, CompareOperator, Number};

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
    pub fn parse(statement: &StatementType) -> Vec<Self> {
        match statement {
            StatementType::Assign { targets, value } => {
                assert_eq!(targets.len(), 1);
                let value = Expr::parse(&value.node);
                match &targets[0].node {
                    rustpython_parser::ast::ExpressionType::Tuple { elements } => {
                        let mut result = vec![];
                        let tmp_target = Expr::VariableName("__tmp_for_tuple".into());
                        result.push(Self::Assign {
                            target: tmp_target.clone(),
                            value,
                        });

                        for (i, element) in elements.iter().enumerate() {
                            result.push(Self::Assign {
                                target: Expr::parse(&element.node),
                                value: Expr::Index {
                                    value: Box::new(tmp_target.clone()),
                                    index: Box::new(Expr::Number(Number::Int(i.to_string()))),
                                },
                            });
                        }
                        result
                    }
                    target => {
                        let target = Expr::parse(target);
                        vec![Self::Assign { target, value }]
                    }
                }
            }
            StatementType::Expression { expression } => {
                vec![Self::Expression(Expr::parse(&expression.node))]
            }
            StatementType::If { test, body, orelse } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                let orelse = orelse
                    .as_ref()
                    .map(|s| parse_statements(s))
                    .unwrap_or_default();
                vec![Self::If { test, body, orelse }]
            }
            StatementType::FunctionDef {
                is_async: _,
                decorator_list: _,
                returns: _,
                name,
                args,
                body,
            } => {
                let name = name.to_string();
                let args = args.args.iter().map(|arg| arg.arg.clone()).collect();
                let body = parse_statements(body);
                vec![Self::Func { name, args, body }]
            }
            StatementType::Return { value } => {
                let value = value.as_ref().map(|value| Expr::parse(&value.node));
                vec![Self::Return(value)]
            }
            StatementType::While {
                test,
                body,
                orelse: _,
            } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                vec![Statement::While { test, body }]
            }
            StatementType::For {
                is_async: _,
                target,
                iter,
                body,
                orelse: _,
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
                let target = Expr::parse(&target.node);
                let iter = Expr::parse(&iter.node);
                let mut while_body = vec![Statement::Assign {
                    target: target.clone(),
                    value: Expr::CallMethod {
                        value: Box::new(tmp_variable.clone()),
                        name: "pop".into(),
                        args: vec![],
                    },
                }];
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
                                right: Box::new(Expr::Number(Number::Int("0".into()))),
                                op: CompareOperator::Greater,
                            }],
                        },
                        body: while_body,
                    },
                ]
            }
            StatementType::Break => vec![Statement::Break],
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[rustpython_parser::ast::Statement]) -> Vec<Statement> {
    statements
        .iter()
        .flat_map(|s| Statement::parse(&s.node))
        .collect()
}
