use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PreModifier};
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData, CONTEXT},
};

use elysian_decl_macros::elysian_function;

pub const POLAR_TO_CARTESIAN: FunctionIdentifier =
    FunctionIdentifier::new("polar_to_cartesian", 11770202273010537);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PolarToCartesian;

impl Hash for PolarToCartesian {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        POLAR_TO_CARTESIAN.uuid().hash(state);
    }
}

impl Domains for PolarToCartesian {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsModule for PolarToCartesian {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let position = if spec.contains(&POSITION_2D.into()) {
            POSITION_2D
        } else if spec.contains(&POSITION_3D.into()) {
            POSITION_3D
        } else {
            panic!("No position domain")
        };

        let function = match &position {
            p if *p == POSITION_2D => elysian_function! {
                fn POLAR_TO_CARTESIAN(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = VECTOR2 {
                        X: CONTEXT.position.X * CONTEXT.position.Y.cos(),
                        Y: CONTEXT.position.X * CONTEXT.position.Y.sin(),
                    };
                    return CONTEXT;
                }
            },
            p if *p == POSITION_3D => elysian_function! {
                fn POLAR_TO_CARTESIAN(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = VECTOR3 {
                        X: CONTEXT.position.X * CONTEXT.position.Y.sin() * CONTEXT.position.Z.cos(),
                        Y: CONTEXT.position.X * CONTEXT.position.Y.sin() * CONTEXT.position.Z.sin(),
                        Z: CONTEXT.position.X * CONTEXT.position.Y.cos(),
                    };
                    return CONTEXT;
                }
            },
            _ => unreachable!(),
        };

        Module::new(self, spec, function)
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl PreModifier for PolarToCartesian {}

pub trait IntoPolarToCartesian {
    fn polar_to_cartesian(self) -> Modify;
}

impl<T> IntoPolarToCartesian for T
where
    T: IntoModify,
{
    fn polar_to_cartesian(self) -> Modify {
        self.modify().push_pre(PolarToCartesian)
    }
}
