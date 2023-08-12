use std::borrow::Cow;

use elysian_core::ast::property_identifier::PropertyIdentifier;

mod struct_identifier;
pub use struct_identifier::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDefinition {
    pub id: StructIdentifier,
    pub public: bool,
    pub fields: Cow<'static, [FieldDefinition]>,
}

impl StructDefinition {
    pub fn name(&self) -> &str {
        self.id.name()
    }

    pub fn name_unique(&self) -> String {
        self.id.name_unique()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition {
    pub id: PropertyIdentifier,
    pub public: bool,
}
