use std::{
    cell::{OnceCell, UnsafeCell},
    collections::HashMap,
    hash::Hash,
};

pub struct SafeHashMap<K, V> {
    pub map: OnceCell<UnsafeCell<HashMap<K, V>>>,
}

unsafe impl<K, V> Send for SafeHashMap<K, V> {}
unsafe impl<K, V> Sync for SafeHashMap<K, V> {}

impl<K, V> SafeHashMap<K, V>
where
    K: Eq + Hash,
{
    pub const fn new() -> Self {
        Self {
            map: OnceCell::new(),
        }
    }

    pub fn init(&self) {
        self.map.get_or_init(|| UnsafeCell::new(HashMap::new()));
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        assert!(self.map.get().is_some());

        let map = self.map.get().unwrap();

        unsafe { (*map.get()).insert(key, value) }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        assert!(self.map.get().is_some());

        let map = self.map.get().unwrap();

        unsafe { (*map.get()).get(key) }
    }
}
