use super::{Combinator, Combine};

#[derive(Debug)]
pub struct CombineBuilder(Vec<Box<dyn Combinator>>);

impl CombineBuilder {
    pub fn build() -> Self {
        CombineBuilder(Default::default())
    }

    pub fn push(mut self, combinator: impl 'static + Combinator) -> Self {
        self.0.push(Box::new(combinator) as Box<dyn Combinator>);
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
    type Item = Box<dyn Combinator>;

    type IntoIter = std::vec::IntoIter<Box<dyn Combinator>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
