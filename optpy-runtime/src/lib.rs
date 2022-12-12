macro_rules! include_module {
    ($file:expr, $name:ident) => {
        concat!("mod ", stringify!($name), "{", include_str!($file), "}")
    };
}

macro_rules! include_nested_modules {
    ($parent:ident, $($name:ident),+) => {
        concat!("mod ", stringify!($parent), "{",$(concat!(
            "mod ",
            stringify!($name),
            "{",
            include_str!(concat!("./", stringify!($parent), "/", stringify!($name), ".rs")),
            "}",
            "pub use self::",
            stringify!($name),
            "::*;"
        )),+, "}")
    };
}

/// The compiler will bundle the following string to the generated code.
/// Please add your module into not only this lib.rs file but also the following string when you add a new module.
pub const OPTPY_STD_STR: &str = concat!(
    include_module!("./builtin.rs", builtin),
    include_module!("./cell.rs", cell),
    include_module!("./number.rs", number),
    include_nested_modules!(stdlib, collections, math, sys, heapq),
    include_nested_modules!(value, value, list, dict, deque, string, iter),
    "pub use builtin::*;",
    "pub use stdlib::*;",
    "pub use value::*;"
);

mod builtin;
mod cell;
mod number;
mod stdlib;
mod value;

pub use builtin::*;
pub use stdlib::*;
pub use value::*;
