use std::rc::Rc;

use crate::Value;

pub fn map<F>(f: F, list: Value) -> Value
where
    F: Fn(Value) -> Value,
{
    match list {
        Value::List { inner } => {
            let result = inner.iter().map(|v| f(v.clone())).collect::<Vec<_>>();
            Value::List {
                inner: Rc::new(result),
            }
        }
        _ => panic!(),
    }
}
