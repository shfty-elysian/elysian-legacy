use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    module::{AsIR, Domains, FunctionIdentifier, NumericType, SpecializationData, Type, CONTEXT},
    property,
};
use elysian_proc_macros::elysian_stmt;

use crate::modify::{Isosurface, Manifold};

use super::{Circle, RADIUS};

pub const RING: FunctionIdentifier = FunctionIdentifier::new("ring", 18972348581943461950);

pub const WIDTH: Identifier = Identifier::new("width", 2742125101201765597);
property!(WIDTH, WIDTH_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub struct Ring {
    radius: Expr,
    width: Expr,
}

impl Ring {
    pub fn new(radius: impl IntoExpr, width: impl IntoExpr) -> Self {
        Ring {
            radius: radius.expr(),
            width: width.expr(),
        }
    }
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
    fn entry_point(&self) -> FunctionIdentifier {
        RING
    }

    fn arguments(&self, input: elysian_ir::ast::Expr) -> Vec<elysian_ir::ast::Expr> {
        vec![self.radius.clone().into(), self.width.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_ir::module::FunctionDefinition> {
        let (_, circle_call, circle_functions) =
            Circle::new(self.radius.clone()).call(spec, elysian_stmt! { CONTEXT });

        let (_, manifold_call, manifold_functions) = Manifold.call(spec, circle_call);

        let (_, isosurface_call, isosurface_functions) =
            Isosurface::new(self.width.clone()).call(spec, manifold_call);

        circle_functions
            .into_iter()
            .chain(manifold_functions)
            .chain(isosurface_functions)
            .chain(elysian_function! {
                fn entry_point(RADIUS, WIDTH, CONTEXT) -> CONTEXT {
                    return #isosurface_call;
                }
            })
            .collect()
    }
}
