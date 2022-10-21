use anyhow::Result;
use optpy_std_bundle::OPTPY_STD_CONTENT;

pub fn transpile(python_code: &str) -> Result<String> {
    let mut result = OPTPY_STD_CONTENT.to_string();
    let code = optpy_compiler::compile_code(python_code)?;
    result.push_str(&code.to_string());
    Ok(result)
}
