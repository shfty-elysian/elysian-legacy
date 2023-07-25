use std::hash::Hash;

use elysian_core::ir::{
    as_ir::{AsIR, Domains},
    ast::{
        Expr, Identifier, IntoBlock, IntoRead, IntoWrite, DISTANCE, GRADIENT_2D, GRADIENT_3D,
        POSITION_2D, POSITION_3D,
    },
    module::{FunctionDefinition, InputDefinition, SpecializationData, CONTEXT},
};

pub const POINT: Identifier = Identifier::new("point", 2023836058494613125);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl Domains for Point {
    fn domains() -> Vec<Identifier> {
        vec![POSITION_2D, POSITION_3D, DISTANCE, GRADIENT_2D, GRADIENT_3D]
    }
}

impl AsIR for Point {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let position = if spec.contains(&POSITION_2D) {
            POSITION_2D
        } else if spec.contains(&POSITION_3D) {
            POSITION_3D
        } else {
            panic!("No position domain set")
        };

        let distance = spec.contains(&DISTANCE);

        let gradient = if spec.contains(&POSITION_2D) {
            Some(GRADIENT_2D)
        } else if spec.contains(&POSITION_3D) {
            Some(GRADIENT_3D)
        } else {
            None
        };

        let mut block = vec![];
        if distance {
            block.push([CONTEXT, DISTANCE].write([CONTEXT, position.clone()].read().length()))
        };

        if let Some(gradient) = gradient {
            block.push([CONTEXT, gradient].write([CONTEXT, position].read().normalize()));
        }

        block.push(CONTEXT.read().output());

        vec![FunctionDefinition {
            id: POINT.specialize(&spec),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT,
                mutable: true,
            }],
            output: CONTEXT,
            block: block.block(),
        }]
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr {
        POINT.specialize(spec).call(input)
    }
}
