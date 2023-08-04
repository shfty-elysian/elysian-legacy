use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        ast::Identifier,
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, NumericType, PropertyIdentifier,
            SpecializationData, Type, CONTEXT,
        },
    },
    property,
};
use elysian_decl_macros::elysian_function;

use crate::modify::{Isosurface, Manifold};

use super::{Circle, RADIUS};

pub const RING: FunctionIdentifier = FunctionIdentifier::new("ring", 18972348581943461950);

pub const WIDTH: Identifier = Identifier::new("width", 2742125101201765597);
property!(WIDTH, WIDTH_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub struct Ring {
    pub radius: Expr,
    pub width: Expr,
}

impl Hash for Ring {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        RING.uuid().hash(state);
        self.radius.hash(state);
        self.width.hash(state);
    }
}

impl Domains for Ring {
    fn domains() -> Vec<PropertyIdentifier> {
        Circle::domains()
            .into_iter()
            .chain(Manifold::domains())
            .chain(Isosurface::domains())
            .collect()
    }
}

impl AsIR for Ring {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        RING.specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.radius.clone().into(), self.width.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let isosurface = Isosurface {
            dist: self.width.clone(),
        };
        let (_, isosurface_entry, isosurface_functions) = isosurface.prepare(spec);

        let manifold = Manifold;
        let (_, manifold_entry, manifold_functions) = manifold.prepare(spec);

        let circle = Circle {
            radius: self.radius.clone(),
        };
        let (_, circle_entry, circle_functions) = circle.prepare(spec);

        circle_functions
            .into_iter()
            .chain(manifold_functions)
            .chain(isosurface_functions)
            .chain(elysian_function! {
                fn entry_point(RADIUS, WIDTH, CONTEXT) -> CONTEXT {
                    return isosurface_entry(WIDTH, manifold_entry(circle_entry(RADIUS, CONTEXT)));
                }
            })
            .collect()
    }
}
