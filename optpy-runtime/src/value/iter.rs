use std::{fmt::Debug, rc::Rc};

use crate::{cell::UnsafeRefCell, List, Value};

#[derive(Clone)]
pub struct Iter<T> {
    iter: Rc<UnsafeRefCell<Box<dyn Iterator<Item = T>>>>,
    peeked: Rc<UnsafeRefCell<Option<T>>>,
}

impl<T> Debug for Iter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Iter").finish()
    }
}

impl Iter<Value> {
    pub fn new(iter: Box<dyn Iterator<Item = Value>>) -> Self {
        Self {
            iter: UnsafeRefCell::rc(iter),
            peeked: UnsafeRefCell::rc(None),
        }
    }
    pub fn test(&self) -> bool {
        true
    }
    pub fn __next(&self) -> Option<Value> {
        if let Some(peeked) = self.peeked.borrow_mut().take() {
            return Some(peeked);
        }
        self.iter.borrow_mut().next()
    }
    pub fn __has_next(&self) -> bool {
        if self.peeked.borrow().is_some() {
            return true;
        }
        match self.iter.borrow_mut().next() {
            Some(peeked) => {
                self.peeked.borrow_mut().replace(peeked);
                true
            }
            None => false,
        }
    }

    pub fn __list(&self) -> Value {
        let mut list = vec![];
        if let Some(peeked) = self.peeked.borrow_mut().take() {
            list.push(UnsafeRefCell::rc(peeked));
        }
        while let Some(v) = self.iter.borrow_mut().next() {
            list.push(UnsafeRefCell::rc(v));
        }
        Value::List(List(UnsafeRefCell::rc(list)))
    }

    pub fn any(&self) -> bool {
        if let Some(peeked) = self.peeked.borrow_mut().take() {
            if peeked.test() {
                return true;
            }
        }
        self.iter.borrow_mut().any(|v| v.test())
    }
    pub fn all(&self) -> bool {
        if let Some(peeked) = self.peeked.borrow_mut().take() {
            if !peeked.test() {
                return false;
            }
        }
        self.iter.borrow_mut().all(|v| v.test())
    }
}
