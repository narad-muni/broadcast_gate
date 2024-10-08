use std::{
    cell::UnsafeCell,
    collections::HashMap,
    hash::Hash,
};

pub struct SafeHashMap<K, V> {
    pub map: UnsafeCell<HashMap<K, V>>,
}

unsafe impl<K, V> Send for SafeHashMap<K, V> {}
unsafe impl<K, V> Sync for SafeHashMap<K, V> {}

impl<K, V> SafeHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            map: UnsafeCell::new(HashMap::new()),
        }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        unsafe { (*self.map.get()).insert(key, value) }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe { (*self.map.get()).get(key) }
    }
}
