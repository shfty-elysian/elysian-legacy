use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify};
use elysian_core::{
    ast::expr::IntoExpr,
    ir::{
        ast::{Expr, Identifier},
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
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier(Identifier::new_dynamic("set").concat(&self.id))
    }

    fn functions(
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
    fn set_pre(self, id: impl Into<PropertyIdentifier>, expr: impl IntoExpr) -> Modify;
    fn set_post(self, id: impl Into<PropertyIdentifier>, expr: impl IntoExpr) -> Modify;
}

impl<T> IntoSet for T
where
    T: 'static + AsIR,
{
    fn set_pre(self, id: impl Into<PropertyIdentifier>, expr: impl IntoExpr) -> Modify {
        self.modify().push_pre(Set {
            id: id.into(),
            expr: expr.expr(),
        })
    }

    fn set_post(self, id: impl Into<PropertyIdentifier>, expr: impl IntoExpr) -> Modify {
        self.modify().push_post(Set {
            id: id.into(),
            expr: expr.expr(),
        })
    }
}
