use std::{
    collections::{HashMap, VecDeque},
    ops::Mul,
    rc::Rc,
};

use crate::{cell::UnsafeRefMut, dict::DictKey, number::Number, List};

type RefCell<T> = crate::cell::UnsafeRefCell<T>;

#[derive(Debug, Clone)]
pub enum Value {
    List(List),
    String(Rc<String>),
    Number(Number),
    Boolean(bool),
    Dict(Rc<RefCell<HashMap<DictKey, Rc<RefCell<Value>>>>>),
    Deque(Rc<RefCell<VecDeque<Value>>>),
    None,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => lhs.partial_cmp(rhs),
            (Value::String(lhs), Value::String(rhs)) => lhs.partial_cmp(rhs),
            _ => todo!(),
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0.eq(r0),
            (Self::Dict(l0), Self::Dict(r0)) => {
                let l = l0
                    .borrow()
                    .iter()
                    .all(|(key, value)| match r0.borrow().get(key) {
                        Some(r) => value.borrow().eq(&r.borrow()),
                        None => false,
                    });

                let r = r0
                    .borrow()
                    .iter()
                    .all(|(key, value)| match l0.borrow().get(key) {
                        Some(l) => value.borrow().eq(&l.borrow()),
                        None => false,
                    });
                r && l
            }
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

macro_rules! impl_binop {
    ($name:ident, $op:ident) => {
        impl Value {
            pub fn $name(&self, rhs: &Value) -> Value {
                #[allow(unused_imports)]
                use std::ops::*;
                match (self, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs.$op(*rhs)),
                    _ => unreachable!(),
                }
            }
        }
    };
}
impl_binop!(__add, add);
impl_binop!(__sub, sub);
impl_binop!(__rem, rem);
impl_binop!(__div, div);
impl_binop!(__pow, pow);

impl Value {
    pub fn __mul(&self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::List(list), rhs) => list.__mul(rhs),
            (Value::Number(a), Value::Number(b)) => Value::Number(*a * *b),
            _ => todo!(),
        }
    }
}

macro_rules! impl_compare {
    ($name:ident, $op:ident) => {
        impl Value {
            pub fn $name(&self, rhs: &Value) -> Value {
                Value::Boolean(self.$op(rhs))
            }
        }
    };
}
impl_compare!(__gt, gt);
impl_compare!(__ge, ge);
impl_compare!(__lt, lt);
impl_compare!(__le, le);
impl_compare!(__eq, eq);
impl_compare!(__ne, ne);
impl Value {
    fn includes(&self, value: &Value) -> bool {
        match self {
            Value::List(list) => list.includes(value),
            Value::Dict(map) => map.borrow().contains_key(&value.__as_dict_key()),
            _ => todo!(),
        }
    }
    pub fn __in(&self, rhs: &Value) -> Value {
        Value::Boolean(rhs.includes(self))
    }
    pub fn __not_in(&self, rhs: &Value) -> Value {
        Value::Boolean(!rhs.includes(self))
    }

    pub fn __bit_and(&self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a && *b),
            _ => todo!(),
        }
    }

    pub fn __delete(&self, index: &Value) {
        match self {
            Value::List(list) => list.__delete(index),
            Value::Dict(dict) => {
                let key = index.__as_dict_key();
                dict.borrow_mut().remove(&key);
            }
            _ => todo!(),
        }
    }

    pub fn none() -> Value {
        Value::None
    }
    pub fn dict(pairs: Vec<(Value, Value)>) -> Value {
        let map = pairs
            .into_iter()
            .map(|(key, value)| {
                let key = key.__as_dict_key();
                let value = Rc::new(RefCell::new(value));
                (key, value)
            })
            .collect::<HashMap<_, _>>();
        Value::Dict(Rc::new(RefCell::new(map)))
    }

    pub fn __shallow_copy(&self) -> Value {
        self.clone()
    }

    pub fn split(&self) -> Self {
        match self {
            Value::String(s) => {
                let list = s
                    .split_ascii_whitespace()
                    .map(|s| Value::String(Rc::new(s.to_string())))
                    .map(|v| Rc::new(RefCell::new(v)))
                    .collect();
                Value::List(List(Rc::new(RefCell::new(list))))
            }
            _ => unreachable!(),
        }
    }

    pub fn __index_ref(&self, index: &Value) -> UnsafeRefMut<Value> {
        match self {
            Value::List(list) => list.__index_ref(index),
            Value::Dict(dict) => {
                let key = index.__as_dict_key();
                dict.borrow_mut()
                    .entry(key)
                    .or_insert_with(|| Rc::new(RefCell::new(Value::none())))
                    .borrow_mut()
            }
            _ => todo!(),
        }
    }
    pub fn __index_value(&self, index: &Value) -> Value {
        match self {
            Value::List(list) => list.__index_value(index),
            Value::Dict(dict) => {
                let key = index.__as_dict_key();
                dict.borrow_mut()
                    .entry(key)
                    .or_insert_with(|| Rc::new(RefCell::new(Value::none())))
                    .borrow()
                    .clone()
            }
            _ => todo!(),
        }
    }

    pub fn __as_dict_key(&self) -> DictKey {
        match self {
            Value::String(s) => DictKey::String(s.to_string()),
            Value::Number(n) => DictKey::Number(*n),
            _ => unreachable!(),
        }
    }

    pub fn keys(&self) -> Value {
        match self {
            Value::Dict(dict) => {
                let list = dict
                    .borrow()
                    .keys()
                    .map(|s| Rc::new(RefCell::new(s.clone().into())))
                    .collect();
                Value::List(List(Rc::new(RefCell::new(list))))
            }
            _ => todo!(),
        }
    }

    pub fn assign(&mut self, value: &Value) {
        *self = value.clone();
    }

    pub fn reverse(&self) {
        match self {
            Value::List(list) => list.reverse(),
            _ => unreachable!(),
        }
    }

    pub fn pop(&self) -> Value {
        match self {
            Value::List(list) => list.pop(),
            _ => unreachable!(),
        }
    }
    pub fn popleft(&self) -> Value {
        match self {
            Value::Deque(deque) => deque
                .borrow_mut()
                .pop_front()
                .expect("pop from an empty deque"),
            _ => todo!(),
        }
    }
    pub fn strip(&self) -> Value {
        match self {
            Value::String(s) => Value::from(s.trim()),
            _ => unreachable!(),
        }
    }
    pub fn append(&self, value: &Value) {
        match self {
            Value::List(list) => list.append(value),
            Value::Deque(deque) => {
                deque.borrow_mut().push_back(value.clone());
            }
            _ => unreachable!(),
        }
    }
    pub fn appendleft(&self, value: &Value) {
        match self {
            Value::Deque(deque) => deque.borrow_mut().push_front(value.clone()),
            _ => unreachable!(),
        }
    }

    pub fn setdefault(&self, key: &Value, value: &Value) {
        match self {
            Value::Dict(dict) => {
                let key = key.__as_dict_key();
                dict.borrow_mut()
                    .entry(key)
                    .or_insert_with(|| Rc::new(RefCell::new(value.clone())));
            }
            _ => todo!(),
        }
    }
    pub fn add(&self, value: &Value) {
        match self {
            Value::Dict(map) => {
                let key = value.__as_dict_key();
                map.borrow_mut()
                    .insert(key, Rc::new(RefCell::new(Value::None)));
            }
            _ => unreachable!(),
        }
    }

    pub fn __floor_div(&self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs.floor_div(rhs)),
            _ => unreachable!(),
        }
    }

    pub fn __unary_add(&self) -> Value {
        self.clone()
    }
    pub fn __unary_sub(&self) -> Value {
        match self {
            Value::Number(i) => Value::Number(i.mul(Number::Int64(-1))),
            _ => unreachable!(),
        }
    }
    pub fn __unary_not(&self) -> Value {
        match self {
            Value::Boolean(b) => Value::Boolean(!*b),
            _ => unreachable!(),
        }
    }
    pub fn __len(&self) -> Value {
        match self {
            Value::List(list) => list.__len(),
            Value::String(s) => Value::Number(Number::Int64(s.chars().count() as i64)),
            Value::Dict(d) => Value::Number(Number::Int64(d.borrow().len() as i64)),
            _ => unreachable!(),
        }
    }
    pub fn sort(&self) {
        match self {
            Value::List(list) => list.sort(),
            _ => unreachable!(),
        }
    }

    pub fn test(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => unreachable!(),
        }
    }

    pub fn __number(&self) -> Number {
        match self {
            Value::Number(n) => *n,
            _ => unreachable!(),
        }
    }

    pub fn count(&self, value: &Value) -> Value {
        match (self, value) {
            (Value::List(list), value) => list.count(value),
            (Value::String(lhs), Value::String(rhs)) => {
                let lhs = lhs.as_str();
                let rhs = rhs.as_str();
                Value::Number(Number::Int64(lhs.split(rhs).count() as i64 - 1))
            }
            _ => todo!(),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(Rc::new(s.to_string()))
    }
}
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Number(Number::Int64(v))
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Number(Number::Float(v))
    }
}
impl From<Vec<Value>> for Value {
    fn from(list: Vec<Value>) -> Self {
        Value::List(List::from(list))
    }
}
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}
impl From<&Value> for Value {
    fn from(v: &Value) -> Self {
        v.__shallow_copy()
    }
}
impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.to_string(),
            Value::Number(n) => n.to_string(),
            _ => todo!(),
        }
    }
}
