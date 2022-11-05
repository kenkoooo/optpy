pub mod value {
    use std::{
        cell::RefCell,
        ops::{Mul, Rem},
        rc::Rc,
    };

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Value {
        pub inner: Rc<RefCell<Inner>>,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Inner {
        List(Vec<Value>),
        String(Rc<String>),
        Int64(i64),
        None,
    }

    impl Into<Value> for Inner {
        fn into(self) -> Value {
            Value {
                inner: Rc::new(RefCell::new(self)),
            }
        }
    }

    impl Value {
        pub fn none() -> Self {
            Inner::None.into()
        }
        pub fn split(&self) -> Self {
            match &*self.inner.borrow() {
                Inner::String(s) => {
                    let list = s
                        .split_whitespace()
                        .map(|s| Inner::String(Rc::new(s.to_string())).into())
                        .collect();
                    Inner::List(list).into()
                }
                _ => panic!("undefined method"),
            }
        }

        pub fn shallow_copy(&self) -> Self {
            self.clone()
        }

        pub fn index(&self, index: Self) -> Self {
            match (&*self.inner.borrow(), &*index.inner.borrow()) {
                (Inner::List(list), Inner::Int64(i)) => list[*i as usize].shallow_copy(),
                _ => todo!(),
            }
        }

        pub fn assign(&mut self, value: Value) {
            self.inner = value.inner.clone();
        }

        pub fn count(&self, value: Value) -> Value {
            match (&*self.inner.borrow(), &*value.inner.borrow()) {
                (Inner::String(lhs), Inner::String(rhs)) => {
                    let mut i = 0;
                    let mut result = 0;
                    while i < lhs.len() {
                        if lhs[i..].starts_with(rhs.as_ref()) {
                            i += rhs.len();
                            result += 1;
                        } else {
                            i += 1;
                        }
                    }
                    Inner::Int64(result).into()
                }
                _ => todo!(),
            }
        }
    }

    impl From<&str> for Value {
        fn from(s: &str) -> Self {
            Inner::String(Rc::new(s.to_string())).into()
        }
    }
    impl From<i64> for Value {
        fn from(v: i64) -> Self {
            Inner::Int64(v).into()
        }
    }
    impl From<Vec<Value>> for Value {
        fn from(list: Vec<Value>) -> Self {
            Inner::List(list).into()
        }
    }
    impl Rem for Value {
        type Output = Value;

        fn rem(self, rhs: Self) -> Self::Output {
            match (&*self.inner.borrow(), &*rhs.inner.borrow()) {
                (Inner::Int64(lhs), Inner::Int64(rhs)) => Inner::Int64(lhs % rhs).into(),
                _ => todo!(),
            }
        }
    }
    impl Mul for Value {
        type Output = Value;

        fn mul(self, rhs: Self) -> Self::Output {
            match (&*self.inner.borrow(), &*rhs.inner.borrow()) {
                (Inner::Int64(lhs), Inner::Int64(rhs)) => Inner::Int64(lhs * rhs).into(),
                _ => todo!(),
            }
        }
    }
}

pub mod builtin {
    use std::io::stdin;

    use crate::value::{Inner, Value};

    pub fn input() -> Value {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        Value::from(buf.trim())
    }

    pub fn print(value: Value) {
        match &*value.inner.borrow() {
            Inner::String(s) => {
                println!("{}", s);
            }
            Inner::Int64(i) => {
                println!("{}", i);
            }
            _ => todo!(),
        }
    }

    pub fn map_int(value: Value) -> Value {
        match &*value.inner.borrow() {
            Inner::List(list) => {
                let list = list.iter().map(|v| int(v.shallow_copy())).collect();
                Inner::List(list).into()
            }
            _ => todo!(),
        }
    }
    pub fn int(value: Value) -> Value {
        match &*value.inner.borrow() {
            Inner::String(s) => {
                if let Ok(i) = s.parse::<i64>() {
                    Inner::Int64(i).into()
                } else {
                    todo!()
                }
            }
            Inner::Int64(_) => value.clone(),
            _ => panic!("invalid"),
        }
    }
}

pub use builtin::*;
pub use value::*;
