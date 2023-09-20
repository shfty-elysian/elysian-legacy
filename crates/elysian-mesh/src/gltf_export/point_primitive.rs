use crate::vector_space::VectorSpace;

use super::{Indices, Points};

#[derive(Clone)]
pub struct PointPrimitive<D>
where
    D: VectorSpace<f64>,
{
    pub points: Points<D>,
    pub indices: Option<Indices>,
}
