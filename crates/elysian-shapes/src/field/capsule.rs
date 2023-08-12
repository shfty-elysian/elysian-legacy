use std::fmt::Debug;
use std::hash::Hash;

use elysian_core::{
    expr::{Expr as ElysianExpr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D},
    module::{AsIR, Domains, FunctionIdentifier, SpecializationData, CONTEXT},
};
use elysian_proc_macros::elysian_stmt;

use crate::modify::{Isosurface, DIR_2D, DIR_3D};

use super::{Line, RADIUS};

pub const CAPSULE: FunctionIdentifier = FunctionIdentifier::new("capsule", 14339483921749952476);

#[derive(Debug, Clone)]
pub struct Capsule {
    dir: ElysianExpr,
    radius: ElysianExpr,
}

impl Capsule {
    pub fn new(dir: impl IntoExpr, radius: impl IntoExpr) -> Self {
        Capsule {
            dir: dir.expr(),
            radius: radius.expr(),
        }
    }
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
    fn entry_point(&self) -> FunctionIdentifier {
        CAPSULE
    }

    fn arguments(&self, input: elysian_ir::ast::Expr) -> Vec<elysian_ir::ast::Expr> {
        vec![self.dir.clone().into(), self.radius.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_ir::module::FunctionDefinition> {
        let dir = if spec.contains(&POSITION_2D.into()) {
            DIR_2D
        } else if spec.contains(&POSITION_3D.into()) {
            DIR_3D
        } else {
            panic!("No position domain");
        };

        let (_, line_call, line_functions) =
            Line::centered(self.dir.clone()).call(spec, elysian_stmt! { CONTEXT });

        let (_, isosurface_call, isosurface_functions) =
            Isosurface::new(self.radius.clone()).call(spec, line_call);

        line_functions
            .into_iter()
            .chain(isosurface_functions)
            .chain(elysian_function! {
                fn entry_point(dir, RADIUS, CONTEXT) -> CONTEXT {
                    return #isosurface_call;
                }
            })
            .collect()
    }
}
