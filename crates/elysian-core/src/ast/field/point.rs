use std::hash::Hash;

use crate::{
    ast::field::CONTEXT_STRUCT,
    ir::{
        as_ir::FilterSpec,
        ast::{
            Expr, Identifier, IntoBlock, IntoRead, IntoWrite, CONTEXT, DISTANCE, GRADIENT_2D,
            GRADIENT_3D, POSITION_2D, POSITION_3D,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

use super::AsIR;

pub const POINT: Identifier = Identifier::new("point", 2023836058494613125);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl FilterSpec for Point {
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        spec.filter([
            POSITION_2D.id(),
            POSITION_3D.id(),
            DISTANCE.id(),
            GRADIENT_2D.id(),
            GRADIENT_3D.id(),
        ])
    }
}

impl AsIR for Point {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let position = if spec.contains(POSITION_2D.id()) {
            POSITION_2D
        } else if spec.contains(POSITION_3D.id()) {
            POSITION_3D
        } else {
            panic!("No position domain set")
        };

        let distance = spec.contains(DISTANCE.id());

        let gradient = if spec.contains(POSITION_2D.id()) {
            Some(GRADIENT_2D)
        } else if spec.contains(POSITION_3D.id()) {
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
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: block.block(),
        }]
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr {
        POINT.specialize(spec).call(input)
    }
}
