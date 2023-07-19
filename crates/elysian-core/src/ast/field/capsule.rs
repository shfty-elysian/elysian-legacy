use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ir::module::SpecializationData;
use crate::{
    ast::{
        expr::Expr,
        field::{Line, CONTEXT_STRUCT},
        modify::{Isosurface, DIR_2D, ISOSURFACE},
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, TypeSpec, CONTEXT},
        module::{FunctionDefinition, InputDefinition},
    },
};

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

impl<T> AsIR<T> for Capsule<T>
where
    T: TypeSpec,
{
    fn functions(&self, spec: &SpecializationData) -> Vec<crate::ir::module::FunctionDefinition<T>> {
        Line {
            dir: self.dir.clone(),
        }
        .functions(spec)
        .into_iter()
        .chain(
            Isosurface {
                dist: self.radius.clone(),
            }
            .functions(spec),
        )
        .chain(FunctionDefinition {
            id: CAPSULE,
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: DIR_2D,
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
            block: ISOSURFACE
                .call([RADIUS.read(), LINE.call([DIR_2D.read(), CONTEXT.read()])])
                .output()
                .block(),
        })
        .collect()
    }

    fn expression(
        &self,
        _: &SpecializationData,
        input: crate::ir::ast::Expr<T>,
    ) -> crate::ir::ast::Expr<T> {
        CAPSULE.call([self.dir.clone().into(), self.radius.clone().into(), input])
    }
}
