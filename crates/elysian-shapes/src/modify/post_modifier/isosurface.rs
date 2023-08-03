use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Block, Expr, Identifier, DISTANCE, POSITION_2D, POSITION_3D, UV, X},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, NumericType,
            PropertyIdentifier, SpecializationData, Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr as AstExpr;
use elysian_proc_macros::elysian_stmt;

pub const ISOSURFACE: FunctionIdentifier =
    FunctionIdentifier::new("isosurface", 1163045471729794054);

pub const DIST: Identifier = Identifier::new("dist", 463524741302033362);
property!(DIST, DIST_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub struct Isosurface {
    pub dist: AstExpr,
}

impl Hash for Isosurface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ISOSURFACE.uuid().hash(state);
        self.dist.hash(state);
    }
}

impl Domains for Isosurface {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![
            POSITION_2D.into(),
            POSITION_3D.into(),
            DISTANCE.into(),
            UV.into(),
        ]
    }
}

impl AsIR for Isosurface {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let mut block = Block::default();

        if spec.contains(&DISTANCE.into()) {
            block.push(elysian_stmt! {
                CONTEXT.DISTANCE = CONTEXT.DISTANCE - DIST
            })
        }

        if spec.contains(&UV.into()) && spec.contains(&POSITION_2D.into()) {
            block.push(elysian_stmt! {
                CONTEXT.UV.X = CONTEXT.UV.X - DIST
            })
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

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
