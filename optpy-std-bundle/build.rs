use std::{
    collections::HashMap,
    env,
    fs::{self, read_to_string},
    path::Path,
};

use anyhow::{Context, Result};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{token::Brace, Item};

fn read_src<P: AsRef<Path>>(src: P) -> Result<HashMap<Vec<String>, String>> {
    let mut map = HashMap::new();
    list_module_files(src, &[], &mut map)?;
    Ok(map)
}

fn list_module_files<P: AsRef<Path>>(
    dir: P,
    mod_path: &[String],
    map: &mut HashMap<Vec<String>, String>,
) -> Result<()> {
    for entry in dir.as_ref().read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let mut mod_path = mod_path.to_vec();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .context("invalid directory name")?;
            mod_path.push(name.to_string());
            list_module_files(path, &mod_path, map)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let file_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .context("invalid module name")?;
            let content = read_to_string(&path)?;
            if file_name == "lib" && mod_path.is_empty() {
                map.insert(mod_path.to_vec(), content);
            } else if file_name == "mod" && !mod_path.is_empty() {
                map.insert(mod_path.to_vec(), content);
            } else {
                let mut mod_path = mod_path.to_vec();
                mod_path.push(file_name.to_string());
                map.insert(mod_path, content);
            }
        }
    }
    Ok(())
}

fn concat_modules(module_map: &HashMap<Vec<String>, String>) -> Result<TokenStream> {
    let items = concat_file(module_map, &[])?;
    Ok(quote! {#(#items)*})
}

fn concat_file(module_map: &HashMap<Vec<String>, String>, path: &[String]) -> Result<Vec<Item>> {
    let file = module_map.get(path).context("invalid module")?;
    let mut file = syn::parse_file(file)?;

    for item in file.items.iter_mut() {
        match item {
            syn::Item::Mod(item) => match item.content {
                Some(_) => todo!(),
                None => {
                    let mut path = path.to_vec();
                    path.push(item.ident.to_string());
                    let content = concat_file(module_map, &path)?;
                    item.content = Some((Brace(Span::call_site()), content))
                }
            },
            _ => {}
        }
    }
    Ok(file.items)
}

fn main() -> Result<()> {
    let std_module_map = read_src("../optpy-std/src")?;
    let std_modules = concat_modules(&std_module_map)?;

    let out_dir = env::var("OUT_DIR")?;
    let filepath = Path::new(&out_dir).join("optpy-std.rs");
    fs::write(filepath, std_modules.to_string())?;

    Ok(())
}
