use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{ir::{
    ast::{
        Expr, Identifier, IntoBlock, IntoRead, CONTEXT, GRADIENT_2D, GRADIENT_3D, POSITION_2D,
        POSITION_3D, X, Y, IntoWrite,
    },
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData},
}, ast::modify::CONTEXT_STRUCT};

pub const CROSS_SECTION: Identifier = Identifier::new("cross_section", 11670715461129592823);

pub struct CrossSection {
    pub field: Box<dyn AsModule>,
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

impl AsModule for CrossSection {
    fn entry_point(&self) -> Identifier {
        CROSS_SECTION
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        _: &Identifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        if !spec.contains(POSITION_2D.id()) {
            panic!("CrossSection is only compatible with the 2D position domain");
        }

        let spec_3d = SpecializationData::new_3d();
        let field_entry_point = self.field.entry_point();
        self.field
            .functions(&spec_3d, &field_entry_point)
            .into_iter()
            .chain([FunctionDefinition {
                id: CROSS_SECTION,
                public: false,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                }],
                output: CONTEXT_STRUCT,
                block: [
                    [CONTEXT, POSITION_3D].write(
                        Expr::from(self.x_axis.clone()) * [CONTEXT, POSITION_2D, X].read()
                            + Expr::from(self.y_axis.clone()) * [CONTEXT, POSITION_2D, Y].read(),
                    ),
                    CONTEXT.bind(field_entry_point.call(CONTEXT.read())),
                    [CONTEXT, GRADIENT_2D].write(Expr::vector2(
                        [CONTEXT, GRADIENT_3D, X].read(),
                        [CONTEXT, GRADIENT_3D, Y].read(),
                    )),
                    CONTEXT.read().output(),
                ]
                .block(),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}
