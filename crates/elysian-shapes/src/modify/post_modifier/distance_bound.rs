use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{expr::Expr, field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains, HashIR},
        ast::{Block, Identifier, DISTANCE, POSITION_2D, POSITION_3D},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, NumericType,
            PropertyIdentifier, SpecializationData, Type, CONTEXT,
        },
    },
    property,
};

use elysian_proc_macros::elysian_stmt;

use crate::modify::BoundType;

pub const DISTANCE_LOWER_BOUND: FunctionIdentifier =
    FunctionIdentifier::new("distance_lower_bound", 1708761321235124517);

pub const DISTANCE_UPPER_BOUND: FunctionIdentifier =
    FunctionIdentifier::new("distance_upper_bound", 8154816731892893782);

pub const BOUND: Identifier = Identifier::new("bound", 906044067471398839);
property!(BOUND, BOUND_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub struct DistanceBound {
    ty: BoundType,
    bound: Expr,
}

impl Hash for DistanceBound {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.ty {
            BoundType::Lower => DISTANCE_LOWER_BOUND.uuid().hash(state),
            BoundType::Upper => DISTANCE_UPPER_BOUND.uuid().hash(state),
        };

        state.write_u64(self.bound.hash_ir());
    }
}

impl Domains for DistanceBound {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for DistanceBound {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let mut block = Block::default();

        match self.ty {
            BoundType::Lower => block.push(elysian_stmt! {
                CONTEXT.DISTANCE = CONTEXT.DISTANCE.max(BOUND)
            }),
            BoundType::Upper => block.push(elysian_stmt! {
                CONTEXT.DISTANCE = CONTEXT.DISTANCE.min(BOUND)
            }),
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        let distance_bound = match self.ty {
            BoundType::Lower => DISTANCE_LOWER_BOUND.specialize(spec),
            BoundType::Upper => DISTANCE_UPPER_BOUND.specialize(spec),
        };

        vec![FunctionDefinition {
            id: distance_bound,
            public: false,
            inputs: vec![
                InputDefinition {
                    id: BOUND.into(),
                    mutable: false,
                },
                InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                },
            ],
            output: CONTEXT.into(),
            block,
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        match self.ty {
            BoundType::Lower => DISTANCE_LOWER_BOUND,
            BoundType::Upper => DISTANCE_UPPER_BOUND,
        }
        .specialize(spec)
        .call([self.bound.clone().into(), input])
    }
}

pub trait IntoDistanceBound {
    fn distance_bound(self, ty: BoundType, bound: Expr) -> Modify;
}

impl IntoDistanceBound for Field {
    fn distance_bound(self, ty: BoundType, bound: Expr) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: vec![Box::new(DistanceBound { ty, bound })],
        }
    }
}

impl IntoDistanceBound for Modify {
    fn distance_bound(mut self, ty: BoundType, bound: Expr) -> Modify {
        self.post_modifiers
            .push(Box::new(DistanceBound { ty, bound }));
        self
    }
}
