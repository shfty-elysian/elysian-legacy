use std::fmt::Debug;
use std::hash::Hash;

use elysian_core::ir::module::{FunctionIdentifier, PropertyIdentifier, CONTEXT};
use elysian_core::{
    ast::expr::Expr as ElysianExpr,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{POSITION_2D, POSITION_3D},
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};
use elysian_decl_macros::elysian_function;

use crate::modify::{Isosurface, DIR_2D, DIR_3D, ISOSURFACE};

use super::{Line, LINE, RADIUS};

pub const CAPSULE: FunctionIdentifier = FunctionIdentifier::new("capsule", 14339483921749952476);

#[derive(Debug, Clone, Hash)]
pub struct Capsule {
    pub dir: ElysianExpr,
    pub radius: ElysianExpr,
}

impl Domains for Capsule {
    fn domains() -> Vec<PropertyIdentifier> {
        Line::domains()
            .into_iter()
            .chain(Isosurface::domains())
            .collect()
    }
}

impl AsIR for Capsule {
    fn functions_impl(
        &self,
        spec: &SpecializationData,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let dir = if spec.contains(&POSITION_2D.into()) {
            DIR_2D
        } else if spec.contains(&POSITION_3D.into()) {
            DIR_3D
        } else {
            panic!("No position domain");
        };

        let isosurface_spec = spec.filter(Isosurface::domains());
        let line_spec = spec.filter(Line::domains());

        let isosurface_func = ISOSURFACE.specialize(&isosurface_spec);
        let line_func = LINE.specialize(&line_spec);

        let capsule = CAPSULE.specialize(spec);

        Line {
            dir: self.dir.clone(),
        }
        .functions(spec)
        .into_iter()
        .chain(
            Isosurface {
                dist: self.radius.clone(),
            }
            .functions(spec),
        )
        .chain(elysian_function! {
            fn capsule(dir, RADIUS, CONTEXT) -> CONTEXT {
                return isosurface_func(RADIUS, line_func(dir, CONTEXT));
            }
        })
        .collect()
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        CAPSULE
            .specialize(spec)
            .call([self.dir.clone().into(), self.radius.clone().into(), input])
    }
}
