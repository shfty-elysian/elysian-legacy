use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::ir::{
    ast::{
        Expr, GlamF32, Identifier, IntoBlock, IntoLiteral, IntoWrite, TypeSpec, CONTEXT, DISTANCE,
        GRADIENT_2D, GRADIENT_3D,
    },
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData},
};

use super::modify::{CONTEXT_STRUCT, TRANSLATE};

pub struct CentralDiffGradient<T>
where
    T: TypeSpec,
{
    pub field: Box<dyn AsModule<T>>,
    pub epsilon: T::NUMBER,
}

impl<T> Debug for CentralDiffGradient<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CentralDiffGradient")
            .field("field", &self.field)
            .finish()
    }
}

impl<T> Hash for CentralDiffGradient<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl AsModule<GlamF32> for CentralDiffGradient<GlamF32> {
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("central_diff_gradient")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<crate::ir::module::FunctionDefinition<GlamF32>> {
        let (gradient, vec_x, vec_y) = if spec.domains.contains(&GRADIENT_2D) {
            (GRADIENT_2D, Vec2::X.literal(), Vec2::Y.literal())
        } else if spec.domains.contains(&GRADIENT_3D) {
            (GRADIENT_2D, Vec3::X.literal(), Vec3::Y.literal())
        } else {
            panic!("No gradient domain");
        };

        let field_entry_point = self.field.entry_point();
        let epsilon = self.epsilon.literal();
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
                    CONTEXT.write(field_entry_point.call(CONTEXT.read())),
                    [CONTEXT, gradient].write(
                        Expr::vector2(
                            field_entry_point
                                .call(
                                    TRANSLATE
                                        .call([vec_x.clone() * -epsilon.clone(), CONTEXT.read()]),
                                )
                                .read(DISTANCE)
                                - field_entry_point
                                    .call(TRANSLATE.call([vec_x * epsilon.clone(), CONTEXT.read()]))
                                    .read(DISTANCE),
                            field_entry_point
                                .call(
                                    TRANSLATE
                                        .call([vec_y.clone() * -epsilon.clone(), CONTEXT.read()]),
                                )
                                .read(DISTANCE)
                                - field_entry_point
                                    .call(TRANSLATE.call([vec_y * epsilon, CONTEXT.read()]))
                                    .read(DISTANCE),
                        )
                        .normalize(),
                    ),
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
