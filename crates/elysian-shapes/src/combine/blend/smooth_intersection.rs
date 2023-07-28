use std::fmt::Debug;

use elysian_core::{
    ast::{
        combine::{LEFT, OUT, RIGHT},
        expr::Expr,
    },
    ir::{
        as_ir::{AsIR, Domains},
        ast::{COMBINE_CONTEXT, DISTANCE, NUM},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, PropertyIdentifier,
            SpecializationData,
        },
    },
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::combine::K;

pub const SMOOTH_INTERSECTION: FunctionIdentifier =
    FunctionIdentifier::new("smooth_intersection", 18033822391797795038);

#[derive(Debug, Clone, Hash)]
pub struct SmoothIntersection {
    pub prop: PropertyIdentifier,
    pub k: Expr,
}

impl Domains for SmoothIntersection {}

impl AsIR for SmoothIntersection {
    fn functions_impl(&self, _: &SpecializationData) -> Vec<FunctionDefinition> {
        let prop = (*self.prop).clone();

        let mut block = elysian_block! {
            let NUM = (0.5 - 0.5 * (
                COMBINE_CONTEXT.RIGHT.DISTANCE
                    - COMBINE_CONTEXT.LEFT.DISTANCE
            ) / K).max(0.0).min(1.0);
            let prop = COMBINE_CONTEXT.RIGHT.prop.mix(
                COMBINE_CONTEXT.LEFT.prop,
                NUM
            );
        };

        if prop == DISTANCE {
            block.push(elysian_stmt!(
                COMBINE_CONTEXT.OUT.DISTANCE = COMBINE_CONTEXT.OUT.DISTANCE + K * NUM * (1.0 - NUM)
            ));
        }

        block.push(elysian_stmt!(return COMBINE_CONTEXT));

        vec![FunctionDefinition {
            id: FunctionIdentifier(SMOOTH_INTERSECTION.0.concat(&self.prop)),
            public: false,
            inputs: vec![
                InputDefinition {
                    id: K.into(),
                    mutable: false,
                },
                InputDefinition {
                    id: COMBINE_CONTEXT.into(),
                    mutable: true,
                },
            ],
            output: COMBINE_CONTEXT.into(),
            block,
        }]
    }

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        elysian_core::ir::ast::Expr::Call {
            function: FunctionIdentifier(SMOOTH_INTERSECTION.0.concat(&self.prop)),
            args: vec![self.k.clone().into(), input],
        }
    }
}
