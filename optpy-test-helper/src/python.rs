use optpy_parser::{BinaryOperator, BoolOperator, CompareOperator, Expr, Statement};

pub fn to_python_code(statements: &[Statement]) -> Vec<String> {
    let mut result = vec![];
    for statement in statements {
        match statement {
            Statement::Assign { target, value } => {
                result.push(format!("{} = {}", format_expr(target), format_expr(value)));
            }
            Statement::Expression(expr) => result.push(format_expr(expr)),
            Statement::If { test, body, orelse } => {
                result.push(format!("if {}:", format_expr(test)));
                let body = to_python_code(body);
                for line in body {
                    result.push(format!("    {line}"));
                }
                if !orelse.is_empty() {
                    let orelse = to_python_code(orelse);
                    result.push("else:".to_string());
                    for line in orelse {
                        result.push(format!("    {line}"));
                    }
                }
            }
            Statement::Func { name, args, body } => {
                result.push(format!("def {name}({}):", args.join(", ")));
                let body = to_python_code(body);
                for line in body {
                    result.push(format!("    {line}"));
                }
            }
            Statement::Return(value) => match value {
                Some(expr) => result.push(format!("return {}", format_expr(expr))),
                None => result.push("return".to_string()),
            },
            Statement::While { test, body } => {
                result.push(format!("while {}:", format_expr(test)));
                let body = to_python_code(body);
                for line in body {
                    result.push(format!("    {line}"));
                }
            }
            Statement::Break => result.push("break".into()),
        }
    }
    result
}

fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::CallFunction { name, args } => {
            let args = args
                .iter()
                .map(|arg| format_expr(arg))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{name}({args})")
        }
        Expr::CallMethod { value, name, args } => {
            let value = format_expr(value);
            let args = args
                .iter()
                .map(|arg| format_expr(arg))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{value}.{name}({args})")
        }
        Expr::Tuple(tuple) => tuple
            .iter()
            .map(|arg| format_expr(arg))
            .collect::<Vec<_>>()
            .join(", "),
        Expr::VariableName(name) => name.clone(),
        Expr::BoolOperation { op, conditions } => {
            let op = format_bool_operator(op);
            conditions
                .iter()
                .map(|e| format!("({})", format_expr(e)))
                .collect::<Vec<_>>()
                .join(&op)
        }
        Expr::Compare { left, right, op } => {
            format!(
                "{}{}{}",
                format_expr(left),
                format_compare_operator(op),
                format_expr(right)
            )
        }
        Expr::BinaryOperation { left, right, op } => format!(
            "{}{}{}",
            format_expr(left),
            format_binary_operator(op),
            format_expr(right)
        ),
        Expr::Number(number) => match number {
            optpy_parser::Number::Int(int) => int.to_string(),
            optpy_parser::Number::Float(float) => float.to_string(),
        },
        Expr::Index { value, index } => {
            format!("{}[{}]", format_expr(value), format_expr(index))
        }
        Expr::ConstantString(value) => format!(r#""{value}""#),
        Expr::ConstantBoolean(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
    }
}

fn format_bool_operator(op: &BoolOperator) -> String {
    match op {
        BoolOperator::And => " and ".to_string(),
        BoolOperator::Or => " or ".to_string(),
    }
}
fn format_compare_operator(op: &CompareOperator) -> String {
    match op {
        CompareOperator::Less => " < ".to_string(),
        CompareOperator::LessOrEqual => " <= ".to_string(),
        CompareOperator::Equal => " == ".to_string(),
        CompareOperator::Greater => " > ".to_string(),
        CompareOperator::NotEqual => " != ".to_string(),
    }
}
fn format_binary_operator(op: &BinaryOperator) -> String {
    match op {
        BinaryOperator::Add => " + ".to_string(),
        BinaryOperator::Sub => " - ".to_string(),
        BinaryOperator::Mul => " * ".to_string(),
        BinaryOperator::Div => " / ".to_string(),
        BinaryOperator::Mod => " % ".to_string(),
        BinaryOperator::FloorDiv => " // ".to_string(),
    }
}
