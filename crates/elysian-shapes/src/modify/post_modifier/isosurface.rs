use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{
        field::Field,
        modify::{Modify, CONTEXT_STRUCT},
    },
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{Identifier, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, DISTANCE},
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use elysian_core::ast::expr::Expr;

pub const ISOSURFACE: Identifier = Identifier::new("isosurface", 1163045471729794054);
pub const DIST: Property = Property::new("property", Type::Number, 463524741302033362);

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

impl FilterSpec for Isosurface {}

impl AsIR for Isosurface {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        vec![FunctionDefinition {
            id: ISOSURFACE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: DIST,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, DISTANCE].read() - DIST.read()),
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