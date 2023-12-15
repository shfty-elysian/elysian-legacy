use crate::{
    bounds::Bounds,
    dual_graph::{AsDualGraph, DualPair},
    has_sign_change::HasSignChange,
    marching_cells::Corners,
    neighbours::Neighbours,
    sample::Sample,
    subdivision_tree::{SubdivisionCell, SubdivisionTree},
    vector_space::VectorSpace,
};

#[derive(Default, Clone)]
pub struct DualContour<'a, D>(Vec<[&'a SubdivisionCell<D>; 2]>)
where
    D: VectorSpace<f64>;

impl<'a, D> FromIterator<[&'a SubdivisionCell<D>; 2]> for DualContour<'a, D>
where
    D: VectorSpace<f64>,
{
    fn from_iter<T: IntoIterator<Item = [&'a SubdivisionCell<D>; 2]>>(iter: T) -> Self {
        DualContour(iter.into_iter().collect())
    }
}

impl<'a, D> IntoIterator for DualContour<'a, D>
where
    D: VectorSpace<f64>,
{
    type Item = [&'a SubdivisionCell<D>; 2];

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, D> DualContour<'a, D>
where
    D: VectorSpace<f64>,
{
    pub fn iter(&self) -> impl Iterator<Item = &[&'a SubdivisionCell<D>; 2]> {
        self.0.iter()
    }

    pub fn contours(&self) -> impl Iterator<Item = &[&'a SubdivisionCell<D>; 2]> {
        self.iter()
            .filter(|[a, b]| a.is_contour() && b.is_contour())
    }
}

pub trait AsDualContour<D>: AsDualGraph<D>
where
    D: VectorSpace<f64>,
{
    fn as_dual_contour<'a, 'b>(&'a self, evaluator: &impl Sample<'b, D>) -> DualContour<'a, D>;
}

impl<D> AsDualContour<D> for SubdivisionTree<D>
where
    Self: AsDualGraph<D>,
    Corners<D>: Neighbours<D> + HasSignChange<D>,
    D: VectorSpace<f64>,
    Bounds<D>: IntoIterator<Item = D::DimensionVector>,
{
    fn as_dual_contour<'a, 'b>(&'a self, evaluator: &impl Sample<'b, D>) -> DualContour<'a, D> {
        self.as_dual_graph()
            .contours()
            // Filter connected contour cells
            .filter_map(|DualPair { face, lhs, rhs }| {
                let ca = lhs.bounds.sample_corners(evaluator).unwrap();
                let cb = rhs.bounds.sample_corners(evaluator).unwrap();

                if !ca.has_sign_change(&face) || !ca.neighbours(&cb, &face) {
                    return None;
                }

                Some([*lhs, *rhs])
            })
            .collect()
    }
}
