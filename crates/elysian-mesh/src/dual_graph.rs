use crate::{
    has_sign_change::HasSignChange,
    marching_cells::{Corners, Face},
    neighbours::Neighbours,
    subdivision_tree::SubdivisionCell,
    vector_space::VectorSpace,
};

#[derive(Copy, Clone)]
pub struct DualPair<'a, D>
where
    D: VectorSpace<f64>,
{
    pub face: Face<D>,
    pub lhs: &'a SubdivisionCell<D>,
    pub rhs: &'a SubdivisionCell<D>,
}

impl<'a, D> IntoIterator for DualPair<'a, D>
where
    D: VectorSpace<f64>,
{
    type Item = &'a SubdivisionCell<D>;

    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.lhs, self.rhs].into_iter()
    }
}

impl<'a, D> DualPair<'a, D>
where
    D: VectorSpace<f64>,
{
    pub fn iter(&self) -> impl Iterator<Item = &'a SubdivisionCell<D>> {
        [self.lhs, self.rhs].into_iter()
    }
}

#[derive(Default, Clone)]
pub struct DualGraph<'a, D>(Vec<DualPair<'a, D>>)
where
    D: VectorSpace<f64>;

impl<'a, D> FromIterator<DualPair<'a, D>> for DualGraph<'a, D>
where
    D: VectorSpace<f64>,
{
    fn from_iter<T: IntoIterator<Item = DualPair<'a, D>>>(iter: T) -> Self {
        DualGraph(iter.into_iter().collect())
    }
}

impl<'a, D> IntoIterator for DualGraph<'a, D>
where
    D: VectorSpace<f64>,
{
    type Item = DualPair<'a, D>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'d, D> DualGraph<'d, D>
where
    D: VectorSpace<f64>,
    Corners<D>: HasSignChange<D> + Neighbours<D>,
{
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a DualPair<'d, D>> {
        self.0.iter()
    }

    pub fn contours<'a>(&'a self) -> impl Iterator<Item = &'a DualPair<'d, D>> {
        self.iter()
            .filter(|DualPair { lhs, rhs, .. }| lhs.is_contour() && rhs.is_contour())
    }
}

pub trait AsDualGraph<D>
where
    D: VectorSpace<f64>,
{
    fn as_dual_graph<'a>(&'a self) -> DualGraph<'a, D>;
}
