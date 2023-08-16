use image::{ImageBuffer, Rgb};

use crate::{
    tree::{Tree, Tree4},
    util::Vec2,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct QuadCell {
    pub min: [f64; 2],
    pub max: [f64; 2],
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
    pub fn new(min: [f64; 2], max: [f64; 2], level: usize) -> Self {
        let size = [max.x() - min.x(), max.y() - min.y()];
        let hsize = [size.x() * 0.5, size.y() * 0.5];

        if level > 0 {
            let mut trees = vec![];

            for iy in 0..2 {
                for ix in 0..2 {
                    let lmin = [
                        min.x() + hsize.x() * ix as f64,
                        min.y() + hsize.y() * iy as f64,
                    ];
                    let lmax = [lmin.x() + hsize.x(), lmin.y() + hsize.y()];
                    trees.push(Box::new(Self::new(lmin, lmax, level - 1)));
                }
            }

            Self::Root(trees.try_into().unwrap())
        } else {
            Self::Leaf(QuadCell {
                min,
                max,
                ty: QuadCellType::Contour,
            })
        }
    }

    pub fn bounds(&self) -> ([f64; 2], [f64; 2]) {
        self.bounds_impl([f64::MAX; 2], [f64::MIN; 2])
    }

    pub fn bounds_impl(&self, min: [f64; 2], max: [f64; 2]) -> ([f64; 2], [f64; 2]) {
        match self {
            Tree::Root(t) => t.into_iter().map(|t| t.bounds_impl(min, max)).fold(
                (min, max),
                |(min, max), (cmin, cmax)| {
                    (
                        [min.x().min(cmin.x()), min.y().min(cmin.y())],
                        [max.x().max(cmax.x()), max.y().max(cmax.y())],
                    )
                },
            ),
            Tree::Leaf(QuadCell { min, max, .. }) => (*min, *max),
        }
    }

    pub fn collapse(self, sample: impl Fn([f64; 2]) -> f64 + Copy) -> Self {
        let (min, max) = self.bounds();

        match self {
            Self::Leaf(QuadCell { min, max, .. }) => {
                let iter = [min.y(), max.y()]
                    .into_iter()
                    .flat_map(|y| [min.x(), max.x()].into_iter().map(move |x| [x, y]))
                    .map(|p| sample(p));

                Self::Leaf(QuadCell {
                    min,
                    max,
                    ty: if iter.clone().all(|t| t <= 0.0) {
                        QuadCellType::Full
                    } else if iter.clone().all(|t| t > 0.0) {
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
                        min,
                        max,
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
                        min,
                        max,
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

    tree.iter()
        .zip(tree.map_depth())
        .for_each(|(QuadCell { min, max, ty }, depth)| {
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
        });

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
        let quad_tree = QuadTree::new([-1.0, -1.0], [1.0, 1.0], 6).collapse(sample);

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
