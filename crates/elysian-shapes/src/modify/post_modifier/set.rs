use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::Modify,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, IntoBlock, GRADIENT_2D, GRADIENT_3D},
        module::{
            AsModule, FunctionDefinition, InputDefinition, IntoRead, IntoWrite, PropertyIdentifier,
            SpecializationData, CONTEXT_PROP,
        },
    },
};

pub const SET: Identifier = Identifier::new("set", 1768232690987692666);

#[derive(Debug, Clone, Hash)]
pub struct Set {
    id: PropertyIdentifier,
    expr: elysian_core::ast::expr::Expr,
}

impl Domains for Set {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![GRADIENT_2D, GRADIENT_3D]
    }
}

impl AsIR for Set {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let block = [
            [CONTEXT_PROP, self.id.clone()].write(self.expr.clone().into()),
            CONTEXT_PROP.read().output(),
        ]
        .block();

        vec![FunctionDefinition {
            id: SET.specialize(spec),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT_PROP,
                mutable: true,
            }],
            output: CONTEXT_PROP,
            block,
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        SET.specialize(spec).call(input)
    }
}

pub trait IntoSet {
    fn set(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify;
}

impl<T> IntoSet for T
where
    T: AsModule,
{
    fn set(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: vec![Box::new(Set { id, expr })],
        }
    }
}
