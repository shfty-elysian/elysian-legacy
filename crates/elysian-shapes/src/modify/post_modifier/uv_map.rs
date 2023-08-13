use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    modify::{IntoModify, Modify, PostModifier},
    shape::{DynShape, IntoShape},
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, UV},
    module::{
        AsModule, DomainsDyn, FunctionIdentifier, HashIR, Module, SpecializationData, CONTEXT,
    },
};
use elysian_proc_macros::elysian_stmt;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UvMap {
    field: DynShape,
}

impl UvMap {
    pub fn new(field: impl IntoShape) -> Self {
        UvMap {
            field: field.shape(),
        }
    }
}

impl Hash for UvMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir())
    }
}

impl DomainsDyn for UvMap {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsModule for UvMap {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let spec_map = SpecializationData::new_2d();
        let field_module = self.field.module_impl(&spec_map);
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let uv_map = FunctionIdentifier::new_dynamic("uv_map".into());

        field_module.concat(Module::new(
            self,
            spec,
            elysian_function! {
                fn uv_map(mut CONTEXT) -> CONTEXT {
                    CONTEXT.POSITION_2D = CONTEXT.UV;

                    CONTEXT = #field_call;

                    return CONTEXT;
                }
            },
        ))
    }
}

#[cfg_attr(feature = "serde", typetag::serialize)]
impl PostModifier for UvMap {}

pub trait IntoUvMap {
    fn uv_map(self, field: impl IntoShape) -> Modify;
}

impl<T> IntoUvMap for T
where
    T: IntoModify,
{
    fn uv_map(self, field: impl IntoShape) -> Modify {
        self.modify().push_post(UvMap {
            field: field.shape(),
        })
    }
}
