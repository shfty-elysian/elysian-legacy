use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PostModifier};
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{Block, DISTANCE, GRADIENT_2D, GRADIENT_3D, NUM, UV, X},
    module::{
        AsModule, Domains, FunctionDefinition, FunctionIdentifier,
        InputDefinition, Module, SpecializationData, CONTEXT,
    },
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const MANIFOLD: FunctionIdentifier = FunctionIdentifier::new("manifold", 7861274791729269697);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Manifold;

impl Hash for Manifold {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        MANIFOLD.uuid().hash(state);
    }
}

impl Domains for Manifold {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![GRADIENT_2D.into(), GRADIENT_3D.into(), UV.into()]
    }
}

impl AsModule for Manifold {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let mut block = Block::default();

        block.extend(elysian_block! {
            let NUM = CONTEXT.DISTANCE;
            CONTEXT.DISTANCE = NUM.abs();
        });

        let gradient = if spec.contains(&GRADIENT_2D.into()) {
            Some(GRADIENT_2D)
        } else if spec.contains(&GRADIENT_3D.into()) {
            Some(GRADIENT_3D)
        } else {
            None
        };

        if let Some(gradient) = gradient {
            block.push(elysian_stmt! {
                CONTEXT.gradient = CONTEXT.gradient * NUM.sign()
            })
        };

        if spec.contains(&UV.into()) {
            block.push(elysian_stmt! {
                CONTEXT.UV.X = CONTEXT.UV.X * NUM.sign()
            })
        }

        block.push(elysian_stmt! { return CONTEXT });

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: MANIFOLD,
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                }],
                output: CONTEXT.into(),
                block,
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serialize)]
impl PostModifier for Manifold {}

pub trait IntoManifold {
    fn manifold(self) -> Modify;
}

impl<T> IntoManifold for T
where
    T: IntoModify,
{
    fn manifold(self) -> Modify {
        self.modify().push_post(Manifold)
    }
}
