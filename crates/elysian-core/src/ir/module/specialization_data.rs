use std::collections::BTreeSet;

use crate::ir::ast::{
    Identifier, Property, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D,
};

use super::StructDefinition;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecializationData {
    domains: BTreeSet<Identifier>,
    context_struct: Option<StructDefinition>,
    context: Option<Property>,
}

impl SpecializationData {
    pub fn new_2d() -> Self {
        Self {
            domains: [POSITION_2D.id(), DISTANCE.id(), GRADIENT_2D.id()]
                .into_iter()
                .cloned()
                .collect(),
            ..Default::default()
        }
    }

    pub fn new_3d() -> Self {
        Self {
            domains: [POSITION_3D.id(), DISTANCE.id(), GRADIENT_3D.id()]
                .into_iter()
                .cloned()
                .collect(),
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
            context_struct: self.context_struct.clone(),
            context: self.context.clone(),
        }
    }

    pub fn specialize_id(&self, id: Identifier) -> Identifier {
        self.domains.iter().fold(id, |acc, next| acc.concat(next))
    }

    pub fn context_struct(&self) -> &StructDefinition {
        self.context_struct.as_ref().expect("No Context Struct")
    }

    pub fn context(&self) -> &Property {
        self.context.as_ref().expect("No Context Struct")
    }
}
