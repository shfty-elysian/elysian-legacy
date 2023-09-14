use std::ops::{BitAnd, BitOr, BitXor, Not};

use super::{Corner, Faces, Power2U8, ToCorners};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Face<const D: usize>(pub Power2U8);

impl<const D: usize> Face<D> {
    pub const unsafe fn new_unchecked(n: u8) -> Self {
        Self(Power2U8::new_unchecked(n))
    }

    pub fn new(n: u8) -> Option<Self> {
        Some(Self(Power2U8::new(n)?))
    }
}

impl Face<2> {
    pub const L: Self = unsafe { Self::new_unchecked(1) };
    pub const R: Self = unsafe { Self::new_unchecked(2) };
    pub const U: Self = unsafe { Self::new_unchecked(4) };
    pub const D: Self = unsafe { Self::new_unchecked(8) };
}

impl Face<3> {
    pub const L: Self = unsafe { Self::new_unchecked(1) };
    pub const R: Self = unsafe { Self::new_unchecked(2) };
    pub const U: Self = unsafe { Self::new_unchecked(4) };
    pub const D: Self = unsafe { Self::new_unchecked(8) };
    pub const F: Self = unsafe { Self::new_unchecked(16) };
    pub const B: Self = unsafe { Self::new_unchecked(32) };
}

impl<const D: usize> BitAnd for Face<D> {
    type Output = Faces<D>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Faces(self.0 & rhs.0)
    }
}

impl<const D: usize> BitOr for Face<D> {
    type Output = Faces<D>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Faces(self.0 | rhs.0)
    }
}

impl<const D: usize> BitXor for Face<D> {
    type Output = Faces<D>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Faces(self.0 ^ rhs.0)
    }
}

impl<const D: usize> Not for Face<D> {
    type Output = Faces<D>;

    fn not(self) -> Self::Output {
        Faces(!self.0)
    }
}

impl ToCorners<2> for Face<2> {
    fn to_corners(&self) -> super::Corners<2> {
        match *self {
            Self::L => Corner::<2>::DL | Corner::<2>::UL,
            Self::R => Corner::<2>::DR | Corner::<2>::UR,
            Self::U => Corner::<2>::UL | Corner::<2>::UR,
            Self::D => Corner::<2>::DL | Corner::<2>::DR,
            _ => unreachable!(),
        }
    }
}

impl ToCorners<3> for Face<3> {
    fn to_corners(&self) -> super::Corners<3> {
        match *self {
            Self::L => Corner::<3>::FDL | Corner::<3>::FUL | Corner::<3>::BDL | Corner::<3>::BUL,
            Self::R => Corner::<3>::FDR | Corner::<3>::FUR | Corner::<3>::BDR | Corner::<3>::BUR,
            Self::U => Corner::<3>::FUL | Corner::<3>::FUR | Corner::<3>::BUL | Corner::<3>::BUR,
            Self::D => Corner::<3>::FDL | Corner::<3>::FDR | Corner::<3>::BDL | Corner::<3>::BDR,
            Self::F => Corner::<3>::FUL | Corner::<3>::FUR | Corner::<3>::FDL | Corner::<3>::FDR,
            Self::B => Corner::<3>::BUL | Corner::<3>::BUR | Corner::<3>::BDL | Corner::<3>::BDR,
            _ => unreachable!(),
        }
    }
}
