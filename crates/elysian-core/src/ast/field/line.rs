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
        ast::{Identifier, IntoBlock, TypeSpec, CONTEXT},
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

impl<T> AsIR<T> for Line<T>
where
    T: TypeSpec,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<T>> {
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
            .chain(FunctionDefinition {
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
                block: POINT
                    .call(ELONGATE.call([DIR.read(), CONTEXT.read()]))
                    .output()
                    .block(),
            })
            .collect()
    }

    fn expression(&self, input: crate::ir::ast::Expr<T>) -> crate::ir::ast::Expr<T> {
        LINE.call([self.dir.clone().into(), input])
    }
}
