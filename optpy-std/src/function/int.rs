use std::{rc::Rc, str::FromStr};

use num_bigint::BigInt;

use crate::Value;

pub fn int(value: Value) -> Value {
    match value {
        Value::String { inner } => {
            let integer = BigInt::from_str(&inner).unwrap();
            Value::Integer {
                inner: Rc::new(integer),
            }
        }
        Value::Integer { inner } => Value::Integer { inner },
        _ => panic!(),
    }
}
