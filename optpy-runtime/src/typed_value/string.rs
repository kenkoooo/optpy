use std::rc::Rc;

use super::{TypedList, TypedValue};

pub struct TypedString(pub Rc<String>);

impl TypedString {
    pub fn split(&self) -> TypedList<TypedString> {
        let list = self
            .0
            .split_ascii_whitespace()
            .map(|s| TypedString::from(s))
            .collect::<Vec<_>>();
        TypedList::from(list)
    }
}

impl TypedValue for TypedString {
    fn __shallow_copy(&self) -> Self {
        todo!()
    }
}

impl Default for TypedString {
    fn default() -> Self {
        Self(Rc::new(String::new()))
    }
}
impl From<&str> for TypedString {
    fn from(v: &str) -> Self {
        Self(Rc::new(v.to_string()))
    }
}
