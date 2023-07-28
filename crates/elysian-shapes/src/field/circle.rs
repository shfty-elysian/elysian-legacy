use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ir::{
        as_ir::{AsIR, Domains},
        ast::Identifier,
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, NumericType,
            PropertyIdentifier, SpecializationData, Type, CONTEXT,
        },
    },
    property,
};
use elysian_decl_macros::elysian_function;

use crate::modify::{Isosurface, ISOSURFACE};

use super::{Point, POINT};

pub const CIRCLE: FunctionIdentifier = FunctionIdentifier::new("circle", 15738477621793375359);

pub const RADIUS: Identifier = Identifier::new("radius", 213754678517975478);
property!(RADIUS, RADIUS_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone, Hash)]
pub struct Circle {
    pub radius: elysian_core::ast::expr::Expr,
}

impl Domains for Circle {
    fn domains() -> Vec<PropertyIdentifier> {
        Point::domains()
            .into_iter()
            .chain(Isosurface::domains())
            .collect()
    }
}

impl AsIR for Circle {
    fn functions_impl(
        &self,
        spec: &SpecializationData,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let point_spec = spec.filter(Point::domains());
        let isosurface_spec = spec.filter(Isosurface::domains());

        let point_func = POINT.specialize(&point_spec);
        let isosurface_func = ISOSURFACE.specialize(&isosurface_spec);

        let circle = CIRCLE.specialize(spec);

        Point
            .functions(spec)
            .into_iter()
            .chain(
                Isosurface {
                    dist: self.radius.clone(),
                }
                .functions(spec),
            )
            .chain(elysian_function! {
                fn circle(RADIUS, CONTEXT) -> CONTEXT {
                    return isosurface_func(RADIUS, point_func(CONTEXT));
                }
            })
            .collect()
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        CIRCLE
            .specialize(spec)
            .call([self.radius.clone().into(), input])
    }
}
