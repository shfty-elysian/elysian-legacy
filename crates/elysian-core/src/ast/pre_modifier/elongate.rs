use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    as_ir::AsIR,
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, POSITION},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition, Type},
};

use crate::ast::expr::Expr;

pub const ELONGATE: Identifier = Identifier::new("elongate", 1022510703206415324);
pub const ELONGATE_INFINITE: Identifier = Identifier::new("elongate_infinite", 1799909959882308009);
pub const DIR: Property = Property::new("dir", Type::Vector, 10994004961423687819);

#[derive(Debug, Clone)]
pub struct Elongate<N, V> {
    pub dir: Expr<N, V>,
    pub infinite: bool,
}

impl<N, V> Hash for Elongate<N, V>
where
    N: 'static,
    V: 'static,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.infinite.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Elongate<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<FunctionDefinition<N, V>> {
        vec![FunctionDefinition {
            id: if self.infinite {
                ELONGATE_INFINITE
            } else {
                ELONGATE
            },
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: DIR,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: {
                let expr = [CONTEXT, POSITION].read().dot(DIR.read().normalize());

                [
                    [CONTEXT, POSITION].write(
                        [CONTEXT, POSITION].read()
                            - DIR.read().normalize()
                                * if self.infinite {
                                    expr
                                } else {
                                    expr.max(-DIR.read().length()).min(DIR.read().length())
                                },
                    ),
                    CONTEXT.read().output(),
                ]
                .block()
            },
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<N, V>) -> crate::ir::ast::Expr<N, V> {
        crate::ir::ast::Expr::Call {
            function: if self.infinite {
                ELONGATE_INFINITE
            } else {
                ELONGATE
            },
            args: vec![self.dir.clone().into(), input],
        }
    }
}
