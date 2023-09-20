use crate::{bounds::Bounds, vector_space::VectorSpace};

use super::Point;

/// Given a Bounds, return a point transformed into its local space
pub trait LocalPoint<D: VectorSpace<f64>>: Point<D> {
    fn local_point(self, bounds: Bounds<D>) -> D::DimensionVector;
}

impl<D, T> LocalPoint<D> for T
where
    T: Point<D>,
    D: VectorSpace<f64>,
    D::DimensionVector: From<D::DimensionArray<f64>>,
{
    fn local_point(self, bounds: Bounds<D>) -> D::DimensionVector {
        let p = self.point();
        let p = D::component_add(&D::component_mul(&p.into(), &D::splat(0.5)), &D::splat(0.5));
        let ofs = D::component_mul(&p, &bounds.size());

        D::component_add(&bounds.min, &ofs)
    }
}
