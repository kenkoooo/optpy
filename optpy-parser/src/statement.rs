use rustpython_parser::ast::StatementType;

use crate::{expression::Expr, Number};

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
