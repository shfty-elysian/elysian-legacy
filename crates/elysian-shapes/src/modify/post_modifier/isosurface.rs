use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify};
use elysian_core::{
    expr::IntoExpr, identifier::Identifier, property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{Block, Expr, DISTANCE, POSITION_2D, POSITION_3D, UV, X},
    module::{
        AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition, NumericType,
        SpecializationData, Type, CONTEXT,
    },
    property,
};

use elysian_core::expr::Expr as AstExpr;
use elysian_proc_macros::elysian_stmt;

pub const ISOSURFACE: FunctionIdentifier =
    FunctionIdentifier::new("isosurface", 1163045471729794054);

pub const DIST: Identifier = Identifier::new("dist", 463524741302033362);
property!(DIST, DIST_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub struct Isosurface {
    dist: AstExpr,
}

impl Isosurface {
    pub fn new(dist: impl IntoExpr) -> Self {
        Isosurface { dist: dist.expr() }
    }
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
    fn entry_point(&self) -> FunctionIdentifier {
        ISOSURFACE
    }

    fn arguments(&self, input: Expr) -> Vec<Expr> {
        vec![self.dist.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
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
            id: entry_point.clone(),
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
}

pub trait IntoIsosurface {
    fn isosurface(self, dist: impl IntoExpr) -> Modify;
}

impl<T> IntoIsosurface for T
where
    T: IntoModify,
{
    fn isosurface(self, dist: impl IntoExpr) -> Modify {
        self.modify().push_post(Isosurface::new(dist))
    }
}
