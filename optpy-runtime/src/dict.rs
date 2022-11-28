use std::rc::Rc;

use crate::{number::Number, Value};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DictKey {
    Number(Number),
    String(String),
}

impl Into<Value> for DictKey {
    fn into(self) -> Value {
        match self {
            DictKey::Number(n) => Value::Number(n),
            DictKey::String(s) => Value::String(Rc::new(s)),
        }
    }
}
