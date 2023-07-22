use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::Domains,
        ast::{
            vector2, vector3, Expr, Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite,
            Number, CONTEXT, DISTANCE, GRADIENT_2D, GRADIENT_3D, LEFT, RIGHT, VECTOR2_STRUCT, X, Y,
        },
        module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData},
    },
};

use crate::modify::{Translate, TRANSLATE};

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
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let field_entry_point = self.field.entry_point();

        let (gradient, vec_x, vec_y) = if spec.contains(GRADIENT_2D.id()) {
            (
                GRADIENT_2D,
                vector2([1.0, 0.0]).literal(),
                vector2([0.0, 1.0]).literal(),
            )
        } else if spec.contains(GRADIENT_3D.id()) {
            (
                GRADIENT_3D,
                vector3([1.0, 0.0, 0.0]).literal(),
                vector3([0.0, 1.0, 0.0]).literal(),
            )
        } else {
            return self
                .field
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
                    block: [CONTEXT.read().output()].block(),
                }])
                .collect();
        };

        let translate_spec = spec.filter(Translate::domains());

        let epsilon = self.epsilon.literal();

        let expr_lx = field_entry_point.call(
            TRANSLATE
                .specialize(&translate_spec)
                .call([vec_x.clone() * -epsilon.clone(), CONTEXT.read()]),
        );

        let expr_rx = field_entry_point.call(
            TRANSLATE
                .specialize(&translate_spec)
                .call([vec_x * epsilon.clone(), CONTEXT.read()]),
        );

        let expr_ly = field_entry_point.call(
            TRANSLATE
                .specialize(&translate_spec)
                .call([vec_y.clone() * -epsilon.clone(), CONTEXT.read()]),
        );

        let expr_ry = field_entry_point.call(
            TRANSLATE
                .specialize(&translate_spec)
                .call([vec_y * epsilon.clone(), CONTEXT.read()]),
        );

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
                    LEFT.bind(expr_lx),
                    RIGHT.bind(expr_rx),
                    X.bind([LEFT, DISTANCE].read() - [RIGHT, DISTANCE].read()),
                    LEFT.bind(expr_ly),
                    RIGHT.bind(expr_ry),
                    Y.bind([LEFT, DISTANCE].read() - [RIGHT, DISTANCE].read()),
                    [CONTEXT, gradient].write(Expr::Struct(
                        VECTOR2_STRUCT,
                        [(X, X.read()), (Y, Y.read())].into_iter().collect(),
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
