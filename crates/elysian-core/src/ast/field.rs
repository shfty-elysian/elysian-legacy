use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    ast::{Expr, IntoBlock},
    module::{
        AsIR, DynAsIR, FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead,
        PropertyIdentifier, SpecializationData, CONTEXT, DomainsDyn,
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

impl DomainsDyn for Field {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsIR for Field {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("field")
    }

    fn functions_impl(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let field_entry_point = self.field.entry_point(spec);
        let field_args = self.field.arguments(PropertyIdentifier(CONTEXT).read());

        self.field
            .functions_impl(spec, &field_entry_point)
            .into_iter()
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    id: PropertyIdentifier(CONTEXT),
                    mutable: false,
                }],
                output: PropertyIdentifier(CONTEXT),
                block: Expr::Call {
                    function: field_entry_point,
                    args: field_args,
                }
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
