use image::{ImageBuffer, Rgb};

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
    pub fn merge(self, sample: impl Fn([f64; 2]) -> f64 + Clone, epsilon: f64) -> QuadTree {
        fn interpolate(
            sample: impl Fn([f64; 2]) -> f64 + Clone,
            bounds: Bounds,
            p: [f64; 2],
        ) -> f64 {
            let dx = (p.x() - bounds.min.x()) / (bounds.max.x() - bounds.min.x());
            let dy = (p.y() - bounds.min.y()) / (bounds.max.y() - bounds.min.y());
            let ab =
                sample(bounds.min) * (1.0 - dx) + sample([bounds.max.x(), bounds.min.y()]) * dx;
            let cd =
                sample([bounds.min.x(), bounds.max.y()]) * (1.0 - dx) + sample(bounds.max) * dx;
            ab * (1.0 - dy) + cd * dy
        }

        fn score(sample: impl Fn([f64; 2]) -> f64 + Clone, bounds: Bounds, p: [f64; 2]) -> f64 {
            (interpolate(sample.clone(), bounds, p) - sample(p)).abs()
        }

        match self {
            Tree::Root(t) => {
                let t = t.map(|t| t.merge(sample.clone(), epsilon));
                let (a, b, c, d) = (&t[0], &t[1], &t[2], &t[3]);

                match (a, b, c, d) {
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
                        if [i, q, r, s, t]
                            .into_iter()
                            .map(|t| {
                                score(
                                    sample.clone(),
                                    Bounds {
                                        min: *min,
                                        max: *max,
                                    },
                                    *t,
                                )
                            })
                            .all(|t| t < epsilon)
                        {
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
        }
    }

    /// Given a sampling function, collapse Leaf cells into Full and Empty variants
    pub fn collapse(self, sample: impl Fn([f64; 2]) -> f64 + Copy) -> Self {
        let Bounds { min, max } = self.bounds();

        match self {
            Self::Leaf(QuadCell {
                bounds: Bounds { min, max },
                ..
            }) => {
                let mut iter = [min.y(), max.y()]
                    .into_iter()
                    .flat_map(|y| [min.x(), max.x()].into_iter().map(move |x| [x, y]))
                    .map(|p| sample(p));

                Self::Leaf(QuadCell {
                    bounds: Bounds { min, max },
                    ty: if iter.clone().all(|t| t <= 0.0) {
                        QuadCellType::Full
                    } else if iter.all(|t| t > 0.0) {
                        QuadCellType::Empty
                    } else {
                        QuadCellType::Contour
                    },
                })
            }
            Self::Root(leaves) => {
                let leaves = leaves
                    .into_iter()
                    .map(|t| Box::new(t.collapse(sample)))
                    .collect::<Vec<_>>();

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
        }
    }
}

pub fn draw_quad_tree(tree: QuadTree) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let max_depth = tree.depth();
    let size = (tree.resolution() as f64).sqrt() as u32;
    let mut image = ImageBuffer::new(size, size);

    tree.iter().zip(tree.map_depth()).for_each(
        |(
            QuadCell {
                bounds: Bounds { min, max },
                ty,
            },
            depth,
        )| {
            let min_x = ((min.x() * 0.5 + 0.5) * size as f64).floor() as u32;
            let min_y = ((min.y() * 0.5 + 0.5) * size as f64).floor() as u32;

            let max_x = ((max.x() * 0.5 + 0.5) * size as f64).floor() as u32;
            let max_y = ((max.y() * 0.5 + 0.5) * size as f64).floor() as u32;

            let c = depth as f32 / max_depth as f32;
            let p = Rgb(match ty {
                QuadCellType::Empty => [c, 0.0, 0.0],
                QuadCellType::Full => [0.0, 0.0, c],
                QuadCellType::Contour => [0.0, c, 0.0],
            });

            for y in min_y..max_y {
                for x in min_x..max_x {
                    image.put_pixel(x, y, p);
                }
            }
        },
    );

    image
}

#[cfg(test)]
mod test {
    use viuer::Config;

    use crate::quad_tree::draw_quad_tree;

    use super::*;

    #[test]
    fn test_quad_tree() {
        let sample = |p: [f64; 2]| (p.x() * p.x() + p.y() * p.y()).sqrt() - 0.6;
        let quad_tree = QuadTree::new(
            Bounds {
                min: [-1.0, -1.0],
                max: [1.0, 1.0],
            },
            6,
        )
        .merge(sample, 0.001)
        .collapse(sample);

        let image = draw_quad_tree(quad_tree);

        viuer::print(
            &image.into(),
            &Config {
                transparent: false,
                absolute_offset: false,
                x: 0,
                y: 0,
                restore_cursor: false,
                width: None,
                height: None,
                truecolor: true,
                use_kitty: true,
                use_iterm: false,
            },
        )
        .unwrap();

        panic!();
    }
}
