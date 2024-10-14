use std::{
    collections::HashMap,
    hash::Hash, sync::{RwLock, RwLockReadGuard},
};

pub struct SafeHashMap<K, V> {
    pub map: RwLock<HashMap<K, V>>,
}

unsafe impl<K, V> Send for SafeHashMap<K, V> {}
unsafe impl<K, V> Sync for SafeHashMap<K, V> {}

impl<K, V> SafeHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, key: K, value: V) {
        self.map.write().unwrap().insert(key, value);
    }

    pub fn read(&self) -> RwLockReadGuard<HashMap<K, V>> {
        self.map.read().unwrap()
    }
}
