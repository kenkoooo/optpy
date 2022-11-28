macro_rules! include_module {
    ($file:expr, $name:ident) => {
        concat!("mod ", stringify!($name), "{", include_str!($file), "}")
    };
}

/// The compiler will bundle the following string to the generated code.
/// Please add your module into not only this lib.rs file but also the following string when you add a new module.
pub const OPTPY_STD_STR: &str = concat!(
    include_module!("./builtin.rs", builtin),
    include_module!("./cell.rs", cell),
    include_module!("./dict.rs", dict),
    include_module!("./macros.rs", macros),
    include_module!("./number.rs", number),
    include_module!("./value.rs", value),
    "mod stdlib {",
    include_module!("./stdlib/collections.rs", collections),
    include_module!("./stdlib/math.rs", math),
    include_module!("./stdlib/sys.rs", sys),
    "pub use self::collections::*;",
    "pub use self::math::*;",
    "pub use self::sys::*;",
    "}",
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
