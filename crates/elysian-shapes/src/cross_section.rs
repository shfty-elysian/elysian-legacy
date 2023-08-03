use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::ir::{
    ast::{Expr, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
    module::{
        AsIR, DomainsDyn, FunctionDefinition, FunctionIdentifier, SpecializationData, CONTEXT,
    },
};
use elysian_decl_macros::elysian_function;

pub const CROSS_SECTION: FunctionIdentifier =
    FunctionIdentifier::new("cross_section", 11670715461129592823);

pub struct CrossSection {
    pub field: Box<dyn AsIR>,
    pub x_axis: elysian_core::ast::expr::Expr,
    pub y_axis: elysian_core::ast::expr::Expr,
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
    fn domains_dyn(&self) -> Vec<elysian_core::ir::module::PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsIR for CrossSection {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        CROSS_SECTION
    }

    fn functions_impl(
        &self,
        spec: &SpecializationData,
        _: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        if !spec.contains(&POSITION_2D.into()) {
            panic!("CrossSection is only compatible with the 2D position domain");
        }

        let x_axis = Expr::from(self.x_axis.clone());
        let y_axis = Expr::from(self.y_axis.clone());

        let field_entry_point = self.field.entry_point(spec);

        self.field
            .functions(&SpecializationData::new_3d())
            .into_iter()
            .chain([elysian_function! {
                fn CROSS_SECTION(mut CONTEXT) -> CONTEXT {
                    CONTEXT.POSITION_3D =
                        #x_axis * CONTEXT.POSITION_2D.X
                            + #y_axis * CONTEXT.POSITION_2D.Y;

                    let CONTEXT = field_entry_point(CONTEXT);
                    CONTEXT.GRADIENT_2D = VECTOR2 {
                        X: CONTEXT.GRADIENT_3D.X,
                        Y: CONTEXT.GRADIENT_3D.Y,
                    };
                    return CONTEXT;
                }
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}
