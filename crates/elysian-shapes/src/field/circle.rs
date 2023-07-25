use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, IntoBlock, Property, CONTEXT},
        module::{
            FunctionDefinition, InputDefinition, NumericType, SpecializationData, Type, PROPERTIES,
        },
    },
};

use crate::modify::{Isosurface, ISOSURFACE};

use super::{Point, POINT};

pub const CIRCLE: Identifier = Identifier::new("circle", 15738477621793375359);

pub const RADIUS: Identifier = Identifier::new("radius", 213754678517975478);
#[linkme::distributed_slice(PROPERTIES)]
static RADIUS_PROP: Property = Property {
    id: RADIUS,
    ty: Type::Number(NumericType::Float),
};

pub struct Circle {
    pub radius: Expr,
}

impl Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Circle")
            .field("radius", &self.radius)
            .finish()
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Self {
            radius: self.radius.clone(),
        }
    }
}

impl Hash for Circle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
    }
}

impl Domains for Circle {
    fn domains() -> Vec<Identifier> {
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

        Point
            .functions(spec)
            .into_iter()
            .chain(
                Isosurface {
                    dist: self.radius.clone(),
                }
                .functions(spec),
            )
            .chain(FunctionDefinition {
                id: CIRCLE.specialize(spec),
                public: false,
                inputs: vec![
                    InputDefinition {
                        id: RADIUS,
                        mutable: false,
                    },
                    InputDefinition {
                        id: CONTEXT,
                        mutable: false,
                    },
                ],
                output: CONTEXT,
                block: ISOSURFACE
                    .specialize(&isosurface_spec)
                    .call([
                        RADIUS.read(),
                        POINT.specialize(&point_spec).call(CONTEXT.read()),
                    ])
                    .output()
                    .block(),
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
