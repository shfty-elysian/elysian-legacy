use crate::ir::ast::{Block, Identifier, Property};

use super::StructDefinition;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionDefinition<N, V> {
    pub id: Identifier,
    pub public: bool,
    pub inputs: &'static [InputDefinition],
    pub output: &'static StructDefinition,
    pub block: Block<N, V>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputDefinition {
    pub prop: Property,
    pub mutable: bool,
}
