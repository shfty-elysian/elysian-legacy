use std::{fmt::Debug, hash::Hash};

use rust_gpu_bridge::One;

use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{
            Expr, Identifier, IntoBlock, IntoLiteral, IntoRead, IntoValue, IntoWrite, TypeSpec,
            CONTEXT, GRADIENT_2D, GRADIENT_3D, NORMAL, X, Y,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

pub const GRADIENT_NORMALS: Identifier = Identifier::new("gradient_normals", 18573716892008865657);

#[derive(Debug, Clone, Hash)]
pub struct GradientNormals;

impl FilterSpec for GradientNormals {
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        spec.filter([GRADIENT_2D.id(), GRADIENT_3D.id()])
    }
}

impl<T> AsIR<T> for GradientNormals
where
    T: TypeSpec,
    T::NUMBER: One + IntoValue<T>,
{
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        vec![FunctionDefinition {
            id: GRADIENT_NORMALS.specialize(spec),
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: if spec.contains(GRADIENT_2D.id()) {
                [
                    GRADIENT_2D.bind([CONTEXT, GRADIENT_2D].read().normalize()),
                    [CONTEXT, NORMAL].write(
                        Expr::vector3(
                            [GRADIENT_2D, X].read(),
                            [GRADIENT_2D, Y].read(),
                            T::NUMBER::ONE.literal(),
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
                panic!("No gradient domain")
            },
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: crate::ir::ast::Expr<T>,
    ) -> crate::ir::ast::Expr<T> {
        GRADIENT_NORMALS.specialize(spec).call(input)
    }
}
