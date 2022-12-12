use crate::{number::Number, Value};

#[allow(non_snake_case)]
pub fn __math__gcd(a: Value, b: Value) -> Value {
    let a = a.__number();
    let b = b.__number();

    fn gcd(a: Number, b: Number) -> Number {
        if b == Number::Int64(0) {
            a
        } else {
            gcd(b, a % b)
        }
    }
    Value::Number(gcd(a, b))
}
