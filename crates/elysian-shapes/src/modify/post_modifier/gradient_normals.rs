use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::{Modify, CONTEXT_STRUCT},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Expr, Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite, CONTEXT, GRADIENT_2D,
            GRADIENT_3D, NORMAL, X, Y,
        },
        module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData},
    },
};

pub const GRADIENT_NORMALS: Identifier = Identifier::new("gradient_normals", 18573716892008865657);

#[derive(Debug, Clone, Hash)]
pub struct GradientNormals;

impl Domains for GradientNormals {
    fn domains() -> Vec<Identifier> {
        vec![GRADIENT_2D.id().clone(), GRADIENT_3D.id().clone()]
    }
}

impl AsIR for GradientNormals {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let block = if spec.contains(GRADIENT_2D.id()) {
            [
                GRADIENT_2D.bind([CONTEXT, GRADIENT_2D].read().normalize()),
                [CONTEXT, NORMAL].write(
                    Expr::vector3(
                        [GRADIENT_2D, X].read(),
                        [GRADIENT_2D, Y].read(),
                        1.0.literal(),
                    )
                    .normalize(),
                ),
                CONTEXT.read().output(),
            ]
            .block()
        } else if spec.contains(GRADIENT_3D.id()) {
            [
                [CONTEXT, NORMAL].write([CONTEXT, GRADIENT_3D].read().normalize()),
                CONTEXT.read().output(),
            ]
            .block()
        } else {
            [CONTEXT.read().output()].block()
        };

        vec![FunctionDefinition {
            id: GRADIENT_NORMALS.specialize(spec),
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
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
