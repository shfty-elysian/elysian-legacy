use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{
        expr::IntoExpr,
        modify::{IntoModify, Modify},
    },
    ir::{
        ast::{
            vector2, vector3, Identifier, IntoLiteral, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3,
        },
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
            SpecializationData, StructIdentifier, Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;
use elysian_decl_macros::elysian_function;

pub const FLIP_BASIS: FunctionIdentifier =
    FunctionIdentifier::new("flip_basis", 1894406051684466109);

pub const FLIP_2D: Identifier = Identifier::new("flip_2d", 4082005642022253885);
property!(
    FLIP_2D,
    FLIP_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const FLIP_3D: Identifier = Identifier::new("flip_3d", 6664639271221372354);
property!(
    FLIP_3D,
    FLIP_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

#[derive(Debug, Clone)]
pub struct FlipBasis {
    pub basis: Expr,
}

impl Hash for FlipBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        FLIP_BASIS.uuid().hash(state);
        self.basis.hash(state);
    }
}

impl Domains for FlipBasis {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for FlipBasis {
    fn entry_point(&self) -> FunctionIdentifier {
        FLIP_BASIS
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.basis.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (position, flip, one) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, FLIP_2D, vector2([1.0, 1.0]).literal())
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, FLIP_3D, vector3([1.0, 1.0, 1.0]).literal())
        } else {
            panic!("No position domain")
        };

        vec![elysian_function! {
            fn entry_point(flip, mut CONTEXT) -> CONTEXT {
                CONTEXT.position = CONTEXT.position * ((#one - flip) * 2.0 - #one).sign();
                return CONTEXT;
            }
        }]
    }
}

pub trait IntoFlipBasis {
    fn flip_basis(self, basis: impl IntoExpr) -> Modify;
}

impl<T> IntoFlipBasis for T
where
    T: IntoModify,
{
    fn flip_basis(self, basis: impl IntoExpr) -> Modify {
        self.modify().push_pre(FlipBasis {
            basis: basis.expr(),
        })
    }
}
