use quote::{quote, ToTokens};

use super::Identifier;

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let name = &self.name;
        let uuid = self.uuid.to_u128_le();

        tokens.extend(quote!(Identifier {
            name: std::borrow::Cow::Borrowed(#name),
            uuid: elysian::core::uuid::Uuid::from_u128(#uuid),
        }));
    }
}
