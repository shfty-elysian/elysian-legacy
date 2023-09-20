use gltf_json::{Index, Node, Root, Scene};

use super::ExportPrimitives;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Nodes<'a>(Vec<ExportPrimitives<'a>>);

impl<'a> FromIterator<ExportPrimitives<'a>> for Nodes<'a> {
    fn from_iter<T: IntoIterator<Item = ExportPrimitives<'a>>>(iter: T) -> Self {
        Nodes(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for Nodes<'a> {
    type Item = ExportPrimitives<'a>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Nodes<'_> {
    pub fn to_gltf(self, root: &mut Root) {
        let from = root.nodes.len();

        self.into_iter().for_each(|primitives| {
            primitives.to_mesh(root);

            let node = Node {
                camera: None,
                children: None,
                extensions: Default::default(),
                extras: Default::default(),
                matrix: None,
                mesh: Some(Index::new(root.meshes.len() as u32 - 1)),
                rotation: None,
                scale: None,
                translation: None,
                skin: None,
                weights: None,
            };

            root.nodes.push(node);
        });

        root.scenes.push(Scene {
            extensions: Default::default(),
            extras: Default::default(),
            nodes: (from..root.nodes.len())
                .into_iter()
                .map(|i| Index::new(i as u32))
                .collect(),
        })
    }
}
