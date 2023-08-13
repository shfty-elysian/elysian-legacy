use std::{
    fmt::Debug,
    hash::Hash,
};

use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{GRADIENT_2D, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
    module::{
        DomainsDyn, FunctionIdentifier, Module, NumericType, SpecializationData,
        Type, CONTEXT,
    },
    property,
};

use crate::{shape::Shape, wrap::{Wrapper, Wrap}};

pub const ANGLE: Identifier = Identifier::new("angle", 17396665761465842676);
property!(ANGLE, ANGLE_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotate {
    pub angle: Expr,
}

impl DomainsDyn for Rotate {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wrapper for Rotate {
    fn module(&self,spec: &SpecializationData,field_call:elysian_ir::ast::Expr) -> Module {
        let rotate = FunctionIdentifier::new_dynamic("rotate".into());

        Module::new(self, spec, elysian_function! {
            pub fn rotate(ANGLE, mut CONTEXT) -> CONTEXT {
                CONTEXT.POSITION_2D = VECTOR2 {
                    X: CONTEXT.POSITION_2D.X * ANGLE.cos() - CONTEXT.POSITION_2D.Y * ANGLE.sin(),
                    Y: CONTEXT.POSITION_2D.Y * ANGLE.cos() + CONTEXT.POSITION_2D.X * ANGLE.sin(),
                };

                CONTEXT = #field_call;

                let ANGLE = -ANGLE;
                CONTEXT.GRADIENT_2D = VECTOR2 {
                    X: CONTEXT.GRADIENT_2D.X * ANGLE.cos() - CONTEXT.GRADIENT_2D.Y * ANGLE.sin(),
                    Y: CONTEXT.GRADIENT_2D.Y * ANGLE.cos() + CONTEXT.GRADIENT_2D.X * ANGLE.sin(),
                };

                return CONTEXT;
            }
        }).with_args(
            [self.angle.clone().into()], 
        )
    }
}

pub trait IntoRotate {
    fn rotate(self, angle: impl IntoExpr) -> Wrap;
}

impl<T> IntoRotate for T
where
    T: 'static + Shape,
{
    fn rotate(self, angle: impl IntoExpr) -> Wrap {
        Wrap::new(
            Rotate {
                angle: angle.expr(),
            },
            self,
        )
    }
}
