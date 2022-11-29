use std::{collections::VecDeque, iter::FromIterator, rc::Rc};

use crate::{cell::UnsafeRefCell, List, Value};

#[derive(Debug, Clone)]
pub struct Deque(Rc<UnsafeRefCell<VecDeque<Value>>>);

impl Default for Deque {
    fn default() -> Self {
        Self(UnsafeRefCell::rc(Default::default()))
    }
}

impl Deque {
    pub fn popleft(&self) -> Value {
        self.0
            .borrow_mut()
            .pop_front()
            .expect("pop from an empty deque")
    }
    pub fn append(&self, value: &Value) {
        self.0.borrow_mut().push_back(value.clone());
    }
    pub fn appendleft(&self, value: &Value) {
        self.0.borrow_mut().push_front(value.clone());
    }
}

impl From<&List> for Deque {
    fn from(list: &List) -> Self {
        Deque(UnsafeRefCell::rc(VecDeque::from_iter(
            list.0.borrow().iter().map(|v| v.borrow().clone()),
        )))
    }
}
