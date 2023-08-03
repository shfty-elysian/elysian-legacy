use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        ast::{COMBINE_CONTEXT, DISTANCE},
        module::{AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData},
    },
};
use elysian_decl_macros::elysian_function;

pub const UNION: FunctionIdentifier = FunctionIdentifier::new("union", 1894363406191409858);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Union;

impl Domains for Union {}

impl AsIR for Union {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        UNION
    }

    fn functions_impl(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        vec![elysian_function! {
            fn entry_point(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                if COMBINE_CONTEXT.LEFT.DISTANCE < COMBINE_CONTEXT.RIGHT.DISTANCE {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                }
                else {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                }

                return COMBINE_CONTEXT;
            }
        }]
    }
}
