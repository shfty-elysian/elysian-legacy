use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use rust_gpu_bridge::glam::Vec2;

use crate::ir::{
    ast::{
        Expr, GlamF32, Identifier, IntoBlock, IntoLiteral, IntoWrite, TypeSpec, CONTEXT, DISTANCE,
        GRADIENT_2D,
    },
    module::{AsModule, FunctionDefinition, InputDefinition},
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
        entry_point: &Identifier,
    ) -> Vec<crate::ir::module::FunctionDefinition<GlamF32>> {
        let field_entry_point = self.field.entry_point();
        let epsilon = self.epsilon.literal();
        self.field
            .functions(&field_entry_point)
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
                    [CONTEXT, GRADIENT_2D].write(
                        Expr::vector2(
                            field_entry_point
                                .call(
                                    TRANSLATE.call([
                                        Vec2::X.literal() * -epsilon.clone(),
                                        CONTEXT.read(),
                                    ]),
                                )
                                .read(DISTANCE)
                                - field_entry_point
                                    .call(TRANSLATE.call([
                                        Vec2::X.literal() * epsilon.clone(),
                                        CONTEXT.read(),
                                    ]))
                                    .read(DISTANCE),
                            field_entry_point
                                .call(
                                    TRANSLATE.call([
                                        Vec2::Y.literal() * -epsilon.clone(),
                                        CONTEXT.read(),
                                    ]),
                                )
                                .read(DISTANCE)
                                - field_entry_point
                                    .call(
                                        TRANSLATE
                                            .call([Vec2::Y.literal() * epsilon, CONTEXT.read()]),
                                    )
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
