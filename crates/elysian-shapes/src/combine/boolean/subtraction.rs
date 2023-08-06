use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        ast::{COMBINE_CONTEXT, DISTANCE},
        module::{AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData},
    },
};
use elysian_decl_macros::elysian_function;

pub const SUBTRACTION: FunctionIdentifier =
    FunctionIdentifier::new("subtraction", 1414822549598552032);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subtraction;

impl Domains for Subtraction {}

impl AsIR for Subtraction {
    fn entry_point(&self) -> FunctionIdentifier {
        SUBTRACTION
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        vec![elysian_function! {
            fn entry_point(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                COMBINE_CONTEXT.OUT.DISTANCE =
                    -COMBINE_CONTEXT.OUT.DISTANCE;

                if COMBINE_CONTEXT.LEFT.DISTANCE >= COMBINE_CONTEXT.OUT.DISTANCE {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                }

                return COMBINE_CONTEXT;
            }
        }]
    }
}
