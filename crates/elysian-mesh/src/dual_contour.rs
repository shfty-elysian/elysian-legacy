use elysian_ir::{
    ast::GRADIENT_2D,
    module::{Evaluate, EvaluateError},
};

use nalgebra::{Matrix2, Vector2};

use crate::{
    marching_squares::{contours, Contour},
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
) -> Result<Option<(Contour, [f64; 2])>, EvaluateError> {
    let (contour, edges) = contours(evaluator, bounds)?;

    Ok(if edges.len() >= 1 {
        let points: Vec<_> = edges.into_iter().flatten().collect();

        let pts: Vec<_> = points.iter().map(|p| Vector2::new(p.x(), p.y())).collect();

        let nms: Vec<_> = points
            .iter()
            .map(|p| {
                Ok(
                    <[f64; 2]>::try_from(evaluator.sample_2d(*p)?.get(&GRADIENT_2D.into()))
                        .unwrap(),
                ) as Result<[f64; 2], EvaluateError>
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|t| Vector2::new(t.x(), t.y()))
            .collect();

        let center: Vector2<f64> = pts.iter().sum::<Vector2<f64>>() / pts.len() as f64;

        let a = Matrix2::from_row_iterator(points.into_iter().flatten());

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
            contour,
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
                    } else if let (Some((contour_l, feature_l)), Some((contour_r, feature_r))) = (
                        feature(evaluator, a.bounds, epsilon)?,
                        feature(evaluator, b.bounds, epsilon)?,
                    ) {
                        if contour_l.has_sign_change(Contour::RIGHT)
                            && contour_l.neighbours(contour_r, Contour::RIGHT)
                        {
                            println!("Connecting {contour_l:?}, {contour_r:?}");
                            vec![[feature_l, feature_r]]
                        } else {
                            println!("Skipping {contour_l:?}, {contour_r:?}");
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
                    } else if let (Some((contour_l, feature_l)), Some((contour_r, feature_r))) = (
                        feature(evaluator, a.bounds, epsilon)?,
                        feature(evaluator, b.bounds, epsilon)?,
                    ) {
                        if contour_l.has_sign_change(Contour::UP)
                            && contour_l.neighbours(contour_r, Contour::UP)
                        {
                            println!("Connecting {contour_l:?}, {contour_r:?}");
                            vec![[feature_l, feature_r]]
                        } else {
                            println!("Skipping {contour_l:?}, {contour_r:?}");
                            vec![]
                        }
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
