use std::rc::Rc;

use crate::{
    cell::{UnsafeRefCell, UnsafeRefMut},
    number::Number,
    Value,
};

#[derive(Debug, Clone)]
pub struct List(pub Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<Value>>>>>);

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .borrow()
            .iter()
            .zip(other.0.borrow().iter())
            .all(|(l, r)| l.borrow().eq(&r.borrow()))
    }
}

impl List {
    pub fn __mul(&self, rhs: &Value) -> Value {
        match rhs {
            Value::Number(Number::Int64(n)) => {
                let mut result = vec![];
                for _ in 0..(*n) {
                    for element in self.0.borrow().iter() {
                        result.push(Rc::new(UnsafeRefCell::new(element.borrow().clone())));
                    }
                }
                Value::List(List(Rc::new(UnsafeRefCell::new(result))))
            }
            _ => todo!(),
        }
    }
    pub fn includes(&self, value: &Value) -> bool {
        self.0.borrow().iter().any(|e| e.borrow().eq(value))
    }
    pub fn __delete(&self, index: &Value) {
        match index {
            Value::Number(Number::Int64(i)) => {
                if *i < 0 {
                    let i = self.0.borrow().len() as i64 + *i;
                    self.0.borrow_mut().remove(i as usize);
                } else {
                    self.0.borrow_mut().remove(*i as usize);
                }
            }
            _ => todo!(),
        }
    }
    pub fn __index_ref(&self, index: &Value) -> UnsafeRefMut<Value> {
        match index {
            Value::Number(Number::Int64(i)) => {
                if *i < 0 {
                    let i = self.0.borrow().len() as i64 + *i;
                    self.0.borrow_mut()[i as usize].borrow_mut()
                } else {
                    self.0.borrow_mut()[*i as usize].borrow_mut()
                }
            }
            _ => todo!(),
        }
    }
    pub fn __index_value(&self, index: &Value) -> Value {
        match index {
            Value::Number(Number::Int64(i)) => {
                if *i < 0 {
                    let i = self.0.borrow().len() as i64 + *i;
                    self.0.borrow()[i as usize].borrow().clone()
                } else {
                    self.0.borrow()[*i as usize].borrow().clone()
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
    pub fn append(&self, value: &Value) {
        self.0
            .borrow_mut()
            .push(Rc::new(UnsafeRefCell::new(value.clone())));
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
}

impl From<Vec<Value>> for List {
    fn from(list: Vec<Value>) -> Self {
        let list = list
            .into_iter()
            .map(|v| Rc::new(UnsafeRefCell::new(v)))
            .collect();
        Self(Rc::new(UnsafeRefCell::new(list)))
    }
}
