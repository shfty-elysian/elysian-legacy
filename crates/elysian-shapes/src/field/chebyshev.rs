use std::hash::Hash;

use elysian_core::ir::{
    ast::{Block, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, VECTOR2, X, Y, Z},
    module::{
        AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition,
        PropertyIdentifier, SpecializationData, CONTEXT,
    },
};

use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const CHEBYSHEV: FunctionIdentifier = FunctionIdentifier::new("chebyshev", 2147444445290820053);

#[derive(Debug, Copy, Clone)]
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
        ]
    }
}

impl AsIR for Chebyshev {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        CHEBYSHEV.specialize(spec)
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (position, gradient) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, Some(GRADIENT_2D))
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, Some(GRADIENT_3D))
        } else {
            panic!("No position domain set")
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

        block.push(elysian_stmt! { return CONTEXT });

        vec![FunctionDefinition {
            id: entry_point.clone(),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT.into(),
                mutable: true,
            }],
            output: CONTEXT.into(),
            block,
        }]
    }
}
