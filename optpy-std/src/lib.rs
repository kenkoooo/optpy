#[cfg(feature = "unsafecell")]
pub mod cell {
    use std::{
        cell::UnsafeCell,
        ops::{Deref, DerefMut},
        ptr::NonNull,
    };

    pub struct UnsafeRef<T: ?Sized> {
        value: NonNull<T>,
    }
    impl<T: ?Sized> Deref for UnsafeRef<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            unsafe { self.value.as_ref() }
        }
    }
    pub struct UnsafeRefMut<T: ?Sized> {
        value: NonNull<T>,
    }

    impl<T: ?Sized> Deref for UnsafeRefMut<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            unsafe { self.value.as_ref() }
        }
    }

    impl<T: ?Sized> DerefMut for UnsafeRefMut<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            unsafe { self.value.as_mut() }
        }
    }

    #[derive(Debug)]
    pub struct UnsafeRefCell<T> {
        cell: UnsafeCell<T>,
    }

    impl<T> UnsafeRefCell<T> {
        pub fn new(value: T) -> UnsafeRefCell<T> {
            Self {
                cell: UnsafeCell::new(value),
            }
        }
        pub fn borrow(&self) -> UnsafeRef<T> {
            let value = unsafe { NonNull::new_unchecked(self.cell.get()) };
            UnsafeRef { value }
        }
        pub fn borrow_mut(&self) -> UnsafeRefMut<T> {
            let value = unsafe { NonNull::new_unchecked(self.cell.get()) };
            UnsafeRefMut { value }
        }
        pub fn replace(&self, t: T) -> T {
            std::mem::replace(&mut *self.borrow_mut(), t)
        }
    }
}
pub mod value {
    use std::rc::Rc;

    #[cfg(not(feature = "unsafecell"))]
    type RefCell<T> = std::cell::RefCell<T>;
    #[cfg(feature = "unsafecell")]
    type RefCell<T> = crate::cell::UnsafeRefCell<T>;

    #[derive(Debug, Clone)]
    pub enum Value {
        List(Rc<RefCell<Vec<Ref>>>),
        Ref(Ref),
        Primitive(Primitive),
    }

    #[derive(Debug, Clone)]
    pub struct Ref(pub Rc<RefCell<Value>>);
    impl Ref {
        pub fn new(value: Value) -> Self {
            Self(Rc::new(RefCell::new(value)))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Primitive {
        String(Rc<String>),
        Number(Number),
        Boolean(bool),
        None,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Number {
        Int64(i64),
        Float(f64),
    }

    impl PartialOrd for Number {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Number::Int64(l0), Number::Int64(r0)) => l0.partial_cmp(r0),
                (Number::Float(l0), Number::Float(r0)) => l0.partial_cmp(r0),
                (Number::Int64(l0), Number::Float(r0)) => (*l0 as f64).partial_cmp(r0),
                (Number::Float(l0), Number::Int64(r0)) => l0.partial_cmp(&(*r0 as f64)),
            }
        }
    }
    impl PartialEq for Number {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Number::Int64(l0), Number::Int64(r0)) => l0.eq(r0),
                (Number::Float(l0), Number::Float(r0)) => l0.eq(r0),
                (Number::Int64(l0), Number::Float(r0)) => *l0 as f64 == *r0,
                (Number::Float(l0), Number::Int64(r0)) => *l0 == *r0 as f64,
            }
        }
    }

    impl Into<Value> for Primitive {
        fn into(self) -> Value {
            Value::Primitive(self)
        }
    }

    macro_rules! impl_value_compare {
        ($name:ident, $op:ident) => {
            pub fn $name(&self, value: &Value) -> Value {
                match (self.__primitive(), value.__primitive()) {
                    (Primitive::Number(lhs), Primitive::Number(rhs)) => {
                        Value::Primitive(Primitive::Boolean(lhs.$op(&rhs)))
                    }
                    _ => todo!(),
                }
            }
        };
    }

    macro_rules! impl_value_binop {
        ($name:ident, $op:ident) => {
            pub fn $name(&self, rhs: &Value) -> Value {
                use std::ops::*;
                match (self.__primitive(), rhs.__primitive()) {
                    (Primitive::Number(lhs), Primitive::Number(rhs)) => match (lhs, rhs) {
                        (Number::Int64(l0), Number::Int64(r0)) => {
                            Value::Primitive(Primitive::Number(Number::Int64(l0.$op(r0))))
                        }
                        (Number::Float(l0), Number::Float(r0)) => {
                            Value::Primitive(Primitive::Number(Number::Float(l0.$op(r0))))
                        }
                        (Number::Int64(l0), Number::Float(r0)) => {
                            Value::Primitive(Primitive::Number(Number::Float((l0 as f64).$op(r0))))
                        }
                        (Number::Float(l0), Number::Int64(r0)) => {
                            Value::Primitive(Primitive::Number(Number::Float(l0.$op(&(r0 as f64)))))
                        }
                    },
                    _ => todo!("{:?} {:?}", self, rhs),
                }
            }
        };
    }

    impl Value {
        pub fn none() -> Self {
            Self::Primitive(Primitive::None)
        }

        pub fn list(list: Vec<Ref>) -> Self {
            Value::List(Rc::new(RefCell::new(list)))
        }

        pub fn __primitive(&self) -> Primitive {
            match self {
                Value::Ref(r) => r.0.borrow().__primitive(),
                Value::Primitive(p) => p.clone(),
                _ => unreachable!(),
            }
        }
        pub fn __shallow_copy(&self) -> Value {
            self.clone()
        }
        pub fn __inner(&self) -> Value {
            match self {
                Value::Ref(r) => r.0.borrow().__inner(),
                _ => self.clone(),
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
                        Primitive::Number(Number::Int64(i)) => i as usize,
                        _ => unreachable!(),
                    };
                    let r = list.borrow()[i].clone();
                    Value::Ref(r)
                }
                _ => unreachable!(),
            }
        }

        pub fn assign(&mut self, value: &Value) {
            let value = value.__inner();
            match self {
                Value::Ref(r) => {
                    r.0.replace(value);
                }
                _ => {
                    *self = value;
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
            Value::Primitive(Primitive::Number(Number::Int64(result)))
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
                (Primitive::Number(Number::Int64(lhs)), Primitive::Number(Number::Int64(rhs))) => {
                    Value::Primitive(Primitive::Number(Number::Int64(lhs / rhs)))
                }
                _ => todo!(),
            }
        }
        pub fn __div(&self, rhs: &Value) -> Value {
            match (self.__primitive(), rhs.__primitive()) {
                (Primitive::Number(Number::Int64(lhs)), Primitive::Number(Number::Int64(rhs))) => {
                    if lhs % rhs == 0 {
                        Value::Primitive(Primitive::Number(Number::Int64(lhs / rhs)))
                    } else {
                        Value::Primitive(Primitive::Number(Number::Float(lhs as f64 / rhs as f64)))
                    }
                }
                (Primitive::Number(Number::Int64(lhs)), Primitive::Number(Number::Float(rhs))) => {
                    Value::Primitive(Primitive::Number(Number::Float(lhs as f64 / rhs)))
                }
                (Primitive::Number(Number::Float(lhs)), Primitive::Number(Number::Int64(rhs))) => {
                    Value::Primitive(Primitive::Number(Number::Float(lhs / rhs as f64)))
                }
                _ => todo!(),
            }
        }

        pub fn __unary_add(&self) -> Value {
            self.clone()
        }
        pub fn __unary_sub(&self) -> Value {
            match self.__primitive() {
                Primitive::Number(Number::Int64(i)) => Primitive::Number(Number::Int64(-i)).into(),
                Primitive::Number(Number::Float(f)) => Primitive::Number(Number::Float(-f)).into(),
                _ => todo!(),
            }
        }
        pub fn __len(&self) -> Value {
            match self {
                Value::List(list) => {
                    Primitive::Number(Number::Int64(list.borrow().len() as i64)).into()
                }
                _ => todo!(),
            }
        }
        pub fn sort(&self) {
            match self {
                Value::List(list) => {
                    list.borrow_mut().sort_by(|a, b| {
                        let a = a.0.borrow().__primitive();
                        let b = b.0.borrow().__primitive();
                        match (a, b) {
                            (Primitive::Number(a), Primitive::Number(b)) => {
                                a.partial_cmp(&b).unwrap()
                            }
                            _ => todo!(),
                        }
                    });
                }
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
            Primitive::Number(Number::Int64(v)).into()
        }
    }
    impl From<f64> for Value {
        fn from(v: f64) -> Self {
            Primitive::Number(Number::Float(v)).into()
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
            match self {
                Value::List(list) => {
                    format!(
                        "[{}]",
                        list.borrow()
                            .iter()
                            .map(|v| v.0.borrow().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
                Value::Ref(r) => r.0.borrow().to_string(),
                Value::Primitive(value) => match value {
                    Primitive::String(s) => s.to_string(),
                    Primitive::Number(Number::Int64(i)) => i.to_string(),
                    Primitive::Number(Number::Float(f)) => f.to_string(),
                    _ => todo!(),
                },
            }
        }
    }
}

pub mod builtin {
    use std::io::stdin;

    use crate::{value::Value, Number, Primitive, Ref};

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
                    .collect::<Vec<_>>();
                Value::list(list)
            }
            Value::Ref(r) => map_int(&r.0.borrow()),
            _ => todo!(),
        }
    }
    pub fn int(value: &Value) -> Value {
        match value.__primitive() {
            Primitive::String(s) => {
                if let Ok(i) = s.parse::<i64>() {
                    Primitive::Number(Number::Int64(i)).into()
                } else {
                    todo!()
                }
            }
            Primitive::Number(Number::Int64(_)) => value.clone(),
            _ => panic!("invalid"),
        }
    }

    pub fn list(value: &Value) -> Value {
        match value {
            Value::List(list) => {
                let list = list.borrow().clone();
                Value::list(list)
            }
            _ => todo!(),
        }
    }

    pub fn range1(value: &Value) -> Value {
        match value.__primitive() {
            Primitive::Number(Number::Int64(i)) => {
                let list = (0..i)
                    .map(|i| Ref::new(Primitive::Number(Number::Int64(i)).into()))
                    .collect::<Vec<_>>();
                Value::list(list)
            }
            _ => todo!(),
        }
    }

    pub fn sorted(value: &Value) -> Value {
        value.sort();
        value.clone()
    }

    pub fn len(value: &Value) -> Value {
        value.__len()
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
