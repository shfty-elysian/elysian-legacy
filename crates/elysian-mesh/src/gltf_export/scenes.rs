use gltf_json::Root;

use super::Nodes;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Scenes<'a>(Vec<Nodes<'a>>);

impl<'a> FromIterator<Nodes<'a>> for Scenes<'a> {
    fn from_iter<T: IntoIterator<Item = Nodes<'a>>>(iter: T) -> Self {
        Scenes(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for Scenes<'a> {
    type Item = Nodes<'a>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Scenes<'_>> for Root {
    fn from(value: Scenes<'_>) -> Self {
        let mut root = Root::default();
        value.into_iter().for_each(|scene| scene.to_gltf(&mut root));
        root
    }
}
