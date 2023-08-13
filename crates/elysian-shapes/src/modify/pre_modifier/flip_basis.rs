use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    expr::IntoExpr, identifier::Identifier, property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{
        vector2, vector3, IntoLiteral, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D, VECTOR2,
        VECTOR3,
    },
    module::{
        AsModule, DomainsDyn, FunctionDefinition, FunctionIdentifier, HashIR, InputDefinition,
        Module, SpecializationData, StructIdentifier, Type, CONTEXT,
    },
    property,
};

use elysian_core::expr::Expr;
use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::shape::{DynShape, IntoShape};

pub const FLIP_2D: Identifier = Identifier::new("flip_2d", 4082005642022253885);
property!(
    FLIP_2D,
    FLIP_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const FLIP_3D: Identifier = Identifier::new("flip_3d", 6664639271221372354);
property!(
    FLIP_3D,
    FLIP_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FlipBasis {
    pub basis: Expr,
    pub field: DynShape,
}

impl Hash for FlipBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.basis.hash(state);
        state.write_u64(self.field.hash_ir());
    }
}

impl DomainsDyn for FlipBasis {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.field
            .domains_dyn()
            .into_iter()
            .chain([
                POSITION_2D.into(),
                POSITION_3D.into(),
                GRADIENT_2D.into(),
                GRADIENT_3D.into(),
            ])
            .collect()
    }
}

impl AsModule for FlipBasis {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let (position, flip, one) = if spec.contains(&POSITION_2D.into()) {
            (POSITION_2D, FLIP_2D, vector2([1.0, 1.0]).literal())
        } else if spec.contains(&POSITION_3D.into()) {
            (POSITION_3D, FLIP_3D, vector3([1.0, 1.0, 1.0]).literal())
        } else {
            panic!("No position domain")
        };

        let gradient = match (
            spec.contains(&GRADIENT_2D.into()),
            spec.contains(&GRADIENT_3D.into()),
        ) {
            (true, false) => Some(GRADIENT_2D),
            (false, true) => Some(GRADIENT_3D),
            (false, false) => None,
            _ => panic!("Invalid Gradient Domain"),
        };

        let field_module = self.field.module_impl(spec);
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let mut block = elysian_block! {
            CONTEXT.position = CONTEXT.position * ((#one - flip) * 2.0 - #one).sign();
            CONTEXT = #field_call;
        };

        if let Some(gradient) = gradient {
            block.push(elysian_stmt! {
                CONTEXT.gradient = CONTEXT.gradient * ((#one - flip) * 2.0 - #one).sign()
            });
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        field_module.concat(
            Module::new(
                self,
                spec,
                FunctionDefinition {
                    id: FunctionIdentifier::new_dynamic("flip_basis".into()),
                    public: false,
                    inputs: vec![
                        InputDefinition {
                            id: flip.into(),
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
            .with_args([self.basis.clone().into()]),
        )
    }
}

pub trait IntoFlipBasis {
    fn flip_basis(self, basis: impl IntoExpr) -> FlipBasis;
}

impl<T> IntoFlipBasis for T
where
    T: IntoShape,
{
    fn flip_basis(self, basis: impl IntoExpr) -> FlipBasis {
        FlipBasis {
            basis: basis.expr(),
            field: self.shape(),
        }
    }
}
