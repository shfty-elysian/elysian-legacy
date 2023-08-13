use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{GRADIENT_2D, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
    module::{
        AsModule, DomainsDyn, FunctionIdentifier, ErasedHash, Module, NumericType, SpecializationData,
        Type, CONTEXT,
    },
    property,
};
use elysian_proc_macros::elysian_stmt;

use crate::shape::{DynShape, Shape};

pub const ANGLE: Identifier = Identifier::new("angle", 17396665761465842676);
property!(ANGLE, ANGLE_PROP, Type::Number(NumericType::Float));

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotate {
    pub field: DynShape,
    pub angle: Expr,
}

impl Hash for Rotate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.erased_hash());
    }
}

impl DomainsDyn for Rotate {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.field
            .domains_dyn()
            .into_iter()
            .chain([POSITION_2D.into(), POSITION_3D.into()])
            .collect()
    }
}

impl AsModule for Rotate {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        assert!(
            spec.contains(&POSITION_2D.into()),
            "Rotate currently requires the 2D Position domain"
        );

        let field_module = self.field.module(spec);
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let rotate = FunctionIdentifier::new_dynamic("rotate".into());

        field_module.concat(Module::new(self, spec, elysian_function! {
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
        ))
    }
}

#[typetag::serde]
impl Shape for Rotate {}

pub trait IntoRotate {
    fn rotate(self, angle: impl IntoExpr) -> Rotate;
}

impl<T> IntoRotate for T
where
    T: 'static + Shape,
{
    fn rotate(self, angle: impl IntoExpr) -> Rotate {
        Rotate {
            field: Box::new(self),
            angle: angle.expr(),
        }
    }
}
