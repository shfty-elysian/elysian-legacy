use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::expr::{Expr, IntoExpr};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{DISTANCE, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{
        AsModule, DomainsDyn, FunctionIdentifier, HashIR, Module, SpecializationData, CONTEXT,
    },
};
use elysian_proc_macros::{elysian_expr, elysian_stmt};

use crate::shape::{DynShape, Shape};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Scale {
    pub field: DynShape,
    pub factor: Expr,
}

impl Hash for Scale {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl DomainsDyn for Scale {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.field
            .domains_dyn()
            .into_iter()
            .chain([POSITION_2D.into(), POSITION_3D.into()])
            .collect()
    }
}

impl AsModule for Scale {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
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

        let field_module = self.field.module_impl(spec);
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let scale = FunctionIdentifier::new_dynamic("scale".into());

        field_module.concat(Module::new(
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
        ))
    }
}

pub trait IntoScale {
    fn scale(self, factor: impl IntoExpr) -> Scale;
}

impl<T> IntoScale for T
where
    T: 'static + Shape,
{
    fn scale(self, factor: impl IntoExpr) -> Scale {
        Scale {
            field: Box::new(self),
            factor: factor.expr(),
        }
    }
}
