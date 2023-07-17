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
        as_ir::{clone_ir, hash_ir, AsIR},
        ast::Identifier,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Circle<N, V> {
    pub radius: Expr<N, V>,
}

impl<N, V> Hash for Circle<N, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
    }
}

pub const CIRCLE: Identifier = Identifier::new("circle", 15738477621793375359);

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
                .functions()
                .into_iter(),
            )
            .collect()
    }

    fn expressions(&self, input: crate::ir::ast::Expr<N, V>) -> Vec<crate::ir::ast::Expr<N, V>> {
        Point
            .expressions(input.clone())
            .into_iter()
            .chain([crate::ir::ast::Expr::Call {
                function: ISOSURFACE,
                args: vec![self.radius.clone().into(), input],
            }])
            .collect()
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}
