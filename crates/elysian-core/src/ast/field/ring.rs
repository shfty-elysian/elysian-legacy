use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::{Circle, CONTEXT_STRUCT},
        modify::{Isosurface, Manifold, ISOSURFACE, MANIFOLD},
    },
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{Identifier, IntoBlock, Property, TypeSpec, CONTEXT},
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use super::{CIRCLE, RADIUS};

pub const RING: Identifier = Identifier::new("ring", 18972348581943461950);
pub const WIDTH: Property = Property::new("width", Type::Number, 2742125101201765597);

pub struct Ring<T>
where
    T: TypeSpec,
{
    pub radius: Expr<T>,
    pub width: Expr<T>,
}

impl<T> Debug for Ring<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ring")
            .field("radius", &self.radius)
            .field("width", &self.width)
            .finish()
    }
}

impl<T> Clone for Ring<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            radius: self.radius.clone(),
            width: self.width.clone(),
        }
    }
}

impl<T> Hash for Ring<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
        self.width.hash(state);
    }
}

impl<T> FilterSpec for Ring<T>
where
    T: TypeSpec,
{
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        Circle::<T>::filter_spec(spec)
            .union(&Manifold::filter_spec(spec))
            .union(&Isosurface::<T>::filter_spec(spec))
    }
}

impl<T> AsIR<T> for Ring<T>
where
    T: TypeSpec,
{
    fn functions_impl(
        &self,
        spec: &SpecializationData,
    ) -> Vec<crate::ir::module::FunctionDefinition<T>> {
        let isosurface_spec = Isosurface::<T>::filter_spec(spec);
        let manifold_spec = Manifold::filter_spec(spec);
        let circle_spec = Circle::<T>::filter_spec(spec);

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
                    prop: RADIUS,
                    mutable: false,
                },
                InputDefinition {
                    prop: WIDTH,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                },
            ],
            output: CONTEXT_STRUCT,
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
        input: crate::ir::ast::Expr<T>,
    ) -> crate::ir::ast::Expr<T> {
        RING.specialize(spec)
            .call([self.radius.clone().into(), self.width.clone().into(), input])
    }
}
