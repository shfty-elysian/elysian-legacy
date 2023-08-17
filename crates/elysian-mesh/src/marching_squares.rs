use elysian_ir::{
    ast::DISTANCE,
    module::{Evaluate, EvaluateError},
};
use image::{ImageBuffer, Rgb};

use crate::{
    quad_tree::{Bounds, QuadCellType, QuadTree},
    util::Vec2,
};

pub trait MarchingSquares {
    fn marching_squares<'a>(
        &self,
        sample: &impl Evaluate<'a>,
    ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError>;
}

impl MarchingSquares for QuadTree {
    fn marching_squares<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
    ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
        Ok(self
            .iter()
            .filter(|t| t.ty == QuadCellType::Contour)
            .map(|t| contours(evaluator, t.bounds))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Upper,
    Lower,
    Left,
    Right,
}

fn pt<'a>(
    evaluator: &impl Evaluate<'a>,
    bounds: Bounds,
    side: Side,
) -> Result<[f64; 2], EvaluateError> {
    match side {
        Side::Left => zero(
            evaluator,
            Bounds {
                min: [bounds.min.x(), bounds.min.y()],
                max: [bounds.min.x(), bounds.max.y()],
            },
        ),
        Side::Right => zero(
            evaluator,
            Bounds {
                min: [bounds.max.x(), bounds.min.y()],
                max: [bounds.max.x(), bounds.max.y()],
            },
        ),
        Side::Lower => zero(
            evaluator,
            Bounds {
                min: [bounds.min.x(), bounds.min.y()],
                max: [bounds.max.x(), bounds.min.y()],
            },
        ),
        Side::Upper => zero(
            evaluator,
            Bounds {
                min: [bounds.min.x(), bounds.max.y()],
                max: [bounds.max.x(), bounds.max.y()],
            },
        ),
    }
}

fn zero<'a>(evaluator: &impl Evaluate<'a>, bounds: Bounds) -> Result<[f64; 2], EvaluateError> {
    fn pos(f: f64, bounds: Bounds) -> [f64; 2] {
        [
            bounds.min.x() * (1.0 - f) + bounds.max.x() * f,
            bounds.min.y() * (1.0 - f) + bounds.max.y() * f,
        ]
    }

    fn zero_impl<'a>(
        f: f64,
        step: f64,
        i: usize,
        evaluator: &impl Evaluate<'a>,
        bounds: Bounds,
    ) -> Result<[f64; 2], EvaluateError> {
        if i == 0 {
            Ok(pos(f, bounds))
        } else if f64::from(evaluator.sample_2d(pos(f, bounds))?.get(&DISTANCE.into())) < 0.0 {
            zero_impl(f + step, step / 2.0, i - 1, evaluator, bounds)
        } else {
            zero_impl(f - step, step / 2.0, i - 1, evaluator, bounds)
        }
    }

    zero_impl(
        0.5,
        0.25,
        10,
        evaluator,
        if f64::from(evaluator.sample_2d(bounds.min)?.get(&DISTANCE.into())) >= 0.0 {
            Bounds {
                min: bounds.max,
                max: bounds.min,
            }
        } else {
            bounds
        },
    )
}

pub fn contours<'a>(
    evaluator: &impl Evaluate<'a>,
    bounds: Bounds,
) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
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

    fn index<'a>(evaluator: &impl Evaluate<'a>, bounds: Bounds) -> Result<usize, EvaluateError> {
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
                Ok(
                    if f64::from(evaluator.sample_2d(pt)?.get(&DISTANCE.into())) < 0.0 {
                        2usize.pow((3 - i) as u32)
                    } else {
                        0
                    },
                )
            })
            .sum()
    }

    fn edges<'a>(
        evaluator: &impl Evaluate<'a>,
        bounds: Bounds,
    ) -> Result<Vec<(Side, Side)>, EvaluateError> {
        Ok(lut(index(evaluator, bounds)?))
    }

    Ok(edges(evaluator, bounds)?
        .into_iter()
        .flat_map(|(a, b)| [pt(evaluator, bounds, a), pt(evaluator, bounds, b)])
        .collect::<Result<Vec<_>, _>>()?
        .chunks(2)
        .map(|chunk| <[[f64; 2]; 2]>::try_from(chunk).unwrap())
        .collect())
}

pub fn draw_marching_squares(contours: Vec<[[f64; 2]; 2]>) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
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
    use elysian_interpreter::Interpreted;
    use elysian_ir::module::{AsModule, Dispatch, EvaluateError, SpecializationData};
    use elysian_shapes::{
        field::Point,
        modify::{ClampMode, IntoElongateAxis, IntoIsosurface},
    };
    use elysian_static::Precompiled;
    use viuer::Config;

    use crate::quad_tree::Bounds;

    use super::*;

    #[test]
    fn test_marching_squares() -> Result<(), EvaluateError> {
        let module = Point
            .isosurface(0.3)
            .elongate_axis([0.3, 0.0], ClampMode::Dir, ClampMode::Dir)
            .module(&SpecializationData::new_2d());

        let evaluator = Dispatch(vec![
            Box::new(Precompiled(&module)),
            Box::new(Interpreted(&module)),
        ]);

        let contours = QuadTree::new(
            Bounds {
                min: [-1.0, -1.0],
                max: [1.0, 1.0],
            },
            4,
        )
        .merge(&evaluator, 0.001)?
        .collapse(&evaluator)?
        .marching_squares(&evaluator)?;

        let image = draw_marching_squares(contours);

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
