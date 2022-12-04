use crate::{cell::UnsafeRefMut, number::Number};

use super::{Bool, TypedList, TypedString};

pub trait TypedValue: AsValue + UnaryOps + BinOps + IndexOps + Sized + Default {
    fn __len(&self) -> Number {
        todo!()
    }
    fn __shallow_copy(&self) -> Self {
        todo!()
    }
    fn __abs(&self) -> Number {
        todo!()
    }
    fn test(&self) -> bool {
        todo!()
    }
    fn split(&self) -> TypedList<TypedString> {
        todo!()
    }
    fn assign(&mut self, _: &Self) {
        todo!()
    }
}

pub trait AsValue {
    fn __as_number(&self) -> Number {
        todo!()
    }
}

pub trait BinOps: Sized {
    fn __gt<T: TypedValue>(&self, _: &T) -> Bool {
        todo!()
    }

    fn __add<T: TypedValue>(&self, _: &T) -> Self {
        todo!()
    }
    fn __sub<T: TypedValue>(&self, _: &T) -> Self {
        todo!()
    }
    fn __mul<T: TypedValue>(&self, _: &T) -> Self {
        todo!()
    }

    fn __min<T: TypedValue>(&self, _: &T) -> Self {
        todo!()
    }
}

pub trait UnaryOps: Sized {
    fn __unary_sub(&self) -> Self {
        todo!()
    }
    fn __unary_not(&self) -> Self {
        todo!()
    }
}

pub trait IndexOps
where
    Self::Item: TypedValue,
{
    type Item;
    fn __index_value<T: TypedValue>(&self, _: &T) -> Self::Item {
        todo!()
    }
    fn __index_ref<T: TypedValue>(&self, _: &T) -> UnsafeRefMut<Self::Item> {
        todo!()
    }
    fn append(&self, _: &Self::Item) {
        todo!()
    }
    fn pop(&self) -> Self::Item {
        todo!()
    }
    fn reverse(&self) {
        todo!()
    }
}
