use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    ast::{
        Expr, Identifier, IntoBlock, IntoRead, IntoBind, TypeSpec, CONTEXT, GRADIENT_2D,
        GRADIENT_3D, POSITION_2D, POSITION_3D, X, Y,
    },
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData},
};

use super::modify::CONTEXT_STRUCT;

pub const CROSS_SECTION: Identifier = Identifier::new("cross_section", 11670715461129592823);

pub struct CrossSection<T>
where
    T: TypeSpec,
{
    pub field: Box<dyn AsModule<T>>,
    pub x_axis: crate::ast::expr::Expr<T>,
    pub y_axis: crate::ast::expr::Expr<T>,
}

impl<T> Debug for CrossSection<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Raymarch")
            .field("field", &self.field)
            .finish()
    }
}

impl<T> Hash for CrossSection<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir())
    }
}

impl<T> AsModule<T> for CrossSection<T>
where
    T: TypeSpec,
{
    fn entry_point(&self) -> Identifier {
        CROSS_SECTION
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        _: &Identifier,
    ) -> Vec<crate::ir::module::FunctionDefinition<T>> {
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
                    [CONTEXT, POSITION_3D].bind(
                        Expr::from(self.x_axis.clone()) * [CONTEXT, POSITION_2D, X].read()
                            + Expr::from(self.y_axis.clone()) * [CONTEXT, POSITION_2D, Y].read(),
                    ),
                    CONTEXT.bind(field_entry_point.call(CONTEXT.read())),
                    [CONTEXT, GRADIENT_2D].bind(Expr::vector2(
                        [CONTEXT, GRADIENT_3D, X].read(),
                        [CONTEXT, GRADIENT_3D, Y].read(),
                    )),
                    CONTEXT.read().output(),
                ]
                .block(),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<crate::ir::module::StructDefinition> {
        self.field.structs()
    }
}
