use std::hash::Hash;

use elysian_core::ir::{
    as_ir::{AsIR, Domains},
    ast::{Expr, IntoBlock, Stmt, DISTANCE, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D},
    module::{
        FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead, PropertyIdentifier,
        SpecializationData, CONTEXT_PROP,
    },
};

use elysian_macros::elysian_stmt;

pub const POINT: FunctionIdentifier = FunctionIdentifier::new("point", 2023836058494613125);

#[derive(Debug, Copy, Clone, Hash)]
pub struct Point;

impl Domains for Point {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D, POSITION_3D, DISTANCE, GRADIENT_2D, GRADIENT_3D]
    }
}

impl AsIR for Point {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let position = if spec.contains(&POSITION_2D) {
            POSITION_2D
        } else if spec.contains(&POSITION_3D) {
            POSITION_3D
        } else {
            panic!("No position domain set")
        };

        let distance = spec.contains(&DISTANCE);

        let gradient = if spec.contains(&POSITION_2D) {
            Some(GRADIENT_2D)
        } else if spec.contains(&POSITION_3D) {
            Some(GRADIENT_3D)
        } else {
            None
        };

        let mut block = vec![];
        if distance {
            block.push(elysian_stmt!(#CONTEXT_PROP.#DISTANCE = #CONTEXT_PROP.#position.length()))
        };

        if let Some(gradient) = gradient {
            block
                .push(elysian_stmt!(#CONTEXT_PROP.#gradient = #CONTEXT_PROP.#position.normalize()));
        }

        block.push(CONTEXT_PROP.read().output());

        vec![FunctionDefinition {
            id: POINT.specialize(&spec),
            public: false,
            inputs: vec![InputDefinition {
                id: CONTEXT_PROP,
                mutable: true,
            }],
            output: CONTEXT_PROP,
            block: block.block(),
        }]
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr {
        POINT.specialize(spec).call(input)
    }
}
