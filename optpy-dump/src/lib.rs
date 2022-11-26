use optpy_parser::{
    Assign, BinaryOperation, BoolOperation, CallFunction, CallMethod, Compare, Dict, Expr, Func,
    If, Index, Number, Statement, UnaryOperation, While,
};

pub trait DumpPython {
    fn to_python_code(&self) -> String;
}

impl DumpPython for Statement {
    fn to_python_code(&self) -> String {
        match self {
            Statement::Assign(Assign { target, value }) => {
                format!("{} = {}", target.to_python_code(), value.to_python_code())
            }
            Statement::Expression(expr) => expr.to_python_code(),
            Statement::If(If { test, body, orelse }) => {
                let test = test.to_python_code();
                let body = body.to_python_code();
                let orelse = orelse.to_python_code();
                format!(
                    "if {test}:\n{}\nelse:\n{}",
                    indent_code(&body),
                    indent_code(&orelse)
                )
            }
            Statement::Func(Func { name, args, body }) => {
                let args = args.join(", ");
                let body = body.to_python_code();
                format!("def {name}({args}):\n{}", indent_code(&body))
            }
            Statement::Return(r) => match r {
                Some(r) => format!("return {}", r.to_python_code()),
                None => "return".into(),
            },
            Statement::While(While { test, body }) => {
                let test = test.to_python_code();
                let body = body.to_python_code();
                format!("while {test}:\n{}", indent_code(&body))
            }
            Statement::Break => "break".into(),
            Statement::Continue => "continue".into(),
        }
    }
}

impl DumpPython for Expr {
    fn to_python_code(&self) -> String {
        match self {
            Expr::CallFunction(CallFunction { name, args }) => {
                let args = args
                    .iter()
                    .map(|arg| arg.to_python_code())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{name}({args})")
            }
            Expr::CallMethod(CallMethod { value, name, args }) => {
                let value = value.to_python_code();
                let args = args
                    .iter()
                    .map(|arg| arg.to_python_code())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{value}.{name}({args})")
            }
            Expr::Tuple(tuple) => {
                let tuple = tuple
                    .iter()
                    .map(|arg| arg.to_python_code())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({tuple})")
            }
            Expr::VariableName(name) => name.to_string(),
            Expr::BoolOperation(BoolOperation { op, conditions }) => {
                if conditions.len() == 1 {
                    conditions[0].to_python_code()
                } else {
                    conditions
                        .iter()
                        .map(|c| format!("({})", c.to_python_code()))
                        .collect::<Vec<_>>()
                        .join(match op {
                            optpy_parser::BoolOperator::And => " and ",
                            optpy_parser::BoolOperator::Or => " or ",
                        })
                }
            }
            Expr::Compare(Compare { left, right, op }) => {
                let left = left.to_python_code();
                let right = right.to_python_code();
                format!(
                    "({left} {} {right})",
                    match op {
                        optpy_parser::CompareOperator::Less => "<",
                        optpy_parser::CompareOperator::LessOrEqual => "<=",
                        optpy_parser::CompareOperator::Greater => ">",
                        optpy_parser::CompareOperator::GreaterOrEqual => ">=",
                        optpy_parser::CompareOperator::Equal => "==",
                        optpy_parser::CompareOperator::NotEqual => "!=",
                        optpy_parser::CompareOperator::NotIn => "not in",
                        optpy_parser::CompareOperator::In => "in",
                    }
                )
            }
            Expr::UnaryOperation(UnaryOperation { value, op }) => {
                format!(
                    "({}{})",
                    match op {
                        optpy_parser::UnaryOperator::Add => "+",
                        optpy_parser::UnaryOperator::Sub => "-",
                        optpy_parser::UnaryOperator::Not => "not ",
                    },
                    value.to_python_code()
                )
            }
            Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
                let left = left.to_python_code();
                let right = right.to_python_code();
                format!(
                    "({left} {} {right})",
                    match op {
                        optpy_parser::BinaryOperator::Add => "+",
                        optpy_parser::BinaryOperator::Sub => "-",
                        optpy_parser::BinaryOperator::Mul => "*",
                        optpy_parser::BinaryOperator::Div => "/",
                        optpy_parser::BinaryOperator::Mod => "%",
                        optpy_parser::BinaryOperator::FloorDiv => "//",
                        optpy_parser::BinaryOperator::Pow => "**",
                        optpy_parser::BinaryOperator::BitAnd => "&",
                    }
                )
            }
            Expr::Index(Index { value, index }) => {
                format!("{}[{}]", value.to_python_code(), index.to_python_code())
            }
            Expr::ConstantNumber(n) => match n {
                Number::Int(n) => n.to_string(),
                Number::Float(n) => n.to_string(),
            },
            Expr::ConstantString(s) => format!(r#""{}""#, s),
            Expr::ConstantBoolean(b) => {
                if *b {
                    "True".into()
                } else {
                    "False".into()
                }
            }
            Expr::None => "None".into(),
            Expr::List(list) => {
                let list = list.iter().map(|e| e.to_python_code()).collect::<Vec<_>>();
                format!("[{}]", list.join(", "))
            }
            Expr::Dict(Dict { pairs }) => {
                let pairs = pairs
                    .iter()
                    .map(|(k, v)| format!("{}:{}", k.to_python_code(), v.to_python_code()))
                    .collect::<Vec<_>>();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
}

fn indent_code(code: &str) -> String {
    let mut result = String::new();
    for line in code.split('\n') {
        result += "    ";
        result += line;
        result += "\n";
    }
    result
}

impl DumpPython for Vec<Statement> {
    fn to_python_code(&self) -> String {
        self.iter()
            .map(|s| s.to_python_code())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
