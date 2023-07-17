use std::hash::Hash;

use crate::ir::{
    as_ir::{clone_ir, hash_ir},
    ast::{
        Expr, Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, DISTANCE, GRADIENT,
        POSITION,
    },
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition},
};

use super::AsIR;

pub const POINT: Identifier = Identifier::new("point", 419357041369711478);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl<N, V> AsIR<N, V> for Point {
    fn functions(&self) -> Vec<FunctionDefinition<N, V>> {
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

    fn expressions(&self, input: Property) -> Vec<Expr<N, V>> {
        vec![Expr::Call {
            function: POINT,
            args: vec![input.read()],
        }]
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}