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

    pub struct UnsafeRefCell<T> {
        cell: UnsafeCell<T>,
    }
    impl<T: Debug> Debug for UnsafeRefCell<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.borrow().fmt(f)
        }
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
    use std::{
        hash::Hash,
        ops::{Add, Div, Mul, Rem, Sub},
    };

    #[derive(Debug, Clone, Copy)]
    pub enum Number {
        Int64(i64),
        Float(f64),
    }
    impl Hash for Number {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match self {
                Number::Int64(i) => i.hash(state),
                Number::Float(_) => todo!(),
            }
        }
    }
    impl Eq for Number {}

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

pub mod dict {

    use crate::number::Number;

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    pub enum DictKey {
        Number(Number),
        String(String),
    }
}
mod object {
    use std::{fmt::Debug, rc::Rc};

    use crate::{cell::UnsafeRefCell, number::Number, value::Value};

    pub enum Object {
        Ref(Rc<UnsafeRefCell<Value>>),
        Value(Value),
    }

    impl PartialEq for Object {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Object::Ref(l), Object::Ref(r)) => l.borrow().eq(&r.borrow()),
                (Object::Ref(l), Object::Value(r)) => l.borrow().eq(&r),
                (Object::Value(l), Object::Ref(r)) => l.eq(&r.borrow()),
                (Object::Value(l), Object::Value(r)) => l.eq(&r),
            }
        }
    }
    impl Eq for Object {}
    impl Debug for Object {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Object::Ref(r) => r.borrow().fmt(f),
                Object::Value(v) => v.fmt(f),
            }
        }
    }

    impl Object {
        pub fn none() -> Object {
            Object::Value(Value::none())
        }
        pub fn dict(pairs: Vec<(Object, Object)>) -> Object {
            let pairs = pairs
                .into_iter()
                .map(|(key, value)| {
                    fn inner(obj: Object) -> Value {
                        match obj {
                            Object::Ref(r) => r.borrow().clone(),
                            Object::Value(v) => v,
                        }
                    }
                    let key = inner(key);
                    let value = inner(value);
                    (key, value)
                })
                .collect();
            Object::Value(Value::dict(pairs))
        }

        pub fn assign(&mut self, value: &Object) {
            match (self, value) {
                (Object::Ref(l), Object::Ref(r)) => l.borrow_mut().assign(&r.borrow()),
                (Object::Ref(l), Object::Value(r)) => l.borrow_mut().assign(r),
                (Object::Value(l), Object::Ref(r)) => l.assign(&r.borrow()),
                (Object::Value(l), Object::Value(r)) => l.assign(r),
            }
        }

        pub fn index(&self, index: &Object) -> Object {
            let r = match (self, index) {
                (Object::Ref(l), Object::Ref(r)) => l.borrow().index(&r.borrow()),
                (Object::Ref(l), Object::Value(r)) => l.borrow().index(&r),
                (Object::Value(l), Object::Ref(r)) => l.index(&r.borrow()),
                (Object::Value(l), Object::Value(r)) => l.index(&r),
            };
            Object::Ref(r)
        }
        pub fn append(&self, object: &Object) {
            match (self, object) {
                (Object::Ref(l), Object::Ref(r)) => l.borrow().append(&r.borrow()),
                (Object::Ref(l), Object::Value(r)) => l.borrow().append(&r),
                (Object::Value(l), Object::Ref(r)) => l.append(&r.borrow()),
                (Object::Value(l), Object::Value(r)) => l.append(&r),
            }
        }
        pub fn test(&self) -> bool {
            match self {
                Object::Ref(r) => r.borrow().test(),
                Object::Value(v) => v.test(),
            }
        }

        pub fn __number(&self) -> Number {
            match self {
                Object::Ref(r) => r.borrow().__number(),
                Object::Value(v) => v.__number(),
            }
        }
    }

    fn method<F: Fn(&Value)>(obj: &Object, f: F) {
        match obj {
            Object::Ref(r) => f(&r.borrow()),
            Object::Value(v) => f(v),
        }
    }
    macro_rules! impl_method {
        ($name:ident) => {
            impl Object {
                pub fn $name(&self) {
                    method(self, Value::$name);
                }
            }
        };
    }
    impl_method!(sort);
    impl_method!(reverse);

    fn map0<F: Fn(&Value) -> Value>(obj: &Object, f: F) -> Object {
        let value = match obj {
            Object::Ref(r) => f(&r.borrow()),
            Object::Value(v) => f(v),
        };
        Object::Value(value)
    }

    macro_rules! impl_map0 {
        ($name:ident) => {
            impl Object {
                pub fn $name(&self) -> Object {
                    map0(self, Value::$name)
                }
            }
        };
    }
    impl_map0!(__shallow_copy);
    impl_map0!(split);
    impl_map0!(pop);
    impl_map0!(__unary_add);
    impl_map0!(__unary_sub);
    impl_map0!(__len);

    fn map1<F: Fn(&Value, &Value) -> Value>(obj1: &Object, obj2: &Object, f: F) -> Object {
        let value = match (obj1, obj2) {
            (Object::Ref(l), Object::Ref(r)) => f(&l.borrow(), &r.borrow()),
            (Object::Ref(l), Object::Value(r)) => f(&l.borrow(), &r),
            (Object::Value(l), Object::Ref(r)) => f(&l, &r.borrow()),
            (Object::Value(l), Object::Value(r)) => f(&l, &r),
        };
        Object::Value(value)
    }

    macro_rules! impl_map1 {
        ($name:ident) => {
            impl Object {
                pub fn $name(&self, value: &Object) -> Object {
                    map1(self, value, Value::$name)
                }
            }
        };
    }

    impl_map1!(__floor_div);
    impl_map1!(__add);
    impl_map1!(__sub);
    impl_map1!(__mul);
    impl_map1!(__rem);
    impl_map1!(__div);
    impl_map1!(__gt);
    impl_map1!(__ge);
    impl_map1!(__lt);
    impl_map1!(__le);
    impl_map1!(__eq);
    impl_map1!(__ne);
    impl_map1!(__in);
    impl_map1!(__not_in);

    macro_rules! impl_from {
        ($t:ty) => {
            impl From<$t> for Object {
                fn from(v: $t) -> Self {
                    Object::Value(Value::from(v))
                }
            }
        };
    }
    impl_from!(&str);
    impl_from!(i64);
    impl_from!(f64);
    impl_from!(bool);

    impl From<&Object> for Object {
        fn from(obj: &Object) -> Self {
            obj.__shallow_copy()
        }
    }
    impl From<Vec<Object>> for Object {
        fn from(list: Vec<Object>) -> Self {
            let list = list
                .into_iter()
                .map(|obj| {
                    Rc::new(UnsafeRefCell::new(match obj {
                        Object::Ref(r) => r.borrow().clone(),
                        Object::Value(v) => v,
                    }))
                })
                .collect::<Vec<_>>();
            Object::Value(Value::List(Rc::new(UnsafeRefCell::new(list))))
        }
    }

    impl ToString for Object {
        fn to_string(&self) -> String {
            match self {
                Object::Ref(r) => r.borrow().to_string(),
                Object::Value(v) => v.to_string(),
            }
        }
    }
}

pub mod value {
    use std::{collections::HashMap, ops::Mul, rc::Rc};

    use crate::{cell::UnsafeRefMut, dict::DictKey, number::Number};

    type RefCell<T> = crate::cell::UnsafeRefCell<T>;

    #[derive(Debug, Clone)]
    pub enum Value {
        List(Rc<RefCell<Vec<Rc<RefCell<Value>>>>>),
        String(Rc<String>),
        Number(Number),
        Boolean(bool),
        Dict(Rc<RefCell<HashMap<DictKey, Rc<RefCell<Value>>>>>),
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
        fn includes(&self, value: &Value) -> bool {
            match self {
                Value::List(list) => list.borrow().iter().any(|e| e.borrow().eq(value)),
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
    }

    impl Value {
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
                    Value::List(Rc::new(RefCell::new(list)))
                }
                _ => unreachable!(),
            }
        }

        pub fn index(&self, index: &Value) -> Rc<RefCell<Value>> {
            match (self, index) {
                (Value::List(list), Value::Number(Number::Int64(i))) => {
                    list.borrow_mut()[*i as usize].clone()
                }
                (Value::Dict(dict), _) => {
                    let key = index.__as_dict_key();
                    dict.borrow_mut()
                        .entry(key)
                        .or_insert_with(|| Rc::new(RefCell::new(Value::none())))
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

        pub fn assign(&mut self, value: &Value) {
            *self = value.clone();
        }

        pub fn reverse(&self) {
            match self {
                Value::List(list) => {
                    list.borrow_mut().reverse();
                }
                _ => unreachable!(),
            }
        }

        pub fn pop(&self) -> Value {
            match self {
                Value::List(list) => {
                    let last = list.borrow_mut().pop().expect("empty list");
                    last.borrow().clone()
                }
                _ => unreachable!(),
            }
        }
        pub fn append(&self, value: &Value) {
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
                Value::String(s) => Value::Number(Number::Int64(s.chars().count() as i64)),
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

        pub fn __number(&self) -> Number {
            match self {
                Value::Number(n) => *n,
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
    impl From<Vec<Value>> for Value {
        fn from(list: Vec<Value>) -> Self {
            let list = list.into_iter().map(|v| Rc::new(RefCell::new(v))).collect();
            Value::List(Rc::new(RefCell::new(list)))
        }
    }
    impl From<&Value> for Value {
        fn from(r: &Value) -> Self {
            r.__shallow_copy()
        }
    }
    impl From<UnsafeRefMut<Value>> for Value {
        fn from(r: UnsafeRefMut<Value>) -> Self {
            r.__shallow_copy()
        }
    }
    impl From<&UnsafeRefMut<Value>> for Value {
        fn from(r: &UnsafeRefMut<Value>) -> Self {
            r.__shallow_copy()
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

    use crate::{number::Number, value::Value, Object};

    mod value {
        use std::io::stdin;

        use crate::{number::Number, value::Value};

        pub(super) fn input() -> Value {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            Value::from(buf.trim())
        }

        pub(super) fn map_int(value: &Value) -> Value {
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
        pub(super) fn int(value: &Value) -> Value {
            match value {
                Value::String(s) => {
                    Value::Number(Number::Int64(s.parse::<i64>().expect("non-integer")))
                }
                Value::Number(Number::Int64(i)) => Value::Number(Number::Int64(*i)),
                _ => unreachable!(),
            }
        }

        pub(super) fn list(value: &Value) -> Value {
            match value {
                Value::List(_) => value.clone(),
                _ => todo!(),
            }
        }

        pub(super) fn range1(value: &Value) -> Value {
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

        pub(super) fn sorted(value: &Value) -> Value {
            value.sort();
            value.clone()
        }

        pub(super) fn len(value: &Value) -> Value {
            value.__len()
        }
    }

    fn map0<F: Fn(&Value) -> Value>(obj: &Object, f: F) -> Object {
        let value = match obj {
            Object::Ref(r) => f(&r.borrow()),
            Object::Value(v) => f(v),
        };
        Object::Value(value)
    }

    macro_rules! define_map0 {
        ($name:ident) => {
            pub fn $name(obj: &Object) -> Object {
                map0(obj, value::$name)
            }
        };
    }
    define_map0!(len);
    define_map0!(sorted);
    define_map0!(range1);
    define_map0!(list);
    define_map0!(int);
    define_map0!(map_int);

    pub fn __pow3(number: &Object, power: &Object, modulus: &Object) -> Object {
        let number = number.__number();
        let power = power.__number();
        let modulus = modulus.__number();
        let int = |n: Number| match n {
            Number::Int64(i) => i,
            Number::Float(_) => unreachable!(),
        };

        let number = int(number);
        let power = int(power);
        let modulus = int(modulus);

        let mut result = 1;
        let mut cur = number;
        let mut e = power;
        while e > 0 {
            if e & 1 == 1 {
                result = (result * cur) % modulus;
            }
            cur = (cur * cur) % modulus;
            e >>= 1;
        }
        Object::Value(Value::Number(Number::Int64(result)))
    }

    pub fn input() -> Object {
        Object::Value(value::input())
    }
}

pub use builtin::*;
pub use object::*;

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

#[macro_export]
macro_rules! pow {
    ($number:expr, $power:expr, $modulus:expr) => {
        __pow3($number, $power, $modulus)
    };
}
