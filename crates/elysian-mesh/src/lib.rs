use bounds::Bounds;
use dual_contour::AsDualContour;
use dual_graph::AsDualGraph;
use elysian_core::property_identifier::IntoPropertyIdentifier;
use elysian_ir::{
    ast::{GRADIENT_2D, GRADIENT_3D, NORMAL, POSITION_2D, POSITION_3D},
    module::EvaluateError,
};
use evaluator::Evaluator;
use gltf_export::{
    ExportPrimitive, ExportPrimitives, Indices, Nodes, PointPrimitives, Properties, Scenes,
};
use gltf_json::{mesh::Mode, Root};
use has_sign_change::HasSignChange;
use marching_cells::{Corners, Edge, LocalPoint};
use nalgebra::{Matrix2, Matrix3, Vector2, Vector3};
use neighbours::Neighbours;
use sample::Sample;
use subdivision_tree::{CellIndices, QuadTree, SubdivisionTree};
use util::CollectArray;
use vector_space::{VectorSpace, D2};

use crate::{marching_cells::ToMarchPrimitives, subdivision_tree::Octree, vector_space::D3};

pub mod bounds;
pub mod dual_contour;
pub mod dual_graph;
pub mod evaluator;
pub mod gltf_export;
pub mod has_sign_change;
pub mod interpolate_cell;
pub mod marching_cells;
pub mod neighbours;
pub mod sample;
pub mod subdivision_tree;
pub mod tree;
pub mod util;
pub mod vector_space;

fn subdivision_tree_lines<'a, 'b, D>(
    tree: &SubdivisionTree<D>,
    evaluator: &impl Sample<'a, D>,
    properties: Properties<'b>,
) -> Result<ExportPrimitives<'b>, EvaluateError>
where
    D: VectorSpace<f64>,
    SubdivisionTree<D>: CellIndices,
    Bounds<D>: IntoIterator<Item = D::DimensionVector>,
{
    Ok(tree
        .iter()
        .map(|cell| cell.bounds.samples(evaluator))
        .collect::<Result<Vec<_>, EvaluateError>>()?
        .into_iter()
        .map(|samples| ExportPrimitive {
            mode: Mode::Lines,
            properties: properties.clone(),
            samples,
            indices: Some(Indices::from_iter(
                SubdivisionTree::<D>::CELL_INDICES.into_iter().copied(),
            )),
        })
        .collect())
}

fn subdivision_tree_marching_primitives<'a, D>(
    tree: &SubdivisionTree<D>,
    evaluator: &impl Evaluator<'a, D>,
) -> Result<Vec<PointPrimitives<D>>, EvaluateError>
where
    D: VectorSpace<f64>,
    Bounds<D>: Clone + IntoIterator<Item = D::DimensionVector>,
    Corners<D>: ToMarchPrimitives<D>,
    Edge<D>: LocalPoint<D>,
{
    tree.cells_contour()
        .map(|cell| {
            let bounds = &cell.bounds;
            Ok(bounds
                .sample_corners(evaluator)?
                .march_primitives()
                .zero_points(evaluator, bounds)?)
        })
        .collect()
}

fn dual_graph_lines<'a, 'b, D>(
    tree: &SubdivisionTree<D>,
    evaluator: &impl Evaluator<'a, D>,
    properties: &Properties<'b>,
) -> Result<ExportPrimitives<'b>, EvaluateError>
where
    D: VectorSpace<f64>,
    SubdivisionTree<D>: AsDualGraph<D>,
    Bounds<D>: Clone + IntoIterator<Item = D::DimensionVector>,
    Corners<D>: ToMarchPrimitives<D> + Neighbours<D> + HasSignChange<D>,
    Edge<D>: LocalPoint<D>,
    D::DimensionVector: std::ops::Mul<f64, Output = D::DimensionVector>,
{
    tree.as_dual_graph()
        .iter()
        .map(|dual_pair| {
            let points = dual_pair
                .iter()
                .map(|cell| {
                    let p = cell.bounds.center();
                    evaluator.sample(p)
                })
                .collect::<Result<_, _>>()?;

            Ok(ExportPrimitive {
                mode: Mode::Lines,
                properties: properties.clone(),
                samples: points,
                indices: None,
            })
        })
        .collect::<Result<ExportPrimitives, EvaluateError>>()
}

pub fn quad_tree_2d<'a>(
    quad_tree: &QuadTree,
    evaluator: &impl Evaluator<'a, D2>,
) -> Result<(), EvaluateError> {
    let position_2d = POSITION_2D.prop();

    let quad_tree_lines =
        subdivision_tree_lines(&quad_tree, evaluator, Properties::from_iter([&position_2d]))?;

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([quad_tree_lines])]));

    std::fs::write("./2d_1_quad_tree.gltf", root.to_string()?)?;

    Ok(())
}

pub fn marching_squares_2d<'a>(
    quad_tree: &QuadTree,
    evaluator: &impl Evaluator<'a, D2>,
) -> Result<(), EvaluateError> {
    let position_2d = POSITION_2D.prop();

    // Generate vertices for marching cells
    let marching_squares = subdivision_tree_marching_primitives(&quad_tree, evaluator)?
        .into_iter()
        .map(|point_primitives| {
            point_primitives.export_lines(&Properties::from_iter([&position_2d]))
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?
        .into_iter()
        .flatten()
        .collect();

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([marching_squares])]));

    std::fs::write("./2d_2_marching_squares.gltf", root.to_string()?)?;

    // Done
    Ok(())
}

pub fn dual_graph_2d<'a>(
    quad_tree: &QuadTree,
    evaluator: &impl Evaluator<'a, D2>,
) -> Result<(), EvaluateError> {
    let position_2d = POSITION_2D.prop();

    let dual_graph_lines =
        dual_graph_lines(quad_tree, evaluator, &Properties::from_iter([&position_2d]))?;

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([dual_graph_lines])]));

    std::fs::write("./2d_3_dual_graph.gltf", root.to_string()?)?;

    Ok(())
}

pub fn dual_contour_2d<'a>(
    quad_tree: &QuadTree,
    evaluator: &impl Evaluator<'a, D2>,
) -> Result<(), EvaluateError> {
    let position_2d = POSITION_2D.prop();

    // Calculate position within cell
    let dual_contour_lines: ExportPrimitives = quad_tree
        .as_dual_contour(evaluator)
        .into_iter()
        .map(|contour| {
            Ok(contour
                .into_iter()
                .map(|cell| {
                    let bounds = &cell.bounds;

                    let points = bounds
                        .sample_corners(evaluator)?
                        .march_primitives()
                        .zero_points(evaluator, bounds)
                        .into_iter()
                        .flat_map(|point_primitives| {
                            point_primitives
                                .into_iter()
                                .flat_map(|point_primitive| point_primitive.points)
                        })
                        .collect::<Vec<_>>();

                    let normals: Vec<_> = points
                        .iter()
                        .map(|p| {
                            Ok(<[f64; 2]>::try_from(
                                Sample::<D2>::sample(evaluator, (*p).into())?
                                    .get(&GRADIENT_2D.into()),
                            )
                            .unwrap())
                                as Result<[f64; 2], EvaluateError>
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|t| Vector2::from(t))
                        .collect();

                    let center: Vector2<f64> =
                        points.iter().sum::<Vector2<_>>() / points.len() as f64;

                    let a = Matrix2::from_row_iterator(points.iter().flat_map(|pt| [pt.x, pt.y]));

                    let cols = &points
                        .into_iter()
                        .zip(normals)
                        .map(|(pt, nm)| {
                            let d = pt - center;
                            d.dot(&nm)
                        })
                        .collect::<Vec<_>>();
                    let b = Vector2::from_column_slice(cols);

                    let p = center
                        + (a.svd_unordered(true, true).solve(&b, 1.0))
                            .unwrap()
                            .column(0);

                    let bounds = &cell.bounds;
                    Ok(Vector2::new(
                        p.x.clamp(bounds.min.x, bounds.max.x),
                        p.y.clamp(bounds.min.y, bounds.max.y),
                    ))
                })
                .collect::<Result<Vec<_>, EvaluateError>>()?
                .into_iter()
                .map(|p| {
                    Ok(Sample::<D2>::sample(evaluator, p.into())?
                        .set(POSITION_2D.into(), [p.x, p.y].into()))
                })
                .collect::<Result<_, EvaluateError>>()?)
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?
        .into_iter()
        .map(|samples| ExportPrimitive {
            mode: Mode::Lines,
            properties: Properties::from_iter([&position_2d]),
            samples,
            indices: None,
        })
        .collect();

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([dual_contour_lines])]));

    std::fs::write("./2d_4_dual_contour.gltf", root.to_string()?)?;

    // Done
    Ok(())
}

pub fn octree_3d<'a>(
    octree: &Octree,
    evaluator: &impl Evaluator<'a, D3>,
) -> Result<(), EvaluateError> {
    let position_3d = POSITION_3D.prop();
    let normal = NORMAL.prop();

    let octree_lines = octree
        .iter()
        .map(|cell| cell.bounds.samples(evaluator))
        .collect::<Result<Vec<_>, EvaluateError>>()?
        .into_iter()
        .map(|samples| ExportPrimitive {
            mode: Mode::Lines,
            properties: Properties::from_iter([&position_3d, &normal]),
            samples,
            indices: Some(Indices::from_iter(
                SubdivisionTree::<D3>::CELL_INDICES.into_iter().copied(),
            )),
        })
        .collect();

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([octree_lines])]));

    std::fs::write("./3d_1_octree.gltf", root.to_string()?)?;

    // Done
    Ok(())
}

pub fn marching_cubes_3d<'a>(
    octree: &Octree,
    evaluator: &impl Evaluator<'a, D3>,
) -> Result<(), EvaluateError> {
    let position_3d = POSITION_3D.prop();
    let normal = NORMAL.prop();

    // Synthesize samples for GLTF conversion
    let marching_cubes = subdivision_tree_marching_primitives(&octree, evaluator)?
        .into_iter()
        .map(|point_primitives| {
            point_primitives
                .export_lines(&Properties::from_iter([&position_3d, &normal]), evaluator)
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?
        .into_iter()
        .flatten()
        .collect();

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([marching_cubes])]));

    std::fs::write("./3d_2_marching_cubes.gltf", root.to_string()?)?;

    // Done
    Ok(())
}

pub fn dual_graph_3d<'a>(
    octree: &Octree,
    evaluator: &impl Evaluator<'a, D3>,
) -> Result<(), EvaluateError> {
    let position_3d = POSITION_3D.prop();

    let dual_graph_lines =
        dual_graph_lines(octree, evaluator, &Properties::from_iter([&position_3d]))?;

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([dual_graph_lines])]));

    std::fs::write("./3d_3_dual_graph.gltf", root.to_string()?)?;

    Ok(())
}

pub fn dual_contour_3d<'a>(
    octree: &Octree,
    evaluator: &impl Evaluator<'a, D3>,
) -> Result<(), EvaluateError> {
    let position_3d = POSITION_3D.prop();
    let normal = NORMAL.prop();

    // Dual contouring

    let dual_graph = octree.as_dual_graph();

    let dual_edges: Vec<[Vector3<f64>; 2]> = dual_graph
        .contours()
        .map(|pair| {
            pair.into_iter()
                .map(|cell| {
                    let bounds = cell.bounds;

                    // Calculate position within cell
                    let points = bounds
                        .sample_corners(evaluator)?
                        .march_primitives()
                        .into_iter()
                        .flat_map(|march_primitive| {
                            march_primitive
                                .zero_points(evaluator, &bounds)
                                .unwrap()
                                .points
                            //.map(|edge| edge.local_point(bounds))
                        })
                        .collect::<Vec<_>>();

                    let normals: Vec<_> = points
                        .iter()
                        .map(|p| {
                            Ok(<[f64; 3]>::try_from(
                                Sample::<D3>::sample(evaluator, (*p).into())?
                                    .get(&GRADIENT_3D.into()),
                            )
                            .unwrap())
                                as Result<[f64; 3], EvaluateError>
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|t| Vector3::from(t))
                        .collect();

                    let center: Vector3<f64> =
                        points.iter().sum::<Vector3<_>>() / points.len() as f64;

                    let a =
                        Matrix3::from_row_iterator(points.iter().flat_map(|pt| [pt.x, pt.y, pt.z]));

                    let cols = &points
                        .into_iter()
                        .zip(normals)
                        .map(|(pt, nm)| {
                            let d = pt - center;
                            d.dot(&nm)
                        })
                        .collect::<Vec<_>>();
                    let b = Vector3::from_column_slice(cols);

                    let p = center
                        + (a.svd_unordered(true, true).solve(&b, 1.0))
                            .unwrap()
                            .column(0);

                    Ok(Vector3::new(
                        p.x.clamp(bounds.min.x, bounds.max.x),
                        p.y.clamp(bounds.min.y, bounds.max.y),
                        p.z.clamp(bounds.min.z, bounds.max.z),
                    ))
                })
                .collect::<Result<Vec<_>, EvaluateError>>()
                .unwrap()
                .into_iter()
                .collect_array()
        })
        .collect::<Vec<_>>();

    let edge_pairs: Vec<[&Vector3<f64>; 3]> = dual_edges
        .iter()
        .flat_map(|l @ [la, lb]| {
            dual_edges.iter().flat_map(move |r @ [ra, rb]| {
                // An edge cannot pair with itself
                if l == r {
                    return None;
                }

                // Order pairs such that shared points occupy the two middle indices
                if la == ra {
                    Some([lb, la, rb])
                } else if la == rb {
                    Some([lb, la, ra])
                } else if lb == ra {
                    Some([la, lb, rb])
                } else if lb == rb {
                    Some([la, lb, ra])
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let mut unique_edge_pairs: Vec<[&Vector3<f64>; 3]> = vec![];
    for pair @ [la, lb, lc] in edge_pairs.iter() {
        if unique_edge_pairs
            .iter()
            .find(|[ra, rb, rc]| {
                (la == ra && lb == rb && lc == rc) || (la == rc && lb == rb && lc == ra)
            })
            .is_some()
        {
            continue;
        }

        unique_edge_pairs.push(*pair);
    }

    let edge_loops: Vec<[&Vector3<f64>; 4]> = unique_edge_pairs
        .iter()
        .flat_map(|[la, lb, lc]| {
            unique_edge_pairs.iter().flat_map(move |[ra, rb, rc]| {
                if la == ra && lb != rb && lc == rc {
                    Some([*la, *lb, *lc, *rb])
                } else if lc == ra && lb != rb && la == rc {
                    Some([*lc, *lb, *la, *rb])
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let mut unique_edge_loops: Vec<[&Vector3<f64>; 4]> = vec![];
    for l @ [la, lb, lc, ld] in edge_loops.iter() {
        if unique_edge_loops
            .iter()
            .find(|r| r.contains(la) && r.contains(lb) && r.contains(lc) && r.contains(ld))
            .is_some()
        {
            continue;
        }

        unique_edge_loops.push(*l);
    }

    // Generate triangle geometry
    let dual_contour = unique_edge_loops
        .iter()
        .copied()
        .map(|points| {
            let mut points = points.to_vec();

            let cell_normal = points
                .iter()
                .copied()
                .map(|p| {
                    let [a, b, c]: [f64; 3] = Sample::<D3>::sample(evaluator, *p)
                        .unwrap()
                        .get(&GRADIENT_3D.prop())
                        .into();

                    Vector3::new(a, b, c)
                })
                .sum::<Vector3<f64>>()
                .normalize();

            let tangent = points[0].normalize().cross(&cell_normal);
            let cotangent = cell_normal.cross(&tangent);

            points.sort_by(|l, r| {
                let lx = l.dot(&tangent);
                let ly = l.dot(&cotangent);

                let rx = r.dot(&tangent);
                let ry = r.dot(&cotangent);

                ly.atan2(lx).partial_cmp(&ry.atan2(rx)).unwrap()
            });

            ExportPrimitive {
                mode: Mode::Triangles,
                properties: Properties::from_iter([&position_3d, &normal]),
                samples: points
                    .into_iter()
                    .map(|p| {
                        Sample::<D3>::sample(evaluator, *p)
                            .unwrap()
                            .set(POSITION_3D.into(), [p.x, p.y, p.z].into())
                    })
                    .collect(),
                indices: Some(Indices::from_iter([0, 1, 2, 2, 3, 0])),
            }
        })
        .collect();

    let root = Root::from(Scenes::from_iter([Nodes::from_iter([dual_contour])]));

    std::fs::write("./3d_4_dual_contour.gltf", root.to_string()?)?;

    // Done
    Ok(())
}
