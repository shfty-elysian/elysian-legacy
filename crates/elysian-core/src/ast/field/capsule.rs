use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ast::modify::{DIR_3D, ISOSURFACE};
use crate::ir::as_ir::FilterSpec;
use crate::ir::ast::{POSITION_2D, POSITION_3D};
use crate::ir::module::SpecializationData;
use crate::{
    ast::{
        expr::Expr,
        field::{Line, CONTEXT_STRUCT},
        modify::{Isosurface, DIR_2D},
    },
    ir::{
        as_ir::AsIR,
        ast::{Identifier, IntoBlock, TypeSpec, CONTEXT},
        module::{FunctionDefinition, InputDefinition},
    },
};

use super::{LINE, RADIUS};

pub const CAPSULE: Identifier = Identifier::new("capsule", 14339483921749952476);

pub struct Capsule<T>
where
    T: TypeSpec,
{
    pub dir: Expr<T>,
    pub radius: Expr<T>,
}

impl<T> Debug for Capsule<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capsule")
            .field("dir", &self.dir)
            .field("radius", &self.radius)
            .finish()
    }
}

impl<T> Clone for Capsule<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            radius: self.radius.clone(),
        }
    }
}

impl<T> Hash for Capsule<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.radius.hash(state);
    }
}

impl<T> FilterSpec for Capsule<T>
where
    T: TypeSpec,
{
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        Line::<T>::filter_spec(spec).union(&Isosurface::<T>::filter_spec(spec))
    }
}

impl<T> AsIR<T> for Capsule<T>
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
            panic!("No position domain");
        };

        let isosurface_spec = Isosurface::<T>::filter_spec(spec);
        let line_spec = Line::<T>::filter_spec(spec);

        Line {
            dir: self.dir.clone(),
        }
        .functions(spec)
        .into_iter()
        .chain(
            Isosurface {
                dist: self.radius.clone(),
            }
            .functions(spec),
        )
        .chain(FunctionDefinition {
            id: CAPSULE.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: dir.clone(),
                    mutable: false,
                },
                InputDefinition {
                    prop: RADIUS,
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
                    RADIUS.read(),
                    LINE.specialize(&line_spec)
                        .call([dir.read(), CONTEXT.read()]),
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
        CAPSULE
            .specialize(spec)
            .call([self.dir.clone().into(), self.radius.clone().into(), input])
    }
}
