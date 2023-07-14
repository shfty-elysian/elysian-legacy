use std::collections::BTreeMap;

use crate::ir::{ast::Property, module::Type};

#[derive(Debug, Clone)]
pub struct StructDefinition<N, V> {
    pub name: &'static str,
    pub public: bool,
    pub fields: BTreeMap<Property, FieldDefinition<N, V>>,
}

#[derive(Debug, Clone)]
pub struct FieldDefinition<N, V> {
    pub ty: Type<N, V>,
    pub public: bool,
}
