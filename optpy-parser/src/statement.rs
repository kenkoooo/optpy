use rustpython_parser::ast::{Stmt, StmtKind};

use crate::{expression::Expr, BinaryOperator};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Statement {
    Assign(Assign),
    Expression(Expr),
    If(If<Statement>),
    Func(Func<Statement>),
    Return(Option<Expr>),
    While(While<Statement>),
    Break,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]

pub struct Assign {
    pub target: Expr,
    pub value: Expr,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]

pub struct If<S> {
    pub test: Expr,
    pub body: Vec<S>,
    pub orelse: Vec<S>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Func<S> {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<S>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct While<S> {
    pub test: Expr,
    pub body: Vec<S>,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]

pub struct For<S> {
    pub(crate) target: Expr,
    pub(crate) iter: Expr,
    pub(crate) body: Vec<S>,
}

impl RawStatement {
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
                Self::Assign(Assign { target, value })
            }
            StmtKind::Expr { value } => Self::Expression(Expr::parse(&value.node)),
            StmtKind::If { test, body, orelse } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                let orelse = parse_statements(orelse);
                Self::If(If { test, body, orelse })
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
                Self::Func(Func { name, args, body })
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
                Self::While(While { test, body })
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
                Self::For(For { target, iter, body })
            }
            StmtKind::Break => Self::Break,
            StmtKind::AugAssign { target, op, value } => {
                let target = Expr::parse(&target.node);
                let value = Expr::parse(&value.node);
                Self::Assign(Assign {
                    target: target.clone(),
                    value: Expr::BinaryOperation {
                        left: Box::new(target),
                        right: Box::new(value),
                        op: BinaryOperator::parse(op),
                    },
                })
            }
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[Stmt]) -> Vec<RawStatement> {
    statements
        .iter()
        .map(|s| RawStatement::parse(&s.node))
        .collect()
}

#[derive(Hash)]
pub(crate) enum RawStatement {
    Assign(Assign),
    Expression(Expr),
    If(If<RawStatement>),
    Func(Func<RawStatement>),
    Return(Option<Expr>),
    While(While<RawStatement>),
    Break,
    For(For<RawStatement>),
}
