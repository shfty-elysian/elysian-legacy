use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PostModifier};
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{GRADIENT_2D, GRADIENT_3D, NORMAL, VECTOR3, X, Y, Z},
    module::{
        AsModule, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition, Module,
        SpecializationData, CONTEXT,
    },
};
use elysian_proc_macros::elysian_block;

pub const GRADIENT_NORMALS: FunctionIdentifier =
    FunctionIdentifier::new("gradient_normals", 18573716892008865657);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientNormals;

impl Hash for GradientNormals {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        GRADIENT_NORMALS.uuid().hash(state);
    }
}

impl Domains for GradientNormals {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![GRADIENT_2D.into(), GRADIENT_3D.into()]
    }
}

impl AsModule for GradientNormals {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let block = if spec.contains(&GRADIENT_2D.into()) {
            elysian_block! {
                CONTEXT.NORMAL = VECTOR3 {
                    X: CONTEXT.GRADIENT_2D.X,
                    Y: CONTEXT.GRADIENT_2D.Y,
                    Z: 1.0,
                }.normalize();
                return CONTEXT;
            }
        } else if spec.contains(&GRADIENT_3D.into()) {
            elysian_block! {
                CONTEXT.NORMAL = CONTEXT.GRADIENT_3D.normalize();
                return CONTEXT;
            }
        } else {
            elysian_block! {
                return CONTEXT;
            }
        };

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: GRADIENT_NORMALS,
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                }],
                output: CONTEXT.into(),
                block,
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serialize)]
impl PostModifier for GradientNormals {}

pub trait IntoGradientNormals {
    fn gradient_normals(self) -> Modify;
}

impl<T> IntoGradientNormals for T
where
    T: 'static + IntoModify,
{
    fn gradient_normals(self) -> Modify {
        self.modify().push_post(GradientNormals)
    }
}
