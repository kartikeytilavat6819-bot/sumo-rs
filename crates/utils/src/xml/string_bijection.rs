//! Port of utils/common/StringBijection.h and utils/common/SequentialStringBijection.h
// shortcut: lives in xml until utils::common has its own port, then replace with utils::common
use super::stand_ins::ProcessError;
use std::collections::BTreeMap;

/// Ordered like the C++ std::maps, so getStrings/getValues keep the C++ order.
#[derive(Debug, Clone, Default)]
pub struct StringBijection<T: Copy + Ord> {
    my_string2t: BTreeMap<String, T>,
    my_t2string: BTreeMap<T, String>,
}

impl<T: Copy + Ord> StringBijection<T> {
    /// Inserts entries up to and including the one with `terminator_key`.
    pub fn new(entries: &[(&str, T)], terminator_key: T, check_duplicates: bool) -> Self {
        let mut b = StringBijection { my_string2t: BTreeMap::new(), my_t2string: BTreeMap::new() };
        for &(s, key) in entries {
            b.insert(s, key, check_duplicates).unwrap_or_else(|e| panic!("{e}"));
            if key == terminator_key {
                break;
            }
        }
        b
    }
    pub fn insert(&mut self, str: &str, key: T, check_duplicates: bool) -> Result<(), ProcessError> {
        if check_duplicates {
            if self.has(key) {
                return Err(ProcessError::InvalidArgument("Duplicate key.".into()));
            }
            if self.has_string(str) {
                return Err(ProcessError::InvalidArgument(format!("Duplicate string '{str}'.")));
            }
        }
        self.my_string2t.insert(str.to_string(), key);
        self.my_t2string.insert(key, str.to_string());
        Ok(())
    }
    pub fn add_alias(&mut self, str: &str, key: T) {
        self.my_string2t.insert(str.to_string(), key);
    }
    pub fn remove(&mut self, str: &str, key: T) {
        self.my_string2t.remove(str);
        self.my_t2string.remove(&key);
    }
    pub fn get(&self, str: &str) -> Result<T, ProcessError> {
        self.my_string2t.get(str).copied().ok_or_else(|| ProcessError::InvalidArgument(format!("String '{str}' not found.")))
    }
    /// C++ throws InvalidArgument("Key not found."); for enum keys that is a programming error, so we panic.
    pub fn get_string(&self, key: T) -> &str {
        self.my_t2string.get(&key).map(String::as_str).expect("Key not found.")
    }
    pub fn has_string(&self, str: &str) -> bool {
        self.my_string2t.contains_key(str)
    }
    pub fn has(&self, key: T) -> bool {
        self.my_t2string.contains_key(&key)
    }
    pub fn size(&self) -> i32 {
        self.my_string2t.len() as i32
    }
    pub fn get_strings(&self) -> Vec<String> {
        self.my_t2string.values().cloned().collect()
    }
    pub fn get_values(&self) -> Vec<T> {
        self.my_t2string.keys().copied().collect()
    }
    pub fn get_multiline_string(&self) -> String {
        self.get_strings().join("\n")
    }
}

#[derive(Debug, Clone, Default)]
pub struct SequentialStringBijection {
    my_string2t: BTreeMap<String, i32>,
    my_t2string: Vec<String>,
}

impl SequentialStringBijection {
    pub fn new(entries: &[(&str, i32)], terminator_key: i32, check_duplicates: bool) -> Self {
        let mut b = SequentialStringBijection {
            my_string2t: BTreeMap::new(),
            my_t2string: vec![String::new(); terminator_key as usize + 1],
        };
        for &(s, key) in entries {
            b.insert(s, key, check_duplicates).unwrap_or_else(|e| panic!("{e}"));
            if key == terminator_key {
                break;
            }
        }
        b
    }
    pub fn insert(&mut self, str: &str, key: i32, check_duplicates: bool) -> Result<(), ProcessError> {
        if check_duplicates {
            if self.has(key) {
                return Err(ProcessError::InvalidArgument("Duplicate key.".into()));
            }
            if self.has_string(str) {
                return Err(ProcessError::InvalidArgument(format!("Duplicate string '{str}'.")));
            }
        }
        self.my_string2t.insert(str.to_string(), key);
        self.my_t2string[key as usize] = str.to_string();
        Ok(())
    }
    pub fn add_alias(&mut self, str: &str, key: i32) {
        self.my_string2t.insert(str.to_string(), key);
    }
    pub fn remove(&mut self, str: &str, key: i32) {
        self.my_string2t.remove(str);
        self.my_t2string[key as usize].clear();
    }
    pub fn get(&self, str: &str) -> Result<i32, ProcessError> {
        self.my_string2t.get(str).copied().ok_or_else(|| ProcessError::InvalidArgument(format!("String '{str}' not found.")))
    }
    /// Unregistered keys below the terminator give ""; C++ throws "Key not found." beyond it (we panic).
    pub fn get_string(&self, key: i32) -> &str {
        self.my_t2string.get(key as usize).map(String::as_str).expect("Key not found.")
    }
    pub fn has_string(&self, str: &str) -> bool {
        self.my_string2t.contains_key(str)
    }
    pub fn has(&self, key: i32) -> bool {
        self.my_t2string.get(key as usize).is_some_and(|s| !s.is_empty())
    }
    pub fn size(&self) -> i32 {
        self.my_string2t.len() as i32
    }
    pub fn get_strings(&self) -> Vec<String> {
        self.my_t2string.clone()
    }
    pub fn get_values(&self) -> Vec<i32> {
        self.my_string2t.values().copied().collect()
    }
}
