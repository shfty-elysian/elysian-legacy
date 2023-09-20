use std::{
    collections::BTreeMap,
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::vector_space::{D2, D3};

use super::{edge::Edge, All, Cell, Corner, Edges, ToEdges};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Corners<D>(pub u8, PhantomData<D>);

impl<D> Corners<D> {
    pub const EMPTY: Self = Corners::new(0);

    pub const fn new(corners: u8) -> Self {
        Corners(corners, PhantomData)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == Self::EMPTY.0
    }
}

impl IntoIterator for Corners<D2> {
    type Item = Corner<D2>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..4).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is either 0 or 1, so v << i will always be a power of two
                Some(unsafe { Corner::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl IntoIterator for Corners<D3> {
    type Item = Corner<D3>;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut cand = self.0;
        Box::new((0..8).into_iter().flat_map(move |i| {
            let v = cand & 1;
            cand = cand >> 1;
            if v > 0 {
                // Safety: v is either 0 or 1, so v << i will always be a power of two
                Some(unsafe { Corner::new_unchecked(v << i) })
            } else {
                None
            }
        }))
    }
}

impl All for Corners<D2> {
    fn all() -> Self {
        Corner::<D2>::DL | Corner::<D2>::DR | Corner::<D2>::UL | Corner::<D2>::UR
    }
}

impl All for Corners<D3> {
    fn all() -> Self {
        Corner::<D3>::FDL
            | Corner::<D3>::FDR
            | Corner::<D3>::FUL
            | Corner::<D3>::FUR
            | Corner::<D3>::BDL
            | Corner::<D3>::BDR
            | Corner::<D3>::BUL
            | Corner::<D3>::BUR
    }
}

impl<D> ToEdges<D> for Corners<D>
where
    Corners<D>: IntoIterator<Item = Corner<D>>,
    Corner<D>: ToEdges<D>,
    D: Copy,
{
    fn to_edges(&self) -> Edges<D> {
        self.into_iter()
            .fold(Edges::<D>::EMPTY, |acc, next| acc | next.to_edges())
    }
}

impl<D> BitAnd<Corner<D>> for Corners<D> {
    type Output = Self;

    fn bitand(self, rhs: Corner<D>) -> Self::Output {
        Self::new(self.0 & rhs.0.get())
    }
}

impl<D> BitOr<Corner<D>> for Corners<D> {
    type Output = Self;

    fn bitor(self, rhs: Corner<D>) -> Self::Output {
        Self::new(self.0 | rhs.0.get())
    }
}

impl<D> BitXor<Corner<D>> for Corners<D> {
    type Output = Self;

    fn bitxor(self, rhs: Corner<D>) -> Self::Output {
        Self::new(self.0 ^ rhs.0.get())
    }
}

impl<D> BitAnd for Corners<D> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.0 & rhs.0)
    }
}

impl<D> BitAndAssign for Corners<D> where D: Copy {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<D> BitOr for Corners<D> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 | rhs.0)
    }
}

impl<D> BitOrAssign for Corners<D> where D: Copy {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<D> BitXor for Corners<D> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 ^ rhs.0)
    }
}

impl<D> BitXorAssign for Corners<D>
where
    D: Copy,
{
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<D> Not for Corners<D> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(!self.0)
    }
}

impl<D> Cell<D> for Corners<D>
where
    Corners<D>: Copy + IntoIterator<Item = Corner<D>>,
    Corner<D>: ToEdges<D>,
    Edges<D>: Copy + All + IntoIterator<Item = Edge<D>>,
    Edge<D>: Copy + ToCorners<D>,
    D: Copy + Ord,
{
    fn cell(&self) -> BTreeMap<Corner<D>, Edges<D>> {
        let mut edges = self.to_edges();

        for edge in <Edges<D> as All>::all().into_iter() {
            if *self & edge.to_corners() == edge.to_corners() {
                edges &= !edge;
            }
        }

        let mut cell = BTreeMap::new();

        for corner in (*self).into_iter() {
            let mut out = Edges::<D>::EMPTY;
            for edge in edges.into_iter() {
                if (corner.to_edges() & edge).0 == edge.0.get() {
                    out = out | edge;
                }
            }
            if out != Edges::EMPTY {
                cell.insert(corner, out);
            }
        }

        cell
    }
}

pub trait ToCorners<D> {
    fn to_corners(&self) -> Corners<D>;
}
