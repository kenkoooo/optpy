use std::{env::args, fs::read_to_string};

use optpy_compiler::compile_code;

fn main() {
    let path = args().skip(1).next().unwrap();
    let code = read_to_string(path).unwrap();
    let result = compile_code(&code).unwrap();
    println!("{}", result);
}
