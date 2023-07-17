use std::{fmt::Debug, hash::Hash};

use crate::ir::{
    as_ir::AsIR,
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, POSITION},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition, Type},
};

use crate::ast::expr::Expr;

pub const TRANSLATE: Identifier = Identifier::new("translate", 419357041369711478);
pub const DELTA: Property = Property::new("delta", Type::Vector, 1292788437813720044);

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
                    prop: DELTA,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, POSITION].write([CONTEXT, POSITION].read() - DELTA.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<N, V>) -> crate::ir::ast::Expr<N, V> {
        crate::ir::ast::Expr::Call {
            function: TRANSLATE,
            args: vec![self.delta.clone().into(), input],
        }
    }
}
