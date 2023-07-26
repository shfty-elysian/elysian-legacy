use quote::{quote, ToTokens};

use super::Block;

impl ToTokens for Block {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let stmts = &self.0;
        tokens.extend(quote!(Block([#(#stmts),*].into_iter().collect())))
    }
}
