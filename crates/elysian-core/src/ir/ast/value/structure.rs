use tracing::instrument;

use crate::ir::ast::{Property, Value};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use super::TypeSpec;

pub trait StructIO<T>
where
    T: TypeSpec,
{
    fn get(&self, key: &Property) -> Value<T>
    where
        Value<T>: Clone,
    {
        self.get_ref(key).clone()
    }

    fn get_ref(&self, key: &Property) -> &Value<T> {
        self.try_get_ref(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    fn get_mut(&mut self, key: &Property) -> &mut Value<T> {
        self.try_get_mut(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    fn try_get(&self, key: &Property) -> Option<Value<T>>
    where
        Value<T>: Clone,
    {
        self.try_get_ref(key).cloned()
    }

    fn try_get_ref(&self, key: &Property) -> Option<&Value<T>>;
    fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value<T>>;

    fn set(mut self, key: Property, t: Value<T>) -> Self
    where
        Self: Sized,
    {
        self.set_mut(key, t);
        self
    }

    fn set_mut(&mut self, key: Property, t: Value<T>);
}

impl<T> StructIO<T> for Struct<T>
where
    T: TypeSpec,
{
    fn try_get_ref(&self, key: &Property) -> Option<&Value<T>> {
        self.members.get(key)
    }

    fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value<T>> {
        self.members.get_mut(key)
    }

    fn set_mut(&mut self, key: Property, t: Value<T>) {
        self.members.insert(key, t);
    }
}

pub struct Struct<T>
where
    T: TypeSpec + ?Sized,
{
    pub members: BTreeMap<Property, Value<T>>,
}

impl<T> Debug for Struct<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct")
            .field("members", &self.members)
            .finish()
    }
}

impl<T> Clone for Struct<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
        }
    }
}

impl<T> Default for Struct<T>
where
    T: TypeSpec,
{
    fn default() -> Self {
        Self {
            members: Default::default(),
        }
    }
}

impl<T> PartialEq for Struct<T>
where
    T: TypeSpec,
{
    fn eq(&self, other: &Self) -> bool {
        self.members == other.members
    }
}

impl<T> Hash for Struct<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.members.hash(state);
    }
}

impl<T> Struct<T>
where
    T: TypeSpec,
{
    #[instrument]
    pub fn remove(&mut self, key: &Property) -> Value<T> {
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
    pub fn get_vector3(&self, key: &Property) -> T::VECTOR3
    where
        T: TypeSpec,
    {
        let value = self.get_ref(key);
        let Value::Vector3(v) = value else {
            panic!("Value {value:#?} for key {key:?} is not a Vector3");
        };

        v.clone()
    }

    #[instrument]
    pub fn get_vector4(&self, key: &Property) -> T::VECTOR4
    where
        T: TypeSpec,
    {
        let value = self.get_ref(key);
        let Value::Vector4(v) = value else {
            panic!("Value {value:#?} for key {key:?} is not a Vector4");
        };

        v.clone()
    }

    #[instrument]
    pub fn get_context(&self, key: &Property) -> Struct<T>
    where
        Struct<T>: Clone,
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
