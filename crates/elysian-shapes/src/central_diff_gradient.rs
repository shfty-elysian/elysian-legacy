use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    combine::{LEFT, RIGHT},
    modify::{Translate, TRANSLATE},
};
use elysian_core::number::Number;
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{IntoLiteral, DISTANCE, GRADIENT_2D, GRADIENT_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{AsIR, Domains, DomainsDyn, FunctionIdentifier, SpecializationData, CONTEXT},
};
use elysian_proc_macros::elysian_expr;

pub struct CentralDiffGradient {
    pub field: Box<dyn AsIR>,
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

impl DomainsDyn for CentralDiffGradient {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsIR for CentralDiffGradient {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("central_diff_gradient".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_ir::module::FunctionDefinition> {
        let entry_point = entry_point.clone();

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
            return vec![elysian_function! {
                fn entry_point(mut CONTEXT) -> CONTEXT {
                    return CONTEXT;
                }
            }];
        };

        let translate = TRANSLATE.specialize(&spec.filter(Translate::domains()));

        let epsilon = self.epsilon.literal();

        let (_, field_entry, field_functions) = self.field.prepare(spec);

        field_functions
            .into_iter()
            .chain([elysian_function! {
                pub fn entry_point(mut CONTEXT) -> CONTEXT {
                    let CONTEXT = field_entry(CONTEXT);
                    let LEFT = field_entry(#translate(#vec_x * -#epsilon, CONTEXT));
                    let RIGHT = field_entry(#translate(#vec_x * #epsilon, CONTEXT));
                    let X = LEFT.DISTANCE - RIGHT.DISTANCE;
                    let LEFT = field_entry(#translate(#vec_y * -#epsilon, CONTEXT));
                    let RIGHT = field_entry(#translate(#vec_y * #epsilon, CONTEXT));
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

    fn structs(&self) -> Vec<elysian_ir::module::StructDefinition> {
        self.field.structs()
    }
}
