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
    ast::{Identifier, IntoBlock, TypeSpec, CONTEXT},
    module::{AsModule, FunctionDefinition, InputDefinition, StructDefinition},
};

use crate::ir::ast::IntoValue;

use super::modify::CONTEXT_STRUCT;

pub struct Field<T> {
    pub field: DynAsIR<T>,
}

impl<T> Debug for Field<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").field("field", &self.field).finish()
    }
}

impl<T> Hash for Field<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl<T> AsModule<T> for Field<T>
where
    T: TypeSpec,
    T::NUMBER: IntoValue<T>,
    T::VECTOR2: IntoValue<T>,
{
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("field")
    }

    fn functions(&self, entry_point: &Identifier) -> Vec<FunctionDefinition<T>> {
        self.field
            .functions()
            .into_iter()
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT,
                block: self.field.expression(CONTEXT.read()).output().block(),
            })
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoField<T>: 'static + Sized + AsIR<T>
where
    T: TypeSpec,
{
    fn field(self) -> Field<T> {
        Field {
            field: Box::new(self),
        }
    }
}

impl<T, U> IntoField<U> for T
where
    T: 'static + Sized + AsIR<U>,
    U: TypeSpec,
{
}
