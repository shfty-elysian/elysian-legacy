use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3},
        module::{
            FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
            SpecializationData, StructIdentifier, Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;
use elysian_decl_macros::elysian_function;

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
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let (position, delta) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, DELTA_2D)
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, DELTA_3D)
        } else {
            panic!("No position domain")
        };

        let translate = TRANSLATE.specialize(spec);

        vec![elysian_function! {
            fn translate(delta, mut CONTEXT) -> CONTEXT {
                CONTEXT.position = CONTEXT.position - delta;
                return CONTEXT;
            }
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        TRANSLATE
            .specialize(spec)
            .call([self.delta.clone().into(), input])
    }
}

pub trait IntoTranslate {
    fn translate(self, delta: elysian_core::ast::expr::Expr) -> Modify;
}

impl IntoTranslate for Field {
    fn translate(self, delta: elysian_core::ast::expr::Expr) -> Modify {
        Modify {
            pre_modifiers: vec![Box::new(Translate { delta })],
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl IntoTranslate for Modify {
    fn translate(mut self, delta: elysian_core::ast::expr::Expr) -> Modify {
        self.pre_modifiers.push(Box::new(Translate { delta }));
        self
    }
}
