macro_rules! include_optpy_std_module {
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
        concat!(include_optpy_std_module!($module),include_optpy_std_module!($($modules),+))
    };
}

/// The compiler will bundle the following string to the generated code.
/// Please add your module into not only this lib.rs file but also the following string when you add a new module.
pub const OPTPY_STD_STR: &str = concat!(
    include_optpy_std_module!(builtin, cell, dict, macros, number, value),
    "pub use builtin::*;",
    "pub use value::*;"
);

mod builtin;
mod cell;
mod dict;
mod macros;
mod number;
mod value;

pub use builtin::*;
pub use value::*;
