use std::rc::Rc;

use crate::{number::Number, Value};

pub trait ImmutableString {
    fn split(&self) -> Value;
    fn strip(&self) -> Value;
    fn __len(&self) -> Value;
    fn count(&self, value: &Value) -> Value;
    fn test(&self) -> bool;
}

impl ImmutableString for Rc<String> {
    fn split(&self) -> Value {
        let list = self
            .split_ascii_whitespace()
            .map(|s| Rc::new(s.to_string()))
            .map(|s| Value::String(s))
            .collect::<Vec<Value>>();
        Value::from(list)
    }
    fn strip(&self) -> Value {
        Value::String(Rc::new(self.trim().to_string()))
    }
    fn __len(&self) -> Value {
        Value::Number(Number::Int64(self.chars().count() as i64))
    }
    fn count(&self, value: &Value) -> Value {
        match value {
            Value::String(value) => {
                let lhs = self.as_str();
                let rhs = value.as_str();
                Value::Number(Number::Int64(lhs.split(rhs).count() as i64 - 1))
            }
            _ => todo!(),
        }
    }
    fn test(&self) -> bool {
        !self.is_empty()
    }
}
