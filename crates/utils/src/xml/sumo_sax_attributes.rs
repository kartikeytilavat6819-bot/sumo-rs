//! Port of utils/xml/SUMOSAXAttributes.{h,cpp}, SUMOSAXAttributesImpl_Xerces.{h,cpp} and
//! SUMOSAXAttributesImpl_Cached.{h,cpp}: one concrete type; `cached` marks the Cached flavour
//! (std::map ordered, keyed by local name) produced by `clone()`.
use super::stand_ins::{tokenize, tokenize_white, write_error, Boundary, Position, PositionVector, ProcessError};
use super::sumo_xml_definitions as defs;
use super::sumo_xml_definitions::{FringeType, ParkingType, RightOfWay, RoundaboutType, SumoXMLEdgeFunc, SumoXMLNodeType};
use crate::common::string_utils;
use crate::common::time::{string2time, SUMOTime, SUMOTIME_MAX};
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

/// The encoding of parsed strings
pub const ENCODING: &str = " encoding=\"UTF-8\"";

#[derive(Debug)]
pub struct SUMOSAXAttributes {
    my_object_type: String,
    /// (name, value) in document order; local names sorted by name when `cached`
    my_attrs: Vec<(String, String)>,
    /// attribute id -> name (myPredefinedTagsMML)
    my_predefined_tags_mml: Arc<Vec<String>>,
    cached: bool,
}

/// `fromString<T>` + `invalid_return<T>` of the C++ template getters; implement it for more types
/// (e.g. RGBColor in utils::common) to make them usable with `get`/`get_opt`.
pub trait FromAttr: Sized {
    fn invalid_return() -> Self;
    fn from_attr(value: &str) -> Result<Self, ProcessError>;
}

fn non_empty(value: &str) -> Result<&str, ProcessError> {
    if value.is_empty() { Err(ProcessError::EmptyData) } else { Ok(value) }
}
fn to_int(value: &str) -> Result<i32, ProcessError> {
    string_utils::to_int(non_empty(value)?)
}
fn to_double(value: &str) -> Result<f64, ProcessError> {
    string_utils::to_double(non_empty(value)?)
}
fn to_bool(value: &str) -> Result<bool, ProcessError> {
    string_utils::to_bool(non_empty(value)?).map_err(|_| ProcessError::bool_format(&value.to_ascii_lowercase()))
}

impl FromAttr for String {
    fn invalid_return() -> Self {
        String::new()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        Ok(non_empty(value)?.to_string())
    }
}
impl FromAttr for i32 {
    fn invalid_return() -> Self {
        -1
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        to_int(value)
    }
}
impl FromAttr for i64 {
    fn invalid_return() -> Self {
        -1
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        string_utils::to_long(non_empty(value)?)
    }
}
impl FromAttr for f64 {
    fn invalid_return() -> Self {
        -1.
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        to_double(value)
    }
}
impl FromAttr for bool {
    fn invalid_return() -> Self {
        false
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        to_bool(value)
    }
}
impl FromAttr for Position {
    fn invalid_return() -> Self {
        Position::default()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        // only the first position is used
        if let Some(first) = tokenize_white(value).first() {
            let pos = tokenize(first, ",");
            // check that position has X-Y or X-Y-Z
            if pos.len() != 2 && pos.len() != 3 {
                return Err(ProcessError::Format("is not a valid position".into()));
            }
            let x = to_double(pos[0])?;
            let y = to_double(pos[1])?;
            let z = if pos.len() == 3 { to_double(pos[2])? } else { 0. };
            return Ok(Position::new(x, y, z));
        }
        // empty positions aren't allowed
        Err(ProcessError::Format("is not a valid position".into()))
    }
}
impl FromAttr for PositionVector {
    fn invalid_return() -> Self {
        PositionVector::default()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        let mut shape = PositionVector::default();
        for p in tokenize_white(value) {
            let pos = tokenize(p, ",");
            if pos.len() != 2 && pos.len() != 3 {
                return Err(ProcessError::Format("is not a valid list of positions".into()));
            }
            let x = to_double(pos[0])?;
            let y = to_double(pos[1])?;
            let z = if pos.len() == 3 { to_double(pos[2])? } else { 0. };
            shape.0.push(Position::new(x, y, z));
        }
        Ok(shape)
    }
}
// the real geom::PositionVector (netload/netbuild ports); same parsing as the stand-in above
impl FromAttr for crate::geom::position_vector::PositionVector {
    fn invalid_return() -> Self {
        Self::default()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        let mut shape = Self::default();
        for p in tokenize_white(value) {
            let pos = tokenize(p, ",");
            if pos.len() != 2 && pos.len() != 3 {
                return Err(ProcessError::Format("is not a valid list of positions".into()));
            }
            let x = to_double(pos[0])?;
            let y = to_double(pos[1])?;
            let z = if pos.len() == 3 { to_double(pos[2])? } else { 0. };
            shape.push(crate::geom::position::Position::new3(x, y, z));
        }
        Ok(shape)
    }
}
impl FromAttr for Boundary {
    fn invalid_return() -> Self {
        Boundary::default()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        let st = tokenize(value, ",");
        if st.len() != 4 {
            return Err(ProcessError::Format("is not a valid boundary".into()));
        }
        Ok(Boundary::new(to_double(st[0])?, to_double(st[1])?, to_double(st[2])?, to_double(st[3])?))
    }
}
// the real geom::Position / geom::Boundary (same parsing as the stand-ins above)
impl FromAttr for crate::geom::position::Position {
    fn invalid_return() -> Self {
        Self::default()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        // only the first position is used
        if let Some(first) = tokenize_white(value).first() {
            let pos = tokenize(first, ",");
            // check that position has X-Y or X-Y-Z
            if pos.len() != 2 && pos.len() != 3 {
                return Err(ProcessError::Format("is not a valid position".into()));
            }
            let x = to_double(pos[0])?;
            let y = to_double(pos[1])?;
            let z = if pos.len() == 3 { to_double(pos[2])? } else { 0. };
            return Ok(crate::geom::position::Position::new3(x, y, z));
        }
        // empty positions aren't allowed
        Err(ProcessError::Format("is not a valid position".into()))
    }
}
impl FromAttr for crate::geom::boundary::Boundary {
    fn invalid_return() -> Self {
        Self::default()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        let st = tokenize(value, ",");
        if st.len() != 4 {
            return Err(ProcessError::Format("is not a valid boundary".into()));
        }
        Ok(crate::geom::boundary::Boundary::from_xy(to_double(st[0])?, to_double(st[1])?, to_double(st[2])?, to_double(st[3])?))
    }
}
macro_rules! from_bijection {
    ($t:ty, $invalid:expr, $bij:expr, $msg:literal) => {
        impl FromAttr for $t {
            fn invalid_return() -> Self {
                $invalid
            }
            fn from_attr(value: &str) -> Result<Self, ProcessError> {
                $bij.get(value).map_err(|_| ProcessError::Format($msg.into()))
            }
        }
    };
}
from_bijection!(SumoXMLEdgeFunc, SumoXMLEdgeFunc::NORMAL, defs::EdgeFunctions, "is not a valid edge function");
from_bijection!(SumoXMLNodeType, SumoXMLNodeType::UNKNOWN, defs::NodeTypes, "is not a valid node type");
from_bijection!(RightOfWay, RightOfWay::DEFAULT, defs::RightOfWayValues, "is not a valid right of way value");
from_bijection!(FringeType, FringeType::DEFAULT, defs::FringeTypeValues, "is not a valid fringe type");
from_bijection!(RoundaboutType, RoundaboutType::DEFAULT, defs::RoundaboutTypeValues, "is not a valid roundabout type");
impl FromAttr for ParkingType {
    fn invalid_return() -> Self {
        ParkingType::ONROAD
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        if value == defs::ParkingTypes.get_string(ParkingType::OPPORTUNISTIC) {
            Ok(ParkingType::OPPORTUNISTIC)
        } else {
            Ok(if to_bool(value)? { ParkingType::OFFROAD } else { ParkingType::ONROAD })
        }
    }
}
impl FromAttr for Vec<String> {
    fn invalid_return() -> Self {
        Vec::new()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        let ret: Vec<String> = tokenize_white(value).into_iter().map(String::from).collect();
        if ret.is_empty() { Err(ProcessError::EmptyData) } else { Ok(ret) }
    }
}
impl FromAttr for Vec<i32> {
    fn invalid_return() -> Self {
        Vec::new()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        let tmp = tokenize_white(value);
        if tmp.is_empty() {
            return Err(ProcessError::EmptyData);
        }
        tmp.into_iter().map(to_int).collect()
    }
}
impl FromAttr for Vec<f64> {
    fn invalid_return() -> Self {
        Vec::new()
    }
    fn from_attr(value: &str) -> Result<Self, ProcessError> {
        let tmp = tokenize_white(value);
        if tmp.is_empty() {
            return Err(ProcessError::EmptyData);
        }
        tmp.into_iter().map(to_double).collect()
    }
}

/// string2time with the C++ exception split: EmptyData vs. any other ProcessError
fn parse_time(val: &str) -> Result<SUMOTime, ProcessError> {
    string2time(non_empty(val)?)
}

fn local_name(qname: &str) -> &str {
    qname.rsplit(':').next().unwrap_or(qname)
}

impl SUMOSAXAttributes {
    /// SUMOSAXAttributesImpl_Xerces: `attrs` are (qname, value) in document order
    pub fn new(attrs: Vec<(String, String)>, predefined_tags_mml: Arc<Vec<String>>, object_type: &str) -> Self {
        SUMOSAXAttributes { my_object_type: object_type.to_string(), my_attrs: attrs, my_predefined_tags_mml: predefined_tags_mml, cached: false }
    }
    /// SUMOSAXAttributesImpl_Cached
    pub fn new_cached(attrs: BTreeMap<String, String>, predefined_tags_mml: Arc<Vec<String>>, object_type: &str) -> Self {
        SUMOSAXAttributes { my_object_type: object_type.to_string(), my_attrs: attrs.into_iter().collect(), my_predefined_tags_mml: predefined_tags_mml, cached: true }
    }

    fn find(&self, name: &str) -> Option<&str> {
        self.my_attrs.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
    }
    fn value_of(&self, id: i32) -> Option<&str> {
        self.find(&self.my_predefined_tags_mml[id as usize])
    }

    pub fn get<T: FromAttr>(&self, attr: i32, objectid: &str, ok: &mut bool, report: bool) -> T {
        let err = match self.value_of(attr) {
            Some(v) => match T::from_attr(v) {
                Ok(r) => return r,
                Err(e) => e,
            },
            None => {
                if report {
                    self.emit_ungiven_error(&self.get_name(attr), objectid);
                }
                *ok = false;
                return T::invalid_return();
            }
        };
        if report {
            self.emit_error(attr, &err, objectid);
        }
        *ok = false;
        T::invalid_return()
    }

    pub fn get_opt<T: FromAttr>(&self, attr: i32, objectid: &str, ok: &mut bool, default_value: T, report: bool) -> T {
        let Some(v) = self.value_of(attr) else {
            return default_value;
        };
        match T::from_attr(v) {
            Ok(r) => r,
            Err(e) => {
                if report {
                    self.emit_error(attr, &e, objectid);
                }
                *ok = false;
                T::invalid_return()
            }
        }
    }

    fn emit_error(&self, attr: i32, e: &ProcessError, objectid: &str) {
        match e {
            ProcessError::EmptyData => self.emit_empty_error(&self.get_name(attr), objectid),
            _ => self.emit_format_error(&self.get_name(attr), &e.to_string(), objectid),
        }
    }
    fn time_error(&self, attr: i32, e: &ProcessError, objectid: &str, report: bool) -> SUMOTime {
        if report {
            match e {
                ProcessError::EmptyData => self.emit_empty_error(&self.get_name(attr), objectid),
                _ => self.emit_format_error(&self.get_name(attr), "is not a valid time value", objectid),
            }
        }
        -1
    }

    pub fn get_sumo_time_reporting(&self, attr: i32, objectid: &str, ok: &mut bool, report: bool) -> SUMOTime {
        let Some(val) = self.value_of(attr) else {
            if report {
                self.emit_ungiven_error(&self.get_name(attr), objectid);
            }
            *ok = false;
            return -1;
        };
        parse_time(val).unwrap_or_else(|e| {
            *ok = false;
            self.time_error(attr, &e, objectid, report)
        })
    }

    pub fn get_period(&self, objectid: &str, ok: &mut bool, report: bool) -> SUMOTime {
        let mut attr = defs::SUMO_ATTR_PERIOD;
        let val = match self.value_of(attr) {
            Some(v) => v,
            None => {
                // try 'freq' as alias for 'period'
                attr = defs::SUMO_ATTR_FREQUENCY;
                match self.value_of(attr) {
                    Some(v) => v,
                    None => {
                        if report {
                            self.emit_ungiven_error(&self.get_name(defs::SUMO_ATTR_PERIOD), objectid);
                        }
                        *ok = false;
                        return -1;
                    }
                }
            }
        };
        parse_time(val).unwrap_or_else(|e| {
            *ok = false;
            self.time_error(attr, &e, objectid, report)
        })
    }

    pub fn get_opt_sumo_time_reporting(&self, attr: i32, objectid: &str, ok: &mut bool, default_value: SUMOTime, report: bool) -> SUMOTime {
        let Some(val) = self.value_of(attr) else {
            return default_value;
        };
        parse_time(val).unwrap_or_else(|e| {
            *ok = false;
            self.time_error(attr, &e, objectid, report)
        })
    }

    pub fn get_opt_offset_reporting(&self, attr: i32, objectid: &str, ok: &mut bool, default_value: SUMOTime, report: bool) -> SUMOTime {
        let Some(val) = self.value_of(attr) else {
            return default_value;
        };
        if val == "begin" {
            return SUMOTIME_MAX;
        }
        parse_time(val).unwrap_or_else(|e| {
            *ok = false;
            self.time_error(attr, &e, objectid, report)
        })
    }

    pub fn get_opt_period(&self, objectid: &str, ok: &mut bool, default_value: SUMOTime, report: bool) -> SUMOTime {
        let mut attr = defs::SUMO_ATTR_PERIOD;
        let val = match self.value_of(attr) {
            Some(v) => v,
            None => {
                // try 'freq' as alias for 'period'
                attr = defs::SUMO_ATTR_FREQUENCY;
                match self.value_of(attr) {
                    Some(v) => v,
                    None => return default_value,
                }
            }
        };
        parse_time(val).unwrap_or_else(|e| {
            *ok = false;
            self.time_error(attr, &e, objectid, report)
        })
    }

    pub fn has_attribute(&self, id: i32) -> bool {
        self.value_of(id).is_some()
    }
    /// hasAttribute(const std::string&)
    pub fn has_attribute_name(&self, id: &str) -> bool {
        self.find(id).is_some()
    }
    /// getBool: EmptyData if missing or empty, Format for a bad value
    pub fn get_bool(&self, id: i32) -> Result<bool, ProcessError> {
        to_bool(&self.get_string(id))
    }
    pub fn get_int(&self, id: i32) -> Result<i32, ProcessError> {
        to_int(&self.get_string(id))
    }
    pub fn get_long(&self, id: i32) -> Result<i64, ProcessError> {
        i64::from_attr(&self.get_string(id))
    }
    pub fn get_float(&self, id: i32) -> Result<f64, ProcessError> {
        to_double(&self.get_string(id))
    }
    /// getString(id, &isPresent): None if the attribute is not given
    pub fn get_string_opt(&self, id: i32) -> Option<String> {
        self.value_of(id).map(String::from)
    }
    /// getString(id); "" if not given (C++ dereferences a null isPresent there)
    pub fn get_string(&self, id: i32) -> String {
        self.value_of(id).unwrap_or("").to_string()
    }
    pub fn get_string_secure(&self, id: i32, def: &str) -> String {
        match self.value_of(id) {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => def.to_string(),
        }
    }
    /// getFloat(const std::string&)
    pub fn get_float_name(&self, id: &str) -> Result<f64, ProcessError> {
        to_double(self.find(id).ok_or(ProcessError::EmptyData)?)
    }
    /// getStringSecure(const std::string&, def): the Cached flavour also maps "" to def
    pub fn get_string_secure_name(&self, id: &str, def: &str) -> String {
        match self.find(id) {
            Some(v) if !(self.cached && v.is_empty()) => v.to_string(),
            _ => def.to_string(),
        }
    }
    pub fn get_name(&self, attr: i32) -> String {
        self.my_predefined_tags_mml[attr as usize].clone()
    }
    /// Prints all attribute names and values (operator<<)
    pub fn serialize(&self) -> String {
        self.my_attrs.iter().map(|(k, v)| format!(" {}=\"{v}\"", local_name(k))).collect()
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        self.my_attrs.iter().map(|(k, _)| local_name(k).to_string()).collect()
    }
    pub fn get_object_type(&self) -> &str {
        &self.my_object_type
    }
    /// deep copy as SUMOSAXAttributesImpl_Cached
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> SUMOSAXAttributes {
        let attrs = self.my_attrs.iter().map(|(k, v)| (local_name(k).to_string(), v.clone())).collect();
        Self::new_cached(attrs, self.my_predefined_tags_mml.clone(), &self.my_object_type)
    }

    fn describe(&self, objectid: &str) -> String {
        if objectid.is_empty() { format!("a {}", self.my_object_type) } else { format!("{} '{objectid}'", self.my_object_type) }
    }
    fn emit_ungiven_error(&self, attrname: &str, objectid: &str) {
        write_error(&format!("Attribute '{attrname}' is missing in definition of {}.", self.describe(objectid)));
    }
    fn emit_empty_error(&self, attrname: &str, objectid: &str) {
        write_error(&format!("Attribute '{attrname}' in definition of {} is empty.", self.describe(objectid)));
    }
    fn emit_format_error(&self, attrname: &str, type_: &str, objectid: &str) {
        write_error(&format!("Attribute '{attrname}' in definition of {} {type_}.", self.describe(objectid)));
    }
}

impl fmt::Display for SUMOSAXAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::stand_ins::{clear_errors, error_was_informed};
    use crate::xml::sumo_xml_definitions::*;

    fn names() -> Arc<Vec<String>> {
        Arc::new(Attrs.get_strings())
    }
    fn attrs(a: &[(&str, &str)]) -> SUMOSAXAttributes {
        SUMOSAXAttributes::new(a.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(), names(), "vehicle")
    }

    #[test]
    fn getters() {
        let a = attrs(&[("id", "v0"), ("speed", "13.5"), ("lane", ""), ("depart", "1:00:00"), ("x", "abc"), ("shape", "0,0 1,2,3"), ("vClass", "a  b"), ("period", "5")]);
        let mut ok = true;
        assert_eq!(a.get::<String>(SUMO_ATTR_ID, "", &mut ok, true), "v0");
        assert_eq!(a.get::<f64>(SUMO_ATTR_SPEED, "v0", &mut ok, true), 13.5);
        assert_eq!(a.get_sumo_time_reporting(SUMO_ATTR_DEPART, "v0", &mut ok, true), 3_600_000);
        assert_eq!(a.get::<PositionVector>(SUMO_ATTR_SHAPE, "", &mut ok, true).0, vec![Position::new(0., 0., 0.), Position::new(1., 2., 3.)]);
        assert_eq!(a.get::<Vec<String>>(SUMO_ATTR_VCLASS, "", &mut ok, true), vec!["a", "b"]);
        assert_eq!(a.get_period("", &mut ok, true), 5000);
        assert_eq!(a.get_opt::<i32>(SUMO_ATTR_INDEX, "", &mut ok, 7, true), 7);
        assert!(ok);
        assert_eq!(a.get::<f64>(SUMO_ATTR_X, "v0", &mut ok, false), -1.);
        assert!(!ok);
        assert_eq!(a.get_string_secure(SUMO_ATTR_LANE, "def"), "def");
        assert_eq!(a.get_int(SUMO_ATTR_LANE), Err(ProcessError::EmptyData));
        assert_eq!(a.get_float(SUMO_ATTR_X).unwrap_err().to_string(), "Invalid Number Format (double) abc");
        assert_eq!(a.serialize(), " id=\"v0\" speed=\"13.5\" lane=\"\" depart=\"1:00:00\" x=\"abc\" shape=\"0,0 1,2,3\" vClass=\"a  b\" period=\"5\"");
        let c = a.clone();
        assert_eq!(c.get_attribute_names()[0], "depart");
        assert_eq!(c.get_string_secure_name("lane", "d"), "d");
        assert_eq!(a.get_string_secure_name("lane", "d"), "");
        assert_eq!(a.get::<Boundary>(SUMO_ATTR_SHAPE, "", &mut ok, false), Boundary::default());
        let b = attrs(&[("convBoundary", "3,4,1,2")]).get::<Boundary>(SUMO_ATTR_CONV_BOUNDARY, "", &mut ok, true);
        assert_eq!((b.xmin, b.ymin, b.xmax, b.ymax), (1., 2., 3., 4.));
    }

    #[test]
    fn error_messages() {
        let a = attrs(&[("speed", "x"), ("lane", ""), ("begin", "1:2")]);
        let mut ok = true;
        assert_eq!(a.describe(""), "a vehicle");
        assert_eq!(a.describe("v0"), "vehicle 'v0'");
        clear_errors();
        a.get::<String>(SUMO_ATTR_ID, "v0", &mut ok, true); // Attribute 'id' is missing in definition of vehicle 'v0'.
        assert!(error_was_informed() && !ok);
        // the emitted texts, as built by the emit_* helpers
        let e = ProcessError::number_format("(double) x");
        assert_eq!(format!("Attribute 'speed' in definition of {} {e}.", a.describe("v0")), "Attribute 'speed' in definition of vehicle 'v0' Invalid Number Format (double) x.");
        assert_eq!(a.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut ok, true), -1);
        assert_eq!(a.get_opt_offset_reporting(SUMO_ATTR_OFFSET, "", &mut ok, 3, true), 3);
        assert!(!a.get::<bool>(SUMO_ATTR_LANE, "", &mut ok, true));
        assert_eq!(bool::from_attr("Maybe").unwrap_err().to_string(), "Invalid Bool Format maybe");
        assert_eq!(ParkingType::from_attr("opportunistic").unwrap(), ParkingType::OPPORTUNISTIC);
        assert_eq!(ParkingType::from_attr("true").unwrap(), ParkingType::OFFROAD);
        assert_eq!(SumoXMLNodeType::from_attr("x").unwrap_err().to_string(), "is not a valid node type");
    }
}
