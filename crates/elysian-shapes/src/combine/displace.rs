use std::fmt::Debug;

use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        ast::COMBINE_CONTEXT,
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
            SpecializationData,
        },
    },
};
use elysian_decl_macros::elysian_function;

pub const DISPLACE: FunctionIdentifier = FunctionIdentifier::new("displace", 13382542451638139261);

#[derive(Debug, Clone, Hash)]
pub struct Displace {
    prop: PropertyIdentifier,
}

impl Displace {
    pub fn new(prop: impl Into<PropertyIdentifier>) -> Self {
        Displace { prop: prop.into() }
    }
}

impl Domains for Displace {}

impl AsIR for Displace {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier(DISPLACE.0.concat(&self.prop))
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prop = (*self.prop).clone();

        vec![elysian_function! {
            fn entry_point(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.LEFT.prop + COMBINE_CONTEXT.RIGHT.prop;
                return COMBINE_CONTEXT;
            }
        }]
    }
}
