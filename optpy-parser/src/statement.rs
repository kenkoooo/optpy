use rustpython_parser::ast::StatementType;

use crate::expression::Expr;

#[derive(Debug, PartialEq, Eq)]
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
}

impl Statement {
    pub fn parse(statement: &StatementType) -> Self {
        match statement {
            StatementType::Assign { targets, value } => {
                assert_eq!(targets.len(), 1);
                let target = Expr::parse(&targets[0].node);
                let value = Expr::parse(&value.node);
                Self::Assign { target, value }
            }
            StatementType::Expression { expression } => {
                Self::Expression(Expr::parse(&expression.node))
            }
            StatementType::If { test, body, orelse } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                let orelse = orelse
                    .as_ref()
                    .map(|s| parse_statements(s))
                    .unwrap_or_default();
                Self::If { test, body, orelse }
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
                Self::Func { name, args, body }
            }
            StatementType::Return { value } => {
                let value = value.as_ref().map(|value| Expr::parse(&value.node));
                Self::Return(value)
            }
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[rustpython_parser::ast::Statement]) -> Vec<Statement> {
    statements
        .iter()
        .map(|s| Statement::parse(&s.node))
        .collect()
}
