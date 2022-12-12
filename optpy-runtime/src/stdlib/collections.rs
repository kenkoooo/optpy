use crate::{Deque, Value};

#[allow(non_snake_case)]
pub fn __collections__deque0() -> Value {
    Value::Deque(Default::default())
}

#[allow(non_snake_case)]
pub fn __collections__deque1(value: Value) -> Value {
    match value {
        Value::List(list) => Value::Deque(Deque::from(list)),
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
