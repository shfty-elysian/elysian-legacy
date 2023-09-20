use gltf_json::{
    mesh::{Mode, Primitive as GltfPrimitive},
    Index, Root,
};

use super::{index_buffer, Indices, Properties, Samples};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExportPrimitive<'a> {
    pub mode: Mode,
    pub properties: Properties<'a>,
    pub samples: Samples,
    pub indices: Option<Indices>,
}

impl ExportPrimitive<'_> {
    pub fn to_gltf(self, root: &mut Root) -> GltfPrimitive {
        GltfPrimitive {
            attributes: self.samples.to_buffer(root, self.properties),
            extensions: Default::default(),
            extras: Default::default(),
            indices: if let Some(indices) = self.indices {
                index_buffer(root, indices);
                Some(Index::new(root.buffers.len() as u32 - 1))
            } else {
                None
            },
            material: None,
            mode: gltf_json::validation::Checked::Valid(self.mode),
            targets: None,
        }
    }
}
