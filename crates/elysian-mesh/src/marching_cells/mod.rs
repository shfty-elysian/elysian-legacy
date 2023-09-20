use std::collections::BTreeMap;

use crate::vector_space::VectorSpace;

mod corner;
mod corners;
mod edge;
mod edges;
mod face;
mod faces;
mod local_point;
mod march_primitive;
mod march_primitives;
mod marching_cubes;
mod marching_squares;
mod power_2_usize;
mod zero_point;

pub use corner::*;
pub use corners::*;
pub use edge::*;
pub use edges::*;
pub use face::*;
pub use faces::*;
pub use local_point::*;
pub use march_primitive::*;
pub use march_primitives::*;
pub use marching_cubes::*;
pub use marching_squares::*;
pub use power_2_usize::*;
pub use zero_point::*;

pub trait All {
    fn all() -> Self;
}

pub trait Cell<D>: ToEdges<D> {
    fn cell(&self) -> BTreeMap<Corner<D>, Edges<D>>;
}

pub trait Point<D: VectorSpace<f64>> {
    fn point(&self) -> D::DimensionArray<f64>;
}
