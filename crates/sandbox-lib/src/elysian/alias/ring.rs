use std::fmt::Debug;

use crate::elysian::{alias::Circle, expand::Expand, expr::Expr, Elysian};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ring<N, V> {
    pub radius: Expr<N, V>,
    pub width: Expr<N, V>,
}

impl<N, V> Expand<N, V> for Ring<N, V>
where
    N: Debug + Clone,
    V: Debug + Clone,
{
    fn expand(&self) -> Elysian<N, V> {
        Circle {
            radius: self.radius.clone(),
        }
        .expand()
        .manifold()
        .isosurface(self.width.clone())
    }
}
