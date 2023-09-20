use elysian_ir::{module::{Evaluate, EvaluateError}, ast::DISTANCE};

use crate::{vector_space::{VectorSpace, D2, DimensionVector, D3}, bounds::Bounds, sample::Sample};

pub trait InterpolateCell<D: VectorSpace<f64>> {
    fn interpolate<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
        p: D::DimensionVector,
    ) -> Result<f64, EvaluateError>;
}

impl InterpolateCell<D2> for Bounds<D2> {
    fn interpolate<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
        p: <D2 as DimensionVector<f64>>::DimensionVector,
    ) -> Result<f64, EvaluateError> {
        let delta = (p - self.min).component_div(&self.size());

        let ab = f64::from(Sample::<D2>::sample(evaluator, self.min.into())?.get(&DISTANCE.into()))
            * (1.0 - delta.x)
            + f64::from(
                Sample::<D2>::sample(evaluator, [self.max.x, self.min.y].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let cd: f64 = f64::from(
            Sample::<D2>::sample(evaluator, [self.min.x, self.max.y].into())?.get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(Sample::<D2>::sample(evaluator, self.max.into())?.get(&DISTANCE.into()))
                * delta.x;

        Ok(ab * (1.0 - delta.y) + cd * delta.y)
    }
}

impl InterpolateCell<D3> for Bounds<D3> {
    fn interpolate<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
        p: <D3 as DimensionVector<f64>>::DimensionVector,
    ) -> Result<f64, EvaluateError> {
        let delta = (p - self.min).component_div(&self.size());

        let ab = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.min.y, self.min.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.min.y, self.min.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let cd: f64 = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.max.y, self.min.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.max.y, self.min.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let abcd = ab * (1.0 - delta.y) + cd * delta.y;

        let ef = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.min.y, self.max.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.min.y, self.max.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let gh: f64 = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.max.y, self.max.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.max.y, self.max.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let efgh = ef * (1.0 - delta.y) + gh * delta.y;

        Ok(abcd * (1.0 - delta.z) + efgh * delta.z)
    }
}
