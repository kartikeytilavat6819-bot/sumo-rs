//! Port of utils/common/Parameterised.{h,cpp}
// shortcut: writeParams(OutputDevice&) is not ported, add it with utils/iodevices
use super::msg_handler::write_warning;
use super::string_tokenizer::StringTokenizer;
use super::string_utils;
use super::util_exceptions::ProcessError;
use crate::write_warningf;
use std::collections::BTreeMap;

pub type ParameterisedMap = BTreeMap<String, String>;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Parameterised {
    map: ParameterisedMap,
}

/// SUMOXMLDefinitions::isValidParameterKey
fn is_valid_parameter_key(value: &str) -> bool {
    !value.is_empty() && !value.contains(['\t', '\n', '\r', '&', '|', '\\', '\'', '"', '<', '>'])
}

impl Parameterised {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_map(map: ParameterisedMap) -> Self {
        Parameterised { map }
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    pub fn unset_parameter(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn update_parameters(&mut self, map_arg: &ParameterisedMap) {
        for (k, v) in map_arg {
            self.set_parameter(k, v);
        }
    }

    /// separator default " ", unique_values default true
    pub fn merge_parameters(&mut self, map_arg: &ParameterisedMap, separator: &str, unique_values: bool) {
        for (k, v) in map_arg {
            if self.has_parameter(k) {
                let old = self.get_parameter(k, "");
                if !(unique_values && old == *v) {
                    self.set_parameter(k, &format!("{old}{separator}{v}"));
                }
            } else {
                self.set_parameter(k, v);
            }
        }
    }

    pub fn has_parameter(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn get_parameter(&self, key: &str, default_value: &str) -> String {
        self.map.get(key).cloned().unwrap_or_else(|| default_value.to_string())
    }

    pub fn get_double(&self, key: &str, default_value: f64) -> f64 {
        match self.map.get(key) {
            Some(v) => match string_utils::to_double(v) {
                Ok(d) => d,
                Err(ProcessError::EmptyData) => {
                    write_warning("Invalid conversion from string to double (empty value)");
                    default_value
                }
                Err(_) => {
                    write_warningf!("Invalid conversion from string to double (%)", v);
                    default_value
                }
            },
            None => default_value,
        }
    }

    pub fn clear_parameter(&mut self) {
        self.map.clear();
    }

    pub fn get_parameters_map(&self) -> &ParameterisedMap {
        &self.map
    }

    /// kvsep default "=", sep default "|"
    pub fn get_parameters_str(&self, kvsep: &str, sep: &str) -> String {
        self.map.iter().map(|(k, v)| format!("{k}{kvsep}{v}")).collect::<Vec<_>>().join(sep)
    }

    pub fn set_parameters(&mut self, params: &Parameterised) {
        self.map = params.map.clone();
    }

    pub fn set_parameters_vec(&mut self, params: &[(String, String)]) {
        self.map.clear();
        for (k, v) in params {
            self.map.insert(k.clone(), v.clone());
        }
    }

    pub fn set_parameters_str(&mut self, params_string: &str, kvsep: &str, sep: &str) {
        self.map.clear();
        for key_value in StringTokenizer::with_token(params_string, sep, false).get_vector() {
            let kv = StringTokenizer::with_token(&key_value, kvsep, false).get_vector();
            // C++ front()/back() on an empty vector is undefined; skip it
            if let (Some(k), Some(v)) = (kv.first(), kv.last()) {
                self.set_parameter(k, v);
            }
        }
    }

    pub fn are_parameters_valid(value: &str, report: bool, kvsep: &str, sep: &str) -> bool {
        for key_value_str in StringTokenizer::with_token(value, sep, false).get_vector() {
            if !Self::is_parameter_valid(&key_value_str, kvsep, sep) {
                if report {
                    write_warningf!("Invalid format of parameter (%)", key_value_str);
                }
                return false;
            }
        }
        true
    }

    pub fn are_attributes_valid(value: &str, report: bool, kvsep: &str, sep: &str) -> bool {
        for key_value_str in StringTokenizer::with_token(value, sep, false).get_vector() {
            if Self::is_parameter_valid(&key_value_str, kvsep, sep) {
                // C++ tokenizes the whole value here, not the single parameter
                let attr = StringTokenizer::with_token(value, kvsep, false).get_vector().into_iter().next().unwrap_or_default();
                let letter = attr.chars().next().unwrap_or('\0');
                if !letter.is_ascii_alphabetic() {
                    if report {
                        write_warningf!("Invalid format of attribute '%'. Attribute must start with a letter", attr);
                    }
                    return false;
                }
            } else {
                if report {
                    write_warningf!("Invalid format of attribute (%)", key_value_str);
                }
                return false;
            }
        }
        true
    }

    fn is_parameter_valid(value: &str, kvsep: &str, sep: &str) -> bool {
        if value.contains(sep) || !value.contains(kvsep) {
            return false;
        }
        let kv = StringTokenizer::with_token(value, kvsep, false).get_vector();
        kv.len() == 2 && is_valid_parameter_key(&kv[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn params() {
        let mut p = Parameterised::new();
        p.set_parameters_str("a=1|b=x", "=", "|");
        assert_eq!(p.get_parameters_str("=", "|"), "a=1|b=x");
        assert_eq!(p.get_double("a", 0.), 1.);
        assert_eq!(p.get_double("b", 7.), 7.);
        assert_eq!(p.get_double("c", 3.), 3.);
        let mut m = ParameterisedMap::new();
        m.insert("a".into(), "2".into());
        m.insert("c".into(), "3".into());
        p.merge_parameters(&m, " ", true);
        assert_eq!(p.get_parameter("a", ""), "1 2");
        assert!(Parameterised::are_parameters_valid("a=1|b=2", false, "=", "|"));
        assert!(!Parameterised::are_parameters_valid("a=1=2", false, "=", "|"));
        assert!(!Parameterised::are_attributes_valid("1a=2", false, "=", "|"));
    }
}
