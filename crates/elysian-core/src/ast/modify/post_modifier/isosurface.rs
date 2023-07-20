use std::{fmt::Debug, hash::Hash};

use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{Identifier, IntoBlock, IntoRead, IntoBind, Property, TypeSpec, CONTEXT, DISTANCE},
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use crate::ast::expr::Expr;

pub const ISOSURFACE: Identifier = Identifier::new("isosurface", 1163045471729794054);
pub const DIST: Property = Property::new("property", Type::Number, 463524741302033362);

pub struct Isosurface<T>
where
    T: TypeSpec,
{
    pub dist: Expr<T>,
}

impl<T> Debug for Isosurface<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Isosurface")
            .field("dist", &self.dist)
            .finish()
    }
}

impl<T> Clone for Isosurface<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            dist: self.dist.clone(),
        }
    }
}

impl<T> Hash for Isosurface<T>
where
    T: TypeSpec,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dist.hash(state);
    }
}

impl<T> FilterSpec for Isosurface<T> where T: TypeSpec {}

impl<T> AsIR<T> for Isosurface<T>
where
    T: TypeSpec,
{
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        vec![FunctionDefinition {
            id: ISOSURFACE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: DIST,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].bind([CONTEXT, DISTANCE].read() - DIST.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: crate::ir::ast::Expr<T>,
    ) -> crate::ir::ast::Expr<T> {
        ISOSURFACE
            .specialize(spec)
            .call([self.dist.clone().into(), input])
    }
}
