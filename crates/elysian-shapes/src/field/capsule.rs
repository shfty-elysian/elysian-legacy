use std::fmt::Debug;
use std::hash::Hash;

use elysian_core::{
    expr::{Expr as ElysianExpr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData, CONTEXT},
};

use crate::{
    modify::{Isosurface, DIR_2D, DIR_3D},
    shape::Shape,
};

use super::{Line, RADIUS};

pub const CAPSULE: FunctionIdentifier = FunctionIdentifier::new("capsule", 14339483921749952476);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl AsModule for Capsule {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let dir = if spec.contains(&POSITION_2D.into()) {
            DIR_2D
        } else if spec.contains(&POSITION_3D.into()) {
            DIR_3D
        } else {
            panic!("No position domain");
        };

        let line_module = Line::centered(self.dir.clone()).module(spec);
        let line_entry_point = &line_module.entry_point;

        let isosurface_module = Isosurface::new(self.radius.clone()).module(spec);
        let isosurface_entry_point = &isosurface_module.entry_point;

        let capsule_module = Module::new(
            self,
            spec,
            elysian_function! {
                fn CAPSULE(dir, RADIUS, CONTEXT) -> CONTEXT {
                    return #isosurface_entry_point(RADIUS, #line_entry_point(dir, CONTEXT));
                }
            },
        )
        .with_args([self.dir.clone().into(), self.radius.clone().into()]);

        line_module.concat(isosurface_module).concat(capsule_module)
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Capsule {}
