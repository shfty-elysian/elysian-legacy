use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{IntoBlock, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead, IntoWrite,
            PropertyIdentifier, SpecializationData, StructIdentifier, Type, CONTEXT_PROP,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;

pub const TRANSLATE: FunctionIdentifier = FunctionIdentifier::new("translate", 419357041369711478);

pub const DELTA_2D: PropertyIdentifier = PropertyIdentifier::new("delta_2d", 1292788437813720044);
property!(
    DELTA_2D,
    DELTA_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const DELTA_3D: PropertyIdentifier = PropertyIdentifier::new("delta_3d", 8306277011223488934);
property!(
    DELTA_3D,
    DELTA_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub struct Translate {
    pub delta: Expr,
}

impl Debug for Translate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Translate")
            .field("delta", &self.delta)
            .finish()
    }
}

impl Clone for Translate {
    fn clone(&self) -> Self {
        Self {
            delta: self.delta.clone(),
        }
    }
}

impl Hash for Translate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.delta.hash(state);
    }
}

impl Domains for Translate {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D, POSITION_3D]
    }
}

impl AsIR for Translate {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let (position, delta) = if spec.contains(&POSITION_2D) {
            (POSITION_2D, DELTA_2D)
        } else if spec.contains(&POSITION_3D) {
            (POSITION_3D, DELTA_3D)
        } else {
            panic!("No position domain")
        };

        vec![FunctionDefinition {
            id: TRANSLATE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    id: delta.clone(),
                    mutable: false,
                },
                InputDefinition {
                    id: CONTEXT_PROP,
                    mutable: true,
                },
            ],
            output: CONTEXT_PROP,
            block: [
                [CONTEXT_PROP, position.clone()]
                    .write([CONTEXT_PROP, position].read() - delta.read()),
                CONTEXT_PROP.read().output(),
            ]
            .block(),
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
