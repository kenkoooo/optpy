use std::collections::BTreeSet;

use optpy_parser::{Expr, Statement};

#[cfg(test)]
mod tests {
    use optpy_parser::parse;
    use optpy_test_helper::{to_python_code, StripMargin};

    use crate::resolve_names;

    #[test]
    fn test_call_resolver() {
        let code = r"
            |a, b = map(int, input().split())
            |c = a + b
            |def f(a):
            |    def g(a):
            |        d = b + c
            |        return a + d
            |    return g(b) + a
            |d = f(a + b + c)
            |print(d)
        "
        .strip_margin();

        let expected = r"
            |__v0, __v1 = map(int, input().split())
            |__v2 = __v0 + __v1
            |def __f0(__v3):
            |    def __f1(__v4):
            |        __v5 = __v1 + __v2
            |        return __v4 + __v5
            |    return __f1(__v1) + __v3
            |__v6 = __f0(__v0 + __v1 + __v2)
            |print(__v6)    
        "
        .strip_margin();

        assert_eq!(
            to_python_code(&resolve_names(&parse(code).unwrap()).unwrap()).join("\n"),
            expected
        );
    }
}
