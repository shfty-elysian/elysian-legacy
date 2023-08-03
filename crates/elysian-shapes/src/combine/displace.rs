use std::fmt::Debug;

use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        as_ir::{AsIR, Domains},
        ast::COMBINE_CONTEXT,
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, PropertyIdentifier,
            SpecializationData,
        },
    },
};

use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const DISPLACE: FunctionIdentifier =
    FunctionIdentifier::new("displace", 13382542451638139261);

#[derive(Debug, Clone, Hash)]
pub struct Displace {
    pub prop: PropertyIdentifier,
}

impl Domains for Displace {}

impl AsIR for Displace {
    fn functions_impl(&self, _: &SpecializationData) -> Vec<FunctionDefinition> {
        let prop = (*self.prop).clone();

        let mut block = elysian_block! {
            COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.LEFT.prop + COMBINE_CONTEXT.RIGHT.prop;
        };

        block.push(elysian_stmt!(return COMBINE_CONTEXT));

        vec![FunctionDefinition {
            id: FunctionIdentifier(DISPLACE.0.concat(&self.prop)),
            public: false,
            inputs: vec![InputDefinition {
                id: COMBINE_CONTEXT.into(),
                mutable: true,
            }],
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
            function: FunctionIdentifier(DISPLACE.0.concat(&self.prop)),
            args: vec![input],
        }
    }
}

