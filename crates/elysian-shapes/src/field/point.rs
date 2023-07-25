use std::hash::Hash;

use elysian_core::ir::{
    as_ir::{AsIR, Domains},
    ast::{
        Expr, Identifier, IntoBlock, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D,
    },
    module::{
        FunctionDefinition, InputDefinition, IntoRead, IntoWrite, PropertyIdentifier,
        SpecializationData, CONTEXT_PROP,
    },
};

pub const POINT: Identifier = Identifier::new("point", 2023836058494613125);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl Domains for Point {
    fn domains() -> Vec<PropertyIdentifier> {
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
            block.push(
                [CONTEXT_PROP, DISTANCE].write([CONTEXT_PROP, position.clone()].read().length()),
            )
        };

        if let Some(gradient) = gradient {
            block.push([CONTEXT_PROP, gradient].write([CONTEXT_PROP, position].read().normalize()));
        }

        block.push(CONTEXT_PROP.read().output());

        vec![FunctionDefinition {
            id: POINT.specialize(&spec),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT_PROP,
                mutable: true,
            }],
            output: CONTEXT_PROP,
            block: block.block(),
        }]
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr {
        POINT.specialize(spec).call(input)
    }
}
