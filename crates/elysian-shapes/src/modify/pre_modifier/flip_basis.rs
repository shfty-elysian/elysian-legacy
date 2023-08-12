use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{expr::IntoExpr, identifier::Identifier, property_identifier::PropertyIdentifier},
    ir::{
        ast::{
            vector2, vector3, IntoLiteral, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D,
            VECTOR2, VECTOR3,
        },
        module::{
            AsIR, DomainsDyn, DynAsIR, FunctionDefinition, FunctionIdentifier, InputDefinition,
            IntoAsIR, SpecializationData, StructDefinition, StructIdentifier, Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;
use elysian_proc_macros::{elysian_block, elysian_stmt};

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
pub struct FlipBasis {
    pub basis: Expr,
    pub field: DynAsIR,
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

impl AsIR for FlipBasis {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("flip_basis".into())
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.basis.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
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

        let (_, field_call, field_functions) = self.field.call(spec, elysian_stmt! { CONTEXT });

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

        field_functions
            .into_iter()
            .chain([FunctionDefinition {
                id: entry_point.clone(),
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
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoFlipBasis {
    fn flip_basis(self, basis: impl IntoExpr) -> FlipBasis;
}

impl<T> IntoFlipBasis for T
where
    T: IntoAsIR,
{
    fn flip_basis(self, basis: impl IntoExpr) -> FlipBasis {
        FlipBasis {
            basis: basis.expr(),
            field: self.as_ir(),
        }
    }
}
