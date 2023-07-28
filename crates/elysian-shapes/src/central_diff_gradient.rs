use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::combine::{LEFT, RIGHT},
    ir::{
        as_ir::Domains,
        ast::{Number, DISTANCE, GRADIENT_2D, GRADIENT_3D, VECTOR2, VECTOR3, X, Y, Z},
        module::{
            AsModule, FunctionDefinition, FunctionIdentifier, InputDefinition, PropertyIdentifier,
            SpecializationData, Type, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;
use elysian_proc_macros::elysian_expr;
use indexmap::IndexMap;

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
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("central_diff_gradient")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<PropertyIdentifier, Type>,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let entry_point = entry_point.clone();
        let field_entry_point = self.field.entry_point();

        let (gradient, vec_x, vec_y) = if spec.contains(&GRADIENT_2D.into()) {
            (
                GRADIENT_2D,
                elysian_expr!(VECTOR2 { X: 1.0, Y: 0.0 }),
                elysian_expr!(VECTOR2 { X: 0.0, Y: 1.0 }),
            )
        } else if spec.contains(&GRADIENT_3D.into()) {
            (
                GRADIENT_3D,
                elysian_expr!(VECTOR3 {
                    X: 1.0,
                    Y: 0.0,
                    Z: 0.0
                }),
                elysian_expr!(VECTOR3 {
                    X: 0.0,
                    Y: 1.0,
                    Z: 0.0
                }),
            )
        } else {
            return self
                .field
                .functions(spec, tys, &field_entry_point)
                .into_iter()
                .chain(elysian_function! {
                    fn entry_point(mut CONTEXT) -> CONTEXT {
                        return CONTEXT;
                    }
                })
                .collect();
        };

        let translate_spec = spec.filter(Translate::domains());
        let translate_func = TRANSLATE.specialize(&translate_spec);

        let epsilon = self.epsilon.literal();

        self.field
            .functions(spec, tys, &field_entry_point)
            .into_iter()
            .chain([elysian_function! {
                pub fn entry_point(mut CONTEXT) -> CONTEXT {
                    let CONTEXT = field_entry_point(CONTEXT);
                    let LEFT = field_entry_point(translate_func(#vec_x * -#epsilon, CONTEXT));
                    let RIGHT = field_entry_point(#translate_func(#vec_x * #epsilon, CONTEXT));
                    let X = LEFT.DISTANCE - RIGHT.DISTANCE;
                    let LEFT = field_entry_point(translate_func(#vec_y * -#epsilon, CONTEXT));
                    let RIGHT = field_entry_point(translate_func(#vec_y * #epsilon, CONTEXT));
                    let Y = LEFT.DISTANCE - RIGHT.DISTANCE;
                    CONTEXT.gradient = VECTOR2 {
                        X: X,
                        Y: Y,
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
