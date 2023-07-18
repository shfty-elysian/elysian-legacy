use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    as_ir::AsIR,
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, Property, TypeSpec, CONTEXT, POSITION, VectorSpace},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition, Type},
};

use crate::ast::expr::Expr;

pub const ELONGATE: Identifier = Identifier::new("elongate", 1022510703206415324);
pub const ELONGATE_INFINITE: Identifier = Identifier::new("elongate_infinite", 1799909959882308009);
pub const DIR: Property = Property::new("dir", Type::Vector2, 10994004961423687819);

pub struct Elongate<T>
where
    T: TypeSpec,
{
    pub dir: Expr<T>,
    pub infinite: bool,
}

impl<T> Debug for Elongate<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Elongate")
            .field("dir", &self.dir)
            .field("infinite", &self.infinite)
            .finish()
    }
}

impl<T> Clone for Elongate<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            infinite: self.infinite.clone(),
        }
    }
}

impl<T> Hash for Elongate<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.infinite.hash(state);
    }
}

impl<T, const N: usize> AsIR<T, N> for Elongate<T>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<FunctionDefinition<T, N>> {
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

    fn expression(&self, input: crate::ir::ast::Expr<T, N>) -> crate::ir::ast::Expr<T, N> {
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
