use std::sync::OnceLock;

use elysian_core::{property_identifier::PropertyIdentifier, uuid::Uuid};
use indexmap::IndexMap;

use crate::ast::Property;

use super::Type;

#[macro_export]
macro_rules! property {
    ($id:ident, $prop:ident, $ty:expr) => {
        #[linkme::distributed_slice($crate::module::PROPERTIES)]
        static $prop: $crate::ast::Property = $crate::ast::Property {
            id: elysian_core::property_identifier::PropertyIdentifier($id),
            ty: $ty,
        };
    };
}

/// Distributed slice of Identifier -> Type pairs
#[linkme::distributed_slice]
pub static PROPERTIES: [Property] = [..];

pub static PROPERTIES_MAP: OnceLock<IndexMap<PropertyIdentifier, Type>> = OnceLock::new();

pub fn properties() -> &'static IndexMap<PropertyIdentifier, Type> {
    PROPERTIES_MAP.get_or_init(|| {
        let props: IndexMap<_, _> = PROPERTIES
            .into_iter()
            .map(|prop| (prop.id.clone(), prop.ty.clone()))
            .collect();

        for (i, (id, _)) in props.iter().enumerate() {
            if let Some((_, (cand, _))) =
                props
                    .iter()
                    .enumerate()
                    .filter(|(u, _)| i != *u)
                    .find(|(_, (cand, _))| {
                        let id = id.uuid();
                        let cand = cand.uuid();
                        let nil = Uuid::nil();
                        if *id == nil && *cand == nil {
                            return false;
                        }

                        id == cand
                    })
            {
                panic!(
                    "Properties: UUID Collision between {} and {}",
                    cand.name(),
                    id.name()
                )
            }
        }

        props
    })
}
