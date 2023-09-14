use std::ops::{BitAnd, BitOr, BitXor, Mul, Not};

use elysian_ir::{ast::DISTANCE, module::EvaluateError};
use nalgebra::{Vector2, Vector3};

use crate::{
    bounds::Bounds,
    sample::Sample,
    util::CollectArray,
    vector_space::{DimensionArray, DimensionVector, D2, D3},
};

use super::{Corner, Corners, Edges, Point, Power2U16, ToCorners};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge<const D: usize>(pub Power2U16);

impl<const D: usize> Edge<D> {
    pub const unsafe fn new_unchecked(n: u16) -> Self {
        Self(Power2U16::new_unchecked(n))
    }

    pub fn new(n: u16) -> Option<Self> {
        Some(Self(Power2U16::new(n)?))
    }
}

impl Edge<2> {
    pub const L: Self = unsafe { Self::new_unchecked(1) };
    pub const R: Self = unsafe { Self::new_unchecked(2) };
    pub const U: Self = unsafe { Self::new_unchecked(4) };
    pub const D: Self = unsafe { Self::new_unchecked(8) };
}

impl Edge<3> {
    pub const FD: Self = unsafe { Self::new_unchecked(1) };
    pub const FR: Self = unsafe { Self::new_unchecked(2) };
    pub const FU: Self = unsafe { Self::new_unchecked(4) };
    pub const FL: Self = unsafe { Self::new_unchecked(8) };

    pub const BD: Self = unsafe { Self::new_unchecked(16) };
    pub const BR: Self = unsafe { Self::new_unchecked(32) };
    pub const BU: Self = unsafe { Self::new_unchecked(64) };
    pub const BL: Self = unsafe { Self::new_unchecked(128) };

    pub const LD: Self = unsafe { Self::new_unchecked(256) };
    pub const RD: Self = unsafe { Self::new_unchecked(512) };
    pub const RU: Self = unsafe { Self::new_unchecked(1024) };
    pub const LU: Self = unsafe { Self::new_unchecked(2048) };
}

impl<const D: usize> BitAnd for Edge<D> {
    type Output = Edges<D>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Edges(self.0 & rhs.0)
    }
}

impl<const D: usize> BitOr for Edge<D> {
    type Output = Edges<D>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Edges(self.0 | rhs.0)
    }
}

impl<const D: usize> BitXor for Edge<D> {
    type Output = Edges<D>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Edges(self.0 ^ rhs.0)
    }
}

impl<const D: usize> Not for Edge<D> {
    type Output = Edges<D>;

    fn not(self) -> Self::Output {
        Edges(!self.0)
    }
}

impl ToCorners<2> for Edge<2> {
    fn to_corners(&self) -> Corners<2> {
        let mut corner = Corners::<2>::EMPTY;

        match *self {
            Edge::<2>::L => corner |= Corner::<2>::DL | Corner::<2>::UL,
            Edge::<2>::R => corner |= Corner::<2>::DR | Corner::<2>::UR,
            Edge::<2>::U => corner |= Corner::<2>::UL | Corner::<2>::UR,
            Edge::<2>::D => corner |= Corner::<2>::DL | Corner::<2>::DR,
            _ => unreachable!(),
        }

        corner
    }
}

impl ToCorners<3> for Edge<3> {
    fn to_corners(&self) -> Corners<3> {
        let mut corner = Corners::<3>::EMPTY;

        match *self {
            Edge::<3>::FL => corner |= Corner::<3>::FDL | Corner::<3>::FUL,
            Edge::<3>::FD => corner |= Corner::<3>::FDL | Corner::<3>::FDR,
            Edge::<3>::FU => corner |= Corner::<3>::FUL | Corner::<3>::FUR,
            Edge::<3>::FR => corner |= Corner::<3>::FDR | Corner::<3>::FUR,

            Edge::<3>::BL => corner |= Corner::<3>::BDL | Corner::<3>::BUL,
            Edge::<3>::BD => corner |= Corner::<3>::BDL | Corner::<3>::BDR,
            Edge::<3>::BU => corner |= Corner::<3>::BUL | Corner::<3>::BUR,
            Edge::<3>::BR => corner |= Corner::<3>::BDR | Corner::<3>::BUR,

            Edge::<3>::LD => corner |= Corner::<3>::FDL | Corner::<3>::BDL,
            Edge::<3>::RD => corner |= Corner::<3>::FDR | Corner::<3>::BDR,
            Edge::<3>::LU => corner |= Corner::<3>::FUL | Corner::<3>::BUL,
            Edge::<3>::RU => corner |= Corner::<3>::FUR | Corner::<3>::BUR,

            _ => unreachable!(),
        }

        corner
    }
}

impl Point<D2> for Edge<2> {
    fn point(&self) -> <D2 as DimensionArray<f64>>::DimensionArray {
        (self
            .to_corners()
            .into_iter()
            .map(|corner| Vector2::<_>::from(corner.point()))
            .sum::<Vector2<_>>()
            / 2.0)
            .into()
    }
}

impl Point<D3> for Edge<3> {
    fn point(&self) -> <D3 as DimensionArray<f64>>::DimensionArray {
        (self
            .to_corners()
            .into_iter()
            .map(|corner| Vector3::<_>::from(corner.point()))
            .sum::<Vector3<_>>()
            / 2.0)
            .into()
    }
}

pub trait LocalPoint<D: DimensionVector<f64> + DimensionArray<f64>>: Point<D> {
    fn local_point(self, bounds: Bounds<D>) -> <D as DimensionVector<f64>>::DimensionVector;
}

impl<D, T> LocalPoint<D> for T
where
    T: Point<D>,
    D: DimensionArray<f64> + DimensionVector<f64>,
    D::DimensionVector: From<D::DimensionArray>,
{
    fn local_point(self, bounds: Bounds<D>) -> <D as DimensionVector<f64>>::DimensionVector {
        let p = self.point();
        let p = D::component_add(
            &D::component_mul(
                &<D as DimensionVector<f64>>::DimensionVector::from(p),
                &D::splat(0.5),
            ),
            &D::splat(0.5),
        );
        let ofs = D::component_mul(&p, &bounds.size());

        D::component_add(&bounds.min, &ofs)
    }
}

pub trait ZeroPoint<'a, D: DimensionVector<f64>, const N: usize>: Sample<'a, D> {
    fn zero_point(
        &self,
        edge: Edge<N>,
        bounds: Bounds<D>,
    ) -> Result<<D as DimensionVector<f64>>::DimensionVector, EvaluateError>;
}

fn lerp<D: DimensionVector<f64>>(
    from: <D as DimensionVector<f64>>::DimensionVector,
    to: <D as DimensionVector<f64>>::DimensionVector,
    f: f64,
) -> D::DimensionVector
where
    D::DimensionVector: Mul<f64, Output = D::DimensionVector>,
{
    from * (1.0 - f) + to * f
}

fn zero_impl<'a, D: DimensionVector<f64>>(
    evaluator: &impl Sample<'a, D>,
    f: f64,
    step: f64,
    i: usize,
    from: <D as DimensionVector<f64>>::DimensionVector,
    to: <D as DimensionVector<f64>>::DimensionVector,
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

impl<'a, T> ZeroPoint<'a, D2, 2> for T
where
    T: Sample<'a, D2>,
{
    fn zero_point(
        &self,
        edge: Edge<2>,
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

impl<'a, T> ZeroPoint<'a, D3, 3> for T
where
    T: Sample<'a, D3>,
{
    fn zero_point(
        &self,
        edge: Edge<3>,
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
