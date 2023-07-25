use std::borrow::Cow;

use crate::ir::ast::Identifier;

use super::PropertyIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDefinition {
    pub id: Identifier,
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
