use tracing::instrument;

use crate::ir::ast::{Property, Value};
use std::{collections::BTreeMap, fmt::Debug};

pub trait StructIO<N, V> {
    fn get(&self, key: &Property) -> Value<N, V>
    where
        Value<N, V>: Clone,
    {
        self.get_ref(key).clone()
    }

    fn get_ref(&self, key: &Property) -> &Value<N, V> {
        self.try_get_ref(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    fn get_mut(&mut self, key: &Property) -> &mut Value<N, V> {
        self.try_get_mut(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    fn try_get(&self, key: &Property) -> Option<Value<N, V>>
    where
        Value<N, V>: Clone,
    {
        self.try_get_ref(key).cloned()
    }

    fn try_get_ref(&self, key: &Property) -> Option<&Value<N, V>>;
    fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value<N, V>>;

    fn set(mut self, key: Property, t: Value<N, V>) -> Self
    where
        Self: Sized,
    {
        self.set_mut(key, t);
        self
    }

    fn set_mut(&mut self, key: Property, t: Value<N, V>);
}

impl<N, V> StructIO<N, V> for Struct<N, V> {
    fn try_get_ref(&self, key: &Property) -> Option<&Value<N, V>> {
        self.members.get(key)
    }

    fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value<N, V>> {
        self.members.get_mut(key)
    }

    fn set_mut(&mut self, key: Property, t: Value<N, V>) {
        self.members.insert(key, t);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Struct<N, V> {
    pub members: BTreeMap<Property, Value<N, V>>,
}

impl<N, V> Default for Struct<N, V> {
    fn default() -> Self {
        Self {
            members: Default::default(),
        }
    }
}

impl<N, V> Struct<N, V>
where
    N: Debug,
    V: Debug,
{
    #[instrument]
    pub fn remove(&mut self, key: &Property) -> Value<N, V> {
        self.members
            .remove(key)
            .unwrap_or_else(|| panic!("Invalid key {key:?}"))
    }

    #[instrument]
    pub fn get_boolean(&self, key: &Property) -> bool
    where
        N: Clone,
    {
        let Value::Boolean(b) = self.get_ref(key) else {
        panic!("Value is not a boolean")
    };

        *b
    }

    #[instrument]
    pub fn get_number(&self, key: &Property) -> N
    where
        N: Debug + Clone,
        V: Debug,
    {
        let value = self.get_ref(key);
        let Value::Number(n) = value else {
        panic!("Value {value:#?} for key {key:?} distance is not a number");
    };

        n.clone()
    }

    #[instrument]
    pub fn get_vector(&self, key: &Property) -> V
    where
        N: Debug,
        V: Debug + Clone,
    {
        let value = self.get_ref(key);
        let Value::Vector(v) = value else {
        panic!("Value {value:#?} for key {key:?} is not a vector");
    };

        v.clone()
    }

    #[instrument]
    pub fn get_context(&self, key: &Property) -> Struct<N, V>
    where
        Struct<N, V>: Clone,
    {
        let Value::Struct(c) = self.get_ref(key) else {
        panic!("Value is not a context")
    };

        c.clone()
    }

    #[instrument]
    pub fn set_number(mut self, key: Property, n: N) -> Self {
        self.members.insert(key, Value::Number(n));
        self
    }

    #[instrument]
    pub fn set_vector(mut self, key: Property, v: V) -> Self {
        self.members.insert(key, Value::Vector(v));
        self
    }
}
