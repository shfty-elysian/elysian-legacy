use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use rust_gpu_bridge::glam::Vec2;

use crate::ir::{
    ast::{
        Block, Expr, GlamF32, Identifier, IntoLiteral, IntoWrite, VectorSpace, CONTEXT, DISTANCE,
        GRADIENT,
    },
    module::{AsModule, FunctionDefinition, InputDefinition},
};

use super::modify::{CONTEXT_STRUCT, TRANSLATE};

pub struct CentralDiffGradient<T, const N: usize>
where
    T: VectorSpace<N>,
{
    pub field: Box<dyn AsModule<T, N>>,
    pub epsilon: T::NUMBER,
}

impl<T, const N: usize> Debug for CentralDiffGradient<T, N>
where
    T: VectorSpace<N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CentralDiffGradient")
            .field("field", &self.field)
            .finish()
    }
}

impl<T, const N: usize> Hash for CentralDiffGradient<T, N>
where
    T: VectorSpace<N>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
    }
}

impl AsModule<GlamF32, 2> for CentralDiffGradient<GlamF32, 2> {
    fn entry_point(&self) -> crate::ir::ast::Identifier {
        Identifier::new_dynamic("central_diff_gradient")
    }

    fn functions(
        &self,
        entry_point: crate::ir::ast::Identifier,
    ) -> Vec<crate::ir::module::FunctionDefinition<GlamF32, 2>> {
        let field_entry_point = self.field.entry_point();
        let epsilon = self.epsilon.literal();
        self.field
            .functions(field_entry_point.clone())
            .into_iter()
            .chain([FunctionDefinition {
                id: entry_point,
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                }],
                output: CONTEXT_STRUCT,
                block: Block(vec![
                    [CONTEXT].write(Expr::Call {
                        function: field_entry_point.clone(),
                        args: vec![CONTEXT.read()],
                    }),
                    [CONTEXT, GRADIENT].write(
                        Expr::Vector2(
                            Box::new(
                                Expr::Read(
                                    Some(Box::new(Expr::Call {
                                        function: field_entry_point.clone(),
                                        args: vec![Expr::Call {
                                            function: TRANSLATE,
                                            args: vec![
                                                Vec2::X.literal() * -epsilon.clone(),
                                                CONTEXT.read(),
                                            ],
                                        }],
                                    })),
                                    vec![DISTANCE],
                                ) - Expr::Read(
                                    Some(Box::new(Expr::Call {
                                        function: field_entry_point.clone(),
                                        args: vec![Expr::Call {
                                            function: TRANSLATE,
                                            args: vec![
                                                Vec2::X.literal() * epsilon.clone(),
                                                CONTEXT.read(),
                                            ],
                                        }],
                                    })),
                                    vec![DISTANCE],
                                ),
                            ),
                            Box::new(
                                Expr::Read(
                                    Some(Box::new(Expr::Call {
                                        function: field_entry_point.clone(),
                                        args: vec![Expr::Call {
                                            function: TRANSLATE,
                                            args: vec![
                                                Vec2::Y.literal() * -epsilon.clone(),
                                                CONTEXT.read(),
                                            ],
                                        }],
                                    })),
                                    vec![DISTANCE],
                                ) - Expr::Read(
                                    Some(Box::new(Expr::Call {
                                        function: field_entry_point.clone(),
                                        args: vec![Expr::Call {
                                            function: TRANSLATE,
                                            args: vec![
                                                Vec2::Y.literal() * epsilon,
                                                CONTEXT.read(),
                                            ],
                                        }],
                                    })),
                                    vec![DISTANCE],
                                ),
                            ),
                        )
                        .normalize(),
                    ),
                    CONTEXT.read().output(),
                ]),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<crate::ir::module::StructDefinition> {
        self.field.structs()
    }
}
