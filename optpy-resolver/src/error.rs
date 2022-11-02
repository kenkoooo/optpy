#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unresolved name: {0}")]
    Unresolved(String),
}

pub type Result<T> = std::result::Result<T, Error>;
