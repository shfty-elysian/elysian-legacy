use std::fmt::Debug;

use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        ast::COMBINE_CONTEXT,
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition,
            PropertyIdentifier, SpecializationData,
        },
    },
};

use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const DISPLACE: FunctionIdentifier = FunctionIdentifier::new("displace", 13382542451638139261);

#[derive(Debug, Clone, Hash)]
pub struct Displace {
    pub prop: PropertyIdentifier,
}

impl Domains for Displace {}

impl AsIR for Displace {
    fn functions_impl(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prop = (*self.prop).clone();

        let mut block = elysian_block! {
            COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.LEFT.prop + COMBINE_CONTEXT.RIGHT.prop;
        };

        block.push(elysian_stmt!(return COMBINE_CONTEXT));

        vec![FunctionDefinition {
            id: entry_point.clone(),
            public: false,
            inputs: vec![InputDefinition {
                id: COMBINE_CONTEXT.into(),
                mutable: true,
            }],
            output: COMBINE_CONTEXT.into(),
            block,
        }]
    }

    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier(DISPLACE.0.concat(&self.prop))
    }
}
