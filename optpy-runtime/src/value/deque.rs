use std::{collections::VecDeque, iter::FromIterator, rc::Rc};

use crate::{cell::UnsafeRefCell, List, ToValue, Value};

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
    pub fn append<'a, T>(&self, value: &'a T)
    where
        T: ToValue,
    {
        self.0.borrow_mut().push_back(value.to_value());
    }
    pub fn appendleft<'a, T>(&self, value: &'a T)
    where
        T: ToValue,
    {
        self.0.borrow_mut().push_front(value.to_value());
    }
    pub fn test(&self) -> bool {
        !self.0.borrow().is_empty()
    }
}

impl From<&List> for Deque {
    fn from(list: &List) -> Self {
        Deque(UnsafeRefCell::rc(VecDeque::from_iter(
            list.0.borrow().iter().map(|v| v.borrow().clone()),
        )))
    }
}
