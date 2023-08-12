mod as_ir;
mod domains;
mod function_definition;
mod hash_ir;
mod specialization_data;
mod struct_definition;
mod ty;

pub use as_ir::*;
pub use domains::*;
pub use function_definition::*;
pub use hash_ir::*;
pub use specialization_data::*;
pub use struct_definition::*;
pub use ty::*;

use indexmap::IndexMap;

use crate::ast::property_identifier::PropertyIdentifier;

#[derive(Debug)]
pub struct Module {
    pub props: IndexMap<PropertyIdentifier, Type>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub struct_definitions: Vec<StructDefinition>,
    pub entry_point: FunctionIdentifier,
}
