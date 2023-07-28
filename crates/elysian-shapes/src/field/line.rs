use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{POSITION_2D, POSITION_3D},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, PropertyIdentifier,
            SpecializationData, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;

use crate::modify::{Elongate, DIR_2D, DIR_3D, ELONGATE};

use super::{Point, POINT};

pub const LINE: FunctionIdentifier = FunctionIdentifier::new("line", 14339483921749952476);

#[derive(Debug, Clone, Hash)]
pub struct Line {
    pub dir: Expr,
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
    fn functions_impl(
        &self,
        spec: &SpecializationData,
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
        let line = LINE.specialize(spec);

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
            .chain(elysian_function! {
                fn line(dir, CONTEXT) -> CONTEXT {
                    return point(elongate(dir, CONTEXT));
                }
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
