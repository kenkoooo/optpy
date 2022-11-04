pub mod value {
    use std::{
        ops::{Mul, Rem},
        rc::Rc,
    };

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Value {
        List(Vec<Value>),
        String(Rc<String>),
        Int64(i64),
        None,
    }

    impl Value {
        pub fn split(&self) -> Self {
            match self {
                Value::String(s) => Self::List(
                    s.split_whitespace()
                        .map(|s| Self::String(Rc::new(s.to_string())))
                        .collect(),
                ),
                _ => panic!("undefined method"),
            }
        }

        pub fn shallow_copy(&self) -> Self {
            todo!()
        }

        pub fn index(&self, index: Self) -> Self {
            match (self, index) {
                (Self::List(list), Self::Int64(i)) => list[i as usize].shallow_copy(),
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
    use std::{io::stdin, rc::Rc};

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
            _ => todo!(),
        }
    }
}

pub use builtin::*;
pub use value::*;
