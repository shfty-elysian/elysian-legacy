use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ir::{
        ast::Identifier,
        module::Domains,
        module::{
            AsIR, FunctionDefinition, FunctionIdentifier, NumericType, PropertyIdentifier,
            SpecializationData, Type, CONTEXT,
        },
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
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        CIRCLE.specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.radius.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let point = Point;
        let (_, point_entry, point_functions) = point.prepare(spec);

        let isosurface = Isosurface {
            dist: self.radius.clone(),
        };
        let (_, isosurface_entry, isosurface_functions) = isosurface.prepare(spec);

        point_functions
            .into_iter()
            .chain(isosurface_functions)
            .chain(elysian_function! {
                fn entry_point(RADIUS, CONTEXT) -> CONTEXT {
                    return isosurface_entry(RADIUS, point_entry(CONTEXT));
                }
            })
            .collect()
    }
}
