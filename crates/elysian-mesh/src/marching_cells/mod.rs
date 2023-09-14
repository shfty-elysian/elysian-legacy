use std::collections::BTreeMap;

use crate::vector_space::DimensionArray;

mod corner;
mod corners;
mod edge;
mod edges;
mod face;
mod faces;
mod marching_cubes;
mod marching_squares;
mod power_2_usize;

pub use corner::*;
pub use corners::*;
pub use edge::*;
pub use edges::*;
pub use face::*;
pub use faces::*;
pub use marching_cubes::*;
pub use marching_squares::*;
pub use power_2_usize::*;

pub trait All {
    fn all() -> Self;
}

pub trait Cell<const D: usize>: ToEdges<D> {
    fn cell(&self) -> BTreeMap<Corner<D>, Edges<D>>;
}

pub trait Point<D: DimensionArray<f64>> {
    fn point(&self) -> D::DimensionArray;
}

pub trait MarchingCell {
    type Item;

    fn marching_cell(self) -> Vec<Self::Item>;
}
