use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::modify::{IntoModify, Modify},
    ir::{
        ast::{Identifier, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition,
            PropertyIdentifier, SpecializationData, StructIdentifier, Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;
use elysian_proc_macros::{elysian_block, elysian_expr};

pub const ELONGATE: FunctionIdentifier = FunctionIdentifier::new("elongate", 1022510703206415324);

pub const DIR_2D: Identifier = Identifier::new("dir_2d", 10994004961423687819);
property!(DIR_2D, DIR_2D_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const DIR_3D: Identifier = Identifier::new("dir_3d", 66909101541205811);
property!(DIR_3D, DIR_3D_PROP, Type::Struct(StructIdentifier(VECTOR3)));

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub struct Elongate {
    pub dir: Expr,
    pub clamp_neg: ClampMode,
    pub clamp_pos: ClampMode,
}

impl Hash for Elongate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ELONGATE.uuid().hash(state);
        self.clamp_neg.hash(state);
        self.clamp_pos.hash(state);
        self.dir.hash(state);
        self.clamp_pos.hash(state);
    }
}

impl Domains for Elongate {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for Elongate {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        ELONGATE
            .concat(&FunctionIdentifier::new_dynamic(
                self.clamp_neg.to_string().into(),
            ))
            .concat(&FunctionIdentifier::new_dynamic(
                self.clamp_pos.to_string().into(),
            ))
            .specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.dir.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (position, dir) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, DIR_2D)
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, DIR_3D)
        } else {
            panic!("No position domain");
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

        vec![FunctionDefinition {
            id: entry_point.clone(),
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
        }]
    }
}

pub trait IntoElongate {
    fn elongate(
        self,
        dir: elysian_core::ast::expr::Expr,
        clamp_neg: ClampMode,
        clamp_pos: ClampMode,
    ) -> Modify;
}

impl<T> IntoElongate for T
where
    T: IntoModify,
{
    fn elongate(
        self,
        dir: elysian_core::ast::expr::Expr,
        clamp_neg: ClampMode,
        clamp_pos: ClampMode,
    ) -> Modify {
        let mut m = self.modify();
        m.pre_modifiers.push(Box::new(Elongate {
            dir,
            clamp_neg,
            clamp_pos,
        }));
        m
    }
}
