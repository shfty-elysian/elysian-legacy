use crate::ir::ast::{Property, Value};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Struct<N, V> {
    pub(crate) members: BTreeMap<Property, Value<N, V>>,
}

impl<N, V> Default for Struct<N, V> {
    fn default() -> Self {
        Self {
            members: Default::default(),
        }
    }
}

impl<N, V> Struct<N, V> {
    pub fn get(&self, key: &Property) -> Value<N, V>
    where
        Value<N, V>: Clone,
    {
        self.get_ref(key).clone()
    }

    pub fn get_ref(&self, key: &Property) -> &Value<N, V> {
        self.members
            .get(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    pub fn get_mut(&mut self, key: &Property) -> &mut Value<N, V> {
        self.members.get_mut(key).expect("Invalid key")
    }

    pub fn set(mut self, key: Property, t: Value<N, V>) -> Self {
        self.members.insert(key, t);
        self
    }

    pub fn set_mut(&mut self, key: Property, t: Value<N, V>) {
        self.members.insert(key, t);
    }

    pub fn copy(self, source: &Property, target: Property) -> Self
    where
        Value<N, V>: Clone,
    {
        let v = self.get(source);
        self.set(target, v)
    }

    pub fn remove(&mut self, key: &Property) -> Value<N, V> {
        self.members
            .remove(key)
            .unwrap_or_else(|| panic!("Invalid key {key:?}"))
    }

    pub fn get_boolean(&self, key: &Property) -> bool
    where
        N: Clone,
    {
        let Value::Boolean(b) = self.get_ref(key) else {
        panic!("Value is not a boolean")
    };

        *b
    }

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

    pub fn get_context(&self, key: &Property) -> Struct<N, V>
    where
        Struct<N, V>: Clone,
    {
        let Value::Struct(c) = self.get_ref(key) else {
        panic!("Value is not a context")
    };

        c.clone()
    }

    pub fn set_number(mut self, key: Property, n: N) -> Self {
        self.members.insert(key, Value::Number(n));
        self
    }

    pub fn set_vector(mut self, key: Property, v: V) -> Self {
        self.members.insert(key, Value::Vector(v));
        self
    }
}
