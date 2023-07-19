use std::{fmt::Debug, hash::Hash};

use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::AsIR,
        ast::{
            Identifier, IntoBlock, IntoRead, IntoWrite, Property, TypeSpec, CONTEXT,
            POSITION_2D,
        },
        module::{FunctionDefinition, InputDefinition, Type},
    },
};

use crate::ast::expr::Expr;

pub const TRANSLATE: Identifier = Identifier::new("translate", 419357041369711478);
pub const DELTA: Property = Property::new("delta", Type::Vector2, 1292788437813720044);

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

impl<T> AsIR<T> for Translate<T>
where
    T: TypeSpec,
{
    fn functions(&self) -> Vec<FunctionDefinition<T>> {
        vec![FunctionDefinition {
            id: TRANSLATE,
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: DELTA,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, POSITION_2D].write([CONTEXT, POSITION_2D].read() - DELTA.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<T>) -> crate::ir::ast::Expr<T> {
        TRANSLATE.call([self.delta.clone().into(), input])
    }
}