use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use indexmap::IndexMap;

use crate::ir::{
    as_ir::{AsIR, DynAsIR},
    ast::IntoBlock,
    module::{
        AsModule, FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead,
        PropertyIdentifier, SpecializationData, Type, CONTEXT_PROP,
    },
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
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("field")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        _: &IndexMap<PropertyIdentifier, Type>,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        self.field
            .functions(spec)
            .into_iter()
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    id: CONTEXT_PROP,
                    mutable: false,
                }],
                output: CONTEXT_PROP,
                block: self
                    .field
                    .expression(spec, CONTEXT_PROP.read())
                    .output()
                    .block(),
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
