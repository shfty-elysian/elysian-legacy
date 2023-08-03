use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        ast::{COMBINE_CONTEXT, DISTANCE},
        module::{AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData},
    },
};
use elysian_decl_macros::elysian_function;

pub const INTERSECTION: FunctionIdentifier =
    FunctionIdentifier::new("intersection", 18033822391797795038);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intersection;

impl Domains for Intersection {}

impl AsIR for Intersection {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        INTERSECTION
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        vec![elysian_function! {
            fn entry_point(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                if COMBINE_CONTEXT.LEFT.DISTANCE >= COMBINE_CONTEXT.RIGHT.DISTANCE {
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
