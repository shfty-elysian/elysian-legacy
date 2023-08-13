use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PostModifier};
use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
    property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{Block, DISTANCE, POSITION_2D, POSITION_3D},
    module::{AsModule, Domains, ErasedHash},
    module::{
        FunctionDefinition, FunctionIdentifier, InputDefinition, Module, NumericType,
        SpecializationData, Type, CONTEXT,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

        state.write_u64(self.bound.erased_hash());
    }
}

impl Domains for DistanceBound {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsModule for DistanceBound {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
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

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: match self.ty {
                    BoundType::Lower => DISTANCE_LOWER_BOUND,
                    BoundType::Upper => DISTANCE_UPPER_BOUND,
                },
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
            },
        )
        .with_args([self.bound.clone().into()])
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl PostModifier for DistanceBound {}

pub trait IntoDistanceBound {
    fn distance_bound(self, ty: BoundType, bound: impl IntoExpr) -> Modify;
}

impl<T> IntoDistanceBound for T
where
    T: IntoModify,
{
    fn distance_bound(self, ty: BoundType, bound: impl IntoExpr) -> Modify {
        self.modify().push_post(DistanceBound {
            ty,
            bound: bound.expr(),
        })
    }
}
