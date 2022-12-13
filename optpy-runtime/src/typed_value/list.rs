use std::rc::Rc;

use crate::{
    cell::{UnsafeRefCell, UnsafeRefMut},
    number::Number,
};

use super::TypedValue;

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

impl<T: TypedValue> TypedList<T> {
    pub fn __index_value(&self, index: Number) -> T {
        match index {
            Number::Int64(i) => {
                if i < 0 {
                    let i = self.0.borrow().len() as i64 + i;
                    self.0.borrow()[i as usize].borrow().__shallow_copy()
                } else {
                    self.0.borrow()[i as usize].borrow().__shallow_copy()
                }
            }
            _ => todo!(),
        }
    }
    pub fn pop(&self) -> T {
        self.0.borrow_mut().pop().unwrap().borrow().__shallow_copy()
    }
    pub fn __mul(&self, x: Number) -> Self {
        let mut list = vec![];
        let x = match x {
            Number::Int64(x) => x,
            Number::Float(_) => unreachable!(),
        };
        for _ in 0..x {
            for element in self.0.borrow().iter() {
                list.push(element.borrow().__shallow_copy());
            }
        }
        Self::from(list)
    }
}

impl<T> TypedList<T> {
    pub fn __len(&self) -> Number {
        Number::Int64(self.0.borrow().len() as i64)
    }
    pub fn reverse(&self) {
        self.0.borrow_mut().reverse();
    }
    pub fn append(&self, x: T) {
        self.0.borrow_mut().push(UnsafeRefCell::rc(x))
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
