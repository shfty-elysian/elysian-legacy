use std::{fmt::Debug, hash::Hash, borrow::Cow};

use elysian_core::{
    ast::{
        field::Field,
        modify::{Modify, CONTEXT_STRUCT},
    },
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, POSITION_2D,
            POSITION_3D, VECTOR2_STRUCT, VECTOR3_STRUCT,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use elysian_core::ast::expr::Expr;

pub const TRANSLATE: Identifier = Identifier::new("translate", 419357041369711478);
pub const DELTA_2D: Property = Property::new(
    "delta_2d",
    Type::Struct(Cow::Borrowed(VECTOR2_STRUCT)),
    1292788437813720044,
);
pub const DELTA_3D: Property = Property::new(
    "delta_3d",
    Type::Struct(Cow::Borrowed(VECTOR3_STRUCT)),
    8306277011223488934,
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
    fn domains() -> Vec<Identifier> {
        vec![POSITION_2D.id().clone(), POSITION_3D.id().clone()]
    }
}

impl AsIR for Translate {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let (position, delta) = if spec.contains(POSITION_2D.id()) {
            (POSITION_2D, DELTA_2D)
        } else if spec.contains(POSITION_3D.id()) {
            (POSITION_3D, DELTA_3D)
        } else {
            panic!("No position domain")
        };

        vec![FunctionDefinition {
            id: TRANSLATE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: delta.clone(),
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: CONTEXT_STRUCT.clone(),
            block: [
                [CONTEXT, position.clone()].write([CONTEXT, position].read() - delta.read()),
                CONTEXT.read().output(),
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
