use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use nalgebra::{Vector2, Vector3};

use crate::vector_space::{DimensionArray, D2, D3};

use super::{edge::Edge, All, Corner, Corners, Point, ToCorners};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edges<const D: usize>(pub u16);

impl<const D: usize> Edges<D> {
    pub const EMPTY: Edges<D> = Edges(0);
}

impl IntoIterator for Edges<2> {
    type Item = Edge<2>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..4).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is 1 here, so v << i will always be a power of two
                Some(unsafe { Edge::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl IntoIterator for Edges<3> {
    type Item = Edge<3>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..16).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is 1 here, so v << i will always be a power of two
                Some(unsafe { Edge::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl All for Edges<2> {
    fn all() -> Self {
        Edge::L | Edge::R | Edge::U | Edge::D
    }
}

impl All for Edges<3> {
    fn all() -> Self {
        Edge::FL
            | Edge::FR
            | Edge::FU
            | Edge::FD
            | Edge::BL
            | Edge::BR
            | Edge::BU
            | Edge::BD
            | Edge::LD
            | Edge::RD
            | Edge::LU
            | Edge::RU
    }
}

impl ToCorners<2> for Edges<2> {
    fn to_corners(&self) -> Corners<2> {
        let mut corner = Corners::<2>::EMPTY;

        for edge in self.into_iter() {
            match edge {
                Edge::<2>::L => corner |= Corner::<2>::DL | Corner::<2>::UL,
                Edge::<2>::R => corner |= Corner::<2>::DR | Corner::<2>::UR,
                Edge::<2>::U => corner |= Corner::<2>::UL | Corner::<2>::UR,
                Edge::<2>::D => corner |= Corner::<2>::DL | Corner::<2>::DR,
                t => unreachable!("{t:?}"),
            }
        }

        corner
    }
}

impl ToCorners<3> for Edges<3> {
    fn to_corners(&self) -> Corners<3> {
        let mut corner = Corners::<3>::EMPTY;

        for edge in self.into_iter() {
            match edge {
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

                t => unreachable!("{t:?}"),
            }
        }

        corner
    }
}

impl Point<D2> for Edges<2> {
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

impl Point<D3> for Edges<3> {
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

impl<const D: usize> BitAnd for Edges<D> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Edges(self.0 & rhs.0)
    }
}

impl<const D: usize> BitAndAssign for Edges<D> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<const D: usize> BitOr for Edges<D> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Edges(self.0 | rhs.0)
    }
}

impl<const D: usize> BitOrAssign for Edges<D> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<const D: usize> BitXor for Edges<D> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Edges(self.0 ^ rhs.0)
    }
}

impl<const D: usize> BitXorAssign for Edges<D> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<const D: usize> BitAnd<Edge<D>> for Edges<D> {
    type Output = Self;

    fn bitand(self, rhs: Edge<D>) -> Self::Output {
        Edges(self.0 & rhs.0.get())
    }
}

impl<const D: usize> BitOr<Edge<D>> for Edges<D> {
    type Output = Self;

    fn bitor(self, rhs: Edge<D>) -> Self::Output {
        Edges(self.0 | rhs.0.get())
    }
}

impl<const D: usize> BitXor<Edge<D>> for Edges<D> {
    type Output = Self;

    fn bitxor(self, rhs: Edge<D>) -> Self::Output {
        Edges(self.0 ^ rhs.0.get())
    }
}

impl<const D: usize> Not for Edges<D> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Edges(!self.0)
    }
}

pub trait ToEdges<const D: usize> {
    fn to_edges(&self) -> Edges<D>;
}

