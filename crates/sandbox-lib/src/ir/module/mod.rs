mod function_definition;
mod struct_definition;
mod ty;

pub use ty::*;
pub use function_definition::*;
pub use struct_definition::*;

#[derive(Debug)]
pub struct Module<N, V> {
    pub function_definitions: Vec<FunctionDefinition<N, V>>,
    pub struct_definitions: Vec<StructDefinition<N, V>>,
    pub entry_point: FunctionDefinition<N, V>,
}
