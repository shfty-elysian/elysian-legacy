use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ir::{
        as_ir::{AsIR, Domains},
        ast::Identifier,
        module::{
            FunctionDefinition, FunctionIdentifier, NumericType,
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

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: elysian_core::ast::expr::Expr,
}

impl Hash for Circle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        CIRCLE.uuid().hash(state);
        self.radius.hash(state);
    }
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
        let point = POINT.specialize(&spec.filter(Point::domains()));
        let isosurface = ISOSURFACE.specialize(&spec.filter(Isosurface::domains()));
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
                    return isosurface(RADIUS, point(CONTEXT));
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
