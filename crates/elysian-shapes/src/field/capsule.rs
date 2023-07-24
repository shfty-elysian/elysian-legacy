use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_core::ast::modify::CONTEXT_STRUCT;
use elysian_core::ir::ast::IntoBlock;
use elysian_core::{
    ast::expr::Expr,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, CONTEXT, POSITION_2D, POSITION_3D},
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

use crate::modify::{Isosurface, DIR_2D, DIR_3D, ISOSURFACE};

use super::{Line, LINE, RADIUS};

pub const CAPSULE: Identifier = Identifier::new("capsule", 14339483921749952476);

pub struct Capsule {
    pub dir: Expr,
    pub radius: Expr,
}

impl Debug for Capsule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capsule")
            .field("dir", &self.dir)
            .field("radius", &self.radius)
            .finish()
    }
}

impl Clone for Capsule {
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            radius: self.radius.clone(),
        }
    }
}

impl Hash for Capsule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.radius.hash(state);
    }
}

impl Domains for Capsule {
    fn domains() -> Vec<Identifier> {
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
        let dir = if spec.contains(POSITION_2D.id()) {
            DIR_2D
        } else if spec.contains(POSITION_3D.id()) {
            DIR_3D
        } else {
            panic!("No position domain");
        };

        let isosurface_spec = spec.filter(Isosurface::domains());
        let line_spec = spec.filter(Line::domains());

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
        .chain(FunctionDefinition {
            id: CAPSULE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: dir.clone(),
                    mutable: false,
                },
                InputDefinition {
                    prop: RADIUS,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                },
            ],
            output: CONTEXT_STRUCT.clone(),
            block: ISOSURFACE
                .specialize(&isosurface_spec)
                .call([
                    RADIUS.read(),
                    LINE.specialize(&line_spec)
                        .call([dir.read(), CONTEXT.read()]),
                ])
                .output()
                .block(),
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
