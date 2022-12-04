use crate::number::Number;

use super::{AsValue, BinOps, IndexOps, TypedValue, UnaryOps};

impl TypedValue for Number {}
impl AsValue for Number {}
impl BinOps for Number {
    fn __min<T: TypedValue>(&self, rhs: &T) -> Self {
        let rhs = rhs.__as_number();
        if self < &rhs {
            *self
        } else {
            rhs
        }
    }
}
impl UnaryOps for Number {}
impl IndexOps for Number {
    type Item = Self;
}

impl Default for Number {
    fn default() -> Self {
        Number::Int64(0)
    }
}
