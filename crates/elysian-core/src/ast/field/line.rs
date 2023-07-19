use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::{Point, CONTEXT_STRUCT},
        modify::{Elongate, DIR, ELONGATE},
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, TypeSpec, VectorSpace, CONTEXT},
        module::{FunctionDefinition, InputDefinition},
    },
};

use super::POINT;

pub const LINE: Identifier = Identifier::new("line", 14339483921749952476);

pub struct Line<T>
where
    T: TypeSpec,
{
    pub dir: Expr<T>,
}

impl<T> Debug for Line<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line").field("dir", &self.dir).finish()
    }
}

impl<T> Clone for Line<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
        }
    }
}

impl<T> Hash for Line<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
    }
}

impl<T, const N: usize> AsIR<T, N> for Line<T>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<T, N>> {
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

    fn expression(&self, input: crate::ir::ast::Expr<T, N>) -> crate::ir::ast::Expr<T, N> {
        crate::ir::ast::Expr::Call {
            function: LINE,
            args: vec![self.dir.clone().into(), input],
        }
    }
}
