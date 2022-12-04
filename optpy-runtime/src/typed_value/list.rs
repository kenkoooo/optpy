use std::rc::Rc;

use crate::cell::UnsafeRefCell;

use super::{AsValue, BinOps, IndexOps, TypedValue, UnaryOps};

pub struct TypedList<T: TypedValue>(pub Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<T>>>>>);
impl<T: TypedValue> TypedValue for TypedList<T> {}
impl<T: TypedValue> IndexOps for TypedList<T> {
    type Item = T;
}
impl<T: TypedValue> UnaryOps for TypedList<T> {}
impl<T: TypedValue> BinOps for TypedList<T> {}
impl<T: TypedValue> AsValue for TypedList<T> {}

impl<T: TypedValue> From<Vec<T>> for TypedList<T> {
    fn from(v: Vec<T>) -> Self {
        let list = v.into_iter().map(|v| UnsafeRefCell::rc(v)).collect();
        Self(UnsafeRefCell::rc(list))
    }
}

impl<T: TypedValue> Default for TypedList<T> {
    fn default() -> Self {
        Self(UnsafeRefCell::rc(vec![]))
    }
}

impl<T: TypedValue> TypedList<T> {
    pub fn __list(&self) -> TypedList<T> {
        let list = self
            .0
            .borrow()
            .iter()
            .map(|v| UnsafeRefCell::rc(v.borrow().__shallow_copy()))
            .collect::<Vec<_>>();
        Self(UnsafeRefCell::rc(list))
    }
}
