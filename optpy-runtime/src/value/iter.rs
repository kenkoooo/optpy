use std::{fmt::Debug, rc::Rc};

use crate::Value;

#[derive(Clone)]
pub struct Iter(Rc<Box<dyn Iterator<Item = Value>>>);

impl Debug for Iter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Iter").finish()
    }
}

impl Iter {
    pub fn test(&self) -> bool {
        true
    }
}
