use std::hash::Hash;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{IntoLiteral, DISTANCE},
    module::{AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData, CONTEXT},
};

use elysian_decl_macros::elysian_function;

pub const INFINITY: FunctionIdentifier = FunctionIdentifier::new("infinity", 349698827217118514);

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Infinity;

impl Hash for Infinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        INFINITY.uuid().hash(state);
    }
}

impl Domains for Infinity {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![DISTANCE.into()]
    }
}

impl AsIR for Infinity {
    fn entry_point(&self) -> FunctionIdentifier {
        INFINITY
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        assert!(
            spec.contains(&DISTANCE.into()),
            "Infinity requires the Distance domain"
        );

        let infinity = f32::MAX.literal();

        vec![elysian_function! {
            fn entry_point(mut CONTEXT) -> CONTEXT {
                CONTEXT.DISTANCE = #infinity;
                return CONTEXT;
            }
        }]
    }
}
