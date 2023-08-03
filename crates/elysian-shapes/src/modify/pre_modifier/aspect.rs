use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{
        field::Field,
        modify::{IntoModify, Modify},
    },
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
        module::{
            FunctionDefinition, FunctionIdentifier, NumericType, PropertyIdentifier,
            SpecializationData, Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;
use elysian_decl_macros::elysian_function;

pub const ASPECT: Identifier = Identifier::new("aspect", 346035631277210970);
property!(ASPECT, ASPECT_PROP_DEF, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub struct Aspect {
    pub aspect: Expr,
}

impl Hash for Aspect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ASPECT.uuid().hash(state);
        self.aspect.hash(state);
    }
}

impl Domains for Aspect {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for Aspect {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let aspect = FunctionIdentifier(ASPECT).specialize(spec);

        vec![elysian_function! {
            fn aspect(ASPECT, mut CONTEXT) -> CONTEXT {
                CONTEXT.POSITION_2D = CONTEXT.POSITION_2D * VECTOR2 { X: ASPECT, Y: 1.0 };
                return CONTEXT;
            }
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        FunctionIdentifier(ASPECT)
            .specialize(spec)
            .call([self.aspect.clone().into(), input])
    }
}

pub trait IntoAspect {
    fn aspect(self, delta: elysian_core::ast::expr::Expr) -> Modify;
}

impl<T> IntoAspect for T
where
    T: IntoModify,
{
    fn aspect(self, aspect: elysian_core::ast::expr::Expr) -> Modify {
        let mut m = self.modify();
        m.pre_modifiers.push(Box::new(Aspect { aspect }));
        m
    }
}
