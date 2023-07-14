use std::fmt::Debug;

use crate::elysian::{expand::Expand, expr::Expr, Elysian};

use super::Line;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capsule<N, V> {
    pub dir: Expr<N, V>,
    pub radius: Expr<N, V>,
}

impl<N, V> Expand<N, V> for Capsule<N, V>
where
    N: Debug + Clone,
    V: Debug + Clone,
{
    fn expand(&self) -> Elysian<N, V> {
        Line {
            dir: self.dir.clone(),
        }
        .expand()
        .isosurface(self.radius.clone())
    }
}

