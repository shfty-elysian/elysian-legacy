use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ast::expr::Expr;
use crate::ast::post_modifier::isosurface::{Isosurface, ISOSURFACE};

use crate::ast::field::Line;
use crate::ast::pre_modifier::elongate::DIR;
use crate::ir::as_ir::AsIR;
use crate::ir::ast::{Identifier, IntoBlock, TypeSpec, CONTEXT, VectorSpace};
use crate::ir::from_elysian::CONTEXT_STRUCT;
use crate::ir::module::{FunctionDefinition, InputDefinition};

use super::{LINE, RADIUS};

pub const CAPSULE: Identifier = Identifier::new("capsule", 14339483921749952476);

pub struct Capsule<T>
where
    T: TypeSpec,
{
    pub dir: Expr<T>,
    pub radius: Expr<T>,
}

impl<T> Debug for Capsule<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capsule")
            .field("dir", &self.dir)
            .field("radius", &self.radius)
            .finish()
    }
}

impl<T> Clone for Capsule<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            radius: self.radius.clone(),
        }
    }
}

impl<T> Hash for Capsule<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.radius.hash(state);
    }
}

impl<T, const N: usize> AsIR<T, N> for Capsule<T>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<T, N>> {
        Line {
            dir: self.dir.clone(),
        }
        .functions()
        .into_iter()
        .chain(
            Isosurface {
                dist: self.radius.clone(),
            }
            .functions(),
        )
        .chain([FunctionDefinition {
            id: CAPSULE,
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: DIR,
                    mutable: false,
                },
                InputDefinition {
                    prop: RADIUS,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                },
            ],
            output: CONTEXT_STRUCT,
            block: [crate::ir::ast::Expr::Call {
                function: ISOSURFACE,
                args: vec![
                    RADIUS.read(),
                    crate::ir::ast::Expr::Call {
                        function: LINE,
                        args: vec![DIR.read(), CONTEXT.read()],
                    },
                ],
            }
            .output()]
            .block(),
        }])
        .collect()
    }

    fn expression(&self, input: crate::ir::ast::Expr<T, N>) -> crate::ir::ast::Expr<T, N> {
        crate::ir::ast::Expr::Call {
            function: CAPSULE,
            args: vec![self.dir.clone().into(), self.radius.clone().into(), input],
        }
    }
}
