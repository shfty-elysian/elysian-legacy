use std::fmt::Debug;
use std::hash::Hash;

use elysian_core::ir::module::{AsIR, FunctionIdentifier, PropertyIdentifier, CONTEXT};
use elysian_core::{
    ast::expr::Expr as ElysianExpr,
    ir::{
        ast::{POSITION_2D, POSITION_3D},
        module::{Domains, FunctionDefinition, SpecializationData},
    },
};
use elysian_decl_macros::elysian_function;

use crate::modify::{Isosurface, DIR_2D, DIR_3D, ISOSURFACE};

use super::{Line, LINE, RADIUS};

pub const CAPSULE: FunctionIdentifier = FunctionIdentifier::new("capsule", 14339483921749952476);

#[derive(Debug, Clone)]
pub struct Capsule {
    pub dir: ElysianExpr,
    pub radius: ElysianExpr,
}

impl Hash for Capsule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        CAPSULE.uuid().hash(state);
        self.dir.hash(state);
        self.radius.hash(state);
    }
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
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        CAPSULE.specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.dir.clone().into(), self.radius.clone().into(), input]
    }

    fn functions_impl(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let dir = if spec.contains(&POSITION_2D.into()) {
            DIR_2D
        } else if spec.contains(&POSITION_3D.into()) {
            DIR_3D
        } else {
            panic!("No position domain");
        };

        let isosurface = ISOSURFACE.specialize(&spec.filter(Isosurface::domains()));
        let line = LINE.specialize(&spec.filter(Line::domains()));

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
            fn entry_point(dir, RADIUS, CONTEXT) -> CONTEXT {
                return isosurface(RADIUS, line(dir, CONTEXT));
            }
        })
        .collect()
    }
}
