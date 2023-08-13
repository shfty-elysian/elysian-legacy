use crate::combine::{Combinator, LEFT, OUT, RIGHT};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData},
};

pub const SUBTRACTION: FunctionIdentifier =
    FunctionIdentifier::new("subtraction", 1414822549598552032);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Subtraction;

impl Domains for Subtraction {}

impl AsModule for Subtraction {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        Module::new(
            self,
            spec,
            elysian_function! {
                fn SUBTRACTION(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                    COMBINE_CONTEXT.OUT.DISTANCE =
                        -COMBINE_CONTEXT.OUT.DISTANCE;

                    if COMBINE_CONTEXT.LEFT.DISTANCE >= COMBINE_CONTEXT.OUT.DISTANCE {
                        COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                    }

                    return COMBINE_CONTEXT;
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Combinator for Subtraction {}
