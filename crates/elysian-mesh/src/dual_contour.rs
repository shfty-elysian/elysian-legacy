use elysian_ir::{
    ast::GRADIENT_2D,
    module::{Evaluate, EvaluateError},
};
use image::{ImageBuffer, Rgb};
use nalgebra::{Matrix2, Vector2};

use crate::{
    marching_squares::{contours, Side},
    quad_tree::{Bounds, QuadCellType, QuadTree},
    util::Vec2,
};

pub trait DualContour<'a> {
    fn dual_contour(
        &self,
        sample: &impl Evaluate<'a>,
        epsilon: f64,
    ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError>;
}

pub fn feature<'a>(
    evaluator: &impl Evaluate<'a>,
    bounds: Bounds,
    epsilon: f64,
) -> Result<Option<(Vec<Side>, [f64; 2])>, EvaluateError> {
    let pts_: Vec<_> = contours(evaluator, bounds)?
        .into_iter()
        .map(|(_, points)| points)
        .flatten()
        .collect();

    Ok(if pts_.len() >= 2 {
        let sides: Vec<_> = pts_.iter().map(|(side, _)| *side).collect();

        let pts: Vec<_> = pts_
            .iter()
            .map(|(_, pt)| Vector2::new(pt.x(), pt.y()))
            .collect();

        let nms: Vec<_> = pts_
            .iter()
            .map(|(side, t)| {
                Ok(
                    <[f64; 2]>::try_from(evaluator.sample_2d(*t)?.get(&GRADIENT_2D.into()))
                        .unwrap(),
                ) as Result<[f64; 2], EvaluateError>
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|t| Vector2::new(t.x(), t.y()))
            .collect();

        let center: Vector2<f64> = pts.iter().sum::<Vector2<f64>>() / pts.len() as f64;

        let a = Matrix2::from_row_iterator(pts_.into_iter().map(|(_, v)| v).flatten());

        let cols = &pts
            .into_iter()
            .zip(nms)
            .map(|(pt, nm)| (pt - center).dot(&nm))
            .collect::<Vec<_>>();
        let b = Vector2::from_column_slice(cols);

        let p = center
            + (a.svd_unordered(true, true).solve(&b, epsilon))
                .unwrap()
                .column(0);
        Some((
            sides,
            [
                p.x.clamp(bounds.min.x(), bounds.max.x()),
                p.y.clamp(bounds.min.y(), bounds.max.y()),
            ],
        ))
    } else {
        None
    })
}

impl<'a> DualContour<'a> for QuadTree {
    fn dual_contour(
        &self,
        evaluator: &impl Evaluate<'a>,
        epsilon: f64,
    ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
        fn edge_proc_h<'a>(
            evaluator: &impl Evaluate<'a>,
            a: QuadTree,
            b: QuadTree,
            epsilon: f64,
        ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
            Ok(match (a, b) {
                (crate::tree::Tree::Leaf(a), crate::tree::Tree::Leaf(b)) => {
                    if a.ty != QuadCellType::Contour || b.ty != QuadCellType::Contour {
                        vec![]
                    } else if let (Some((sides_l, feature_l)), Some((sides_r, feature_r))) = (
                        feature(evaluator, a.bounds, epsilon)?,
                        feature(evaluator, b.bounds, epsilon)?,
                    ) {
                        if true
                        // TODO: Check compatibility between cell sides
                        {
                            println!("Connecting {sides_l:?}, {sides_r:?}");
                            vec![[feature_l, feature_r]]
                        } else {
                            println!("Skipping {sides_l:?}, {sides_r:?}");
                            vec![]
                        }
                    } else {
                        vec![]
                    }
                }
                (crate::tree::Tree::Leaf(l), crate::tree::Tree::Root([a, _, c, _])) => {
                    if l.ty != QuadCellType::Contour {
                        vec![]
                    } else {
                        let leaf = crate::tree::Tree::Leaf(l);

                        edge_proc_h(evaluator, leaf.clone(), *a, epsilon)?
                            .into_iter()
                            .chain(edge_proc_h(evaluator, leaf, *c, epsilon)?)
                            .collect()
                    }
                }
                (crate::tree::Tree::Root([_, b, _, d]), crate::tree::Tree::Leaf(l)) => {
                    if l.ty != QuadCellType::Contour {
                        vec![]
                    } else {
                        let leaf = crate::tree::Tree::Leaf(l);

                        edge_proc_h(evaluator, *b, leaf.clone(), epsilon)?
                            .into_iter()
                            .chain(edge_proc_h(evaluator, *d, leaf, epsilon)?)
                            .collect()
                    }
                }
                (crate::tree::Tree::Root([_, b, _, d]), crate::tree::Tree::Root([a, _, c, _])) => {
                    edge_proc_h(evaluator, *b, *a, epsilon)?
                        .into_iter()
                        .chain(edge_proc_h(evaluator, *d, *c, epsilon)?)
                        .collect()
                }
            })
        }

        fn edge_proc_v<'a>(
            evaluator: &impl Evaluate<'a>,
            a: QuadTree,
            b: QuadTree,
            epsilon: f64,
        ) -> Result<Vec<[[f64; 2]; 2]>, EvaluateError> {
            Ok(match (a, b) {
                (crate::tree::Tree::Leaf(a), crate::tree::Tree::Leaf(b)) => {
                    if a.ty != QuadCellType::Contour || b.ty != QuadCellType::Contour {
                        vec![]
                    } else if let (Some((sides_l, feature_l)), Some((sides_r, feature_r))) = (
                        feature(evaluator, a.bounds, epsilon)?,
                        feature(evaluator, b.bounds, epsilon)?,
                    ) {
                        vec![[feature_l, feature_r]]
                    } else {
                        vec![]
                    }
                }
                (crate::tree::Tree::Root([_, _, c, d]), crate::tree::Tree::Leaf(l)) => {
                    if l.ty != QuadCellType::Contour {
                        vec![]
                    } else {
                        let leaf = crate::tree::Tree::Leaf(l);

                        edge_proc_v(evaluator, *c, leaf.clone(), epsilon)?
                            .into_iter()
                            .chain(edge_proc_v(evaluator, *d, leaf.clone(), epsilon)?)
                            .collect()
                    }
                }
                (crate::tree::Tree::Leaf(l), crate::tree::Tree::Root([a, b, _, _])) => {
                    if l.ty != QuadCellType::Contour {
                        vec![]
                    } else {
                        let leaf = crate::tree::Tree::Leaf(l);

                        edge_proc_v(evaluator, leaf.clone(), *a, epsilon)?
                            .into_iter()
                            .chain(edge_proc_v(evaluator, leaf.clone(), *b, epsilon)?)
                            .collect()
                    }
                }
                (crate::tree::Tree::Root([_, _, c, d]), crate::tree::Tree::Root([a, b, _, _])) => {
                    edge_proc_v(evaluator, *c, *a, epsilon)?
                        .into_iter()
                        .chain(edge_proc_v(evaluator, *d, *b, epsilon)?)
                        .collect()
                }
            })
        }

        Ok(match self {
            crate::tree::Tree::Root([a, b, c, d]) => [
                a.dual_contour(evaluator, epsilon)?,
                b.dual_contour(evaluator, epsilon)?,
                c.dual_contour(evaluator, epsilon)?,
                d.dual_contour(evaluator, epsilon)?,
            ]
            .into_iter()
            .flatten()
            .chain(edge_proc_h(evaluator, *a.clone(), *b.clone(), epsilon)?)
            .chain(edge_proc_h(evaluator, *c.clone(), *d.clone(), epsilon)?)
            .chain(edge_proc_v(evaluator, *a.clone(), *c.clone(), epsilon)?)
            .chain(edge_proc_v(evaluator, *b.clone(), *d.clone(), epsilon)?)
            .collect(),
            _ => vec![],
        })
    }
}

pub fn draw_dual_contour(contours: Vec<[[f64; 2]; 2]>) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let mut image = ImageBuffer::new(64, 64);

    for [from, to] in contours.iter() {
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
        .dual_contour(&evaluator, 4.0)?;

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
