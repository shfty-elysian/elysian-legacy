use std::hash::Hash;

use crate::{
    ast::field::CONTEXT_STRUCT,
    ir::{
        ast::{
            Expr, Identifier, IntoBlock, IntoRead, IntoWrite, TypeSpec, CONTEXT,
            DISTANCE, GRADIENT_2D, POSITION_2D,
        },
        module::{FunctionDefinition, InputDefinition},
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
    fn functions(&self) -> Vec<FunctionDefinition<T>> {
        vec![FunctionDefinition {
            id: POINT,
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, POSITION_2D].read().length()),
                [CONTEXT, GRADIENT_2D].write([CONTEXT, POSITION_2D].read().normalize()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: Expr<T>) -> Expr<T> {
        POINT.call(input)
    }
}
