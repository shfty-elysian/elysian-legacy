use std::fmt::Debug;

use crate::combine::{LEFT, OUT, RIGHT};
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::COMBINE_CONTEXT,
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData},
};

pub const DISPLACE: FunctionIdentifier = FunctionIdentifier::new("displace", 13382542451638139261);

#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Displace {
    prop: PropertyIdentifier,
}

impl Displace {
    pub fn new(prop: impl Into<PropertyIdentifier>) -> Self {
        Displace { prop: prop.into() }
    }
}

impl Domains for Displace {}

impl AsModule for Displace {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let displace = FunctionIdentifier(DISPLACE.0.concat(&self.prop));
        let prop = (*self.prop).clone();

        Module::new(
            self,
            spec,
            elysian_function! {
                fn displace(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                    COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.LEFT.prop + COMBINE_CONTEXT.RIGHT.prop;
                    return COMBINE_CONTEXT;
                }
            },
        )
    }
}
