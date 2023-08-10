use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::{IntoModify, Modify},
    ir::{
        ast::{
            Identifier, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, VECTOR2,
            VECTOR3,
        },
        module::{
            AsIR, DomainsDyn, FunctionIdentifier, SpecializationData,
            StructIdentifier, Type, CONTEXT,
        },
    },
    property,
};
use elysian_decl_macros::elysian_function;

pub const SUPPORT_VECTOR_2D: Identifier =
    Identifier::new("support_vector_2d", 10984286761467088554);
property! {
    SUPPORT_VECTOR_2D,
    SUPPORT_VECTOR_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
}

pub const SUPPORT_VECTOR_3D: Identifier = Identifier::new("support_vector_3d", 67268427451093381);
property! {
    SUPPORT_VECTOR_3D,
    SUPPORT_VECTOR_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
}

#[derive(Debug, Hash)]
pub struct DeriveSupportVector;

impl DomainsDyn for DeriveSupportVector {
    fn domains_dyn(&self) -> Vec<elysian_core::ir::module::PropertyIdentifier> {
        vec![
            POSITION_2D.into(),
            POSITION_3D.into(),
            GRADIENT_2D.into(),
            GRADIENT_3D.into(),
        ]
    }
}

impl AsIR for DeriveSupportVector {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("derive_support_vector".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let entry_point = entry_point.clone();

        let support_vector = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => SUPPORT_VECTOR_2D,
            (false, true) => SUPPORT_VECTOR_3D,
            _ => panic!("Invalid Position domain"),
        };

        let gradient = match (
            spec.contains(&GRADIENT_2D.into()),
            spec.contains(&GRADIENT_3D.into()),
        ) {
            (true, false) => GRADIENT_2D,
            (false, true) => GRADIENT_3D,
            _ => panic!("Invalid Gradient domain"),
        };

        vec![elysian_function! {
            pub fn entry_point(mut CONTEXT) -> CONTEXT {
                CONTEXT.support_vector = -CONTEXT.gradient.normalize() * CONTEXT.DISTANCE;
                return CONTEXT;
            }
        }]
    }
}

pub trait IntoDeriveSupportVector {
    fn derive_support_vector(self) -> Modify;
}

impl<T> IntoDeriveSupportVector for T
where
    T: IntoModify,
{
    fn derive_support_vector(self) -> Modify {
        self.modify().push_post(DeriveSupportVector)
    }
}
