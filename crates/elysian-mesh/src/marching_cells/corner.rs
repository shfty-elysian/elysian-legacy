use std::{ops::{BitAnd, BitOr, BitXor, Not}, marker::PhantomData};

use crate::vector_space::{DimensionArray, D2, D3, VectorSpace};

use super::{edge::Edge, Corners, Edges, Point, Power2U8, ToEdges};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Corner<D>(pub Power2U8, PhantomData<D>);

impl<D> Corner<D> where D: VectorSpace<f64> {
    pub const EMPTY: Self = Self(unsafe { Power2U8::new_unchecked(0) }, PhantomData);

    const fn max_value() -> u8 {
        2u8.pow(2u8.pow(D::DIMENSION as u32) as u32 - 1)
    }

    pub const unsafe fn new_unchecked(v: u8) -> Self {
        assert!(v <= Self::max_value());
        Corner(Power2U8::new_unchecked(v), PhantomData)
    }

    pub fn new(v: u8) -> Option<Self> {
        if v <= Self::max_value() {
            Some(Corner(Power2U8::new(v)?, PhantomData))
        } else {
            None
        }
    }
}

impl Corner<D2> {
    pub const DL: Self = unsafe { Self::new_unchecked(1) };
    pub const DR: Self = unsafe { Self::new_unchecked(2) };
    pub const UL: Self = unsafe { Self::new_unchecked(4) };
    pub const UR: Self = unsafe { Self::new_unchecked(8) };
}

impl Corner<D3> {
    pub const BDL: Self = unsafe { Self::new_unchecked(1) };
    pub const BDR: Self = unsafe { Self::new_unchecked(2) };
    pub const BUL: Self = unsafe { Self::new_unchecked(4) };
    pub const BUR: Self = unsafe { Self::new_unchecked(8) };
    pub const FDL: Self = unsafe { Self::new_unchecked(16) };
    pub const FDR: Self = unsafe { Self::new_unchecked(32) };
    pub const FUL: Self = unsafe { Self::new_unchecked(64) };
    pub const FUR: Self = unsafe { Self::new_unchecked(128) };
}

impl<D> BitAnd for Corner<D> {
    type Output = Corners<D>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Corners::new(self.0 & rhs.0)
    }
}

impl<D> BitOr for Corner<D> {
    type Output = Corners<D>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Corners::new(self.0 | rhs.0)
    }
}

impl<D> BitXor for Corner<D> {
    type Output = Corners<D>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Corners::new(self.0 ^ rhs.0)
    }
}

impl<D> Not for Corner<D> {
    type Output = Corners<D>;

    fn not(self) -> Self::Output {
        Corners::new(!self.0)
    }
}

impl Point<D2> for Corner<D2> {
    fn point(&self) -> <D2 as DimensionArray>::DimensionArray<f64> {
        match *self {
            Self::DL => [-1.0, -1.0],
            Self::DR => [1.0, -1.0],
            Self::UL => [-1.0, 1.0],
            Self::UR => [1.0, 1.0],
            _ => unimplemented!(),
        }
    }
}

impl Point<D3> for Corner<D3> {
    fn point(&self) -> <D3 as DimensionArray>::DimensionArray<f64> {
        match *self {
            Self::FDL => [-1.0, -1.0, 1.0],
            Self::FDR => [1.0, -1.0, 1.0],
            Self::FUL => [-1.0, 1.0, 1.0],
            Self::FUR => [1.0, 1.0, 1.0],
            Self::BDL => [-1.0, -1.0, -1.0],
            Self::BDR => [1.0, -1.0, -1.0],
            Self::BUL => [-1.0, 1.0, -1.0],
            Self::BUR => [1.0, 1.0, -1.0],
            _ => unimplemented!(),
        }
    }
}

impl ToEdges<D2> for Corner<D2> {
    fn to_edges(&self) -> Edges<D2> {
        let mut edge = Edges::<D2>::EMPTY;

        match *self {
            Corner::<D2>::DL => edge |= Edge::<D2>::D | Edge::<D2>::L,
            Corner::<D2>::DR => edge |= Edge::<D2>::D | Edge::<D2>::R,
            Corner::<D2>::UL => edge |= Edge::<D2>::U | Edge::<D2>::L,
            Corner::<D2>::UR => edge |= Edge::<D2>::U | Edge::<D2>::R,
            _ => unreachable!(),
        }

        edge
    }
}

impl ToEdges<D3> for Corner<D3> {
    fn to_edges(&self) -> Edges<D3> {
        let mut edge = Edges::<D3>::EMPTY;

        match *self {
            Corner::<D3>::FDL => edge |= Edge::<D3>::FD | Edge::<D3>::FL | Edge::<D3>::LD,
            Corner::<D3>::FDR => edge |= Edge::<D3>::FD | Edge::<D3>::FR | Edge::<D3>::RD,
            Corner::<D3>::FUL => edge |= Edge::<D3>::FU | Edge::<D3>::FL | Edge::<D3>::LU,
            Corner::<D3>::FUR => edge |= Edge::<D3>::FU | Edge::<D3>::FR | Edge::<D3>::RU,
            Corner::<D3>::BDL => edge |= Edge::<D3>::BD | Edge::<D3>::BL | Edge::<D3>::LD,
            Corner::<D3>::BDR => edge |= Edge::<D3>::BD | Edge::<D3>::BR | Edge::<D3>::RD,
            Corner::<D3>::BUL => edge |= Edge::<D3>::BU | Edge::<D3>::BL | Edge::<D3>::LU,
            Corner::<D3>::BUR => edge |= Edge::<D3>::BU | Edge::<D3>::BR | Edge::<D3>::RU,
            t => unreachable!("{t:?}"),
        }

        edge
    }
}
