use std::collections::BTreeSet;

use crate::ir::ast::{Identifier, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecializationData {
    domains: BTreeSet<Identifier>,
}

impl SpecializationData {
    pub fn new_2d() -> Self {
        Self {
            domains: [POSITION_2D, DISTANCE, GRADIENT_2D].into_iter().collect(),
            ..Default::default()
        }
    }

    pub fn new_3d() -> Self {
        Self {
            domains: [POSITION_3D, DISTANCE, GRADIENT_3D].into_iter().collect(),
            ..Default::default()
        }
    }

    pub fn contains(&self, prop: &Identifier) -> bool {
        self.domains.contains(prop)
    }

    pub fn filter<I: IntoIterator<Item = Identifier>>(&self, props: I) -> Self {
        let props: BTreeSet<_> = props.into_iter().collect();
        SpecializationData {
            domains: self
                .domains
                .iter()
                .cloned()
                .filter(|t| props.contains(t))
                .collect(),
        }
    }

    pub fn specialize_id(&self, id: Identifier) -> Identifier {
        self.domains.iter().fold(id, |acc, next| acc.concat(next))
    }
}
