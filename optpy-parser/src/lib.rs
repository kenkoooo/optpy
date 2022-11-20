mod expression;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub use expression::{
    BinaryOperation, BinaryOperator, BoolOperation, BoolOperator, CallFunction, CallMethod,
    Compare, CompareOperator, Dict, Expr, Index, Number, UnaryOperation, UnaryOperator,
};

mod statement;
pub(crate) use statement::For;
use statement::RawStmt;
pub use statement::{Assign, Func, If, Statement, While};

use rustpython_parser::error::ParseError;

mod simplify;

pub fn parse<S: AsRef<str>>(code: S) -> Result<Vec<Statement>, ParseError> {
    let ast = rustpython_parser::parser::parse_program(code.as_ref(), "<embedded>")?;
    let statements = ast
        .iter()
        .flat_map(|s| RawStmt::parse(&s.node))
        .collect::<Vec<_>>();
    let statements = simplify::simplify_list_comprehensions(statements);
    let statements = simplify::simplify_for_loops(statements);
    let statements = simplify::simplify_tuple_assignments(statements);
    Ok(statements)
}

pub(crate) fn hash<T: Hash>(x: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic() {
        let code = r"
a, b, c = input().split()
print(a)
";

        let expected = r"
__tmp_for_tuple = input().split()
a = __tmp_for_tuple[0]
b = __tmp_for_tuple[1]
c = __tmp_for_tuple[2]
print(a)";
        assert_eq!(parse(code).unwrap(), parse(expected).unwrap());
    }

    #[test]
    fn test_if_statement() {
        let code = r"
if a <= c < b:
    result = 1
else:
    result = 2
print(result)
";
        let expected = r"
if a<=c and c<b:
    result = 1
else:
    result = 2
print(result)
";
        assert_eq!(parse(code).unwrap(), parse(expected).unwrap());
    }

    #[test]
    fn test_for_loop() {
        let code = r"
for i in range(N):
    print(i)
";
        let expected = r"
__tmp_for_loop_iter_14800386153579835208 = list(range(N))
__tmp_for_loop_iter_14800386153579835208.reverse()
while len(__tmp_for_loop_iter_14800386153579835208) > 0:
    i = __tmp_for_loop_iter_14800386153579835208.pop()
    print(i)
";
        assert_eq!(parse(code).unwrap(), parse(expected).unwrap());
    }
}
