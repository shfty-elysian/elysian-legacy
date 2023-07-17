use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::point::Point,
        post_modifier::isosurface::{Isosurface, ISOSURFACE},
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, Property, CONTEXT},
        from_elysian::CONTEXT_STRUCT,
        module::{FunctionDefinition, InputDefinition, Type},
    },
};

use super::POINT;

pub const CIRCLE: Identifier = Identifier::new("circle", 15738477621793375359);
pub const RADIUS: Property = Property::new("radius", Type::Number, 213754678517975478);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Circle<N, V> {
    pub radius: Expr<N, V>,
}

impl<N, V> Hash for Circle<N, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Circle<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<N, V>> {
        Point
            .functions()
            .into_iter()
            .chain(
                Isosurface {
                    dist: self.radius.clone(),
                }
                .functions(),
            )
            .chain([FunctionDefinition {
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
                block: [crate::ir::ast::Expr::Call {
                    function: ISOSURFACE,
                    args: vec![
                        RADIUS.read(),
                        crate::ir::ast::Expr::Call {
                            function: POINT,
                            args: vec![CONTEXT.read()],
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
            function: CIRCLE,
            args: vec![self.radius.clone().into(), input],
        }
    }
}
