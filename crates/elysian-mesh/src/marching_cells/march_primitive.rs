use elysian_ir::module::EvaluateError;

use crate::{
    bounds::Bounds,
    gltf_export::{Indices, PointPrimitive},
    vector_space::VectorSpace,
};

use super::{Edge, LocalPoint, ZeroPoint};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Hash)]
pub struct MarchPrimitive<D> {
    pub edges: Vec<Edge<D>>,
    pub indices: Option<Indices>,
}

impl<D> MarchPrimitive<D> {
    pub fn new(
        edges: impl IntoIterator<Item = Edge<D>>,
        indices: Option<impl IntoIterator<Item = u32>>,
    ) -> Self {
        MarchPrimitive {
            edges: edges.into_iter().collect(),
            indices: indices.map(|it| it.into_iter().collect()),
        }
    }

    pub fn local_points<'a>(self, bounds: &Bounds<D>) -> Result<PointPrimitive<D>, EvaluateError>
    where
        D: VectorSpace<f64>,
        Edge<D>: LocalPoint<D>,
    {
        let MarchPrimitive { edges, indices } = self;

        let points = edges
            .into_iter()
            .map(|edge| edge.local_point(bounds.clone()))
            .collect();

        Ok(PointPrimitive { points, indices })
    }

    pub fn zero_points<'a>(
        self,
        evaluator: &impl ZeroPoint<'a, D>,
        bounds: &Bounds<D>,
    ) -> Result<PointPrimitive<D>, EvaluateError>
    where
        D: VectorSpace<f64>,
    {
        let MarchPrimitive { edges, indices } = self;

        let points = edges
            .into_iter()
            .map(|edge| evaluator.zero_point(edge, bounds.clone()))
            .collect::<Result<_, EvaluateError>>()?;

        Ok(PointPrimitive { points, indices })
    }
}
