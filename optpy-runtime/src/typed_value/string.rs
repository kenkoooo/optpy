use super::{AsValue, BinOps, IndexOps, TypedValue, UnaryOps};

pub struct TypedString();
impl TypedValue for TypedString {}
impl IndexOps for TypedString {
    type Item = Self;
}
impl UnaryOps for TypedString {}
impl BinOps for TypedString {}
impl AsValue for TypedString {}
impl Default for TypedString {
    fn default() -> Self {
        Self()
    }
}
