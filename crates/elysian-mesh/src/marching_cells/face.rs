use std::{ops::{BitAnd, BitOr, BitXor, Not}, marker::PhantomData};

use crate::vector_space::{D2, D3};

use super::{Corner, Faces, Power2U8, ToCorners};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Face<D>(pub Power2U8, pub PhantomData<D>);

impl<D> Face<D> {
    pub const unsafe fn new_unchecked(n: u8) -> Self {
        Self(Power2U8::new_unchecked(n), PhantomData)
    }

    pub fn new(n: u8) -> Option<Self> {
        Some(Self(Power2U8::new(n)?, PhantomData))
    }
}

impl Face<D2> {
    pub const L: Self = unsafe { Self::new_unchecked(1) };
    pub const R: Self = unsafe { Self::new_unchecked(2) };
    pub const U: Self = unsafe { Self::new_unchecked(4) };
    pub const D: Self = unsafe { Self::new_unchecked(8) };
}

impl Face<D3> {
    pub const L: Self = unsafe { Self::new_unchecked(1) };
    pub const R: Self = unsafe { Self::new_unchecked(2) };
    pub const U: Self = unsafe { Self::new_unchecked(4) };
    pub const D: Self = unsafe { Self::new_unchecked(8) };
    pub const F: Self = unsafe { Self::new_unchecked(16) };
    pub const B: Self = unsafe { Self::new_unchecked(32) };
}

impl<D> BitAnd for Face<D> {
    type Output = Faces<D>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Faces::new(self.0 & rhs.0)
    }
}

impl<D> BitOr for Face<D> {
    type Output = Faces<D>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Faces::new(self.0 | rhs.0)
    }
}

impl<D> BitXor for Face<D> {
    type Output = Faces<D>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Faces::new(self.0 ^ rhs.0)
    }
}

impl<D> Not for Face<D> {
    type Output = Faces<D>;

    fn not(self) -> Self::Output {
        Faces::new(!self.0)
    }
}

impl ToCorners<D2> for Face<D2> {
    fn to_corners(&self) -> super::Corners<D2> {
        match *self {
            Self::L => Corner::<D2>::DL | Corner::<D2>::UL,
            Self::R => Corner::<D2>::DR | Corner::<D2>::UR,
            Self::U => Corner::<D2>::UL | Corner::<D2>::UR,
            Self::D => Corner::<D2>::DL | Corner::<D2>::DR,
            _ => unreachable!(),
        }
    }
}

impl ToCorners<D3> for Face<D3> {
    fn to_corners(&self) -> super::Corners<D3> {
        match *self {
            Self::L => {
                Corner::<D3>::FDL | Corner::<D3>::FUL | Corner::<D3>::BDL | Corner::<D3>::BUL
            }
            Self::R => {
                Corner::<D3>::FDR | Corner::<D3>::FUR | Corner::<D3>::BDR | Corner::<D3>::BUR
            }
            Self::U => {
                Corner::<D3>::FUL | Corner::<D3>::FUR | Corner::<D3>::BUL | Corner::<D3>::BUR
            }
            Self::D => {
                Corner::<D3>::FDL | Corner::<D3>::FDR | Corner::<D3>::BDL | Corner::<D3>::BDR
            }
            Self::F => {
                Corner::<D3>::FUL | Corner::<D3>::FUR | Corner::<D3>::FDL | Corner::<D3>::FDR
            }
            Self::B => {
                Corner::<D3>::BUL | Corner::<D3>::BUR | Corner::<D3>::BDL | Corner::<D3>::BDR
            }
            _ => unreachable!(),
        }
    }
}
