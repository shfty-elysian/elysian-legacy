use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::Modify,
    ir::{
        ast::Expr,
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
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

impl Domains for Set {}

impl AsIR for Set {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier((*SET).concat(&(*self.id))).specialize(spec)
    }

    fn functions_impl(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prop = self.id.clone();
        let expr = Expr::from(self.expr.clone());

        vec![elysian_function! {
            fn entry_point(mut CONTEXT) -> CONTEXT {
                CONTEXT.prop = #expr;
                return CONTEXT
            }
        }]
    }
}

pub trait IntoSet {
    fn set_pre(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify;
    fn set_post(self, id: PropertyIdentifier, expr: elysian_core::ast::expr::Expr) -> Modify;
}

impl<T> IntoSet for T
where
    T: 'static + AsIR,
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
