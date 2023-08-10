use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::modify::{IntoModify, Modify},
    ir::{
        ast::{POSITION_2D, UV},
        module::{
            AsIR, DomainsDyn, DynAsIR, FunctionIdentifier, IntoAsIR, SpecializationData, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;
use elysian_proc_macros::elysian_stmt;

#[derive(Debug)]
pub struct UvMap {
    field: DynAsIR,
}

impl UvMap {
    pub fn new(field: impl IntoAsIR) -> Self {
        UvMap {
            field: field.as_ir(),
        }
    }
}

impl Hash for UvMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir())
    }
}

impl DomainsDyn for UvMap {
    fn domains_dyn(&self) -> Vec<elysian_core::ir::module::PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsIR for UvMap {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("uv_map".into())
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let spec_map = SpecializationData::new_2d();
        let (_, field_call, field_functions) =
            self.field.call(&spec_map, elysian_stmt! { CONTEXT });

        field_functions
            .into_iter()
            .chain([elysian_function! {
                fn entry_point(mut CONTEXT) -> CONTEXT {
                    CONTEXT.POSITION_2D = CONTEXT.UV;

                    CONTEXT = #field_call;

                    return CONTEXT;
                }
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoUvMap {
    fn uv_map(self, field: impl IntoAsIR) -> Modify;
}

impl<T> IntoUvMap for T
where
    T: IntoModify,
{
    fn uv_map(self, field: impl IntoAsIR) -> Modify {
        self.modify().push_post(UvMap {
            field: field.as_ir(),
        })
    }
}
