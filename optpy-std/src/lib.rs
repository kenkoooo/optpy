pub mod value {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug, PartialEq, Clone, PartialOrd)]
    pub enum Value {
        List(Rc<RefCell<Vec<Ref>>>),
        Ref(Ref),
        Primitive(Primitive),
    }

    #[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
    pub struct Ref(pub Rc<RefCell<Value>>);
    impl Ref {
        pub fn new(value: Value) -> Self {
            Self(Rc::new(RefCell::new(value)))
        }
    }

    #[derive(Debug, PartialEq, Clone, PartialOrd)]
    pub enum Primitive {
        String(Rc<String>),
        Int64(i64),
        Float(f64),
        Boolean(bool),
        None,
    }

    impl Into<Value> for Primitive {
        fn into(self) -> Value {
            Value::Primitive(self)
        }
    }

    impl Ord for Value {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    impl Eq for Value {}

    macro_rules! impl_value_compare {
        ($name:ident, $op:ident) => {
            pub fn $name(&self, value: &Value) -> Value {
                Value::Primitive(Primitive::Boolean(self.$op(value)))
            }
        };
    }

    macro_rules! impl_value_binop {
        ($name:ident, $op:ident) => {
            pub fn $name(&self, rhs: &Value) -> Value {
                use std::ops::*;
                match (self.__primitive(), rhs.__primitive()) {
                    (Primitive::Int64(lhs), Primitive::Int64(rhs)) => {
                        Value::Primitive(Primitive::Int64(lhs.$op(rhs)))
                    }
                    (Primitive::Float(lhs), Primitive::Float(rhs)) => {
                        Value::Primitive(Primitive::Float(lhs.$op(rhs)))
                    }
                    (Primitive::Int64(lhs), Primitive::Float(rhs)) => {
                        Value::Primitive(Primitive::Float((lhs as f64).$op(rhs)))
                    }
                    (Primitive::Float(lhs), Primitive::Int64(rhs)) => {
                        Value::Primitive(Primitive::Float(lhs.$op(rhs as f64)))
                    }
                    _ => todo!("{:?} {:?}", self, rhs),
                }
            }
        };
    }

    impl Value {
        pub fn none() -> Self {
            Self::Primitive(Primitive::None)
        }

        pub fn __primitive(&self) -> Primitive {
            match self {
                Value::Ref(r) => r.0.borrow().__primitive(),
                Value::Primitive(p) => p.clone(),
                _ => unreachable!(),
            }
        }

        pub fn split(&self) -> Self {
            let value = self.__primitive();
            match value {
                Primitive::String(s) => {
                    let list = s
                        .split_whitespace()
                        .map(|s| Value::Primitive(Primitive::String(Rc::new(s.to_string()))))
                        .map(|s| Ref::new(s))
                        .collect();
                    Value::List(Rc::new(RefCell::new(list)))
                }
                _ => unreachable!(),
            }
        }

        pub fn index(&self, index: &Value) -> Self {
            match self {
                Value::Ref(r) => r.0.borrow().index(index),
                Value::List(list) => {
                    let i = match index.__primitive() {
                        Primitive::Int64(i) => i as usize,
                        _ => unreachable!(),
                    };
                    let r = &list.borrow()[i];
                    Value::Ref(r.clone())
                }
                _ => unreachable!(),
            }
        }

        pub fn assign(&mut self, value: &Value) {
            match (&self, value) {
                (_, Value::Ref(r)) => {
                    self.assign(&r.0.borrow());
                }
                (Value::Ref(r), _) => {
                    let value = value.clone();
                    r.0.replace(value);
                }
                _ => {
                    *self = value.clone();
                }
            }
        }

        pub fn count(&self, value: &Value) -> Value {
            let (lhs, rhs) = match (self.__primitive(), value.__primitive()) {
                (Primitive::String(lhs), Primitive::String(rhs)) => (lhs, rhs),
                _ => unreachable!(),
            };

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
            Value::Primitive(Primitive::Int64(result))
        }

        pub fn reverse(&mut self) {
            match self {
                Value::Ref(r) => r.0.borrow_mut().reverse(),
                Value::List(list) => {
                    list.borrow_mut().reverse();
                }
                _ => todo!(),
            }
        }

        pub fn pop(&mut self) -> Value {
            match self {
                Value::List(list) => match list.borrow_mut().pop() {
                    Some(last) => Value::Ref(last),
                    None => {
                        panic!("empty");
                    }
                },
                Value::Ref(r) => r.0.borrow_mut().pop(),
                _ => todo!(),
            }
        }
        pub fn append(&mut self, value: &Value) {
            match self {
                Value::List(list) => {
                    list.borrow_mut().push(Ref::new(value.clone()));
                }
                Value::Ref(r) => r.0.borrow_mut().append(value),
                _ => unreachable!(),
            }
        }

        impl_value_compare!(__gt, gt);
        impl_value_compare!(__ge, ge);
        impl_value_compare!(__lt, lt);
        impl_value_compare!(__le, le);
        impl_value_compare!(__eq, eq);
        impl_value_compare!(__ne, ne);

        impl_value_binop!(__add, add);
        impl_value_binop!(__sub, sub);
        impl_value_binop!(__mul, mul);
        impl_value_binop!(__mod, rem);

        pub fn __floor_div(&self, rhs: &Value) -> Value {
            match (self.__primitive(), rhs.__primitive()) {
                (Primitive::Int64(lhs), Primitive::Int64(rhs)) => {
                    Value::Primitive(Primitive::Int64(lhs / rhs))
                }
                _ => todo!(),
            }
        }
        pub fn __div(&self, rhs: &Value) -> Value {
            match (self.__primitive(), rhs.__primitive()) {
                (Primitive::Int64(lhs), Primitive::Int64(rhs)) => {
                    if lhs % rhs == 0 {
                        Value::Primitive(Primitive::Int64(lhs / rhs))
                    } else {
                        Value::Primitive(Primitive::Float(lhs as f64 / rhs as f64))
                    }
                }
                _ => todo!(),
            }
        }

        pub fn __unary_add(&self) -> Value {
            self.clone()
        }
        pub fn __unary_sub(&self) -> Value {
            match self.__primitive() {
                Primitive::Int64(i) => Primitive::Int64(-i).into(),
                Primitive::Float(f) => Primitive::Float(-f).into(),
                _ => todo!(),
            }
        }

        pub fn test(&self) -> bool {
            match self.__primitive() {
                Primitive::Boolean(x) => x,
                _ => todo!(),
            }
        }
    }

    impl From<&str> for Value {
        fn from(s: &str) -> Self {
            Primitive::String(Rc::new(s.to_string())).into()
        }
    }
    impl From<i64> for Value {
        fn from(v: i64) -> Self {
            Primitive::Int64(v).into()
        }
    }
    impl From<f64> for Value {
        fn from(v: f64) -> Self {
            Primitive::Float(v).into()
        }
    }
    impl From<Vec<Value>> for Value {
        fn from(list: Vec<Value>) -> Self {
            let list = list.into_iter().map(Ref::new).collect();
            Value::List(Rc::new(RefCell::new(list)))
        }
    }
    impl From<bool> for Value {
        fn from(b: bool) -> Self {
            Primitive::Boolean(b).into()
        }
    }
    impl ToString for Value {
        fn to_string(&self) -> String {
            match self.__primitive() {
                Primitive::String(s) => s.to_string(),
                Primitive::Int64(i) => i.to_string(),
                Primitive::Float(f) => f.to_string(),
                _ => todo!(),
            }
        }
    }
}

pub mod builtin {
    use std::{cell::RefCell, io::stdin, rc::Rc};

    use crate::{value::Value, Primitive, Ref};

    pub fn input() -> Value {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        Value::from(buf.trim())
    }

    pub fn map_int(value: &Value) -> Value {
        match value {
            Value::List(list) => {
                let list = list
                    .borrow()
                    .iter()
                    .map(|v| int(&v.0.borrow()))
                    .map(Ref::new)
                    .collect();
                Value::List(Rc::new(RefCell::new(list)))
            }
            Value::Ref(r) => map_int(&r.0.borrow()),
            _ => todo!(),
        }
    }
    pub fn int(value: &Value) -> Value {
        match value.__primitive() {
            Primitive::String(s) => {
                if let Ok(i) = s.parse::<i64>() {
                    Primitive::Int64(i).into()
                } else {
                    todo!()
                }
            }
            Primitive::Int64(_) => value.clone(),
            _ => panic!("invalid"),
        }
    }

    pub fn list(value: &Value) -> Value {
        match value {
            Value::List(list) => {
                let list = list.borrow().clone();
                Value::List(Rc::new(RefCell::new(list)))
            }
            _ => todo!(),
        }
    }

    pub fn range1(value: &Value) -> Value {
        match value.__primitive() {
            Primitive::Int64(i) => {
                let list = (0..i)
                    .map(|i| Ref::new(Primitive::Int64(i).into()))
                    .collect();
                Value::List(Rc::new(RefCell::new(list)))
            }
            _ => todo!(),
        }
    }

    pub fn sorted(value: &Value) -> Value {
        match value {
            Value::List(list) => {
                list.borrow_mut().sort();
            }
            _ => todo!(),
        }
        value.clone()
    }

    pub fn len(value: &Value) -> Value {
        match value {
            Value::List(list) => Primitive::Int64(list.borrow().len() as i64).into(),
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
