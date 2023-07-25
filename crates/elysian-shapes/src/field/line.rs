use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, IntoBlock, CONTEXT, POSITION_2D, POSITION_3D},
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

use crate::modify::{Elongate, DIR_2D, DIR_3D, ELONGATE};

use super::{Point, POINT};

pub const LINE: Identifier = Identifier::new("line", 14339483921749952476);

pub struct Line {
    pub dir: Expr,
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line").field("dir", &self.dir).finish()
    }
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
        }
    }
}

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
    }
}

impl Domains for Line {
    fn domains() -> Vec<Identifier> {
        Point::domains()
            .into_iter()
            .chain(Elongate::domains())
            .collect()
    }
}

impl AsIR for Line {
    fn functions_impl(
        &self,
        spec: &SpecializationData,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let dir = if spec.contains(&POSITION_2D) {
            DIR_2D
        } else if spec.contains(&POSITION_3D) {
            DIR_3D
        } else {
            panic!("No position domain set")
        };

        let point_spec = spec.filter(Point::domains());
        let elongate_spec = spec.filter(Elongate::domains());

        Point
            .functions(spec)
            .into_iter()
            .chain(
                Elongate {
                    dir: self.dir.clone(),
                    infinite: false,
                }
                .functions(spec),
            )
            .chain(FunctionDefinition {
                id: LINE.specialize(spec),
                public: false,
                inputs: vec![
                    InputDefinition {
                        id: dir.clone(),
                        mutable: false,
                    },
                    InputDefinition {
                        id: CONTEXT,
                        mutable: false,
                    },
                ],
                output: CONTEXT,
                block: POINT
                    .specialize(&point_spec)
                    .call([ELONGATE
                        .specialize(&elongate_spec)
                        .call([dir.read(), CONTEXT.read()])])
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
        LINE.specialize(spec).call([self.dir.clone().into(), input])
    }
}
