mod expression;
pub use expression::OptpyExpression;

mod statement;
pub use statement::OptpyStatement;

use anyhow::Result;

pub fn parse(code: &str) -> Result<Vec<OptpyStatement>> {
    let ast = rustpython_parser::parser::parse_program(code)?;
    let statements = ast
        .statements
        .iter()
        .map(|s| OptpyStatement::parse(&s.node))
        .collect();
    Ok(statements)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic() -> Result<()> {
        let code = r"
a, b, c = input().split()
print(a)
";
        let statements = parse(code)?;
        assert_eq!(
            statements,
            [
                OptpyStatement::Assign {
                    target: OptpyExpression::Tuple(vec![
                        OptpyExpression::Ident("a".into()),
                        OptpyExpression::Ident("b".into()),
                        OptpyExpression::Ident("c".into())
                    ]),
                    value: OptpyExpression::CallMethod {
                        value: Box::new(OptpyExpression::CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                OptpyStatement::Expression(OptpyExpression::CallFunction {
                    name: "print".into(),
                    args: vec![OptpyExpression::Ident("a".into())]
                })
            ]
        );
        Ok(())
    }

    #[test]
    fn test_if_statement() -> Result<()> {
        let code = r"
a, b, c = input().split()
if a <= c < b:
    result = 1
else:
    result = 2
print(result)
";
        let statements = parse(code)?;
        assert_eq!(
            statements,
            [
                OptpyStatement::Assign {
                    target: OptpyExpression::Tuple(vec![
                        OptpyExpression::Ident("a".into()),
                        OptpyExpression::Ident("b".into()),
                        OptpyExpression::Ident("c".into())
                    ]),
                    value: OptpyExpression::CallMethod {
                        value: Box::new(OptpyExpression::CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                OptpyStatement::If {
                    test: OptpyExpression::BoolOperation {
                        op: expression::BoolOperator::And,
                        conditions: vec![
                            OptpyExpression::Compare {
                                left: Box::new(OptpyExpression::Ident("a".into())),
                                right: Box::new(OptpyExpression::Ident("c".into())),
                                op: expression::CompareOperator::LessOrEqual
                            },
                            OptpyExpression::Compare {
                                left: Box::new(OptpyExpression::Ident("c".into())),
                                right: Box::new(OptpyExpression::Ident("b".into())),
                                op: expression::CompareOperator::Less
                            }
                        ]
                    },
                    body: vec![OptpyStatement::Assign {
                        target: OptpyExpression::Ident("result".into()),
                        value: OptpyExpression::Number(expression::Number::Int("1".into()))
                    }],
                    orelse: vec![OptpyStatement::Assign {
                        target: OptpyExpression::Ident("result".into()),
                        value: OptpyExpression::Number(expression::Number::Int("2".into()))
                    }]
                },
                OptpyStatement::Expression(OptpyExpression::CallFunction {
                    name: "print".into(),
                    args: vec![OptpyExpression::Ident("result".into())]
                }),
            ]
        );
        Ok(())
    }
}
