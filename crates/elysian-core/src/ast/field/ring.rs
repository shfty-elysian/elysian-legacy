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
        ast::{Identifier, IntoBlock, Property, CONTEXT},
        from_elysian::CONTEXT_STRUCT,
        module::{FunctionDefinition, InputDefinition, Type},
    },
};

use super::{CIRCLE, RADIUS};

pub const RING: Identifier = Identifier::new("ring", 18972348581943461950);
pub const WIDTH: Property = Property::new("width", Type::Number, 2742125101201765597);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ring<N, V> {
    pub radius: Expr<N, V>,
    pub width: Expr<N, V>,
}

impl<N, V> Hash for Ring<N, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
        self.width.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Ring<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<N, V>> {
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

    fn expression(&self, input: crate::ir::ast::Expr<N, V>) -> crate::ir::ast::Expr<N, V> {
        crate::ir::ast::Expr::Call {
            function: RING,
            args: vec![self.radius.clone().into(), self.width.clone().into(), input],
        }
    }
}
