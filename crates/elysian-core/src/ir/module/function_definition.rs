use std::hash::{Hash, Hasher};

use crate::ir::ast::{Block, Identifier, Property};

use super::StructDefinition;

pub struct FunctionDefinition {
    pub id: Identifier,
    pub public: bool,
    pub inputs: Vec<InputDefinition>,
    pub output: &'static StructDefinition,
    pub block: Block,
}

impl IntoIterator for FunctionDefinition {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl std::fmt::Debug for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDefinition")
            .field("id", &self.id)
            .field("public", &self.public)
            .field("inputs", &self.inputs)
            .field("output", &self.output)
            .field("block", &self.block)
            .finish()
    }
}

impl Clone for FunctionDefinition {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            public: self.public.clone(),
            inputs: self.inputs.clone(),
            output: self.output,
            block: self.block.clone(),
        }
    }
}

impl Hash for FunctionDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.public.hash(state);
        self.inputs.hash(state);
        self.output.hash(state);
        self.block.hash(state);
    }
}

impl FunctionDefinition {
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
