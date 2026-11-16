//! Port of utils/common/Named.h
// shortcut: ComparatorIdLess / ComparatorNumericalIdLess are not ported;
// sort with `sort_by(|a, b| a.get_id().cmp(b.get_id()))` until a container needs them
use super::ptr::Ptr;
use super::to_string::ToStringPrec;
use std::collections::BTreeSet;

/// Anything with an id (the virtual getID() of Named subclasses)
pub trait HasId {
    fn get_id(&self) -> &str;
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
}
