use quote::{quote, ToTokens};

use super::Expr;

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Expr::Literal(literal) => quote!(Expr::Literal(#literal)),
            Expr::Struct(struct_id, members) => {
                let (keys, values): (Vec<_>, Vec<_>) = members.into_iter().unzip();
                quote!(Expr::Struct(#struct_id, [#((#keys, #values)),*].into_iter().collect()))
            }
            Expr::Read(path) => quote!(Expr::Read([#(#path),*].into_iter().collect())),
            Expr::Call { function, args } => {
                quote!(Expr::Call { function: #function, args: [#(#args),*].into_iter().collect() })
            }
            Expr::Neg(t) => quote!(Expr::Neg(Box::new(#t))),
            Expr::Abs(t) => quote!(Expr::Abs(Box::new(#t))),
            Expr::Sign(t) => quote!(Expr::Sign(Box::new(#t))),
            Expr::Length(t) => quote!(Expr::Length(Box::new(#t))),
            Expr::Normalize(t) => quote!(Expr::Normalize(Box::new(#t))),
            Expr::Add(lhs, rhs) => quote!(Expr::Add(Box::new(#lhs), Box::new(#rhs))),
            Expr::Sub(lhs, rhs) => quote!(Expr::Sub(Box::new(#lhs), Box::new(#rhs))),
            Expr::Mul(lhs, rhs) => quote!(Expr::Mul(Box::new(#lhs), Box::new(#rhs))),
            Expr::Div(lhs, rhs) => quote!(Expr::Div(Box::new(#lhs), Box::new(#rhs))),
            Expr::Lt(lhs, rhs) => quote!(Expr::Lt(Box::new(#lhs), Box::new(#rhs))),
            Expr::Gt(lhs, rhs) => quote!(Expr::Gt(Box::new(#lhs), Box::new(#rhs))),
            Expr::Min(lhs, rhs) => quote!(Expr::Min(Box::new(#lhs), Box::new(#rhs))),
            Expr::Max(lhs, rhs) => quote!(Expr::Max(Box::new(#lhs), Box::new(#rhs))),
            Expr::Dot(lhs, rhs) => quote!(Expr::Dot(Box::new(#lhs), Box::new(#rhs))),
            Expr::Mix(lhs, rhs, t) => quote!(Expr::Mix(Box::new(#lhs), Box::new(#rhs, Box::new(#t)))),
        })
    }
}
