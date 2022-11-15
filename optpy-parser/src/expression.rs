use rustpython_parser::ast::{Boolop, Cmpop, ExprKind, Unaryop};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Expr {
    CallFunction(CallFunction),
    CallMethod(CallMethod),
    Tuple(Vec<Expr>),
    VariableName(String),
    BoolOperation(BoolOperation),
    Compare(Compare),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Index(Index),
    ConstantNumber(Number),
    ConstantString(String),
    ConstantBoolean(bool),
    List(Vec<Expr>),
    ListComprehension(ListComprehension),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CallFunction {
    pub name: String,
    pub args: Vec<Expr>,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CallMethod {
    pub value: Box<Expr>,
    pub name: String,
    pub args: Vec<Expr>,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BoolOperation {
    pub op: BoolOperator,
    pub conditions: Vec<Expr>,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Compare {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: CompareOperator,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct UnaryOperation {
    pub value: Box<Expr>,
    pub op: UnaryOperator,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BinaryOperation {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinaryOperator,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Index {
    pub value: Box<Expr>,
    pub index: Box<Expr>,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ListComprehension {
    pub value: Box<Expr>,
    pub generators: Vec<Comprehension>,
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
                        Expr::CallMethod(CallMethod {
                            value: Box::new(value),
                            name: attr.into(),
                            args,
                        })
                    }
                    ExprKind::Name { id, ctx: _ } => Expr::CallFunction(CallFunction {
                        name: id.into(),
                        args,
                    }),
                    function => todo!("{:#?}", function),
                }
            }
            ExprKind::BoolOp { op, values } => {
                let conditions = parse_expressions(values);
                let op = BoolOperator::parse(op);
                Expr::BoolOperation(BoolOperation { op, conditions })
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
                    conditions.push(Expr::Compare(Compare {
                        left: Box::new(left),
                        right: Box::new(right.clone()),
                        op,
                    }));
                    left = right;
                }
                assert!(conditions.len() > 0);
                if conditions.len() == 1 {
                    let condition = conditions.pop().expect("no condition");
                    condition
                } else {
                    Expr::BoolOperation(BoolOperation {
                        op: BoolOperator::And,
                        conditions,
                    })
                }
            }
            ExprKind::BinOp { op, left, right } => {
                let left = Expr::parse(&left.node);
                let right = Expr::parse(&right.node);
                Self::BinaryOperation(BinaryOperation {
                    left: Box::new(left),
                    right: Box::new(right),
                    op: BinaryOperator::parse(op),
                })
            }
            ExprKind::Subscript {
                value,
                slice,
                ctx: _,
            } => {
                let value = Expr::parse(&value.node);
                let index = Expr::parse(&slice.node);
                Self::Index(Index {
                    value: Box::new(value),
                    index: Box::new(index),
                })
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
                Self::ListComprehension(ListComprehension {
                    value: Box::new(value),
                    generators,
                })
            }
            ExprKind::UnaryOp { op, operand } => {
                let value = Expr::parse(&operand.node);
                let op = UnaryOperator::parse(op);
                Self::UnaryOperation(UnaryOperation {
                    value: Box::new(value),
                    op,
                })
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
    GreaterOrEqual,
    Equal,
    NotEqual,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum UnaryOperator {
    Add,
    Sub,
}

impl UnaryOperator {
    pub fn parse(op: &Unaryop) -> Self {
        match op {
            Unaryop::UAdd => Self::Add,
            Unaryop::USub => Self::Sub,
            op => todo!("{:?}", op),
        }
    }
}
