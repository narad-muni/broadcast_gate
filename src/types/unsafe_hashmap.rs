use std::{cell::UnsafeCell, collections::{hash_map::Entry, HashMap}, hash::Hash};

pub struct UnsafeHashMap<K, V> {
    pub map: UnsafeCell<HashMap<K, V>>,
}

unsafe impl<K, V> Send for UnsafeHashMap<K, V> {}
unsafe impl<K, V> Sync for UnsafeHashMap<K, V> {}

impl<K, V> UnsafeHashMap<K, V>
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

    pub fn entry(&self, key: K) -> Entry<K, V> {
        unsafe { (*self.map.get()).entry(key) }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe { (*self.map.get()).get(key) }
    }
}
