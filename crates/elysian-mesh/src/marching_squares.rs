use std::ops::Not;

use elysian_ir::{
    ast::{Struct, DISTANCE, POSITION_2D},
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
    ) -> Result<Vec<(Contour, [(Side, [f64; 2]); 2])>, EvaluateError>;
}

impl MarchingSquares for QuadTree {
    fn marching_squares<'a>(
        &self,
        evaluator: &impl Evaluate<'a>,
    ) -> Result<Vec<(Contour, [(Side, [f64; 2]); 2])>, EvaluateError> {
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

bitflags::bitflags! {
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Contour: usize {
        const UR = 0b0001;
        const UL = 0b0010;
        const DR = 0b0100;
        const DL = 0b1000;

        const UP = Self::UL.union(Self::UR).bits();
        const LEFT = Self::UL.union(Self::DL).bits();
        const RIGHT = Self::UR.union(Self::DR).bits();
        const DOWN = Self::DL.union(Self::DR).bits();

        const INV_UR = 0b1110;
        const INV_UL = 0b1101;
        const INV_DR = 0b1011;
        const INV_DL = 0b0111;

        const UL_DR = Self::UL.union(Self::DR).bits();
        const UR_DL = Self::UR.union(Self::DL).bits();

        const EMPTY = Self::empty().bits();
        const FULL = Self::UL.union(Self::UR).union(Self::DL).union(Self::DR).bits();
    }
}

impl Contour {
    /// Test whether `self` can connect to `rhs` on the given side
    pub fn neighbours(self, rhs: Self, side: Contour) -> bool {
        assert!(
            [Contour::UP, Contour::LEFT, Contour::RIGHT, Contour::DOWN].contains(&side),
            "Side must be one of Contour::{{ UP, LEFT, RIGHT, DOWN }}"
        );

        side.iter()
            .zip(side.not().iter())
            .fold(true, |acc, (from, to)| {
                acc & ((self & from).is_empty() == (rhs & to).is_empty())
            })
    }

    pub fn sides(self) -> Vec<[Side; 2]> {
        match self {
            Contour::EMPTY => vec![],

            Contour::UL => vec![[Side::Upper, Side::Left]],
            Contour::UR => vec![[Side::Upper, Side::Right]],
            Contour::DL => vec![[Side::Lower, Side::Left]],
            Contour::DR => vec![[Side::Lower, Side::Right]],

            Contour::UP => vec![[Side::Left, Side::Right]],
            Contour::RIGHT => vec![[Side::Upper, Side::Lower]],
            Contour::DOWN => vec![[Side::Left, Side::Right]],
            Contour::LEFT => vec![[Side::Upper, Side::Lower]],

            Contour::INV_UL => Contour::UL.sides(),
            Contour::INV_UR => Contour::UR.sides(),
            Contour::INV_DL => Contour::DL.sides(),
            Contour::INV_DR => Contour::DR.sides(),

            Contour::UL_DR => Contour::UL
                .sides()
                .into_iter()
                .chain(Contour::DR.sides())
                .collect(),
            Contour::UR_DL => Contour::UR
                .sides()
                .into_iter()
                .chain(Contour::DL.sides())
                .collect(),

            Contour::FULL => vec![],

            _ => unimplemented!(),
        }
    }
}

#[test]
fn test_contour_flags() {
    assert!(Contour::empty().neighbours(Contour::empty(), Contour::UP));
    assert!(Contour::empty().neighbours(Contour::empty(), Contour::LEFT));
    assert!(Contour::empty().neighbours(Contour::empty(), Contour::DOWN));
    assert!(Contour::empty().neighbours(Contour::empty(), Contour::RIGHT));

    assert!(Contour::empty().neighbours(Contour::UL, Contour::UP));
    assert!(Contour::empty().neighbours(Contour::UL, Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::UL, Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::UL, Contour::RIGHT));

    assert!(Contour::empty().neighbours(Contour::UR, Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::UR, Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::UR, Contour::DOWN));
    assert!(Contour::empty().neighbours(Contour::UR, Contour::RIGHT));

    assert!(Contour::empty().neighbours(Contour::UP, Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::UP, Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::UP, Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::UP, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::DR, Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::DR, Contour::LEFT));
    assert!(Contour::empty().neighbours(Contour::DR, Contour::DOWN));
    assert!(Contour::empty().neighbours(Contour::DR, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::DR | Contour::UL, Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::DR | Contour::UL, Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::DR | Contour::UL, Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::DR | Contour::UL, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::RIGHT, Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::RIGHT, Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::RIGHT, Contour::DOWN));
    assert!(Contour::empty().neighbours(Contour::RIGHT, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::DL.not(), Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::DL.not(), Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::DL.not(), Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::DL.not(), Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::DL, Contour::UP));
    assert!(Contour::empty().neighbours(Contour::DL, Contour::LEFT));
    assert!(Contour::empty().neighbours(Contour::DL, Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::DL, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::LEFT, Contour::UP));
    assert!(Contour::empty().neighbours(Contour::LEFT, Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::LEFT, Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::LEFT, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::UR | Contour::DL, Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::UR | Contour::DL, Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::UR | Contour::DL, Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::UR | Contour::DL, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::DR.not(), Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::DR.not(), Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::DR.not(), Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::DR.not(), Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::DOWN, Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::DOWN, Contour::LEFT));
    assert!(Contour::empty().neighbours(Contour::DOWN, Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::DOWN, Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::UR.not(), Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::UR.not(), Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::UR.not(), Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::UR.not(), Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::UL.not(), Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::UL.not(), Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::UL.not(), Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::UL.not(), Contour::RIGHT));

    assert!(!Contour::empty().neighbours(Contour::all(), Contour::UP));
    assert!(!Contour::empty().neighbours(Contour::all(), Contour::LEFT));
    assert!(!Contour::empty().neighbours(Contour::all(), Contour::DOWN));
    assert!(!Contour::empty().neighbours(Contour::all(), Contour::RIGHT));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Upper,
    Lower,
    Left,
    Right,
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
) -> Result<Vec<(Contour, [(Side, [f64; 2]); 2])>, EvaluateError> {
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

    let contour = Contour::from_bits(index(evaluator, bounds)?).unwrap();

    Ok(contour
        .sides()
        .into_iter()
        .map(|[a, b]| {
            Ok((
                contour,
                [
                    (a, pt(evaluator, bounds, a)?),
                    (b, pt(evaluator, bounds, b)?),
                ],
            )) as Result<(Contour, [(Side, [f64; 2]); 2]), EvaluateError>
        })
        .collect::<Result<Vec<_>, _>>()?
        .chunks(2)
        .flat_map(|chunk| chunk.to_vec())
        .collect())
}

pub fn draw_marching_squares(samples: Vec<[Struct; 2]>) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let mut image = ImageBuffer::new(64, 64);

    for [from, to] in samples.iter() {
        let from = <[f64; 2]>::from(from.get(&POSITION_2D.into()));
        let to = <[f64; 2]>::from(to.get(&POSITION_2D.into()));

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
            .elongate_axis([0.1, 0.0], ClampMode::Dir, ClampMode::Dir)
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

        let samples = contours
            .into_iter()
            .map(|(_, [(_, from), (_, to)])| {
                Ok(<[Struct; 2]>::try_from([
                    evaluator
                        .sample_2d(from)?
                        .set(POSITION_2D.into(), from.into()),
                    evaluator.sample_2d(to)?.set(POSITION_2D.into(), to.into()),
                ])
                .unwrap())
            })
            .collect::<Result<Vec<_>, EvaluateError>>()?;

        let image = draw_marching_squares(samples);

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
