use rustpython_parser::ast::{Stmt, StmtKind};

use crate::{
    expression::{Expr, RawExpr},
    unixtime_nano, BinaryOperation, BinaryOperator, CallMethod, Index,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Assign(Assign<Expr>),
    Expression(Expr),
    If(If<Statement, Expr>),
    Func(Func<Statement>),
    Return(Option<Expr>),
    While(While<Statement, Expr>),
    Break,
    Continue,
    Import(Import),
    FromImport(FromImport),
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Assign<E> {
    pub target: E,
    pub value: E,
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct If<S, E> {
    pub test: E,
    pub body: Vec<S>,
    pub orelse: Vec<S>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Func<S> {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<S>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct While<S, E> {
    pub test: E,
    pub body: Vec<S>,
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct For<S, E> {
    pub(crate) target: E,
    pub(crate) iter: E,
    pub(crate) body: Vec<S>,
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Import {
    pub import: String,
    pub alias: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct FromImport {
    pub from: String,
    pub import: String,
    pub alias: String,
}

impl RawStmt<RawExpr> {
    pub fn parse(statement: &StmtKind) -> Vec<Self> {
        match statement {
            StmtKind::Assign {
                targets,
                value,
                type_comment: _,
            } => {
                let value = RawExpr::parse(&value.node);
                if targets.len() == 1 {
                    let target = RawExpr::parse(&targets[0].node);
                    vec![Self::Assign(Assign { target, value })]
                } else {
                    let first_target =
                        RawExpr::VariableName(format!("__assign_tmp_{}", unixtime_nano()));
                    let mut result = vec![Self::Assign(Assign {
                        target: first_target.clone(),
                        value,
                    })];
                    for target in targets {
                        let target = RawExpr::parse(&target.node);
                        result.push(Self::Assign(Assign {
                            target,
                            value: first_target.clone(),
                        }));
                    }
                    result
                }
            }
            StmtKind::Expr { value } => vec![Self::Expression(RawExpr::parse(&value.node))],
            StmtKind::If { test, body, orelse } => {
                let test = RawExpr::parse(&test.node);
                let body = parse_statements(body);
                let orelse = parse_statements(orelse);
                vec![Self::If(If { test, body, orelse })]
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
                vec![Self::Func(Func { name, args, body })]
            }
            StmtKind::Return { value } => {
                let value = value.as_ref().map(|value| RawExpr::parse(&value.node));
                vec![Self::Return(value)]
            }
            StmtKind::While {
                test,
                body,
                orelse: _,
            } => {
                let test = RawExpr::parse(&test.node);
                let body = parse_statements(body);
                vec![Self::While(While { test, body })]
            }
            StmtKind::For {
                target,
                iter,
                body,
                orelse: _,
                type_comment: _,
            } => {
                let target = RawExpr::parse(&target.node);
                let iter = RawExpr::parse(&iter.node);
                let body = parse_statements(body);
                vec![Self::For(For { target, iter, body })]
            }
            StmtKind::Break => vec![Self::Break],
            StmtKind::Continue => vec![Self::Continue],
            StmtKind::AugAssign { target, op, value } => {
                let target = RawExpr::parse(&target.node);
                let value = RawExpr::parse(&value.node);
                vec![Self::Assign(Assign {
                    target: target.clone(),
                    value: RawExpr::BinaryOperation(BinaryOperation {
                        left: Box::new(target),
                        right: Box::new(value),
                        op: BinaryOperator::parse(op),
                    }),
                })]
            }
            StmtKind::Pass => vec![],
            StmtKind::Delete { targets } => targets
                .iter()
                .map(|target| {
                    let target = RawExpr::parse(&target.node);
                    match target {
                        RawExpr::Index(Index { value, index }) => {
                            Self::Expression(RawExpr::CallMethod(CallMethod {
                                value,
                                name: "__delete".into(),
                                args: vec![*index],
                            }))
                        }
                        target => Self::Assign(Assign {
                            target,
                            value: RawExpr::None,
                        }),
                    }
                })
                .collect(),
            StmtKind::ImportFrom {
                module,
                names,
                level: _,
            } => {
                let mut statements = vec![];
                let module = module.as_ref().cloned().expect("unknown module syntax");
                for name in names {
                    let import = name.node.name.clone();
                    let alias = name
                        .node
                        .asname
                        .as_ref()
                        .map(|alias| alias.clone())
                        .unwrap_or_else(|| import.clone());
                    statements.push(RawStmt::FromImport(FromImport {
                        from: module.clone(),
                        import,
                        alias,
                    }));
                }
                statements
            }
            StmtKind::Import { names } => {
                let mut statements = vec![];
                for name in names {
                    let import = name.node.name.clone();
                    let alias = name
                        .node
                        .asname
                        .as_ref()
                        .map(|alias| alias.clone())
                        .unwrap_or_else(|| import.clone());
                    statements.push(RawStmt::Import(Import { import, alias }));
                }
                statements
            }
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[Stmt]) -> Vec<RawStmt<RawExpr>> {
    statements
        .iter()
        .flat_map(|s| RawStmt::parse(&s.node))
        .collect()
}

pub(crate) enum RawStmt<E> {
    Assign(Assign<E>),
    Expression(E),
    If(If<RawStmt<E>, E>),
    Func(Func<RawStmt<E>>),
    Return(Option<E>),
    While(While<RawStmt<E>, E>),
    Break,
    Continue,
    For(For<RawStmt<E>, E>),
    Import(Import),
    FromImport(FromImport),
}
