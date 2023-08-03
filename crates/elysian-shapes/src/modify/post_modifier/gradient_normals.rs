use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::Modify,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{GRADIENT_2D, GRADIENT_3D, NORMAL, VECTOR3, X, Y, Z},
        module::{
            AsModule, FunctionDefinition, FunctionIdentifier, InputDefinition, PropertyIdentifier,
            SpecializationData, CONTEXT,
        },
    },
};
use elysian_proc_macros::elysian_block;

pub const GRADIENT_NORMALS: FunctionIdentifier =
    FunctionIdentifier::new("gradient_normals", 18573716892008865657);

#[derive(Debug, Clone)]
pub struct GradientNormals;

impl Hash for GradientNormals {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        GRADIENT_NORMALS.uuid().hash(state);
    }
}

impl Domains for GradientNormals {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![GRADIENT_2D.into(), GRADIENT_3D.into()]
    }
}

impl AsIR for GradientNormals {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let block = if spec.contains(&GRADIENT_2D.into()) {
            elysian_block! {
                CONTEXT.NORMAL = VECTOR3 {
                    X: CONTEXT.GRADIENT_2D.X,
                    Y: CONTEXT.GRADIENT_2D.Y,
                    Z: 1.0,
                }.normalize();
                return CONTEXT;
            }
        } else if spec.contains(&GRADIENT_3D.into()) {
            elysian_block! {
                CONTEXT.NORMAL = CONTEXT.GRADIENT_3D.normalize();
                return CONTEXT;
            }
        } else {
            elysian_block! {
                return CONTEXT;
            }
        };

        vec![FunctionDefinition {
            id: GRADIENT_NORMALS.specialize(spec),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT.into(),
                mutable: true,
            }],
            output: CONTEXT.into(),
            block,
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        GRADIENT_NORMALS.specialize(spec).call(input)
    }
}

pub trait IntoGradientNormals {
    fn gradient_normals(self) -> Modify;
}

impl<T> IntoGradientNormals for T
where
    T: AsModule,
{
    fn gradient_normals(self) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: vec![Box::new(GradientNormals)],
        }
    }
}
