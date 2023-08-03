use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
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
pub const ELONGATE_INFINITE: FunctionIdentifier =
    FunctionIdentifier::new("elongate_infinite", 1799909959882308009);

pub const DIR_2D: Identifier = Identifier::new("dir_2d", 10994004961423687819);
property!(DIR_2D, DIR_2D_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const DIR_3D: Identifier = Identifier::new("dir_3d", 66909101541205811);
property!(DIR_3D, DIR_3D_PROP, Type::Struct(StructIdentifier(VECTOR3)));

#[derive(Debug, Clone)]
pub struct Elongate {
    pub dir: Expr,
    pub infinite: bool,
}

impl Hash for Elongate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.infinite {
            ELONGATE.uuid().hash(state);
        } else {
            ELONGATE_INFINITE.uuid().hash(state);
        }
        self.dir.hash(state);
        self.infinite.hash(state);
    }
}

impl Domains for Elongate {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for Elongate {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        if self.infinite {
            ELONGATE_INFINITE
        } else {
            ELONGATE
        }
        .specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.dir.clone().into(), input]
    }

    fn functions_impl(
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

        let block = if self.infinite {
            elysian_block! {
                CONTEXT.position = CONTEXT.position - dir.normalize() * #expr;
                return CONTEXT
            }
        } else {
            elysian_block! {
                CONTEXT.position = CONTEXT.position - dir.normalize() * #expr.max(-dir.length()).min(dir.length());
                return CONTEXT
            }
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
    fn elongate(self, dir: elysian_core::ast::expr::Expr, infinite: bool) -> Modify;
}

impl IntoElongate for Field {
    fn elongate(self, dir: elysian_core::ast::expr::Expr, infinite: bool) -> Modify {
        Modify {
            pre_modifiers: vec![Box::new(Elongate { dir, infinite })],
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl IntoElongate for Modify {
    fn elongate(mut self, dir: elysian_core::ast::expr::Expr, infinite: bool) -> Modify {
        self.pre_modifiers
            .push(Box::new(Elongate { dir, infinite }));
        self
    }
}
