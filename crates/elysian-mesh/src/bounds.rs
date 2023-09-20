use std::{
    hash::{Hash, Hasher},
    ops::Mul,
};

use elysian_ir::{ast::DISTANCE, module::EvaluateError};
use nalgebra::{Vector1, Vector2, Vector3};

use crate::{
    gltf_export::Samples,
    marching_cells::Corners,
    sample::Sample,
    vector_space::{DimensionVector, VectorSpace, D1, D2, D3},
};

/// Closed range in a vector space
pub struct Bounds<V: VectorSpace<f64>> {
    pub min: V::DimensionVector,
    pub max: V::DimensionVector,
}

impl<V> std::fmt::Debug for Bounds<V>
where
    V: VectorSpace<f64>,
    V::DimensionVector: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bounds")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl<V> Clone for Bounds<V>
where
    V: VectorSpace<f64>,
    V::DimensionVector: Clone,
{
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<V> Copy for Bounds<V>
where
    V: VectorSpace<f64>,
    V::DimensionVector: Copy,
{
}

impl<V> PartialEq for Bounds<V>
where
    V: VectorSpace<f64>,
    V::DimensionVector: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl<V> PartialOrd for Bounds<V>
where
    V: VectorSpace<f64>,
    V::DimensionVector: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.min.partial_cmp(&other.min) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.max.partial_cmp(&other.max)
    }
}

impl<V> Hash for Bounds<V>
where
    V: VectorSpace<f64>,
    V::DimensionVector: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.min.hash(state);
        self.max.hash(state);
    }
}

impl IntoIterator for Bounds<D1> {
    type Item = <D1 as DimensionVector<f64>>::DimensionVector;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            [self.min.x, self.max.x]
                .into_iter()
                .map(move |x| Vector1::new(x)),
        )
    }
}

impl IntoIterator for Bounds<D2> {
    type Item = <D2 as DimensionVector<f64>>::DimensionVector;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new([self.min.y, self.max.y].into_iter().flat_map(move |y| {
            [self.min.x, self.max.x]
                .into_iter()
                .map(move |x| Vector2::new(x, y))
        }))
    }
}

impl IntoIterator for Bounds<D3> {
    type Item = <D3 as DimensionVector<f64>>::DimensionVector;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new([self.min.z, self.max.z].into_iter().flat_map(move |z| {
            [self.min.y, self.max.y].into_iter().flat_map(move |y| {
                [self.min.x, self.max.x]
                    .into_iter()
                    .map(move |x| Vector3::new(x, y, z))
            })
        }))
    }
}

impl<V> Bounds<V>
where
    V: VectorSpace<f64>,
{
    pub fn size(&self) -> V::DimensionVector {
        self.max.clone() - self.min.clone()
    }

    pub fn center(&self) -> V::DimensionVector
    where
        V::DimensionVector: Mul<f64, Output = V::DimensionVector>,
    {
        self.min.clone() + self.size() * 0.5
    }

    pub fn samples<'a>(&self, evaluator: &impl Sample<'a, V>) -> Result<Samples, EvaluateError>
    where
        Self: IntoIterator<Item = V::DimensionVector>,
    {
        self.clone()
            .into_iter()
            .map(|p| evaluator.sample(p.into()))
            .collect()
    }

    pub fn sample_corners<'a>(
        &self,
        evaluator: &impl Sample<'a, V>,
    ) -> Result<Corners<V>, EvaluateError>
    where
        Self: IntoIterator<Item = V::DimensionVector>,
    {
        Ok(Corners::new(
            self.clone()
                .into_iter()
                .enumerate()
                .map(|(i, pt)| {
                    Ok(
                        if f64::from(evaluator.sample(pt.into())?.get(&DISTANCE.into())) < 0.0 {
                            2u8.pow(i as u32) as u8
                        } else {
                            0
                        },
                    )
                })
                .sum::<Result<u8, EvaluateError>>()?,
        ))
    }
}
