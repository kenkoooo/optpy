use rustpython_parser::ast::{Boolop, Cmpop, ExprKind};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    ConstantNumber(Number),
    ConstantString(String),
    ConstantBoolean(bool),
    List(Vec<Expr>),
    ListComprehension {
        value: Box<Expr>,
        generators: Vec<Comprehension>,
    },
}

impl Expr {
    pub fn parse(expr: &ExprKind) -> Self {
        match expr {
            ExprKind::Tuple { elts, ctx: _ } => {
                let elements = parse_expressions(elts);
                Expr::Tuple(elements)
            }
            ExprKind::Name { id, ctx: _ } => Expr::VariableName(id.into()),
            ExprKind::Call {
                args,
                keywords,
                func,
            } => {
                assert!(keywords.is_empty());
                let args = parse_expressions(args);
                match &func.node {
                    ExprKind::Attribute {
                        value,
                        attr,
                        ctx: _,
                    } => {
                        let value = Expr::parse(&value.node);
                        Expr::CallMethod {
                            value: Box::new(value),
                            name: attr.into(),
                            args,
                        }
                    }
                    ExprKind::Name { id, ctx: _ } => Expr::CallFunction {
                        name: id.into(),
                        args,
                    },
                    function => todo!("{:#?}", function),
                }
            }
            ExprKind::BoolOp { op, values } => {
                let conditions = parse_expressions(values);
                let op = BoolOperator::parse(op);
                Expr::BoolOperation { op, conditions }
            }
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => {
                let mut conditions = vec![];
                let mut left = Expr::parse(&left.node);
                for (op, right) in ops.iter().zip(comparators) {
                    let op = CompareOperator::parse(op);
                    let right = Expr::parse(&right.node);
                    conditions.push(Expr::Compare {
                        left: Box::new(left),
                        right: Box::new(right.clone()),
                        op,
                    });
                    left = right;
                }
                Expr::BoolOperation {
                    op: BoolOperator::And,
                    conditions,
                }
            }
            ExprKind::BinOp { op, left, right } => {
                let left = Expr::parse(&left.node);
                let right = Expr::parse(&right.node);
                Self::BinaryOperation {
                    left: Box::new(left),
                    right: Box::new(right),
                    op: BinaryOperator::parse(op),
                }
            }
            ExprKind::Subscript {
                value,
                slice,
                ctx: _,
            } => {
                let value = Expr::parse(&value.node);
                let index = Expr::parse(&slice.node);
                Self::Index {
                    value: Box::new(value),
                    index: Box::new(index),
                }
            }
            ExprKind::Constant { value, kind: _ } => match value {
                rustpython_parser::ast::Constant::Bool(b) => Expr::ConstantBoolean(*b),
                rustpython_parser::ast::Constant::Str(s) => Expr::ConstantString(s.clone()),
                rustpython_parser::ast::Constant::Int(i) => {
                    Expr::ConstantNumber(Number::Int(i.to_string()))
                }
                rustpython_parser::ast::Constant::Float(f) => {
                    Expr::ConstantNumber(Number::Float(f.to_string()))
                }
                value => todo!("{:?}", value),
            },
            ExprKind::List { elts, ctx: _ } => {
                let list = parse_expressions(elts);
                Self::List(list)
            }
            ExprKind::ListComp { elt, generators } => {
                let value = Expr::parse(&elt.node);
                let generators = generators
                    .iter()
                    .map(
                        |rustpython_parser::ast::Comprehension {
                             target, iter, ifs, ..
                         }| {
                            let target = Expr::parse(&target.node);
                            let iter = Expr::parse(&iter.node);
                            let ifs = parse_expressions(ifs);
                            Comprehension {
                                target: Box::new(target),
                                iter: Box::new(iter),
                                ifs,
                            }
                        },
                    )
                    .collect();
                Self::ListComprehension {
                    value: Box::new(value),
                    generators,
                }
            }
            expr => todo!("unsupported expression: {:?}", expr),
        }
    }
}

fn parse_expressions(expressions: &[rustpython_parser::ast::Expr]) -> Vec<Expr> {
    expressions.iter().map(|e| Expr::parse(&e.node)).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BoolOperator {
    And,
    Or,
}
impl BoolOperator {
    pub fn parse(op: &Boolop) -> Self {
        match op {
            Boolop::And => Self::And,
            Boolop::Or => Self::Or,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CompareOperator {
    Less,
    LessOrEqual,
    Greater,
    Equal,
    NotEqual,
}

impl CompareOperator {
    pub fn parse(op: &Cmpop) -> Self {
        match op {
            Cmpop::LtE => Self::LessOrEqual,
            Cmpop::Lt => Self::Less,
            Cmpop::Eq => Self::Equal,
            Cmpop::Gt => Self::Greater,
            Cmpop::NotEq => Self::NotEqual,
            op => todo!("{:?}", op),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Number {
    Int(String),
    Float(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    FloorDiv,
}

impl BinaryOperator {
    pub fn parse(op: &rustpython_parser::ast::Operator) -> Self {
        match op {
            rustpython_parser::ast::Operator::Add => Self::Add,
            rustpython_parser::ast::Operator::Sub => Self::Sub,
            rustpython_parser::ast::Operator::Mult => Self::Mul,
            rustpython_parser::ast::Operator::Div => Self::Div,
            rustpython_parser::ast::Operator::Mod => Self::Mod,
            rustpython_parser::ast::Operator::FloorDiv => Self::FloorDiv,
            op => todo!("{:?}", op),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Comprehension {
    pub target: Box<Expr>,
    pub iter: Box<Expr>,
    pub ifs: Vec<Expr>,
}
