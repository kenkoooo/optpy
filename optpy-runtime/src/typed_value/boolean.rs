use super::{AsValue, BinOps, IndexOps, TypedValue, UnaryOps};

pub struct Bool(pub bool);
impl TypedValue for Bool {
    fn test(&self) -> bool {
        self.0
    }
}
impl AsValue for Bool {}
impl BinOps for Bool {}
impl UnaryOps for Bool {}
impl IndexOps for Bool {
    type Item = Self;
}

impl Default for Bool {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl From<bool> for Bool {
    fn from(v: bool) -> Self {
        Self(v)
    }
}
