use std::{fmt::Debug, hash::Hash};

use crate::ir::{
    as_ir::AsIR,
    ast::{Identifier, IntoBlock, IntoRead, IntoWrite, Property, TypeSpec, CONTEXT, DISTANCE, VectorSpace},
    from_elysian::CONTEXT_STRUCT,
    module::{FunctionDefinition, InputDefinition, Type},
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

impl<T, const N: usize> AsIR<T, N> for Isosurface<T>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<FunctionDefinition<T, N>> {
        vec![FunctionDefinition {
            id: ISOSURFACE,
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
                [CONTEXT, DISTANCE].write([CONTEXT, DISTANCE].read() - DIST.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<T, N>) -> crate::ir::ast::Expr<T, N> {
        crate::ir::ast::Expr::Call {
            function: ISOSURFACE,
            args: vec![self.dist.clone().into(), input],
        }
    }
}
