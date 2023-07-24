use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    as_ir::{AsIR, DynAsIR},
    ast::{Identifier, IntoBlock, CONTEXT},
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData, StructDefinition},
};

use super::modify::CONTEXT_STRUCT;

pub struct Field {
    pub field: DynAsIR,
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").field("field", &self.field).finish()
    }
}

impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl AsModule for Field {
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("field")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition> {
        self.field
            .functions(spec)
            .into_iter()
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT.clone(),
                block: self.field.expression(spec, CONTEXT.read()).output().block(),
            })
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoField: 'static + Sized + AsIR {
    fn field(self) -> Field {
        Field {
            field: Box::new(self),
        }
    }
}

impl<T> IntoField for T where T: 'static + Sized + AsIR {}
