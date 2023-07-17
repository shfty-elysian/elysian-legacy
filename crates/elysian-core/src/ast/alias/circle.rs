use std::fmt::Debug;

use crate::ast::{
    expand::Expand,
    expr::Expr,
    field::{IntoField, PointField},
    Elysian,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Circle<N, V> {
    pub radius: Expr<N, V>,
}

impl<N, V> Expand<N, V> for Circle<N, V>
where
    N: Debug + Clone,
    V: Debug + Clone,
{
    fn expand(&self) -> Elysian<N, V> {
        PointField.field().isosurface(self.radius.clone())
    }
}
