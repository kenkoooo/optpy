use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::Statement;

mod builtin;
mod call;
mod name;
pub mod util;

pub fn resolve(statements: &[Statement]) -> (Vec<Statement>, BTreeMap<String, BTreeSet<String>>) {
    let statements = name::resolve_names(statements);
    let statements = builtin::resolve_builtin_functions(&statements);
    let (statements, definitions) = call::resolve_function_calls(&statements);
    (statements, definitions)
}
