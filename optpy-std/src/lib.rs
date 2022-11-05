pub mod value {
    use std::{
        cell::RefCell,
        ops::{Mul, Rem},
        rc::Rc,
    };

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Value {
        List(Rc<RefCell<Vec<Value>>>),
        String(Rc<String>),
        Int64(i64),
        None,
    }

    impl Value {
        pub fn split(&self) -> Self {
            match self {
                Value::String(s) => {
                    let list = s
                        .split_whitespace()
                        .map(|s| Self::String(Rc::new(s.to_string())))
                        .collect();
                    Self::List(Rc::new(RefCell::new(list)))
                }
                _ => panic!("undefined method"),
            }
        }

        pub fn shallow_copy(&self) -> Self {
            self.clone()
        }

        pub fn index(&self, index: Self) -> Self {
            match (self, index) {
                (Self::List(list), Self::Int64(i)) => list.borrow()[i as usize].shallow_copy(),
                _ => todo!(),
            }
        }

        pub fn count(&self, value: Value) -> Value {
            match (self, value) {
                (Value::String(lhs), Value::String(rhs)) => {
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
                    Value::Int64(result)
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
            Value::Int64(v)
        }
    }
    impl Rem for Value {
        type Output = Value;

        fn rem(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Self::Int64(lhs), Self::Int64(rhs)) => Self::Int64(lhs % rhs),
                _ => todo!(),
            }
        }
    }
    impl Mul for Value {
        type Output = Value;

        fn mul(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Self::Int64(lhs), Self::Int64(rhs)) => Self::Int64(lhs * rhs),
                _ => todo!(),
            }
        }
    }
}

pub mod builtin {
    use std::{cell::RefCell, io::stdin, rc::Rc};

    use crate::value::Value;

    pub fn input() -> Value {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        Value::String(Rc::new(buf.trim().to_string()))
    }

    pub fn print(value: Value) {
        match value {
            Value::String(s) => {
                println!("{}", s);
            }
            Value::Int64(i) => {
                println!("{}", i);
            }
            _ => todo!(),
        }
    }

    pub fn map_int(value: Value) -> Value {
        match value {
            Value::List(list) => {
                let list = list
                    .borrow()
                    .iter()
                    .map(|v| int(v.shallow_copy()))
                    .collect();
                Value::List(Rc::new(RefCell::new(list)))
            }
            _ => todo!(),
        }
    }
    pub fn int(value: Value) -> Value {
        match value {
            Value::String(s) => {
                if let Ok(i) = s.parse::<i64>() {
                    Value::Int64(i)
                } else {
                    todo!()
                }
            }
            Value::Int64(_) => value,
            _ => panic!("invalid"),
        }
    }
}

pub use builtin::*;
pub use value::*;
