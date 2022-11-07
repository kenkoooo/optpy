pub mod value {
    use std::{
        cell::RefCell,
        ops::{Mul, Rem},
        rc::Rc,
    };

    #[derive(Debug, PartialEq, Clone)]
    pub struct Value {
        pub inner: Rc<RefCell<Inner>>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Inner {
        List(Vec<Value>),
        String(Rc<String>),
        Int64(i64),
        Float(f64),
        Boolean(bool),
        None,
    }

    impl Into<Value> for Inner {
        fn into(self) -> Value {
            Value {
                inner: Rc::new(RefCell::new(self)),
            }
        }
    }

    macro_rules! impl_value_compare {
        ($name:ident, $op:ident) => {
            pub fn $name(&self, value: Value) -> Value {
                match (&*self.inner.borrow(), &*value.inner.borrow()) {
                    (Inner::Int64(lhs), Inner::Int64(rhs)) => Inner::Boolean(lhs.$op(rhs)).into(),
                    _ => todo!(),
                }
            }
        };
    }

    macro_rules! impl_value_binop {
        ($name:ident, $op:ident) => {
            pub fn $name(&self, rhs: Value) -> Value {
                use std::ops::*;
                match (&*self.inner.borrow(), &*rhs.inner.borrow()) {
                    (Inner::Int64(lhs), Inner::Int64(rhs)) => Inner::Int64(lhs.$op(rhs)).into(),
                    _ => todo!(),
                }
            }
        };
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
            let rc = Rc::clone(&self.inner);
            Self { inner: rc }
        }

        pub fn index(&self, index: Self) -> Self {
            match (&*self.inner.borrow(), &*index.inner.borrow()) {
                (Inner::List(list), Inner::Int64(i)) => list[*i as usize].shallow_copy(),
                _ => todo!(),
            }
        }

        pub fn assign(&mut self, value: Value) {
            self.inner.replace(value.inner.borrow().clone());
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

        pub fn reverse(&mut self) {
            match &mut *self.inner.borrow_mut() {
                Inner::List(list) => list.reverse(),
                _ => todo!(),
            }
        }

        pub fn pop(&mut self) -> Value {
            match &mut *self.inner.borrow_mut() {
                Inner::List(list) => {
                    let last = list.pop();
                    match last {
                        Some(element) => element,
                        None => Inner::None.into(),
                    }
                }
                _ => todo!(),
            }
        }
        impl_value_compare!(is_gt, gt);
        impl_value_compare!(is_lt, lt);
        impl_value_compare!(is_le, le);
        impl_value_compare!(is_eq, eq);
        impl_value_compare!(is_ne, ne);

        impl_value_binop!(__add, add);
        impl_value_binop!(__sub, sub);
        impl_value_binop!(__mul, mul);
        impl_value_binop!(__mod, rem);

        pub fn __floor_div(&self, rhs: Value) -> Value {
            match (&*self.inner.borrow(), &*rhs.inner.borrow()) {
                (Inner::Int64(lhs), Inner::Int64(rhs)) => Inner::Int64(lhs / rhs).into(),
                _ => todo!(),
            }
        }
        pub fn __div(&self, rhs: Value) -> Value {
            match (&*self.inner.borrow(), &*rhs.inner.borrow()) {
                (Inner::Int64(lhs), Inner::Int64(rhs)) => {
                    if lhs % rhs == 0 {
                        Inner::Int64(lhs / rhs).into()
                    } else {
                        Inner::Float(*lhs as f64 / *rhs as f64).into()
                    }
                }
                _ => todo!(),
            }
        }

        pub fn __bool_or(&self, rhs: Value) -> Value {
            match (&*self.inner.borrow(), &*rhs.inner.borrow()) {
                (Inner::Boolean(lhs), Inner::Boolean(rhs)) => Inner::Boolean(*lhs || *rhs).into(),
                _ => todo!(),
            }
        }

        pub fn __bool_and(&self, rhs: Value) -> Value {
            match (&*self.inner.borrow(), &*rhs.inner.borrow()) {
                (Inner::Boolean(lhs), Inner::Boolean(rhs)) => Inner::Boolean(*lhs && *rhs).into(),
                _ => todo!(),
            }
        }

        pub fn test(&self) -> bool {
            match &*self.inner.borrow() {
                Inner::Boolean(x) => *x,
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
    impl From<bool> for Value {
        fn from(b: bool) -> Self {
            Inner::Boolean(b).into()
        }
    }
    impl ToString for Value {
        fn to_string(&self) -> String {
            match &*self.inner.borrow() {
                Inner::String(s) => s.to_string(),
                Inner::Int64(i) => i.to_string(),
                Inner::Float(f) => f.to_string(),
                _ => todo!(),
            }
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

    pub fn list(value: Value) -> Value {
        match &*value.inner.borrow() {
            Inner::List(list) => Inner::List(list.clone()).into(),
            _ => todo!(),
        }
    }

    pub fn range1(value: Value) -> Value {
        match &*value.inner.borrow() {
            Inner::Int64(i) => {
                let list = (0..(*i)).map(|i| Inner::Int64(i).into()).collect();
                Inner::List(list).into()
            }
            _ => todo!(),
        }
    }

    pub fn len(value: Value) -> Value {
        match &*value.inner.borrow() {
            Inner::List(list) => Inner::Int64(list.len() as i64).into(),
            _ => todo!(),
        }
    }
}

pub use builtin::*;
pub use value::*;

#[macro_export]
macro_rules! range {
    ($stop:expr) => {
        range1($stop)
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:expr),+) => {
        let s = [$($arg),+].iter().map(|v| v.to_string()).collect::<Vec<_>>();
        println!("{}", s.join(" "));
    };
}
