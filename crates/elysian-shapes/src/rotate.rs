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
        AsIR, DomainsDyn, FunctionIdentifier, NumericType, SpecializationData, Type, CONTEXT,
    },
    property,
};
use elysian_proc_macros::elysian_stmt;

pub const ANGLE: Identifier = Identifier::new("angle", 17396665761465842676);
property!(ANGLE, ANGLE_PROP, Type::Number(NumericType::Float));

#[derive(Debug)]
pub struct Rotate {
    pub field: Box<dyn AsIR>,
    pub angle: Expr,
}

impl Hash for Rotate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
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

impl AsIR for Rotate {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("rotate".into())
    }

    fn arguments(&self, input: elysian_ir::ast::Expr) -> Vec<elysian_ir::ast::Expr> {
        vec![self.angle.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_ir::module::FunctionDefinition> {
        assert!(
            spec.contains(&POSITION_2D.into()),
            "Rotate currently requires the 2D Position domain"
        );

        let (_, field_call, field_functions) = self.field.call(spec, elysian_stmt! { CONTEXT });

        field_functions
            .into_iter()
            .chain([elysian_function! {
                pub fn entry_point(ANGLE, mut CONTEXT) -> CONTEXT {
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
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_ir::module::StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoRotate {
    fn rotate(self, angle: impl IntoExpr) -> Rotate;
}

impl<T> IntoRotate for T
where
    T: 'static + AsIR,
{
    fn rotate(self, angle: impl IntoExpr) -> Rotate {
        Rotate {
            field: Box::new(self),
            angle: angle.expr(),
        }
    }
}
