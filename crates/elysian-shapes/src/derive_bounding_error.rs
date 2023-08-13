use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::identifier::Identifier;
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{DISTANCE, ERROR, POSITION_2D, POSITION_3D},
    module::{
        AsModule, DomainsDyn, ErasedHash, FunctionIdentifier, Module, SpecializationData,
        StructIdentifier, Type, CONTEXT,
    },
    property,
};
use elysian_proc_macros::elysian_expr;

use crate::{
    modify::{SUPPORT_VECTOR_2D, SUPPORT_VECTOR_3D},
    shape::{DynShape, IntoShape},
};

pub const DERIVE_CONTEXT: Identifier = Identifier::new("derive_context", 9284830371501785757);
property!(
    DERIVE_CONTEXT,
    DERIVE_CONTEXT_PROP,
    Type::Struct(StructIdentifier(CONTEXT))
);

#[derive(Debug)]
pub struct DeriveBoundingError {
    pub field: DynShape,
}

impl Hash for DeriveBoundingError {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.erased_hash());
    }
}

impl DomainsDyn for DeriveBoundingError {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.field
            .domains_dyn()
            .into_iter()
            .chain([POSITION_2D.into(), POSITION_3D.into()])
            .collect()
    }
}

impl AsModule for DeriveBoundingError {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let derive_bounding_error = FunctionIdentifier::new_dynamic("derive_bounding_error".into());

        let (position, support_vector) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, SUPPORT_VECTOR_2D),
            (false, true) => (POSITION_3D, SUPPORT_VECTOR_3D),
            _ => panic!("Invalid Position domain"),
        };

        let field_module = self.field.module(spec);
        let field_call_context = field_module.call(elysian_expr! { CONTEXT });
        let field_call_derive_context = field_module.call(elysian_expr! { DERIVE_CONTEXT });

        field_module.concat(Module::new(
            self,
            spec,
            elysian_function! {
                pub fn derive_bounding_error(mut CONTEXT) -> CONTEXT {
                    let mut DERIVE_CONTEXT = CONTEXT;
                    CONTEXT = #field_call_context;

                    if CONTEXT.support_vector.length().abs() == 0.0 {
                        return CONTEXT;
                    }

                    DERIVE_CONTEXT.position = DERIVE_CONTEXT.position + CONTEXT.support_vector;
                    DERIVE_CONTEXT = #field_call_derive_context;
                    CONTEXT.ERROR = DERIVE_CONTEXT.DISTANCE;
                    return CONTEXT;
                }
            },
        ))
    }
}

pub trait IntoDeriveBoundingError {
    fn derive_bounding_error(self) -> DeriveBoundingError;
}

impl<T> IntoDeriveBoundingError for T
where
    T: IntoShape,
{
    fn derive_bounding_error(self) -> DeriveBoundingError {
        DeriveBoundingError {
            field: self.shape(),
        }
    }
}
