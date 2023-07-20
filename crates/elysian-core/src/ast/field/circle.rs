use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::{point::Point, CONTEXT_STRUCT},
        modify::{Isosurface, ISOSURFACE},
    },
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{Identifier, IntoBlock, Property, CONTEXT},
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use super::POINT;

pub const CIRCLE: Identifier = Identifier::new("circle", 15738477621793375359);
pub const RADIUS: Property = Property::new("radius", Type::Number, 213754678517975478);

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

impl FilterSpec for Circle {
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        Point::filter_spec(spec).union(&Isosurface::filter_spec(spec))
    }
}

impl AsIR for Circle {
    fn functions_impl(
        &self,
        spec: &SpecializationData,
    ) -> Vec<crate::ir::module::FunctionDefinition> {
        let point_spec = Point::filter_spec(spec);
        let isosurface_spec = Isosurface::filter_spec(spec);

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
                        prop: RADIUS,
                        mutable: false,
                    },
                    InputDefinition {
                        prop: CONTEXT,
                        mutable: false,
                    },
                ],
                output: &CONTEXT_STRUCT,
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
        input: crate::ir::ast::Expr,
    ) -> crate::ir::ast::Expr {
        CIRCLE
            .specialize(spec)
            .call([self.radius.clone().into(), input])
    }
}
