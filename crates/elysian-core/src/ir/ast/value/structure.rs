use tracing::instrument;

use crate::ir::ast::{Property, Value};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
};

use super::Number;

pub struct Struct {
    pub members: BTreeMap<Property, Value>,
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (prop, val) in &self.members {
            write!(f, "{prop:}: {val:}")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Debug for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct")
            .field("members", &self.members)
            .finish()
    }
}

impl Clone for Struct {
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
        }
    }
}

impl Default for Struct {
    fn default() -> Self {
        Self {
            members: Default::default(),
        }
    }
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.members == other.members
    }
}

impl Hash for Struct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.members.hash(state);
    }
}

impl Struct {
    pub fn try_get_ref(&self, key: &Property) -> Option<&Value> {
        self.members.get(key)
    }

    pub fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value> {
        self.members.get_mut(key)
    }

    pub fn set_mut(&mut self, key: Property, t: Value) {
        self.members.insert(key, t);
    }

    pub fn get(&self, key: &Property) -> Value {
        self.get_ref(key).clone()
    }

    fn get_ref(&self, key: &Property) -> &Value {
        self.try_get_ref(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    pub fn get_mut(&mut self, key: &Property) -> &mut Value {
        self.try_get_mut(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    pub fn try_get(&self, key: &Property) -> Option<Value> {
        self.try_get_ref(key).cloned()
    }

    pub fn set(mut self, key: Property, t: Value) -> Self
    where
        Self: Sized,
    {
        self.set_mut(key, t);
        self
    }

    #[instrument]
    pub fn remove(&mut self, key: &Property) -> Value {
        self.members
            .remove(key)
            .unwrap_or_else(|| panic!("Invalid key {key:?}"))
    }

    #[instrument]
    pub fn get_context(&self, key: &Property) -> Struct {
        let Value::Struct(c) = self.get_ref(key) else {
        panic!("Value is not a context")
    };

        c.clone()
    }

    #[instrument]
    pub fn set_number(mut self, key: Property, n: Number) -> Self {
        self.members.insert(key, Value::Number(n));
        self
    }
}
