use std::rc::Rc;

use crate::{
    cell::{UnsafeRefCell, UnsafeRefMut},
    number::Number,
    DictKey, Value,
};

#[derive(Debug, Clone)]
pub struct List(pub Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<Value>>>>>);

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().eq(&other.0.borrow())
    }
}
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().partial_cmp(&other.0.borrow())
    }
}

impl List {
    pub fn __mul(&self, rhs: Number) -> Value {
        match rhs {
            Number::Int64(n) => {
                let mut result = vec![];
                for _ in 0..n {
                    for element in self.0.borrow().iter() {
                        result.push(UnsafeRefCell::rc(element.borrow().clone()));
                    }
                }
                Value::List(List(UnsafeRefCell::rc(result)))
            }
            _ => todo!(),
        }
    }
    pub fn includes<'a, T>(&self, value: &'a T) -> bool
    where
        Value: From<&'a T>,
    {
        self.0
            .borrow()
            .iter()
            .any(|e| e.borrow().eq(&Value::from(value)))
    }
    pub fn __delete<'a, T>(&self, index: &'a T)
    where
        Number: From<&'a T>,
    {
        match Number::from(index) {
            Number::Int64(i) => {
                if i < 0 {
                    let i = self.0.borrow().len() as i64 + i;
                    self.0.borrow_mut().remove(i as usize);
                } else {
                    self.0.borrow_mut().remove(i as usize);
                }
            }
            _ => todo!(),
        }
    }
    pub fn __index_ref<'a, I>(&self, index: &'a I) -> UnsafeRefMut<Value>
    where
        DictKey: From<&'a I>,
    {
        match DictKey::from(index) {
            DictKey::Number(Number::Int64(i)) => {
                if i < 0 {
                    let i = self.0.borrow().len() as i64 + i;
                    self.0.borrow_mut()[i as usize].borrow_mut()
                } else {
                    self.0.borrow_mut()[i as usize].borrow_mut()
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn __index_value<'a, I>(&self, index: &'a I) -> Value
    where
        DictKey: From<&'a I>,
    {
        match DictKey::from(index) {
            DictKey::Number(Number::Int64(i)) => {
                if i < 0 {
                    let i = self.0.borrow().len() as i64 + i;
                    self.0.borrow()[i as usize].borrow().clone()
                } else {
                    self.0.borrow()[i as usize].borrow().clone()
                }
            }
            _ => todo!(),
        }
    }
    pub fn reverse(&self) {
        self.0.borrow_mut().reverse();
    }
    pub fn pop(&self) -> Value {
        let last = self.0.borrow_mut().pop().expect("empty list");
        last.borrow().clone()
    }
    pub fn append<'a, T>(&self, value: &'a T)
    where
        Value: From<&'a T>,
    {
        self.0
            .borrow_mut()
            .push(UnsafeRefCell::rc(Value::from(value)));
    }
    pub fn __len(&self) -> Value {
        Value::Number(Number::Int64(self.0.borrow().len() as i64))
    }
    pub fn sort(&self) {
        self.0.borrow_mut().sort_by(|a, b| {
            let a = a.borrow();
            let b = b.borrow();
            a.partial_cmp(&b).unwrap()
        })
    }
    pub fn count(&self, value: &Value) -> Value {
        let count = self
            .0
            .borrow()
            .iter()
            .filter(|v| v.borrow().eq(value))
            .count();
        Value::Number(Number::Int64(count as i64))
    }
    pub fn test(&self) -> bool {
        self.0.borrow().len() > 0
    }
}

impl ToString for List {
    fn to_string(&self) -> String {
        let mut result = String::from("[");
        for (i, v) in self.0.borrow().iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&v.borrow().to_string());
        }
        result.push_str("]");
        result
    }
}

impl From<Vec<Value>> for List {
    fn from(list: Vec<Value>) -> Self {
        let list = list.into_iter().map(|v| UnsafeRefCell::rc(v)).collect();
        Self(UnsafeRefCell::rc(list))
    }
}
