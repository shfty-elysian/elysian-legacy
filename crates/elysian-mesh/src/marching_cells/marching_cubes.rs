use std::cmp::Ordering;

use nalgebra::Vector3;

use crate::{
    marching_cells::{Cell, Corner, Point},
    util::CollectArray,
    vector_space::D3,
};

use super::{edge::Edge, Corners, Edges, MarchPrimitive, MarchPrimitives, ToMarchPrimitives};

/// Comparator function for finding the closest edge position to a given point
fn distance_to(p: impl Into<Vector3<f64>>) -> impl Fn(&Edge<D3>, &Edge<D3>) -> Ordering {
    let p = p.into();
    move |lhs: &Edge<D3>, rhs: &Edge<D3>| {
        (Vector3::from(lhs.point()) - p)
            .magnitude()
            .partial_cmp(&(Vector3::from(rhs.point()) - p).magnitude())
            .unwrap()
    }
}

pub const QUAD_INDICES: [u32; 6] = [0, 1, 3, 3, 1, 2];

fn hexagon(
    points: Vec<Edge<D3>>,
    mut lines: Vec<Edges<D3>>,
    folded: bool,
) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
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

fn double_quad(lines: Vec<Edges<D3>>) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
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

fn fan(points: Vec<Edge<D3>>, lines: Vec<Edges<D3>>) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
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
    points: Vec<Edge<D3>>,
    lines: Vec<Edges<D3>>,
) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
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
    points: Vec<Edge<D3>>,
    lines: Vec<Edges<D3>>,
) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
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

fn double_triangle(mut points: Vec<Edge<D3>>) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
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

fn triangle(edges: Vec<Edge<D3>>) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
    vec![((edges, None))]
}

fn point_quad(points: Vec<Edge<D3>>) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
    vec![((points.into_iter().collect(), Some(QUAD_INDICES.to_vec())))]
}

fn line_quad(lines: Vec<Edges<D3>>) -> Vec<(Vec<Edge<D3>>, Option<Vec<u32>>)> {
    vec![
        ((
            lines.into_iter().flat_map(|t| t.into_iter()).collect(),
            Some(QUAD_INDICES.to_vec()),
        )),
    ]
}

impl ToMarchPrimitives<D3> for Corners<D3> {
    fn march_primitives(self) -> MarchPrimitives<D3> {
        let (corners, mut edges): (Vec<_>, Vec<_>) = self.cell().into_iter().unzip();

        let mut triangles = vec![];
        let mut push_edges =
            |corners: &[Corner<D3>], mut edges: Vec<Edge<D3>>, inds: Option<Vec<u32>>| {
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

                triangles.push(MarchPrimitive::new(edges, inds));
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

        MarchPrimitives::from_iter(triangles)
    }
}

#[cfg(test)]
mod test {
    use elysian_core::property_identifier::IntoPropertyIdentifier;
    use elysian_ir::{
        ast::{Struct, POSITION_3D},
        module::{StructIdentifier, CONTEXT},
    };
    use gltf_json::{mesh::Mode, Root};
    use nalgebra::Vector3;

    use crate::{
        gltf_export::{ExportPrimitive, Indices, Nodes, Samples, Scenes},
        marching_cells::{Corners, Point, ToMarchPrimitives}, vector_space::D3,
    };

    #[test]
    fn test_marching_cubes() {
        let position_3d = POSITION_3D.prop();

        let nodes = (1..=254)
            .into_iter()
            .map(|i| Corners::<D3>::new(i).march_primitives())
            .enumerate()
            .map(|(i, triangles)| {
                let x = (i % 6) as isize * 3;
                let y = ((i / 6) % 6) as isize * 3;
                let z = (i / (6 * 6)) as isize * 3;

                // Synthesize samples for GLTF conversion
                triangles
                    .into_iter()
                    .map(|marching_cell| ExportPrimitive {
                        mode: Mode::Triangles,
                        properties: [&position_3d].into_iter().collect(),
                        samples: marching_cell
                            .edges
                            .into_iter()
                            .map(|t| {
                                let v = Vector3::<f64>::from(t.point())
                                    + Vector3::<f64>::new(x as f64, y as f64, z as f64);

                                Struct::new(StructIdentifier(CONTEXT))
                                    .set(POSITION_3D.into(), [v.x, v.y, v.z].into())
                            })
                            .collect::<Samples>(),
                        indices: marching_cell
                            .indices
                            .map(|indices| indices.into_iter().collect::<Indices>()),
                    })
                    .collect()
            })
            .collect::<Nodes>();

        let root = Root::from([nodes].into_iter().collect::<Scenes>());

        std::fs::write("ser.gltf", root.to_string_pretty().unwrap()).unwrap();

        panic!();
    }
}
