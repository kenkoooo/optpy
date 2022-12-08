#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Boolean(bool);

impl Boolean {
    pub fn __test(&self) -> bool {
        self.0
    }

    pub fn __bit_and(&self, rhs: &Self) -> Self {
        Self(self.0 && rhs.0)
    }
}

impl From<bool> for Boolean {
    fn from(b: bool) -> Self {
        Self(b)
    }
}
