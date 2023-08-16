use image::{ImageBuffer, Rgb};

use crate::{
    quad_tree::{QuadCell, QuadCellType, QuadTree},
    tree::Tree,
    util::Vec2,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Upper,
    Lower,
    Left,
    Right,
}

fn lut(index: usize) -> Vec<(Side, Side)> {
    match index {
        0 => vec![],
        1 => vec![(Side::Upper, Side::Right)],
        2 => vec![(Side::Left, Side::Upper)],
        3 => vec![(Side::Left, Side::Right)],
        4 => vec![(Side::Right, Side::Lower)],
        5 => vec![(Side::Upper, Side::Lower)],
        6 => vec![(Side::Right, Side::Lower), (Side::Left, Side::Upper)],
        7 => vec![(Side::Left, Side::Lower)],
        8 => vec![(Side::Lower, Side::Left)],
        9 => vec![(Side::Lower, Side::Left), (Side::Upper, Side::Right)],
        10 => vec![(Side::Lower, Side::Upper)],
        11 => vec![(Side::Lower, Side::Right)],
        12 => vec![(Side::Right, Side::Left)],
        13 => vec![(Side::Upper, Side::Left)],
        14 => vec![(Side::Right, Side::Upper)],
        15 => vec![],
        i => unimplemented!("{i:}"),
    }
}

fn index(sample: impl Fn([f64; 2]) -> f64, min: [f64; 2], max: [f64; 2]) -> usize {
    let pts: Vec<_> = [min.y(), max.y()]
        .into_iter()
        .flat_map(|y| [min.x(), max.x()].into_iter().map(move |x| [x, y]))
        .collect();

    pts.into_iter()
        .enumerate()
        .map(|(i, pt)| {
            if sample(pt) < 0.0 {
                2usize.pow((3 - i) as u32)
            } else {
                0
            }
        })
        .sum()
}

fn edges(sample: impl Fn([f64; 2]) -> f64, min: [f64; 2], max: [f64; 2]) -> Vec<(Side, Side)> {
    lut(index(sample, min, max))
}

fn pt(sample: impl Fn([f64; 2]) -> f64, min: [f64; 2], max: [f64; 2], side: Side) -> [f64; 2] {
    match side {
        Side::Left => zero(sample, [min.x(), min.y()], [min.x(), max.y()]),
        Side::Right => zero(sample, [max.x(), min.y()], [max.x(), max.y()]),
        Side::Lower => zero(sample, [min.x(), min.y()], [max.x(), min.y()]),
        Side::Upper => zero(sample, [min.x(), max.y()], [max.x(), max.y()]),
    }
}

fn zero(sample: impl Fn([f64; 2]) -> f64, min: [f64; 2], max: [f64; 2]) -> [f64; 2] {
    fn pos(f: f64, min: [f64; 2], max: [f64; 2]) -> [f64; 2] {
        [
            min.x() * (1.0 - f) + max.x() * f,
            min.y() * (1.0 - f) + max.y() * f,
        ]
    }

    fn zero_(
        f: f64,
        step: f64,
        i: usize,
        sample: impl Fn([f64; 2]) -> f64,
        min: [f64; 2],
        max: [f64; 2],
    ) -> [f64; 2] {
        if i == 0 {
            pos(f, min, max)
        } else if sample(pos(f, min, max)) < 0.0 {
            zero_(f + step, step / 2.0, i - 1, sample, min, max)
        } else {
            zero_(f - step, step / 2.0, i - 1, sample, min, max)
        }
    }

    if sample(min) >= 0.0 {
        zero_(0.5, 0.25, 10, sample, max, min)
    } else {
        zero_(0.5, 0.25, 10, sample, min, max)
    }
}

pub fn contours(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    min: [f64; 2],
    max: [f64; 2],
) -> Vec<[[f64; 2]; 2]> {
    edges(sample.clone(), min, max)
        .into_iter()
        .map(|(a, b)| {
            [
                pt(sample.clone(), min, max, a),
                pt(sample.clone(), min, max, b),
            ]
        })
        .collect()
}

pub fn draw_marching_squares(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    tree: QuadTree,
) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let contours: Vec<_> = tree
        .into_iter()
        .filter(|t| t.ty == QuadCellType::Contour)
        .flat_map(|t| contours(sample.clone(), t.min, t.max))
        .collect();

    let mut image = ImageBuffer::new(64, 64);

    for contour in contours.iter() {
        let from_x = ((contour[0].x() * 0.5 + 0.5) * image.width() as f64).floor() as u32;
        let from_y = ((contour[0].y() * 0.5 + 0.5) * image.height() as f64).floor() as u32;

        let to_x = ((contour[1].x() * 0.5 + 0.5) * image.width() as f64).floor() as u32;
        let to_y = ((contour[1].y() * 0.5 + 0.5) * image.height() as f64).floor() as u32;

        image.put_pixel(from_x, from_y, Rgb([1.0, 0.0, 0.0]));
        image.put_pixel(to_x, to_y, Rgb([0.0, 0.0, 1.0]));
    }

    image
}

pub fn interpolate(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    min: [f64; 2],
    max: [f64; 2],
    p: [f64; 2],
) -> f64 {
    let dx = (p.x() - min.x()) / (max.x() - min.x());
    let dy = (p.y() - min.y()) / (max.y() - min.y());
    let ab = sample(min) * (1.0 - dx) + sample([max.x(), min.y()]) * dx;
    let cd = sample([min.x(), max.y()]) * (1.0 - dx) + sample(max) * dx;
    ab * (1.0 - dy) + cd * dy
}

pub fn score(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    min: [f64; 2],
    max: [f64; 2],
    p: [f64; 2],
) -> f64 {
    (interpolate(sample.clone(), min, max, p) - sample(p)).abs()
}

pub fn merge(sample: impl Fn([f64; 2]) -> f64 + Clone, tree: QuadTree) -> QuadTree {
    fn merge_(
        sample: impl Fn([f64; 2]) -> f64 + Clone,
        a: QuadTree,
        b: QuadTree,
        c: QuadTree,
        d: QuadTree,
    ) -> QuadTree {
        match (&a, &b, &c, &d) {
            (
                QuadTree::Leaf(QuadCell {
                    min,
                    max: i,
                    ty: QuadCellType::Contour,
                }),
                QuadTree::Leaf(QuadCell {
                    min: q,
                    max: r,
                    ty: QuadCellType::Contour,
                }),
                QuadTree::Leaf(QuadCell {
                    min: s,
                    max: t,
                    ty: QuadCellType::Contour,
                }),
                QuadTree::Leaf(QuadCell {
                    max,
                    ty: QuadCellType::Contour,
                    ..
                }),
            ) => {
                if [i, q, r, s, t]
                    .into_iter()
                    .map(|t| score(sample.clone(), *min, *max, *t))
                    .all(|t| t < 0.001)
                {
                    QuadTree::Leaf(QuadCell {
                        min: *min,
                        max: *max,
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

    match tree {
        Tree::Root(t) => {
            let mut iter = t.into_iter().map(|t| merge(sample.clone(), *t));
            let (a, b, c, d) = (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            );
            merge_(sample.clone(), a, b, c, d)
        }
        Tree::Leaf(_) => tree,
    }
}

#[cfg(test)]
mod test {
    use viuer::Config;

    use super::*;

    #[test]
    fn test_quad_tree() {
        let sample = |p: [f64; 2]| (p.x() * p.x() + p.y() * p.y()).sqrt() - 0.6;
        let quad_tree = QuadTree::new([-1.0, -1.0], [1.0, 1.0], 2);
        let quad_tree = merge(sample, quad_tree);
        let quad_tree = quad_tree.collapse(sample);

        let image = draw_marching_squares(sample, quad_tree);

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
