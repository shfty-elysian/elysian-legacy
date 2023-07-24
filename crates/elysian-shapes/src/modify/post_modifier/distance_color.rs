use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::{Modify, CONTEXT_STRUCT},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Expr, Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite, COLOR, CONTEXT,
            DISTANCE, VECTOR4_STRUCT, W, X, Y, Z,
        },
        module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData},
    },
};

pub const DISTANCE_COLOR: Identifier = Identifier::new("distance_color", 17678775431864410174);

#[derive(Debug, Clone, Hash)]
pub struct DistanceColor;

impl Domains for DistanceColor {}

impl AsIR for DistanceColor {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let block = [
            DISTANCE.bind([CONTEXT, DISTANCE].read()),
            DISTANCE.write(
                (1.0.literal() - DISTANCE.read())
                    .min(1.0.literal())
                    .max(0.0.literal()),
            ),
            [CONTEXT, COLOR].write(Expr::Struct(
                VECTOR4_STRUCT,
                [
                    (X, DISTANCE.read()),
                    (Y, DISTANCE.read()),
                    (Z, DISTANCE.read()),
                    (W, 1.0.literal()),
                ]
                .into_iter()
                .collect(),
            )),
            CONTEXT.read().output(),
        ]
        .block();

        vec![FunctionDefinition {
            id: DISTANCE_COLOR.specialize(spec),
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: CONTEXT_STRUCT.clone(),
            block,
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        DISTANCE_COLOR.specialize(spec).call(input)
    }
}

pub trait IntoDistanceColor {
    fn distance_color(self) -> Modify;
}

impl<T> IntoDistanceColor for T
where
    T: AsModule,
{
    fn distance_color(self) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: vec![Box::new(DistanceColor)],
        }
    }
}
