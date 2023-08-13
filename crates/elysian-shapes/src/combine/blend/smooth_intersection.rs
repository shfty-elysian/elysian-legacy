use std::fmt::Debug;

use crate::combine::{LEFT, OUT, RIGHT};
use elysian_core::{
    expr::{Expr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE, NUM},
    module::{
        AsModule, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition, Module,
        SpecializationData,
    },
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::combine::K;

pub const SMOOTH_INTERSECTION: FunctionIdentifier =
    FunctionIdentifier::new("smooth_intersection", 18033822391797795038);

#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SmoothIntersection {
    prop: PropertyIdentifier,
    k: Expr,
}

impl SmoothIntersection {
    pub fn new(prop: impl Into<PropertyIdentifier>, k: impl IntoExpr) -> Self {
        SmoothIntersection {
            prop: prop.into(),
            k: k.expr(),
        }
    }
}

impl Domains for SmoothIntersection {}

impl AsModule for SmoothIntersection {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
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

        let smooth_intersection = FunctionIdentifier(SMOOTH_INTERSECTION.0.concat(&self.prop));

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: smooth_intersection,
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
            },
        )
        .with_args([self.k.clone().into()])
    }
}
