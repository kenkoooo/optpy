#[cfg(test)]
pub mod test_env;

use anyhow::Result;
use optpy_generator::generate_code;
use optpy_parser::parse;
use optpy_resolver::resolve;

const STD: &str = include_str!("../optpy-std/src/lib.rs");
pub fn compile<S: AsRef<str>>(code: S) -> Result<String> {
    let ast = parse(code)?;
    let (ast, definitions) = resolve(&ast);
    let code = generate_code(&ast, &definitions);

    let mut result = STD.to_string();
    result += &code.to_string();
    Ok(result)
}
