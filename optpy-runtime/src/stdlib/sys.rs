use crate::Value;

/// It does nothing, just for pass the compile.
#[allow(non_snake_case)]
pub fn __sys__setrecursionlimit(_: &Value) -> Value {
    Value::None
}
