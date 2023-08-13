use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PreModifier};
use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
    property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{Block, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{
        AsModule, Domains, FunctionDefinition, FunctionIdentifier, ErasedHash,
        InputDefinition, Module, SpecializationData, StructIdentifier, Type, CONTEXT,
    },
    property,
};

use elysian_proc_macros::elysian_stmt;

pub const BASIS_LOWER_BOUND: FunctionIdentifier =
    FunctionIdentifier::new("basis_lower_bound", 16618558971927128577);

pub const BASIS_UPPER_BOUND: FunctionIdentifier =
    FunctionIdentifier::new("basis_upper_bound", 1860524201863764510);

pub const BOUND_2D: Identifier = Identifier::new("bound_2d", 1575459390820067951);
property!(
    BOUND_2D,
    BOUND_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const BOUND_3D: Identifier = Identifier::new("bound_3d", 9676647711973555547);
property!(
    BOUND_3D,
    BOUND_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoundType {
    Lower,
    Upper,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasisBound {
    ty: BoundType,
    bound: Expr,
}

impl Hash for BasisBound {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.ty {
            BoundType::Lower => BASIS_LOWER_BOUND.uuid().hash(state),
            BoundType::Upper => BASIS_UPPER_BOUND.uuid().hash(state),
        };

        state.write_u64(self.bound.erased_hash());
    }
}

impl Domains for BasisBound {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsModule for BasisBound {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let (position, bound) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, BOUND_2D),
            (false, true) => (POSITION_3D, BOUND_3D),
            _ => panic!("No position domain"),
        };

        let mut block = Block::default();

        match &position {
            p if *p == POSITION_2D => match self.ty {
                BoundType::Lower => {
                    block.push(elysian_stmt! {
                        CONTEXT.position = VECTOR2 {
                            X: CONTEXT.position.X.max(bound.X),
                            Y: CONTEXT.position.Y.max(bound.Y),
                        }
                    });
                }
                BoundType::Upper => {
                    block.push(elysian_stmt! {
                        CONTEXT.position = VECTOR2 {
                            X: CONTEXT.position.X.min(bound.X),
                            Y: CONTEXT.position.Y.min(bound.Y),
                        }
                    });
                }
            },
            p if *p == POSITION_3D => match self.ty {
                BoundType::Lower => {
                    block.push(elysian_stmt! {
                        CONTEXT.position = VECTOR3 {
                            X: CONTEXT.position.X.max(bound.X),
                            Y: CONTEXT.position.Y.max(bound.Y),
                            Z: CONTEXT.position.Z.max(bound.Z),
                        }
                    });
                }
                BoundType::Upper => {
                    block.push(elysian_stmt! {
                        CONTEXT.position = VECTOR3 {
                            X: CONTEXT.position.X.min(bound.X),
                            Y: CONTEXT.position.Y.min(bound.Y),
                            Z: CONTEXT.position.Z.min(bound.Z),
                        }
                    });
                }
            },
            _ => unreachable!(),
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: match self.ty {
                    BoundType::Lower => BASIS_LOWER_BOUND,
                    BoundType::Upper => BASIS_UPPER_BOUND,
                },
                public: false,
                inputs: vec![
                    InputDefinition {
                        id: bound.into(),
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
impl PreModifier for BasisBound {}

pub trait IntoBasisBound {
    fn basis_bound(self, ty: BoundType, bound: impl IntoExpr) -> Modify;
}

impl<T> IntoBasisBound for T
where
    T: IntoModify,
{
    fn basis_bound(self, ty: BoundType, bound: impl IntoExpr) -> Modify {
        self.modify().push_pre(BasisBound {
            ty,
            bound: bound.expr(),
        })
    }
}
