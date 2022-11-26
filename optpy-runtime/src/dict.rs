
use crate::number::Number;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DictKey {
    Number(Number),
    String(String),
}
