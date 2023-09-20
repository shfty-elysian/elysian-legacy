use std::ops::Mul;

use elysian_ir::{ast::DISTANCE, module::EvaluateError};

use crate::{
    bounds::Bounds,
    sample::Sample,
    util::CollectArray,
    vector_space::{DimensionVector, VectorSpace, D2, D3},
};

use super::{Edge, LocalPoint, ToCorners};

/// Find the closest distance to zero along the provided edge
pub trait ZeroPoint<'a, D: VectorSpace<f64>>: Sample<'a, D> {
    fn zero_point(
        &self,
        edge: Edge<D>,
        bounds: Bounds<D>,
    ) -> Result<D::DimensionVector, EvaluateError>;
}

impl<'a, T> ZeroPoint<'a, D2> for T
where
    T: Sample<'a, D2>,
{
    fn zero_point(
        &self,
        edge: Edge<D2>,
        bounds: Bounds<D2>,
    ) -> Result<<D2 as DimensionVector<f64>>::DimensionVector, EvaluateError> {
        let [from, to] = edge
            .to_corners()
            .into_iter()
            .map(|t| t.local_point(bounds))
            .collect_array();

        let (from, to) = if f64::from(self.sample(from)?.get(&DISTANCE.into())) >= 0.0 {
            (to, from)
        } else {
            (from, to)
        };

        zero_impl(self, 0.5, 0.25, 10, from, to)
    }
}

impl<'a, T> ZeroPoint<'a, D3> for T
where
    T: Sample<'a, D3>,
{
    fn zero_point(
        &self,
        edge: Edge<D3>,
        bounds: Bounds<D3>,
    ) -> Result<<D3 as DimensionVector<f64>>::DimensionVector, EvaluateError> {
        let [from, to] = edge
            .to_corners()
            .into_iter()
            .map(|t| t.local_point(bounds))
            .collect_array();

        let (from, to) = if f64::from(self.sample(from)?.get(&DISTANCE.into())) >= 0.0 {
            (to, from)
        } else {
            (from, to)
        };

        zero_impl(self, 0.5, 0.25, 10, from, to)
    }
}

fn lerp<D: VectorSpace<f64>>(
    from: D::DimensionVector,
    to: D::DimensionVector,
    f: f64,
) -> D::DimensionVector
where
    D::DimensionVector: Mul<f64, Output = D::DimensionVector>,
{
    from * (1.0 - f) + to * f
}

fn zero_impl<'a, D: VectorSpace<f64>>(
    evaluator: &impl Sample<'a, D>,
    f: f64,
    step: f64,
    i: usize,
    from: D::DimensionVector,
    to: D::DimensionVector,
) -> Result<D::DimensionVector, EvaluateError>
where
    D::DimensionVector: Mul<f64, Output = D::DimensionVector>,
{
    if i == 0 {
        Ok(lerp::<D>(from, to, f).into())
    } else if f64::from(
        Sample::<D>::sample(evaluator, lerp::<D>(from.clone(), to.clone(), f))?
            .get(&DISTANCE.into()),
    ) < 0.0
    {
        zero_impl(evaluator, f + step, step / 2.0, i - 1, from, to)
    } else {
        zero_impl(evaluator, f - step, step / 2.0, i - 1, from, to)
    }
}
