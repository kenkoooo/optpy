use std::{ops::Sub, rc::Rc};

use num_bigint::BigInt;

pub enum Value {
    String { inner: Rc<String> },
    Integer { inner: Rc<BigInt> },
    List { inner: Rc<Vec<Value>> },
    None,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::String { inner } => Self::String {
                inner: Rc::clone(inner),
            },
            Self::Integer { inner } => Self::Integer {
                inner: Rc::clone(inner),
            },
            Self::List { inner } => Self::List {
                inner: Rc::clone(inner),
            },
            Self::None => Self::None,
        }
    }
}

impl Value {
    pub fn i64(v: i64) -> Self {
        Self::Integer {
            inner: Rc::new(v.into()),
        }
    }
    pub fn split(&self) -> Self {
        match self {
            Self::String { inner } => {
                let list = inner
                    .split_ascii_whitespace()
                    .map(|s| Self::String {
                        inner: Rc::new(s.to_string()),
                    })
                    .collect();
                Self::List {
                    inner: Rc::new(list),
                }
            }
            _ => panic!(),
        }
    }
    pub fn index(&self, value: Value) -> Self {
        match (self, value) {
            (Self::List { inner: list }, Self::Integer { inner: index }) => {
                let index: usize = index.as_ref().try_into().unwrap();
                list.get(index).unwrap().clone()
            }
            _ => todo!(),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::String { inner } => inner.as_ref().clone(),
            Value::Integer { inner } => inner.as_ref().to_string(),
            Value::None => String::new(),
            _ => todo!(),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer { inner: lhs }, Value::Integer { inner: rhs }) => {
                let result = lhs.as_ref() - rhs.as_ref();
                Value::Integer {
                    inner: Rc::new(result),
                }
            }
            _ => panic!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::String { inner: lhs }, Value::String { inner: rhs }) => lhs.partial_cmp(rhs),
            (Value::Integer { inner: lhs }, Value::Integer { inner: rhs }) => lhs.partial_cmp(rhs),
            (Value::None, Value::None) => Some(std::cmp::Ordering::Equal),
            _ => todo!(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String { inner: l_inner }, Self::String { inner: r_inner }) => {
                l_inner == r_inner
            }
            (Self::Integer { inner: l_inner }, Self::Integer { inner: r_inner }) => {
                l_inner == r_inner
            }
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}
