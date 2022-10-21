use anyhow::Result;
use optpy_std_bundle::OPTPY_STD_CONTENT;

/// Convert Python code to Rust code and put polyfilled Python's standard libraries into the Rust file.
pub fn transpile(python_code: &str) -> Result<String> {
    let mut result = OPTPY_STD_CONTENT.to_string();
    let code = optpy_compiler::compile_code(python_code)?;
    result.push_str(&code.to_string());
    Ok(result)
}
