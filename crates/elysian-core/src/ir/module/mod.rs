mod function_definition;
mod struct_definition;
mod ty;

pub use function_definition::*;
pub use struct_definition::*;
pub use ty::*;

use super::ast::{TypeSpec, VectorSpace};

#[derive(Debug)]
pub struct Module<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N>,
{
    pub function_definitions: Vec<FunctionDefinition<T, N>>,
    pub struct_definitions: Vec<StructDefinition>,
    pub entry_point: FunctionDefinition<T, N>,
}
