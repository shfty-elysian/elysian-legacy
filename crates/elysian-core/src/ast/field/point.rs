use std::hash::Hash;

use crate::{
    ast::field::CONTEXT_STRUCT,
    ir::{
        ast::{
            Expr, Identifier, IntoBlock, IntoRead, IntoWrite, TypeSpec, CONTEXT, DISTANCE,
            GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

use super::AsIR;

pub const POINT: Identifier = Identifier::new("point", 419357041369711478);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl<T> AsIR<T> for Point
where
    T: TypeSpec,
{
    fn functions(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        let position = if spec.domains.contains(&POSITION_2D) {
            POSITION_2D
        } else if spec.domains.contains(&POSITION_3D) {
            POSITION_3D
        } else {
            panic!("No position domain set")
        };

        let distance = spec.domains.contains(&DISTANCE);

        let gradient = if spec.domains.contains(&POSITION_2D) {
            Some(GRADIENT_2D)
        } else if spec.domains.contains(&POSITION_3D) {
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
            id: POINT,
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: block.block(),
        }]
    }

    fn expression(&self, _: &SpecializationData, input: Expr<T>) -> Expr<T> {
        POINT.call(input)
    }
}
