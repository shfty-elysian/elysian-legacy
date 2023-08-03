use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{expr::Expr, field::Field, modify::Modify},
    ir::{
        ast::{Block, Identifier, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, HashIR, InputDefinition,
            PropertyIdentifier, SpecializationData, StructIdentifier, Type, CONTEXT,
        },
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
pub enum BoundType {
    Lower,
    Upper,
}

#[derive(Debug, Clone)]
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

        state.write_u64(self.bound.hash_ir());
    }
}

impl Domains for BasisBound {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

impl AsIR for BasisBound {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        match self.ty {
            BoundType::Lower => BASIS_LOWER_BOUND,
            BoundType::Upper => BASIS_UPPER_BOUND,
        }
        .specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.bound.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (position, bound) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, BOUND_2D)
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, BOUND_3D)
        } else {
            panic!("No position domain")
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

        vec![FunctionDefinition {
            id: entry_point.clone(),
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
        }]
    }
}

pub trait IntoBasisBound {
    fn basis_bound(self, ty: BoundType, bound: Expr) -> Modify;
}

impl IntoBasisBound for Field {
    fn basis_bound(self, ty: BoundType, bound: Expr) -> Modify {
        Modify {
            pre_modifiers: vec![Box::new(BasisBound { ty, bound })],
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl IntoBasisBound for Modify {
    fn basis_bound(mut self, ty: BoundType, bound: Expr) -> Modify {
        self.pre_modifiers.push(Box::new(BasisBound { ty, bound }));
        self
    }
}
