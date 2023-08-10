use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::expr::IntoExpr,
    ir::{
        ast::Identifier,
        module::{
            AsIR, FunctionIdentifier, NumericType, PropertyIdentifier,
            SpecializationData, Type, CONTEXT,
        },
        module::{Domains, IntoRead},
    },
    property,
};
use elysian_decl_macros::elysian_function;

use crate::modify::Isosurface;

use super::Point;

pub const CIRCLE: FunctionIdentifier = FunctionIdentifier::new("circle", 15738477621793375359);

pub const RADIUS: Identifier = Identifier::new("radius", 213754678517975478);
property!(RADIUS, RADIUS_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub struct Circle {
    radius: elysian_core::ast::expr::Expr,
}

impl Circle {
    pub fn new(radius: impl IntoExpr) -> Self {
        Circle {
            radius: radius.expr(),
        }
    }
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
    fn entry_point(&self) -> FunctionIdentifier {
        CIRCLE
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.radius.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let (_, point_call, point_functions) = Point.call(spec, PropertyIdentifier(CONTEXT).read());

        let (_, isosurface_entry, isosurface_functions) =
            Isosurface::new(self.radius.clone()).prepare(spec);

        point_functions
            .into_iter()
            .chain(isosurface_functions)
            .chain(elysian_function! {
                fn entry_point(RADIUS, CONTEXT) -> CONTEXT {
                    return #isosurface_entry(RADIUS, #point_call);
                }
            })
            .collect()
    }
}
