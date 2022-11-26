use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::Statement;

mod builtin;
mod call;
mod name;
pub mod util;

pub fn resolve(statements: &[Statement]) -> (Vec<Statement>, BTreeMap<String, BTreeSet<String>>) {
    let mut statements = statements.to_vec();
    loop {
        let new_statements = name::resolve_names(&statements);
        let new_statements = builtin::resolve_builtin_functions(&new_statements);
        let (new_statements, definitions) = call::resolve_function_calls(&new_statements);
        if new_statements == statements {
            return (new_statements, definitions);
        }
        statements = new_statements;
    }
}
