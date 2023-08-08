use std::hash::Hash;

use elysian_core::{
    ast::expr::{Expr, IntoExpr},
    ir::{
        ast::{Identifier, DISTANCE, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, NumericType, PropertyIdentifier,
            SpecializationData, Type, CONTEXT,
        },
    },
    property,
};

use elysian_decl_macros::elysian_function;

use crate::field::RADIUS;

pub const ARC: FunctionIdentifier = FunctionIdentifier::new("arc", 257188426632189116);

pub const ANGLE: Identifier = Identifier::new("angle", 1618400179859647938);
property! {
    ANGLE,
    ANGLE_PROP,
    Type::Number(NumericType::Float)
}

#[derive(Debug, Clone)]
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

impl AsIR for Arc {
    fn entry_point(&self) -> FunctionIdentifier {
        ARC
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.angle.clone().into(), self.radius.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        assert!(
            spec.contains(&POSITION_2D.into()),
            "Arc currently requires the 2D Position domain"
        );

        assert!(
            spec.contains(&DISTANCE.into()),
            "Arc currently requires the Distance domain"
        );

        vec![elysian_function! {
            fn entry_point(ANGLE, RADIUS, mut CONTEXT) -> CONTEXT {
                CONTEXT.POSITION_2D.X = CONTEXT.POSITION_2D.X.abs();
                if ANGLE.cos() * CONTEXT.POSITION_2D.X > ANGLE.sin() * CONTEXT.POSITION_2D.Y {
                    CONTEXT.DISTANCE = (
                        CONTEXT.POSITION_2D -
                        VECTOR2 {
                            X: ANGLE.sin(),
                            Y: ANGLE.cos()
                        } * RADIUS
                    ).length()
                } else {
                    CONTEXT.DISTANCE = (CONTEXT.POSITION_2D.length() - RADIUS).abs()
                }
                return CONTEXT
            }
        }]
    }
}
