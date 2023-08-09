use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::expr::IntoExpr,
    ir::{
        ast::{
            vector2, vector3, Identifier, IntoLiteral, DISTANCE, POSITION_2D, POSITION_3D, VECTOR2,
            VECTOR3, X, Y, Z,
        },
        module::{
            AsIR, DomainsDyn, DynAsIR, FunctionDefinition, FunctionIdentifier, InputDefinition,
            IntoAsIR, PropertyIdentifier, SpecializationData, StructDefinition, StructIdentifier,
            Type, CONTEXT,
        },
    },
    property,
};

use elysian_core::ast::expr::Expr;
use elysian_proc_macros::{elysian_block, elysian_expr, elysian_stmt};

pub const EXTENT_2D: Identifier = Identifier::new("extent_2d", 9222786191981609495);
property!(
    EXTENT_2D,
    EXTENTDIR_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const EXTENT_3D: Identifier = Identifier::new("extent_3d", 3599864941396865140);
property!(
    EXTENT_3D,
    EXTENT_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

#[derive(Debug)]
pub struct ElongateBasis {
    pub field: DynAsIR,
    pub extent: Expr,
}

impl Hash for ElongateBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
        self.extent.hash(state);
    }
}

impl DomainsDyn for ElongateBasis {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.field
            .domains_dyn()
            .into_iter()
            .chain([POSITION_2D.into(), POSITION_3D.into()])
            .collect()
    }
}

impl AsIR for ElongateBasis {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("elongate_basis".into())
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.extent.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (position, extent, zero) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, EXTENT_2D, vector2([0.0, 0.0]).literal()),
            (false, true) => (POSITION_3D, EXTENT_3D, vector3([0.0, 0.0, 0.0]).literal()),
            _ => panic!("Invalid position domain"),
        };

        let (_, field_call, field_functions) = self.field.call(spec, elysian_expr! { CONTEXT });

        let extent_expr = elysian_core::ir::ast::Expr::from(self.extent.clone());

        let pos_clamped = match &position {
            p if *p == POSITION_2D => elysian_stmt! {
                position.X.max(position.Y).min(0.0)
            },
            p if *p == POSITION_3D => elysian_stmt! {
                position.X.max(position.Y).max(position.Z).min(0.0)
            },
            _ => unreachable!(),
        };

        let block = elysian_block! {
            let position = CONTEXT.position.abs() - #extent_expr;
            CONTEXT.position = CONTEXT.position.sign() * position.max(#zero);
            CONTEXT = #field_call;
            CONTEXT.DISTANCE = CONTEXT.DISTANCE + #pos_clamped;
            return CONTEXT
        };

        field_functions
            .into_iter()
            .chain([FunctionDefinition {
                id: entry_point.clone(),
                public: false,
                inputs: vec![
                    InputDefinition {
                        id: extent.clone().into(),
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

pub trait IntoElongateBasis {
    fn elongate_basis(self, dir: impl IntoExpr) -> ElongateBasis;
}

impl<T> IntoElongateBasis for T
where
    T: IntoAsIR,
{
    fn elongate_basis(self, dir: impl IntoExpr) -> ElongateBasis {
        ElongateBasis {
            field: self.as_ir(),
            extent: dir.expr(),
        }
    }
}
