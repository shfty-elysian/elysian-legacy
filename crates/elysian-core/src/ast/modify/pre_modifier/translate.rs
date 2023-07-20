use std::{fmt::Debug, hash::Hash};

use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{
            Identifier, IntoBlock, IntoRead, IntoBind, Property, TypeSpec, CONTEXT, POSITION_2D,
            POSITION_3D,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

use crate::ast::expr::Expr;

pub const TRANSLATE: Identifier = Identifier::new("translate", 419357041369711478);
pub const DELTA_2D: Property = Property::new("delta_2d", Type::Vector2, 1292788437813720044);
pub const DELTA_3D: Property = Property::new("delta_3d", Type::Vector3, 8306277011223488934);

pub struct Translate<T>
where
    T: TypeSpec,
{
    pub delta: Expr<T>,
}

impl<T> Debug for Translate<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Translate")
            .field("delta", &self.delta)
            .finish()
    }
}

impl<T> Clone for Translate<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            delta: self.delta.clone(),
        }
    }
}

impl<T> Hash for Translate<T>
where
    T: TypeSpec,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.delta.hash(state);
    }
}

impl<T> FilterSpec for Translate<T>
where
    T: TypeSpec,
{
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        spec.filter([POSITION_2D.id(), POSITION_3D.id()])
    }
}

impl<T> AsIR<T> for Translate<T>
where
    T: TypeSpec,
{
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        let (position, delta) = if spec.contains(POSITION_2D.id()) {
            (POSITION_2D, DELTA_2D)
        } else if spec.contains(POSITION_3D.id()) {
            (POSITION_3D, DELTA_3D)
        } else {
            panic!("No position domain")
        };

        vec![FunctionDefinition {
            id: TRANSLATE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: delta.clone(),
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, position.clone()].bind([CONTEXT, position].read() - delta.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: crate::ir::ast::Expr<T>,
    ) -> crate::ir::ast::Expr<T> {
        TRANSLATE
            .specialize(spec)
            .call([self.delta.clone().into(), input])
    }
}
