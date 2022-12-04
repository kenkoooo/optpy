use super::{TypedList, TypedValue};

pub struct TypedString();

impl TypedString {
    pub fn split(&self) -> TypedList<TypedString> {
        todo!()
    }
}

impl TypedValue for TypedString {
    fn __shallow_copy(&self) -> Self {
        todo!()
    }
}

impl Default for TypedString {
    fn default() -> Self {
        Self()
    }
}
