//! Interior-mutable keyed cache for C++ `mutable std::map` members that `const` methods fill.
//!
//! C++ `std::map` nodes never move, so a `const` method can fill `myClassesSuccessorMap[vClass]`
//! and return a reference into it. A Rust `BTreeMap` moves values on insert, so each value is boxed
//! and only its address is stored in the map. Values are never mutated after insertion and are freed
//! only by `clear()` or on drop, so references handed out by `get_or_insert_with` stay valid while the
//! cache is alive, as they do in C++.
use std::cell::RefCell;
use std::collections::BTreeMap;

pub struct StableCache<K: Ord, V> {
    map: RefCell<BTreeMap<K, *mut V>>,
}

impl<K: Ord, V> Default for StableCache<K, V> {
    fn default() -> Self {
        Self { map: RefCell::new(BTreeMap::new()) }
    }
}

impl<K: Ord, V> StableCache<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.map.borrow().contains_key(k)
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let p = *self.map.borrow().get(k)?;
        Some(unsafe { &*p })
    }

    /// Returns the cached value for `k`, computing and storing it with `f` on first use.
    pub fn get_or_insert_with(&self, k: K, f: impl FnOnce() -> V) -> &V {
        if let Some(v) = self.get(&k) {
            return v;
        }
        let p = Box::into_raw(Box::new(f()));
        {
            let mut map = self.map.borrow_mut();
            if !map.contains_key(&k) {
                map.insert(k, p);
                return unsafe { &*p };
            }
        }
        // f() re-entered and filled this key; keep the first value, as C++ `operator[]` would
        unsafe { drop(Box::from_raw(p)) };
        self.get(&k).unwrap()
    }

    pub fn clear(&self) {
        let old = std::mem::take(&mut *self.map.borrow_mut());
        for (_, p) in old {
            unsafe { drop(Box::from_raw(p)) };
        }
    }
}

impl<K: Ord, V> Drop for StableCache<K, V> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<K: Ord + Clone, V: Clone> Clone for StableCache<K, V> {
    fn clone(&self) -> Self {
        let out = Self::new();
        for (k, p) in self.map.borrow().iter() {
            let v = unsafe { (**p).clone() };
            out.map.borrow_mut().insert(k.clone(), Box::into_raw(Box::new(v)));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn references_survive_later_inserts() {
        let c: StableCache<i32, Vec<i32>> = StableCache::new();
        let first = c.get_or_insert_with(1, || vec![10]);
        for k in 2..50 {
            c.get_or_insert_with(k, || vec![k]);
        }
        assert_eq!(first, &vec![10]);
        assert_eq!(c.get(&7), Some(&vec![7]));
        assert!(c.get(&99).is_none());
    }

    #[test]
    fn first_value_wins_and_clone_copies() {
        let c: StableCache<i32, i32> = StableCache::new();
        assert_eq!(*c.get_or_insert_with(3, || 5), 5);
        assert_eq!(*c.get_or_insert_with(3, || 9), 5);
        let d = c.clone();
        c.clear();
        assert!(!c.contains_key(&3));
        assert_eq!(d.get(&3), Some(&5));
    }
}
