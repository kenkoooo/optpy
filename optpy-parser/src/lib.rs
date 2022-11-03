mod expression;
pub use expression::{BinaryOperator, BoolOperator, CompareOperator, Expr, Number};

mod statement;
use rustpython_parser::error::ParseError;
pub use statement::Statement;

pub fn parse<S: AsRef<str>>(code: S) -> Result<Vec<Statement>, ParseError> {
    let ast = rustpython_parser::parser::parse_program(code.as_ref())?;
    let statements = ast
        .statements
        .iter()
        .flat_map(|s| Statement::parse(&s.node))
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
    fn test_tuple_assign() {
        let code = r"
a = b[0]";
        let statements = parse(code).unwrap();
    }

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
                        VariableName("a".into()),
                        VariableName("b".into()),
                        VariableName("c".into())
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
                    args: vec![VariableName("a".into())]
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
                        VariableName("a".into()),
                        VariableName("b".into()),
                        VariableName("c".into())
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
                                left: Box::new(VariableName("a".into())),
                                right: Box::new(VariableName("c".into())),
                                op: expression::CompareOperator::LessOrEqual
                            },
                            Compare {
                                left: Box::new(VariableName("c".into())),
                                right: Box::new(VariableName("b".into())),
                                op: expression::CompareOperator::Less
                            }
                        ]
                    },
                    body: vec![Assign {
                        target: VariableName("result".into()),
                        value: Number(expression::Number::Int("1".into()))
                    }],
                    orelse: vec![Assign {
                        target: VariableName("result".into()),
                        value: Number(expression::Number::Int("2".into()))
                    }]
                },
                Expression(CallFunction {
                    name: "print".into(),
                    args: vec![VariableName("result".into())]
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
                        VariableName("a".into()),
                        VariableName("b".into()),
                        VariableName("c".into())
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
                        left: Box::new(VariableName("a".into())),
                        right: Box::new(VariableName("d".into())),
                        op: BinaryOperator::Add
                    }))]
                },
                Assign {
                    target: VariableName("e".into()),
                    value: CallFunction {
                        name: "f".into(),
                        args: vec![VariableName("b".into())]
                    }
                },
                Expression(CallFunction {
                    name: "print".into(),
                    args: vec![VariableName("e".into())]
                })
            ]
        );
    }
}