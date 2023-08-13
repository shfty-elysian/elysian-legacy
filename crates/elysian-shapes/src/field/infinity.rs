use std::hash::Hash;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{IntoLiteral, DISTANCE},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData, CONTEXT},
};

use elysian_decl_macros::elysian_function;

use crate::shape::Shape;

pub const INFINITY: FunctionIdentifier = FunctionIdentifier::new("infinity", 349698827217118514);

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl AsModule for Infinity {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        assert!(
            spec.contains(&DISTANCE.into()),
            "Infinity requires the Distance domain"
        );

        let infinity = f32::MAX.literal();

        Module::new(
            self,
            spec,
            elysian_function! {
                fn INFINITY(mut CONTEXT) -> CONTEXT {
                    CONTEXT.DISTANCE = #infinity;
                    return CONTEXT;
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Infinity {}
