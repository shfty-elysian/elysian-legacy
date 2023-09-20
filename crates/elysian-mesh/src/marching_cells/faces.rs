use std::{
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::vector_space::{D2, D3};

use super::{face::Face, All};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Faces<D>(pub u8, PhantomData<D>);

impl<D> Faces<D> {
    pub const EMPTY: Self = Self::new(0);

    pub const fn new(faces: u8) -> Self {
        Self(faces, PhantomData)
    }
}

impl IntoIterator for Faces<D2> {
    type Item = Face<D2>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..4).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is either 0 or 1, so v << i will always be a power of two
                Some(unsafe { Face::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl IntoIterator for Faces<D3> {
    type Item = Face<D3>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..8).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is either 0 or 1, so v << i will always be a power of two
                Some(unsafe { Face::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl All for Faces<D2> {
    fn all() -> Self {
        Face::<D2>::L | Face::<D2>::R | Face::<D2>::U | Face::<D2>::D
    }
}

impl All for Faces<D3> {
    fn all() -> Self {
        Face::<D3>::L
            | Face::<D3>::R
            | Face::<D3>::U
            | Face::<D3>::D
            | Face::<D3>::F
            | Face::<D3>::B
    }
}

impl<D> BitAnd<Face<D>> for Faces<D> {
    type Output = Self;

    fn bitand(self, rhs: Face<D>) -> Self::Output {
        Self::new(self.0 & rhs.0.get())
    }
}

impl<D> BitOr<Face<D>> for Faces<D> {
    type Output = Self;

    fn bitor(self, rhs: Face<D>) -> Self::Output {
        Self::new(self.0 | rhs.0.get())
    }
}

impl<D> BitXor<Face<D>> for Faces<D> {
    type Output = Self;

    fn bitxor(self, rhs: Face<D>) -> Self::Output {
        Self::new(self.0 ^ rhs.0.get())
    }
}

impl<D> BitAnd for Faces<D> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.0 & rhs.0)
    }
}

impl<D> BitAndAssign for Faces<D> where D: Copy {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<D> BitOr for Faces<D> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 | rhs.0)
    }
}

impl<D> BitOrAssign for Faces<D> where D: Copy {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<D> BitXor for Faces<D> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 ^ rhs.0)
    }
}

impl<D> BitXorAssign for Faces<D> where D: Copy {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<D> Not for Faces<D> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(!self.0)
    }
}
