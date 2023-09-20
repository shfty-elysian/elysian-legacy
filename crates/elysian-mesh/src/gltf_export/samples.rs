use std::collections::BTreeMap;

use elysian_ir::ast::{
    Struct, COLOR, NORMAL, POSITION_2D, POSITION_3D, TANGENT_2D, TANGENT_3D, UV, VECTOR2, VECTOR3,
    VECTOR4,
};
use gltf_json::{mesh::Semantic, validation::Checked, Accessor, Index, Root};

use super::{float_buffer, vec2_buffer, vec3_buffer, vec4_buffer, Properties};

iterator_newtype!(Samples, Struct);

impl Samples {
    pub fn to_buffer(
        self,
        root: &mut Root,
        attrs: Properties,
    ) -> BTreeMap<Checked<Semantic>, Index<Accessor>> {
        let samples: Vec<_> = self.into_iter().collect();

        attrs
            .into_iter()
            .map(|attr| {
                let semantic = match &attr {
                    a if ***a == POSITION_2D => Semantic::Positions,
                    a if ***a == POSITION_3D => Semantic::Positions,
                    a if ***a == NORMAL => Semantic::Normals,
                    a if ***a == TANGENT_2D => Semantic::Tangents,
                    a if ***a == TANGENT_3D => Semantic::Tangents,
                    a if ***a == COLOR => Semantic::Colors(0),
                    a if ***a == UV => Semantic::TexCoords(0),
                    _ => unimplemented!(),
                };

                let val = samples[0].get(attr);
                match val {
                    elysian_ir::ast::Value::Number(n) => match n {
                        elysian_core::number::Number::Float(_) => {
                            float_buffer(root, &samples, attr)
                        }
                        _ => unimplemented!(),
                    },
                    elysian_ir::ast::Value::Struct(s) => match &s.id {
                        id if **id == VECTOR2 => {
                            if semantic == Semantic::Positions {
                                vec3_buffer(root, &samples, attr)
                            } else {
                                vec2_buffer(root, &samples, attr)
                            }
                        }
                        id if **id == VECTOR3 => vec3_buffer(root, &samples, attr),
                        id if **id == VECTOR4 => vec4_buffer(root, &samples, attr),
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                };

                (
                    Checked::Valid(semantic),
                    Index::new(root.accessors.len() as u32 - 1),
                )
            })
            .collect()
    }
}
