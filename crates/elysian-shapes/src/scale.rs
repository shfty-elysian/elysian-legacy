use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        ast::{DISTANCE, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
        module::{
            AsIR, DomainsDyn, FunctionDefinition, FunctionIdentifier, SpecializationData, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;
use elysian_proc_macros::elysian_expr;

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
    fn domains_dyn(&self) -> Vec<elysian_core::ir::module::PropertyIdentifier> {
        self.field
            .domains_dyn()
            .into_iter()
            .chain([POSITION_2D.into(), POSITION_3D.into()])
            .collect()
    }
}

impl AsIR for Scale {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("scale".into()).specialize(spec)
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let factor = elysian_core::ir::ast::Expr::from(self.factor.clone());

        let (position, factor_vec) = if spec.contains(&POSITION_2D.into()) {
            (
                POSITION_2D,
                elysian_expr! { VECTOR2 { X: #factor, Y: #factor }},
            )
        } else if spec.contains(&POSITION_3D.into()) {
            (
                POSITION_3D,
                elysian_expr! { VECTOR3 { X: #factor, Y: #factor, Z: #factor }},
            )
        } else {
            panic!("No position domain")
        };

        let (_, field_entry, field_functions) = self.field.prepare(spec);

        field_functions
            .into_iter()
            .chain([elysian_function! {
                pub fn entry_point(mut CONTEXT) -> CONTEXT {
                    CONTEXT.position = CONTEXT.position / #factor_vec;
                    CONTEXT = #field_entry(CONTEXT);
                    CONTEXT.DISTANCE = CONTEXT.DISTANCE * #factor;
                    return CONTEXT;
                }
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoScale {
    fn scale(self, factor: Expr) -> Scale;
}

impl<T> IntoScale for T
where
    T: 'static + AsIR,
{
    fn scale(self, factor: Expr) -> Scale {
        Scale {
            field: Box::new(self),
            factor,
        }
    }
}
