use std::{collections::VecDeque, iter::FromIterator};

use crate::{cell::UnsafeRefCell, Value};

#[allow(non_snake_case)]
pub fn __collections__deque0() -> Value {
    Value::Deque(UnsafeRefCell::rc(VecDeque::new()))
}

#[allow(non_snake_case)]
pub fn __collections__deque1(value: &Value) -> Value {
    match value {
        Value::List(list) => Value::Deque(UnsafeRefCell::rc(VecDeque::from_iter(
            list.0.borrow().iter().map(|v| v.borrow().clone()),
        ))),
        _ => todo!(),
    }
}

#[macro_export]
macro_rules! __collections__deque {
    () => {
        __collections__deque0()
    };
    ($value:expr) => {
        __collections__deque1($value)
    };
}
