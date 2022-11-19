
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
impl_map1!(__pow);
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
