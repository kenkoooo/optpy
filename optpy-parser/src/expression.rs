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
    BoolOperation {
        op: BoolOperator,
        conditions: Vec<OptpyExpression>,
    },
    Compare {
        left: Box<OptpyExpression>,
        right: Box<OptpyExpression>,
        op: CompareOperator,
    },
    Number(Number),
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
            ExpressionType::BoolOp { op, values } => {
                let conditions = parse_expressions(values);
                let op = BoolOperator::parse(op);
                OptpyExpression::BoolOperation { op, conditions }
            }
            ExpressionType::Compare { vals, ops } => {
                let mut conditions = vec![];
                assert_eq!(vals.len(), ops.len() + 1);
                for (i, op) in ops.iter().enumerate() {
                    let left = OptpyExpression::parse(&vals[i].node);
                    let right = OptpyExpression::parse(&vals[i + 1].node);
                    conditions.push(OptpyExpression::Compare {
                        left: Box::new(left),
                        right: Box::new(right),
                        op: CompareOperator::parse(op),
                    })
                }
                OptpyExpression::BoolOperation {
                    op: BoolOperator::And,
                    conditions,
                }
            }
            ExpressionType::Number { value } => match value {
                rustpython_parser::ast::Number::Integer { value } => {
                    OptpyExpression::Number(Number::Int(value.to_string()))
                }
                rustpython_parser::ast::Number::Float { value } => {
                    OptpyExpression::Number(Number::Float(value.to_string()))
                }
                value => todo!("{:?}", value),
            },
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BoolOperator {
    And,
    Or,
}
impl BoolOperator {
    pub fn parse(op: &rustpython_parser::ast::BooleanOperator) -> Self {
        match op {
            rustpython_parser::ast::BooleanOperator::And => Self::And,
            rustpython_parser::ast::BooleanOperator::Or => Self::Or,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompareOperator {
    Less,
    LessOrEqual,
}

impl CompareOperator {
    pub fn parse(op: &rustpython_parser::ast::Comparison) -> Self {
        match op {
            rustpython_parser::ast::Comparison::LessOrEqual => Self::LessOrEqual,
            rustpython_parser::ast::Comparison::Less => Self::Less,
            op => todo!("{:?}", op),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Number {
    Int(String),
    Float(String),
}
