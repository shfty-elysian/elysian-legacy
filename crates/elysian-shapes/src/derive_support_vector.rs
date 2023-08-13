use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PostModifier};
use elysian_core::identifier::Identifier;
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3},
    module::{
        AsModule, DomainsDyn, FunctionIdentifier, Module, SpecializationData, StructIdentifier,
        Type, CONTEXT,
    },
    property,
};

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeriveSupportVector;

impl DomainsDyn for DeriveSupportVector {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        vec![
            POSITION_2D.into(),
            POSITION_3D.into(),
            GRADIENT_2D.into(),
            GRADIENT_3D.into(),
        ]
    }
}

impl AsModule for DeriveSupportVector {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
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

        let derive_support_vector = FunctionIdentifier::new_dynamic("derive_support_vector".into());

        Module::new(
            self,
            spec,
            elysian_function! {
                pub fn derive_support_vector(mut CONTEXT) -> CONTEXT {
                    CONTEXT.support_vector = -CONTEXT.gradient.normalize() * CONTEXT.DISTANCE;
                    return CONTEXT;
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl PostModifier for DeriveSupportVector {}

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
