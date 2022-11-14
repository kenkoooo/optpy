pub mod cell {
    use std::{
        cell::UnsafeCell,
        fmt::Debug,
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

    impl<T: ?Sized + PartialEq> PartialEq<T> for UnsafeRef<T> {
        fn eq(&self, other: &T) -> bool {
            self.deref() == other
        }
    }
    impl<T: ?Sized + PartialEq> PartialEq<T> for UnsafeRefMut<T> {
        fn eq(&self, other: &T) -> bool {
            self.deref() == other
        }
    }
    impl<T: Debug> Debug for UnsafeRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.deref().fmt(f)
        }
    }
    impl<T: Debug> Debug for UnsafeRefMut<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.deref().fmt(f)
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
pub mod number {
    use std::ops::{Add, Div, Mul, Rem, Sub};

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

    impl Number {
        pub fn floor_div(&self, rhs: &Number) -> Number {
            match (self, rhs) {
                (Number::Int64(l0), Number::Int64(r0)) => Number::Int64(l0 / r0),
                _ => todo!(),
            }
        }
    }
    impl ToString for Number {
        fn to_string(&self) -> String {
            match self {
                Number::Int64(i) => i.to_string(),
                Number::Float(f) => f.to_string(),
            }
        }
    }

    macro_rules! impl_binop {
        ($t:tt, $name:ident) => {
            impl $t for Number {
                type Output = Number;

                fn $name(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        (Number::Int64(lhs), Number::Int64(rhs)) => Number::Int64(lhs.$name(rhs)),
                        (Number::Int64(lhs), Number::Float(rhs)) => {
                            Number::Float((lhs as f64).$name(rhs))
                        }
                        (Number::Float(lhs), Number::Int64(rhs)) => {
                            Number::Float(lhs.$name(rhs as f64))
                        }
                        (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs.$name(rhs)),
                    }
                }
            }
        };
    }
    impl_binop!(Add, add);
    impl_binop!(Mul, mul);
    impl_binop!(Sub, sub);
    impl_binop!(Rem, rem);
    impl Div for Number {
        type Output = Number;

        fn div(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Number::Int64(lhs), Number::Int64(rhs)) => Number::Float(lhs as f64 / rhs as f64),
                (Number::Int64(lhs), Number::Float(rhs)) => Number::Float(lhs as f64 / rhs),
                (Number::Float(lhs), Number::Int64(rhs)) => Number::Float(lhs / rhs as f64),
                (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs / rhs),
            }
        }
    }
}
pub mod value {
    use std::{ops::Mul, rc::Rc};

    use crate::{cell::UnsafeRefMut, number::Number};

    type RefCell<T> = crate::cell::UnsafeRefCell<T>;

    #[derive(Debug, Clone)]
    pub enum Value {
        List(Rc<RefCell<Vec<Rc<RefCell<Value>>>>>),
        String(Rc<String>),
        Number(Number),
        Boolean(bool),
        None,
    }

    impl PartialOrd for Value {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Value::Number(lhs), Value::Number(rhs)) => lhs.partial_cmp(rhs),
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
                (Self::List(l0), Self::List(r0)) => l0
                    .borrow()
                    .iter()
                    .zip(r0.borrow().iter())
                    .all(|(l, r)| l.borrow().eq(&r.borrow())),
                _ => false,
            }
        }
    }

    macro_rules! impl_binop {
        ($name:ident, $op:ident) => {
            impl Value {
                pub fn $name(&self, rhs: &Value) -> Value {
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
    impl_binop!(__mul, mul);
    impl_binop!(__rem, rem);
    impl_binop!(__div, div);

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
        pub fn none() -> Value {
            Value::None
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
                    Value::List(Rc::new(RefCell::new(list)))
                }
                _ => unreachable!(),
            }
        }

        pub fn index(&self, index: &Value) -> UnsafeRefMut<Value> {
            match (self, index) {
                (Value::List(list), Value::Number(Number::Int64(i))) => {
                    list.borrow_mut()[*i as usize].borrow_mut()
                }
                _ => todo!(),
            }
        }

        pub fn assign(&mut self, value: &Value) {
            *self = value.clone();
        }

        pub fn reverse(&mut self) {
            match self {
                Value::List(list) => {
                    list.borrow_mut().reverse();
                }
                _ => unreachable!(),
            }
        }

        pub fn pop(&mut self) -> Value {
            match self {
                Value::List(list) => {
                    let last = list.borrow_mut().pop().expect("empty list");
                    last.borrow().clone()
                }
                _ => unreachable!(),
            }
        }
        pub fn append(&mut self, value: &Value) {
            match self {
                Value::List(list) => {
                    list.borrow_mut().push(Rc::new(RefCell::new(value.clone())));
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
        pub fn __len(&self) -> Value {
            match self {
                Value::List(list) => Value::Number(Number::Int64(list.borrow().len() as i64)),
                Value::String(s) => Value::Number(Number::Int64(s.len() as i64)),
                _ => unreachable!(),
            }
        }
        pub fn sort(&self) {
            match self {
                Value::List(list) => list.borrow_mut().sort_by(|a, b| {
                    let a = a.borrow();
                    let b = b.borrow();
                    a.partial_cmp(&b).unwrap()
                }),
                _ => unreachable!(),
            }
        }

        pub fn test(&self) -> bool {
            match self {
                Value::Boolean(b) => *b,
                _ => unreachable!(),
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
    impl<T> From<Vec<T>> for Value
    where
        Value: From<T>,
    {
        fn from(list: Vec<T>) -> Self {
            let list = list
                .into_iter()
                .map(|t| Rc::new(RefCell::new(Value::from(t))))
                .collect();
            Value::List(Rc::new(RefCell::new(list)))
        }
    }
    impl From<UnsafeRefMut<Value>> for Value {
        fn from(r: UnsafeRefMut<Value>) -> Self {
            r.clone()
        }
    }
    impl From<bool> for Value {
        fn from(b: bool) -> Self {
            Value::Boolean(b)
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
}

pub mod builtin {
    use std::io::stdin;

    use crate::{number::Number, value::Value};

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
                    .map(|v| int(&v.borrow()))
                    .collect::<Vec<_>>();
                Value::from(list)
            }
            _ => unreachable!(),
        }
    }
    pub fn int(value: &Value) -> Value {
        match value {
            Value::String(s) => {
                Value::Number(Number::Int64(s.parse::<i64>().expect("non-integer")))
            }
            Value::Number(Number::Int64(i)) => Value::Number(Number::Int64(*i)),
            _ => unreachable!(),
        }
    }

    pub fn list(value: &Value) -> Value {
        match value {
            Value::List(_) => value.clone(),
            _ => todo!(),
        }
    }

    pub fn range1(value: &Value) -> Value {
        match value {
            Value::Number(Number::Int64(i)) => {
                let list = (0..*i)
                    .map(|i| Value::Number(Number::Int64(i)))
                    .collect::<Vec<_>>();
                Value::from(list)
            }
            _ => unreachable!(),
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
