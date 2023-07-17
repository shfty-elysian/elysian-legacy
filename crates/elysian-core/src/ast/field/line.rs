use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::Point,
        pre_modifier::elongate::{Elongate, DIR, ELONGATE},
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, CONTEXT},
        from_elysian::CONTEXT_STRUCT,
        module::{FunctionDefinition, InputDefinition},
    },
};

use super::POINT;

pub const LINE: Identifier = Identifier::new("line", 14339483921749952476);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line<N, V> {
    pub dir: Expr<N, V>,
}

impl<N, V> Hash for Line<N, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Line<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<N, V>> {
        Point
            .functions()
            .into_iter()
            .chain(
                Elongate {
                    dir: self.dir.clone(),
                    infinite: false,
                }
                .functions(),
            )
            .chain([FunctionDefinition {
                id: LINE,
                public: false,
                inputs: vec![
                    InputDefinition {
                        prop: DIR,
                        mutable: false,
                    },
                    InputDefinition {
                        prop: CONTEXT,
                        mutable: false,
                    },
                ],
                output: CONTEXT_STRUCT,
                block: [crate::ir::ast::Expr::Call {
                    function: POINT,
                    args: vec![crate::ir::ast::Expr::Call {
                        function: ELONGATE,
                        args: vec![DIR.read(), CONTEXT.read()],
                    }],
                }
                .output()]
                .block(),
            }])
            .collect()
    }

    fn expression(&self, input: crate::ir::ast::Expr<N, V>) -> crate::ir::ast::Expr<N, V> {
        crate::ir::ast::Expr::Call {
            function: LINE,
            args: vec![self.dir.clone().into(), input],
        }
    }
}
