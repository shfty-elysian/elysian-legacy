use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
    property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{
        vector2, vector3, IntoLiteral, DISTANCE, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y,
        Z,
    },
    module::{
        DomainsDyn, FunctionDefinition, FunctionIdentifier, InputDefinition, Module,
        SpecializationData, StructIdentifier, Type, CONTEXT,
    },
    property,
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::{
    shape::IntoShape,
    wrap::{Wrap, Wrapper},
};

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

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElongateBasis {
    pub extent: Expr,
}

impl DomainsDyn for ElongateBasis {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        vec![POSITION_2D.into(), POSITION_3D.into()]
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wrapper for ElongateBasis {
    fn module(
        &self,
        spec: &SpecializationData,
        field_call: elysian_ir::ast::Expr,
    ) -> elysian_ir::module::Module {
        let (position, extent, zero) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, EXTENT_2D, vector2([0.0, 0.0]).literal()),
            (false, true) => (POSITION_3D, EXTENT_3D, vector3([0.0, 0.0, 0.0]).literal()),
            _ => panic!("Invalid position domain"),
        };

        let extent_expr = elysian_ir::ast::Expr::from(self.extent.clone());

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

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: FunctionIdentifier::new_dynamic("elongate_basis".into()),
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
            },
        )
        .with_args([self.extent.clone().into()])
    }
}

pub trait IntoElongateBasis {
    fn elongate_basis(self, dir: impl IntoExpr) -> Wrap;
}

impl<T> IntoElongateBasis for T
where
    T: IntoShape,
{
    fn elongate_basis(self, dir: impl IntoExpr) -> Wrap {
        Wrap::new(ElongateBasis { extent: dir.expr() }, self)
    }
}
