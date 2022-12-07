mod parser;

use optpy_parser::Statement;
use parser::Type;

pub fn resolve_types(statements: &[Statement]) {
    let mut parser = parser::Parser::default();

    parser.add_env(
        "range__macro__",
        Type::Fun(
            vec![Type::Int],
            Box::new(Type::Fun(vec![Type::Int], Box::new(Type::Int))),
        ),
    );

    parser.parse_statements(statements);
}
