use std::{fmt::Debug, rc::Rc};

use crate::{cell::UnsafeRefCell, List, Value};

#[derive(Clone)]
pub struct Iter(Rc<UnsafeRefCell<Box<dyn Iterator<Item = Value>>>>);

impl Debug for Iter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Iter").finish()
    }
}

impl Iter {
    pub fn new(iter: Box<dyn Iterator<Item = Value>>) -> Self {
        Self(UnsafeRefCell::rc(iter))
    }
    pub fn test(&self) -> bool {
        true
    }
    pub fn __next(&self) -> Value {
        self.0.borrow_mut().next().expect("stop iteration")
    }
    pub fn __list(&self) -> Value {
        let mut list = vec![];
        while let Some(v) = self.0.borrow_mut().next() {
            list.push(UnsafeRefCell::rc(v));
        }
        Value::List(List(UnsafeRefCell::rc(list)))
    }
}
