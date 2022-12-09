use std::{collections::HashMap, rc::Rc};

use crate::{
    cell::{UnsafeRefCell, UnsafeRefMut},
    number::Number,
    ImmutableString, Value,
};

#[derive(Debug, Clone)]
pub struct Dict(pub Rc<UnsafeRefCell<HashMap<DictKey, Rc<UnsafeRefCell<Value>>>>>);

impl Default for Dict {
    fn default() -> Self {
        Self(UnsafeRefCell::rc(Default::default()))
    }
}
impl PartialEq for Dict {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().eq(&other.0.borrow())
    }
}

impl Dict {
    pub fn includes<'a, T>(&self, value: &'a T) -> bool
    where
        Value: From<&'a T>,
    {
        self.0
            .borrow()
            .contains_key(&DictKey::from(&Value::from(value)))
    }
    pub fn __delete<'a, T>(&self, index: &'a T)
    where
        DictKey: From<&'a T>,
    {
        self.0.borrow_mut().remove(&DictKey::from(index));
    }

    pub fn __index_ref<'a, I>(&self, index: &'a I) -> UnsafeRefMut<Value>
    where
        DictKey: From<&'a I>,
    {
        let key = DictKey::from(index);
        self.0
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| UnsafeRefCell::rc(Default::default()))
            .borrow_mut()
    }
    pub fn __index_value<'a, I>(&self, index: &'a I) -> Value
    where
        DictKey: From<&'a I>,
    {
        let key = DictKey::from(index);
        self.0
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| UnsafeRefCell::rc(Default::default()))
            .borrow()
            .clone()
    }

    pub fn keys(&self) -> Value {
        let list = self
            .0
            .borrow()
            .keys()
            .map(|s| s.clone().into())
            .collect::<Vec<Value>>();
        Value::from(list)
    }
    pub fn setdefault(&self, key: &Value, value: &Value) {
        let key = DictKey::from(key);
        self.0
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| UnsafeRefCell::rc(value.clone()));
    }
    pub fn add<'a, T>(&self, value: &'a T)
    where
        DictKey: From<&'a T>,
    {
        let key = DictKey::from(value);
        self.0
            .borrow_mut()
            .insert(key, UnsafeRefCell::rc(Value::None));
    }
    pub fn __len(&self) -> Value {
        Value::Number(Number::Int64(self.0.borrow().len() as i64))
    }
    pub fn test(&self) -> bool {
        !self.0.borrow().is_empty()
    }
}

impl From<Vec<(Value, Value)>> for Dict {
    fn from(pairs: Vec<(Value, Value)>) -> Self {
        let map = pairs
            .into_iter()
            .map(|(key, value)| {
                let key = DictKey::from(&key);
                let value = UnsafeRefCell::rc(value);
                (key, value)
            })
            .collect::<HashMap<_, _>>();
        Self(UnsafeRefCell::rc(map))
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
            DictKey::String(s) => Value::String(ImmutableString(Rc::new(s))),
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

impl From<&Number> for DictKey {
    fn from(n: &Number) -> Self {
        Self::Number(*n)
    }
}
