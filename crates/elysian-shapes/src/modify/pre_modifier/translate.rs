use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PreModifier};
use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, VECTOR2, VECTOR3},
    module::{
        AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData,
        StructIdentifier, Type, CONTEXT,
    },
    property,
};

pub const TRANSLATE: FunctionIdentifier = FunctionIdentifier::new("translate", 419357041369711478);

pub const DELTA_2D: Identifier = Identifier::new("delta_2d", 1292788437813720044);
property!(
    DELTA_2D,
    DELTA_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const DELTA_3D: Identifier = Identifier::new("delta_3d", 8306277011223488934);
property!(
    DELTA_3D,
    DELTA_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Translate {
    pub delta: Expr,
}

impl Hash for Translate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        TRANSLATE.uuid().hash(state);
        self.delta.hash(state);
    }
}

impl Domains for Translate {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for Translate {
    fn entry_point(&self) -> FunctionIdentifier {
        TRANSLATE
    }

    fn arguments(&self, input: elysian_ir::ast::Expr) -> Vec<elysian_ir::ast::Expr> {
        vec![self.delta.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (position, delta) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, DELTA_2D)
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, DELTA_3D)
        } else {
            panic!("No position domain")
        };

        vec![elysian_function! {
            fn entry_point(delta, mut CONTEXT) -> CONTEXT {
                CONTEXT.position = CONTEXT.position - delta;
                return CONTEXT;
            }
        }]
    }
}

impl PreModifier for Translate {}

pub trait IntoTranslate {
    fn translate(self, delta: impl IntoExpr) -> Modify;
}

impl<T> IntoTranslate for T
where
    T: IntoModify,
{
    fn translate(self, delta: impl IntoExpr) -> Modify {
        self.modify().push_pre(Translate {
            delta: delta.expr(),
        })
    }
}
