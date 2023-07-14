use std::collections::BTreeMap;

use crate::ir::{
    ast::{Block, Property},
    module::Type,
};

#[derive(Debug)]
pub struct FunctionDefinition<N, V> {
    pub name: &'static str,
    pub public: bool,
    pub inputs: BTreeMap<Property, InputDefinition>,
    pub output: Type,
    pub block: Block<N, V>,
}

#[derive(Debug, Clone)]
pub struct InputDefinition {
    pub mutable: bool,
}
