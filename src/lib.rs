use anyhow::Result;
use optpy_generator::{generate_code, generate_typed_code};
use optpy_parser::parse;
use optpy_resolver::resolve;
use optpy_runtime::{OPTPY_RUNTIME, OPTPY_TYPED_RUNTIME};

pub fn compile<S: AsRef<str>>(code: S) -> Result<String> {
    let ast = parse(code)?;
    let (ast, definitions) = resolve(&ast);
    let code = generate_code(&ast, &definitions);

    let mut result = OPTPY_RUNTIME.to_string();
    result += &code.to_string();
    Ok(result)
}

pub fn typed_compile<S: AsRef<str>>(code: S) -> Result<String> {
    let ast = parse(code)?;
    let (ast, definitions) = resolve(&ast);
    let code = generate_typed_code(&ast, &definitions);

    let mut result = OPTPY_TYPED_RUNTIME.to_string();
    result += &code.to_string();
    Ok(result)
}
