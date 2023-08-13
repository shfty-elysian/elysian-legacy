use crate::combine::{LEFT, OUT, RIGHT};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE},
    module::{AsModule, Domains, DomainsDyn, FunctionIdentifier, Module, SpecializationData},
};

pub const OVERLAY: FunctionIdentifier = FunctionIdentifier::new("overlay", 566703164678686767);

// Pick a base context from either the left or right side
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Overlay;

impl Domains for Overlay {}

impl AsModule for Overlay {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        Module::new(
            self,
            &spec.filter(self.domains_dyn()),
            elysian_function! {
                fn OVERLAY(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                    if COMBINE_CONTEXT.RIGHT.DISTANCE < 0.0 {
                        COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                    } else {
                        COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                    }
                    return COMBINE_CONTEXT
                }
            },
        )
    }
}
