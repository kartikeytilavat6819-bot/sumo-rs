//! Port of utils/distribution/DistributionCont.{h,cpp}
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::distribution::Distribution;

/// Map from distribution ids to distributions
type DistDict = BTreeMap<String, Arc<dyn Distribution>>;
/// Map from distribution types to distribution ids to distributions
type TypedDistDict = BTreeMap<String, DistDict>;

/// The global container (C++ static DistributionCont::myDict)
static MY_DICT: Mutex<TypedDistDict> = Mutex::new(BTreeMap::new());

fn dict() -> std::sync::MutexGuard<'static, TypedDistDict> {
    MY_DICT.lock().unwrap_or_else(|e| e.into_inner())
}

/// Adds a distribution of the given type and name. Returns false (and keeps nothing)
/// if the type already has a distribution with this id.
pub fn dictionary_add(dist_type: &str, id: &str, d: Arc<dyn Distribution>) -> bool {
    let mut dict = dict();
    let typed = dict.entry(dist_type.to_string()).or_default();
    if typed.contains_key(id) {
        return false;
    }
    typed.insert(id.to_string(), d);
    true
}

/// Retrieves the distribution described by a type and a name
pub fn dictionary_get(dist_type: &str, id: &str) -> Option<Arc<dyn Distribution>> {
    dict().get(dist_type)?.get(id).cloned()
}

/// Deletes all stored distributions.
/// C++ deletes the objects but leaves the map entries (dangling pointers); here the entries are removed.
pub fn clear() {
    dict().clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distribution::distribution_points::DistributionPoints;

    #[test]
    fn add_get_clear() {
        let mut p = DistributionPoints::new("a");
        p.add(1., 1., true);
        assert!(dictionary_add("speedTest", "a", Arc::new(p)));
        let p2 = DistributionPoints::new("a");
        assert!(!dictionary_add("speedTest", "a", Arc::new(p2)));
        assert!(dictionary_get("otherType", "a").is_none());
        assert_eq!(dictionary_get("speedTest", "a").unwrap().get_id(), "a");
        clear();
        assert!(dictionary_get("speedTest", "a").is_none());
    }
}
