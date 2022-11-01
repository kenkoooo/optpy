use std::collections::BTreeSet;

use optpy_parser::{Expr, Statement};

pub fn resolve_calls(statements: &[Statement]) {}

fn collect_referred_variables(body: &[Statement]) -> BTreeSet<String> {
    let mut variables = BTreeSet::new();
    for statement in body {
        match statement {
            Statement::Assign { target: _, value } => {}
            Statement::Expression(_) => todo!(),
            Statement::If { test, body, orelse } => todo!(),
            Statement::Func { name, args, body } => {
                let args = args.iter().cloned().collect::<BTreeSet<_>>();
                let body = collect_referred_variables(body);
                let external = body.difference(&args).cloned().collect::<BTreeSet<_>>();
                // TODO name -> external
                variables.extend(external);
            }
            Statement::Return(_) => todo!(),
        }
    }
    todo!()
}

fn collect_one(expr: &Expr) -> BTreeSet<String> {
    match expr {
        Expr::CallFunction { name: _, args } => collect(args),
        Expr::CallMethod {
            value,
            name: _,
            args,
        } => {
            let mut value = collect_one(value);
            value.extend(collect(args));
            value
        }
        Expr::Tuple(e) => collect(e),
        Expr::Ident(name) => BTreeSet::from([name.clone()]),
        Expr::BoolOperation { op: _, conditions } => collect(conditions),
        Expr::Compare { left, right, op: _ } => {
            let mut left = collect_one(left);
            let right = collect_one(right);
            left.extend(right);
            left
        }
        Expr::BinaryOperation { left, right, op: _ } => {
            let mut left = collect_one(left);
            let right = collect_one(right);
            left.extend(right);
            left
        }
        Expr::Number(_) => BTreeSet::new(),
    }
}
fn collect(exprs: &[Expr]) -> BTreeSet<String> {
    exprs.iter().flat_map(|e| collect_one(e)).collect()
}

#[cfg(test)]
mod tests {
    use optpy_parser::parse;
    use optpy_test_helper::{to_python_code, StripMargin};

    use crate::resolve_names;

    #[test]
    fn test_call_resolver() {
        let code = r"
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
            expected.trim()
        );
    }
}
