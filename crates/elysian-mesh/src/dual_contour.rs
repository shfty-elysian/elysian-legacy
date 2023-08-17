use elysian_ir::{
    ast::DISTANCE,
    module::{Evaluate, EvaluateError},
};
use image::{ImageBuffer, Rgb};
use nalgebra::{Matrix2, Vector2};

use crate::{
    marching_squares::contours,
    quad_tree::{Bounds, QuadCellType, QuadTree},
    util::Vec2,
};

pub fn deriv<'a>(evaluator: &impl Evaluate<'a>, p: [f64; 2]) -> Result<[f64; 2], EvaluateError> {
    let epsilon = 0.001;
    let dx = f64::from(
        evaluator
            .sample_2d([p.x() + epsilon, p.y()])?
            .get(&DISTANCE.into()),
    ) - f64::from(
        evaluator
            .sample_2d([p.x() - epsilon, p.y()])?
            .get(&DISTANCE.into()),
    );
    let dy = f64::from(
        evaluator
            .sample_2d([p.x(), p.y() + epsilon])?
            .get(&DISTANCE.into()),
    ) - f64::from(
        evaluator
            .sample_2d([p.x(), p.y() - epsilon])?
            .get(&DISTANCE.into()),
    );
    let len = (dx * dx + dy * dy).sqrt();
    Ok([dx / len, dy / len])
}

pub trait DualContour<'a> {
    fn dual_contour(&self, sample: &impl Evaluate<'a>)
        -> Result<Vec<[[f64; 2]; 2]>, EvaluateError>;
}

impl<'a> DualContour<'a> for QuadTree {
    fn dual_contour(
        &self,
        sample: &impl Evaluate<'a>,
    ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
        fn edge_proc_h<'a>(
            sample: &impl Evaluate<'a>,
            a: QuadTree,
            b: QuadTree,
        ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
            Ok(match (a, b) {
                (crate::tree::Tree::Leaf(a), crate::tree::Tree::Leaf(b)) => {
                    if a.ty != QuadCellType::Contour || b.ty != QuadCellType::Contour {
                        return Ok(vec![]);
                    }

                    if let (Some(lhs), Some(rhs)) =
                        (feature(sample, a.bounds)?, feature(sample, b.bounds)?)
                    {
                        vec![[lhs, rhs]]
                    } else {
                        vec![]
                    }
                }
                (crate::tree::Tree::Leaf(l), crate::tree::Tree::Root(t)) => {
                    if l.ty != QuadCellType::Contour {
                        return Ok(vec![]);
                    }

                    let leaf = crate::tree::Tree::Leaf(l);

                    let (a, c) = (*t[0].clone(), *t[2].clone());
                    edge_proc_h(sample, leaf.clone(), a)?
                        .into_iter()
                        .chain(edge_proc_h(sample, leaf, c)?)
                        .collect()
                }
                (crate::tree::Tree::Root(t), crate::tree::Tree::Leaf(l)) => {
                    if l.ty != QuadCellType::Contour {
                        return Ok(vec![]);
                    }

                    let (b, d) = (*t[1].clone(), *t[3].clone());

                    let leaf = crate::tree::Tree::Leaf(l);

                    edge_proc_h(sample, leaf.clone(), b)?
                        .into_iter()
                        .chain(edge_proc_h(sample, leaf, d)?)
                        .collect()
                }
                (crate::tree::Tree::Root(lhs), crate::tree::Tree::Root(rhs)) => {
                    let (b, d) = (*lhs[1].clone(), *lhs[3].clone());
                    let (a, c) = (*rhs[0].clone(), *rhs[2].clone());
                    edge_proc_h(sample, b, a)?
                        .into_iter()
                        .chain(edge_proc_h(sample, d, c)?)
                        .collect()
                }
            })
        }

        fn edge_proc_v<'a>(
            sample: &impl Evaluate<'a>,
            a: QuadTree,
            b: QuadTree,
        ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
            Ok(match (a, b) {
                (crate::tree::Tree::Leaf(a), crate::tree::Tree::Leaf(b)) => {
                    if a.ty != QuadCellType::Contour || b.ty != QuadCellType::Contour {
                        return Ok(vec![]);
                    }

                    if let (Some(lhs), Some(rhs)) =
                        (feature(sample, a.bounds)?, feature(sample, b.bounds)?)
                    {
                        vec![[lhs, rhs]]
                    } else {
                        vec![]
                    }
                }
                (crate::tree::Tree::Root(t), crate::tree::Tree::Leaf(l)) => {
                    if l.ty != QuadCellType::Contour {
                        return Ok(vec![]);
                    }

                    let leaf = crate::tree::Tree::Leaf(l);

                    let (c, d) = (*t[2].clone(), *t[3].clone());
                    edge_proc_v(sample, leaf.clone(), c)?
                        .into_iter()
                        .chain(edge_proc_v(sample, leaf.clone(), d)?)
                        .collect()
                }
                (crate::tree::Tree::Leaf(l), crate::tree::Tree::Root(t)) => {
                    if l.ty != QuadCellType::Contour {
                        return Ok(vec![]);
                    }

                    let leaf = crate::tree::Tree::Leaf(l);

                    let (a, b) = (*t[0].clone(), *t[2].clone());
                    edge_proc_v(sample, leaf.clone(), a)?
                        .into_iter()
                        .chain(edge_proc_v(sample, leaf.clone(), b)?)
                        .collect()
                }
                (crate::tree::Tree::Root(lhs), crate::tree::Tree::Root(rhs)) => {
                    let (c, a) = (*lhs[2].clone(), *lhs[0].clone());
                    let (d, b) = (*rhs[3].clone(), *rhs[1].clone());
                    edge_proc_v(sample, b, a)?
                        .into_iter()
                        .chain(edge_proc_v(sample, d, c)?)
                        .collect()
                }
            })
        }

        pub fn feature<'a>(
            evaluator: &impl Evaluate<'a>,
            bounds: Bounds,
        ) -> Result<Option<[f64; 2]>, EvaluateError> {
            let pts_: Vec<_> = contours(evaluator, bounds)?.into_iter().flatten().collect();

            Ok(if pts_.len() >= 2 {
                let pts: Vec<_> = pts_
                    .iter()
                    .map(|foo| Vector2::new(foo.x(), foo.y()))
                    .collect();

                let nms: Vec<_> = pts_
                    .iter()
                    .map(|t| deriv(evaluator, *t))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
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
            })
        }

        Ok(match self {
            crate::tree::Tree::Root(t) => {
                let (a, b, c, d) = (*t[0].clone(), *t[1].clone(), *t[2].clone(), *t[3].clone());
                t.into_iter()
                    .map(|t| t.dual_contour(sample))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten()
                    .chain(edge_proc_h(sample, a.clone(), b.clone())?)
                    .chain(edge_proc_h(sample, c.clone(), d.clone())?)
                    .chain(edge_proc_v(sample, a.clone(), c.clone())?)
                    .chain(edge_proc_v(sample, b.clone(), d.clone())?)
                    .collect()
            }
            _ => vec![],
        })
    }
}

pub fn draw_dual_contour(contours: Vec<[[f64; 2]; 2]>) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
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
    use elysian_interpreter::Interpreted;
    use elysian_ir::module::{AsModule, Dispatch, EvaluateError, SpecializationData};
    use elysian_shapes::{
        field::Point,
        modify::{ClampMode, IntoElongateAxis, IntoIsosurface},
    };
    use elysian_static::Precompiled;
    use viuer::Config;

    use crate::quad_tree::{Bounds, QuadTree};

    use super::*;

    #[test]
    fn test_dual_contour() -> Result<(), EvaluateError> {
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
        .dual_contour(&evaluator)?;

        let image = draw_dual_contour(contours);

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
