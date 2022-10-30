mod expression;
pub use expression::Expr;

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

    use crate::expression::BinaryOperator;

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
                    target: Expr::Tuple(vec![
                        Expr::Ident("a".into()),
                        Expr::Ident("b".into()),
                        Expr::Ident("c".into())
                    ]),
                    value: Expr::CallMethod {
                        value: Box::new(Expr::CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                OptpyStatement::Expression(Expr::CallFunction {
                    name: "print".into(),
                    args: vec![Expr::Ident("a".into())]
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
                    target: Expr::Tuple(vec![
                        Expr::Ident("a".into()),
                        Expr::Ident("b".into()),
                        Expr::Ident("c".into())
                    ]),
                    value: Expr::CallMethod {
                        value: Box::new(Expr::CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                OptpyStatement::If {
                    test: Expr::BoolOperation {
                        op: expression::BoolOperator::And,
                        conditions: vec![
                            Expr::Compare {
                                left: Box::new(Expr::Ident("a".into())),
                                right: Box::new(Expr::Ident("c".into())),
                                op: expression::CompareOperator::LessOrEqual
                            },
                            Expr::Compare {
                                left: Box::new(Expr::Ident("c".into())),
                                right: Box::new(Expr::Ident("b".into())),
                                op: expression::CompareOperator::Less
                            }
                        ]
                    },
                    body: vec![OptpyStatement::Assign {
                        target: Expr::Ident("result".into()),
                        value: Expr::Number(expression::Number::Int("1".into()))
                    }],
                    orelse: vec![OptpyStatement::Assign {
                        target: Expr::Ident("result".into()),
                        value: Expr::Number(expression::Number::Int("2".into()))
                    }]
                },
                OptpyStatement::Expression(Expr::CallFunction {
                    name: "print".into(),
                    args: vec![Expr::Ident("result".into())]
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_function() -> Result<()> {
        let code = r"
a, b, c = input().split()
def f(d):
    return a + d
e = f(b)
print(e)
";
        let statements = parse(code)?;
        assert_eq!(
            statements,
            [
                OptpyStatement::Assign {
                    target: Expr::Tuple(vec![
                        Expr::Ident("a".into()),
                        Expr::Ident("b".into()),
                        Expr::Ident("c".into())
                    ]),
                    value: Expr::CallMethod {
                        value: Box::new(Expr::CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                OptpyStatement::Func {
                    name: "f".into(),
                    args: vec!["d".into()],
                    body: vec![OptpyStatement::Return(Some(Expr::BinaryOperation {
                        left: Box::new(Expr::Ident("a".into())),
                        right: Box::new(Expr::Ident("d".into())),
                        op: BinaryOperator::Add
                    }))]
                },
                OptpyStatement::Assign {
                    target: Expr::Ident("e".into()),
                    value: Expr::CallFunction {
                        name: "f".into(),
                        args: vec![Expr::Ident("b".into())]
                    }
                },
                OptpyStatement::Expression(Expr::CallFunction {
                    name: "print".into(),
                    args: vec![Expr::Ident("e".into())]
                })
            ]
        );
        Ok(())
    }
}
