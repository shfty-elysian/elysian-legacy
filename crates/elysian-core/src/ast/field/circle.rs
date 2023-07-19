use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::{point::Point, CONTEXT_STRUCT},
        modify::{Isosurface, ISOSURFACE},
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, Property, TypeSpec, CONTEXT},
        module::{FunctionDefinition, InputDefinition, Type, SpecializationData},
    },
};

use super::POINT;

pub const CIRCLE: Identifier = Identifier::new("circle", 15738477621793375359);
pub const RADIUS: Property = Property::new("radius", Type::Number, 213754678517975478);

pub struct Circle<T>
where
    T: TypeSpec,
{
    pub radius: Expr<T>,
}

impl<T> Debug for Circle<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Circle")
            .field("radius", &self.radius)
            .finish()
    }
}

impl<T> Clone for Circle<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            radius: self.radius.clone(),
        }
    }
}

impl<T> Hash for Circle<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
    }
}

impl<T> AsIR<T> for Circle<T>
where
    T: TypeSpec,
{
    fn functions(&self, spec: &SpecializationData) -> Vec<crate::ir::module::FunctionDefinition<T>> {
        Point
            .functions(spec)
            .into_iter()
            .chain(
                Isosurface {
                    dist: self.radius.clone(),
                }
                .functions(spec),
            )
            .chain(FunctionDefinition {
                id: CIRCLE,
                public: false,
                inputs: vec![
                    InputDefinition {
                        prop: RADIUS,
                        mutable: false,
                    },
                    InputDefinition {
                        prop: CONTEXT,
                        mutable: false,
                    },
                ],
                output: &CONTEXT_STRUCT,
                block: ISOSURFACE
                    .call([RADIUS.read(), POINT.call(CONTEXT.read())])
                    .output()
                    .block(),
            })
            .collect()
    }

    fn expression(&self, _: &SpecializationData, input: crate::ir::ast::Expr<T>) -> crate::ir::ast::Expr<T> {
        CIRCLE.call([self.radius.clone().into(), input])
    }
}
