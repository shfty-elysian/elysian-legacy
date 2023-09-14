use std::{
    collections::BTreeMap,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use elysian_ir::{
    ast::{Struct, DISTANCE},
    module::EvaluateError,
};

use crate::{
    bounds::Bounds,
    sample::Sample,
    vector_space::{DimensionVector, VectorSubdivision, D2, D3},
};

use super::{edge::Edge, All, Cell, Corner, Edges, ToEdges};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Corners<const D: usize>(pub u8);

impl<const D: usize> Corners<D> {
    pub const EMPTY: Self = Corners(0);

    pub fn is_empty(&self) -> bool {
        *self == Self::EMPTY
    }
}

impl IntoIterator for Corners<2> {
    type Item = Corner<2>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..4).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is either 0 or 1, so v << i will always be a power of two
                Some(unsafe { Corner::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl IntoIterator for Corners<3> {
    type Item = Corner<3>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..8).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is either 0 or 1, so v << i will always be a power of two
                Some(unsafe { Corner::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl All for Corners<2> {
    fn all() -> Self {
        Corner::<2>::DL | Corner::<2>::DR | Corner::<2>::UL | Corner::<2>::UR
    }
}

impl All for Corners<3> {
    fn all() -> Self {
        Corner::<3>::FDL
            | Corner::<3>::FDR
            | Corner::<3>::FUL
            | Corner::<3>::FUR
            | Corner::<3>::BDL
            | Corner::<3>::BDR
            | Corner::<3>::BUL
            | Corner::<3>::BUR
    }
}

impl<const D: usize> ToEdges<D> for Corners<D>
where
    Corners<D>: IntoIterator<Item = Corner<D>>,
    Corner<D>: ToEdges<D>,
{
    fn to_edges(&self) -> Edges<D> {
        self.into_iter()
            .fold(Edges::<D>::EMPTY, |acc, next| acc | next.to_edges())
    }
}

impl<const D: usize> BitAnd<Corner<D>> for Corners<D> {
    type Output = Self;

    fn bitand(self, rhs: Corner<D>) -> Self::Output {
        Self(self.0 & rhs.0.get())
    }
}

impl<const D: usize> BitOr<Corner<D>> for Corners<D> {
    type Output = Self;

    fn bitor(self, rhs: Corner<D>) -> Self::Output {
        Self(self.0 | rhs.0.get())
    }
}

impl<const D: usize> BitXor<Corner<D>> for Corners<D> {
    type Output = Self;

    fn bitxor(self, rhs: Corner<D>) -> Self::Output {
        Self(self.0 ^ rhs.0.get())
    }
}

impl<const D: usize> BitAnd for Corners<D> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<const D: usize> BitAndAssign for Corners<D> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<const D: usize> BitOr for Corners<D> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl<const D: usize> BitOrAssign for Corners<D> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<const D: usize> BitXor for Corners<D> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl<const D: usize> BitXorAssign for Corners<D> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<const D: usize> Not for Corners<D> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<const D: usize> Cell<D> for Corners<D>
where
    Corners<D>: IntoIterator<Item = Corner<D>>,
    Corner<D>: ToEdges<D>,
    Edges<D>: All + IntoIterator<Item = Edge<D>>,
    Edge<D>: ToCorners<D>,
{
    fn cell(&self) -> BTreeMap<Corner<D>, Edges<D>> {
        let mut edges = self.to_edges();

        for edge in <Edges<D> as All>::all().into_iter() {
            if *self & edge.to_corners() == edge.to_corners() {
                edges &= !edge;
            }
        }

        let mut cell = BTreeMap::new();

        for corner in (*self).into_iter() {
            let mut out = Edges::<D>::EMPTY;
            for edge in edges.into_iter() {
                if (corner.to_edges() & edge).0 == edge.0.get() {
                    out = out | edge;
                }
            }
            if out != Edges::EMPTY {
                cell.insert(corner, out);
            }
        }

        cell
    }
}

pub trait ToCorners<const D: usize> {
    fn to_corners(&self) -> Corners<D>;
}

/// `Sample` extension trait for producing a set of `Corners`
pub trait SampleCorners<'a, D, const N: usize>: Sample<'a, D>
where
    D: DimensionVector<f64> + VectorSubdivision,
    Bounds<D>: IntoIterator<Item = D::DimensionVector>,
{
    fn sample_corners(&self, bounds: Bounds<D>) -> Result<Corners<N>, EvaluateError>;
}

impl<'a, T> SampleCorners<'a, D2, 2> for T
where
    T: Sample<'a, D2>,
    D2: DimensionVector<f64> + VectorSubdivision,
    Bounds<D2>: IntoIterator<Item = <D2 as DimensionVector<f64>>::DimensionVector>,
{
    fn sample_corners(&self, bounds: Bounds<D2>) -> Result<Corners<2>, EvaluateError> {
        Ok(Corners(
            bounds
                .into_iter()
                .enumerate()
                .map(|(i, pt)| {
                    Ok(
                        if f64::from(Sample::<D2>::sample(self, pt.into())?.get(&DISTANCE.into()))
                            < 0.0
                        {
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

impl<'a, T> SampleCorners<'a, D3, 3> for T
where
    T: Sample<'a, D3>,
    D3: DimensionVector<f64> + VectorSubdivision,
    Bounds<D3>: IntoIterator<Item = <D3 as DimensionVector<f64>>::DimensionVector>,
{
    fn sample_corners(&self, bounds: Bounds<D3>) -> Result<Corners<3>, EvaluateError> {
        Ok(Corners(
            bounds
                .into_iter()
                .enumerate()
                .map(|(i, pt)| {
                    Ok(
                        if f64::from(Sample::<D3>::sample(self, pt.into())?.get(&DISTANCE.into()))
                            < 0.0
                        {
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

/// `Sample` extension trait for sampling at the corners of a `Bounds`
pub trait SampleBounds<'a, D>
where
    D: DimensionVector<f64>,
{
    fn sample_bounds(&self, bounds: Bounds<D>) -> Result<Vec<Struct>, EvaluateError>;
}

impl<'a, D, T> SampleBounds<'a, D> for T
where
    D: DimensionVector<f64>,
    T: Sample<'a, D>,
    Bounds<D>: IntoIterator,
    <D as DimensionVector<f64>>::DimensionVector: From<<Bounds<D> as IntoIterator>::Item>,
{
    fn sample_bounds(&self, bounds: Bounds<D>) -> Result<Vec<Struct>, EvaluateError> {
        bounds
            .into_iter()
            .map(|p| Sample::<D>::sample(self, p.into()))
            .collect::<Result<Vec<_>, _>>()
    }
}
