use std::{collections::HashMap, rc::Rc};

use crate::{
    cell::{UnsafeRefCell, UnsafeRefMut},
    number::Number,
    Value,
};

pub trait Dict {
    fn includes(&self, value: &Value) -> bool;
    fn __delete(&self, index: &Value);
    fn __index_ref(&self, index: &Value) -> UnsafeRefMut<Value>;
    fn __index_value(&self, index: &Value) -> Value;
    fn keys(&self) -> Value;
    fn setdefault(&self, key: &Value, value: &Value);
    fn add(&self, value: &Value);
    fn __len(&self) -> Value;
    fn test(&self) -> bool;
}

impl Dict for Rc<UnsafeRefCell<HashMap<DictKey, Rc<UnsafeRefCell<Value>>>>> {
    fn includes(&self, value: &Value) -> bool {
        self.borrow().contains_key(&DictKey::from(value))
    }
    fn __delete(&self, index: &Value) {
        self.borrow_mut().remove(&DictKey::from(index));
    }

    fn __index_ref(&self, index: &Value) -> UnsafeRefMut<Value> {
        let key = DictKey::from(index);
        self.borrow_mut()
            .entry(key)
            .or_insert_with(|| UnsafeRefCell::rc(Default::default()))
            .borrow_mut()
    }
    fn __index_value(&self, index: &Value) -> Value {
        let key = DictKey::from(index);
        self.borrow_mut()
            .entry(key)
            .or_insert_with(|| UnsafeRefCell::rc(Default::default()))
            .borrow()
            .clone()
    }

    fn keys(&self) -> Value {
        let list = self
            .borrow()
            .keys()
            .map(|s| s.clone().into())
            .collect::<Vec<Value>>();
        Value::from(list)
    }
    fn setdefault(&self, key: &Value, value: &Value) {
        let key = DictKey::from(key);
        self.borrow_mut()
            .entry(key)
            .or_insert_with(|| UnsafeRefCell::rc(value.clone()));
    }
    fn add(&self, value: &Value) {
        let key = DictKey::from(value);
        self.borrow_mut()
            .insert(key, UnsafeRefCell::rc(Value::None));
    }
    fn __len(&self) -> Value {
        Value::Number(Number::Int64(self.borrow().len() as i64))
    }
    fn test(&self) -> bool {
        !self.borrow().is_empty()
    }
}

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

impl From<&Value> for DictKey {
    fn from(value: &Value) -> Self {
        match value {
            Value::String(s) => Self::String(s.to_string()),
            Value::Number(n) => Self::Number(*n),
            _ => unreachable!(),
        }
    }
}
