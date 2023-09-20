use elysian_ir::module::EvaluateError;

use crate::{vector_space::VectorSpace, bounds::Bounds, gltf_export::PointPrimitives};

use super::{MarchPrimitive, ZeroPoint, Edge, LocalPoint};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Hash)]
pub struct MarchPrimitives<D>(Vec<MarchPrimitive<D>>);

impl<D> FromIterator<MarchPrimitive<D>> for MarchPrimitives<D> {
    fn from_iter<T: IntoIterator<Item = MarchPrimitive<D>>>(iter: T) -> Self {
        MarchPrimitives(iter.into_iter().collect())
    }
}

impl<D> IntoIterator for MarchPrimitives<D> {
    type Item = MarchPrimitive<D>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<D> MarchPrimitives<D>
where
    D: VectorSpace<f64>,
{
    pub fn zero_points<'a>(
        self,
        evaluator: &impl ZeroPoint<'a, D>,
        bounds: &Bounds<D>,
    ) -> Result<PointPrimitives<D>, EvaluateError> {
        self.into_iter()
            .map(|march_primitive| march_primitive.zero_points(evaluator, bounds))
            .collect()
    }

    pub fn center_points<'a>(
        self,
        bounds: &Bounds<D>,
    ) -> Result<PointPrimitives<D>, EvaluateError>
    where
        Edge<D>: LocalPoint<D>,
    {
        self.into_iter()
            .map(|march_primitive| march_primitive.local_points(bounds))
            .collect()
    }
}

pub trait ToMarchPrimitives<D> {
    fn march_primitives(self) -> MarchPrimitives<D>;
}

