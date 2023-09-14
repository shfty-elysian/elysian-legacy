use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use super::{face::Face, All};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Faces<const D: usize>(pub u8);

impl<const D: usize> Faces<D> {
    pub const EMPTY: Self = Faces(0);
}

impl IntoIterator for Faces<2> {
    type Item = Face<2>;

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

impl IntoIterator for Faces<3> {
    type Item = Face<3>;

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

impl All for Faces<2> {
    fn all() -> Self {
        Face::<2>::L | Face::<2>::R | Face::<2>::U | Face::<2>::D
    }
}

impl All for Faces<3> {
    fn all() -> Self {
        Face::<3>::L | Face::<3>::R | Face::<3>::U | Face::<3>::D | Face::<3>::F | Face::<3>::B
    }
}

impl<const D: usize> BitAnd<Face<D>> for Faces<D> {
    type Output = Self;

    fn bitand(self, rhs: Face<D>) -> Self::Output {
        Self(self.0 & rhs.0.get())
    }
}

impl<const D: usize> BitOr<Face<D>> for Faces<D> {
    type Output = Self;

    fn bitor(self, rhs: Face<D>) -> Self::Output {
        Self(self.0 | rhs.0.get())
    }
}

impl<const D: usize> BitXor<Face<D>> for Faces<D> {
    type Output = Self;

    fn bitxor(self, rhs: Face<D>) -> Self::Output {
        Self(self.0 ^ rhs.0.get())
    }
}

impl<const D: usize> BitAnd for Faces<D> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<const D: usize> BitAndAssign for Faces<D> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<const D: usize> BitOr for Faces<D> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl<const D: usize> BitOrAssign for Faces<D> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<const D: usize> BitXor for Faces<D> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl<const D: usize> BitXorAssign for Faces<D> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<const D: usize> Not for Faces<D> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
