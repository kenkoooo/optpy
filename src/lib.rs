pub mod dump;

use anyhow::Result;
use optpy_generator::generate_code;
use optpy_parser::parse;
use optpy_resolver::resolve;
use optpy_runtime::OPTPY_STD_STR;

pub fn compile<S: AsRef<str>>(code: S) -> Result<String> {
    let ast = parse(code)?;
    let (ast, definitions) = resolve(&ast);
    let code = generate_code(&ast, &definitions);

    let mut result = OPTPY_STD_STR.to_string();
    result += &code.to_string();
    Ok(result)
}
