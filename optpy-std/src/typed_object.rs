use std::{collections::HashMap, fmt::Debug, hash::Hash, rc::Rc};

use crate::{cell::UnsafeRefCell, number::Number};

mod object {
    use std::{fmt::Debug, rc::Rc};

    use crate::{cell::UnsafeRefCell, number::Number};

    use super::{BooleanValue, ListValue, NumberValue, StringValue, Value};

    pub enum Object<V> {
        Ref(Rc<UnsafeRefCell<V>>),
        Value(V),
    }

    impl<V: PartialEq> PartialEq for Object<V> {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Object::Ref(l), Object::Ref(r)) => l.borrow().eq(&r.borrow()),
                (Object::Ref(l), Object::Value(r)) => l.borrow().eq(&r),
                (Object::Value(l), Object::Ref(r)) => l.eq(&r.borrow()),
                (Object::Value(l), Object::Value(r)) => l.eq(&r),
            }
        }
    }
    impl<V: Eq> Eq for Object<V> {}
    impl<V: Debug> Debug for Object<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Object::Ref(r) => r.borrow().fmt(f),
                Object::Value(v) => v.fmt(f),
            }
        }
    }

    impl<V: Clone> Clone for Object<V> {
        fn clone(&self) -> Self {
            match self {
                Self::Ref(r) => Self::Value(r.borrow().clone()),
                Self::Value(v) => Self::Value(v.clone()),
            }
        }
    }
    impl<V: PartialOrd> PartialOrd for Object<V> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Object::Ref(l), Object::Ref(r)) => l.borrow().partial_cmp(&r.borrow()),
                (Object::Ref(l), Object::Value(r)) => l.borrow().partial_cmp(&r),
                (Object::Value(l), Object::Ref(r)) => l.partial_cmp(&r.borrow()),
                (Object::Value(l), Object::Value(r)) => l.partial_cmp(&r),
            }
        }
    }
    impl<T: Default> Default for Object<T> {
        fn default() -> Self {
            Object::Value(T::default())
        }
    }

    macro_rules! impl_binop {
        ($name:ident) => {
            pub fn $name<T: Value>(&self, other: &Object<T>) -> Self {
                match (self, other) {
                    (Object::Ref(l), Object::Ref(r)) => {
                        Object::Value(l.borrow().$name(&*r.borrow()))
                    }
                    (Object::Ref(l), Object::Value(r)) => Object::Value(l.borrow().$name(r)),
                    (Object::Value(l), Object::Ref(r)) => Object::Value(l.$name(&*r.borrow())),
                    (Object::Value(l), Object::Value(r)) => Object::Value(l.$name(r)),
                }
            }
        };
    }

    impl<V: Value> Object<V> {
        pub fn __inner(&self) -> V {
            match self {
                Object::Ref(r) => r.borrow().clone(),
                Object::Value(v) => v.clone(),
            }
        }
        pub fn __shallow_copy(&self) -> Self {
            self.clone()
        }
        pub fn assign(&mut self, value: &Self) {
            match (self, value) {
                (Object::Ref(l), Object::Ref(r)) => l.borrow_mut().assign(&r.borrow()),
                (Object::Ref(l), Object::Value(r)) => l.borrow_mut().assign(r),
                (Object::Value(l), Object::Ref(r)) => l.assign(&r.borrow()),
                (Object::Value(l), Object::Value(r)) => l.assign(r),
            }
        }
        impl_binop!(__add);
        impl_binop!(__mul);
        impl_binop!(__rem);
        pub fn __eq<T: Value>(&self, other: &Object<T>) -> Object<BooleanValue> {
            match (self, other) {
                (Object::Ref(l), Object::Ref(r)) => {
                    Object::Value(BooleanValue(l.borrow().__eq(&*r.borrow())))
                }
                (Object::Ref(l), Object::Value(r)) => {
                    Object::Value(BooleanValue(l.borrow().__eq(r)))
                }
                (Object::Value(l), Object::Ref(r)) => {
                    Object::Value(BooleanValue(l.__eq(&*r.borrow())))
                }
                (Object::Value(l), Object::Value(r)) => Object::Value(BooleanValue(l.__eq(r))),
            }
        }

        pub fn index<T: Value>(&self, index: &Object<T>) -> Object<impl Value> {
            match (self, index) {
                (Object::Ref(value), Object::Ref(index)) => value.borrow().index(&*index.borrow()),
                (Object::Ref(value), Object::Value(index)) => todo!(),
                (Object::Value(value), Object::Ref(index)) => todo!(),
                (Object::Value(value), Object::Value(index)) => todo!(),
            }
        }

        pub fn test(&self) -> bool {
            match self {
                Object::Ref(r) => r.borrow().test(),
                Object::Value(v) => v.test(),
            }
        }
    }
    impl<T: Value> From<&Object<T>> for Object<T> {
        fn from(obj: &Object<T>) -> Self {
            Object::Value(obj.__inner())
        }
    }
    impl From<&str> for Object<StringValue> {
        fn from(s: &str) -> Self {
            Object::Value(StringValue(Rc::new(s.to_string())))
        }
    }
    impl From<i64> for Object<NumberValue> {
        fn from(v: i64) -> Self {
            Object::Value(NumberValue(Number::Int64(v)))
        }
    }
    impl From<f64> for Object<NumberValue> {
        fn from(v: f64) -> Self {
            Object::Value(NumberValue(Number::Float(v)))
        }
    }
    impl<T: Value> From<Vec<Object<T>>> for Object<ListValue<T>> {
        fn from(list: Vec<Object<T>>) -> Self {
            let list = list
                .into_iter()
                .map(|v| Rc::new(UnsafeRefCell::new(v.__inner())))
                .collect();
            Object::Value(ListValue(Rc::new(UnsafeRefCell::new(list))))
        }
    }
}
mod value {
    use std::{fmt::Debug, rc::Rc};

    use crate::number::Number;

    pub trait Value: Clone + Debug + Default {
        fn assign(&mut self, value: &Self) {
            *self = value.clone();
        }
        fn __number(&self) -> Number {
            todo!()
        }
        fn __string(&self) -> Rc<String> {
            todo!()
        }

        fn __add<T: Value>(&self, _: &T) -> Self {
            todo!()
        }
        fn __mul<T: Value>(&self, _: &T) -> Self {
            todo!()
        }
        fn __rem<T: Value>(&self, _: &T) -> Self {
            todo!()
        }
        fn __eq<T: Value>(&self, _: &T) -> bool {
            todo!()
        }
        fn test(&self) -> bool {
            todo!()
        }
        fn index<T: Value, S: Value>(&self, _: &T) -> S {
            todo!()
        }
    }
}
pub use object::*;
pub use value::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, PartialOrd)]
pub struct NumberValue(pub Number);
impl Value for NumberValue {
    fn __number(&self) -> Number {
        self.0.clone()
    }
    fn __add<T: Value>(&self, other: &T) -> Self {
        let x = self.0 + other.__number();
        Self(x)
    }

    fn __mul<T: Value>(&self, other: &T) -> Self {
        let x = self.0 * other.__number();
        Self(x)
    }
    fn __rem<T: Value>(&self, other: &T) -> Self {
        let x = self.0 % other.__number();
        Self(x)
    }
    fn __eq<T: Value>(&self, other: &T) -> bool {
        self.0 == other.__number()
    }
}

impl From<NumberValue> for i64 {
    fn from(n: NumberValue) -> Self {
        match n.0 {
            Number::Int64(i) => i,
            Number::Float(f) => f as i64,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ListValue<T>(pub Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<T>>>>>);

impl<T: Debug> Value for ListValue<T> {}
impl<T> Clone for ListValue<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<T> Default for ListValue<T> {
    fn default() -> Self {
        Self(Rc::new(UnsafeRefCell::new(vec![])))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct StringValue(pub Rc<String>);
impl Value for StringValue {
    fn __string(&self) -> Rc<String> {
        Rc::clone(&self.0)
    }
}

impl From<StringValue> for i64 {
    fn from(n: StringValue) -> Self {
        n.0.parse().expect("non-integer")
    }
}
impl Default for StringValue {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BooleanValue(pub bool);
impl Value for BooleanValue {
    fn test(&self) -> bool {
        self.0
    }
}
impl Default for BooleanValue {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(PartialEq, Debug)]
pub struct DictValue<K: Eq + Hash, V>(pub Rc<UnsafeRefCell<HashMap<K, Rc<UnsafeRefCell<V>>>>>);
impl<K: Eq + Hash + Debug, V: Debug> Value for DictValue<K, V> {}
impl<K: Eq + Hash, V> Clone for DictValue<K, V> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<K: Eq + Hash, V> Default for DictValue<K, V> {
    fn default() -> Self {
        Self(Rc::new(UnsafeRefCell::new(HashMap::new())))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NoneValue;
impl Value for NoneValue {}
impl Default for NoneValue {
    fn default() -> Self {
        Self
    }
}

pub trait AddValue<T> {
    fn __add(&self, rhs: &Object<T>) -> Self;
}

impl AddValue<NumberValue> for Object<NumberValue> {
    fn __add(&self, rhs: &Object<NumberValue>) -> Self {
        let sum = self.__inner().0 + rhs.__inner().0;
        Object::Value(NumberValue(sum))
    }
}
