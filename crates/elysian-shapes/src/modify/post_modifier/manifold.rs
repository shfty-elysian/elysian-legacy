use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{DISTANCE, GRADIENT_2D, GRADIENT_3D, NUM},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, PropertyIdentifier,
            SpecializationData, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;

pub const MANIFOLD: FunctionIdentifier = FunctionIdentifier::new("manifold", 7861274791729269697);

#[derive(Debug, Clone, Hash)]
pub struct Manifold;

impl Domains for Manifold {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![GRADIENT_2D.into(), GRADIENT_3D.into()]
    }
}

impl AsIR for Manifold {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let manifold = MANIFOLD.specialize(spec);

        let gradient = if spec.contains(&GRADIENT_2D.into()) {
            GRADIENT_2D
        } else if spec.contains(&GRADIENT_3D.into()) {
            GRADIENT_3D
        } else {
            return vec![elysian_function! {
                fn manifold(CONTEXT) -> CONTEXT {
                    return CONTEXT
                }
            }];
        };

        vec![elysian_function! {
            fn manifold(mut CONTEXT) -> CONTEXT {
                let NUM = CONTEXT.DISTANCE;
                CONTEXT.DISTANCE = NUM.abs();
                CONTEXT.gradient = CONTEXT.gradient * NUM.sign();
                return CONTEXT;
            }
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
