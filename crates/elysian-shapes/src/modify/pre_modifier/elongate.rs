use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Identifier, IntoBlock, IntoRead, IntoWrite, POSITION_2D,
            POSITION_3D, VECTOR2, VECTOR3,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type, CONTEXT},
    }, property,
};

use elysian_core::ast::expr::Expr;

pub const ELONGATE: Identifier = Identifier::new("elongate", 1022510703206415324);
pub const ELONGATE_INFINITE: Identifier = Identifier::new("elongate_infinite", 1799909959882308009);

pub const DIR_2D: Identifier = Identifier::new("dir_2d", 10994004961423687819);
property!(DIR_2D, DIR_2D_PROP, Type::Struct(VECTOR2));

pub const DIR_3D: Identifier = Identifier::new("dir_3d", 66909101541205811);
property!(DIR_3D, DIR_3D_PROP, Type::Struct(VECTOR3));

pub struct Elongate {
    pub dir: Expr,
    pub infinite: bool,
}

impl Debug for Elongate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Elongate")
            .field("dir", &self.dir)
            .field("infinite", &self.infinite)
            .finish()
    }
}

impl Clone for Elongate {
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            infinite: self.infinite.clone(),
        }
    }
}

impl Hash for Elongate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.infinite.hash(state);
    }
}

impl Domains for Elongate {
    fn domains() -> Vec<Identifier> {
        vec![POSITION_2D, POSITION_3D]
    }
}

impl AsIR for Elongate {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let (position, dir) = if spec.contains(&POSITION_2D) {
            (POSITION_2D, DIR_2D)
        } else if spec.contains(&POSITION_3D) {
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
                    id: dir.clone(),
                    mutable: false,
                },
                InputDefinition {
                    id: CONTEXT,
                    mutable: true,
                },
            ],
            output: CONTEXT,
            block: {
                let expr = [CONTEXT, position.clone()]
                    .read()
                    .dot(dir.clone().read().normalize());

                [
                    [CONTEXT, position.clone()].write(
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
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        if self.infinite {
            ELONGATE_INFINITE.specialize(spec)
        } else {
            ELONGATE.specialize(spec)
        }
        .call([self.dir.clone().into(), input])
    }
}

pub trait IntoElongate {
    fn elongate(self, dir: elysian_core::ast::expr::Expr, infinite: bool) -> Modify;
}

impl IntoElongate for Field {
    fn elongate(self, dir: elysian_core::ast::expr::Expr, infinite: bool) -> Modify {
        Modify {
            pre_modifiers: vec![Box::new(Elongate { dir, infinite })],
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl IntoElongate for Modify {
    fn elongate(mut self, dir: elysian_core::ast::expr::Expr, infinite: bool) -> Modify {
        self.pre_modifiers
            .push(Box::new(Elongate { dir, infinite }));
        self
    }
}
