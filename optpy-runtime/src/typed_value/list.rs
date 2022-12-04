use std::rc::Rc;

use crate::{
    cell::{UnsafeRefCell, UnsafeRefMut},
    number::Number,
};

use super::{IndexValue, TypedValue};

pub struct TypedList<T>(pub Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<T>>>>>);
impl<T> From<Vec<T>> for TypedList<T> {
    fn from(v: Vec<T>) -> Self {
        let list = v.into_iter().map(|v| UnsafeRefCell::rc(v)).collect();
        Self(UnsafeRefCell::rc(list))
    }
}

impl<T> Default for TypedList<T> {
    fn default() -> Self {
        Self(UnsafeRefCell::rc(vec![]))
    }
}

impl<T> TypedList<T> {
    pub fn __len(&self) -> Number {
        todo!()
    }
    pub fn reverse(&self) {
        todo!()
    }
    pub fn __index_value<I: IndexValue>(&self, _: I) -> T {
        todo!()
    }
    pub fn append(&self, x: T) {
        todo!()
    }
    pub fn pop(&self) -> T {
        todo!()
    }
    pub fn __mul(&self, _: Number) -> Self {
        todo!()
    }
    pub fn __index_ref(&self, index: Number) -> UnsafeRefMut<T> {
        match index {
            Number::Int64(i) => {
                if i < 0 {
                    let i = self.0.borrow().len() as i64 + i;
                    self.0.borrow_mut()[i as usize].borrow_mut()
                } else {
                    self.0.borrow_mut()[i as usize].borrow_mut()
                }
            }
            _ => todo!(),
        }
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
impl<T: TypedValue> TypedValue for TypedList<T> {
    fn __shallow_copy(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
