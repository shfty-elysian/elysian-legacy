use std::collections::BTreeMap;

use crate::ir::ast::Property;

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: &'static str,
    pub public: bool,
    pub fields: BTreeMap<Property, FieldDefinition>,
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub public: bool,
}
