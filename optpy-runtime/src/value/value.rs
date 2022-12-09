use std::ops::Mul;

use crate::{cell::UnsafeRefMut, number::Number, Deque, Dict, ImmutableString, List};

#[derive(Debug, Clone)]
pub enum Value {
    List(List),
    String(ImmutableString),
    Number(Number),
    Boolean(bool),
    Dict(Dict),
    Deque(Deque),
    None,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => lhs.partial_cmp(rhs),
            (Value::String(lhs), Value::String(rhs)) => lhs.partial_cmp(rhs),
            (Value::List(lhs), Value::List(rhs)) => lhs.partial_cmp(rhs),
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
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Dict(l0), Self::Dict(r0)) => l0 == r0,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::None
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
            Value::Dict(map) => map.includes(value),
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
            Value::Dict(dict) => dict.__delete(index),
            _ => todo!(),
        }
    }

    pub fn dict(pairs: Vec<(Value, Value)>) -> Value {
        Value::Dict(Dict::from(pairs))
    }

    pub fn __shallow_copy(&self) -> Value {
        self.clone()
    }

    pub fn split(&self) -> Self {
        match self {
            Value::String(s) => s.split(),
            _ => unreachable!(),
        }
    }

    pub fn __index_ref(&self, index: &Value) -> UnsafeRefMut<Value> {
        match self {
            Value::List(list) => list.__index_ref(index),
            Value::Dict(dict) => dict.__index_ref(index),
            _ => todo!(),
        }
    }
    pub fn __index_value(&self, index: &Value) -> Value {
        match self {
            Value::List(list) => list.__index_value(index),
            Value::Dict(dict) => dict.__index_value(index),
            _ => todo!(),
        }
    }

    pub fn keys(&self) -> Value {
        match self {
            Value::Dict(dict) => dict.keys(),
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
            Value::Deque(deque) => deque.popleft(),
            _ => todo!(),
        }
    }
    pub fn strip(&self) -> Value {
        match self {
            Value::String(s) => s.strip(),
            _ => unreachable!(),
        }
    }
    pub fn append(&self, value: &Value) {
        match self {
            Value::List(list) => list.append(value),
            Value::Deque(deque) => deque.append(value),
            _ => unreachable!(),
        }
    }
    pub fn appendleft(&self, value: &Value) {
        match self {
            Value::Deque(deque) => deque.appendleft(value),
            _ => unreachable!(),
        }
    }

    pub fn setdefault(&self, key: &Value, value: &Value) {
        match self {
            Value::Dict(dict) => dict.setdefault(key, value),
            _ => todo!(),
        }
    }
    pub fn add(&self, value: &Value) {
        match self {
            Value::Dict(dict) => dict.add(value),
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
    pub fn __left_shift(&self, value: &Value) -> Value {
        match (self, value) {
            (Value::Number(i), Value::Number(x)) => Value::Number(i.__left_shift(x)),
            _ => todo!(),
        }
    }
    pub fn __right_shift(&self, value: &Value) -> Value {
        match (self, value) {
            (Value::Number(i), Value::Number(x)) => Value::Number(i.__right_shift(x)),
            _ => todo!(),
        }
    }
    pub fn __len(&self) -> Value {
        match self {
            Value::List(list) => list.__len(),
            Value::Dict(dict) => dict.__len(),
            Value::String(s) => s.__len(),
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
            Value::List(list) => list.test(),
            Value::String(s) => s.test(),
            Value::Number(n) => n.test(),
            Value::Dict(dict) => dict.test(),
            Value::Deque(deque) => deque.test(),
            Value::None => false,
        }
    }

    pub fn __number(&self) -> Number {
        match self {
            Value::Number(n) => *n,
            _ => unreachable!(),
        }
    }

    pub fn count(&self, value: &Value) -> Value {
        match self {
            Value::List(list) => list.count(value),
            Value::String(s) => s.count(value),
            _ => todo!(),
        }
    }
    pub fn index(&self, value: &Value) -> Value {
        match self {
            Value::List(list) => list.index(value),
            _ => todo!(),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(ImmutableString::from(s))
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
            Value::List(list) => list.to_string(),
            _ => todo!(),
        }
    }
}
