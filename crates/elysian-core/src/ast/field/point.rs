use std::hash::Hash;

use crate::ir::{
    ast::{
        Expr, Identifier, IntoBlock, IntoRead, IntoWrite, TypeSpec, CONTEXT, DISTANCE, GRADIENT,
        POSITION, VectorSpace,
    },
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition},
};

use super::AsIR;

pub const POINT: Identifier = Identifier::new("point", 419357041369711478);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl<T, const N: usize> AsIR<T, N> for Point
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<FunctionDefinition<T, N>> {
        vec![FunctionDefinition {
            id: POINT,
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, POSITION].read().length()),
                [CONTEXT, GRADIENT].write([CONTEXT, POSITION].read().normalize()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: Expr<T, N>) -> Expr<T, N> {
        Expr::Call {
            function: POINT,
            args: vec![input],
        }
    }
}
