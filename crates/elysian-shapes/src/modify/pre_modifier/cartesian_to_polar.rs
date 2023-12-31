use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PreModifier};
use elysian_core::property_identifier::PropertyIdentifier;

use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData, CONTEXT},
};

pub const CARTESIAN_TO_POLAR: FunctionIdentifier =
    FunctionIdentifier::new("cartesian_to_polar", 1761953720101289514);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CartesianToPolar;

impl Hash for CartesianToPolar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        CARTESIAN_TO_POLAR.uuid().hash(state);
    }
}

impl Domains for CartesianToPolar {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsModule for CartesianToPolar {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let position = if spec.contains(&POSITION_2D.into()) {
            POSITION_2D
        } else if spec.contains(&POSITION_3D.into()) {
            POSITION_3D
        } else {
            panic!("No position domain")
        };

        let function = match &position {
            p if *p == POSITION_2D => elysian_function! {
                fn CARTESIAN_TO_POLAR(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = VECTOR2 {
                        X: CONTEXT.position.length(),
                        Y: CONTEXT.position.Y.atan2(CONTEXT.position.X),
                    };
                    return CONTEXT;
                }
            },
            p if *p == POSITION_3D => elysian_function! {
                fn CARTESIAN_TO_POLAR(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = VECTOR3 {
                        X: CONTEXT.position.length(),
                        Y: (CONTEXT.position.Z / CONTEXT.position.length()).acos(),
                        Z: CONTEXT.position.Y.sign() * (
                            CONTEXT.position.X / VECTOR2 {
                                X: CONTEXT.position.X,
                                Y: CONTEXT.position.Y,
                            }.length()
                        ).acos(),
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
impl PreModifier for CartesianToPolar {}

pub trait IntoCartesianToPolar {
    fn cartesian_to_polar(self) -> Modify;
}

impl<T> IntoCartesianToPolar for T
where
    T: IntoModify,
{
    fn cartesian_to_polar(self) -> Modify {
        self.modify().push_pre(CartesianToPolar)
    }
}
