use optpy_parser::{
    Assign, BinaryOperation, BoolOperation, CallFunction, CallMethod, Compare, Dict, Expr, Func,
    If, Index, Statement, UnaryOperation, While,
};

pub fn resolve_builtin_functions(statements: &[Statement]) -> Vec<Statement> {
    statements.resolve()
}

trait StatementResolve {
    fn resolve(&self) -> Statement;
}

impl StatementResolve for Statement {
    fn resolve(&self) -> Statement {
        match self {
            Statement::Assign(Assign { target, value }) => Statement::Assign(Assign {
                target: target.resolve(),
                value: value.resolve(),
            }),
            Statement::Expression(e) => Statement::Expression(e.resolve()),
            Statement::If(If { test, body, orelse }) => Statement::If(If {
                test: test.resolve(),
                body: body.resolve(),
                orelse: orelse.resolve(),
            }),
            Statement::Func(Func { name, args, body }) => Statement::Func(Func {
                name: name.clone(),
                args: args.clone(),
                body: body.resolve(),
            }),
            Statement::Return(v) => Statement::Return(v.as_ref().map(|e| e.resolve())),
            Statement::While(While { test, body }) => Statement::While(While {
                test: test.resolve(),
                body: body.resolve(),
            }),
            Statement::Break => Statement::Break,
            Statement::Continue => Statement::Continue,
        }
    }
}

trait StatementResolves {
    fn resolve(&self) -> Vec<Statement>;
}
impl StatementResolves for [Statement] {
    fn resolve(&self) -> Vec<Statement> {
        self.iter().map(|s| s.resolve()).collect()
    }
}

trait ExprResolve {
    fn resolve(&self) -> Expr;
}

impl ExprResolve for Expr {
    fn resolve(&self) -> Self {
        match self {
            Expr::CallFunction(CallFunction { name, args }) => {
                if name == "map" && args[0] == Expr::VariableName("int".into()) {
                    let args = args[1..].resolve();
                    Expr::CallFunction(CallFunction {
                        name: "map_int".into(),
                        args,
                    })
                } else {
                    match name.as_str() {
                        "range" | "print" | "pow" | "set" | "exit" | "max" | "min" | "sum" => {
                            Expr::CallFunction(CallFunction {
                                name: format!("{name}__macro__"),
                                args: args.resolve(),
                            })
                        }
                        _ => Expr::CallFunction(CallFunction {
                            name: name.to_string(),
                            args: args.resolve(),
                        }),
                    }
                }
            }
            Expr::CallMethod(CallMethod { value, name, args }) => Expr::CallMethod(CallMethod {
                value: Box::new(value.resolve()),
                name: name.to_string(),
                args: args.resolve(),
            }),
            Expr::Tuple(tuple) => Expr::Tuple(tuple.resolve()),
            Expr::BoolOperation(BoolOperation { op, conditions }) => {
                Expr::BoolOperation(BoolOperation {
                    op: *op,
                    conditions: conditions.resolve(),
                })
            }
            Expr::Compare(Compare { left, right, op }) => Expr::Compare(Compare {
                left: Box::new(left.resolve()),
                right: Box::new(right.resolve()),
                op: *op,
            }),
            Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
                Expr::BinaryOperation(BinaryOperation {
                    left: Box::new(left.resolve()),
                    right: Box::new(right.resolve()),
                    op: *op,
                })
            }
            Expr::Index(Index { value, index }) => Expr::Index(Index {
                value: Box::new(value.resolve()),
                index: Box::new(index.resolve()),
            }),
            Expr::List(list) => Expr::List(list.resolve()),
            Expr::Dict(Dict { pairs }) => {
                let pairs = pairs
                    .iter()
                    .map(|(key, value)| (key.resolve(), value.resolve()))
                    .collect();
                Expr::Dict(Dict { pairs })
            }
            Expr::ConstantNumber(_)
            | Expr::ConstantString(_)
            | Expr::VariableName(_)
            | Expr::ConstantBoolean(_)
            | Expr::None => self.clone(),
            Expr::UnaryOperation(UnaryOperation { value, op }) => {
                Expr::UnaryOperation(UnaryOperation {
                    value: Box::new(value.resolve()),
                    op: *op,
                })
            }
        }
    }
}

trait ExprResolves {
    fn resolve(&self) -> Vec<Expr>;
}
impl ExprResolves for [Expr] {
    fn resolve(&self) -> Vec<Expr> {
        self.iter().map(|e| e.resolve()).collect()
    }
}
