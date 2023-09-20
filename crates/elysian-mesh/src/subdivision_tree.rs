use std::ops::Mul;

use crate::{
    bounds::Bounds,
    dual_graph::{AsDualGraph, DualGraph, DualPair},
    face_centers::FaceCenters,
    interpolate_cell::InterpolateCell,
    marching_cells::Face,
    sample::Sample,
    subdivision_cell::{CellType, SubdivisionCell},
    tree::Tree,
    vector_space::{VectorSpace, D2, D3},
};
use elysian_ir::{ast::DISTANCE, module::EvaluateError};

pub type SubdivisionTree<D> = Tree<SubdivisionCell<D>, D>;

pub type QuadTree = SubdivisionTree<D2>;
pub type Octree = SubdivisionTree<D3>;

impl<D> SubdivisionTree<D>
where
    D: VectorSpace<f64>,
    D::DimensionVector: Mul<f64, Output = D::DimensionVector>,
{
    /// Build a tree of empty cells with the provided bounds and sampling level
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
                ty: CellType::Empty,
            })
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
    /// merge same-typed cells whose local error versus linear interpolation falls below the given threshold
    pub fn merge<'a>(
        self,
        evaluator: &impl Sample<'a, D>,
        epsilon: f64,
    ) -> Result<Self, EvaluateError>
    where
        Self: FaceCenters<D>,
        Bounds<D>: InterpolateCell<D>,
    {
        fn score<'a, D: VectorSpace<f64>>(
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

                let root: D::SubdivisionArray<Box<SubdivisionTree<D>>> = root
                    .into_iter()
                    .map(Box::new)
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()
                    .unwrap();

                // Early out if the children are not all leaves
                if !D::iter(&root).all(|t| matches!(**t, Tree::Leaf(_))) {
                    return Ok(Tree::Root(root));
                }

                // Early out if the leaves' types don't match
                let root_ty = {
                    let mut it = D::iter(&root);
                    let Tree::Leaf(first) = &**it.next().unwrap() else { unreachable!() };
                    let root_ty = first.ty;

                    if !it.fold(true, |acc, next| {
                        let Tree::Leaf(next) = &**next else {
                        unreachable!()
                    };

                        acc && next.ty == root_ty
                    }) {
                        return Ok(Tree::Root(root));
                    }

                    root_ty
                };

                // Score face centers
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

                // If scores are all within the epsilon, replace with a single cell
                if scores.iter().all(|score| *score < epsilon) {
                    Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min, max },
                        ty: root_ty,
                    })
                } else {
                    tree
                }
            }
            _ => self,
        })
    }
}

impl<D> SubdivisionTree<D>
where
    D: VectorSpace<f64>,
{
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
        D: VectorSpace<f64>,
    {
        Bounds {
            min: self.min_corner(),
            max: self.max_corner(),
        }
    }

    /// Assign cell types using a sampling function
    pub fn sample<'a, E>(self, evaluator: &E) -> Result<Self, EvaluateError>
    where
        E: Sample<'a, D>,
        Bounds<D>: IntoIterator<Item = D::DimensionVector>,
    {
        let Bounds { min, max } = self.bounds();

        Ok(match self {
            Self::Leaf(cell) => {
                let samples = cell
                    .bounds
                    .clone()
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
                    .map(|t| t.sample(evaluator).map(Box::new))
                    .collect::<Result<Vec<_>, _>>()?;

                Self::Root(
                    leaves
                        .try_into()
                        .ok()
                        .expect("Failed to convert leaf array"),
                )
            }
        })
    }

    /// Given a sampling function, collapse Leaf cells into Full and Empty variants
    pub fn collapse<'a, E>(self, evaluator: &E) -> Result<Self, EvaluateError>
    where
        E: Sample<'a, D>,
        Bounds<D>: IntoIterator<Item = D::DimensionVector>,
    {
        let Bounds { min, max } = self.bounds();

        Ok(match self {
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
            _ => self,
        })
    }
}

impl<D> SubdivisionTree<D>
where
    D: VectorSpace<f64>,
{
    pub fn cells_empty(&self) -> impl Iterator<Item = &SubdivisionCell<D>> {
        self.iter().filter(|cell| cell.is_empty())
    }

    pub fn cells_contour(&self) -> impl Iterator<Item = &SubdivisionCell<D>> {
        self.iter().filter(|cell| cell.is_contour())
    }

    pub fn cells_full(&self) -> impl Iterator<Item = &SubdivisionCell<D>> {
        self.iter().filter(|cell| cell.is_full())
    }
}

impl AsDualGraph<D2> for SubdivisionTree<D2> {
    fn as_dual_graph<'a>(&'a self) -> DualGraph<'a, D2> {
        fn tree_pairs_x<'a>(
            lhs: &'a SubdivisionTree<D2>,
            rhs: &'a SubdivisionTree<D2>,
        ) -> impl IntoIterator<Item = DualPair<'a, D2>> {
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
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![DualPair {
                    face: Face::<D2>::R,
                    lhs,
                    rhs,
                }],
            }
        }

        fn tree_pairs_y<'a>(
            lhs: &'a SubdivisionTree<D2>,
            rhs: &'a SubdivisionTree<D2>,
        ) -> impl IntoIterator<Item = DualPair<'a, D2>> {
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
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![DualPair {
                    face: Face::<D2>::U,
                    lhs,
                    rhs,
                }],
            }
        }

        match self {
            Tree::Root([a, b, c, d]) => a
                .as_dual_graph()
                .into_iter()
                .chain(b.as_dual_graph())
                .chain(c.as_dual_graph())
                .chain(d.as_dual_graph())
                .chain(tree_pairs_x(a, b))
                .chain(tree_pairs_x(c, d))
                .chain(tree_pairs_y(a, c))
                .chain(tree_pairs_y(b, d))
                .collect(),
            Tree::Leaf(_) => DualGraph::default(),
        }
    }
}

impl AsDualGraph<D3> for SubdivisionTree<D3> {
    fn as_dual_graph<'a>(&'a self) -> DualGraph<'a, D3> {
        fn tree_pairs_x<'a>(
            lhs: &'a SubdivisionTree<D3>,
            rhs: &'a SubdivisionTree<D3>,
        ) -> impl IntoIterator<Item = DualPair<'a, D3>> {
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
                    vec![DualPair {
                        face: Face::<D3>::R,
                        lhs,
                        rhs,
                    }]
                }
            }
        }

        fn tree_pairs_y<'a>(
            lhs: &'a SubdivisionTree<D3>,
            rhs: &'a SubdivisionTree<D3>,
        ) -> impl IntoIterator<Item = DualPair<'a, D3>> {
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
                    vec![DualPair {
                        face: Face::<D3>::U,
                        lhs,
                        rhs,
                    }]
                }
            }
        }

        fn tree_pairs_z<'a>(
            lhs: &'a SubdivisionTree<D3>,
            rhs: &'a SubdivisionTree<D3>,
        ) -> impl IntoIterator<Item = DualPair<'a, D3>> {
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
                (Tree::Leaf(lhs), Tree::Leaf(rhs)) => vec![DualPair {
                    face: Face::F,
                    lhs,
                    rhs,
                }],
            }
        }

        match self {
            Tree::Root([a, b, c, d, e, f, g, h]) => a
                .as_dual_graph()
                .into_iter()
                .chain(b.as_dual_graph())
                .chain(c.as_dual_graph())
                .chain(d.as_dual_graph())
                .chain(e.as_dual_graph())
                .chain(f.as_dual_graph())
                .chain(g.as_dual_graph())
                .chain(h.as_dual_graph())
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
            Tree::Leaf(_) => DualGraph::default(),
        }
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
