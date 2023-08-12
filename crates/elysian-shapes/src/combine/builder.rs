use crate::shape::{DynShape, IntoShape};

use super::Combine;

#[derive(Debug)]
pub struct CombineBuilder(Vec<DynShape>);

impl CombineBuilder {
    pub fn build() -> Self {
        CombineBuilder(Default::default())
    }

    pub fn push(mut self, combinator: impl IntoShape) -> Self {
        self.0.push(combinator.shape());
        self
    }

    pub fn combine(self) -> Combine {
        Combine {
            combinator: self.into_iter().collect(),
            shapes: Default::default(),
        }
    }
}

impl IntoIterator for CombineBuilder {
    type Item = DynShape;

    type IntoIter = std::vec::IntoIter<DynShape>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
