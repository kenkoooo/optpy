use rustpython_parser::ast::StatementType;

use crate::expression::OptpyExpression;

#[derive(Debug, PartialEq, Eq)]
pub enum OptpyStatement {
    Assign {
        target: OptpyExpression,
        value: OptpyExpression,
    },
    Expression(OptpyExpression),
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
            statement => todo!("{:?}", statement),
        }
    }
}
