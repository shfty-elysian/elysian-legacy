use std::{fmt::Debug, hash::Hash, borrow::Cow};

use elysian_core::{
    ast::{
        field::Field,
        modify::{Modify, CONTEXT_STRUCT},
    },
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite, Property, CONTEXT,
            POSITION_2D, POSITION_3D, VECTOR2_STRUCT, X, Y,
        },
        module::{FunctionDefinition, InputDefinition, NumericType, SpecializationData, Type},
    },
};

use elysian_core::ast::expr::Expr;

pub const ASPECT_FUNCTION: Identifier = Identifier::new("aspect", 948144044945271613);
pub const ASPECT: Property = Property::new(
    "aspect",
    Type::Number(NumericType::Float),
    346035631277210970,
);

#[derive(Debug, Clone)]
pub struct Aspect {
    pub aspect: Expr,
}

impl Hash for Aspect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.aspect.hash(state);
    }
}

impl Domains for Aspect {
    fn domains() -> Vec<Identifier> {
        vec![POSITION_2D.id().clone(), POSITION_3D.id().clone()]
    }
}

impl AsIR for Aspect {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let aspect = elysian_core::ir::ast::Expr::Struct(
            Cow::Borrowed(VECTOR2_STRUCT),
            [(X, ASPECT.read()), (Y, 1.0.literal())].into(),
        );

        vec![FunctionDefinition {
            id: ASPECT_FUNCTION.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: ASPECT.clone(),
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: CONTEXT_STRUCT.clone(),
            block: [
                [CONTEXT, POSITION_2D].write([CONTEXT, POSITION_2D].read() * aspect),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression_impl(
        &self,
        spec: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        ASPECT_FUNCTION
            .specialize(spec)
            .call([self.aspect.clone().into(), input])
    }
}

pub trait IntoAspect {
    fn aspect(self, delta: elysian_core::ast::expr::Expr) -> Modify;
}

impl IntoAspect for Field {
    fn aspect(self, aspect: elysian_core::ast::expr::Expr) -> Modify {
        Modify {
            pre_modifiers: vec![Box::new(Aspect { aspect })],
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl IntoAspect for Modify {
    fn aspect(mut self, aspect: elysian_core::ast::expr::Expr) -> Modify {
        self.pre_modifiers.push(Box::new(Aspect { aspect }));
        self
    }
}
