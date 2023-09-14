use elysian::{
    core::property_identifier::IntoPropertyIdentifier,
    interpreter::Interpreted,
    ir::{
        ast::{Struct, GRADIENT_2D, GRADIENT_3D, NORMAL, POSITION_2D, POSITION_3D},
        module::{
            AsModule, Dispatch, EvaluateError, SpecializationData, StructIdentifier, CONTEXT,
        },
    },
    mesh::{
        bounds::Bounds,
        gltf_export::{samples_to_root, Mode},
        marching_cells::{MarchingCell, SampleBounds, SampleCorners, ZeroPoint},
        sample::Sample,
        subdivision_tree::{
            CellType, HasSignChange, Neighbours, Octree, Pairs, QuadTree, SubdivisionCell,
        },
        util::CollectArray,
        vector_space::{D2, D3},
    },
    r#static::Precompiled,
    shapes::{
        field::Point,
        modify::{IntoGradientNormals, IntoIsosurface},
    },
};
use nalgebra::{Matrix2, Matrix3, Vector2, Vector3};

fn main() -> Result<(), EvaluateError> {
    main_2d()?;
    main_3d()?;
    Ok(())
}

fn main_2d() -> Result<(), EvaluateError> {
    // Create shape module and evaluator
    let module = Point
        .isosurface(0.8) //test_shapes::test_shape()
        .module(&SpecializationData::new_2d())
        .finalize();

    let evaluator = Dispatch(vec![
        Box::new(Precompiled(&module)),
        Box::new(Interpreted(&module)),
    ]);

    // Create QuadTree, marching cubes, dual contour data
    let level = 5;
    let epsilon = 1.0;

    let quad_tree = QuadTree::new(
        Bounds {
            min: [-1.0, -1.0].into(),
            max: [1.0, 1.0].into(),
        },
        level,
    )
    .merge(&evaluator, 0.04)?
    .collapse(&evaluator)?;

    let position_2d = POSITION_2D.prop();

    let quad_tree_cells = quad_tree
        .iter()
        .map(|SubdivisionCell { bounds, .. }| {
            let mut samples = evaluator.sample_bounds(*bounds)?;
            samples.swap(0, 1);
            Ok(samples)
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?
        .into_iter()
        .map(|l| (Mode::LineLoop, [&position_2d], l.to_vec(), None))
        .collect::<Vec<_>>();

    let marching_squares = quad_tree
        .iter()
        .filter(|cell| cell.ty == CellType::Contour)
        .map(|cell| {
            let corners = evaluator.sample_corners(cell.bounds)?;
            let march_cell = corners.marching_cell();

            // Synthesize samples for GLTF conversion
            let node = march_cell
                .into_iter()
                .map(|edges| {
                    (
                        Mode::Lines,
                        [&position_2d],
                        edges
                            .into_iter()
                            .map(|t| {
                                let p = evaluator.zero_point(t, cell.bounds).unwrap();

                                Struct::new(StructIdentifier(CONTEXT))
                                    .set(POSITION_2D.into(), [p.x, p.y].into())
                            })
                            .collect::<Vec<_>>(),
                        None as Option<Vec<u32>>,
                    )
                })
                .collect::<Vec<_>>();

            Ok(node) as Result<_, EvaluateError>
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten();

    let dual_graph = quad_tree
        .pairs()
        .into_iter()
        // Filter contour cells
        .filter(|(_, [a, b])| a.is_contour() && b.is_contour())
        // Filter cells that should connect
        .filter(|(face, [a, b])| {
            let a = evaluator.sample_corners(a.bounds).unwrap();
            let b = evaluator.sample_corners(b.bounds).unwrap();

            a.has_sign_change(face) && a.neighbours(&b, face)
        })
        // Calculate position within cell
        .map(|(_, [a, b])| {
            Ok((
                Mode::Lines,
                [&position_2d],
                [a, b]
                    .into_iter()
                    .map(|cell| {
                        let corners = evaluator.sample_corners(cell.bounds)?;
                        let march_cell = corners.marching_cell();

                        let points = march_cell
                            .into_iter()
                            .flat_map(|edges| {
                                edges
                                    .into_iter()
                                    .map(|edge| evaluator.zero_point(edge, cell.bounds).unwrap())
                            })
                            //.map(|corner| Vector2::from(corner.local_point(cell.bounds)))
                            .collect::<Vec<_>>();

                        let normals: Vec<_> = points
                            .iter()
                            .map(|p| {
                                Ok(<[f64; 2]>::try_from(
                                    Sample::<D2>::sample(&evaluator, (*p).into())?
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

                        let a =
                            Matrix2::from_row_iterator(points.iter().flat_map(|pt| [pt.x, pt.y]));

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
                            + (a.svd_unordered(true, true).solve(&b, epsilon))
                                .unwrap()
                                .column(0);

                        Ok(Vector2::new(
                            p.x.clamp(cell.bounds.min.x, cell.bounds.max.x),
                            p.y.clamp(cell.bounds.min.y, cell.bounds.max.y),
                        ))
                    })
                    .collect::<Result<Vec<_>, EvaluateError>>()?
                    .into_iter()
                    .map(|p| {
                        Ok(Sample::<D2>::sample(&evaluator, p.into())?
                            .set(POSITION_2D.into(), [p.x, p.y].into()))
                    })
                    .collect::<Result<Vec<_>, EvaluateError>>()?
                    .into_iter()
                    .map(|p| {
                        Struct::new(StructIdentifier(CONTEXT))
                            .set(POSITION_2D.into(), p.get(&POSITION_2D.into()))
                    })
                    .collect::<Vec<_>>(),
                None,
            ))
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?;

    let root = samples_to_root([[
        quad_tree_cells,
        marching_squares.collect::<Vec<_>>(),
        dual_graph,
    ]]);

    std::fs::write("./2d.gltf", root.to_string()?)?;

    // Done
    Ok(())
}

fn main_3d() -> Result<(), EvaluateError> {
    // Create shape module and evaluator
    let module = Point
        .isosurface(0.8)
        .gradient_normals()
        .module(&SpecializationData::new_3d())
        .finalize();

    let evaluator = Dispatch(vec![
        Box::new(Precompiled(&module)),
        Box::new(Interpreted(&module)),
    ]);

    // Create Octree, marching cubes, dual contour data
    let level = 3;
    let epsilon = 1.0;

    let octree = Octree::new(
        Bounds {
            min: [-1.0, -1.0, -1.0].into(),
            max: [1.0, 1.0, 1.0].into(),
        },
        level,
    )
    .merge(&evaluator, 0.04)?
    .collapse(&evaluator)?;

    let position_3d = POSITION_3D.prop();
    let normal = NORMAL.prop();

    let octree_cells = octree
        .iter()
        .map(|SubdivisionCell { bounds, .. }| {
            let samples = evaluator.sample_bounds(*bounds)?;

            Ok([(
                samples,
                vec![
                    0, 1, 1, 3, 2, 3, 2, 0, 4, 5, 5, 7, 6, 7, 6, 4, 0, 4, 1, 5, 2, 6, 3, 7,
                ],
            )])
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?
        .into_iter()
        .flat_map(|points| {
            points.into_iter().map(|(samples, indices)| {
                (
                    Mode::Lines,
                    vec![&position_3d, &normal],
                    samples,
                    Some(indices),
                )
            })
        })
        .collect::<Vec<_>>();

    let marching_cubes = octree
        .iter()
        .filter(|cell| cell.ty == CellType::Contour)
        .map(|cell| {
            let corners = evaluator.sample_corners(cell.bounds)?;
            let march_cell = corners.marching_cell();

            // Synthesize samples for GLTF conversion
            let node = march_cell
                .into_iter()
                .map(|(samples, indices)| {
                    (
                        Mode::Triangles,
                        vec![&position_3d, &normal],
                        samples
                            .into_iter()
                            .map(|t| {
                                let p = evaluator.zero_point(t, cell.bounds).unwrap();
                                let s = Sample::<D3>::sample(&evaluator, p).unwrap();

                                Struct::new(StructIdentifier(CONTEXT))
                                    .set(POSITION_3D.into(), [p.x, p.y, p.z].into())
                                    .set(NORMAL.into(), s.get(&NORMAL.into()))
                            })
                            .collect::<Vec<_>>(),
                        indices,
                    )
                })
                .collect::<Vec<_>>();

            Ok(node) as Result<_, EvaluateError>
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten();

    // Dual contouring

    let dual_pairs = octree
        .pairs()
        .into_iter()
        // Filter contour cells
        .filter(|(_, [a, b])| a.is_contour() && b.is_contour())
        // Filter cells that should connect
        .filter_map(|(face, [a, b])| {
            let ca = evaluator.sample_corners(a.bounds).unwrap();
            let cb = evaluator.sample_corners(b.bounds).unwrap();

            if ca.has_sign_change(&face) && ca.neighbours(&cb, &face) {
                Some([a, b])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let dual_edges: Vec<[Vector3<f64>; 2]> = dual_pairs
        .into_iter()
        .map(|pair| {
            pair.into_iter()
                .map(|cell| {
                    // Calculate position within cell
                    let corners = evaluator.sample_corners(cell.bounds)?;
                    let march_cell = corners.marching_cell();

                    let points = march_cell
                        .into_iter()
                        .flat_map(|(edges, _)| {
                            edges
                                .into_iter()
                                .map(|edge| evaluator.zero_point(edge, cell.bounds).unwrap())
                        })
                        //.map(|corner| Vector2::from(corner.local_point(cell.bounds)))
                        .collect::<Vec<_>>();

                    let normals: Vec<_> = points
                        .iter()
                        .map(|p| {
                            Ok(<[f64; 3]>::try_from(
                                Sample::<D3>::sample(&evaluator, (*p).into())?
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
                        + (a.svd_unordered(true, true).solve(&b, epsilon))
                            .unwrap()
                            .column(0);

                    Ok(Vector3::new(
                        p.x.clamp(cell.bounds.min.x, cell.bounds.max.x),
                        p.y.clamp(cell.bounds.min.y, cell.bounds.max.y),
                        p.z.clamp(cell.bounds.min.z, cell.bounds.max.z),
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

    eprintln!("Edge count: {}", dual_edges.len());
    eprintln!("Edge pair count: {}", edge_pairs.len());
    eprintln!("Unique edge pair count: {}", unique_edge_pairs.len());
    eprintln!("Edge loop count: {}", edge_loops.len());
    eprintln!("Unique edge loop count: {}", unique_edge_loops.len());

    // Generate line geometry
    let dual_graph = dual_edges
        .iter()
        .map(|[a, b]| {
            let a = Sample::<D3>::sample(&evaluator, *a)?
                .set(POSITION_3D.into(), [a.x, a.y, a.z].into());

            let b = Sample::<D3>::sample(&evaluator, *b)?
                .set(POSITION_3D.into(), [b.x, b.y, b.z].into());

            Ok((
                Mode::Lines,
                vec![&position_3d],
                [a, b]
                    .into_iter()
                    .map(|p| {
                        Struct::new(StructIdentifier(CONTEXT))
                            .set(POSITION_3D.into(), p.get(&POSITION_3D.into()))
                    })
                    .collect::<Vec<_>>(),
                None as Option<Vec<u32>>,
            ))
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?;

    // Generate triangle geometry
    let triangles = unique_edge_loops
        .iter()
        .copied()
        .map(|points| {
            let mut points = points.to_vec();

            let cell_normal = points
                .iter()
                .copied()
                .map(|p| {
                    let [a, b, c]: [f64; 3] = Sample::<D3>::sample(&evaluator, *p)
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

            (
                Mode::Triangles,
                vec![&position_3d, &normal],
                points
                    .into_iter()
                    .map(|p| {
                        Sample::<D3>::sample(&evaluator, *p)
                            .unwrap()
                            .set(POSITION_3D.into(), [p.x, p.y, p.z].into())
                    })
                    .collect::<Vec<_>>(),
                Some(vec![0, 1, 2, 2, 3, 0]),
            )
        })
        .collect::<Vec<_>>();

    let root = samples_to_root([[
        //octree_cells,
        //marching_cubes.collect::<Vec<_>>(),
        //dual_graph,
        triangles,
    ]]);

    std::fs::write("./3d.gltf", root.to_string()?)?;

    // Done
    Ok(())
}
