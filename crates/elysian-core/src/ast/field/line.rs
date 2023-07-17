use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{expr::Expr, field::Point, pre_modifier::elongate::Elongate},
    ir::as_ir::{clone_ir, hash_ir, AsIR},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line<N, V> {
    pub dir: Expr<N, V>,
}

impl<N, V> Hash for Line<N, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Line<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<N, V>> {
        Point
            .functions()
            .into_iter()
            .chain(
                Elongate {
                    dir: self.dir.clone(),
                    infinite: false,
                }
                .functions(),
            )
            .collect()
    }

    fn expressions(&self, input: crate::ir::ast::Expr<N, V>) -> Vec<crate::ir::ast::Expr<N, V>> {
        Elongate {
            dir: self.dir.clone(),
            infinite: false,
        }
        .expressions(input.clone())
        .into_iter()
        .chain(Point.expressions(input))
        .collect()
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}
