use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::{Circle, CONTEXT_STRUCT},
        modify::{Isosurface, Manifold, ISOSURFACE, MANIFOLD},
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, Property, TypeSpec, CONTEXT},
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use super::{CIRCLE, RADIUS};

pub const RING: Identifier = Identifier::new("ring", 18972348581943461950);
pub const WIDTH: Property = Property::new("width", Type::Number, 2742125101201765597);

pub struct Ring<T>
where
    T: TypeSpec,
{
    pub radius: Expr<T>,
    pub width: Expr<T>,
}

impl<T> Debug for Ring<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ring")
            .field("radius", &self.radius)
            .field("width", &self.width)
            .finish()
    }
}

impl<T> Clone for Ring<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            radius: self.radius.clone(),
            width: self.width.clone(),
        }
    }
}

impl<T> Hash for Ring<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
        self.width.hash(state);
    }
}

impl<T> AsIR<T> for Ring<T>
where
    T: TypeSpec,
{
    fn functions(&self, spec: &SpecializationData) -> Vec<crate::ir::module::FunctionDefinition<T>> {
        Circle {
            radius: self.radius.clone(),
        }
        .functions(spec)
        .into_iter()
        .chain(Manifold.functions(spec))
        .chain(
            Isosurface {
                dist: self.width.clone(),
            }
            .functions(spec),
        )
        .chain(FunctionDefinition {
            id: RING,
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: RADIUS,
                    mutable: false,
                },
                InputDefinition {
                    prop: WIDTH,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                },
            ],
            output: CONTEXT_STRUCT,
            block: ISOSURFACE
                .call([
                    WIDTH.read(),
                    MANIFOLD.call(CIRCLE.call([RADIUS.read(), CONTEXT.read()])),
                ])
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
        RING.call([self.radius.clone().into(), self.width.clone().into(), input])
    }
}
