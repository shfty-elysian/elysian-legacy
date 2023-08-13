use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    expr::IntoExpr,
    identifier::Identifier,
    property_identifier::{IntoPropertyIdentifier, PropertyIdentifier},
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    module::{AsModule, Domains, IntoRead, Module},
    module::{FunctionIdentifier, NumericType, SpecializationData, Type, CONTEXT},
    property,
};
use elysian_proc_macros::elysian_stmt;

use crate::{modify::Isosurface, shape::Shape};

use super::Point;

pub const CIRCLE: FunctionIdentifier = FunctionIdentifier::new("circle", 15738477621793375359);

pub const RADIUS: Identifier = Identifier::new("radius", 213754678517975478);
property!(RADIUS, RADIUS_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    radius: elysian_core::expr::Expr,
}

impl Circle {
    pub fn new(radius: impl IntoExpr) -> Self {
        Circle {
            radius: radius.expr(),
        }
    }

    pub fn radius(&self) -> &elysian_core::expr::Expr {
        &self.radius
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

impl AsModule for Circle {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let point_module = Point.module(&spec.filter(Point::domains()));
        let point_call = point_module.call(elysian_stmt! { CONTEXT });

        let isosurface_module =
            Isosurface::new(self.radius.clone()).module(&spec.filter(Isosurface::domains()));
        let isosurface_call = isosurface_module
            .entry_point
            .call([RADIUS.prop().read(), CONTEXT.prop().read()]);

        point_module
            .concat(isosurface_module)
            .concat(Module::new(
                self,
                spec,
                elysian_function! {
                    fn CIRCLE(RADIUS, CONTEXT) -> CONTEXT {
                        let CONTEXT = #point_call;
                        let CONTEXT = #isosurface_call;
                        return CONTEXT;
                    }
                },
            ))
            .with_args([self.radius().clone().into()])
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Circle {}
