use std::io::stdin;

use crate::value::Value;

pub fn input() -> Value {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    Value::String { inner: buf }
}
