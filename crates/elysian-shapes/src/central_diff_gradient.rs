use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    combine::{LEFT, RIGHT},
    modify::{Translate, TRANSLATE},
    shape::DynShape,
};
use elysian_core::number::Number;
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{IntoLiteral, DISTANCE, GRADIENT_2D, GRADIENT_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{
        AsModule, Domains, DomainsDyn, FunctionIdentifier, ErasedHash, Module, SpecializationData,
        CONTEXT,
    },
};
use elysian_proc_macros::elysian_expr;

#[derive(Debug)]
pub struct CentralDiffGradient {
    pub field: DynShape,
    pub epsilon: Number,
}

impl Hash for CentralDiffGradient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.erased_hash());
    }
}

impl DomainsDyn for CentralDiffGradient {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsModule for CentralDiffGradient {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let central_diff_gradient = FunctionIdentifier::new_dynamic("central_diff_gradient".into());

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
            return Module::new(
                self,
                spec,
                elysian_function! {
                    fn central_diff_gradient(mut CONTEXT) -> CONTEXT {
                        return CONTEXT;
                    }
                },
            );
        };

        let translate = TRANSLATE.specialize(&spec.filter(Translate::domains()));

        let epsilon = self.epsilon.literal();

        let field_module = self.field.module(spec);
        let field_entry = field_module.entry_point.clone();

        field_module.concat(Module::new(
            self,
            spec,
            elysian_function! {
                pub fn central_diff_gradient(mut CONTEXT) -> CONTEXT {
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
            },
        ))
    }
}
