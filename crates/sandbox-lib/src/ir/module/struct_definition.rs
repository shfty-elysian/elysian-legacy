use crate::ir::ast::{Identifier, Property};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDefinition {
    pub id: Identifier,
    pub public: bool,
    pub fields: &'static [FieldDefinition],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition {
    pub prop: Property,
    pub public: bool,
}
