use quote::{quote, ToTokens};

use super::PropertyIdentifier;

impl ToTokens for PropertyIdentifier {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = &self.0;
        tokens.extend(quote!(PropertyIdentifier(#ident)));
    }
}
