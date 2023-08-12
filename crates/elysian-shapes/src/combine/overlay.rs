use crate::combine::{LEFT, OUT, RIGHT};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE},
    module::{AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData},
};

pub const OVERLAY: FunctionIdentifier = FunctionIdentifier::new("overlay", 566703164678686767);

// Pick a base context from either the left or right side
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Overlay;

impl Domains for Overlay {}

impl AsIR for Overlay {
    fn entry_point(&self) -> FunctionIdentifier {
        OVERLAY
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        vec![elysian_function! {
            fn entry_point(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                if COMBINE_CONTEXT.RIGHT.DISTANCE < 0.0 {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                } else {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                }
                return COMBINE_CONTEXT
            }
        }]
    }
}
