//! Port of utils/common/Named.h
use super::ptr::Ptr;
use super::to_string::ToStringPrec;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

/// Anything with an id (the virtual getID() of Named subclasses)
pub trait HasId {
    fn get_id(&self) -> &str;
}

/// C++ `ComparatorIdLess`: `a->getID() < b->getID()` (bytewise, like std::string)
pub fn id_less<T: HasId + ?Sized>(a: &T, b: &T) -> bool {
    a.get_id() < b.get_id()
}

/// C++ `ComparatorNumericalIdLess` (utils/common/Named.h) expressed as a BTreeMap/BTreeSet key:
/// pointers are ordered by `getNumericalID()`, not by address. The id is captured once (SUMO object
/// ids are immutable); Ord/Eq compare only the id, matching std::map's notion of equivalent keys
/// (a second insert with the same id is ignored, exactly like `emplace`).
pub struct NumIdKey<T>(pub i64, pub Ptr<T>);

impl<T> NumIdKey<T> {
    pub fn new(ptr: Ptr<T>, numerical_id: i64) -> Self {
        NumIdKey(numerical_id, ptr)
    }
    pub fn ptr(self) -> Ptr<T> {
        self.1
    }
}
impl<T> Clone for NumIdKey<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for NumIdKey<T> {}
impl<T> PartialEq for NumIdKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for NumIdKey<T> {}
impl<T> PartialOrd for NumIdKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for NumIdKey<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T> fmt::Debug for NumIdKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("NumIdKey").field(&self.0).field(&self.1).finish()
    }
}

/// C++ `ComparatorIdLess` (utils/common/Named.h) expressed as a BTreeMap/BTreeSet key: pointers are
/// ordered by `getID()` (bytewise std::string order), not by address.
pub struct IdKey<T>(pub String, pub Ptr<T>);

impl<T> IdKey<T> {
    pub fn new(ptr: Ptr<T>, id: &str) -> Self {
        IdKey(id.to_string(), ptr)
    }
    pub fn ptr(&self) -> Ptr<T> {
        self.1
    }
}
impl<T> Clone for IdKey<T> {
    fn clone(&self) -> Self {
        IdKey(self.0.clone(), self.1)
    }
}
impl<T> PartialEq for IdKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for IdKey<T> {}
impl<T> PartialOrd for IdKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for IdKey<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T> fmt::Debug for IdKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IdKey").field(&self.0).field(&self.1).finish()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Named {
    id: String,
}

impl Named {
    pub fn new(id: &str) -> Self {
        Named { id: id.to_string() }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn set_id(&mut self, new_id: &str) {
        self.id = new_id.to_string();
    }

    /// getIDSecure(obj, fallBack = "NULL")
    pub fn get_id_secure<T: HasId + ?Sized>(obj: Option<&T>, fall_back: &str) -> String {
        obj.map_or_else(|| fall_back.to_string(), |o| o.get_id().to_string())
    }

    /// C++ `Named::addTo(const StoringVisitor &) const`
    pub fn add_to(&self, cont: &NamedStoringVisitor) {
        cont.add(Ptr::new(self));
    }
}

/// C++ class `Named::StoringVisitor`: stores the visited objects; the context of an RTree search
pub struct NamedStoringVisitor {
    /// The container
    pub myObjects: Ptr<BTreeSet<Ptr<Named>>>,
}

impl NamedStoringVisitor {
    /// C++ `StoringVisitor(std::set<const Named*>& objects)`
    pub fn new(objects: &mut BTreeSet<Ptr<Named>>) -> Self {
        NamedStoringVisitor { myObjects: Ptr::from_mut(objects) }
    }

    /// C++ `void StoringVisitor::add(const Named* const o) const`
    pub fn add(&self, o: Ptr<Named>) {
        self.myObjects.m().insert(o);
    }
}

impl HasId for Named {
    fn get_id(&self) -> &str {
        &self.id
    }
}

/// toString(const Named*)
impl ToStringPrec for Named {
    fn to_string_prec(&self, _accuracy: i32) -> String {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ids() {
        let mut n = Named::new("a");
        n.set_id("b");
        assert_eq!(Named::get_id_secure(Some(&n), "NULL"), "b");
        assert_eq!(Named::get_id_secure::<Named>(None, "NULL"), "NULL");
    }

    #[test]
    fn id_comparator_is_bytewise() {
        let (a, b) = (Named::new("a10"), Named::new("a9"));
        // "a10" < "a9" as std::string (no numerical ordering)
        assert!(id_less(&a, &b));
        assert!(!id_less(&b, &a));
        assert!(!id_less(&a, &a));
    }

    #[test]
    fn num_id_key_orders_by_id_not_address() {
        let a = Named::new("a");
        let b = Named::new("b");
        // ids 9 and 10: numerical order 9 < 10 even if the addresses compare the other way
        let k9 = NumIdKey::new(Ptr::new(&a), 9);
        let k10 = NumIdKey::new(Ptr::new(&b), 10);
        assert!(k9 < k10);
        assert!(!(k10 < k9));
        // equal ids are equivalent keys (C++ std::map emplace ignores the 2nd insert)
        let k9b = NumIdKey::new(Ptr::new(&b), 9);
        assert_eq!(k9, k9b);
    }

    #[test]
    fn id_key_orders_by_string() {
        let a = Named::new("a10");
        let b = Named::new("a9");
        let ka = IdKey::new(Ptr::new(&a), "a10");
        let kb = IdKey::new(Ptr::new(&b), "a9");
        assert!(ka < kb);
        assert_eq!(ka, IdKey::new(Ptr::new(&a), "a10"));
    }
}
