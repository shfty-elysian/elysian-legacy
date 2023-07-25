use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::ir::{
    ast::{Expr, IntoBlock, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, VECTOR2, X, Y},
    module::{
        AsModule, FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead, IntoWrite,
        PropertyIdentifier, SpecializationData, StructIdentifier, Type, CONTEXT_PROP,
    },
};
use indexmap::IndexMap;

pub const CROSS_SECTION: FunctionIdentifier =
    FunctionIdentifier::new("cross_section", 11670715461129592823);

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
    fn entry_point(&self) -> FunctionIdentifier {
        CROSS_SECTION
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<PropertyIdentifier, Type>,
        _: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        if !spec.contains(&POSITION_2D) {
            panic!("CrossSection is only compatible with the 2D position domain");
        }

        let spec_3d = SpecializationData::new_3d();
        let field_entry_point = self.field.entry_point();
        self.field
            .functions(&spec_3d, tys, &field_entry_point)
            .into_iter()
            .chain([FunctionDefinition {
                id: CROSS_SECTION,
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT_PROP,
                    mutable: true,
                }],
                output: CONTEXT_PROP,
                block: [
                    [CONTEXT_PROP, POSITION_3D].write(
                        Expr::from(self.x_axis.clone()) * [CONTEXT_PROP, POSITION_2D, X].read()
                            + Expr::from(self.y_axis.clone())
                                * [CONTEXT_PROP, POSITION_2D, Y].read(),
                    ),
                    CONTEXT_PROP.bind(field_entry_point.call(CONTEXT_PROP.read())),
                    [CONTEXT_PROP, GRADIENT_2D].write(Expr::Struct(
                        StructIdentifier(VECTOR2),
                        [
                            (X, [CONTEXT_PROP, GRADIENT_3D, X].read()),
                            (Y, [CONTEXT_PROP, GRADIENT_3D, Y].read()),
                        ]
                        .into_iter()
                        .collect(),
                    )),
                    CONTEXT_PROP.read().output(),
                ]
                .block(),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}
