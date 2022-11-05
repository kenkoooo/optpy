use rustpython_parser::ast::ExpressionType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    CallFunction {
        name: String,
        args: Vec<Expr>,
    },
    CallMethod {
        value: Box<Expr>,
        name: String,
        args: Vec<Expr>,
    },
    Tuple(Vec<Expr>),
    VariableName(String),
    BoolOperation {
        op: BoolOperator,
        conditions: Vec<Expr>,
    },
    Compare {
        left: Box<Expr>,
        right: Box<Expr>,
        op: CompareOperator,
    },
    BinaryOperation {
        left: Box<Expr>,
        right: Box<Expr>,
        op: BinaryOperator,
    },
    Index {
        value: Box<Expr>,
        index: Box<Expr>,
    },
    Number(Number),
    ConstantString(String),
    ConstantBoolean(bool),
}

impl Expr {
    pub fn parse(expr: &ExpressionType) -> Self {
        match expr {
            ExpressionType::Tuple { elements } => {
                let elements = parse_expressions(elements);
                Expr::Tuple(elements)
            }
            ExpressionType::Identifier { name } => Expr::VariableName(name.into()),
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                assert!(keywords.is_empty());
                let args = parse_expressions(args);
                match &function.node {
                    ExpressionType::Attribute { value, name } => {
                        let value = Expr::parse(&value.node);
                        Expr::CallMethod {
                            value: Box::new(value),
                            name: name.into(),
                            args,
                        }
                    }
                    ExpressionType::Identifier { name } => Expr::CallFunction {
                        name: name.into(),
                        args,
                    },
                    function => todo!("{:#?}", function),
                }
            }
            ExpressionType::BoolOp { op, values } => {
                let conditions = parse_expressions(values);
                let op = BoolOperator::parse(op);
                Expr::BoolOperation { op, conditions }
            }
            ExpressionType::Compare { vals, ops } => {
                let mut conditions = vec![];
                assert_eq!(vals.len(), ops.len() + 1);
                for (i, op) in ops.iter().enumerate() {
                    let left = Expr::parse(&vals[i].node);
                    let right = Expr::parse(&vals[i + 1].node);
                    conditions.push(Expr::Compare {
                        left: Box::new(left),
                        right: Box::new(right),
                        op: CompareOperator::parse(op),
                    })
                }
                Expr::BoolOperation {
                    op: BoolOperator::And,
                    conditions,
                }
            }
            ExpressionType::Number { value } => match value {
                rustpython_parser::ast::Number::Integer { value } => {
                    Expr::Number(Number::Int(value.to_string()))
                }
                rustpython_parser::ast::Number::Float { value } => {
                    Expr::Number(Number::Float(value.to_string()))
                }
                value => todo!("{:?}", value),
            },
            ExpressionType::Binop { a, op, b } => {
                let left = Expr::parse(&a.node);
                let right = Expr::parse(&b.node);
                Self::BinaryOperation {
                    left: Box::new(left),
                    right: Box::new(right),
                    op: BinaryOperator::parse(op),
                }
            }
            ExpressionType::Subscript { a, b } => {
                let a = Expr::parse(&a.node);
                let b = Expr::parse(&b.node);
                Self::Index {
                    value: Box::new(a),
                    index: Box::new(b),
                }
            }
            ExpressionType::String { value } => match value {
                rustpython_parser::ast::StringGroup::Constant { value } => {
                    Self::ConstantString(value.to_string())
                }
                value => todo!("{:?}", value),
            },
            ExpressionType::True => Self::ConstantBoolean(true),
            expr => todo!("unsupported expression: {:?}", expr),
        }
    }
}

fn parse_expressions(expressions: &[rustpython_parser::ast::Expression]) -> Vec<Expr> {
    expressions.iter().map(|e| Expr::parse(&e.node)).collect()
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
    Greater,
    Equal,
    NotEqual,
}

impl CompareOperator {
    pub fn parse(op: &rustpython_parser::ast::Comparison) -> Self {
        match op {
            rustpython_parser::ast::Comparison::LessOrEqual => Self::LessOrEqual,
            rustpython_parser::ast::Comparison::Less => Self::Less,
            rustpython_parser::ast::Comparison::Equal => Self::Equal,
            rustpython_parser::ast::Comparison::Greater => Self::Greater,
            rustpython_parser::ast::Comparison::NotEqual => Self::NotEqual,
            op => todo!("{:?}", op),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Number {
    Int(String),
    Float(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Mul,
    Mod,
    FloorDiv,
}

impl BinaryOperator {
    pub fn parse(op: &rustpython_parser::ast::Operator) -> Self {
        match op {
            rustpython_parser::ast::Operator::Add => Self::Add,
            rustpython_parser::ast::Operator::Mult => Self::Mul,
            rustpython_parser::ast::Operator::Mod => Self::Mod,
            rustpython_parser::ast::Operator::FloorDiv => Self::FloorDiv,
            op => todo!("{:?}", op),
        }
    }
}
