use std::fmt::Debug;

use nalgebra::{ClosedAdd, ClosedSub};

/// 1D Vector Space
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum D1 {}

/// 2D Vector Space
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum D2 {}

/// 3D Vector Space
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum D3 {}

/// An N-dimension vector space
pub trait VectorDimension {
    const DIMENSION: usize;
}

impl VectorDimension for D1 {
    const DIMENSION: usize = 1;
}

impl VectorDimension for D2 {
    const DIMENSION: usize = 2;
}

impl VectorDimension for D3 {
    const DIMENSION: usize = 3;
}

/// The number of cells needed to uniformly subdivide a vector space
pub trait VectorSubdivision: VectorDimension {
    const SUBDIVISION: usize = 2usize.pow(Self::DIMENSION as u32);
}

/// Array type corresponding to the number of dimensions in a vector space
pub trait DimensionVector<T>: VectorDimension {
    type DimensionVector: Clone + ClosedAdd + ClosedSub;

    fn from_vec(v: Vec<T>) -> Self::DimensionVector;
    fn splat(t: T) -> Self::DimensionVector
    where
        T: Clone,
    {
        Self::from_vec(std::iter::repeat(t).take(Self::DIMENSION).collect())
    }

    fn component_add(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector;

    fn component_sub(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector;

    fn component_mul(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector;

    fn component_div(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector;

    fn component_min(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector;

    fn component_max(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector;
}

impl DimensionVector<f64> for D1 {
    type DimensionVector = nalgebra::Vector1<f64>;

    fn from_vec(v: Vec<f64>) -> Self::DimensionVector {
        Self::DimensionVector::from_vec(v)
    }

    fn component_add(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs + rhs
    }

    fn component_sub(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs - rhs
    }

    fn component_mul(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs.component_mul(&rhs)
    }

    fn component_div(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs.component_div(&rhs)
    }

    fn component_min(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        Self::DimensionVector::new(lhs.x.min(rhs.x))
    }

    fn component_max(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        Self::DimensionVector::new(lhs.x.max(rhs.x))
    }
}

impl DimensionVector<f64> for D2 {
    type DimensionVector = nalgebra::Vector2<f64>;

    fn from_vec(v: Vec<f64>) -> Self::DimensionVector {
        Self::DimensionVector::from_vec(v)
    }

    fn component_add(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs + rhs
    }

    fn component_sub(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs - rhs
    }

    fn component_mul(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs.component_mul(&rhs)
    }

    fn component_div(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs.component_div(&rhs)
    }

    fn component_min(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        Self::DimensionVector::new(lhs.x.min(rhs.x), lhs.y.min(rhs.y))
    }

    fn component_max(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        Self::DimensionVector::new(lhs.x.max(rhs.x), lhs.y.max(rhs.y))
    }
}

impl DimensionVector<f64> for D3 {
    type DimensionVector = nalgebra::Vector3<f64>;

    fn from_vec(v: Vec<f64>) -> Self::DimensionVector {
        Self::DimensionVector::from_vec(v)
    }

    fn component_add(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs + rhs
    }

    fn component_sub(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs - rhs
    }

    fn component_mul(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs.component_mul(&rhs)
    }

    fn component_div(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        lhs.component_div(&rhs)
    }

    fn component_min(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        Self::DimensionVector::new(lhs.x.min(rhs.x), lhs.y.min(rhs.y), lhs.z.min(rhs.z))
    }

    fn component_max(
        lhs: &Self::DimensionVector,
        rhs: &Self::DimensionVector,
    ) -> Self::DimensionVector {
        Self::DimensionVector::new(lhs.x.max(rhs.x), lhs.y.max(rhs.y), lhs.z.max(rhs.z))
    }
}

/// Array type corresponding to the number of dimensions in a vector space
pub trait DimensionArray<T>: VectorDimension {
    type DimensionArray;
}

impl<T> DimensionArray<T> for D1 {
    type DimensionArray = T;
}

impl<T> DimensionArray<T> for D2 {
    type DimensionArray = [T; 2];
}

impl<T> DimensionArray<T> for D3 {
    type DimensionArray = [T; 3];
}

/// Array type corresponding to the number of cells required
/// to evenly subdivide a vector space
pub trait SubdivisionArray<T>: VectorSubdivision {
    type SubdivisionArray: TryFrom<Vec<T>, Error = Vec<T>> + IntoIterator<Item = T>;
    type Mapped<U>: TryFrom<Vec<U>, Error = Vec<U>> + IntoIterator<Item = U>;

    fn map<U>(arr: Self::SubdivisionArray, f: impl FnMut(T) -> U) -> Self::Mapped<U>;
    fn map_ref<U>(arr: &Self::SubdivisionArray, f: impl FnMut(&T) -> U) -> Self::Mapped<U>;

    fn iter<'a>(arr: &'a Self::SubdivisionArray) -> std::slice::Iter<'a, T>;

    fn subdivision_indices() -> Vec<Vec<usize>>;
}

impl<T> SubdivisionArray<T> for D1 {
    type SubdivisionArray = [T; 2];
    type Mapped<U> = [U; 2];

    fn map<U>(arr: Self::SubdivisionArray, f: impl FnMut(T) -> U) -> Self::Mapped<U> {
        arr.map(f)
    }

    fn map_ref<U>(arr: &Self::SubdivisionArray, f: impl FnMut(&T) -> U) -> Self::Mapped<U> {
        arr.iter()
            .map(f)
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }

    fn iter<'a>(arr: &'a Self::SubdivisionArray) -> std::slice::Iter<'a, T> {
        arr.iter()
    }

    fn subdivision_indices() -> Vec<Vec<usize>> {
        vec![vec![0]]
    }
}

impl<T> SubdivisionArray<T> for D2 {
    type SubdivisionArray = [T; 4];
    type Mapped<U> = [U; 4];

    fn map<U>(arr: Self::SubdivisionArray, f: impl FnMut(T) -> U) -> Self::Mapped<U> {
        arr.map(f)
    }

    fn map_ref<U>(arr: &Self::SubdivisionArray, f: impl FnMut(&T) -> U) -> Self::Mapped<U> {
        arr.iter()
            .map(f)
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }

    fn iter<'a>(arr: &'a Self::SubdivisionArray) -> std::slice::Iter<'a, T> {
        arr.iter()
    }

    fn subdivision_indices() -> Vec<Vec<usize>> {
        (0..2)
            .into_iter()
            .flat_map(|y| (0..2).into_iter().map(move |x| vec![x, y]))
            .collect()
    }
}

impl<T> SubdivisionArray<T> for D3 {
    type SubdivisionArray = [T; 8];
    type Mapped<U> = [U; 8];

    fn map<U>(arr: Self::SubdivisionArray, f: impl FnMut(T) -> U) -> Self::Mapped<U> {
        arr.map(f)
    }

    fn map_ref<U>(arr: &Self::SubdivisionArray, f: impl FnMut(&T) -> U) -> Self::Mapped<U> {
        arr.iter()
            .map(f)
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap()
    }

    fn iter<'a>(arr: &'a Self::SubdivisionArray) -> std::slice::Iter<'a, T> {
        arr.iter()
    }

    fn subdivision_indices() -> Vec<Vec<usize>> {
        (0..2)
            .into_iter()
            .flat_map(|z| {
                (0..2)
                    .into_iter()
                    .flat_map(move |y| (0..2).into_iter().map(move |x| vec![x, y, z]))
            })
            .collect()
    }
}

impl<T> VectorSubdivision for T where T: VectorDimension {}
