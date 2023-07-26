use quote::{quote, ToTokens};

use super::Number;

impl ToTokens for Number {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Number::UInt(u) => quote!(Number::UInt(#u)),
            Number::SInt(s) => quote!(Number::SInt(#s)),
            Number::Float(f) => quote!(Number::Float(#f)),
        })
    }
}

