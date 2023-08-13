use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    expr::{Expr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{
        vector2, vector3, Block, IntoLiteral, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D,
    },
    module::{
        AsModule, DomainsDyn, ErasedHash, FunctionDefinition, FunctionIdentifier, InputDefinition,
        Module, SpecializationData, CONTEXT,
    },
};

use elysian_proc_macros::elysian_stmt;

use crate::shape::{DynShape, Shape};

pub const BASIS_MIRROR: FunctionIdentifier =
    FunctionIdentifier::new("basis_mirror", 2763069141557531361);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MirrorMode {
    Basis(Expr),
    Axis(Expr),
}

impl ToString for MirrorMode {
    fn to_string(&self) -> String {
        match self {
            MirrorMode::Basis(_) => "basis",
            MirrorMode::Axis(_) => "axis",
        }
        .to_string()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mirror {
    field: DynShape,
    mode: MirrorMode,
}

impl Hash for Mirror {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        BASIS_MIRROR.uuid().hash(state);
        state.write_u64(self.field.erased_hash());
    }
}

impl DomainsDyn for Mirror {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        vec![
            POSITION_2D.into(),
            POSITION_3D.into(),
            GRADIENT_2D.into(),
            GRADIENT_3D.into(),
        ]
        .into_iter()
        .chain(self.field.domains_dyn())
        .collect()
    }
}

impl AsModule for Mirror {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let (position, one) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, vector2([1.0, 1.0]).literal()),
            (false, true) => (POSITION_3D, vector3([1.0, 1.0, 1.0]).literal()),
            _ => panic!("Invalid position domain"),
        };

        let gradient = match (
            spec.contains(&GRADIENT_2D.into()),
            spec.contains(&GRADIENT_3D.into()),
        ) {
            (true, false) => Some(GRADIENT_2D),
            (false, true) => Some(GRADIENT_3D),
            (true, true) => panic!("Invalid gradient domain"),
            (false, false) => None,
        };

        let mut block = Block::default();

        block.push(elysian_stmt! {
            let position = CONTEXT.position
        });

        match &self.mode {
            MirrorMode::Basis(basis) => {
                let basis = elysian_ir::ast::Expr::from(basis.clone());
                block.push(elysian_stmt! {
                    CONTEXT.position =
                        CONTEXT.position
                        * (
                            CONTEXT.position.sign()
                            + (#one - (#basis * 2.0 - #one))
                        ).sign()
                });
            }
            MirrorMode::Axis(axis) => {
                let axis = elysian_ir::ast::Expr::from(axis.clone());
                block.push(elysian_stmt! {
                    if CONTEXT.position.dot(#axis) < 0.0 {
                        CONTEXT.position = CONTEXT.position - (2.0 * CONTEXT.position.dot(#axis)) * #axis
                    }
                });
            }
        }

        let field_module = self.field.module(spec);
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        block.push(elysian_stmt! {
            let CONTEXT = #field_call
        });

        if let Some(gradient) = gradient {
            match &self.mode {
                MirrorMode::Basis(basis) => {
                    let basis = elysian_ir::ast::Expr::from(basis.clone());
                    block.push(elysian_stmt! {
                        CONTEXT.gradient = CONTEXT.gradient * (
                            position.sign()
                            + (#one - (#basis * 2.0 - #one))
                        ).sign()
                    });
                }
                MirrorMode::Axis(axis) => {
                    let axis = elysian_ir::ast::Expr::from(axis.clone());
                    block.push(elysian_stmt! {
                        if position.dot(#axis) < 0.0 {
                            CONTEXT.gradient = CONTEXT.gradient - (2.0 * CONTEXT.gradient.dot(#axis)) * #axis
                        }
                    });
                }
            }
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        field_module.concat(Module::new(
            self,
            spec,
            FunctionDefinition {
                id: FunctionIdentifier::new_dynamic("basis_mirror".into()),
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                }],
                output: CONTEXT.into(),
                block,
            },
        ))
    }
}

#[typetag::serde]
impl Shape for Mirror {}

pub trait IntoMirror {
    fn mirror_basis(self, basis: impl IntoExpr) -> Mirror;
    fn mirror_axis(self, axis: impl IntoExpr) -> Mirror;
}

impl<T> IntoMirror for T
where
    T: 'static + Shape,
{
    fn mirror_basis(self, basis: impl IntoExpr) -> Mirror {
        Mirror {
            field: Box::new(self),
            mode: MirrorMode::Basis(basis.expr()),
        }
    }

    fn mirror_axis(self, axis: impl IntoExpr) -> Mirror {
        Mirror {
            field: Box::new(self),
            mode: MirrorMode::Axis(axis.expr()),
        }
    }
}
