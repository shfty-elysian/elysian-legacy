use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{COMBINE_CONTEXT, DISTANCE},
        module::{FunctionDefinition, FunctionIdentifier, SpecializationData},
    },
};
use elysian_decl_macros::elysian_function;

pub const SUBTRACTION: FunctionIdentifier =
    FunctionIdentifier::new("subtraction", 1414822549598552032);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subtraction;

impl Domains for Subtraction {}

impl AsIR for Subtraction {
    fn functions_impl(&self, _: &SpecializationData) -> Vec<FunctionDefinition> {
        vec![elysian_function! {
            fn SUBTRACTION(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
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

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        elysian_core::ir::ast::Expr::Call {
            function: SUBTRACTION,
            args: vec![input],
        }
    }
}
