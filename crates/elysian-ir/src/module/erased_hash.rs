use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub trait ErasedHash {
    fn erased_hash(&self) -> u64;
}

impl<T> ErasedHash for T
where
    T: Hash,
{
    fn erased_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
