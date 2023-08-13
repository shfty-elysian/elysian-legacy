use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PreModifier};
use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, VECTOR2, X, Y},
    module::{
        AsModule, Domains, IntoFunctionIdentifier, Module, NumericType, SpecializationData, Type,
        CONTEXT,
    },
    property,
};

pub const ASPECT: Identifier = Identifier::new("aspect", 346035631277210970);
property!(ASPECT, ASPECT_PROP_DEF, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aspect {
    aspect: Expr,
}

impl Hash for Aspect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ASPECT.uuid().hash(state);
        self.aspect.hash(state);
    }
}

impl Domains for Aspect {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsModule for Aspect {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let aspect = ASPECT.function();
        Module::new(
            self,
            spec,
            elysian_function! {
                fn aspect(ASPECT, mut CONTEXT) -> CONTEXT {
                    CONTEXT.POSITION_2D = CONTEXT.POSITION_2D * VECTOR2 { X: ASPECT, Y: 1.0 };
                    return CONTEXT;
                }
            },
        )
        .with_args([self.aspect.clone().into()])
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl PreModifier for Aspect {}

pub trait IntoAspect {
    fn aspect(self, delta: impl IntoExpr) -> Modify;
}

impl<T> IntoAspect for T
where
    T: IntoModify,
{
    fn aspect(self, aspect: impl IntoExpr) -> Modify {
        self.modify().push_pre(Aspect {
            aspect: aspect.expr(),
        })
    }
}
