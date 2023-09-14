use std::cmp::Ordering;

use nalgebra::Vector3;

use crate::{
    marching_cells::{Cell, Corner, Point},
    util::CollectArray,
};

use super::{edge::Edge, Corners, Edges, MarchingCell};

/// Comparator function for finding the closest edge position to a given point
fn distance_to(p: impl Into<Vector3<f64>>) -> impl Fn(&Edge<3>, &Edge<3>) -> Ordering {
    let p = p.into();
    move |lhs: &Edge<3>, rhs: &Edge<3>| {
        (Vector3::from(lhs.point()) - p)
            .magnitude()
            .partial_cmp(&(Vector3::from(rhs.point()) - p).magnitude())
            .unwrap()
    }
}

pub const QUAD_INDICES: [u32; 6] = [0, 1, 3, 3, 1, 2];

fn hexagon(
    points: Vec<Edge<3>>,
    mut lines: Vec<Edges<3>>,
    folded: bool,
) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    let line_base: Vec<_> = if folded {
        points.into_iter().collect()
    } else {
        lines.remove(1).into_iter().collect()
    };

    let [line_left, line_right] = lines.into_iter().collect_array();

    let [center_left, center_right] = [line_left, line_right]
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|t| Vector3::from(t.point()))
                .sum::<Vector3<f64>>()
                / 2.0
        })
        .collect_array();

    let [point_left, point_right] = [center_left, center_right]
        .into_iter()
        .map(|center| {
            let iter = line_base.clone().into_iter();
            if folded {
                iter.max_by(distance_to(center))
            } else {
                iter.min_by(distance_to(center))
            }
            .unwrap()
        })
        .collect_array();

    let [far_left, far_right] = [(line_left, point_left), (line_right, point_right)]
        .into_iter()
        .map(|(line, point)| line.into_iter().max_by(distance_to(point.point())).unwrap())
        .collect_array();

    vec![
        (line_left.into_iter().chain([point_left]).collect(), None),
        (line_right.into_iter().chain([point_right]).collect(), None),
        (
            if folded {
                vec![point_left, far_left, far_right, point_right]
            } else {
                vec![far_left, point_left, point_right, far_right]
            },
            Some(QUAD_INDICES.to_vec()),
        ),
    ]
}

fn double_quad(lines: Vec<Edges<3>>) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    let [(edge_a, position_a), b, c, d] = lines
        .into_iter()
        .map(|t| {
            (
                t,
                t.into_iter()
                    .map(|t| Vector3::from(t.point()))
                    .sum::<Vector3<f64>>()
                    / 2.0,
            )
        })
        .collect_array();

    let mut cands: Vec<_> = [b, c, d].into_iter().collect();
    cands.sort_by(|(_, lhs), (_, rhs)| {
        (lhs - position_a)
            .magnitude()
            .partial_cmp(&(rhs - position_a).magnitude())
            .unwrap()
    });

    let (edge_mid, _) = cands.remove(1);

    vec![
        (
            edge_a.into_iter().chain(edge_mid.into_iter()).collect(),
            Some(QUAD_INDICES.to_vec().clone()),
        ),
        (
            cands
                .into_iter()
                .flat_map(|(edge, _)| edge.into_iter())
                .collect(),
            Some(QUAD_INDICES.to_vec()),
        ),
    ]
}

fn fan(points: Vec<Edge<3>>, lines: Vec<Edges<3>>) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    let point = points[0];

    let [line_a, line_b] = [lines[0], lines[1]];

    let [edge_a, edge_b] = [line_a, line_b]
        .into_iter()
        .map(|line| line.into_iter().max_by(distance_to(point.point())).unwrap())
        .collect_array();

    lines
        .iter()
        .map(|line| (line.into_iter().chain([point]).collect(), None))
        .chain([((vec![edge_a, point, edge_b], None))])
        .collect()
}

fn inverse_fan(
    points: Vec<Edge<3>>,
    lines: Vec<Edges<3>>,
) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    let line = lines[0];

    let [line_left, line_right] = line.into_iter().collect_array();

    let [point_left, point_right] = [line_left, line_right]
        .into_iter()
        .map(|line| {
            points
                .iter()
                .copied()
                .min_by(distance_to(line.point()))
                .unwrap()
        })
        .collect_array();

    let point_center = points
        .iter()
        .find(|t| **t != point_left && **t != point_right)
        .copied()
        .unwrap();

    vec![
        (vec![point_left, point_center, line_left], None),
        (vec![line_left, point_center, line_right], None),
        (vec![line_right, point_center, point_right], None),
    ]
}

fn quad_and_triangle(
    points: Vec<Edge<3>>,
    lines: Vec<Edges<3>>,
) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    let lines_center: Vector3<f64> = lines
        .iter()
        .map(|t| {
            t.into_iter()
                .map(|t| Vector3::from(t.point()))
                .sum::<Vector3<_>>()
                / 2.0
        })
        .sum();

    let base = points
        .iter()
        .copied()
        .min_by(distance_to(lines_center))
        .unwrap();

    let points = points
        .into_iter()
        .filter(|point| *point != base)
        .collect::<Vec<_>>();

    let mut line_verts: Vec<_> = lines.into_iter().flat_map(|t| t.into_iter()).collect();
    line_verts.sort_by(distance_to(base.point()));

    vec![
        ((line_verts.drain(0..2).chain([base]).collect(), None)),
        ((
            line_verts.into_iter().chain(points).collect(),
            Some(QUAD_INDICES.to_vec()),
        )),
    ]
}

fn double_triangle(mut points: Vec<Edge<3>>) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    let base = points[0];
    points.sort_by(distance_to(base.point()));

    vec![
        ((points.drain(0..3).collect(), None)),
        ((points.into_iter().collect(), None)),
    ]
}

#[derive(PartialEq, PartialOrd)]
struct FloatOrd(f64);

impl Eq for FloatOrd {}

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn triangle(edges: Vec<Edge<3>>) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    vec![((edges, None))]
}

fn point_quad(points: Vec<Edge<3>>) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    vec![((points.into_iter().collect(), Some(QUAD_INDICES.to_vec())))]
}

fn line_quad(lines: Vec<Edges<3>>) -> Vec<(Vec<Edge<3>>, Option<Vec<u32>>)> {
    vec![
        ((
            lines.into_iter().flat_map(|t| t.into_iter()).collect(),
            Some(QUAD_INDICES.to_vec()),
        )),
    ]
}

impl MarchingCell for Corners<3> {
    type Item = (Vec<Edge<3>>, Option<Vec<u32>>);

    fn marching_cell(self) -> Vec<Self::Item> {
        let (corners, mut edges): (Vec<_>, Vec<_>) = self.cell().into_iter().unzip();

        let mut triangles = vec![];
        let mut push_edges =
            |corners: &[Corner<3>], mut edges: Vec<Edge<3>>, inds: Option<Vec<u32>>| {
                let corner_count = corners.len();
                let corner_center: Vector3<f64> = corners
                    .into_iter()
                    .map(|t| Vector3::from(t.point()))
                    .sum::<Vector3<_>>()
                    / corner_count as f64;

                let tri_center: Vector3<f64> = edges
                    .iter()
                    .map(|e| Vector3::from(e.point()))
                    .sum::<Vector3<_>>()
                    / 3.0;

                let normal = (tri_center - corner_center).normalize();

                let tangent = Vector3::from(edges[0].point()).normalize().cross(&normal);
                let cotangent = normal.cross(&tangent);

                edges.sort_by_key(|t| {
                    let p = t.point();
                    let p = Vector3::from(p);
                    let x = p.dot(&tangent);
                    let y = p.dot(&cotangent);
                    FloatOrd(y.atan2(x))
                });

                triangles.push((edges, inds));
            };

        // Prune off free-standing corner triangles
        let mut cell = edges
            .drain(..)
            .filter_map(|t| {
                if t.into_iter().count() == 3 {
                    push_edges(&corners, t.into_iter().collect(), None);
                    None
                } else {
                    Some(t)
                }
            })
            .collect::<Vec<_>>();

        // Categorize point and line edges
        let (points, lines) =
            cell.drain(..)
                .fold((vec![], vec![]), |(mut points, mut lines), next| {
                    match next.into_iter().count() {
                        1 => points.extend(next.into_iter()),
                        2 => lines.push(next),
                        _ => unreachable!(),
                    }

                    (points, lines)
                });

        // Process remainder
        match (points.len(), lines.len()) {
            (0, 0) => vec![],
            (3, 0) => triangle(points),
            (4, 0) => point_quad(points),
            (0, 2) => line_quad(lines),
            (1, 2) => fan(points, lines),
            (3, 1) => inverse_fan(points, lines),
            (0, 3) => hexagon(points, lines, false),
            (2, 2) => hexagon(points, lines, true),
            (6, 0) => double_triangle(points),
            (0, 4) => double_quad(lines),
            (3, 2) => quad_and_triangle(points, lines),
            _ => unreachable!(),
        }
        .into_iter()
        .for_each(|(edges, inds)| push_edges(&corners, edges, inds));

        assert!(triangles.len() > 0);

        triangles
    }
}

#[cfg(test)]
mod test {
    use elysian_core::property_identifier::IntoPropertyIdentifier;
    use elysian_ir::{
        ast::{Struct, POSITION_3D},
        module::{StructIdentifier, CONTEXT},
    };
    use gltf_json::mesh::Mode;
    use nalgebra::Vector3;

    use crate::{
        gltf_export::samples_to_root,
        marching_cells::{Corners, MarchingCell, Point},
    };

    #[test]
    fn test_marching_cubes() {
        let position_3d = POSITION_3D.prop();

        let nodes = (1..=254)
            .into_iter()
            .map(|i| Corners::<3>(i).marching_cell())
            .enumerate()
            .map(|(i, triangles)| {
                let x = (i % 6) as isize * 3;
                let y = ((i / 6) % 6) as isize * 3;
                let z = (i / (6 * 6)) as isize * 3;

                // Synthesize samples for GLTF conversion
                triangles
                    .into_iter()
                    .map(|(edges, indices)| {
                        (
                            Mode::Triangles,
                            [&position_3d],
                            edges
                                .into_iter()
                                .map(|t| {
                                    let v = Vector3::<f64>::from(t.point())
                                        + Vector3::<f64>::new(x as f64, y as f64, z as f64);

                                    Struct::new(StructIdentifier(CONTEXT))
                                        .set(POSITION_3D.into(), [v.x, v.y, v.z].into())
                                })
                                .collect::<Vec<_>>(),
                            indices,
                        )
                    })
                    .collect::<Vec<_>>()
            });

        let root = samples_to_root([nodes]);

        std::fs::write("ser.gltf", root.to_string_pretty().unwrap()).unwrap();

        panic!();
    }
}
