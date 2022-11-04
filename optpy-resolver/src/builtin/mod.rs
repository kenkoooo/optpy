use optpy_parser::{Expr, Statement};

pub fn resolve_builtin_functions(statements: &[Statement]) -> Vec<Statement> {
    statements.resolve()
}

trait StatementResolve {
    fn resolve(&self) -> Statement;
}

impl StatementResolve for Statement {
    fn resolve(&self) -> Statement {
        match self {
            Statement::Assign { target, value } => Statement::Assign {
                target: target.resolve(),
                value: value.resolve(),
            },
            Statement::Expression(e) => Statement::Expression(e.resolve()),
            Statement::If { test, body, orelse } => Statement::If {
                test: test.resolve(),
                body: body.resolve(),
                orelse: orelse.resolve(),
            },
            Statement::Func { name, args, body } => Statement::Func {
                name: name.clone(),
                args: args.clone(),
                body: body.resolve(),
            },
            Statement::Return(v) => Statement::Return(v.as_ref().map(|e| e.resolve())),
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
            Expr::CallFunction { name, args } => {
                if name == "map" && args[0] == Expr::VariableName("int".into()) {
                    let args = args[1..].resolve();
                    Expr::CallFunction {
                        name: "map_int".into(),
                        args,
                    }
                } else {
                    Expr::CallFunction {
                        name: name.to_string(),
                        args: args.resolve(),
                    }
                }
            }
            Expr::CallMethod { value, name, args } => Expr::CallMethod {
                value: Box::new(value.resolve()),
                name: name.to_string(),
                args: args.resolve(),
            },
            Expr::Tuple(tuple) => Expr::Tuple(tuple.resolve()),
            Expr::BoolOperation { op, conditions } => Expr::BoolOperation {
                op: *op,
                conditions: conditions.resolve(),
            },
            Expr::Compare { left, right, op } => Expr::Compare {
                left: Box::new(left.resolve()),
                right: Box::new(right.resolve()),
                op: *op,
            },
            Expr::BinaryOperation { left, right, op } => Expr::BinaryOperation {
                left: Box::new(left.resolve()),
                right: Box::new(right.resolve()),
                op: *op,
            },
            Expr::Index { value, index } => Expr::Index {
                value: Box::new(value.resolve()),
                index: Box::new(index.resolve()),
            },
            Expr::Number(_) | Expr::ConstantString(_) | Expr::VariableName(_) => self.clone(),
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
