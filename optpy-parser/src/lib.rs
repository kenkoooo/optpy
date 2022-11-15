mod expression;
pub use expression::{
    BinaryOperation, BinaryOperator, BoolOperation, BoolOperator, CallFunction, CallMethod,
    Compare, CompareOperator, Expr, Index, ListComprehension, Number, UnaryOperation,
    UnaryOperator,
};

mod statement;
pub(crate) use statement::For;
use statement::RawStmt;
pub use statement::{Assign, Func, If, Statement, While};

use rustpython_parser::error::ParseError;

mod simplify;

pub fn parse<S: AsRef<str>>(code: S) -> Result<Vec<Statement>, ParseError> {
    let ast = rustpython_parser::parser::parse_program(code.as_ref(), "<embedded>")?;
    let statements = ast.iter().map(|s| RawStmt::parse(&s.node)).collect();
    let statements = simplify::simplify_list_comprehensions(statements);
    let statements = simplify::simplify_for_loops(statements);
    let statements = simplify::simplify_tuple_assignments(statements);
    Ok(statements)
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

    #[test]
    fn test_list_comprehension() {
        let code = r"a = [[i*j for j in range(M)] for i in range(N)]";

        let expected = r"
def __f15179191387192794179():
    __result = []
    __tmp_for_loop_iter_8723995406448537821 = list(range(N))
    __tmp_for_loop_iter_8723995406448537821.reverse()
    while len(__tmp_for_loop_iter_8723995406448537821) > 0:
        i = __tmp_for_loop_iter_8723995406448537821.pop()
        def __f862823992932926381():
            __result = []
            __tmp_for_loop_iter_324655153418689908 = list(range(M))
            __tmp_for_loop_iter_324655153418689908.reverse()
            while len(__tmp_for_loop_iter_324655153418689908) > 0:
                j = __tmp_for_loop_iter_324655153418689908.pop()
                __result.append(i * j)
            return __result
        __result.append(__f862823992932926381())
    return __result
a = __f15179191387192794179()
";
        assert_eq!(parse(code).unwrap(), parse(expected).unwrap());
    }
}
