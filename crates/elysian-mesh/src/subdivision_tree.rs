use std::{
    hash::{Hash, Hasher},
    ops::Mul,
};

use crate::{
    bounds::Bounds,
    marching_cells::{Corner, Corners, Face, ToCorners},
    sample::Sample,
    tree::Tree,
    vector_space::{DimensionArray, DimensionVector, SubdivisionArray, D2, D3},
};
use elysian_ir::{
    ast::DISTANCE,
    module::{Evaluate, EvaluateError},
};

pub struct SubdivisionCell<D: DimensionVector<f64>> {
    pub bounds: Bounds<D>,
    pub ty: CellType,
}

impl<D> SubdivisionCell<D>
where
    D: DimensionVector<f64>,
{
    pub fn is_contour(&self) -> bool {
        matches!(self.ty, CellType::Contour)
    }
}

impl std::fmt::Debug for SubdivisionCell<D2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubdivisionCell")
            .field("bounds", &self.bounds)
            .field("ty", &self.ty)
            .finish()
    }
}

impl std::fmt::Debug for SubdivisionCell<D3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubdivisionCell")
            .field("bounds", &self.bounds)
            .field("ty", &self.ty)
            .finish()
    }
}

impl<D> Clone for SubdivisionCell<D>
where
    D: DimensionVector<f64>,
{
    fn clone(&self) -> Self {
        Self {
            bounds: self.bounds.clone(),
            ty: self.ty.clone(),
        }
    }
}

impl<D> Copy for SubdivisionCell<D>
where
    D: DimensionVector<f64>,
    D::DimensionVector: Copy,
{
}

impl<D> PartialEq for SubdivisionCell<D>
where
    D: DimensionVector<f64>,
    D::DimensionVector: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.bounds == other.bounds && self.ty == other.ty
    }
}

impl<D> PartialOrd for SubdivisionCell<D>
where
    D: DimensionVector<f64>,
    D::DimensionVector: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ty.partial_cmp(&other.ty) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.bounds.partial_cmp(&other.bounds)
    }
}

impl<D> Hash for SubdivisionCell<D>
where
    D: DimensionVector<f64>,
    D::DimensionVector: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bounds.hash(state);
        self.ty.hash(state);
    }
}

/// State of a cell in an implicit surface sampling grid
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CellType {
    /// Cell is fully outside the isosurface
    Empty,
    /// Cell intersects the isosurface
    Contour,
    /// Cell is fully inside the isosurface
    Full,
}

pub type SubdivisionTree<D> = Tree<SubdivisionCell<D>, D>;

pub type QuadTree = SubdivisionTree<D2>;
pub type Octree = SubdivisionTree<D3>;

impl<D> SubdivisionTree<D>
where
    D: SubdivisionArray<Box<SubdivisionTree<D>>> + DimensionVector<f64>,
    D::DimensionVector: Mul<f64, Output = D::DimensionVector>,
{
    /// Build a full-density subdivision tree with the provided bounds and sampling level
    pub fn new(bounds: Bounds<D>, level: usize) -> Self {
        let size = bounds.size();

        let hsize = size * 0.5;

        if level > 0 {
            let mut leaves = vec![];

            let positions = D::subdivision_indices();

            for p in positions {
                let min = D::component_add(
                    &bounds.min,
                    &D::component_mul(
                        &hsize,
                        &D::from_vec(p.into_iter().map(|t| t as f64).collect()),
                    ),
                );
                let max = D::component_add(&min, &hsize);
                leaves.push(Box::new(Self::new(Bounds { min, max }, level - 1)));
            }

            Self::Root(leaves.try_into().ok().expect("Invalid root length"))
        } else {
            Self::Leaf(SubdivisionCell {
                bounds,
                ty: CellType::Contour,
            })
        }
    }

    /// Retrieve the tree's min corner via recursion
    pub fn min_corner(&self) -> D::DimensionVector {
        match self {
            Tree::Root(root) => Self::min_corner(D::iter(root).next().unwrap()),
            Tree::Leaf(leaf) => leaf.bounds.min.clone(),
        }
    }

    /// Retrieve the tree's max corner via recursion
    pub fn max_corner(&self) -> D::DimensionVector {
        match self {
            Tree::Root(root) => Self::max_corner(D::iter(root).last().unwrap()),
            Tree::Leaf(leaf) => leaf.bounds.max.clone(),
        }
    }

    /// Retrieve the tree's bounds via recursion
    pub fn bounds(&self) -> Bounds<D>
    where
        D: SubdivisionArray<Box<SubdivisionTree<D>>> + DimensionVector<f64>,
    {
        Bounds {
            min: self.min_corner(),
            max: self.max_corner(),
        }
    }

    pub fn center(&self) -> D::DimensionVector {
        match self {
            Tree::Root(_) => {
                let Bounds { min, max } = self.bounds();
                (min + max) * 0.5
            }
            Tree::Leaf(leaf) => leaf.bounds.center(),
        }
    }

    /// Given a sampling function and an epsilon,
    /// merge cells whose local error versus linear interpolation falls below the given threshold
    pub fn merge<'a>(
        self,
        evaluator: &impl Sample<'a, D>,
        epsilon: f64,
    ) -> Result<Self, EvaluateError>
    where
        Self: FaceCenters<D>,
        Bounds<D>: InterpolateCell<D>,
    {
        fn score<'a, D: DimensionVector<f64>>(
            evaluator: &impl Sample<'a, D>,
            bounds: Bounds<D>,
            p: D::DimensionVector,
        ) -> Result<f64, EvaluateError>
        where
            Bounds<D>: InterpolateCell<D>,
        {
            Ok((bounds.interpolate(evaluator, p.clone())?
                - f64::from(Sample::<D>::sample(evaluator, p)?.get(&DISTANCE.into())))
            .abs())
        }

        Ok(match self {
            Tree::Root(root) => {
                let root: Vec<_> = root
                    .into_iter()
                    .map(|t| t.merge(evaluator, epsilon))
                    .collect::<Result<Vec<_>, _>>()?;

                let root: <D as SubdivisionArray<Box<SubdivisionTree<D>>>>::SubdivisionArray = root
                    .into_iter()
                    .map(Box::new)
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()
                    .unwrap();

                if !D::iter(&root).all(|t| matches!(**t, Tree::Leaf(_))) {
                    return Ok(Tree::Root(root));
                }

                let tree = Tree::Root(root);

                let face_centers = tree.face_centers();
                let center = tree.center();
                let (min, max) = (tree.min_corner(), tree.max_corner());

                let scores = face_centers
                    .into_iter()
                    .chain([center])
                    .map(|t| {
                        score(
                            evaluator,
                            Bounds::<D> {
                                min: min.clone(),
                                max: max.clone(),
                            },
                            t,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                if scores.iter().all(|score| *score < epsilon) {
                    Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min, max },
                        ty: CellType::Contour,
                    })
                } else {
                    tree
                }
            }
            _ => self,
        })
    }

    /// Given a sampling function, collapse Leaf cells into Full and Empty variants
    pub fn collapse<'a, E>(self, evaluator: &E) -> Result<Self, EvaluateError>
    where
        E: Sample<'a, D>,
        Bounds<D>: IntoIterator<Item = D::DimensionVector>,
        D: DimensionArray<f64>,
    {
        let Bounds { min, max } = self.bounds();

        Ok(match self {
            Self::Leaf(SubdivisionCell { bounds, .. }) => {
                let samples = bounds
                    .into_iter()
                    .map(|p| evaluator.sample(p))
                    .collect::<Result<Vec<_>, _>>()?;

                Self::Leaf(SubdivisionCell {
                    bounds: Bounds { min, max },
                    ty: if samples
                        .iter()
                        .all(|t| f64::from(t.get(&DISTANCE.into())) <= 0.0)
                    {
                        CellType::Full
                    } else if samples
                        .iter()
                        .all(|t| f64::from(t.get(&DISTANCE.into())) > 0.0)
                    {
                        CellType::Empty
                    } else {
                        CellType::Contour
                    },
                })
            }
            Self::Root(leaves) => {
                let leaves = leaves
                    .into_iter()
                    .map(|t| t.collapse(evaluator).map(Box::new))
                    .collect::<Result<Vec<_>, _>>()?;

                if leaves.iter().all(|leaf| {
                    matches!(
                        **leaf,
                        Tree::Leaf(SubdivisionCell {
                            ty: CellType::Empty,
                            ..
                        })
                    )
                }) {
                    Self::Leaf(SubdivisionCell {
                        bounds: Bounds { min, max },
                        ty: CellType::Empty,
                    })
                } else if leaves.iter().all(|leaf| {
                    matches!(
                        **leaf,
                        Tree::Leaf(SubdivisionCell {
                            ty: CellType::Full,
                            ..
                        })
                    )
                }) {
                    Self::Leaf(SubdivisionCell {
                        bounds: Bounds { min, max },
                        ty: CellType::Full,
                    })
                } else {
                    Self::Root(leaves.try_into().ok().expect("Invalid root length"))
                }
            }
        })
    }
}

pub trait FaceCenters<D: DimensionVector<f64>> {
    fn face_centers(&self) -> Vec<D::DimensionVector>;
}

impl FaceCenters<D2> for SubdivisionTree<D2> {
    fn face_centers(&self) -> Vec<<D2 as DimensionVector<f64>>::DimensionVector> {
        match self {
            Tree::Root([a, b, c, d]) => match [&**a, &**b, &**c, &**d] {
                [Tree::Leaf(_), Tree::Leaf(SubdivisionCell {
                    bounds:
                        Bounds {
                            min: bottom,
                            max: right,
                        },
                    ..
                }), Tree::Leaf(SubdivisionCell {
                    bounds:
                        Bounds {
                            min: left,
                            max: top,
                        },
                    ..
                }), Tree::Leaf(_)] => {
                    vec![*bottom, *right, *left, *top]
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

impl FaceCenters<D3> for SubdivisionTree<D3> {
    fn face_centers(&self) -> Vec<<D3 as DimensionVector<f64>>::DimensionVector> {
        match self {
            Tree::Root([a, b, c, d, e, f, g, h]) => {
                match [&**a, &**b, &**c, &**d, &**e, &**f, &**g, &**h] {
                    [Tree::Leaf(_), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { max: right, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { max: top, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min: front, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { max: back, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min: bottom, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min: left, .. },
                        ..
                    }), Tree::Leaf(_)] => {
                        vec![*right, *top, *front, *back, *bottom, *left]
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}

pub trait InterpolateCell<D: DimensionVector<f64>> {
    fn interpolate<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
        p: D::DimensionVector,
    ) -> Result<f64, EvaluateError>;
}

impl InterpolateCell<D2> for Bounds<D2> {
    fn interpolate<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
        p: <D2 as DimensionVector<f64>>::DimensionVector,
    ) -> Result<f64, EvaluateError> {
        let delta = (p - self.min).component_div(&self.size());

        let ab = f64::from(Sample::<D2>::sample(evaluator, self.min.into())?.get(&DISTANCE.into()))
            * (1.0 - delta.x)
            + f64::from(
                Sample::<D2>::sample(evaluator, [self.max.x, self.min.y].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let cd: f64 = f64::from(
            Sample::<D2>::sample(evaluator, [self.min.x, self.max.y].into())?.get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(Sample::<D2>::sample(evaluator, self.max.into())?.get(&DISTANCE.into()))
                * delta.x;

        Ok(ab * (1.0 - delta.y) + cd * delta.y)
    }
}

impl InterpolateCell<D3> for Bounds<D3> {
    fn interpolate<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
        p: <D3 as DimensionVector<f64>>::DimensionVector,
    ) -> Result<f64, EvaluateError> {
        let delta = (p - self.min).component_div(&self.size());

        let ab = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.min.y, self.min.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.min.y, self.min.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let cd: f64 = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.max.y, self.min.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.max.y, self.min.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let abcd = ab * (1.0 - delta.y) + cd * delta.y;

        let ef = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.min.y, self.max.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.min.y, self.max.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let gh: f64 = f64::from(
            Sample::<D3>::sample(evaluator, [self.min.x, self.max.y, self.max.z].into())?
                .get(&DISTANCE.into()),
        ) * (1.0 - delta.x)
            + f64::from(
                Sample::<D3>::sample(evaluator, [self.max.x, self.max.y, self.max.z].into())?
                    .get(&DISTANCE.into()),
            ) * delta.x;

        let efgh = ef * (1.0 - delta.y) + gh * delta.y;

        Ok(abcd * (1.0 - delta.z) + efgh * delta.z)
    }
}

pub trait Pairs {
    type IntoIter<'a>: IntoIterator
    where
        Self: 'a;

    fn pairs<'a>(&'a self) -> Self::IntoIter<'a>;
}

impl Pairs for SubdivisionTree<D2> {
    type IntoIter<'a> = Vec<(Face<2>, [&'a SubdivisionCell<D2>; 2])> where Self: 'a;

    fn pairs<'a>(&'a self) -> Self::IntoIter<'a> {
        fn tree_pairs_x<'a>(
            lhs: &'a SubdivisionTree<D2>,
            rhs: &'a SubdivisionTree<D2>,
        ) -> impl IntoIterator<Item = (Face<2>, [&'a SubdivisionCell<D2>; 2])> {
            match (lhs, rhs) {
                (Tree::Root([_, b, _, d]), Tree::Root([a, _, c, _])) => tree_pairs_x(b, a)
                    .into_iter()
                    .chain(tree_pairs_x(d, c))
                    .collect(),
                (Tree::Root([_, b, _, d]), Tree::Leaf(_)) => tree_pairs_x(b, rhs)
                    .into_iter()
                    .chain(tree_pairs_x(d, rhs))
                    .collect(),
                (Tree::Leaf(_), Tree::Root([a, _, c, _])) => tree_pairs_x(lhs, a)
                    .into_iter()
                    .chain(tree_pairs_x(lhs, c))
                    .collect(),
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![(Face::<2>::R, [lhs, rhs])],
            }
        }

        fn tree_pairs_y<'a>(
            lhs: &'a SubdivisionTree<D2>,
            rhs: &'a SubdivisionTree<D2>,
        ) -> impl IntoIterator<Item = (Face<2>, [&'a SubdivisionCell<D2>; 2])> {
            match (lhs, rhs) {
                (Tree::Root([_, _, c, d]), Tree::Root([a, b, _, _])) => tree_pairs_y(c, a)
                    .into_iter()
                    .chain(tree_pairs_y(d, b))
                    .collect(),
                (Tree::Root([_, _, c, d]), Tree::Leaf(_)) => tree_pairs_y(c, rhs)
                    .into_iter()
                    .chain(tree_pairs_y(d, rhs))
                    .collect(),
                (Tree::Leaf(_), Tree::Root([a, b, _, _])) => tree_pairs_y(lhs, a)
                    .into_iter()
                    .chain(tree_pairs_y(lhs, b))
                    .collect(),
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![(Face::<2>::U, [lhs, rhs])],
            }
        }

        match self {
            Tree::Root([a, b, c, d]) => a
                .pairs()
                .into_iter()
                .chain(b.pairs())
                .chain(c.pairs())
                .chain(d.pairs())
                .chain(tree_pairs_x(a, b))
                .chain(tree_pairs_x(c, d))
                .chain(tree_pairs_y(a, c))
                .chain(tree_pairs_y(b, d))
                .collect(),
            Tree::Leaf(_) => vec![],
        }
    }
}

impl Pairs for SubdivisionTree<D3> {
    type IntoIter<'a> = Vec<(Face<3>, [&'a SubdivisionCell<D3>; 2])> where Self: 'a;

    fn pairs<'a>(&'a self) -> Self::IntoIter<'a> {
        fn tree_pairs_x<'a>(
            lhs: &'a SubdivisionTree<D3>,
            rhs: &'a SubdivisionTree<D3>,
        ) -> impl IntoIterator<Item = (Face<3>, [&'a SubdivisionCell<D3>; 2])> {
            match (lhs, rhs) {
                (Tree::Root([_, b, _, d, _, f, _, h]), Tree::Root([a, _, c, _, e, _, g, _])) => {
                    tree_pairs_x(b, a)
                        .into_iter()
                        .chain(tree_pairs_x(d, c))
                        .chain(tree_pairs_x(f, e))
                        .chain(tree_pairs_x(h, g))
                        .collect()
                }
                (Tree::Root([_, b, _, d, _, f, _, h]), Tree::Leaf(_)) => tree_pairs_x(b, rhs)
                    .into_iter()
                    .chain(tree_pairs_x(d, rhs))
                    .chain(tree_pairs_x(f, rhs))
                    .chain(tree_pairs_x(h, rhs))
                    .collect(),
                (Tree::Leaf(_), Tree::Root([a, _, c, _, e, _, g, _])) => tree_pairs_x(lhs, a)
                    .into_iter()
                    .chain(tree_pairs_x(lhs, c))
                    .chain(tree_pairs_x(lhs, e))
                    .chain(tree_pairs_x(lhs, g))
                    .collect(),
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => {
                    vec![(Face::<3>::R, [lhs, rhs])]
                }
            }
        }

        fn tree_pairs_y<'a>(
            lhs: &'a SubdivisionTree<D3>,
            rhs: &'a SubdivisionTree<D3>,
        ) -> impl IntoIterator<Item = (Face<3>, [&'a SubdivisionCell<D3>; 2])> {
            match (lhs, rhs) {
                (Tree::Root([_, _, c, d, _, _, g, h]), Tree::Root([a, b, _, _, e, f, _, _])) => {
                    tree_pairs_y(c, a)
                        .into_iter()
                        .chain(tree_pairs_y(d, b))
                        .chain(tree_pairs_y(g, e))
                        .chain(tree_pairs_y(h, f))
                        .collect()
                }
                (Tree::Root([_, _, c, d, _, _, g, h]), Tree::Leaf(_)) => tree_pairs_y(c, rhs)
                    .into_iter()
                    .chain(tree_pairs_y(d, rhs))
                    .chain(tree_pairs_y(g, rhs))
                    .chain(tree_pairs_y(h, rhs))
                    .collect(),
                (Tree::Leaf(_), Tree::Root([a, b, _, _, e, f, _, _])) => tree_pairs_y(lhs, a)
                    .into_iter()
                    .chain(tree_pairs_y(lhs, b))
                    .chain(tree_pairs_y(lhs, e))
                    .chain(tree_pairs_y(lhs, f))
                    .collect(),
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => {
                    vec![(Face::<3>::U, [lhs, rhs])]
                }
            }
        }

        fn tree_pairs_z<'a>(
            lhs: &'a SubdivisionTree<D3>,
            rhs: &'a SubdivisionTree<D3>,
        ) -> impl IntoIterator<Item = (Face<3>, [&'a SubdivisionCell<D3>; 2])> {
            match (lhs, rhs) {
                (Tree::Root([_, _, _, _, e, f, g, h]), Tree::Root([a, b, c, d, _, _, _, _])) => {
                    tree_pairs_z(e, a)
                        .into_iter()
                        .chain(tree_pairs_z(f, b))
                        .chain(tree_pairs_z(g, c))
                        .chain(tree_pairs_z(h, d))
                        .collect()
                }
                (Tree::Root([_, _, _, _, e, f, g, h]), Tree::Leaf(_)) => tree_pairs_z(e, rhs)
                    .into_iter()
                    .chain(tree_pairs_z(f, rhs))
                    .chain(tree_pairs_z(g, rhs))
                    .chain(tree_pairs_z(h, rhs))
                    .collect(),
                (Tree::Leaf(_), Tree::Root([a, b, c, d, _, _, _, _])) => tree_pairs_z(lhs, a)
                    .into_iter()
                    .chain(tree_pairs_z(lhs, b))
                    .chain(tree_pairs_z(lhs, c))
                    .chain(tree_pairs_z(lhs, d))
                    .collect(),
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![(Face::F, [lhs, rhs])],
            }
        }

        match self {
            Tree::Root([a, b, c, d, e, f, g, h]) => a
                .pairs()
                .into_iter()
                .chain(b.pairs())
                .chain(c.pairs())
                .chain(d.pairs())
                .chain(e.pairs())
                .chain(f.pairs())
                .chain(g.pairs())
                .chain(h.pairs())
                .chain(tree_pairs_x(a, b))
                .chain(tree_pairs_x(c, d))
                .chain(tree_pairs_x(e, f))
                .chain(tree_pairs_x(g, h))
                .chain(tree_pairs_y(a, c))
                .chain(tree_pairs_y(b, d))
                .chain(tree_pairs_y(e, g))
                .chain(tree_pairs_y(f, h))
                .chain(tree_pairs_z(a, e))
                .chain(tree_pairs_z(b, f))
                .chain(tree_pairs_z(c, g))
                .chain(tree_pairs_z(d, h))
                .collect(),
            Tree::Leaf(_) => vec![],
        }
    }
}

pub trait Neighbours<const D: usize> {
    fn neighbours(&self, rhs: &Self, side: &Face<D>) -> bool;
}

impl<const N: usize> Neighbours<N> for Corners<N>
where
    Face<N>: ToCorners<N>,
    Corners<N>: IntoIterator<Item = Corner<N>>,
{
    fn neighbours(&self, rhs: &Self, side: &Face<N>) -> bool {
        let corners = side.to_corners();
        corners
            .into_iter()
            .zip((!corners).into_iter())
            .fold(true, |acc, (from, to)| {
                acc & ((*self & from).is_empty() == (*rhs & to).is_empty())
            })
    }
}

pub trait HasSignChange<const D: usize> {
    fn has_sign_change(&self, side: &Face<D>) -> bool;
}

impl<const N: usize> HasSignChange<N> for Corners<N>
where
    Face<N>: ToCorners<N>,
    Corners<N>: IntoIterator<Item = Corner<N>>,
{
    fn has_sign_change(&self, side: &Face<N>) -> bool {
        side.to_corners()
            .into_iter()
            .fold(false, |acc, next| acc | (*self & next).is_empty())
    }
}

#[cfg(test)]
mod test {
    use crate::{bounds::Bounds, tree::Tree, vector_space::D2};

    use super::{SubdivisionCell, SubdivisionTree};

    fn tree_pairs<'a>(tree: &'a SubdivisionTree<D2>) -> Vec<[&'a SubdivisionCell<D2>; 2]> {
        fn tree_pairs_x<'a>(
            lhs: &'a SubdivisionTree<D2>,
            rhs: &'a SubdivisionTree<D2>,
        ) -> Vec<[&'a SubdivisionCell<D2>; 2]> {
            match (lhs, rhs) {
                (Tree::Root([a, _, c, _]), Tree::Root([_, b, _, d])) => tree_pairs_x(a, b)
                    .into_iter()
                    .chain(tree_pairs_x(c, d))
                    .collect(),
                (Tree::Root([_, b, _, d]), Tree::Leaf(_)) => tree_pairs_x(b, rhs)
                    .into_iter()
                    .chain(tree_pairs_x(d, rhs))
                    .collect(),
                (Tree::Leaf(_), Tree::Root([a, _, c, _])) => tree_pairs_x(lhs, a)
                    .into_iter()
                    .chain(tree_pairs_x(lhs, c))
                    .collect(),
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![[lhs, rhs]],
            }
        }

        fn tree_pairs_y<'a>(
            lhs: &'a SubdivisionTree<D2>,
            rhs: &'a SubdivisionTree<D2>,
        ) -> Vec<[&'a SubdivisionCell<D2>; 2]> {
            match (lhs, rhs) {
                (Tree::Root([a, b, _, _]), Tree::Root([_, _, c, d])) => tree_pairs_y(a, c)
                    .into_iter()
                    .chain(tree_pairs_y(b, d))
                    .collect(),
                (Tree::Root([a, b, _, _]), Tree::Leaf(_)) => tree_pairs_x(a, rhs)
                    .into_iter()
                    .chain(tree_pairs_x(b, rhs))
                    .collect(),
                (Tree::Leaf(_), Tree::Root([_, _, c, d])) => tree_pairs_x(lhs, c)
                    .into_iter()
                    .chain(tree_pairs_x(lhs, d))
                    .collect(),
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![[lhs, rhs]],
            }
        }

        match tree {
            Tree::Root([a, b, c, d]) => tree_pairs(a)
                .into_iter()
                .chain(tree_pairs(b))
                .chain(tree_pairs(c))
                .chain(tree_pairs(d))
                .chain(tree_pairs_x(a, b))
                .chain(tree_pairs_x(c, d))
                .chain(tree_pairs_y(a, c))
                .chain(tree_pairs_y(b, d))
                .collect(),
            Tree::Leaf(_) => vec![],
        }
    }

    #[test]
    fn test_subdivision_tree() {
        let tree = SubdivisionTree::<D2>::new(
            Bounds {
                min: [-1.0, -1.0].into(),
                max: [1.0, 1.0].into(),
            },
            1,
        );

        println!("Tree:\n{tree:#?}");

        let pairs = tree_pairs(&tree);

        println!("Pairs:\n{pairs:#?}");

        panic!();
    }
}
