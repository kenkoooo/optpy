use std::rc::Rc;

use crate::{number::Number, ToValue, Value};

#[derive(Debug, Clone)]
pub struct ImmutableString(pub Rc<String>);

impl PartialEq for ImmutableString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for ImmutableString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl ImmutableString {
    pub fn split(&self) -> Value {
        let list = self
            .0
            .split_ascii_whitespace()
            .map(|s| Self(Rc::new(s.to_string())))
            .map(|s| Value::String(s))
            .collect::<Vec<Value>>();
        list.to_value()
    }
    pub fn strip(&self) -> Value {
        Value::String(Self(Rc::new(self.0.trim().to_string())))
    }
    pub fn __len(&self) -> Value {
        Value::Number(Number::Int64(self.0.chars().count() as i64))
    }
    pub fn count(&self, value: &Value) -> Value {
        match value {
            Value::String(value) => {
                let lhs = self.0.as_str();
                let rhs = value.0.as_str();
                Value::Number(Number::Int64(lhs.split(rhs).count() as i64 - 1))
            }
            _ => todo!(),
        }
    }
    pub fn test(&self) -> bool {
        !self.0.is_empty()
    }
}

impl From<&str> for ImmutableString {
    fn from(s: &str) -> Self {
        Self(Rc::new(s.to_string()))
    }
}

impl ToString for ImmutableString {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
