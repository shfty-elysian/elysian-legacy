use std::{fmt::Debug, hash::Hash};

use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{
            Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, POSITION_2D, POSITION_3D,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use crate::ast::expr::Expr;

pub const TRANSLATE: Identifier = Identifier::new("translate", 419357041369711478);
pub const DELTA_2D: Property = Property::new("delta_2d", Type::Vector2, 1292788437813720044);
pub const DELTA_3D: Property = Property::new("delta_3d", Type::Vector3, 8306277011223488934);

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

impl FilterSpec for Translate {
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        spec.filter([POSITION_2D.id(), POSITION_3D.id()])
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
            output: &CONTEXT_STRUCT,
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
        input: crate::ir::ast::Expr,
    ) -> crate::ir::ast::Expr {
        TRANSLATE
            .specialize(spec)
            .call([self.delta.clone().into(), input])
    }
}
