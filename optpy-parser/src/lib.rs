mod expression;
pub use expression::{BinaryOperator, BoolOperator, Expr};

mod statement;
use rustpython_parser::error::ParseError;
pub use statement::Statement;

pub fn parse<S: AsRef<str>>(code: S) -> Result<Vec<Statement>, ParseError> {
    let ast = rustpython_parser::parser::parse_program(code.as_ref())?;
    let statements = ast
        .statements
        .iter()
        .map(|s| Statement::parse(&s.node))
        .collect();
    Ok(statements)
}

#[cfg(test)]
mod tests {

    use crate::expression::BinaryOperator;

    use super::*;
    use Expr::*;
    use Statement::*;

    #[test]
    fn basic() {
        let code = r"
a, b, c = input().split()
print(a)
";
        let statements = parse(code).unwrap();
        assert_eq!(
            statements,
            [
                Assign {
                    target: Tuple(vec![
                        Ident("a".into()),
                        Ident("b".into()),
                        Ident("c".into())
                    ]),
                    value: CallMethod {
                        value: Box::new(CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                Expression(CallFunction {
                    name: "print".into(),
                    args: vec![Ident("a".into())]
                })
            ]
        );
    }

    #[test]
    fn test_if_statement() {
        let code = r"
a, b, c = input().split()
if a <= c < b:
    result = 1
else:
    result = 2
print(result)
";
        let statements = parse(code).unwrap();
        assert_eq!(
            statements,
            [
                Assign {
                    target: Tuple(vec![
                        Ident("a".into()),
                        Ident("b".into()),
                        Ident("c".into())
                    ]),
                    value: CallMethod {
                        value: Box::new(CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                If {
                    test: BoolOperation {
                        op: expression::BoolOperator::And,
                        conditions: vec![
                            Compare {
                                left: Box::new(Ident("a".into())),
                                right: Box::new(Ident("c".into())),
                                op: expression::CompareOperator::LessOrEqual
                            },
                            Compare {
                                left: Box::new(Ident("c".into())),
                                right: Box::new(Ident("b".into())),
                                op: expression::CompareOperator::Less
                            }
                        ]
                    },
                    body: vec![Assign {
                        target: Ident("result".into()),
                        value: Number(expression::Number::Int("1".into()))
                    }],
                    orelse: vec![Assign {
                        target: Ident("result".into()),
                        value: Number(expression::Number::Int("2".into()))
                    }]
                },
                Expression(CallFunction {
                    name: "print".into(),
                    args: vec![Ident("result".into())]
                }),
            ]
        );
    }

    #[test]
    fn test_function() {
        let code = r"
a, b, c = input().split()
def f(d):
    return a + d
e = f(b)
print(e)
";
        let statements = parse(code).unwrap();
        assert_eq!(
            statements,
            [
                Assign {
                    target: Tuple(vec![
                        Ident("a".into()),
                        Ident("b".into()),
                        Ident("c".into())
                    ]),
                    value: CallMethod {
                        value: Box::new(CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                Func {
                    name: "f".into(),
                    args: vec!["d".into()],
                    body: vec![Return(Some(BinaryOperation {
                        left: Box::new(Ident("a".into())),
                        right: Box::new(Ident("d".into())),
                        op: BinaryOperator::Add
                    }))]
                },
                Assign {
                    target: Ident("e".into()),
                    value: CallFunction {
                        name: "f".into(),
                        args: vec![Ident("b".into())]
                    }
                },
                Expression(CallFunction {
                    name: "print".into(),
                    args: vec![Ident("e".into())]
                })
            ]
        );
    }
}
