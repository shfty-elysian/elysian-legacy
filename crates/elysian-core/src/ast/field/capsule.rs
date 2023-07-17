use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ast::expr::Expr;
use crate::ast::post_modifier::isosurface::{Isosurface, ISOSURFACE};

use crate::ast::field::Line;
use crate::ir::as_ir::{clone_ir, hash_ir, AsIR};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Capsule<N, V> {
    pub dir: Expr<N, V>,
    pub radius: Expr<N, V>,
}

impl<N, V> Hash for Capsule<N, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.radius.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Capsule<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<N, V>> {
        Line {
            dir: self.dir.clone(),
        }
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

    fn expressions(&self, input: crate::ir::ast::Property) -> Vec<crate::ir::ast::Expr<N, V>> {
        Line {
            dir: self.dir.clone(),
        }
        .expressions(input.clone())
        .into_iter()
        .chain([crate::ir::ast::Expr::Call {
            function: ISOSURFACE,
            args: vec![self.radius.clone().into(), input.read()],
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
