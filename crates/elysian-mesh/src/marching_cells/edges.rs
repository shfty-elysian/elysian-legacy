use std::{ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not}, marker::PhantomData};

use nalgebra::{Vector2, Vector3};

use crate::vector_space::{DimensionArray, D2, D3};

use super::{edge::Edge, All, Corner, Corners, Point, ToCorners};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edges<D>(pub u16, PhantomData<D>);

impl<D> Edges<D> {
    pub const EMPTY: Edges<D> = Edges(0, PhantomData);

    pub fn new(edges: u16) -> Self {
        Edges(edges, PhantomData)
    }
}

impl IntoIterator for Edges<D2> {
    type Item = Edge<D2>;

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

impl IntoIterator for Edges<D3> {
    type Item = Edge<D3>;

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

impl All for Edges<D2> {
    fn all() -> Self {
        Edge::L | Edge::R | Edge::U | Edge::D
    }
}

impl All for Edges<D3> {
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

impl ToCorners<D2> for Edges<D2> {
    fn to_corners(&self) -> Corners<D2> {
        let mut corner = Corners::<D2>::EMPTY;

        for edge in self.into_iter() {
            match edge {
                Edge::<D2>::L => corner |= Corner::<D2>::DL | Corner::<D2>::UL,
                Edge::<D2>::R => corner |= Corner::<D2>::DR | Corner::<D2>::UR,
                Edge::<D2>::U => corner |= Corner::<D2>::UL | Corner::<D2>::UR,
                Edge::<D2>::D => corner |= Corner::<D2>::DL | Corner::<D2>::DR,
                t => unreachable!("{t:?}"),
            }
        }

        corner
    }
}

impl ToCorners<D3> for Edges<D3> {
    fn to_corners(&self) -> Corners<D3> {
        let mut corner = Corners::<D3>::EMPTY;

        for edge in self.into_iter() {
            match edge {
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

                t => unreachable!("{t:?}"),
            }
        }

        corner
    }
}

impl Point<D2> for Edges<D2> {
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

impl Point<D3> for Edges<D3> {
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

impl<D> BitAnd for Edges<D> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Edges(self.0 & rhs.0, PhantomData)
    }
}

impl<D> BitAndAssign for Edges<D> where D: Copy {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<D> BitOr for Edges<D> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Edges(self.0 | rhs.0, PhantomData)
    }
}

impl<D> BitOrAssign for Edges<D> where D: Copy {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<D> BitXor for Edges<D> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Edges(self.0 ^ rhs.0, PhantomData)
    }
}

impl<D> BitXorAssign for Edges<D> where D: Copy {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<D> BitAnd<Edge<D>> for Edges<D> {
    type Output = Self;

    fn bitand(self, rhs: Edge<D>) -> Self::Output {
        Edges(self.0 & rhs.0.get(), PhantomData)
    }
}

impl<D> BitOr<Edge<D>> for Edges<D> {
    type Output = Self;

    fn bitor(self, rhs: Edge<D>) -> Self::Output {
        Edges(self.0 | rhs.0.get(), PhantomData)
    }
}

impl<D> BitXor<Edge<D>> for Edges<D> {
    type Output = Self;

    fn bitxor(self, rhs: Edge<D>) -> Self::Output {
        Edges(self.0 ^ rhs.0.get(), PhantomData)
    }
}

impl<D> Not for Edges<D> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Edges(!self.0, PhantomData)
    }
}

pub trait ToEdges<D> {
    fn to_edges(&self) -> Edges<D>;
}
