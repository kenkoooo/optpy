use std::fmt::Debug;

use crate::number::Number;

pub trait OptpyValue: ToString + Debug {
    fn assign<T: OptpyValue>(&mut self);
    fn index<T: OptpyValue>(&self, index: &T) -> Self;
    fn test(&self) -> bool;
    fn __number(&self) -> Number;

    fn sort(&self);
    fn reverse(&self);

    fn __shallow_copy(&self) -> Self;
    fn split(&self) -> Self;
    fn pop(&self) -> Self;
    fn strip(&self) -> Self;
    fn __unary_add(&self) -> Self;
    fn __unary_sub(&self) -> Self;
    fn __unary_not(&self) -> Self;
    fn __len(&self) -> Self;

    fn __floor_div<T: OptpyValue>(&self, other: &T) -> Self;
    fn count<T: OptpyValue>(&self, other: &T) -> Self;
    fn __add<T: OptpyValue>(&self, other: &T) -> Self;
    fn __sub<T: OptpyValue>(&self, other: &T) -> Self;
    fn __mul<T: OptpyValue>(&self, other: &T) -> Self;
    fn __rem<T: OptpyValue>(&self, other: &T) -> Self;
    fn __div<T: OptpyValue>(&self, other: &T) -> Self;
    fn __pow<T: OptpyValue>(&self, other: &T) -> Self;
    fn __gt<T: OptpyValue>(&self, other: &T) -> Self;
    fn __ge<T: OptpyValue>(&self, other: &T) -> Self;
    fn __lt<T: OptpyValue>(&self, other: &T) -> Self;
    fn __le<T: OptpyValue>(&self, other: &T) -> Self;
    fn __eq<T: OptpyValue>(&self, other: &T) -> Self;
    fn __ne<T: OptpyValue>(&self, other: &T) -> Self;
    fn __in<T: OptpyValue>(&self, other: &T) -> Self;
    fn __not_in<T: OptpyValue>(&self, other: &T) -> Self;
    fn __bit_and<T: OptpyValue>(&self, other: &T) -> Self;

    fn append<T: OptpyValue>(&self, other: &T);
    fn add<T: OptpyValue>(&self, other: &T);
    fn __delete<T: OptpyValue>(&self, other: &T);
}
