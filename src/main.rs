use std::{env::args, fs::read_to_string};

use anyhow::Result;
use optpy::{concat_modules, read_src};
use optpy_compiler::compile_code;
use quote::quote;

fn main() -> Result<()> {
    let path = args().skip(1).next().unwrap();
    let code = read_to_string(path).unwrap();
    let result = compile_code(&code).unwrap();

    let std_module_map = read_src("./optpy-std/src")?;
    let std_modules = concat_modules(&std_module_map)?;

    let result = quote! {
        #std_modules
        #result
    };
    println!("{}", result);

    Ok(())
}
