use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Expr, Identifier, DISTANCE},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, NumericType,
            PropertyIdentifier, SpecializationData, Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr as AstExpr;
use elysian_proc_macros::elysian_block;

pub const ISOSURFACE: FunctionIdentifier =
    FunctionIdentifier::new("isosurface", 1163045471729794054);

pub const DIST: Identifier = Identifier::new("dist", 463524741302033362);
property!(DIST, DIST_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone, Hash)]
pub struct Isosurface {
    pub dist: AstExpr,
}

impl Domains for Isosurface {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![DISTANCE.into()]
    }
}

impl AsIR for Isosurface {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let block = if spec.contains(&DISTANCE.into()) {
            elysian_block! {
                CONTEXT.DISTANCE = CONTEXT.DISTANCE - DIST;
                return CONTEXT;
            }
        } else {
            elysian_block! {
                return CONTEXT;
            }
        };

        vec![FunctionDefinition {
            id: ISOSURFACE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    id: DIST.into(),
                    mutable: false,
                },
                InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                },
            ],
            output: CONTEXT.into(),
            block,
        }]
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr {
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
