use crate::combine::{LEFT, OUT, RIGHT};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData},
};

pub const INTERSECTION: FunctionIdentifier =
    FunctionIdentifier::new("intersection", 18033822391797795038);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Intersection;

impl Domains for Intersection {}

impl AsModule for Intersection {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        Module::new(
            self,
            spec,
            elysian_function! {
                fn INTERSECTION(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                    if COMBINE_CONTEXT.LEFT.DISTANCE >= COMBINE_CONTEXT.RIGHT.DISTANCE {
                        COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                    }
                    else {
                        COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                    }

                    return COMBINE_CONTEXT;
                }
            },
        )
    }
}
