use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

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

pub const ELONGATE: Identifier = Identifier::new("elongate", 1022510703206415324);
pub const ELONGATE_INFINITE: Identifier = Identifier::new("elongate_infinite", 1799909959882308009);
pub const DIR_2D: Property = Property::new("dir_2d", Type::Vector2, 10994004961423687819);
pub const DIR_3D: Property = Property::new("dir_3d", Type::Vector3, 66909101541205811);

pub struct Elongate<T>
where
    T: TypeSpec,
{
    pub dir: Expr<T>,
    pub infinite: bool,
}

impl<T> Debug for Elongate<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Elongate")
            .field("dir", &self.dir)
            .field("infinite", &self.infinite)
            .finish()
    }
}

impl<T> Clone for Elongate<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            infinite: self.infinite.clone(),
        }
    }
}

impl<T> Hash for Elongate<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.infinite.hash(state);
    }
}

impl<T> FilterSpec for Elongate<T>
where
    T: TypeSpec,
{
    fn filter_spec(spec: &SpecializationData) -> SpecializationData {
        spec.filter([POSITION_2D.id(), POSITION_3D.id()])
    }
}

impl<T> AsIR<T> for Elongate<T>
where
    T: TypeSpec,
{
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        let (position, dir) = if spec.contains(POSITION_2D.id()) {
            (POSITION_2D, DIR_2D)
        } else if spec.contains(POSITION_3D.id()) {
            (POSITION_3D, DIR_3D)
        } else {
            panic!("No position domain");
        };

        vec![FunctionDefinition {
            id: if self.infinite {
                ELONGATE_INFINITE.specialize(spec)
            } else {
                ELONGATE.specialize(spec)
            },
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: dir.clone(),
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: {
                let expr = [CONTEXT, position.clone()]
                    .read()
                    .dot(dir.clone().read().normalize());

                [
                    [CONTEXT, position.clone()].bind(
                        [CONTEXT, position].read()
                            - dir.clone().read().normalize()
                                * if self.infinite {
                                    expr
                                } else {
                                    expr.max(-dir.clone().read().length())
                                        .min(dir.clone().read().length())
                                },
                    ),
                    CONTEXT.read().output(),
                ]
                .block()
            },
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: crate::ir::ast::Expr<T>,
    ) -> crate::ir::ast::Expr<T> {
        if self.infinite {
            ELONGATE_INFINITE.specialize(spec)
        } else {
            ELONGATE.specialize(spec)
        }
        .call([self.dir.clone().into(), input])
    }
}
