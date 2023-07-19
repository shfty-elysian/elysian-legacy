use std::{fmt::Debug, hash::Hash};

use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::AsIR,
        ast::{
            Identifier, IntoBlock, IntoRead, IntoWrite, TypeSpec, CONTEXT, DISTANCE, GRADIENT_2D,
            NUM,
        },
        module::{FunctionDefinition, InputDefinition},
    },
};

pub const MANIFOLD: Identifier = Identifier::new("manifold", 7861274791729269697);

#[derive(Debug, Clone, Hash)]
pub struct Manifold;

impl<T> AsIR<T> for Manifold
where
    T: TypeSpec,
{
    fn functions(&self) -> Vec<FunctionDefinition<T>> {
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
                [CONTEXT, GRADIENT_2D].write([CONTEXT, GRADIENT_2D].read() * NUM.read().sign()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<T>) -> crate::ir::ast::Expr<T> {
        MANIFOLD.call(input)
    }
}
