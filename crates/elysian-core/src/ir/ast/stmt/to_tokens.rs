use quote::{quote, ToTokens};

use super::Stmt;

impl ToTokens for Stmt {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Stmt::Block(block) => quote!(Stmt::Block(#block)),
            Stmt::Bind { prop, expr } => {
                quote!(Stmt::Bind { prop: #prop, expr: #expr })
            }
            Stmt::Write { path, expr } => {
                quote!(Stmt::Write { path: [#(#path),*].into_iter().collect(), expr: #expr })
            }
            Stmt::If {
                cond,
                then,
                otherwise,
            } => {
                let otherwise = match otherwise {
                    Some(t) => quote!(Some(Box::new(#t))),
                    None => quote!(None),
                };
                quote!(Stmt::If {
                    cond: #cond,
                    then: Box::new(#then),
                    otherwise: #otherwise
                })
            }
            Stmt::Loop { stmt } => quote!(Stmt::Loop { stmt: Box::new(#stmt) }),
            Stmt::Break => quote!(Stmt::Break),
            Stmt::Output(output) => quote!(Stmt::Output(#output)),
        })
    }
}
