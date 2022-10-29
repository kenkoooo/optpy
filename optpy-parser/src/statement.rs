use rustpython_parser::ast::StatementType;

use crate::expression::OptpyExpression;

#[derive(Debug, PartialEq, Eq)]
pub enum OptpyStatement {
    Assign {
        target: OptpyExpression,
        value: OptpyExpression,
    },
    Expression(OptpyExpression),
    If {
        test: OptpyExpression,
        body: Vec<OptpyStatement>,
        orelse: Vec<OptpyStatement>,
    },
}

impl OptpyStatement {
    pub fn parse(statement: &StatementType) -> Self {
        match statement {
            StatementType::Assign { targets, value } => {
                assert_eq!(targets.len(), 1);
                let target = OptpyExpression::parse(&targets[0].node);
                let value = OptpyExpression::parse(&value.node);
                Self::Assign { target, value }
            }
            StatementType::Expression { expression } => {
                Self::Expression(OptpyExpression::parse(&expression.node))
            }
            StatementType::If { test, body, orelse } => {
                let test = OptpyExpression::parse(&test.node);
                let body = parse_statements(body);
                let orelse = orelse
                    .as_ref()
                    .map(|s| parse_statements(s))
                    .unwrap_or_default();
                Self::If { test, body, orelse }
            }
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[rustpython_parser::ast::Statement]) -> Vec<OptpyStatement> {
    statements
        .iter()
        .map(|s| OptpyStatement::parse(&s.node))
        .collect()
}
