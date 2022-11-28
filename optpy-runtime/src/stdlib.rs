use std::{collections::VecDeque, iter::FromIterator, rc::Rc};

use crate::{cell::UnsafeRefCell, number::Number, Value};

#[allow(non_snake_case)]
pub fn __math__gcd(a: &Value, b: &Value) -> Value {
    let a = a.__number();
    let b = b.__number();

    fn gcd(a: Number, b: Number) -> Number {
        if b == Number::Int64(0) {
            a
        } else {
            gcd(b, a % b)
        }
    }
    Value::Number(gcd(a, b))
}

#[allow(non_snake_case)]
pub fn __collections__deque0() -> Value {
    Value::Deque(Rc::new(UnsafeRefCell::new(VecDeque::new())))
}

#[allow(non_snake_case)]
pub fn __collections__deque1(value: &Value) -> Value {
    match value {
        Value::List(list) => Value::Deque(Rc::new(UnsafeRefCell::new(VecDeque::from_iter(
            list.borrow().iter().map(|v| v.borrow().clone()),
        )))),
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
