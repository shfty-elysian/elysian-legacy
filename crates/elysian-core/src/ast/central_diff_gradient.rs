use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    as_ir::FilterSpec,
    ast::{
        Expr, Identifier, IntoBlock, IntoLiteral, IntoWrite, Number, CONTEXT, DISTANCE,
        GRADIENT_2D, GRADIENT_3D,
    },
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData},
};

use super::modify::{Translate, CONTEXT_STRUCT, TRANSLATE};

pub struct CentralDiffGradient {
    pub field: Box<dyn AsModule>,
    pub epsilon: Number,
}

impl Debug for CentralDiffGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CentralDiffGradient")
            .field("field", &self.field)
            .finish()
    }
}

impl Hash for CentralDiffGradient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl AsModule for CentralDiffGradient {
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("central_diff_gradient")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<crate::ir::module::FunctionDefinition> {
        let (gradient, vec_x, vec_y) = if spec.contains(GRADIENT_2D.id()) {
            (GRADIENT_2D, [1.0, 0.0].literal(), [0.0, 1.0].literal())
        } else if spec.contains(GRADIENT_3D.id()) {
            (
                GRADIENT_3D,
                [1.0, 0.0, 0.0].literal(),
                [0.0, 1.0, 0.0].literal(),
            )
        } else {
            panic!("No gradient domain");
        };

        let translate_spec = Translate::filter_spec(spec);

        let field_entry_point = self.field.entry_point();
        let epsilon = self.epsilon.literal();

        let expr_x = field_entry_point
            .call(
                TRANSLATE
                    .specialize(&translate_spec)
                    .call([vec_x.clone() * -epsilon.clone(), CONTEXT.read()]),
            )
            .read(DISTANCE)
            - field_entry_point
                .call(
                    TRANSLATE
                        .specialize(&translate_spec)
                        .call([vec_x * epsilon.clone(), CONTEXT.read()]),
                )
                .read(DISTANCE);

        let expr_y = field_entry_point
            .call(
                TRANSLATE
                    .specialize(&translate_spec)
                    .call([vec_y.clone() * -epsilon.clone(), CONTEXT.read()]),
            )
            .read(DISTANCE)
            - field_entry_point
                .call(
                    TRANSLATE
                        .specialize(&translate_spec)
                        .call([vec_y * epsilon.clone(), CONTEXT.read()]),
                )
                .read(DISTANCE);

        let expr_vec = Expr::vector2(expr_x, expr_y);

        self.field
            .functions(spec, &field_entry_point)
            .into_iter()
            .chain([FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                }],
                output: CONTEXT_STRUCT,
                block: [
                    CONTEXT.bind(field_entry_point.call(CONTEXT.read())),
                    [CONTEXT, gradient].write(expr_vec),
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
