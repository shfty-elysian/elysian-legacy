use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::vector_space::{DimensionArray, D2, D3};

use super::{edge::Edge, Corners, Edges, Point, Power2U8, ToEdges};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Corner<const D: usize>(pub Power2U8);

impl<const D: usize> Corner<D> {
    pub const EMPTY: Self = Self(unsafe { Power2U8::new_unchecked(0) });

    const fn max_value() -> u8 {
        2u8.pow(2u8.pow(D as u32) as u32 - 1)
    }

    pub const unsafe fn new_unchecked(v: u8) -> Self {
        assert!(v <= Self::max_value());
        Corner(Power2U8::new_unchecked(v))
    }

    pub fn new(v: u8) -> Option<Self> {
        if v <= Self::max_value() {
            Some(Corner(Power2U8::new(v)?))
        } else {
            None
        }
    }
}

impl Corner<2> {
    pub const DL: Self = unsafe { Self::new_unchecked(1) };
    pub const DR: Self = unsafe { Self::new_unchecked(2) };
    pub const UL: Self = unsafe { Self::new_unchecked(4) };
    pub const UR: Self = unsafe { Self::new_unchecked(8) };
}

impl Corner<3> {
    pub const BDL: Self = unsafe { Self::new_unchecked(1) };
    pub const BDR: Self = unsafe { Self::new_unchecked(2) };
    pub const BUL: Self = unsafe { Self::new_unchecked(4) };
    pub const BUR: Self = unsafe { Self::new_unchecked(8) };
    pub const FDL: Self = unsafe { Self::new_unchecked(16) };
    pub const FDR: Self = unsafe { Self::new_unchecked(32) };
    pub const FUL: Self = unsafe { Self::new_unchecked(64) };
    pub const FUR: Self = unsafe { Self::new_unchecked(128) };
}

impl<const D: usize> BitAnd for Corner<D> {
    type Output = Corners<D>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Corners(self.0 & rhs.0)
    }
}

impl<const D: usize> BitOr for Corner<D> {
    type Output = Corners<D>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Corners(self.0 | rhs.0)
    }
}

impl<const D: usize> BitXor for Corner<D> {
    type Output = Corners<D>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Corners(self.0 ^ rhs.0)
    }
}

impl<const D: usize> Not for Corner<D> {
    type Output = Corners<D>;

    fn not(self) -> Self::Output {
        Corners(!self.0)
    }
}

impl Point<D2> for Corner<2> {
    fn point(&self) -> <D2 as DimensionArray<f64>>::DimensionArray {
        match *self {
            Self::DL => [-1.0, -1.0],
            Self::DR => [1.0, -1.0],
            Self::UL => [-1.0, 1.0],
            Self::UR => [1.0, 1.0],
            _ => unimplemented!(),
        }
    }
}

impl Point<D3> for Corner<3> {
    fn point(&self) -> <D3 as DimensionArray<f64>>::DimensionArray {
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

impl ToEdges<2> for Corner<2> {
    fn to_edges(&self) -> Edges<2> {
        let mut edge = Edges::<2>::EMPTY;

        match *self {
            Corner::<2>::DL => edge |= Edge::<2>::D | Edge::<2>::L,
            Corner::<2>::DR => edge |= Edge::<2>::D | Edge::<2>::R,
            Corner::<2>::UL => edge |= Edge::<2>::U | Edge::<2>::L,
            Corner::<2>::UR => edge |= Edge::<2>::U | Edge::<2>::R,
            _ => unreachable!(),
        }

        edge
    }
}

impl ToEdges<3> for Corner<3> {
    fn to_edges(&self) -> Edges<3> {
        let mut edge = Edges::<3>::EMPTY;

        match *self {
            Corner::<3>::FDL => edge |= Edge::<3>::FD | Edge::<3>::FL | Edge::<3>::LD,
            Corner::<3>::FDR => edge |= Edge::<3>::FD | Edge::<3>::FR | Edge::<3>::RD,
            Corner::<3>::FUL => edge |= Edge::<3>::FU | Edge::<3>::FL | Edge::<3>::LU,
            Corner::<3>::FUR => edge |= Edge::<3>::FU | Edge::<3>::FR | Edge::<3>::RU,
            Corner::<3>::BDL => edge |= Edge::<3>::BD | Edge::<3>::BL | Edge::<3>::LD,
            Corner::<3>::BDR => edge |= Edge::<3>::BD | Edge::<3>::BR | Edge::<3>::RD,
            Corner::<3>::BUL => edge |= Edge::<3>::BU | Edge::<3>::BL | Edge::<3>::LU,
            Corner::<3>::BUR => edge |= Edge::<3>::BU | Edge::<3>::BR | Edge::<3>::RU,
            t => unreachable!("{t:?}"),
        }

        edge
    }
}
