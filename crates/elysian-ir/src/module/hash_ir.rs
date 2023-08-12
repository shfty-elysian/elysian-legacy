use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub trait HashIR {
    fn hash_ir(&self) -> u64;
}

impl<T> HashIR for T
where
    T: Hash,
{
    fn hash_ir(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
