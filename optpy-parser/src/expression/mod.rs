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
}

#[derive(Clone)]
pub(crate) enum RawExpr {
    CallFunction(CallFunction<RawExpr>),
    CallMethod(CallMethod<RawExpr>),
    Tuple(Vec<RawExpr>),
    VariableName(String),
    BoolOperation(BoolOperation<RawExpr>),
    Compare(Compare<RawExpr>),
    UnaryOperation(UnaryOperation<RawExpr>),
    BinaryOperation(BinaryOperation<RawExpr>),
    Index(Index<RawExpr>),
    ConstantNumber(Number),
    ConstantString(String),
    ConstantBoolean(bool),
    List(Vec<RawExpr>),
    ListComprehension(ListComprehension<RawExpr>),
}

impl RawExpr {
    pub fn parse(expr: &ExprKind) -> Self {
        match expr {
            ExprKind::Tuple { elts, ctx: _ } => {
                let elements = parse_expressions(elts);
                RawExpr::Tuple(elements)
            }
            ExprKind::Name { id, ctx: _ } => RawExpr::VariableName(id.into()),
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
                        let value = RawExpr::parse(&value.node);
                        RawExpr::CallMethod(CallMethod {
                            value: Box::new(value),
                            name: attr.into(),
                            args,
                        })
                    }
                    ExprKind::Name { id, ctx: _ } => RawExpr::CallFunction(CallFunction {
                        name: id.into(),
                        args,
                    }),
                    function => todo!("{:#?}", function),
                }
            }
            ExprKind::BoolOp { op, values } => {
                let conditions = parse_expressions(values);
                let op = BoolOperator::parse(op);
                RawExpr::BoolOperation(BoolOperation { op, conditions })
            }
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => {
                let mut conditions = vec![];
                let mut left = RawExpr::parse(&left.node);
                for (op, right) in ops.iter().zip(comparators) {
                    let op = CompareOperator::parse(op);
                    let right = RawExpr::parse(&right.node);
                    conditions.push(RawExpr::Compare(Compare {
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
                    RawExpr::BoolOperation(BoolOperation {
                        op: BoolOperator::And,
                        conditions,
                    })
                }
            }
            ExprKind::BinOp { op, left, right } => {
                let left = RawExpr::parse(&left.node);
                let right = RawExpr::parse(&right.node);
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
                let value = RawExpr::parse(&value.node);
                let index = RawExpr::parse(&slice.node);
                Self::Index(Index {
                    value: Box::new(value),
                    index: Box::new(index),
                })
            }
            ExprKind::Constant { value, kind: _ } => match value {
                rustpython_parser::ast::Constant::Bool(b) => RawExpr::ConstantBoolean(*b),
                rustpython_parser::ast::Constant::Str(s) => RawExpr::ConstantString(s.clone()),
                rustpython_parser::ast::Constant::Int(i) => {
                    RawExpr::ConstantNumber(Number::Int(i.to_string()))
                }
                rustpython_parser::ast::Constant::Float(f) => {
                    RawExpr::ConstantNumber(Number::Float(f.to_string()))
                }
                value => todo!("{:?}", value),
            },
            ExprKind::List { elts, ctx: _ } => {
                let list = parse_expressions(elts);
                Self::List(list)
            }
            ExprKind::ListComp { elt, generators } => {
                let value = RawExpr::parse(&elt.node);
                let generators = generators
                    .iter()
                    .map(
                        |rustpython_parser::ast::Comprehension {
                             target, iter, ifs, ..
                         }| {
                            let target = RawExpr::parse(&target.node);
                            let iter = RawExpr::parse(&iter.node);
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
                let value = RawExpr::parse(&operand.node);
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

fn parse_expressions(expressions: &[rustpython_parser::ast::Expr]) -> Vec<RawExpr> {
    expressions
        .iter()
        .map(|e| RawExpr::parse(&e.node))
        .collect()
}
