use std::{fmt::Debug, hash::Hash};

use elysian_core::expr::{Expr, IntoExpr};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{DISTANCE, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{DomainsDyn, FunctionIdentifier, Module, SpecializationData, CONTEXT},
};
use elysian_proc_macros::elysian_expr;

use crate::{
    shape::Shape,
    wrap::{Wrap, Wrapper},
};

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scale {
    pub factor: Expr,
}

impl DomainsDyn for Scale {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wrapper for Scale {
    fn module(
        &self,
        spec: &SpecializationData,
        field_call: elysian_ir::ast::Expr,
    ) -> elysian_ir::module::Module {
        let factor = elysian_ir::ast::Expr::from(self.factor.clone());

        let (position, factor_vec) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (
                POSITION_2D,
                elysian_expr! { VECTOR2 { X: #factor, Y: #factor }},
            ),
            (false, true) => (
                POSITION_3D,
                elysian_expr! { VECTOR3 { X: #factor, Y: #factor, Z: #factor }},
            ),
            _ => panic!("Invalid position domain"),
        };

        let scale = FunctionIdentifier::new_dynamic("scale".into());

        Module::new(
            self,
            spec,
            elysian_function! {
                pub fn scale(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = CONTEXT.position / #factor_vec;
                    CONTEXT = #field_call;
                    CONTEXT.DISTANCE = CONTEXT.DISTANCE * #factor;
                    return CONTEXT;
                }
            },
        )
    }
}

pub trait IntoScale {
    fn scale(self, factor: impl IntoExpr) -> Wrap;
}

impl<T> IntoScale for T
where
    T: 'static + Shape,
{
    fn scale(self, factor: impl IntoExpr) -> Wrap {
        Wrap::new(
            Scale {
                factor: factor.expr(),
            },
            self,
        )
    }
}
