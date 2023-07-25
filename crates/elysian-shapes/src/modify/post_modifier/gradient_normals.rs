use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::Modify,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Expr, Identifier, IntoBlock, IntoLiteral, GRADIENT_2D, GRADIENT_3D, NORMAL, VECTOR3, X,
            Y, Z,
        },
        module::{
            AsModule, FunctionDefinition, InputDefinition, IntoRead, IntoWrite, PropertyIdentifier,
            SpecializationData, CONTEXT_PROP,
        },
    },
};

pub const GRADIENT_NORMALS: Identifier = Identifier::new("gradient_normals", 18573716892008865657);

#[derive(Debug, Clone, Hash)]
pub struct GradientNormals;

impl Domains for GradientNormals {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![GRADIENT_2D, GRADIENT_3D]
    }
}

impl AsIR for GradientNormals {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let block = if spec.contains(&GRADIENT_2D) {
            [
                GRADIENT_2D.bind([CONTEXT_PROP, GRADIENT_2D].read().normalize()),
                [CONTEXT_PROP, NORMAL].write(
                    Expr::Struct(
                        VECTOR3,
                        [
                            (X, [GRADIENT_2D, X].read()),
                            (Y, [GRADIENT_2D, Y].read()),
                            (Z, 1.0.literal()),
                        ]
                        .into_iter()
                        .collect(),
                    )
                    .normalize(),
                ),
                CONTEXT_PROP.read().output(),
            ]
            .block()
        } else if spec.contains(&GRADIENT_3D) {
            [
                [CONTEXT_PROP, NORMAL].write([CONTEXT_PROP, GRADIENT_3D].read().normalize()),
                CONTEXT_PROP.read().output(),
            ]
            .block()
        } else {
            [CONTEXT_PROP.read().output()].block()
        };

        vec![FunctionDefinition {
            id: GRADIENT_NORMALS.specialize(spec),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT_PROP,
                mutable: true,
            }],
            output: CONTEXT_PROP,
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
