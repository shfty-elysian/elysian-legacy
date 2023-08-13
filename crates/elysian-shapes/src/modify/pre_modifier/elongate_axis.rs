use std::{fmt::Debug, hash::Hash};

use crate::modify::{IntoModify, Modify, PreModifier};
use elysian_core::{
    expr::IntoExpr, identifier::Identifier, property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, VECTOR2, VECTOR3},
    module::{
        AsModule, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition, Module,
        SpecializationData, StructIdentifier, Type, CONTEXT,
    },
    property,
};

use elysian_core::expr::Expr;
use elysian_proc_macros::{elysian_block, elysian_expr};

pub const ELONGATE_AXIS: FunctionIdentifier =
    FunctionIdentifier::new("elongate_axis", 1022510703206415324);

pub const DIR_2D: Identifier = Identifier::new("dir_2d", 10994004961423687819);
property!(DIR_2D, DIR_2D_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const DIR_3D: Identifier = Identifier::new("dir_3d", 66909101541205811);
property!(DIR_3D, DIR_3D_PROP, Type::Struct(StructIdentifier(VECTOR3)));

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClampMode {
    None,
    Dir,
    Zero,
}

impl ToString for ClampMode {
    fn to_string(&self) -> String {
        match self {
            ClampMode::None => "none",
            ClampMode::Dir => "dir",
            ClampMode::Zero => "zero",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElongateAxis {
    pub dir: Expr,
    pub clamp_neg: ClampMode,
    pub clamp_pos: ClampMode,
}

impl Hash for ElongateAxis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ELONGATE_AXIS.uuid().hash(state);
        self.dir.hash(state);
        self.clamp_neg.hash(state);
        self.clamp_pos.hash(state);
    }
}

impl Domains for ElongateAxis {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsModule for ElongateAxis {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let (position, dir) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, DIR_2D),
            (false, true) => (POSITION_3D, DIR_3D),
            _ => panic!("Invalid position domain"),
        };

        let expr = elysian_expr! {
            CONTEXT.position.dot(dir.normalize())
        };

        let expr = match self.clamp_neg {
            ClampMode::None => expr,
            ClampMode::Dir => elysian_expr! {
                #expr.max(-dir.length())
            },
            ClampMode::Zero => elysian_expr! {
                #expr.max(0.0)
            },
        };

        let expr = match self.clamp_pos {
            ClampMode::None => expr,
            ClampMode::Dir => elysian_expr! {
                #expr.min(dir.length())
            },
            ClampMode::Zero => elysian_expr! {
                #expr.min(0.0)
            },
        };

        let block = elysian_block! {
            CONTEXT.position = CONTEXT.position - dir.normalize() * #expr;
            return CONTEXT
        };

        let elongate_axis = ELONGATE_AXIS
            .concat(&FunctionIdentifier::new_dynamic(
                self.clamp_neg.to_string().into(),
            ))
            .concat(&FunctionIdentifier::new_dynamic(
                self.clamp_pos.to_string().into(),
            ));

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: elongate_axis,
                public: false,
                inputs: vec![
                    InputDefinition {
                        id: dir.clone().into(),
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
        .with_args([self.dir.clone().into()])
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl PreModifier for ElongateAxis {}

pub trait IntoElongateAxis {
    fn elongate_axis(
        self,
        dir: impl IntoExpr,
        clamp_neg: ClampMode,
        clamp_pos: ClampMode,
    ) -> Modify;
}

impl<T> IntoElongateAxis for T
where
    T: IntoModify,
{
    fn elongate_axis(
        self,
        dir: impl IntoExpr,
        clamp_neg: ClampMode,
        clamp_pos: ClampMode,
    ) -> Modify {
        self.modify().push_pre(ElongateAxis {
            dir: dir.expr(),
            clamp_neg,
            clamp_pos,
        })
    }
}
