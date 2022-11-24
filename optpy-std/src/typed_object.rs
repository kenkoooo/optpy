use std::{collections::HashMap, fmt::Debug, hash::Hash, rc::Rc};

use crate::{cell::UnsafeRefCell, number::Number};

pub enum Object<V> {
    Ref(Rc<UnsafeRefCell<V>>),
    Value(V),
}

impl<V: PartialEq> PartialEq for Object<V> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Ref(l), Object::Ref(r)) => l.borrow().eq(&r.borrow()),
            (Object::Ref(l), Object::Value(r)) => l.borrow().eq(&r),
            (Object::Value(l), Object::Ref(r)) => l.eq(&r.borrow()),
            (Object::Value(l), Object::Value(r)) => l.eq(&r),
        }
    }
}
impl<V: Eq> Eq for Object<V> {}
impl<V: Debug> Debug for Object<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Ref(r) => r.borrow().fmt(f),
            Object::Value(v) => v.fmt(f),
        }
    }
}

impl<V: Clone> Clone for Object<V> {
    fn clone(&self) -> Self {
        match self {
            Self::Ref(r) => Self::Value(r.borrow().clone()),
            Self::Value(v) => Self::Value(v.clone()),
        }
    }
}
impl<V: PartialOrd> PartialOrd for Object<V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Ref(l), Object::Ref(r)) => l.borrow().partial_cmp(&r.borrow()),
            (Object::Ref(l), Object::Value(r)) => l.borrow().partial_cmp(&r),
            (Object::Value(l), Object::Ref(r)) => l.partial_cmp(&r.borrow()),
            (Object::Value(l), Object::Value(r)) => l.partial_cmp(&r),
        }
    }
}
impl<T: Default> Default for Object<T> {
    fn default() -> Self {
        Object::Value(T::default())
    }
}

pub trait Value: Clone + Debug {
    fn assign(&mut self, value: &Self) {
        *self = value.clone();
    }
}
impl<V: Clone> Object<V> {
    pub fn __inner(&self) -> V {
        match self {
            Object::Ref(r) => r.borrow().clone(),
            Object::Value(v) => v.clone(),
        }
    }
}

impl<V: Value> Object<V> {
    pub fn assign(&mut self, value: &Self) {
        match (self, value) {
            (Object::Ref(l), Object::Ref(r)) => l.borrow_mut().assign(&r.borrow()),
            (Object::Ref(l), Object::Value(r)) => l.borrow_mut().assign(r),
            (Object::Value(l), Object::Ref(r)) => l.assign(&r.borrow()),
            (Object::Value(l), Object::Value(r)) => l.assign(r),
        }
    }
}

impl<T> Object<ListValue<T>> {
    pub fn index(&self, index: &Object<NumberValue>) -> Object<T> {
        let r = match (self, index) {
            (Object::Ref(l), Object::Ref(r)) => l.borrow().index(&r.borrow()),
            (Object::Ref(l), Object::Value(r)) => l.borrow().index(&r),
            (Object::Value(l), Object::Ref(r)) => l.index(&r.borrow()),
            (Object::Value(l), Object::Value(r)) => l.index(&r),
        };
        Object::Ref(r)
    }
}
impl<K: Eq + Hash + Clone, V: Default> Object<DictValue<K, V>> {
    pub fn index(&self, index: &Object<K>) -> Object<V> {
        let r = match (self, index) {
            (Object::Ref(l), Object::Ref(r)) => l.borrow().index(&r.borrow()),
            (Object::Ref(l), Object::Value(r)) => l.borrow().index(&r),
            (Object::Value(l), Object::Ref(r)) => l.index(&r.borrow()),
            (Object::Value(l), Object::Value(r)) => l.index(&r),
        };
        Object::Ref(r)
    }
}

impl<T: Clone> From<&Object<T>> for Object<T> {
    fn from(obj: &Object<T>) -> Self {
        Object::Value(obj.__inner())
    }
}
impl From<&str> for Object<StringValue> {
    fn from(s: &str) -> Self {
        Object::Value(StringValue(Rc::new(s.to_string())))
    }
}
impl From<i64> for Object<NumberValue> {
    fn from(v: i64) -> Self {
        Object::Value(NumberValue(Number::Int64(v)))
    }
}
impl From<f64> for Object<NumberValue> {
    fn from(v: f64) -> Self {
        Object::Value(NumberValue(Number::Float(v)))
    }
}
impl<T: Clone> From<Vec<Object<T>>> for Object<ListValue<T>> {
    fn from(list: Vec<Object<T>>) -> Self {
        let list = list
            .into_iter()
            .map(|v| Rc::new(UnsafeRefCell::new(v.__inner())))
            .collect();
        Object::Value(ListValue(Rc::new(UnsafeRefCell::new(list))))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, PartialOrd)]
pub struct NumberValue(pub Number);
impl Value for NumberValue {}

impl From<NumberValue> for i64 {
    fn from(n: NumberValue) -> Self {
        match n.0 {
            Number::Int64(i) => i,
            Number::Float(f) => f as i64,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ListValue<T>(pub Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<T>>>>>);

impl<T: Debug> Value for ListValue<T> {}
impl<T> Clone for ListValue<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<T> Default for ListValue<T> {
    fn default() -> Self {
        Self(Rc::new(UnsafeRefCell::new(vec![])))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct StringValue(pub Rc<String>);
impl Value for StringValue {}

impl From<StringValue> for i64 {
    fn from(n: StringValue) -> Self {
        n.0.parse().expect("non-integer")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BooleanValue(pub bool);
impl Value for BooleanValue {}
impl BooleanValue {
    pub fn test(&self) -> bool {
        self.0
    }
}

#[derive(PartialEq, Debug)]
pub struct DictValue<K: Eq + Hash, V>(pub Rc<UnsafeRefCell<HashMap<K, Rc<UnsafeRefCell<V>>>>>);
impl<K: Eq + Hash + Debug, V: Debug> Value for DictValue<K, V> {}
impl<K: Eq + Hash, V> Clone for DictValue<K, V> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NoneValue;
impl Value for NoneValue {}

pub trait IndexValue<K, V> {
    fn index(&self, index: &K) -> Rc<UnsafeRefCell<V>>;
}

impl<T> IndexValue<NumberValue, T> for ListValue<T> {
    fn index(&self, index: &NumberValue) -> Rc<UnsafeRefCell<T>> {
        match index.0 {
            Number::Int64(i) => {
                if i < 0 {
                    let i = self.0.borrow().len() as i64 + i;
                    self.0.borrow_mut()[i as usize].clone()
                } else {
                    self.0.borrow_mut()[i as usize].clone()
                }
            }
            Number::Float(_) => unreachable!(),
        }
    }
}
impl<K: Eq + Hash + Clone, V: Default> IndexValue<K, V> for DictValue<K, V> {
    fn index(&self, index: &K) -> Rc<UnsafeRefCell<V>> {
        self.0
            .borrow_mut()
            .entry(index.clone())
            .or_insert_with(|| Rc::new(UnsafeRefCell::new(Default::default())))
            .clone()
    }
}

pub trait AddValue<T> {
    fn __add(&self, rhs: &Object<T>) -> Self;
}

impl AddValue<NumberValue> for Object<NumberValue> {
    fn __add(&self, rhs: &Object<NumberValue>) -> Self {
        let sum = self.__inner().0 + rhs.__inner().0;
        Object::Value(NumberValue(sum))
    }
}

pub trait Length {
    fn __len(&self) -> Object<NumberValue>;
}

impl<T> Length for Object<ListValue<T>> {
    fn __len(&self) -> Object<NumberValue> {
        let len = self.__inner().0.borrow().len() as i64;
        Object::Value(NumberValue(Number::Int64(len)))
    }
}
