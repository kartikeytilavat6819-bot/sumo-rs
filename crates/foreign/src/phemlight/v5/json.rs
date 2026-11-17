//! Helpers that reproduce the `nlohmann::json` calls used by foreign/PHEMlight/V5/cpp
//! (`at`, `contains`, `get<T>`, `items`, `operator[]`). The C++ code lets these throw; here they
//! return `Err` with the nlohmann-style message, and callers propagate it.

use std::collections::BTreeMap;

use serde_json::Value;

/// Parses a whole file the way `std::ifstream >> nlohmann::json` does: a leading UTF-8 BOM is skipped
/// (PHEMlight5 files carry one; serde_json rejects it).
pub fn parse(text: &str) -> Option<Value> {
    serde_json::from_str(text.strip_prefix('\u{feff}').unwrap_or(text)).ok()
}

/// nlohmann type name used in its exception messages.
pub fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// `json.at(key)` on an object.
pub fn at<'a>(v: &'a Value, key: &str) -> Result<&'a Value, String> {
    match v {
        Value::Object(map) => map
            .get(key)
            .ok_or_else(|| format!("[json.exception.out_of_range.403] key '{}' not found", key)),
        other => Err(format!("[json.exception.type_error.304] cannot use at() with {}", type_name(other))),
    }
}

/// `json.contains(key)`; false for non-objects.
pub fn contains(v: &Value, key: &str) -> bool {
    v.as_object().is_some_and(|map| map.contains_key(key))
}

/// `json.get<double>()`
pub fn get_f64(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .ok_or_else(|| format!("[json.exception.type_error.302] type must be number, but is {}", type_name(v)))
}

/// `json.get<std::string>()`
pub fn get_string(v: &Value) -> Result<String, String> {
    v.as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("[json.exception.type_error.302] type must be string, but is {}", type_name(v)))
}

/// `json.get<std::vector<double>>()`
pub fn get_f64_vec(v: &Value) -> Result<Vec<f64>, String> {
    match v {
        Value::Array(items) => items.iter().map(get_f64).collect(),
        other => Err(format!("[json.exception.type_error.302] type must be array, but is {}", type_name(other))),
    }
}

/// `json.get<std::map<std::string, std::vector<double>>>()`
pub fn get_f64_map(v: &Value) -> Result<BTreeMap<String, Vec<f64>>, String> {
    match v {
        Value::Object(map) => map.iter().map(|(k, val)| Ok((k.clone(), get_f64_vec(val)?))).collect(),
        other => Err(format!("[json.exception.type_error.302] type must be object, but is {}", type_name(other))),
    }
}

/// `json[index]` on an array; the C++ operator is unchecked, here an out-of-range index is an error.
pub fn index(v: &Value, i: usize) -> Result<&Value, String> {
    match v {
        Value::Array(items) => items
            .get(i)
            .ok_or_else(|| format!("[json.exception.out_of_range.401] array index {} is out of range", i)),
        other => Err(format!("[json.exception.type_error.305] cannot use operator[] with {}", type_name(other))),
    }
}

/// `for (auto& it : json.items())`: object members in key order (nlohmann's default std::map order).
pub fn items(v: &Value) -> Result<Vec<(&String, &Value)>, String> {
    match v {
        Value::Object(map) => Ok(map.iter().collect()),
        other => Err(format!("[json.exception.type_error.306] cannot use items() with {}", type_name(other))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_skips_utf8_bom_like_nlohmann() {
        // first bytes of PHEMlight5/PC_EU4_G.PHEMLight.veh (EF BB BF), values from its VehicleData
        let text = "\u{feff}{\n  \"VehicleData\": {\"Mass\": 1193.7, \"Cw\": 0.3745, \"A\": 2.12}\n}";
        let root = parse(text).expect("BOM-prefixed JSON must parse");
        assert_eq!(get_f64(at(at(&root, "VehicleData").unwrap(), "Mass").unwrap()).unwrap(), 1193.7);
        assert_eq!(get_f64(at(at(&root, "VehicleData").unwrap(), "Cw").unwrap()).unwrap(), 0.3745);
        assert!(parse("not json").is_none());
    }
}
