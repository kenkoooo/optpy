use rustpython_parser::ast::ExpressionType;

#[derive(Debug, PartialEq, Eq)]
pub enum OptpyExpression {
    CallFunction {
        name: String,
        args: Vec<OptpyExpression>,
    },
    CallMethod {
        value: Box<OptpyExpression>,
        name: String,
        args: Vec<OptpyExpression>,
    },
    Tuple(Vec<OptpyExpression>),
    Ident(String),
}

impl OptpyExpression {
    pub fn parse(expr: &ExpressionType) -> Self {
        match expr {
            ExpressionType::Tuple { elements } => {
                let elements = parse_expressions(elements);
                OptpyExpression::Tuple(elements)
            }
            ExpressionType::Identifier { name } => OptpyExpression::Ident(name.into()),
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                assert!(keywords.is_empty());
                let args = parse_expressions(args);
                match &function.node {
                    ExpressionType::Attribute { value, name } => {
                        let value = OptpyExpression::parse(&value.node);
                        OptpyExpression::CallMethod {
                            value: Box::new(value),
                            name: name.into(),
                            args,
                        }
                    }
                    ExpressionType::Identifier { name } => OptpyExpression::CallFunction {
                        name: name.into(),
                        args,
                    },
                    function => todo!("{:#?}", function),
                }
            }
            expr => todo!("unsupported expression: {:?}", expr),
        }
    }
}

fn parse_expressions(expressions: &[rustpython_parser::ast::Expression]) -> Vec<OptpyExpression> {
    expressions
        .iter()
        .map(|e| OptpyExpression::parse(&e.node))
        .collect()
}
