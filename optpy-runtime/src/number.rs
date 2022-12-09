use std::{
    hash::Hash,
    ops::{Add, Div, Mul, Rem, Sub},
};

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int64(i64),
    Float(f64),
}
impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Number::Int64(i) => i.hash(state),
            Number::Float(_) => todo!(),
        }
    }
}
impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Number::Int64(l0), Number::Int64(r0)) => l0.partial_cmp(r0),
            (Number::Float(l0), Number::Float(r0)) => l0.partial_cmp(r0),
            (Number::Int64(l0), Number::Float(r0)) => (*l0 as f64).partial_cmp(r0),
            (Number::Float(l0), Number::Int64(r0)) => l0.partial_cmp(&(*r0 as f64)),
        }
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Int64(l0), Number::Int64(r0)) => l0.eq(r0),
            (Number::Float(l0), Number::Float(r0)) => l0.eq(r0),
            (Number::Int64(l0), Number::Float(r0)) => *l0 as f64 == *r0,
            (Number::Float(l0), Number::Int64(r0)) => *l0 == *r0 as f64,
        }
    }
}

impl Number {
    pub fn floor_div(&self, rhs: &Number) -> Number {
        match (self, rhs) {
            (Number::Int64(l0), Number::Int64(r0)) => Number::Int64(l0 / r0),
            _ => todo!(),
        }
    }
    pub fn pow(&self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::Int64(l0), Number::Int64(r0)) => Number::Int64(l0.pow(r0 as u32)),
            _ => todo!(),
        }
    }
    pub fn abs(&self) -> Number {
        match self {
            Number::Int64(i) => Number::Int64(i.abs()),
            Number::Float(f) => Number::Float(f.abs()),
        }
    }
    pub fn test(&self) -> bool {
        match self {
            Number::Int64(i) => *i != 0,
            Number::Float(f) => *f != 0.0,
        }
    }
    pub fn __left_shift(&self, value: &Number) -> Number {
        match (self, value) {
            (Number::Int64(i), Number::Int64(x)) => Number::Int64(i << x),
            _ => unreachable!(),
        }
    }
    pub fn __right_shift(&self, value: &Number) -> Number {
        match (self, value) {
            (Number::Int64(i), Number::Int64(x)) => Number::Int64(i >> x),
            _ => unreachable!(),
        }
    }
}
impl ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::Int64(i) => i.to_string(),
            Number::Float(f) => f.to_string(),
        }
    }
}

macro_rules! impl_binop {
    ($t:tt, $name:ident) => {
        impl $t for Number {
            type Output = Number;

            fn $name(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Number::Int64(lhs), Number::Int64(rhs)) => Number::Int64(lhs.$name(rhs)),
                    (Number::Int64(lhs), Number::Float(rhs)) => {
                        Number::Float((lhs as f64).$name(rhs))
                    }
                    (Number::Float(lhs), Number::Int64(rhs)) => {
                        Number::Float(lhs.$name(rhs as f64))
                    }
                    (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs.$name(rhs)),
                }
            }
        }
    };
}
impl_binop!(Add, add);
impl_binop!(Mul, mul);
impl_binop!(Sub, sub);
impl_binop!(Rem, rem);
impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int64(lhs), Number::Int64(rhs)) => Number::Float(lhs as f64 / rhs as f64),
            (Number::Int64(lhs), Number::Float(rhs)) => Number::Float(lhs as f64 / rhs),
            (Number::Float(lhs), Number::Int64(rhs)) => Number::Float(lhs / rhs as f64),
            (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs / rhs),
        }
    }
}
