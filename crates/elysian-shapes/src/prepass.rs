use std::hash::Hash;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_decl_macros::elysian_function;
use elysian_ir::module::{
    AsIR, DomainsDyn, FunctionDefinition, FunctionIdentifier, Prepare, SpecializationData,
    StructDefinition, CONTEXT,
};
use elysian_proc_macros::elysian_stmt;

use crate::shape::{DynShape, IntoShape};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Prepass {
    prepass: DynShape,
    field: DynShape,
}

impl Hash for Prepass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.prepass.hash_ir());
        state.write_u64(self.field.hash_ir());
    }
}

impl DomainsDyn for Prepass {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.prepass
            .domains_dyn()
            .into_iter()
            .chain(self.field.domains_dyn())
            .collect()
    }
}

impl AsIR for Prepass {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("prepass".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (_, prepass_call, prepass_functions) =
            self.prepass.call(spec, elysian_stmt! { CONTEXT });
        let (_, field_call, field_functions) = self.field.call(spec, elysian_stmt! { CONTEXT });

        prepass_functions
            .into_iter()
            .chain(field_functions)
            .chain([elysian_function! {
                fn entry_point(CONTEXT) -> CONTEXT {
                    let CONTEXT = #prepass_call;
                    let CONTEXT = #field_call;
                    return CONTEXT;
                }
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        self.prepass
            .structs()
            .into_iter()
            .chain(self.field.structs())
            .collect()
    }
}

pub trait IntoPrepass {
    fn prepass(self, prepass: impl IntoShape) -> Prepass;
}

impl<T> IntoPrepass for T
where
    T: IntoShape,
{
    fn prepass(self, prepass: impl IntoShape) -> Prepass {
        Prepass {
            prepass: prepass.shape(),
            field: self.shape(),
        }
    }
}
