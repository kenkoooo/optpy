use crate::number::Number;

pub trait TypedValue: Sized {
    fn __shallow_copy(&self) -> Self;
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

pub trait IndexValue {
    fn __as_number(&self) -> Number;
}
