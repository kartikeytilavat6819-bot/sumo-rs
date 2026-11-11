//! Port of utils/common/Named.h
// shortcut: ComparatorIdLess / ComparatorNumericalIdLess and StoringVisitor are not ported;
// sort with `sort_by(|a, b| a.get_id().cmp(b.get_id()))` until a container needs them
use super::to_string::ToStringPrec;

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
