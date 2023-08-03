use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        ast::{Block, DISTANCE, GRADIENT_2D, GRADIENT_3D, NUM, UV, X},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition,
            PropertyIdentifier, SpecializationData, CONTEXT,
        },
    },
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const MANIFOLD: FunctionIdentifier = FunctionIdentifier::new("manifold", 7861274791729269697);

#[derive(Debug, Clone)]
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

impl AsIR for Manifold {
    fn functions_impl(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
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

        vec![FunctionDefinition {
            id: entry_point.clone(),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT.into(),
                mutable: true,
            }],
            output: CONTEXT.into(),
            block,
        }]
    }

    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        MANIFOLD.specialize(spec)
    }
}

pub trait IntoManifold {
    fn manifold(self) -> Modify;
}

impl IntoManifold for Field {
    fn manifold(self) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: vec![Box::new(Manifold)],
        }
    }
}

impl IntoManifold for Modify {
    fn manifold(mut self) -> Modify {
        self.post_modifiers.push(Box::new(Manifold));
        self
    }
}
