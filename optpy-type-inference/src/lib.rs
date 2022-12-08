mod parser;
mod unionfind;

use optpy_parser::Statement;
use parser::Type;

pub fn resolve_types(statements: &[Statement]) {
    let mut parser = parser::Parser::default();

    parser.add_env(
        "range__macro__",
        Type::Fun(vec![Type::Int], Box::new(list(Type::Int))),
    );
    parser.add_env(
        "list",
        Type::Fun(
            vec![list(Type::TypeParam(0))],
            Box::new(list(Type::TypeParam(0))),
        ),
    );

    parser.parse_statements(statements);
}

fn list(t: Type) -> Type {
    Type::Fun(vec![Type::Int], Box::new(t))
}
