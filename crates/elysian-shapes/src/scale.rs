use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::expr::{Expr, IntoExpr};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{DISTANCE, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{AsIR, DomainsDyn, FunctionIdentifier, SpecializationData, CONTEXT},
};
use elysian_proc_macros::{elysian_expr, elysian_stmt};

pub struct Scale {
    pub field: Box<dyn AsIR>,
    pub factor: Expr,
}

impl Debug for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scale")
            .field("field", &self.field)
            .field("factor", &self.factor)
            .finish()
    }
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

impl AsIR for Scale {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("scale".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_ir::module::FunctionDefinition> {
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

        let (_, field_call, field_functions) = self.field.call(spec, elysian_stmt! { CONTEXT });

        field_functions
            .into_iter()
            .chain([elysian_function! {
                pub fn entry_point(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = CONTEXT.position / #factor_vec;
                    CONTEXT = #field_call;
                    CONTEXT.DISTANCE = CONTEXT.DISTANCE * #factor;
                    return CONTEXT;
                }
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_ir::module::StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoScale {
    fn scale(self, factor: impl IntoExpr) -> Scale;
}

impl<T> IntoScale for T
where
    T: 'static + AsIR,
{
    fn scale(self, factor: impl IntoExpr) -> Scale {
        Scale {
            field: Box::new(self),
            factor: factor.expr(),
        }
    }
}
