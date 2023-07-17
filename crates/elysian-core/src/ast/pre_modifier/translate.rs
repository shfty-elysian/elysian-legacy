use std::{fmt::Debug, hash::Hash};

use crate::ir::{
    as_ir::{clone_ir, hash_ir, AsIR},
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, CONTEXT, POSITION, VECT},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition},
};

use crate::ast::expr::Expr;

pub const TRANSLATE: Identifier = Identifier::new("translate", 419357041369711478);

#[derive(Debug, Clone)]
pub struct Translate<N, V> {
    pub delta: Expr<N, V>,
}

impl<N, V> Hash for Translate<N, V>
where
    N: 'static,
    V: 'static,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.delta.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Translate<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<FunctionDefinition<N, V>> {
        vec![FunctionDefinition {
            id: TRANSLATE,
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: VECT,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, POSITION].write([CONTEXT, POSITION].read() - VECT.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expressions(&self, input: crate::ir::ast::Expr<N, V>) -> Vec<crate::ir::ast::Expr<N, V>> {
        vec![crate::ir::ast::Expr::Call {
            function: TRANSLATE,
            args: vec![self.delta.clone().into(), input],
        }]
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}
