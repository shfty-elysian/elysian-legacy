use std::{fmt::Debug, hash::Hash};

use crate::{
    modify::{IntoModify, Modify, PostModifier, PreModifier},
    shape::Shape,
};
use elysian_core::{
    expr::IntoExpr, identifier::Identifier, property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::Expr,
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData, CONTEXT},
};

pub const SET: FunctionIdentifier = FunctionIdentifier::new("set", 1768232690987692666);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Set {
    id: PropertyIdentifier,
    expr: elysian_core::expr::Expr,
}

impl Hash for Set {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        SET.uuid().hash(state);
        self.id.hash(state);
        self.expr.hash(state);
    }
}

impl Domains for Set {}

impl AsModule for Set {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let prop = self.id.clone();
        let expr = Expr::from(self.expr.clone());

        let set = FunctionIdentifier(Identifier::new_dynamic("set").concat(&self.id));

        Module::new(
            self,
            spec,
            elysian_function! {
                fn set(mut CONTEXT) -> CONTEXT {
                    CONTEXT.prop = #expr;
                    return CONTEXT
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl PreModifier for Set {}

#[cfg_attr(feature = "serde", typetag::serde)]
impl PostModifier for Set {}

pub trait IntoSet {
    fn set_pre(self, id: impl Into<PropertyIdentifier>, expr: impl IntoExpr) -> Modify;
    fn set_post(self, id: impl Into<PropertyIdentifier>, expr: impl IntoExpr) -> Modify;
}

impl<T> IntoSet for T
where
    T: 'static + Shape,
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
