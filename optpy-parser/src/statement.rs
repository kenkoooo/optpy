use rustpython_parser::ast::{Stmt, StmtKind};

use crate::{expression::Expr, BinaryOperator};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    For {
        target: Expr,
        iter: Expr,
        body: Vec<Statement>,
    },
}

impl Statement {
    pub fn parse(statement: &StmtKind) -> Self {
        match statement {
            StmtKind::Assign {
                targets,
                value,
                type_comment: _,
            } => {
                assert_eq!(targets.len(), 1);
                let target = Expr::parse(&targets[0].node);
                let value = Expr::parse(&value.node);
                Self::Assign { target, value }
            }
            StmtKind::Expr { value } => Self::Expression(Expr::parse(&value.node)),
            StmtKind::If { test, body, orelse } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                let orelse = parse_statements(orelse);
                Self::If { test, body, orelse }
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
                Self::Func { name, args, body }
            }
            StmtKind::Return { value } => {
                let value = value.as_ref().map(|value| Expr::parse(&value.node));
                Self::Return(value)
            }
            StmtKind::While {
                test,
                body,
                orelse: _,
            } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                Self::While { test, body }
            }
            StmtKind::For {
                target,
                iter,
                body,
                orelse: _,
                type_comment: _,
            } => {
                let target = Expr::parse(&target.node);
                let iter = Expr::parse(&iter.node);
                let body = parse_statements(body);
                Self::For { target, iter, body }
            }
            StmtKind::Break => Statement::Break,
            StmtKind::AugAssign { target, op, value } => {
                let target = Expr::parse(&target.node);
                let value = Expr::parse(&value.node);
                Statement::Assign {
                    target: target.clone(),
                    value: Expr::BinaryOperation {
                        left: Box::new(target),
                        right: Box::new(value),
                        op: BinaryOperator::parse(op),
                    },
                }
            }
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[Stmt]) -> Vec<Statement> {
    statements
        .iter()
        .map(|s| Statement::parse(&s.node))
        .collect()
}
