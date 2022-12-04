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
pub const OPTPY_RUNTIME: &str = concat!(
    include_module!("./builtin.rs", builtin),
    include_module!("./cell.rs", cell),
    include_module!("./number.rs", number),
    include_nested_modules!(stdlib, collections, math, sys, heapq),
    include_nested_modules!(value, value, list, dict, deque, string),
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

pub mod typed_builtin;
pub mod typed_value;

/// The compiler will bundle the following string to the generated code.
/// Please add your module into not only this lib.rs file but also the following string when you add a new module.
pub const OPTPY_TYPED_RUNTIME: &str = concat!(
    include_module!("./typed_builtin.rs", typed_builtin),
    include_module!("./cell.rs", cell),
    include_module!("./number.rs", number),
    include_nested_modules!(typed_value, boolean, list, number, string, traits),
    "pub use typed_builtin::*;",
    "pub use typed_value::*;",
    "pub use number::Number;"
);
