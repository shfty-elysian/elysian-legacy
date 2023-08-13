use std::hash::Hash;

use elysian_core::{
    expr::{Expr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{DISTANCE, GRADIENT_2D, NUM, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData, CONTEXT},
};

use elysian_decl_macros::elysian_function;

use crate::{field::RADIUS, rotate::ANGLE, shape::Shape};

pub const ARC: FunctionIdentifier = FunctionIdentifier::new("arc", 257188426632189116);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arc {
    angle: Expr,
    radius: Expr,
}

impl Arc {
    pub fn new(angle: impl IntoExpr, radius: impl IntoExpr) -> Self {
        Self {
            angle: angle.expr(),
            radius: radius.expr(),
        }
    }
}

impl Hash for Arc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ARC.uuid().hash(state);
    }
}

impl Domains for Arc {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into(), DISTANCE.into()]
    }
}

impl AsModule for Arc {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        assert!(
            spec.contains(&POSITION_2D.into()),
            "Arc currently requires the 2D Position domain"
        );

        assert!(
            spec.contains(&DISTANCE.into()),
            "Arc currently requires the Distance domain"
        );

        Module::new(
            self,
            spec,
            elysian_function! {
                fn ARC(ANGLE, RADIUS, mut CONTEXT) -> CONTEXT {
                    let ANGLE = ANGLE * 0.5;

                    let POSITION_2D = CONTEXT.POSITION_2D;

                    let NUM = CONTEXT.POSITION_2D.X.sign();
                    CONTEXT.POSITION_2D.X = CONTEXT.POSITION_2D.X.abs();

                    if ANGLE.cos() * CONTEXT.POSITION_2D.X > ANGLE.sin() * CONTEXT.POSITION_2D.Y {
                        let POSITION_2D = CONTEXT.POSITION_2D -
                            VECTOR2 {
                                X: ANGLE.sin(),
                                Y: ANGLE.cos()
                            } * RADIUS;

                        let DISTANCE = POSITION_2D.length();

                        CONTEXT.DISTANCE = DISTANCE;

                        CONTEXT.GRADIENT_2D = VECTOR2 {
                            X: NUM * POSITION_2D.X,
                            Y: POSITION_2D.Y
                        } / VECTOR2 {
                            X: DISTANCE,
                            Y: DISTANCE,
                        };
                    } else {
                        let DISTANCE = POSITION_2D.length();
                        CONTEXT.DISTANCE = (DISTANCE - RADIUS).abs();
                        CONTEXT.GRADIENT_2D =  POSITION_2D / VECTOR2 {
                            X: DISTANCE,
                            Y: DISTANCE,
                        } * (DISTANCE - RADIUS).sign();
                    }

                    return CONTEXT
                }
            },
        )
        .with_args([self.angle.clone().into(), self.radius.clone().into()])
    }
}

#[typetag::serde]
impl Shape for Arc {}
