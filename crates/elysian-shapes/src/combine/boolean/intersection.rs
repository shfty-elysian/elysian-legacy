use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{COMBINE_CONTEXT, DISTANCE},
        module::{FunctionDefinition, FunctionIdentifier, InputDefinition, SpecializationData},
    },
};
use elysian_decl_macros::elysian_function;

pub const INTERSECTION: FunctionIdentifier =
    FunctionIdentifier::new("intersection", 18033822391797795038);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intersection;

impl Domains for Intersection {}

impl AsIR for Intersection {
    fn functions_impl(&self, _: &SpecializationData) -> Vec<FunctionDefinition> {
        vec![elysian_function! {
            fn INTERSECTION(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                if COMBINE_CONTEXT.LEFT.DISTANCE > COMBINE_CONTEXT.RIGHT.DISTANCE {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                }
                else {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                }

                return COMBINE_CONTEXT;
            }
        }]
    }

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        elysian_core::ir::ast::Expr::Call {
            function: INTERSECTION,
            args: vec![input],
        }
    }
}
