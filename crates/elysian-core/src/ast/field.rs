use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use indexmap::IndexMap;

use crate::ir::{
    as_ir::{AsIR, DynAsIR},
    ast::{Identifier, IntoBlock, IntoRead},
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData, Type, CONTEXT},
};

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
        _: &IndexMap<Identifier, Type>,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition> {
        self.field
            .functions(spec)
            .into_iter()
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    id: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT,
                block: self.field.expression(spec, CONTEXT.read()).output().block(),
            })
            .collect()
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
