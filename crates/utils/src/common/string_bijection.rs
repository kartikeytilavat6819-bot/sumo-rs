//! Port of utils/common/StringBijection.h
use super::util_exceptions::ProcessError;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct StringBijection<T: Ord + Copy> {
    string2t: BTreeMap<String, T>,
    t2string: BTreeMap<T, String>,
}

impl<T: Ord + Copy> Default for StringBijection<T> {
    fn default() -> Self {
        StringBijection { string2t: BTreeMap::new(), t2string: BTreeMap::new() }
    }
}

impl<T: Ord + Copy> StringBijection<T> {
    /// StringBijection(entries, terminatorKey, checkDuplicates): the slice end is the terminator
    pub fn new(entries: &[(&str, T)], check_duplicates: bool) -> Result<Self, ProcessError> {
        let mut b = Self::default();
        for (s, k) in entries {
            b.insert(s, *k, check_duplicates)?;
        }
        Ok(b)
    }

    pub fn insert(&mut self, s: &str, key: T, check_duplicates: bool) -> Result<(), ProcessError> {
        if check_duplicates {
            if self.has(key) {
                return Err(ProcessError::InvalidArgument("Duplicate key.".into()));
            }
            if self.has_string(s) {
                return Err(ProcessError::InvalidArgument(format!("Duplicate string '{s}'.")));
            }
        }
        self.string2t.insert(s.to_string(), key);
        self.t2string.insert(key, s.to_string());
        Ok(())
    }

    pub fn add_alias(&mut self, s: &str, key: T) {
        self.string2t.insert(s.to_string(), key);
    }

    pub fn remove(&mut self, s: &str, key: T) {
        self.string2t.remove(s);
        self.t2string.remove(&key);
    }

    pub fn get(&self, s: &str) -> Result<T, ProcessError> {
        self.string2t.get(s).copied().ok_or_else(|| ProcessError::InvalidArgument(format!("String '{s}' not found.")))
    }

    pub fn get_string(&self, key: T) -> Result<&str, ProcessError> {
        self.t2string.get(&key).map(String::as_str).ok_or_else(|| ProcessError::InvalidArgument("Key not found.".into()))
    }

    pub fn has_string(&self, s: &str) -> bool {
        self.string2t.contains_key(s)
    }

    pub fn has(&self, key: T) -> bool {
        self.t2string.contains_key(&key)
    }

    pub fn size(&self) -> usize {
        self.string2t.len()
    }

    /// the canonical strings, ordered by key
    pub fn get_strings(&self) -> Vec<String> {
        self.t2string.values().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<T> {
        self.t2string.keys().copied().collect()
    }

    pub fn get_multiline_string(&self) -> String {
        self.get_strings().join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bijection() {
        let b = StringBijection::new(&[("old", 1), ("one", 1), ("two", 2)], false).unwrap();
        assert_eq!(b.get("old").unwrap(), 1);
        assert_eq!(b.get_string(1).unwrap(), "one");
        assert_eq!(b.size(), 3);
        assert_eq!(b.get_strings(), ["one", "two"]);
        assert!(StringBijection::new(&[("a", 1), ("b", 1)], true).is_err());
        assert_eq!(b.get("x").unwrap_err().to_string(), "String 'x' not found.");
    }
}
