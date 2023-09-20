use gltf_json::{Root, Mesh};

use super::ExportPrimitive;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExportPrimitives<'a>(Vec<ExportPrimitive<'a>>);

impl<'a> FromIterator<ExportPrimitive<'a>> for ExportPrimitives<'a> {
    fn from_iter<T: IntoIterator<Item = ExportPrimitive<'a>>>(iter: T) -> Self {
        ExportPrimitives(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for ExportPrimitives<'a> {
    type Item = ExportPrimitive<'a>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ExportPrimitives<'_> {
    pub fn to_mesh(self, root: &mut Root) {
        let primitives: Vec<_> = self
            .into_iter()
            .map(|primitive| primitive.to_gltf(root))
            .collect();

        let mesh = Mesh {
            extensions: Default::default(),
            extras: Default::default(),
            primitives,
            weights: None,
        };

        root.meshes.push(mesh);
    }
}

