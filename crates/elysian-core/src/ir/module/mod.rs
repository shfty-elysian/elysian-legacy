mod as_module;
mod function_definition;
mod struct_definition;
mod ty;

pub use as_module::*;
pub use function_definition::*;
pub use struct_definition::*;
pub use ty::*;

use super::ast::{Identifier, TypeSpec, VectorSpace};

use std::fmt::Debug;

pub struct Module<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N>,
{
    pub function_definitions: Vec<FunctionDefinition<T, N>>,
    pub struct_definitions: Vec<StructDefinition>,
    pub entry_point: Identifier,
}

impl<T, const N: usize> Debug for Module<T, N>
where
    T: VectorSpace<N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("function_definitions", &self.function_definitions)
            .field("struct_definitions", &self.struct_definitions)
            .field("entry_point", &self.entry_point)
            .finish()
    }
}
