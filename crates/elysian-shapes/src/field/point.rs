use std::hash::Hash;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{
        Block, IntoLiteral, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, UV,
        VECTOR2, X, Y, Z,
    },
    module::{
        AsModule, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead,
        Module, SpecializationData, CONTEXT,
    },
};

use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::shape::Shape;

pub const POINT: FunctionIdentifier = FunctionIdentifier::new("point", 2023836058494613125);

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point;

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        POINT.uuid().hash(state);
    }
}

impl Domains for Point {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![
            POSITION_2D.into(),
            POSITION_3D.into(),
            DISTANCE.into(),
            GRADIENT_2D.into(),
            GRADIENT_3D.into(),
            UV.into(),
        ]
    }
}

impl AsModule for Point {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let position = if spec.contains(&POSITION_2D.into()) {
            POSITION_2D
        } else if spec.contains(&POSITION_3D.into()) {
            POSITION_3D
        } else {
            panic!("No position domain set")
        };

        let distance = spec.contains(&DISTANCE.into());

        let gradient = if spec.contains(&POSITION_2D.into()) {
            Some(GRADIENT_2D)
        } else if spec.contains(&POSITION_3D.into()) {
            Some(GRADIENT_3D)
        } else {
            None
        };

        let uv = if spec.contains(&UV.into()) {
            Some(UV)
        } else {
            None
        };

        let mut block = Block::default();

        if distance {
            block.push(elysian_stmt!(CONTEXT.DISTANCE = CONTEXT.position.length()))
        };

        if let Some(gradient) = gradient {
            block.push(elysian_stmt!(
                CONTEXT.gradient = CONTEXT.position.normalize()
            ));
        }

        let pi = core::f32::consts::PI.literal();

        if let Some(uv) = uv {
            match &position {
                p if *p == POSITION_2D => {
                    block.extend(elysian_block! {
                        CONTEXT.uv = VECTOR2 {
                            X: CONTEXT.position.length(),
                            Y: (CONTEXT.position.Y.atan2(CONTEXT.position.X) / #pi) * 0.5 + 0.5
                        };
                    });
                }
                p if *p == POSITION_3D => {
                    block.extend(elysian_block! {
                        CONTEXT.uv = VECTOR2 {
                            X: (CONTEXT.position.Z.sign() * (
                                CONTEXT.position.X / VECTOR2 {
                                    X: CONTEXT.position.X,
                                    Y: CONTEXT.position.Z,
                                }.length()
                            ).acos() / #pi) * -2.0 + 1.0,
                            Y: ((CONTEXT.position.Y / CONTEXT.position.length()).acos() / #pi) * -2.0 + 1.0,
                        };
                    });
                }
                _ => unreachable!(),
            }
        }

        block.push(PropertyIdentifier(CONTEXT).read().output());

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: POINT,
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

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Point {}
