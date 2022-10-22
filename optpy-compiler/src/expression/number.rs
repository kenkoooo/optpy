use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, Clone)]
pub(crate) enum Number {
    Integer { value: String },
    Float { value: f64 },
}

impl From<&rustpython_parser::ast::Number> for Number {
    fn from(v: &rustpython_parser::ast::Number) -> Self {
        match v {
            rustpython_parser::ast::Number::Integer { value } => Number::Integer {
                value: value.to_string(),
            },
            rustpython_parser::ast::Number::Float { value } => Number::Float { value: *value },
            rustpython_parser::ast::Number::Complex { .. } => todo!(),
        }
    }
}

impl ToTokens for Number {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Number::Integer { value } => {
                if let Ok(value) = value.parse::<i64>() {
                    tokens.append_all(quote! {
                        Value::from(#value)
                    });
                } else {
                    tokens.append_all(quote! {
                        Value::bigint(#value)
                    });
                }
            }
            Number::Float { value } => tokens.append_all(quote! { Value::float(#value) }),
        }
    }
}
