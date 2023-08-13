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
        AsModule, Domains, FunctionIdentifier, Module, SpecializationData, StructIdentifier, Type,
        CONTEXT,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl AsModule for Translate {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let (position, delta) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, DELTA_2D)
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, DELTA_3D)
        } else {
            panic!("No position domain")
        };

        Module::new(
            self,
            spec,
            elysian_function! {
                fn TRANSLATE(delta, mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = CONTEXT.position - delta;
                    return CONTEXT;
                }
            },
        )
        .with_args([self.delta.clone().into()])
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
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
