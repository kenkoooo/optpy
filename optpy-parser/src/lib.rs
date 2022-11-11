mod expression;
pub use expression::{BinaryOperator, BoolOperator, CompareOperator, Expr, Number, UnaryOperator};

mod statement;
use rustpython_parser::error::ParseError;
pub use statement::Statement;

mod simplify;

pub fn parse<S: AsRef<str>>(code: S) -> Result<Vec<Statement>, ParseError> {
    let ast = rustpython_parser::parser::parse_program(code.as_ref(), "<embedded>")?;
    let statements = ast.iter().map(|s| Statement::parse(&s.node)).collect();
    let statements = simplify::simplify_list_comprehensions(statements);
    let statements = simplify::simplify_for_loops(statements);
    let statements = simplify::simplify_tuple_assignments(statements);
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
                    target: VariableName("__tmp_for_tuple".into()),
                    value: CallMethod {
                        value: Box::new(CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                Assign {
                    target: VariableName("a".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("0".into())))
                    }
                },
                Assign {
                    target: VariableName("b".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("1".into())))
                    }
                },
                Assign {
                    target: VariableName("c".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("2".into())))
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
                    target: VariableName("__tmp_for_tuple".into()),
                    value: CallMethod {
                        value: Box::new(CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                Assign {
                    target: VariableName("a".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("0".into())))
                    }
                },
                Assign {
                    target: VariableName("b".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("1".into())))
                    }
                },
                Assign {
                    target: VariableName("c".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("2".into())))
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
                        value: ConstantNumber(expression::Number::Int("1".into()))
                    }],
                    orelse: vec![Assign {
                        target: VariableName("result".into()),
                        value: ConstantNumber(expression::Number::Int("2".into()))
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
                    target: VariableName("__tmp_for_tuple".into()),
                    value: CallMethod {
                        value: Box::new(CallFunction {
                            name: "input".into(),
                            args: vec![]
                        }),
                        name: "split".into(),
                        args: vec![]
                    }
                },
                Assign {
                    target: VariableName("a".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("0".into())))
                    }
                },
                Assign {
                    target: VariableName("b".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("1".into())))
                    }
                },
                Assign {
                    target: VariableName("c".into()),
                    value: Index {
                        value: Box::new(VariableName("__tmp_for_tuple".into())),
                        index: Box::new(ConstantNumber(crate::Number::Int("2".into())))
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

    #[test]
    fn test_for_loop() {
        let code = r"
for i in range(N):
    print(i)
";
        let expected = r"
__tmp_for_loop_iter_14800386153579835208 = list(range(N))
__tmp_for_loop_iter_14800386153579835208.reverse()
while len(__tmp_for_loop_iter_14800386153579835208) > 0:
    i = __tmp_for_loop_iter_14800386153579835208.pop()
    print(i)
";
        assert_eq!(parse(code).unwrap(), parse(expected).unwrap());
    }

    #[test]
    fn test_list_comprehension() {
        let code = r"a = [[i*j for j in range(M)] for i in range(N)]";

        let expected = r"
def __f2093947204604468815():
    __result = []
    __tmp_for_loop_iter_8972402058792239477 = list(range(N))
    __tmp_for_loop_iter_8972402058792239477.reverse()
    while len(__tmp_for_loop_iter_8972402058792239477) > 0:
        i = __tmp_for_loop_iter_8972402058792239477.pop()
        def __f9396250878134232951():
            __result = []
            __tmp_for_loop_iter_13945119744480575688 = list(range(M))
            __tmp_for_loop_iter_13945119744480575688.reverse()
            while len(__tmp_for_loop_iter_13945119744480575688) > 0:
                j = __tmp_for_loop_iter_13945119744480575688.pop()
                __result.append(i * j)
            return __result
        __result.append(__f9396250878134232951())
    return __result
a = __f2093947204604468815()
";
        assert_eq!(parse(code).unwrap(), parse(expected).unwrap());
    }
}
