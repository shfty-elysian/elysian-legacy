use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXor, Not},
};

use nalgebra::{Vector2, Vector3};

use crate::vector_space::{DimensionArray, D2, D3};

use super::{Corner, Corners, Edges, Point, Power2U16, ToCorners};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge<D>(pub Power2U16, PhantomData<D>);

impl<D> Edge<D> {
    pub const unsafe fn new_unchecked(n: u16) -> Self {
        Self(Power2U16::new_unchecked(n), PhantomData)
    }

    pub fn new(n: u16) -> Option<Self> {
        Some(Self(Power2U16::new(n)?, PhantomData))
    }
}

impl Edge<D2> {
    pub const L: Self = unsafe { Self::new_unchecked(1) };
    pub const R: Self = unsafe { Self::new_unchecked(2) };
    pub const U: Self = unsafe { Self::new_unchecked(4) };
    pub const D: Self = unsafe { Self::new_unchecked(8) };
}

impl Edge<D3> {
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

impl<D> BitAnd for Edge<D> {
    type Output = Edges<D>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Edges::new(self.0 & rhs.0)
    }
}

impl<D> BitOr for Edge<D> {
    type Output = Edges<D>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Edges::new(self.0 | rhs.0)
    }
}

impl<D> BitXor for Edge<D> {
    type Output = Edges<D>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Edges::new(self.0 ^ rhs.0)
    }
}

impl<D> Not for Edge<D> {
    type Output = Edges<D>;

    fn not(self) -> Self::Output {
        Edges::new(!self.0)
    }
}

impl ToCorners<D2> for Edge<D2> {
    fn to_corners(&self) -> Corners<D2> {
        let mut corner = Corners::<D2>::EMPTY;

        match *self {
            Edge::<D2>::L => corner |= Corner::<D2>::DL | Corner::<D2>::UL,
            Edge::<D2>::R => corner |= Corner::<D2>::DR | Corner::<D2>::UR,
            Edge::<D2>::U => corner |= Corner::<D2>::UL | Corner::<D2>::UR,
            Edge::<D2>::D => corner |= Corner::<D2>::DL | Corner::<D2>::DR,
            _ => unreachable!(),
        }

        corner
    }
}

impl ToCorners<D3> for Edge<D3> {
    fn to_corners(&self) -> Corners<D3> {
        let mut corner = Corners::<D3>::EMPTY;

        match *self {
            Edge::<D3>::FL => corner |= Corner::<D3>::FDL | Corner::<D3>::FUL,
            Edge::<D3>::FD => corner |= Corner::<D3>::FDL | Corner::<D3>::FDR,
            Edge::<D3>::FU => corner |= Corner::<D3>::FUL | Corner::<D3>::FUR,
            Edge::<D3>::FR => corner |= Corner::<D3>::FDR | Corner::<D3>::FUR,

            Edge::<D3>::BL => corner |= Corner::<D3>::BDL | Corner::<D3>::BUL,
            Edge::<D3>::BD => corner |= Corner::<D3>::BDL | Corner::<D3>::BDR,
            Edge::<D3>::BU => corner |= Corner::<D3>::BUL | Corner::<D3>::BUR,
            Edge::<D3>::BR => corner |= Corner::<D3>::BDR | Corner::<D3>::BUR,

            Edge::<D3>::LD => corner |= Corner::<D3>::FDL | Corner::<D3>::BDL,
            Edge::<D3>::RD => corner |= Corner::<D3>::FDR | Corner::<D3>::BDR,
            Edge::<D3>::LU => corner |= Corner::<D3>::FUL | Corner::<D3>::BUL,
            Edge::<D3>::RU => corner |= Corner::<D3>::FUR | Corner::<D3>::BUR,

            _ => unreachable!(),
        }

        corner
    }
}

impl Point<D2> for Edge<D2> {
    fn point(&self) -> <D2 as DimensionArray>::DimensionArray<f64> {
        (self
            .to_corners()
            .into_iter()
            .map(|corner| Vector2::<_>::from(corner.point()))
            .sum::<Vector2<_>>()
            / 2.0)
            .into()
    }
}

impl Point<D3> for Edge<D3> {
    fn point(&self) -> <D3 as DimensionArray>::DimensionArray<f64> {
        (self
            .to_corners()
            .into_iter()
            .map(|corner| Vector3::<_>::from(corner.point()))
            .sum::<Vector3<_>>()
            / 2.0)
            .into()
    }
}
