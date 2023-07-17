use crate::ir::ast::{Block, Identifier, Property};

use super::StructDefinition;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionDefinition<N, V> {
    pub id: Identifier,
    pub public: bool,
    pub inputs: Vec<InputDefinition>,
    pub output: &'static StructDefinition,
    pub block: Block<N, V>,
}

impl<N, V> FunctionDefinition<N, V> {
    pub fn name(&self) -> &str {
        self.id.name()
    }

    pub fn name_unique(&self) -> String {
        self.id.name_unique()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputDefinition {
    pub prop: Property,
    pub mutable: bool,
}

