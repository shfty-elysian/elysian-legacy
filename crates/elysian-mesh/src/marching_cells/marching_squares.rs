use crate::vector_space::D2;

use super::{Cell, Corners, MarchPrimitive, MarchPrimitives, ToMarchPrimitives};

impl ToMarchPrimitives<D2> for Corners<D2> {
    fn march_primitives(self) -> MarchPrimitives<D2> {
        let mut lines = vec![];

        let cell: Vec<_> = self.cell().into_values().collect();

        for line in cell
            .into_iter()
            .flat_map(|t| t.into_iter())
            .collect::<Vec<_>>()
            .chunks(2)
        {
            lines.push(MarchPrimitive {
                edges: line.into_iter().copied().collect(),
                indices: None,
            })
        }

        MarchPrimitives::from_iter(lines)
    }
}

#[cfg(test)]
mod test {
    use elysian_core::property_identifier::IntoPropertyIdentifier;
    use elysian_ir::{
        ast::{Struct, POSITION_2D},
        module::{StructIdentifier, CONTEXT},
    };
    use gltf_json::{mesh::Mode, Root};
    use nalgebra::Vector2;

    use crate::{
        gltf_export::{ExportPrimitive, Indices, Properties, Samples, Scenes},
        marching_cells::{Corners, Point, ToMarchPrimitives},
        vector_space::D2,
    };

    #[test]
    fn test_marching_squares() {
        let position_2d = POSITION_2D.prop();

        let nodes = (1..15)
            .into_iter()
            .map(|i| Corners::<D2>::new(i).march_primitives())
            .enumerate()
            .map(|(i, lines)| {
                let x = (i % 4) as isize * 3;
                let y = (i / 4) as isize * 3;

                // Synthesize samples for GLTF conversion
                lines
                    .into_iter()
                    .map(|marching_cell| ExportPrimitive {
                        mode: Mode::Lines,
                        properties: [&position_2d].into_iter().collect::<Properties>(),
                        samples: marching_cell
                            .edges
                            .into_iter()
                            .map(|t| {
                                let v = Vector2::<f64>::from(t.point())
                                    + Vector2::<f64>::new(x as f64, y as f64);

                                Struct::new(StructIdentifier(CONTEXT))
                                    .set(POSITION_2D.into(), [v.x, v.y].into())
                            })
                            .collect::<Samples>(),
                        indices: marching_cell
                            .indices
                            .map(|indices| indices.into_iter().collect::<Indices>()),
                    })
                    .collect()
            })
            .collect();

        let root = Root::from([nodes].into_iter().collect::<Scenes>());

        std::fs::write("ser.gltf", root.to_string_pretty().unwrap()).unwrap();

        panic!();
    }
}
