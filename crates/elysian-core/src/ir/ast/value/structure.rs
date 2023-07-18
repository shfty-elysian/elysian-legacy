use tracing::instrument;

use crate::ir::ast::{Property, Value};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use super::{TypeSpec, VectorSpace, VectorSpaceT};

pub trait StructIO<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn get(&self, key: &Property) -> Value<T, N>
    where
        Value<T, N>: Clone,
    {
        self.get_ref(key).clone()
    }

    fn get_ref(&self, key: &Property) -> &Value<T, N> {
        self.try_get_ref(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    fn get_mut(&mut self, key: &Property) -> &mut Value<T, N> {
        self.try_get_mut(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    fn try_get(&self, key: &Property) -> Option<Value<T, N>>
    where
        Value<T, N>: Clone,
    {
        self.try_get_ref(key).cloned()
    }

    fn try_get_ref(&self, key: &Property) -> Option<&Value<T, N>>;
    fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value<T, N>>;

    fn set(mut self, key: Property, t: Value<T, N>) -> Self
    where
        Self: Sized,
    {
        self.set_mut(key, t);
        self
    }

    fn set_mut(&mut self, key: Property, t: Value<T, N>);
}

impl<T, const N: usize> StructIO<T, N> for Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn try_get_ref(&self, key: &Property) -> Option<&Value<T, N>> {
        self.members.get(key)
    }

    fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value<T, N>> {
        self.members.get_mut(key)
    }

    fn set_mut(&mut self, key: Property, t: Value<T, N>) {
        self.members.insert(key, t);
    }
}

pub struct Struct<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N> + ?Sized,
{
    pub members: BTreeMap<Property, Value<T, N>>,
}

impl<T, const N: usize> Debug for Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct")
            .field("members", &self.members)
            .finish()
    }
}

impl<T, const N: usize> Clone for Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
        }
    }
}

impl<T, const N: usize> Default for Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn default() -> Self {
        Self {
            members: Default::default(),
        }
    }
}

impl<T, const N: usize> PartialEq for Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn eq(&self, other: &Self) -> bool {
        self.members == other.members
    }
}

impl<T, const N: usize> Hash for Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.members.hash(state);
    }
}

impl<T, const N: usize> Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    #[instrument]
    pub fn remove(&mut self, key: &Property) -> Value<T, N> {
        self.members
            .remove(key)
            .unwrap_or_else(|| panic!("Invalid key {key:?}"))
    }

    #[instrument]
    pub fn get_boolean(&self, key: &Property) -> bool
    where
        T: Clone,
    {
        let Value::Boolean(b) = self.get_ref(key) else {
        panic!("Value is not a boolean")
    };

        *b
    }

    #[instrument]
    pub fn get_number(&self, key: &Property) -> T::NUMBER
    where
        T: TypeSpec,
    {
        let value = self.get_ref(key);
        let Value::Number(n) = value else {
        panic!("Value {value:#?} for key {key:?} distance is not a number");
    };

        n.clone()
    }

    #[instrument]
    pub fn get_vector_space(&self, key: &Property) -> VectorSpaceT<T, N>
    where
        T: TypeSpec,
    {
        let value = self.get_ref(key);
        let Value::VectorSpace(v) = value else {
        panic!("Value {value:#?} for key {key:?} is not a VectorSpace");
    };

        v.clone()
    }

    #[instrument]
    pub fn get_vector2(&self, key: &Property) -> T::VECTOR2
    where
        T: TypeSpec,
    {
        let value = self.get_ref(key);
        let Value::Vector2(v) = value else {
        panic!("Value {value:#?} for key {key:?} is not a Vector2");
    };

        v.clone()
    }

    #[instrument]
    pub fn get_context(&self, key: &Property) -> Struct<T, N>
    where
        Struct<T, N>: Clone,
    {
        let Value::Struct(c) = self.get_ref(key) else {
        panic!("Value is not a context")
    };

        c.clone()
    }

    #[instrument]
    pub fn set_number(mut self, key: Property, n: T::NUMBER) -> Self
    where
        T: TypeSpec,
    {
        self.members.insert(key, Value::Number(n));
        self
    }

    #[instrument]
    pub fn set_vector(mut self, key: Property, v: T::VECTOR2) -> Self
    where
        T: TypeSpec,
    {
        self.members.insert(key, Value::Vector2(v));
        self
    }
}
