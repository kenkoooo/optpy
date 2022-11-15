mod types;
pub use types::{
    BinaryOperation, BinaryOperator, BoolOperation, BoolOperator, CallFunction, CallMethod,
    Compare, CompareOperator, Comprehension, Index, ListComprehension, Number, UnaryOperation,
    UnaryOperator,
};

use rustpython_parser::ast::ExprKind;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Expr {
    CallFunction(CallFunction<Expr>),
    CallMethod(CallMethod<Expr>),
    Tuple(Vec<Expr>),
    VariableName(String),
    BoolOperation(BoolOperation<Expr>),
    Compare(Compare<Expr>),
    UnaryOperation(UnaryOperation<Expr>),
    BinaryOperation(BinaryOperation<Expr>),
    Index(Index<Expr>),
    ConstantNumber(Number),
    ConstantString(String),
    ConstantBoolean(bool),
    List(Vec<Expr>),
    ListComprehension(ListComprehension<Expr>),
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
