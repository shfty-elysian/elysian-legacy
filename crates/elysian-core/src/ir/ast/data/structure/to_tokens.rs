use quote::{quote, ToTokens};

use super::Struct;

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let id = &self.id;
        let (keys, values): (Vec<_>, Vec<_>) = self.members.iter().unzip();
        tokens.extend(quote!(Struct {
            id: #id,
            members: [#((#keys, #values)),*].into_iter().collect()
        }))
    }
}
