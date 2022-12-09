use std::{collections::VecDeque, rc::Rc};

use crate::cell::UnsafeRefCell;

pub trait Deque<T> {
    fn popleft(&self) -> T;
    fn append(&self, value: &T);
    fn appendleft(&self, value: &T);
    fn test(&self) -> bool;
}

impl<T> Deque<T> for Rc<UnsafeRefCell<VecDeque<T>>>
where
    T: Clone,
{
    fn popleft(&self) -> T {
        self.borrow_mut()
            .pop_front()
            .expect("pop from an empty deque")
    }
    fn append(&self, value: &T) {
        self.borrow_mut().push_back(value.clone());
    }
    fn appendleft(&self, value: &T) {
        self.borrow_mut().push_front(value.clone());
    }
    fn test(&self) -> bool {
        !self.borrow().is_empty()
    }
}
