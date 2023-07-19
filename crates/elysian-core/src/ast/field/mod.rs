mod capsule;
mod circle;
mod line;
mod point;
mod ring;

pub use capsule::*;
pub use circle::*;
pub use line::*;
pub use point::*;
pub use ring::*;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    as_ir::{AsIR, DynAsIR},
    ast::{Block, Identifier, TypeSpec, VectorSpace, CONTEXT},
    module::{AsModule, FunctionDefinition, InputDefinition, StructDefinition},
};

use crate::ir::ast::IntoValue;

use super::modify::CONTEXT_STRUCT;

pub struct Field<T, const N: usize> {
    pub field: DynAsIR<T, N>,
}

impl<T, const N: usize> Debug for Field<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").field("field", &self.field).finish()
    }
}

impl<T, const N: usize> Hash for Field<T, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl<T, const N: usize> AsModule<T, N> for Field<T, N>
where
    T: VectorSpace<N>,
    T::NUMBER: IntoValue<T, N>,
    T::VECTOR2: IntoValue<T, N>,
{
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("field")
    }

    fn functions(&self, entry_point: Identifier) -> Vec<FunctionDefinition<T, N>> {
        self.field
            .functions()
            .into_iter()
            .chain([FunctionDefinition {
                id: entry_point,
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT,
                block: Block(
                    [self.field.expression(CONTEXT.read()).output()]
                        .into_iter()
                        .collect(),
                ),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoField<T, const N: usize>: 'static + Sized + AsIR<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn field(self) -> Field<T, N> {
        Field {
            field: Box::new(self),
        }
    }
}

impl<T, U, const N: usize> IntoField<U, N> for T
where
    T: 'static + Sized + AsIR<U, N>,
    U: TypeSpec + VectorSpace<N>,
{
}
