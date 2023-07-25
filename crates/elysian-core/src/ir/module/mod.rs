mod as_module;
mod function_definition;
mod specialization_data;
mod struct_definition;
mod ty;

pub use as_module::*;
pub use function_definition::*;
use indexmap::IndexMap;
pub use specialization_data::*;
pub use struct_definition::*;
pub use ty::*;

use super::ast::Identifier;

#[derive(Debug)]
pub struct Module {
    pub types: IndexMap<Identifier, Type>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub struct_definitions: Vec<StructDefinition>,
    pub entry_point: Identifier,
}
