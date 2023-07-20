use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{
        expr::Expr,
        field::{Point, CONTEXT_STRUCT},
        modify::{Elongate, DIR_2D, DIR_3D, ELONGATE},
    },
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{Identifier, IntoBlock, TypeSpec, CONTEXT, POSITION_2D, POSITION_3D},
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

use super::POINT;

pub const LINE: Identifier = Identifier::new("line", 14339483921749952476);

pub struct Line<T>
where
    T: TypeSpec,
{
    pub dir: Expr<T>,
}

impl<T> Debug for Line<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line").field("dir", &self.dir).finish()
    }
}

impl<T> Clone for Line<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
        }
    }
}

impl<T> Hash for Line<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
    }
}

impl<T> FilterSpec for Line<T>
where
    T: TypeSpec,
{
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        Point::filter_spec(spec).union(&Elongate::<T>::filter_spec(spec))
    }
}

impl<T> AsIR<T> for Line<T>
where
    T: TypeSpec,
{
    fn functions_impl(
        &self,
        spec: &SpecializationData,
    ) -> Vec<crate::ir::module::FunctionDefinition<T>> {
        let dir = if spec.contains(POSITION_2D.id()) {
            DIR_2D
        } else if spec.contains(POSITION_3D.id()) {
            DIR_3D
        } else {
            panic!("No position domain set")
        };

        let point_spec = Point::filter_spec(spec);
        let elongate_spec = Elongate::<T>::filter_spec(spec);

        Point
            .functions(spec)
            .into_iter()
            .chain(
                Elongate {
                    dir: self.dir.clone(),
                    infinite: false,
                }
                .functions(spec),
            )
            .chain(FunctionDefinition {
                id: LINE.specialize(spec),
                public: false,
                inputs: vec![
                    InputDefinition {
                        prop: dir.clone(),
                        mutable: false,
                    },
                    InputDefinition {
                        prop: CONTEXT,
                        mutable: false,
                    },
                ],
                output: CONTEXT_STRUCT,
                block: POINT
                    .specialize(&point_spec)
                    .call([ELONGATE
                        .specialize(&elongate_spec)
                        .call([dir.read(), CONTEXT.read()])])
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
        LINE.specialize(spec).call([self.dir.clone().into(), input])
    }
}
