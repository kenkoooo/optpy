use std::ops::Range;

use crate::Value;

pub fn range2(s: Value, t: Value) -> Range<usize> {
    match (s, t) {
        (Value::Integer { inner: s }, Value::Integer { inner: t }) => {
            let s: usize = s.as_ref().try_into().unwrap();
            let t: usize = t.as_ref().try_into().unwrap();
            s..t
        }
        _ => panic!(),
    }
}

#[macro_export]
macro_rules! range {
    ($s:expr, $t:expr) => {
        range2($s, $t)
    };
}
