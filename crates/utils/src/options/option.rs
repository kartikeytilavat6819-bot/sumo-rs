//! Port of utils/options/Option.{h,cpp}
//! The C++ subclasses (Option_Integer, Option_String, ...) are the variants of `OptionValue`;
//! the class itself is `SumoOption` (to not clash with Rust's `Option`).
use crate::common::msg_handler::write_warning;
use crate::common::string_tokenizer::StringTokenizer;
use crate::common::string_utils;
use crate::common::to_string::{f64_to_string_default, join_to_string, to_string};
use crate::common::util_exceptions::ProcessError;
use crate::tlf;

pub type IntVector = Vec<i32>;
pub type StringVector = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    Integer(i32),
    String(String),
    Float(f64),
    Bool(bool),
    BoolExtended(bool),
    IntVector(IntVector),
    StringVector(StringVector),
    FileName(StringVector),
    Network(String),
    Additional(String),
    Route(String),
    Data(String),
    SumoConfig(String),
    Edge(String),
    EdgeVector(String),
}

#[derive(Debug, Clone)]
pub struct SumoOption {
    value: OptionValue,
    type_name: String,
    value_string: String,
    am_set: bool,
    have_the_default_value: bool,
    am_writable: bool,
    description: String,
    required: bool,
    positional: bool,
    editable: bool,
    list_separator: String,
    sub_topic: String,
}

impl SumoOption {
    /// Option(set)
    fn base(value: OptionValue, set: bool, type_name: &str, value_string: String) -> Self {
        SumoOption {
            value,
            type_name: type_name.to_string(),
            value_string,
            am_set: set,
            have_the_default_value: true,
            am_writable: true,
            description: String::new(),
            required: false,
            positional: false,
            editable: true,
            list_separator: String::new(),
            sub_topic: String::new(),
        }
    }

    // ----- constructors of the subclasses -----
    pub fn integer(value: i32) -> Self {
        Self::base(OptionValue::Integer(value), true, "INT", to_string(&value))
    }
    /// Option_String() (unset)
    pub fn string() -> Self {
        Self::base(OptionValue::String(String::new()), false, "STR", String::new())
    }
    /// Option_String(value, typeName = "STR")
    pub fn string_with(value: &str) -> Self {
        Self::string_typed(value, "STR")
    }
    pub fn string_typed(value: &str, type_name: &str) -> Self {
        Self::base(OptionValue::String(value.to_string()), true, type_name, value.to_string())
    }
    pub fn float(value: f64) -> Self {
        Self::base(OptionValue::Float(value), true, "FLOAT", f64_to_string_default(value))
    }
    pub fn bool(value: bool) -> Self {
        Self::base(OptionValue::Bool(value), true, "BOOL", if value { "true" } else { "false" }.into())
    }
    pub fn bool_extended(value: bool) -> Self {
        Self::base(OptionValue::BoolExtended(value), true, "BOOL", if value { "true" } else { "false" }.into())
    }
    pub fn int_vector() -> Self {
        Self::base(OptionValue::IntVector(Vec::new()), false, "INT[]", String::new())
    }
    pub fn int_vector_with(value: IntVector) -> Self {
        let vs = join_to_string(&value, ",");
        Self::base(OptionValue::IntVector(value), true, "INT[]", vs)
    }
    pub fn string_vector() -> Self {
        Self::base(OptionValue::StringVector(Vec::new()), false, "STR[]", String::new())
    }
    pub fn string_vector_with(value: StringVector) -> Self {
        let vs = join_to_string(&value, ",");
        Self::base(OptionValue::StringVector(value), true, "STR[]", vs)
    }
    pub fn file_name() -> Self {
        Self::base(OptionValue::FileName(Vec::new()), false, "FILE", String::new())
    }
    pub fn file_name_with(value: StringVector) -> Self {
        let vs = join_to_string(&value, ",");
        Self::base(OptionValue::FileName(value), true, "FILE", vs)
    }
    pub fn network(value: &str) -> Self {
        Self::base(OptionValue::Network(value.into()), true, "NETWORK", value.into())
    }
    pub fn additional(value: &str) -> Self {
        Self::base(OptionValue::Additional(value.into()), true, "ADDITIONAL", value.into())
    }
    pub fn route(value: &str) -> Self {
        Self::base(OptionValue::Route(value.into()), true, "ROUTE", value.into())
    }
    pub fn data(value: &str) -> Self {
        Self::base(OptionValue::Data(value.into()), true, "DATA", value.into())
    }
    pub fn sumo_config(value: &str) -> Self {
        Self::base(OptionValue::SumoConfig(value.into()), true, "SUMOCONFIG", value.into())
    }
    pub fn edge(value: &str) -> Self {
        Self::base(OptionValue::Edge(value.into()), true, "EDGE", value.into())
    }
    pub fn edge_vector(value: &str) -> Self {
        Self::base(OptionValue::EdgeVector(value.into()), true, "EDGE[]", value.into())
    }

    pub fn value(&self) -> &OptionValue {
        &self.value
    }

    pub fn is_set(&self) -> bool {
        self.am_set
    }

    pub fn get_float(&self) -> Result<f64, ProcessError> {
        match self.value {
            OptionValue::Float(v) => Ok(v),
            _ => Err(ProcessError::InvalidArgument("This is not a double-option".into())),
        }
    }

    pub fn get_int(&self) -> Result<i32, ProcessError> {
        match self.value {
            OptionValue::Integer(v) => Ok(v),
            _ => Err(ProcessError::InvalidArgument("This is not an int-option".into())),
        }
    }

    pub fn get_string(&self) -> Result<String, ProcessError> {
        match &self.value {
            OptionValue::String(v)
            | OptionValue::Network(v)
            | OptionValue::Additional(v)
            | OptionValue::Route(v)
            | OptionValue::Data(v)
            | OptionValue::SumoConfig(v)
            | OptionValue::Edge(v)
            | OptionValue::EdgeVector(v) => Ok(v.clone()),
            OptionValue::FileName(v) => Ok(join_to_string(v, ",")),
            _ => Err(ProcessError::InvalidArgument("This is not a string-option".into())),
        }
    }

    pub fn get_bool(&self) -> Result<bool, ProcessError> {
        match self.value {
            OptionValue::Bool(v) | OptionValue::BoolExtended(v) => Ok(v),
            _ => Err(ProcessError::InvalidArgument("This is not a bool-option".into())),
        }
    }

    pub fn get_int_vector(&self) -> Result<&IntVector, ProcessError> {
        match &self.value {
            OptionValue::IntVector(v) => Ok(v),
            _ => Err(ProcessError::InvalidArgument("This is not an int vector-option".into())),
        }
    }

    pub fn get_string_vector(&self) -> Result<&StringVector, ProcessError> {
        match &self.value {
            OptionValue::StringVector(v) | OptionValue::FileName(v) => Ok(v),
            _ => Err(ProcessError::InvalidArgument("This is not a string vector-option".into())),
        }
    }

    /// Stores the given value; returns whether the option was writable.
    pub fn set(&mut self, v: &str, orig: &str, append: bool) -> Result<bool, ProcessError> {
        match &mut self.value {
            OptionValue::Integer(val) => match string_utils::to_int(v) {
                Ok(i) => {
                    *val = i;
                    Ok(self.mark_set(orig))
                }
                Err(_) => Err(ProcessError::new(format!("'{v}' is not a valid integer."))),
            },
            OptionValue::String(val)
            | OptionValue::Network(val)
            | OptionValue::Additional(val)
            | OptionValue::Route(val)
            | OptionValue::Data(val)
            | OptionValue::SumoConfig(val)
            | OptionValue::Edge(val)
            | OptionValue::EdgeVector(val) => {
                *val = v.to_string();
                Ok(self.mark_set(orig))
            }
            OptionValue::Float(val) => match string_utils::to_double(v) {
                Ok(d) => {
                    *val = d;
                    Ok(self.mark_set(orig))
                }
                Err(_) => Err(ProcessError::new(tlf!("'%' is not a valid float.", v))),
            },
            OptionValue::Bool(val) => match string_utils::to_bool(v) {
                Ok(b) => {
                    *val = b;
                    Ok(self.mark_set(orig))
                }
                Err(_) => Err(ProcessError::new(tlf!("'%' is not a valid bool.", v))),
            },
            OptionValue::BoolExtended(val) => match string_utils::to_bool(v) {
                Ok(b) => {
                    *val = b;
                    Ok(self.mark_set(""))
                }
                Err(_) => {
                    *val = true;
                    Ok(self.mark_set(orig))
                }
            },
            OptionValue::IntVector(val) => {
                if !append {
                    val.clear();
                }
                if v.contains(';') {
                    write_warning("Please note that using ';' as list separator is deprecated and not accepted anymore.");
                }
                let mut st = StringTokenizer::with_token(v, ",", true);
                while st.has_next() {
                    match string_utils::to_int(&st.next()?) {
                        Ok(i) => val.push(i),
                        Err(ProcessError::EmptyData) => {
                            return Err(ProcessError::new(format!("Empty element occurred in {v}")));
                        }
                        Err(_) => return Err(ProcessError::new(tlf!("'%' is not a valid integer vector.", v))),
                    }
                }
                Ok(self.mark_set(orig))
            }
            OptionValue::StringVector(val) | OptionValue::FileName(val) => {
                if !append {
                    val.clear();
                }
                let mut st = StringTokenizer::with_token(v, ",", false);
                while st.has_next() {
                    val.push(string_utils::prune(&st.next()?));
                }
                let orig = if append && !self.value_string.is_empty() {
                    format!("{},{}", self.value_string, orig)
                } else {
                    orig.to_string()
                };
                Ok(self.mark_set(&orig))
            }
        }
    }

    fn mark_set(&mut self, orig: &str) -> bool {
        let ret = self.am_writable;
        self.have_the_default_value = false;
        self.am_set = true;
        self.am_writable = false;
        self.value_string = orig.to_string();
        ret
    }

    pub fn get_value_string(&self) -> &str {
        &self.value_string
    }

    pub fn is_default(&self) -> bool {
        self.have_the_default_value
    }

    pub fn is_integer(&self) -> bool {
        matches!(self.value, OptionValue::Integer(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self.value, OptionValue::Float(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.value, OptionValue::Bool(_) | OptionValue::BoolExtended(_))
    }

    pub fn is_file_name(&self) -> bool {
        matches!(self.value, OptionValue::FileName(_))
    }

    pub fn is_network(&self) -> bool {
        matches!(self.value, OptionValue::Network(_))
    }

    pub fn is_additional(&self) -> bool {
        matches!(self.value, OptionValue::Additional(_))
    }

    pub fn is_route(&self) -> bool {
        matches!(self.value, OptionValue::Route(_))
    }

    pub fn is_data(&self) -> bool {
        matches!(self.value, OptionValue::Data(_))
    }

    pub fn is_sumo_config(&self) -> bool {
        matches!(self.value, OptionValue::SumoConfig(_))
    }

    pub fn is_edge(&self) -> bool {
        matches!(self.value, OptionValue::Edge(_))
    }

    pub fn is_edge_vector(&self) -> bool {
        matches!(self.value, OptionValue::EdgeVector(_))
    }

    pub fn is_writeable(&self) -> bool {
        self.am_writable
    }

    pub fn reset_writable(&mut self) {
        self.am_writable = true;
    }

    pub fn reset_default(&mut self) {
        // v1.28.0 release only sets the default flag; the snapshot's extra `if value_string.is_empty() { am_set = false }` is gone.
        self.have_the_default_value = true;
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, desc: &str) {
        self.description = desc.to_string();
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn set_required(&mut self) {
        self.required = true;
    }

    pub fn is_positional(&self) -> bool {
        self.positional
    }

    pub fn set_positional(&mut self) {
        self.positional = true;
    }

    pub fn is_editable(&self) -> bool {
        self.editable
    }

    pub fn set_editable(&mut self, value: bool) {
        self.editable = value;
    }

    pub fn get_list_separator(&self) -> &str {
        &self.list_separator
    }

    pub fn set_list_separator(&mut self, list_sep: &str) {
        self.list_separator = list_sep.to_string();
    }

    pub fn get_sub_topic(&self) -> &str {
        &self.sub_topic
    }

    pub fn set_subtopic(&mut self, subtopic: &str) {
        self.sub_topic = subtopic.to_string();
    }

    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        assert_eq!(SumoOption::float(0.1).get_value_string(), "0.1");
        assert_eq!(SumoOption::float(3.).get_value_string(), "3");
        assert_eq!(SumoOption::integer(7).get_value_string(), "7");
        assert_eq!(SumoOption::bool(false).get_value_string(), "false");
        assert!(!SumoOption::string().is_set());
        assert!(SumoOption::string_with("").is_set());
        assert_eq!(SumoOption::int_vector_with(vec![1, 2]).get_value_string(), "1,2");
        assert_eq!(SumoOption::network("").get_type_name(), "NETWORK");
    }

    #[test]
    fn set_values() {
        let mut o = SumoOption::integer(1);
        assert!(o.set("5", "5", false).unwrap());
        assert_eq!(o.get_int().unwrap(), 5);
        assert!(!o.is_default());
        // second set: not writable any more but still stored
        assert!(!o.set("6", "6", false).unwrap());
        assert_eq!(o.set("x", "x", false).unwrap_err().to_string(), "'x' is not a valid integer.");
        assert_eq!(o.get_bool().unwrap_err().to_string(), "This is not a bool-option");

        let mut f = SumoOption::float(1.);
        assert_eq!(f.set("a", "a", false).unwrap_err().to_string(), "'a' is not a valid float.");

        let mut b = SumoOption::bool_extended(false);
        b.set("foo", "foo", false).unwrap();
        assert!(b.get_bool().unwrap());
        assert_eq!(b.get_value_string(), "foo");

        let mut v = SumoOption::int_vector();
        assert_eq!(v.set("1,,2", "1,,2", false).unwrap_err().to_string(), "Empty element occurred in 1,,2");
        let mut v = SumoOption::int_vector();
        v.set("1,2", "1,2", false).unwrap();
        assert_eq!(v.get_int_vector().unwrap(), &vec![1, 2]);

        let mut s = SumoOption::file_name();
        s.set("a, b", "a, b", false).unwrap();
        s.reset_writable();
        s.set("c", "c", true).unwrap();
        assert_eq!(s.get_string_vector().unwrap(), &vec!["a", "b", "c"]);
        assert_eq!(s.get_value_string(), "a, b,c");
        assert_eq!(s.get_string().unwrap(), "a,b,c");
    }

    #[test]
    fn reset_default() {
        let mut o = SumoOption::string();
        o.set("", "", false).unwrap();
        o.reset_default();
        // v1.28.0: resetDefault only restores the default flag; isSet stays true (snapshot cleared it for empty value strings).
        assert!(o.is_set() && o.is_default());
    }
}
