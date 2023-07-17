use crate::ir::ast::{Identifier, Property};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDefinition {
    pub id: Identifier,
    pub public: bool,
    pub fields: &'static [FieldDefinition],
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
    pub prop: Property,
    pub public: bool,
}
