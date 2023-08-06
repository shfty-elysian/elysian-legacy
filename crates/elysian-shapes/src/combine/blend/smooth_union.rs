use std::fmt::Debug;

use elysian_core::{
    ast::{
        combine::{LEFT, OUT, RIGHT},
        expr::Expr,
    },
    ir::{
        ast::{COMBINE_CONTEXT, DISTANCE, NUM},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition,
            PropertyIdentifier, SpecializationData,
        },
    },
};

use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::combine::K;
pub const SMOOTH_UNION: FunctionIdentifier =
    FunctionIdentifier::new("smooth_union", 1894363406191409858);

#[derive(Debug, Clone, Hash)]
pub struct SmoothUnion {
    pub prop: PropertyIdentifier,
    pub k: Expr,
}

impl Domains for SmoothUnion {}

impl AsIR for SmoothUnion {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier(SMOOTH_UNION.0.concat(&self.prop)).specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.k.clone().into(), input]
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prop = (*self.prop).clone();

        let mut block = elysian_block! {
            let NUM = (
                0.5 + 0.5 * (
                    COMBINE_CONTEXT.RIGHT.DISTANCE
                        - COMBINE_CONTEXT.LEFT.DISTANCE
                ) / K
            ).max(0.0).min(1.0);

            COMBINE_CONTEXT.OUT.prop =
                COMBINE_CONTEXT.RIGHT.prop.mix(
                    COMBINE_CONTEXT.LEFT.prop,
                    NUM
                );
        };

        if prop == DISTANCE {
            block.push(elysian_stmt!(
                COMBINE_CONTEXT.OUT.DISTANCE = COMBINE_CONTEXT.OUT.DISTANCE - K * NUM * (1.0 - NUM)
            ));
        }

        block.push(elysian_stmt!(return COMBINE_CONTEXT));

        vec![FunctionDefinition {
            id: entry_point.clone(),
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
}
