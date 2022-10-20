use std::{io::stdin, rc::Rc};

use crate::Value;

pub fn input() -> Value {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    Value::String {
        inner: Rc::new(buf.trim().to_string()),
    }
}
