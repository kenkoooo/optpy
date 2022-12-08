use crate::number::Number;

use super::{Bool, IndexValue, TypedValue};

impl Number {
    pub fn __min(&self, rhs: Self) -> Self {
        if self < &rhs {
            *self
        } else {
            rhs
        }
    }

    pub fn __sub(&self, rhs: Self) -> Self {
        *self - rhs
    }

    pub fn __add(&self, rhs: Self) -> Self {
        *self + rhs
    }
    pub fn __mul(&self, rhs: Self) -> Self {
        *self * rhs
    }

    pub fn __gt(&self, rhs: Self) -> Bool {
        Bool::from(*self > rhs)
    }
    pub fn __eq(&self, rhs: Self) -> Bool {
        Bool::from(*self == rhs)
    }
    pub fn __ne(&self, rhs: Self) -> Bool {
        Bool::from(*self != rhs)
    }
    pub fn __unary_sub(&self) -> Self {
        match self {
            Number::Int64(i) => Number::Int64(-i),
            Number::Float(f) => Number::Float(-f),
        }
    }
}

impl TypedValue for Number {
    fn __shallow_copy(&self) -> Self {
        *self
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::Int64(0)
    }
}
impl IndexValue for Number {
    fn __as_number(&self) -> Number {
        *self
    }
}
