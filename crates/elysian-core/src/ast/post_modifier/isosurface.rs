use std::{fmt::Debug, hash::Hash};

use crate::ir::{
    as_ir::{clone_ir, hash_ir, AsIR},
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, DISTANCE, NUM},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition},
};

use crate::ast::expr::Expr;

pub const ISOSURFACE: Identifier = Identifier::new("isosurface", 1163045471729794054);

#[derive(Debug, Clone)]
pub struct Isosurface<N, V> {
    pub dist: Expr<N, V>,
}

impl<N, V> Hash for Isosurface<N, V>
where
    N: 'static,
    V: 'static,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dist.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Isosurface<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<FunctionDefinition<N, V>> {
        vec![FunctionDefinition {
            id: ISOSURFACE,
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: NUM,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, DISTANCE].read() - NUM.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expressions(&self, input: Property) -> Vec<crate::ir::ast::Expr<N, V>> {
        vec![crate::ir::ast::Expr::Call {
            function: ISOSURFACE,
            args: vec![self.dist.clone().into(), input.read()],
        }]
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}
