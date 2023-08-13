use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    expr::{Expr, IntoExpr},
    identifier::Identifier,
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    module::{
        AsModule, Domains, DomainsDyn, FunctionIdentifier, Module, NumericType, SpecializationData,
        Type, CONTEXT,
    },
    property,
};
use elysian_proc_macros::elysian_stmt;

use crate::{
    modify::{Isosurface, Manifold},
    shape::Shape,
};

use super::{Circle, RADIUS};

pub const RING: FunctionIdentifier = FunctionIdentifier::new("ring", 18972348581943461950);

pub const WIDTH: Identifier = Identifier::new("width", 2742125101201765597);
property!(WIDTH, WIDTH_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl AsModule for Ring {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let circle = Circle::new(self.radius.clone());
        let circle_module = circle.module(&spec.filter(circle.domains_dyn()));
        let circle_call = circle_module.call(elysian_stmt! { CONTEXT });

        let manifold = Manifold;
        let manifold_module = manifold.module(&spec.filter(manifold.domains_dyn()));
        let manifold_call = manifold_module.call(circle_call);

        let isosurface = Isosurface::new(self.width.clone());
        let isosurface_module = isosurface.module(&spec.filter(isosurface.domains_dyn()));
        let isosurface_call = isosurface_module.call(manifold_call);

        circle_module
            .concat(manifold_module)
            .concat(isosurface_module)
            .concat(
                Module::new(
                    self,
                    spec,
                    elysian_function! {
                        fn RING(RADIUS, WIDTH, CONTEXT) -> CONTEXT {
                            return #isosurface_call;
                        }
                    },
                )
                .with_args([self.radius.clone().into(), self.width.clone().into()]),
            )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Ring {}
