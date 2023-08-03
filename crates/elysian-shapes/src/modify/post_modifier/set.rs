use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::Modify,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Expr, GRADIENT_2D, GRADIENT_3D},
        module::{
            AsModule, FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
            SpecializationData, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;

pub const SET: FunctionIdentifier = FunctionIdentifier::new("set", 1768232690987692666);

#[derive(Debug, Clone)]
pub struct Set {
    id: PropertyIdentifier,
    expr: elysian_core::ast::expr::Expr,
}

impl Hash for Set {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        SET.uuid().hash(state);
        self.id.hash(state);
        self.expr.hash(state);
    }
}

impl Domains for Set {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![]
    }
}

impl AsIR for Set {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let prop = self.id.clone();
        let expr = Expr::from(self.expr.clone());

        let set = FunctionIdentifier((*SET).concat(&(*self.id))).specialize(spec);

        vec![elysian_function! {
            fn set(mut CONTEXT) -> CONTEXT {
                CONTEXT.prop = #expr;
                return CONTEXT
            }
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        FunctionIdentifier((*SET).concat(&(*self.id)))
            .specialize(spec)
            .call(input)
    }
}

pub trait IntoSet {
    fn set_pre(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify;
    fn set_post(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify;
}

impl<T> IntoSet for T
where
    T: AsModule,
{
    fn set_pre(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify {
        Modify {
            pre_modifiers: vec![Box::new(Set { id, expr })],
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }

    fn set_post(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: vec![Box::new(Set { id, expr })],
        }
    }
}
