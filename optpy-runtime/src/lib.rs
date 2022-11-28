macro_rules! include_optpy_runtime_module {
    ($module:expr) => {
        concat!(
            "mod ",
            stringify!($module),
            "{",
            include_str!(concat!("./", stringify!($module), ".rs")),
            "}"
        )
    };
    ($module:expr, $($modules:expr),+) => {
        concat!(include_optpy_runtime_module!($module),include_optpy_runtime_module!($($modules),+))
    };
}

/// The compiler will bundle the following string to the generated code.
/// Please add your module into not only this lib.rs file but also the following string when you add a new module.
pub const OPTPY_STD_STR: &str = concat!(
    include_optpy_runtime_module!(builtin, cell, dict, macros, number, stdlib, value),
    "pub use builtin::*;",
    "pub use stdlib::*;",
    "pub use value::*;"
);

mod builtin;
mod cell;
mod dict;
mod macros;
mod number;
mod stdlib;
mod value;

pub use builtin::*;
pub use stdlib::*;
pub use value::*;
