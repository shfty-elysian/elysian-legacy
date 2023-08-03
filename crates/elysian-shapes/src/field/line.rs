use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        ast::{POSITION_2D, POSITION_3D},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
            SpecializationData, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;

use crate::modify::{Elongate, DIR_2D, DIR_3D, ELONGATE};

use super::{Point, POINT};

pub const LINE: FunctionIdentifier = FunctionIdentifier::new("line", 14339483921749952476);

#[derive(Debug, Clone)]
pub struct Line {
    pub dir: Expr,
}

impl Hash for Line {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        LINE.uuid().hash(state);
        self.dir.hash(state);
    }
}

impl Domains for Line {
    fn domains() -> Vec<PropertyIdentifier> {
        Point::domains()
            .into_iter()
            .chain(Elongate::domains())
            .collect()
    }
}

impl AsIR for Line {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        LINE.specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.dir.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let dir = if spec.contains(&POSITION_2D.into()) {
            DIR_2D
        } else if spec.contains(&POSITION_3D.into()) {
            DIR_3D
        } else {
            panic!("No position domain set")
        };

        let point = POINT.specialize(&spec.filter(Point::domains()));
        let elongate = ELONGATE.specialize(&spec.filter(Elongate::domains()));

        Point
            .functions_internal(spec)
            .into_iter()
            .chain(
                Elongate {
                    dir: self.dir.clone(),
                    infinite: false,
                }
                .functions_internal(spec),
            )
            .chain(elysian_function! {
                fn entry_point(dir, CONTEXT) -> CONTEXT {
                    return point(elongate(dir, CONTEXT));
                }
            })
            .collect()
    }
}
