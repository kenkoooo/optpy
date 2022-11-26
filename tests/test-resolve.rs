use optpy_dump::DumpPython;
use optpy_parser::parse;
use optpy_resolver::resolve;

fn dump(code: &str) -> String {
    let ast = parse(code).unwrap();
    let (ast, _) = resolve(&ast);
    ast.to_python_code()
}

#[test]
fn test_sibling_function_resolve() {
    let code = r"
N = 2
def f1():
    return N + 1
def f2():
    return f1()
return f2()
";

    let expected = r"
v1 = 2
def f1(v1):
    return v1 + 1
def f2(v1):
    return f1(v1)
return f2(v1)";

    assert_eq!(dump(code), dump(expected));
}
