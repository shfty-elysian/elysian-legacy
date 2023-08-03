use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        ast::{Block, COMBINE_CONTEXT, DISTANCE},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition,
            SpecializationData,
        },
    },
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const UNION: FunctionIdentifier = FunctionIdentifier::new("union", 1894363406191409858);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Union;

impl Domains for Union {}

impl AsIR for Union {
    fn functions_impl(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let mut block = Block::default();

        block.extend(elysian_block! {
            if COMBINE_CONTEXT.LEFT.DISTANCE < COMBINE_CONTEXT.RIGHT.DISTANCE {
                COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
            }
            else {
                COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
            }
        });

        block.push(elysian_stmt! {
            return COMBINE_CONTEXT
        });

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
        UNION
    }
}
