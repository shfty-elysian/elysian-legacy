use super::{edge::Edge, Cell, Corners, MarchingCell};

impl MarchingCell for Corners<2> {
    type Item = Vec<Edge<2>>;

    fn marching_cell(self) -> Vec<Self::Item> {
        let mut lines = vec![];

        let cell: Vec<_> = self.cell().into_values().collect();

        for line in cell
            .into_iter()
            .flat_map(|t| t.into_iter())
            .collect::<Vec<_>>()
            .chunks(2)
        {
            lines.push(line.into_iter().copied().collect::<Vec<_>>())
        }

        lines
    }
}

#[cfg(test)]
mod test {
    use elysian_core::property_identifier::IntoPropertyIdentifier;
    use elysian_ir::{
        ast::{Struct, POSITION_2D},
        module::{StructIdentifier, CONTEXT},
    };
    use gltf_json::mesh::Mode;
    use nalgebra::Vector2;

    use crate::{
        gltf_export::samples_to_root,
        marching_cells::{Corners, MarchingCell, Point},
    };

    #[test]
    fn test_marching_squares() {
        let position_2d = POSITION_2D.prop();

        let nodes = (1..15)
            .into_iter()
            .map(|i| Corners::<2>(i).marching_cell())
            .enumerate()
            .map(|(i, lines)| {
                let x = (i % 4) as isize * 3;
                let y = (i / 4) as isize * 3;

                // Synthesize samples for GLTF conversion
                lines
                    .into_iter()
                    .map(|edges| {
                        (
                            Mode::Lines,
                            [&position_2d],
                            edges
                                .into_iter()
                                .map(|t| {
                                    let v = Vector2::<f64>::from(t.point())
                                        + Vector2::<f64>::new(x as f64, y as f64);

                                    Struct::new(StructIdentifier(CONTEXT))
                                        .set(POSITION_2D.into(), [v.x, v.y].into())
                                })
                                .collect::<Vec<_>>(),
                            None as Option<Vec<u32>>,
                        )
                    })
                    .collect::<Vec<_>>()
            });

        let root = samples_to_root([nodes]);

        std::fs::write("ser.gltf", root.to_string_pretty().unwrap()).unwrap();

        panic!();
    }
}
