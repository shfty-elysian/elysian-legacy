use std::hash::Hash;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::{
        Block, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, UV, VECTOR2, X, Y, Z,
    },
    module::{
        AsModule, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition, Module,
        SpecializationData, CONTEXT,
    },
};

use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::shape::Shape;

pub const CHEBYSHEV: FunctionIdentifier = FunctionIdentifier::new("chebyshev", 2147444445290820053);

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chebyshev;

impl Hash for Chebyshev {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        CHEBYSHEV.uuid().hash(state);
    }
}

impl Domains for Chebyshev {
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

impl AsModule for Chebyshev {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let position = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => POSITION_2D,
            (false, true) => POSITION_3D,
            _ => panic!("Invalid Position Domain"),
        };

        let gradient = match (
            spec.contains(&GRADIENT_2D.into()),
            spec.contains(&GRADIENT_3D.into()),
        ) {
            (true, false) => Some(GRADIENT_2D),
            (false, true) => Some(GRADIENT_3D),
            _ => None,
        };

        let distance = spec.contains(&DISTANCE.into());

        let mut block = Block::default();

        if distance {
            match &position {
                p if *p == POSITION_2D => block.push(elysian_stmt! {
                    CONTEXT.DISTANCE = CONTEXT.position.X.max(CONTEXT.position.Y)
                }),
                p if *p == POSITION_3D => block.push(elysian_stmt! {
                    CONTEXT.DISTANCE = CONTEXT
                        .position
                        .X
                        .max(CONTEXT.position.Y)
                        .max(CONTEXT.position.Z)
                }),
                _ => unreachable!(),
            }
        } else {
            panic!("No distance domain");
        };

        if let Some(gradient) = gradient {
            match &gradient {
                g if *g == GRADIENT_2D => block.extend(elysian_block! {
                    let mut X = 0.0;
                    if CONTEXT.position.X >= CONTEXT.position.Y {
                        X = 1.0;
                    }

                    let mut Y = 0.0;
                    if CONTEXT.position.Y >= CONTEXT.position.X {
                        Y = 1.0;
                    }

                    CONTEXT.GRADIENT_2D = VECTOR2 { X: X, Y: Y };
                }),
                g if *g == GRADIENT_3D => {}
                _ => unreachable!(),
            }
        }

        if spec.contains(&UV.into()) {
            block.extend(match &position {
                p if *p == POSITION_2D => {
                    elysian_block! {
                        CONTEXT.UV = VECTOR2 {
                            X: CONTEXT.position.X,
                            Y: CONTEXT.position.Y,
                        };
                    }
                }
                p if *p == POSITION_3D => {
                    elysian_block! {
                        let mut X = 0.0;
                        let mut Y = 0.0;

                        if CONTEXT.position.X.abs() <= CONTEXT.position.Y.abs()
                        && CONTEXT.position.X.abs() <= CONTEXT.position.Z.abs()
                        {
                            X = CONTEXT.position.Z;
                            Y = CONTEXT.position.Y;
                        }
                        else {
                            if CONTEXT.position.Y.abs() <= CONTEXT.position.X.abs()
                            && CONTEXT.position.Y.abs() <= CONTEXT.position.Z.abs()
                            {
                                X = CONTEXT.position.X;
                                Y = CONTEXT.position.Z;
                            }
                            else {
                                if CONTEXT.position.Z.abs() <= CONTEXT.position.X.abs()
                                    && CONTEXT.position.Z.abs() <= CONTEXT.position.Y.abs()
                                {
                                    X = CONTEXT.position.X;
                                    Y = CONTEXT.position.Y;
                                }
                            }
                        }

                        CONTEXT.UV = VECTOR2 {
                            X: X,
                            Y: Y,
                        };
                    }
                }
                _ => unreachable!(),
            })
        }

        block.push(elysian_stmt! { return CONTEXT });

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: CHEBYSHEV,
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
impl Shape for Chebyshev {}
