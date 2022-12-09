use std::rc::Rc;

use crate::{
    cell::{UnsafeRefCell, UnsafeRefMut},
    number::Number,
    Value,
};

pub trait List<T> {
    fn __mul(&self, rhs: &T) -> T;
    fn includes(&self, value: &T) -> bool;
    fn __delete(&self, index: &T);
    fn __index_ref(&self, index: &T) -> UnsafeRefMut<T>;
    fn __index_value(&self, index: &T) -> T;
    fn reverse(&self);
    fn pop(&self) -> T;
    fn append(&self, value: &T);
    fn __len(&self) -> T;
    fn sort(&self);
    fn count(&self, value: &T) -> T;
    fn test(&self) -> bool;
    fn __to_string(&self) -> String;
}

impl List<Value> for Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<Value>>>>> {
    fn __mul(&self, rhs: &Value) -> Value {
        match rhs {
            Value::Number(Number::Int64(n)) => {
                let mut result = vec![];
                for _ in 0..(*n) {
                    for element in self.borrow().iter() {
                        result.push(UnsafeRefCell::rc(element.borrow().clone()));
                    }
                }
                Value::List(UnsafeRefCell::rc(result))
            }
            _ => todo!(),
        }
    }
    fn includes(&self, value: &Value) -> bool {
        self.borrow().iter().any(|e| e.borrow().eq(value))
    }
    fn __delete(&self, index: &Value) {
        match index {
            Value::Number(Number::Int64(i)) => {
                if *i < 0 {
                    let i = self.borrow().len() as i64 + *i;
                    self.borrow_mut().remove(i as usize);
                } else {
                    self.borrow_mut().remove(*i as usize);
                }
            }
            _ => todo!(),
        }
    }
    fn __index_ref(&self, index: &Value) -> UnsafeRefMut<Value> {
        match index {
            Value::Number(Number::Int64(i)) => {
                if *i < 0 {
                    let i = self.borrow().len() as i64 + *i;
                    self.borrow_mut()[i as usize].borrow_mut()
                } else {
                    self.borrow_mut()[*i as usize].borrow_mut()
                }
            }
            _ => todo!(),
        }
    }
    fn __index_value(&self, index: &Value) -> Value {
        match index {
            Value::Number(Number::Int64(i)) => {
                if *i < 0 {
                    let i = self.borrow().len() as i64 + *i;
                    self.borrow()[i as usize].borrow().clone()
                } else {
                    self.borrow()[*i as usize].borrow().clone()
                }
            }
            _ => todo!(),
        }
    }
    fn reverse(&self) {
        self.borrow_mut().reverse();
    }
    fn pop(&self) -> Value {
        let last = self.borrow_mut().pop().expect("empty list");
        last.borrow().clone()
    }
    fn append(&self, value: &Value) {
        self.borrow_mut().push(UnsafeRefCell::rc(value.clone()));
    }
    fn __len(&self) -> Value {
        Value::Number(Number::Int64(self.borrow().len() as i64))
    }
    fn sort(&self) {
        self.borrow_mut().sort_by(|a, b| {
            let a = a.borrow();
            let b = b.borrow();
            a.partial_cmp(&b).unwrap()
        })
    }
    fn count(&self, value: &Value) -> Value {
        let count = self
            .borrow()
            .iter()
            .filter(|v| v.borrow().eq(value))
            .count();
        Value::Number(Number::Int64(count as i64))
    }
    fn test(&self) -> bool {
        self.borrow().len() > 0
    }
    fn __to_string(&self) -> String {
        let mut result = String::from("[");
        for (i, v) in self.borrow().iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&v.borrow().to_string());
        }
        result.push_str("]");
        result
    }
}
