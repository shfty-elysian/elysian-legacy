use std::hash::Hash;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_decl_macros::elysian_function;
use elysian_ir::module::{
    AsModule, DomainsDyn, ErasedHash, FunctionIdentifier, Module, SpecializationData, CONTEXT,
};
use elysian_proc_macros::elysian_stmt;

use crate::shape::{DynShape, IntoShape, Shape};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prepass {
    prepass: DynShape,
    field: DynShape,
}

impl Hash for Prepass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.prepass.erased_hash());
        state.write_u64(self.field.erased_hash());
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

impl AsModule for Prepass {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let prepass_module = self.prepass.module(spec);
        let prepass_call = prepass_module.call(elysian_stmt! { CONTEXT });

        let field_module = self.field.module(spec);
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let prepass = FunctionIdentifier::new_dynamic("prepass".into());

        prepass_module.concat(field_module).concat(Module::new(
            self,
            spec,
            elysian_function! {
                fn prepass(CONTEXT) -> CONTEXT {
                    let CONTEXT = #prepass_call;
                    let CONTEXT = #field_call;
                    return CONTEXT;
                }
            },
        ))
    }
}

#[typetag::serde]
impl Shape for Prepass {}

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
