use quote::{quote, ToTokens};

use super::FunctionIdentifier;

impl ToTokens for FunctionIdentifier {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = &self.0;
        tokens.extend(quote!(FunctionIdentifier(#ident)))
    }
}

