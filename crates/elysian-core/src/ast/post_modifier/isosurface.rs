use std::{fmt::Debug, hash::Hash};

use crate::ir::{
    as_ir::AsIR,
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, DISTANCE},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition, Type},
};

use crate::ast::expr::Expr;

pub const ISOSURFACE: Identifier = Identifier::new("isosurface", 1163045471729794054);
pub const DIST: Property = Property::new("property", Type::Number, 463524741302033362);

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
                    prop: DIST,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, DISTANCE].read() - DIST.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<N, V>) -> crate::ir::ast::Expr<N, V> {
        crate::ir::ast::Expr::Call {
            function: ISOSURFACE,
            args: vec![self.dist.clone().into(), input],
        }
    }
}
