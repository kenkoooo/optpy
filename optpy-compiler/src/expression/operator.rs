use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

impl From<&rustpython_parser::ast::Operator> for Operator {
    fn from(op: &rustpython_parser::ast::Operator) -> Self {
        match op {
            rustpython_parser::ast::Operator::Add => Operator::Add,
            rustpython_parser::ast::Operator::Sub => Operator::Sub,
            rustpython_parser::ast::Operator::Mult => Operator::Mult,
            rustpython_parser::ast::Operator::MatMult => Operator::MatMult,
            rustpython_parser::ast::Operator::Div => Operator::Div,
            rustpython_parser::ast::Operator::Mod => Operator::Mod,
            rustpython_parser::ast::Operator::Pow => Operator::Pow,
            rustpython_parser::ast::Operator::LShift => Operator::LShift,
            rustpython_parser::ast::Operator::RShift => Operator::RShift,
            rustpython_parser::ast::Operator::BitOr => Operator::BitOr,
            rustpython_parser::ast::Operator::BitXor => Operator::BitXor,
            rustpython_parser::ast::Operator::BitAnd => Operator::BitAnd,
            rustpython_parser::ast::Operator::FloorDiv => Operator::FloorDiv,
        }
    }
}

impl ToTokens for Operator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Operator::Add => tokens.append_all(quote! { + }),
            Operator::Sub => tokens.append_all(quote! { - }),
            Operator::Mult => tokens.append_all(quote! { * }),
            Operator::MatMult => todo!(),
            Operator::Div => todo!(),
            Operator::Mod => todo!(),
            Operator::Pow => todo!(),
            Operator::LShift => todo!(),
            Operator::RShift => todo!(),
            Operator::BitOr => todo!(),
            Operator::BitXor => todo!(),
            Operator::BitAnd => todo!(),
            Operator::FloorDiv => todo!(),
        }
    }
}
