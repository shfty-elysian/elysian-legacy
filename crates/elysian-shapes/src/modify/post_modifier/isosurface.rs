use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, IntoBlock, DISTANCE},
        module::{
            FunctionDefinition, InputDefinition, IntoRead, IntoWrite, NumericType,
            PropertyIdentifier, SpecializationData, Type, CONTEXT_PROP,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;

pub const ISOSURFACE: Identifier = Identifier::new("isosurface", 1163045471729794054);

pub const DIST: PropertyIdentifier = PropertyIdentifier::new("dist", 463524741302033362);
property!(DIST, DIST_PROP, Type::Number(NumericType::Float));

pub struct Isosurface {
    pub dist: Expr,
}

impl Debug for Isosurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Isosurface")
            .field("dist", &self.dist)
            .finish()
    }
}

impl Clone for Isosurface {
    fn clone(&self) -> Self {
        Self {
            dist: self.dist.clone(),
        }
    }
}

impl Hash for Isosurface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dist.hash(state);
    }
}

impl Domains for Isosurface {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![DISTANCE]
    }
}

impl AsIR for Isosurface {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let block = if spec.contains(&DISTANCE) {
            [
                [CONTEXT_PROP, DISTANCE].write([CONTEXT_PROP, DISTANCE].read() - DIST.read()),
                CONTEXT_PROP.read().output(),
            ]
            .block()
        } else {
            [CONTEXT_PROP.read().output()].block()
        };

        vec![FunctionDefinition {
            id: ISOSURFACE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    id: DIST,
                    mutable: false,
                },
                InputDefinition {
                    id: CONTEXT_PROP,
                    mutable: true,
                },
            ],
            output: CONTEXT_PROP,
            block,
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        ISOSURFACE
            .specialize(spec)
            .call([self.dist.clone().into(), input])
    }
}

pub trait IntoIsosurface {
    fn isosurface(self, dist: elysian_core::ast::expr::Expr) -> Modify;
}

impl IntoIsosurface for Field {
    fn isosurface(self, dist: elysian_core::ast::expr::Expr) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: vec![Box::new(Isosurface { dist })],
        }
    }
}

impl IntoIsosurface for Modify {
    fn isosurface(mut self, dist: elysian_core::ast::expr::Expr) -> Modify {
        self.post_modifiers.push(Box::new(Isosurface { dist }));
        self
    }
}
