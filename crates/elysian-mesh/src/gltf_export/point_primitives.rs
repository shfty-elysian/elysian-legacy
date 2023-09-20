use elysian_ir::{
    ast::{Struct, POSITION_2D, POSITION_3D},
    module::{EvaluateError, StructIdentifier, CONTEXT},
};
use gltf_json::mesh::Mode;

use crate::{
    sample::Sample,
    vector_space::{VectorSpace, D2, D3},
};

use super::{ExportPrimitive, ExportPrimitives, PointPrimitive, Properties};

#[derive(Default)]
pub struct PointPrimitives<D>(Vec<PointPrimitive<D>>)
where
    D: VectorSpace<f64>;

impl<D> FromIterator<PointPrimitive<D>> for PointPrimitives<D>
where
    D: VectorSpace<f64>,
{
    fn from_iter<T: IntoIterator<Item = PointPrimitive<D>>>(iter: T) -> Self {
        PointPrimitives(iter.into_iter().collect())
    }
}

impl<D> IntoIterator for PointPrimitives<D>
where
    D: VectorSpace<f64>,
{
    type Item = PointPrimitive<D>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PointPrimitives<D2> {
    pub fn export_lines<'a>(
        self,
        properties: &Properties<'a>,
    ) -> Result<ExportPrimitives<'a>, EvaluateError> {
        Ok(self
            .into_iter()
            .map(|primitive| {
                let line = primitive
                    .points
                    .into_iter()
                    .map(|p| {
                        Struct::new(StructIdentifier(CONTEXT))
                            .set(POSITION_2D.into(), [p.x, p.y].into())
                    })
                    .collect();

                ExportPrimitive {
                    mode: Mode::Lines,
                    properties: properties.clone(),
                    samples: line,
                    indices: primitive.indices,
                }
            })
            .collect())
    }
}

impl PointPrimitives<D3> {
    pub fn export_lines<'a, 'b>(
        self,
        properties: &Properties<'a>,
        evaluator: &impl Sample<'b, D3>,
    ) -> Result<ExportPrimitives<'a>, EvaluateError> {
        Ok(self
            .into_iter()
            .map(|primitive| ExportPrimitive {
                mode: Mode::Triangles,
                properties: properties.clone(),
                samples: primitive
                    .points
                    .into_iter()
                    .map(|p| {
                        let s = Sample::<D3>::sample(evaluator, p).unwrap();
                        s.set(POSITION_3D.into(), [p.x, p.y, p.z].into())
                    })
                    .collect(),
                indices: primitive.indices,
            })
            .collect())
    }
}
