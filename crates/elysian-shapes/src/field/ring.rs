use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, IntoBlock, IntoRead},
        module::{
            FunctionDefinition, InputDefinition, NumericType, SpecializationData, Type, CONTEXT,
        },
    },
    property,
};

use crate::modify::{Isosurface, Manifold, ISOSURFACE, MANIFOLD};

use super::{Circle, CIRCLE, RADIUS};

pub const RING: Identifier = Identifier::new("ring", 18972348581943461950);

pub const WIDTH: Identifier = Identifier::new("width", 2742125101201765597);
property!(WIDTH, WIDTH_PROP, Type::Number(NumericType::Float));

pub struct Ring {
    pub radius: Expr,
    pub width: Expr,
}

impl Debug for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ring")
            .field("radius", &self.radius)
            .field("width", &self.width)
            .finish()
    }
}

impl Clone for Ring {
    fn clone(&self) -> Self {
        Self {
            radius: self.radius.clone(),
            width: self.width.clone(),
        }
    }
}

impl Hash for Ring {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
        self.width.hash(state);
    }
}

impl Domains for Ring {
    fn domains() -> Vec<Identifier> {
        Circle::domains()
            .into_iter()
            .chain(Manifold::domains())
            .chain(Isosurface::domains())
            .collect()
    }
}

impl AsIR for Ring {
    fn functions_impl(
        &self,
        spec: &SpecializationData,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let isosurface_spec = spec.filter(Isosurface::domains());
        let manifold_spec = spec.filter(Manifold::domains());
        let circle_spec = spec.filter(Circle::domains());

        Circle {
            radius: self.radius.clone(),
        }
        .functions(spec)
        .into_iter()
        .chain(Manifold.functions(spec))
        .chain(
            Isosurface {
                dist: self.width.clone(),
            }
            .functions(spec),
        )
        .chain(FunctionDefinition {
            id: RING.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    id: RADIUS,
                    mutable: false,
                },
                InputDefinition {
                    id: WIDTH,
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
                    WIDTH.read(),
                    MANIFOLD.specialize(&manifold_spec).call(
                        CIRCLE
                            .specialize(&circle_spec)
                            .call([RADIUS.read(), CONTEXT.read()]),
                    ),
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
        RING.specialize(spec)
            .call([self.radius.clone().into(), self.width.clone().into(), input])
    }
}
