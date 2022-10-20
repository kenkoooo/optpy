use std::io::{stdout, Write};

use crate::Value;

pub fn print(value: Value) {
    stdout().write_all(value.to_string().as_bytes()).unwrap();
    stdout().write_all("\n".as_bytes()).unwrap();
}
