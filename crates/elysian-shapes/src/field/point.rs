use std::hash::Hash;

use elysian_core::ir::{
    as_ir::{AsIR, Domains},
    ast::{Block, Expr, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D},
    module::{
        FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead, PropertyIdentifier,
        SpecializationData, CONTEXT,
    },
};

use elysian_proc_macros::elysian_stmt;

pub const POINT: FunctionIdentifier = FunctionIdentifier::new("point", 2023836058494613125);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl Domains for Point {
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

impl AsIR for Point {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
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

        let mut block = Block::default();

        if distance {
            block.push(elysian_stmt!(CONTEXT.DISTANCE = CONTEXT.position.length()))
        };

        if let Some(gradient) = gradient {
            block.push(elysian_stmt!(
                CONTEXT.gradient = CONTEXT.position.normalize()
            ));
        }

        block.push(PropertyIdentifier(CONTEXT).read().output());

        vec![FunctionDefinition {
            id: POINT.specialize(&spec),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT.into(),
                mutable: true,
            }],
            output: CONTEXT.into(),
            block,
        }]
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr {
        POINT.specialize(spec).call(input)
    }
}
