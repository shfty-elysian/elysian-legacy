use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::{field::Field, modify::Modify},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite, Property, CONTEXT,
            POSITION_2D, POSITION_3D, VECTOR2, X, Y,
        },
        module::{
            FunctionDefinition, InputDefinition, NumericType, SpecializationData, Type, PROPERTIES,
        },
    },
};

use elysian_core::ast::expr::Expr;

pub const ASPECT: Identifier = Identifier::new("aspect", 346035631277210970);
#[linkme::distributed_slice(PROPERTIES)]
static ASPECT_PROP: Property = Property {
    id: ASPECT,
    ty: Type::Number(NumericType::Float),
};

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
        vec![POSITION_2D, POSITION_3D]
    }
}

impl AsIR for Aspect {
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        let aspect = elysian_core::ir::ast::Expr::Struct(
            VECTOR2,
            [(X, ASPECT.read()), (Y, 1.0.literal())].into(),
        );

        vec![FunctionDefinition {
            id: ASPECT.specialize(spec),
            public: false,
            inputs: vec![
                InputDefinition {
                    id: ASPECT.clone(),
                    mutable: false,
                },
                InputDefinition {
                    id: CONTEXT,
                    mutable: true,
                },
            ],
            output: CONTEXT,
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
        ASPECT
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
