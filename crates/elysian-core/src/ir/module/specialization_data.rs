use std::collections::BTreeSet;

use crate::ir::ast::{Property, DISTANCE, GRADIENT_2D, POSITION_2D};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecializationData {
    pub domains: BTreeSet<Property>,
}

impl Default for SpecializationData {
    fn default() -> Self {
        Self {
            domains: [POSITION_2D, DISTANCE, GRADIENT_2D].into(),
        }
    }
}
