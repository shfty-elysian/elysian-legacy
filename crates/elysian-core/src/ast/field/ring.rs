use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::Circle,
        post_modifier::{isosurface::Isosurface, manifold::Manifold},
    },
    ir::as_ir::{clone_ir, hash_ir, AsIR},
};

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
        .collect()
    }

    fn expressions(&self, input: crate::ir::ast::Property) -> Vec<crate::ir::ast::Expr<N, V>> {
        Circle {
            radius: self.radius.clone(),
        }
        .expressions(input.clone())
        .into_iter()
        .chain(Manifold.expressions(input.clone()))
        .chain(
            Isosurface {
                dist: self.width.clone(),
            }
            .expressions(input),
        )
        .collect()
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}
