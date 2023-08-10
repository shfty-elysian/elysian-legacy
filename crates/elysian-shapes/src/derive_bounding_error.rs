use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ir::{
        ast::{Identifier, DISTANCE, ERROR, POSITION_2D, POSITION_3D},
        module::{
            AsIR, DomainsDyn, DynAsIR, FunctionIdentifier, IntoAsIR,
            SpecializationData, StructIdentifier, Type, CONTEXT,
        },
    },
    property,
};
use elysian_decl_macros::elysian_function;
use elysian_proc_macros::elysian_expr;

use crate::derive_support_vector::{SUPPORT_VECTOR_2D, SUPPORT_VECTOR_3D};

pub const DERIVE_CONTEXT: Identifier = Identifier::new("derive_context", 9284830371501785757);
property!(
    DERIVE_CONTEXT,
    DERIVE_CONTEXT_PROP,
    Type::Struct(StructIdentifier(CONTEXT))
);

#[derive(Debug)]
pub struct DeriveBoundingError {
    pub field: DynAsIR,
}

impl Hash for DeriveBoundingError {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl DomainsDyn for DeriveBoundingError {
    fn domains_dyn(&self) -> Vec<elysian_core::ir::module::PropertyIdentifier> {
        self.field
            .domains_dyn()
            .into_iter()
            .chain([POSITION_2D.into(), POSITION_3D.into()])
            .collect()
    }
}

impl AsIR for DeriveBoundingError {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("derive_bounding_error".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let entry_point = entry_point.clone();

        let (position, support_vector) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, SUPPORT_VECTOR_2D),
            (false, true) => (POSITION_3D, SUPPORT_VECTOR_3D),
            _ => panic!("Invalid Position domain"),
        };

        let (_, field_entry, field_functions) = self.field.prepare(spec);
        let field_call_context = field_entry.call(self.field.arguments(elysian_expr! { CONTEXT }));
        let field_call_derive_context =
            field_entry.call(self.field.arguments(elysian_expr! { DERIVE_CONTEXT }));

        field_functions
            .into_iter()
            .chain([elysian_function! {
                pub fn entry_point(mut CONTEXT) -> CONTEXT {
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
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoDeriveBoundingError {
    fn derive_bounding_error(self) -> DeriveBoundingError;
}

impl<T> IntoDeriveBoundingError for T
where
    T: IntoAsIR,
{
    fn derive_bounding_error(self) -> DeriveBoundingError {
        DeriveBoundingError {
            field: self.as_ir(),
        }
    }
}
