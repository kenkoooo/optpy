mod generator;
pub use generator::CodeGenerator;

use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::Statement;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_code(
    statements: &[Statement],
    definitions: &BTreeMap<String, BTreeSet<String>>,
) -> TokenStream {
    let generator = CodeGenerator::default();
    let body = generator.generate_function_body(statements, "", definitions);
    quote! {
        fn main() {
            #body
        }
    }
}
