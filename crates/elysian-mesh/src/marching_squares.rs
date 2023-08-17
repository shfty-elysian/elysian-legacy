use image::{ImageBuffer, Rgb};

use crate::{
    quad_tree::{Bounds, QuadCellType, QuadTree},
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

fn index(sample: impl Fn([f64; 2]) -> f64, bounds: Bounds) -> usize {
    let pts: Vec<_> = [bounds.min.y(), bounds.max.y()]
        .into_iter()
        .flat_map(|y| {
            [bounds.min.x(), bounds.max.x()]
                .into_iter()
                .map(move |x| [x, y])
        })
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

fn edges(sample: impl Fn([f64; 2]) -> f64, bounds: Bounds) -> Vec<(Side, Side)> {
    lut(index(sample, bounds))
}

fn pt(sample: impl Fn([f64; 2]) -> f64, bounds: Bounds, side: Side) -> [f64; 2] {
    match side {
        Side::Left => zero(
            sample,
            Bounds {
                min: [bounds.min.x(), bounds.min.y()],
                max: [bounds.min.x(), bounds.max.y()],
            },
        ),
        Side::Right => zero(
            sample,
            Bounds {
                min: [bounds.max.x(), bounds.min.y()],
                max: [bounds.max.x(), bounds.max.y()],
            },
        ),
        Side::Lower => zero(
            sample,
            Bounds {
                min: [bounds.min.x(), bounds.min.y()],
                max: [bounds.max.x(), bounds.min.y()],
            },
        ),
        Side::Upper => zero(
            sample,
            Bounds {
                min: [bounds.min.x(), bounds.max.y()],
                max: [bounds.max.x(), bounds.max.y()],
            },
        ),
    }
}

fn zero(sample: impl Fn([f64; 2]) -> f64, bounds: Bounds) -> [f64; 2] {
    fn pos(f: f64, bounds: Bounds) -> [f64; 2] {
        [
            bounds.min.x() * (1.0 - f) + bounds.max.x() * f,
            bounds.min.y() * (1.0 - f) + bounds.max.y() * f,
        ]
    }

    fn zero_(
        f: f64,
        step: f64,
        i: usize,
        sample: impl Fn([f64; 2]) -> f64,
        bounds: Bounds,
    ) -> [f64; 2] {
        if i == 0 {
            pos(f, bounds)
        } else if sample(pos(f, bounds)) < 0.0 {
            zero_(f + step, step / 2.0, i - 1, sample, bounds)
        } else {
            zero_(f - step, step / 2.0, i - 1, sample, bounds)
        }
    }

    if sample(bounds.min) >= 0.0 {
        zero_(
            0.5,
            0.25,
            10,
            sample,
            Bounds {
                min: bounds.max,
                max: bounds.min,
            },
        )
    } else {
        zero_(0.5, 0.25, 10, sample, bounds)
    }
}

pub fn contours(sample: impl Fn([f64; 2]) -> f64 + Clone, bounds: Bounds) -> Vec<[[f64; 2]; 2]> {
    edges(sample.clone(), bounds)
        .into_iter()
        .map(|(a, b)| [pt(sample.clone(), bounds, a), pt(sample.clone(), bounds, b)])
        .collect()
}

pub fn draw_marching_squares(
    sample: impl Fn([f64; 2]) -> f64 + Clone,
    tree: QuadTree,
) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let contours: Vec<_> = tree
        .into_iter()
        .filter(|t| t.ty == QuadCellType::Contour)
        .flat_map(|t| contours(sample.clone(), t.bounds))
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

#[cfg(test)]
mod test {
    use viuer::Config;

    use crate::quad_tree::Bounds;

    use super::*;

    #[test]
    fn test_marching_squares() {
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
