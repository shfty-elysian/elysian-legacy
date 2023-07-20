use std::{fmt::Debug, hash::Hash};

use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{
            Identifier, IntoBind, IntoBlock, IntoRead, CONTEXT, DISTANCE, GRADIENT_2D, GRADIENT_3D,
            NUM,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

pub const MANIFOLD: Identifier = Identifier::new("manifold", 7861274791729269697);

#[derive(Debug, Clone, Hash)]
pub struct Manifold;

impl FilterSpec for Manifold {
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        spec.filter([GRADIENT_2D.id(), GRADIENT_3D.id()])
    }
}

impl AsIR for Manifold {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let gradient = if spec.contains(GRADIENT_2D.id()) {
            GRADIENT_2D
        } else if spec.contains(GRADIENT_3D.id()) {
            GRADIENT_3D
        } else {
            panic!("No gradient domain")
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
                [CONTEXT, DISTANCE].bind(NUM.read().abs()),
                [CONTEXT, gradient.clone()].bind([CONTEXT, gradient].read() * NUM.read().sign()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: crate::ir::ast::Expr,
    ) -> crate::ir::ast::Expr {
        MANIFOLD.specialize(spec).call(input)
    }
}
