use std::{fmt::Debug, hash::Hash};

use crate::ir::{
    as_ir::AsIR,
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, TypeSpec, CONTEXT, DISTANCE, GRADIENT, NUM, VectorSpace},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition},
};

pub const MANIFOLD: Identifier = Identifier::new("manifold", 7861274791729269697);

#[derive(Debug, Clone, Hash)]
pub struct Manifold;

impl<T, const N: usize> AsIR<T, N> for Manifold
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<FunctionDefinition<T, N>> {
        vec![FunctionDefinition {
            id: MANIFOLD,
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: [
                NUM.write([CONTEXT, DISTANCE].read()),
                [CONTEXT, DISTANCE].write(NUM.read().abs()),
                [CONTEXT, GRADIENT].write([CONTEXT, GRADIENT].read() * NUM.read().sign()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<T, N>) -> crate::ir::ast::Expr<T, N> {
        crate::ir::ast::Expr::Call {
            function: MANIFOLD,
            args: vec![input],
        }
    }
}
