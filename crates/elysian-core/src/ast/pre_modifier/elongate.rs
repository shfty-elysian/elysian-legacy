use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    as_ir::{clone_ir, hash_ir, AsIR},
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, CONTEXT, POSITION, VECT},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition},
};

use crate::ast::expr::Expr;

pub const ELONGATE: Identifier = Identifier::new("elongate", 1022510703206415324);
pub const ELONGATE_INFINITE: Identifier = Identifier::new("elongate_infinite", 1799909959882308009);

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
                    prop: VECT,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: {
                let expr = [CONTEXT, POSITION].read().dot(VECT.read().normalize());

                [
                    [CONTEXT, POSITION].write(
                        [CONTEXT, POSITION].read()
                            - VECT.read().normalize()
                                * if self.infinite {
                                    expr
                                } else {
                                    expr.max(-VECT.read().length()).min(VECT.read().length())
                                },
                    ),
                    CONTEXT.read().output(),
                ]
                .block()
            },
        }]
    }

    fn expressions(&self, input: crate::ir::ast::Property) -> Vec<crate::ir::ast::Expr<N, V>> {
        vec![crate::ir::ast::Expr::Call {
            function: if self.infinite {
                ELONGATE_INFINITE
            } else {
                ELONGATE
            },
            args: vec![self.dir.clone().into(), input.read()],
        }]
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}
