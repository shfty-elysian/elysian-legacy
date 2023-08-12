use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify};
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData, CONTEXT},
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

impl AsIR for PolarToCartesian {
    fn entry_point(&self) -> FunctionIdentifier {
        POLAR_TO_CARTESIAN
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let position = if spec.contains(&POSITION_2D.into()) {
            POSITION_2D
        } else if spec.contains(&POSITION_3D.into()) {
            POSITION_3D
        } else {
            panic!("No position domain")
        };

        vec![match &position {
            p if *p == POSITION_2D => elysian_function! {
                fn entry_point(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = VECTOR2 {
                        X: CONTEXT.position.X * CONTEXT.position.Y.cos(),
                        Y: CONTEXT.position.X * CONTEXT.position.Y.sin(),
                    };
                    return CONTEXT;
                }
            },
            p if *p == POSITION_3D => elysian_function! {
                fn entry_point(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = VECTOR3 {
                        X: CONTEXT.position.X * CONTEXT.position.Y.sin() * CONTEXT.position.Z.cos(),
                        Y: CONTEXT.position.X * CONTEXT.position.Y.sin() * CONTEXT.position.Z.sin(),
                        Z: CONTEXT.position.X * CONTEXT.position.Y.cos(),
                    };
                    return CONTEXT;
                }
            },
            _ => unreachable!(),
        }]
    }
}

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
