use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::ir::{
    ast::{Expr, IntoBlock, IntoRead, IntoWrite, Property, CONTEXT, DISTANCE, GRADIENT, POSITION},
    from_elysian::{CONTEXT_STRUCT, POINT},
    module::{FunctionDefinition, InputDefinition},
};

use super::Elysian;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field<N, V> {
    Point,
    _Phantom(PhantomData<(N, V)>),
}

impl<N, V> Hash for Field<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug, Hash)]
pub struct PointField;

pub trait AsField<N, V>: std::fmt::Debug {
    fn field_expr(&self, input: Property) -> Expr<N, V>;
    fn field_function(&self) -> FunctionDefinition<N, V>;
    fn field_hash(&self) -> u64;
}

impl<N, V> AsField<N, V> for PointField {
    fn field_expr(&self, input: Property) -> Expr<N, V> {
        Expr::Call {
            function: POINT,
            args: vec![input.read()],
        }
    }

    fn field_function(&self) -> FunctionDefinition<N, V> {
        FunctionDefinition {
            id: POINT,
            public: false,
            inputs: vec![InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, POSITION].read().length()),
                [CONTEXT, GRADIENT].write([CONTEXT, POSITION].read().normalize()),
                CONTEXT.read().output(),
            ]
            .block(),
        }
    }
  
    fn field_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T, N, V> AsField<N, V> for &T
where
    T: AsField<N, V>,
{
    fn field_expr(&self, input: Property) -> Expr<N, V> {
        (*self).field_expr(input)
    }

    fn field_function(&self) -> FunctionDefinition<N, V> {
        (*self).field_function()
    }

    fn field_hash(&self) -> u64 {
        (*self).field_hash()
    }
}

pub trait IntoField<N, V>: Sized + AsField<N, V> {
    fn field(self) -> Elysian<N, V> {
        Elysian::Field {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T, N, V> IntoField<N, V> for T where T: Sized + AsField<N, V> {}
