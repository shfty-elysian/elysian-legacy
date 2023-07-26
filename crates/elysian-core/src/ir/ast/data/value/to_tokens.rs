use quote::{quote, ToTokens};

use super::Value;

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Value::Boolean(b) => quote!(Value::Boolean(#b)),
            Value::Number(n) => quote!(Value::Number(#n)),
            Value::Struct(s) => quote!(Value::Struct(#s)),
        })
    }
}
