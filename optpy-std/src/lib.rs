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
    include_optpy_std_module!(builtin, cell, dict, macros, number, object, value),
    "pub use builtin::*;",
    "pub use object::*;"
);

pub mod builtin;
pub mod typed_builtin;

pub mod object;
pub mod typed_object;

mod cell;
mod dict;
mod macros;
mod number;
mod value;
