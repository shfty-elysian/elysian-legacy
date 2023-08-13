use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{Expr, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
    module::{
        AsModule, DomainsDyn, FunctionIdentifier, HashIR, Module, SpecializationData, CONTEXT,
    },
};
use elysian_proc_macros::elysian_stmt;

use crate::shape::DynShape;

pub const CROSS_SECTION: FunctionIdentifier =
    FunctionIdentifier::new("cross_section", 11670715461129592823);

pub struct CrossSection {
    pub field: DynShape,
    pub x_axis: elysian_core::expr::Expr,
    pub y_axis: elysian_core::expr::Expr,
}

impl Debug for CrossSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Raymarch")
            .field("field", &self.field)
            .finish()
    }
}

impl Hash for CrossSection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir())
    }
}

impl DomainsDyn for CrossSection {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsModule for CrossSection {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        if !spec.contains(&POSITION_2D.into()) {
            panic!("CrossSection is only compatible with the 2D position domain");
        }

        let x_axis: Expr = self.x_axis.clone().into();
        let y_axis: Expr = self.y_axis.clone().into();

        let field_module = self.field.module_impl(&SpecializationData::new_3d());
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        field_module.concat(Module::new(
            self,
            spec,
            elysian_function! {
                fn CROSS_SECTION(mut CONTEXT) -> CONTEXT {
                    CONTEXT.POSITION_3D =
                        #x_axis * CONTEXT.POSITION_2D.X
                            + #y_axis * CONTEXT.POSITION_2D.Y;

                    let CONTEXT = #field_call;
                    CONTEXT.GRADIENT_2D = VECTOR2 {
                        X: CONTEXT.GRADIENT_3D.X,
                        Y: CONTEXT.GRADIENT_3D.Y,
                    };
                    return CONTEXT;
                }
            },
        ))
    }
}
