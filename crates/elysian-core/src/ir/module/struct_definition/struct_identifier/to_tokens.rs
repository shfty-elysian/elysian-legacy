use quote::{quote, ToTokens};

use super::StructIdentifier;

impl ToTokens for StructIdentifier {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = &self.0;
        tokens.extend(quote!(StructIdentifier(#ident)));
    }
}
