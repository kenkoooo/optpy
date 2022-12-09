use rustpython_parser::ast::{Boolop, Cmpop, Unaryop};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallFunction<E> {
    pub name: String,
    pub args: Vec<E>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallMethod<E> {
    pub value: Box<E>,
    pub name: String,
    pub args: Vec<E>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoolOperation<E> {
    pub op: BoolOperator,
    pub conditions: Vec<E>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Compare<E> {
    pub left: Box<E>,
    pub right: Box<E>,
    pub op: CompareOperator,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryOperation<E> {
    pub value: Box<E>,
    pub op: UnaryOperator,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryOperation<E> {
    pub left: Box<E>,
    pub right: Box<E>,
    pub op: BinaryOperator,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Index<E> {
    pub value: Box<E>,
    pub index: Box<E>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dict<E> {
    pub pairs: Vec<(E, E)>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct ListComprehension<E> {
    pub(crate) value: Box<E>,
    pub(crate) generators: Vec<Comprehension<E>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompareOperator {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    NotIn,
    In,
}

impl CompareOperator {
    pub fn parse(op: &Cmpop) -> Self {
        match op {
            Cmpop::Lt => Self::Less,
            Cmpop::LtE => Self::LessOrEqual,
            Cmpop::Gt => Self::Greater,
            Cmpop::GtE => Self::GreaterOrEqual,
            Cmpop::Eq => Self::Equal,
            Cmpop::NotEq => Self::NotEqual,
            Cmpop::NotIn => Self::NotIn,
            Cmpop::In => Self::In,
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
    Sub,
    Mul,
    Div,
    Mod,
    FloorDiv,
    Pow,
    BitAnd,
    LeftShift,
    RightShift,
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
            rustpython_parser::ast::Operator::Pow => Self::Pow,
            rustpython_parser::ast::Operator::BitAnd => Self::BitAnd,
            rustpython_parser::ast::Operator::LShift => Self::LeftShift,
            rustpython_parser::ast::Operator::RShift => Self::RightShift,
            op => todo!("{:?}", op),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comprehension<E> {
    pub target: Box<E>,
    pub iter: Box<E>,
    pub ifs: Vec<E>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOperator {
    Add,
    Sub,
    Not,
}

impl UnaryOperator {
    pub fn parse(op: &Unaryop) -> Self {
        match op {
            Unaryop::UAdd => Self::Add,
            Unaryop::USub => Self::Sub,
            Unaryop::Not => Self::Not,
            op => todo!("{:?}", op),
        }
    }
}
