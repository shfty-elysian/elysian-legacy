use image::{ImageBuffer, Rgb};
use nalgebra::{Matrix2, Vector2};

use crate::{
    marching_squares::contours,
    quad_tree::{Bounds, QuadTree},
    util::Vec2,
};

pub fn deriv(sample: impl Fn([f64; 2]) -> f64, p: [f64; 2]) -> [f64; 2] {
    let epsilon = 0.001;
    let dx = sample([p.x() + epsilon, p.y()]) - sample([p.x() - epsilon, p.y()]);
    let dy = sample([p.x(), p.y() + epsilon]) - sample([p.x(), p.y() - epsilon]);
    let len = (dx * dx + dy * dy).sqrt();
    [dx / len, dy / len]
}

pub fn feature(sample: impl Fn([f64; 2]) -> f64 + Clone, bounds: Bounds) -> Option<[f64; 2]> {
    let pts_: Vec<_> = contours(sample.clone(), bounds)
        .into_iter()
        .flatten()
        .collect();

    if pts_.len() >= 2 {
        let pts: Vec<_> = pts_
            .iter()
            .map(|foo| Vector2::new(foo.x(), foo.y()))
            .collect();

        let nms: Vec<_> = pts_
            .iter()
            .map(|t| deriv(sample.clone(), *t))
            .map(|t| Vector2::new(t.x(), t.y()))
            .collect();

        let center: Vector2<f64> = pts.iter().sum::<Vector2<f64>>() / pts.len() as f64;

        let a = Matrix2::from_row_iterator(pts_.into_iter().flatten());

        let cols = &pts
            .into_iter()
            .zip(nms)
            .map(|(pt, nm)| (pt - center).dot(&nm))
            .collect::<Vec<_>>();
        let b = Vector2::from_column_slice(cols);

        let p = center + (a.svd(true, true).solve(&b, 0.001)).unwrap().column(0);
        Some([p.x, p.y])
    } else {
        None
    }
}

fn face_proc(sample: impl Fn([f64; 2]) -> f64 + Clone, tree: QuadTree) -> Vec<[[f64; 2]; 2]> {
    match tree {
        crate::tree::Tree::Root(t) => {
            let (a, b, c, d) = (*t[0].clone(), *t[1].clone(), *t[2].clone(), *t[3].clone());
            t.into_iter()
                .flat_map(|t| face_proc(sample.clone(), *t))
                .chain(edge_proc_h(sample.clone(), a.clone(), b.clone()))
                .chain(edge_proc_h(sample.clone(), c.clone(), d.clone()))
                .chain(edge_proc_v(sample.clone(), a.clone(), c.clone()))
                .chain(edge_proc_v(sample.clone(), b.clone(), d.clone()))
                .collect()
        }
        _ => vec![],
    }
}

fn edge_proc_h(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    a: QuadTree,
    b: QuadTree,
) -> Vec<[[f64; 2]; 2]> {
    match (a, b) {
        (crate::tree::Tree::Leaf(a), crate::tree::Tree::Leaf(b)) => {
            if let (Some(lhs), Some(rhs)) = (
                feature(sample.clone(), a.bounds),
                feature(sample.clone(), b.bounds),
            ) {
                vec![[lhs, rhs]]
            } else {
                vec![]
            }
        }
        (leaf @ crate::tree::Tree::Leaf(_), crate::tree::Tree::Root(t)) => {
            let (a, c) = (*t[0].clone(), *t[2].clone());
            edge_proc_h(sample.clone(), leaf.clone(), a)
                .into_iter()
                .chain(edge_proc_h(sample.clone(), leaf, c))
                .collect()
        }
        (crate::tree::Tree::Root(t), leaf @ crate::tree::Tree::Leaf(_)) => {
            let (b, d) = (*t[1].clone(), *t[3].clone());
            edge_proc_h(sample.clone(), leaf.clone(), b)
                .into_iter()
                .chain(edge_proc_h(sample.clone(), leaf, d))
                .collect()
        }
        (crate::tree::Tree::Root(lhs), crate::tree::Tree::Root(rhs)) => {
            let (b, d) = (*lhs[1].clone(), *lhs[3].clone());
            let (a, c) = (*rhs[0].clone(), *rhs[2].clone());
            edge_proc_h(sample.clone(), b, a)
                .into_iter()
                .chain(edge_proc_h(sample.clone(), d, c))
                .collect()
        }
    }
}

fn edge_proc_v(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    a: QuadTree,
    b: QuadTree,
) -> Vec<[[f64; 2]; 2]> {
    match (a, b) {
        (crate::tree::Tree::Leaf(a), crate::tree::Tree::Leaf(b)) => {
            if let (Some(lhs), Some(rhs)) = (
                feature(sample.clone(), a.bounds),
                feature(sample.clone(), b.bounds),
            ) {
                vec![[lhs, rhs]]
            } else {
                vec![]
            }
        }
        (crate::tree::Tree::Root(t), leaf @ crate::tree::Tree::Leaf(_)) => {
            let (c, d) = (*t[2].clone(), *t[3].clone());
            edge_proc_v(sample.clone(), leaf.clone(), c)
                .into_iter()
                .chain(edge_proc_v(sample.clone(), leaf.clone(), d))
                .collect()
        }
        (leaf @ crate::tree::Tree::Leaf(_), crate::tree::Tree::Root(t)) => {
            let (a, b) = (*t[0].clone(), *t[2].clone());
            edge_proc_v(sample.clone(), leaf.clone(), a)
                .into_iter()
                .chain(edge_proc_v(sample.clone(), leaf.clone(), b))
                .collect()
        }
        (crate::tree::Tree::Root(lhs), crate::tree::Tree::Root(rhs)) => {
            let (c, a) = (*lhs[2].clone(), *lhs[0].clone());
            let (d, b) = (*rhs[3].clone(), *rhs[1].clone());
            edge_proc_v(sample.clone(), b, a)
                .into_iter()
                .chain(edge_proc_v(sample.clone(), d, c))
                .collect()
        }
    }
}

pub fn draw_dual_contour(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    tree: QuadTree,
) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let contours: Vec<_> = face_proc(sample.clone(), tree);

    let mut image = ImageBuffer::new(64, 64);

    for contour in contours.iter() {
        let (from, to) = (contour[0], contour[1]);

        let from_x = ((from.x() * 0.5 + 0.5) * image.width() as f64).floor() as u32;
        let from_y = ((from.y() * 0.5 + 0.5) * image.height() as f64).floor() as u32;

        let to_x = ((to.x() * 0.5 + 0.5) * image.width() as f64).floor() as u32;
        let to_y = ((to.y() * 0.5 + 0.5) * image.height() as f64).floor() as u32;

        image.put_pixel(from_x, from_y, Rgb([1.0, 0.0, 0.0]));
        image.put_pixel(to_x, to_y, Rgb([0.0, 0.0, 1.0]));
    }

    image
}

#[cfg(test)]
mod test {
    use viuer::Config;

    use crate::quad_tree::{Bounds, QuadTree};

    use super::*;

    #[test]
    fn test_dual_contour() {
        let sample = |p: [f64; 2]| (p.x() * p.x() + p.y() * p.y()).sqrt() - 0.6;
        let quad_tree = QuadTree::new(
            Bounds {
                min: [-1.0, -1.0],
                max: [1.0, 1.0],
            },
            2,
        )
        .merge(sample, 0.001)
        .collapse(sample);

        let image = draw_dual_contour(sample, quad_tree);

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
