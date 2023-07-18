use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::Circle,
        post_modifier::{
            isosurface::{Isosurface, ISOSURFACE},
            manifold::{Manifold, MANIFOLD},
        },
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, Property, TypeSpec, CONTEXT, VectorSpace},
        from_elysian::CONTEXT_STRUCT,
        module::{FunctionDefinition, InputDefinition, Type},
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

impl<T, const N: usize> AsIR<T, N> for Ring<T>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<T, N>> {
        Circle {
            radius: self.radius.clone(),
        }
        .functions()
        .into_iter()
        .chain(Manifold.functions())
        .chain(
            Isosurface {
                dist: self.width.clone(),
            }
            .functions(),
        )
        .chain([FunctionDefinition {
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
            block: [crate::ir::ast::Expr::Call {
                function: ISOSURFACE,
                args: vec![
                    WIDTH.read(),
                    crate::ir::ast::Expr::Call {
                        function: MANIFOLD,
                        args: vec![crate::ir::ast::Expr::Call {
                            function: CIRCLE,
                            args: vec![RADIUS.read(), CONTEXT.read()],
                        }],
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
            function: RING,
            args: vec![self.radius.clone().into(), self.width.clone().into(), input],
        }
    }
}
