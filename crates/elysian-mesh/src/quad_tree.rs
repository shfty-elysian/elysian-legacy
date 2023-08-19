use elysian_ir::{
    ast::DISTANCE,
    module::{Evaluate, EvaluateError},
};

use crate::{
    tree::{Tree, Tree4},
    util::Vec2,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Bounds {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct QuadCell {
    pub bounds: Bounds,
    pub ty: QuadCellType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuadCellType {
    Empty,
    Full,
    Contour,
}

pub type QuadTree = Tree4<QuadCell>;

impl QuadTree {
    /// Build a full-density QuadTree with the provided bounds and sampling level
    pub fn new(bounds: Bounds, level: usize) -> Self {
        let size = [
            bounds.max.x() - bounds.min.x(),
            bounds.max.y() - bounds.min.y(),
        ];
        let hsize = [size.x() * 0.5, size.y() * 0.5];

        if level > 0 {
            let mut leaves = vec![];

            for iy in 0..2 {
                for ix in 0..2 {
                    let lmin = [
                        bounds.min.x() + hsize.x() * ix as f64,
                        bounds.min.y() + hsize.y() * iy as f64,
                    ];
                    let lmax = [lmin.x() + hsize.x(), lmin.y() + hsize.y()];
                    leaves.push(Box::new(Self::new(
                        Bounds {
                            min: lmin,
                            max: lmax,
                        },
                        level - 1,
                    )));
                }
            }

            Self::Root(leaves.try_into().unwrap())
        } else {
            Self::Leaf(QuadCell {
                bounds,
                ty: QuadCellType::Contour,
            })
        }
    }

    /// Calculate the bounds of the tree via recursion
    pub fn bounds(&self) -> Bounds {
        fn bounds_impl(tree: &QuadTree, bounds: Bounds) -> Bounds {
            match tree {
                Tree::Root(t) => {
                    t.into_iter()
                        .map(|t| bounds_impl(t, bounds))
                        .fold(bounds, |acc, next| Bounds {
                            min: [acc.min.x().min(next.min.x()), acc.min.y().min(next.min.y())],
                            max: [acc.max.x().max(next.max.x()), acc.max.y().max(next.max.y())],
                        })
                }
                Tree::Leaf(QuadCell { bounds, .. }) => *bounds,
            }
        }

        bounds_impl(
            self,
            Bounds {
                min: [f64::MAX; 2],
                max: [f64::MIN; 2],
            },
        )
    }

    /// Given a sampling function and en epsilon,
    /// merge cells whose local error versus linear interpolation falls below the given threshold
    pub fn merge<'a>(
        self,
        evaluator: &impl Evaluate<'a>,
        epsilon: f64,
    ) -> Result<QuadTree, EvaluateError> {
        fn interpolate<'a>(
            evaluator: &impl Evaluate<'a>,
            bounds: Bounds,
            p: [f64; 2],
        ) -> Result<f64, EvaluateError> {
            let dx = (p.x() - bounds.min.x()) / (bounds.max.x() - bounds.min.x());
            let dy = (p.y() - bounds.min.y()) / (bounds.max.y() - bounds.min.y());

            let ab = f64::from(evaluator.sample_2d(bounds.min)?.get(&DISTANCE.into())) * (1.0 - dx)
                + f64::from(
                    evaluator
                        .sample_2d([bounds.max.x(), bounds.min.y()])?
                        .get(&DISTANCE.into()),
                ) * dx;

            let cd: f64 = f64::from(
                evaluator
                    .sample_2d([bounds.min.x(), bounds.max.y()])?
                    .get(&DISTANCE.into()),
            ) * (1.0 - dx)
                + f64::from(evaluator.sample_2d(bounds.max)?.get(&DISTANCE.into())) * dx;

            Ok(ab * (1.0 - dy) + cd * dy)
        }

        fn score<'a>(
            evaluator: &impl Evaluate<'a>,
            bounds: Bounds,
            p: [f64; 2],
        ) -> Result<f64, EvaluateError> {
            Ok((interpolate(evaluator, bounds, p)?
                - f64::from(evaluator.sample_2d(p)?.get(&DISTANCE.into())))
            .abs())
        }

        Ok(match self {
            Tree::Root([a, b, c, d]) => {
                let [a, b, c, d] = [
                    a.merge(evaluator, epsilon)?,
                    b.merge(evaluator, epsilon)?,
                    c.merge(evaluator, epsilon)?,
                    d.merge(evaluator, epsilon)?,
                ];

                match (&a, &b, &c, &d) {
                    (
                        QuadTree::Leaf(QuadCell {
                            bounds: Bounds { min, max: i },
                            ty: QuadCellType::Contour,
                        }),
                        QuadTree::Leaf(QuadCell {
                            bounds: Bounds { min: q, max: r },
                            ty: QuadCellType::Contour,
                        }),
                        QuadTree::Leaf(QuadCell {
                            bounds: Bounds { min: s, max: t },
                            ty: QuadCellType::Contour,
                        }),
                        QuadTree::Leaf(QuadCell {
                            bounds: Bounds { max, .. },
                            ty: QuadCellType::Contour,
                        }),
                    ) => {
                        let foo = [i, q, r, s, t]
                            .into_iter()
                            .map(|t| {
                                score(
                                    evaluator,
                                    Bounds {
                                        min: *min,
                                        max: *max,
                                    },
                                    *t,
                                )
                            })
                            .collect::<Result<Vec<_>, _>>()?;

                        if foo.iter().all(|t| *t < epsilon) {
                            QuadTree::Leaf(QuadCell {
                                bounds: Bounds {
                                    min: *min,
                                    max: *max,
                                },
                                ty: QuadCellType::Contour,
                            })
                        } else {
                            QuadTree::Root([
                                Box::new(a.clone()),
                                Box::new(b.clone()),
                                Box::new(c.clone()),
                                Box::new(d.clone()),
                            ])
                        }
                    }
                    _ => QuadTree::Root([
                        Box::new(a.clone()),
                        Box::new(b.clone()),
                        Box::new(c.clone()),
                        Box::new(d.clone()),
                    ]),
                }
            }
            Tree::Leaf(_) => self,
        })
    }

    /// Given a sampling function, collapse Leaf cells into Full and Empty variants
    pub fn collapse<'a>(self, evaluator: &impl Evaluate<'a>) -> Result<Self, EvaluateError> {
        let Bounds { min, max } = self.bounds();

        Ok(match self {
            Self::Leaf(QuadCell {
                bounds: Bounds { min, max },
                ..
            }) => {
                let samples = [min.y(), max.y()]
                    .into_iter()
                    .flat_map(|y| [min.x(), max.x()].into_iter().map(move |x| [x, y]))
                    .map(|p| evaluator.sample_2d(p))
                    .collect::<Result<Vec<_>, _>>()?;

                Self::Leaf(QuadCell {
                    bounds: Bounds { min, max },
                    ty: if samples
                        .iter()
                        .all(|t| f64::from(t.get(&DISTANCE.into())) <= 0.0)
                    {
                        QuadCellType::Full
                    } else if samples
                        .iter()
                        .all(|t| f64::from(t.get(&DISTANCE.into())) > 0.0)
                    {
                        QuadCellType::Empty
                    } else {
                        QuadCellType::Contour
                    },
                })
            }
            Self::Root(leaves) => {
                let leaves = leaves
                    .into_iter()
                    .map(|t| t.collapse(evaluator).map(Box::new))
                    .collect::<Result<Vec<_>, _>>()?;

                if leaves.iter().all(|leaf| match **leaf {
                    Tree::Leaf(QuadCell {
                        ty: QuadCellType::Empty,
                        ..
                    }) => true,
                    _ => false,
                }) {
                    Self::Leaf(QuadCell {
                        bounds: Bounds { min, max },
                        ty: QuadCellType::Empty,
                    })
                } else if leaves.iter().all(|leaf| match **leaf {
                    Tree::Leaf(QuadCell {
                        ty: QuadCellType::Full,
                        ..
                    }) => true,
                    _ => false,
                }) {
                    Self::Leaf(QuadCell {
                        bounds: Bounds { min, max },
                        ty: QuadCellType::Full,
                    })
                } else {
                    Self::Root(leaves.try_into().unwrap())
                }
            }
        })
    }
}

