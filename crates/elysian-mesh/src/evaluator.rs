use crate::{bounds::Bounds, marching_cells::ZeroPoint, vector_space::VectorSpace};

pub trait Evaluator<'a, D>: ZeroPoint<'a, D>
where
    D: VectorSpace<f64>,
    Bounds<D>: IntoIterator<Item = D::DimensionVector>,
{
}

impl<'a, T, D> Evaluator<'a, D> for T
where
    T: ZeroPoint<'a, D>,
    D: VectorSpace<f64>,
    Bounds<D>: IntoIterator<Item = D::DimensionVector>,
{
}
