mod as_module;
mod function_definition;
mod struct_definition;
mod specialization_data;
mod ty;

pub use as_module::*;
pub use function_definition::*;
pub use struct_definition::*;
pub use specialization_data::*;
pub use ty::*;

use super::ast::{Identifier, TypeSpec};

use std::fmt::Debug;

pub struct Module<T>
where
    T: TypeSpec,
{
    pub function_definitions: Vec<FunctionDefinition<T>>,
    pub struct_definitions: Vec<StructDefinition>,
    pub entry_point: Identifier,
}

impl<T> Debug for Module<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("function_definitions", &self.function_definitions)
            .field("struct_definitions", &self.struct_definitions)
            .field("entry_point", &self.entry_point)
            .finish()
    }
}
