pub struct Bool(pub bool);
impl Bool {
    pub fn test(&self) -> bool {
        self.0
    }

    pub fn __unary_not(&self) -> Self {
        Self(!self.0)
    }
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
