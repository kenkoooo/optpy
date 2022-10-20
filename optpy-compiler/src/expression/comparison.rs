use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Comparison {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    In,
    NotIn,
    Is,
    IsNot,
}

impl From<&rustpython_parser::ast::Comparison> for Comparison {
    fn from(c: &rustpython_parser::ast::Comparison) -> Self {
        match c {
            rustpython_parser::ast::Comparison::Equal => Comparison::Equal,
            rustpython_parser::ast::Comparison::NotEqual => Comparison::NotEqual,
            rustpython_parser::ast::Comparison::Less => Comparison::Less,
            rustpython_parser::ast::Comparison::LessOrEqual => Comparison::LessOrEqual,
            rustpython_parser::ast::Comparison::Greater => Comparison::Greater,
            rustpython_parser::ast::Comparison::GreaterOrEqual => Comparison::GreaterOrEqual,
            rustpython_parser::ast::Comparison::In => Comparison::In,
            rustpython_parser::ast::Comparison::NotIn => Comparison::NotIn,
            rustpython_parser::ast::Comparison::Is => Comparison::Is,
            rustpython_parser::ast::Comparison::IsNot => Comparison::IsNot,
        }
    }
}

impl ToTokens for Comparison {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Comparison::Equal => todo!(),
            Comparison::NotEqual => todo!(),
            Comparison::Less => tokens.append_all(quote! { < }),
            Comparison::LessOrEqual => tokens.append_all(quote! { <= }),
            Comparison::Greater => todo!(),
            Comparison::GreaterOrEqual => todo!(),
            Comparison::In => todo!(),
            Comparison::NotIn => todo!(),
            Comparison::Is => todo!(),
            Comparison::IsNot => todo!(),
        }
    }
}
