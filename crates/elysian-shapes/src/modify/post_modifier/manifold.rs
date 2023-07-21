use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{
        field::Field,
        modify::{Modify, CONTEXT_STRUCT},
    },
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Identifier, IntoBlock, IntoRead, IntoWrite, CONTEXT, DISTANCE, GRADIENT_2D,
            GRADIENT_3D, NUM,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

pub const MANIFOLD: Identifier = Identifier::new("manifold", 7861274791729269697);

#[derive(Debug, Clone, Hash)]
pub struct Manifold;

impl Domains for Manifold {
    fn domains() -> Vec<Identifier> {
        vec![GRADIENT_2D.id().clone(), GRADIENT_3D.id().clone()]
    }
}

impl AsIR for Manifold {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let gradient = if spec.contains(GRADIENT_2D.id()) {
            GRADIENT_2D
        } else if spec.contains(GRADIENT_3D.id()) {
            GRADIENT_3D
        } else {
            return vec![FunctionDefinition {
                id: MANIFOLD.specialize(spec),
                public: false,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                }],
                output: &CONTEXT_STRUCT,
                block: [CONTEXT.read().output()].block(),
            }];
        };

        vec![FunctionDefinition {
            id: MANIFOLD.specialize(spec),
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: [
                NUM.bind([CONTEXT, DISTANCE].read()),
                [CONTEXT, DISTANCE].write(NUM.read().abs()),
                [CONTEXT, gradient.clone()].write([CONTEXT, gradient].read() * NUM.read().sign()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        MANIFOLD.specialize(spec).call(input)
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
