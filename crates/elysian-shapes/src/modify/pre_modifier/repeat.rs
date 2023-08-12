use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify};
use elysian_core::{
    expr::IntoExpr, identifier::Identifier, property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, VECTOR2, VECTOR3},
    module::{
        AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData,
        StructIdentifier, Type, CONTEXT,
    },
    property,
};

use elysian_core::expr::Expr;
use elysian_decl_macros::elysian_function;

pub const REPEAT_CLAMPED: Identifier = Identifier::new("repeat_clamped", 346035631277210970);
pub const REPEAT_INFINITE: Identifier = Identifier::new("repeat_infinite", 468741336633754013);

pub const PERIOD_2D: Identifier = Identifier::new("period_2d", 6536292381924824837);
property!(
    PERIOD_2D,
    PERIOD_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const PERIOD_3D: Identifier = Identifier::new("period_3d", 890074657369212997);
property!(
    PERIOD_3D,
    PERIOD_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const MIN_2D: Identifier = Identifier::new("min_2d", 2361871511508665757);
property!(MIN_2D, MIN_2D_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const MIN_3D: Identifier = Identifier::new("min_3d", 8723062023762026);
property!(MIN_3D, MIN_3D_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const MAX_2D: Identifier = Identifier::new("max_2d", 1490794385394722553);
property!(MAX_2D, MAX_2D_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const MAX_3D: Identifier = Identifier::new("max_3d", 998568592829815925);
property!(MAX_3D, MAX_3D_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const REPEAT_ID_2D: Identifier = Identifier::new("repeat_id_2d", 1118017393866660680);
property!(
    REPEAT_ID_2D,
    REPEAT_ID_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const REPEAT_ID_3D: Identifier = Identifier::new("repeat_id_3d", 145404617164324305);
property!(
    REPEAT_ID_3D,
    REPEAT_ID_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

#[derive(Debug, Clone)]
pub struct Repeat {
    pub period: Expr,
    pub range: Option<(Expr, Expr)>,
}

impl Hash for Repeat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.range.is_none() {
            REPEAT_INFINITE.uuid().hash(state);
        } else {
            REPEAT_CLAMPED.uuid().hash(state);
        }
        self.period.hash(state);
    }
}

impl Domains for Repeat {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for Repeat {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier(if self.range.is_none() {
            REPEAT_INFINITE
        } else {
            REPEAT_CLAMPED
        })
    }

    fn arguments(&self, input: elysian_ir::ast::Expr) -> Vec<elysian_ir::ast::Expr> {
        if let Some((min, max)) = self.range.as_ref() {
            let min = elysian_ir::ast::Expr::from(min.clone());
            let max = elysian_ir::ast::Expr::from(max.clone());
            vec![self.period.clone().into(), min, max, input]
        } else {
            vec![self.period.clone().into(), input]
        }
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (position, period, min, max, repeat_id) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, PERIOD_2D, MIN_2D, MAX_2D, REPEAT_ID_2D)
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, PERIOD_3D, MIN_3D, MAX_3D, REPEAT_ID_3D)
        } else {
            panic!("No position domain")
        };

        vec![if self.range.is_some() {
            elysian_function! {
                fn entry_point(period, min, max, mut CONTEXT) -> CONTEXT {
                    CONTEXT.repeat_id = (CONTEXT.position / period).round().clamp(min, max);
                    CONTEXT.position =
                        CONTEXT.position - period * (CONTEXT.position / period).round().clamp(min, max);
                    return CONTEXT;
                }
            }
        } else {
            elysian_function! {
                fn entry_point(period, mut CONTEXT) -> CONTEXT {
                    CONTEXT.repeat_id = (CONTEXT.position / period).round();
                    CONTEXT.position =
                        (CONTEXT.position + period * 0.5) % period - period * 0.5;
                    return CONTEXT;
                }
            }
        }]
    }
}

pub trait IntoRepeat {
    fn repeat_infinite(self, period: impl IntoExpr) -> Modify;

    fn repeat_clamped(
        self,
        period: impl IntoExpr,
        min: impl IntoExpr,
        max: impl IntoExpr,
    ) -> Modify;
}

impl<T> IntoRepeat for T
where
    T: IntoModify,
{
    fn repeat_infinite(self, period: impl IntoExpr) -> Modify {
        self.modify().push_pre(Repeat {
            period: period.expr(),
            range: None,
        })
    }

    fn repeat_clamped(
        self,
        period: impl IntoExpr,
        min: impl IntoExpr,
        max: impl IntoExpr,
    ) -> Modify {
        self.modify().push_pre(Repeat {
            period: period.expr(),
            range: Some((min.expr(), max.expr())),
        })
    }
}
