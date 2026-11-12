//! Port of netimport/NIImporter_OpenDrive.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, write_error, write_warning};
use utils::tlf;
use utils::xml::generic_sax_handler::SaxHandler;

// C++ static members of NIImporter_OpenDrive (see comment block below the struct)
thread_local! {
    static MY_IMPORT_ALL_TYPES: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static MY_IMPORT_WIDTHS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static MY_MIN_WIDTH: std::cell::Cell<f64> = const { std::cell::Cell::new(0.0) };
    static MY_IMPORT_INTERNAL_SHAPES: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static MY_IGNORE_MISPLACED_SIGNALS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

// C++ NIImporter_OpenDrive.h: `#define UNSET_CONNECTION 100000`
const UNSET_CONNECTION: i32 = 100000;
// C++ NIImporter_OpenDrive.h: `#define UNSET_LANEVALIDITY 100000`
const UNSET_LANEVALIDITY: i32 = 100000;
/// C++ static NIImporter_OpenDrive::myDummyController
static MY_DUMMY_CONTROLLER: std::sync::OnceLock<NIImporter_OpenDrive_OpenDriveController> = std::sync::OnceLock::new();

/// C++ static NIImporter_OpenDrive::openDriveTags (SequentialStringBijection::Entry[]), terminated by the empty entry
const OPEN_DRIVE_TAGS: &[(&str, i32)] = &[
    ("header", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_HEADER as i32),
    ("road", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_ROAD as i32),
    ("predecessor", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_PREDECESSOR as i32),
    ("successor", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_SUCCESSOR as i32),
    ("geometry", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_GEOMETRY as i32),
    ("line", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LINE as i32),
    ("spiral", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_SPIRAL as i32),
    ("arc", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_ARC as i32),
    ("laneSection", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LANESECTION as i32),
    ("laneOffset", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LANEOFFSET as i32),
    ("left", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT as i32),
    ("center", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_CENTER as i32),
    ("right", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT as i32),
    ("lane", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LANE as i32),
    ("access", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_ACCESS as i32),
    ("signal", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_SIGNAL as i32),
    ("signalReference", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_SIGNALREFERENCE as i32),
    ("controller", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_CONTROLLER as i32),
    ("control", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_CONTROL as i32),
    ("validity", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_VALIDITY as i32),
    ("semantics", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_SEMANTICS as i32),
    ("priority", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_PRIORITY as i32),
    ("junction", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_JUNCTION as i32),
    ("connection", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_CONNECTION as i32),
    ("laneLink", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LANELINK as i32),
    ("width", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_WIDTH as i32),
    ("speed", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_SPEED as i32),
    ("elevation", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_ELEVATION as i32),
    ("geoReference", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_GEOREFERENCE as i32),
    ("offset", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_OFFSET as i32),
    ("object", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_OBJECT as i32),
    ("repeat", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_REPEAT as i32),
    ("include", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_INCLUDE as i32),
    ("", NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_NOTHING as i32),
];
/// C++ static NIImporter_OpenDrive::openDriveAttrs (SequentialStringBijection::Entry[]), terminated by the empty entry
const OPEN_DRIVE_ATTRS: &[(&str, i32)] = &[
    ("revMajor", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_REVMAJOR as i32),
    ("revMinor", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_REVMINOR as i32),
    ("id", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_ID as i32),
    ("length", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_LENGTH as i32),
    ("width", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_WIDTH as i32),
    ("radius", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_RADIUS as i32),
    ("distance", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_DISTANCE as i32),
    ("tStart", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_TSTART as i32),
    ("tEnd", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_TEND as i32),
    ("widthStart", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_WIDTHSTART as i32),
    ("widthEnd", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_WIDTHEND as i32),
    ("junction", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_JUNCTION as i32),
    ("elementType", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_ELEMENTTYPE as i32),
    ("elementId", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_ELEMENTID as i32),
    ("contactPoint", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_CONTACTPOINT as i32),
    ("s", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_S as i32),
    ("t", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_T as i32),
    ("x", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_X as i32),
    ("y", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_Y as i32),
    ("z", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_Z as i32),
    ("hdg", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_HDG as i32),
    ("curvStart", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_CURVSTART as i32),
    ("curvEnd", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_CURVEND as i32),
    ("curvature", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_CURVATURE as i32),
    ("a", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_A as i32),
    ("b", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_B as i32),
    ("c", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_C as i32),
    ("d", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_D as i32),
    ("aU", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_AU as i32),
    ("bU", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_BU as i32),
    ("cU", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_CU as i32),
    ("dU", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_DU as i32),
    ("aV", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_AV as i32),
    ("bV", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_BV as i32),
    ("cV", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_CV as i32),
    ("dV", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_DV as i32),
    ("pRange", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_PRANGE as i32),
    ("type", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_TYPE as i32),
    ("level", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_LEVEL as i32),
    ("orientation", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_ORIENTATION as i32),
    ("dynamic", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_DYNAMIC as i32),
    ("incomingRoad", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_INCOMINGROAD as i32),
    ("connectingRoad", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_CONNECTINGROAD as i32),
    ("from", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_FROM as i32),
    ("to", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_TO as i32),
    ("fromLane", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_FROMLANE as i32),
    ("toLane", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_TOLANE as i32),
    ("max", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_MAX as i32),
    ("sOffset", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_SOFFSET as i32),
    ("rule", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_RULE as i32),
    ("restriction", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_RESTRICTION as i32),
    ("name", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_NAME as i32),
    ("signalId", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_SIGNALID as i32),
    ("file", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_FILE as i32),
    ("unit", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_UNIT as i32),
    ("", NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_NOTHING as i32),
];
// shortcut: NBEdge::UNSPECIFIED_* constants are not ported as Rust items; values copied from NBEdge.cpp
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;
const NBEDGE_UNSPECIFIED_OFFSET: f64 = 0.0;
const NBEDGE_UNSPECIFIED_SPEED: f64 = -1.0;
const NBEDGE_UNSPECIFIED_FRICTION: f64 = 1.0;
const NBEDGE_UNSPECIFIED_CONTPOS: f64 = -1.0;
const NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE: f64 = -1.0;
const NBEDGE_UNSPECIFIED_LOADED_LENGTH: f64 = -1.0;

/// C++ enum `NIImporter_OpenDrive::ContactPoint`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenDrive_ContactPoint {
    OPENDRIVE_CP_UNKNOWN = 0,
    OPENDRIVE_CP_START = 1,
    OPENDRIVE_CP_END = 2,
}

/// C++ enum `NIImporter_OpenDrive::ElementType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenDrive_ElementType {
    OPENDRIVE_ET_UNKNOWN = 0,
    OPENDRIVE_ET_ROAD = 1,
    OPENDRIVE_ET_JUNCTION = 2,
}

/// C++ enum `NIImporter_OpenDrive::GeometryType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenDrive_GeometryType {
    OPENDRIVE_GT_UNKNOWN = 0,
    OPENDRIVE_GT_LINE = 1,
    OPENDRIVE_GT_SPIRAL = 2,
    OPENDRIVE_GT_ARC = 3,
    OPENDRIVE_GT_POLY3 = 4,
    OPENDRIVE_GT_PARAMPOLY3 = 5,
}

/// C++ enum `NIImporter_OpenDrive::LinkType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenDrive_LinkType {
    OPENDRIVE_LT_SUCCESSOR = 0,
    OPENDRIVE_LT_PREDECESSOR = 1,
}

/// C++ enum `NIImporter_OpenDrive::OpenDriveXMLAttr`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenDrive_OpenDriveXMLAttr {
    OPENDRIVE_ATTR_NOTHING = 0,
    OPENDRIVE_ATTR_REVMAJOR = 1,
    OPENDRIVE_ATTR_REVMINOR = 2,
    OPENDRIVE_ATTR_ID = 3,
    OPENDRIVE_ATTR_LENGTH = 4,
    OPENDRIVE_ATTR_WIDTH = 5,
    OPENDRIVE_ATTR_RADIUS = 6,
    OPENDRIVE_ATTR_DISTANCE = 7,
    OPENDRIVE_ATTR_TSTART = 8,
    OPENDRIVE_ATTR_TEND = 9,
    OPENDRIVE_ATTR_WIDTHSTART = 10,
    OPENDRIVE_ATTR_WIDTHEND = 11,
    OPENDRIVE_ATTR_JUNCTION = 12,
    OPENDRIVE_ATTR_ELEMENTTYPE = 13,
    OPENDRIVE_ATTR_ELEMENTID = 14,
    OPENDRIVE_ATTR_CONTACTPOINT = 15,
    OPENDRIVE_ATTR_S = 16,
    OPENDRIVE_ATTR_T = 17,
    OPENDRIVE_ATTR_X = 18,
    OPENDRIVE_ATTR_Y = 19,
    OPENDRIVE_ATTR_Z = 20,
    OPENDRIVE_ATTR_HDG = 21,
    OPENDRIVE_ATTR_CURVSTART = 22,
    OPENDRIVE_ATTR_CURVEND = 23,
    OPENDRIVE_ATTR_CURVATURE = 24,
    OPENDRIVE_ATTR_A = 25,
    OPENDRIVE_ATTR_B = 26,
    OPENDRIVE_ATTR_C = 27,
    OPENDRIVE_ATTR_D = 28,
    OPENDRIVE_ATTR_AU = 29,
    OPENDRIVE_ATTR_BU = 30,
    OPENDRIVE_ATTR_CU = 31,
    OPENDRIVE_ATTR_DU = 32,
    OPENDRIVE_ATTR_AV = 33,
    OPENDRIVE_ATTR_BV = 34,
    OPENDRIVE_ATTR_CV = 35,
    OPENDRIVE_ATTR_DV = 36,
    OPENDRIVE_ATTR_PRANGE = 37,
    OPENDRIVE_ATTR_TYPE = 38,
    OPENDRIVE_ATTR_LEVEL = 39,
    OPENDRIVE_ATTR_ORIENTATION = 40,
    OPENDRIVE_ATTR_DYNAMIC = 41,
    OPENDRIVE_ATTR_INCOMINGROAD = 42,
    OPENDRIVE_ATTR_CONNECTINGROAD = 43,
    OPENDRIVE_ATTR_FROM = 44,
    OPENDRIVE_ATTR_TO = 45,
    OPENDRIVE_ATTR_FROMLANE = 46,
    OPENDRIVE_ATTR_TOLANE = 47,
    OPENDRIVE_ATTR_MAX = 48,
    OPENDRIVE_ATTR_SOFFSET = 49,
    OPENDRIVE_ATTR_RULE = 50,
    OPENDRIVE_ATTR_RESTRICTION = 51,
    OPENDRIVE_ATTR_NAME = 52,
    OPENDRIVE_ATTR_UNIT = 53,
    OPENDRIVE_ATTR_SIGNALID = 54,
    OPENDRIVE_ATTR_FILE = 55,
}

/// C++ enum `NIImporter_OpenDrive::OpenDriveXMLTag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenDrive_OpenDriveXMLTag {
    OPENDRIVE_TAG_NOTHING = 0,
    OPENDRIVE_TAG_HEADER = 1,
    OPENDRIVE_TAG_ROAD = 2,
    OPENDRIVE_TAG_PREDECESSOR = 3,
    OPENDRIVE_TAG_SUCCESSOR = 4,
    OPENDRIVE_TAG_GEOMETRY = 5,
    OPENDRIVE_TAG_LINE = 6,
    OPENDRIVE_TAG_SPIRAL = 7,
    OPENDRIVE_TAG_ARC = 8,
    OPENDRIVE_TAG_POLY3 = 9,
    OPENDRIVE_TAG_PARAMPOLY3 = 10,
    OPENDRIVE_TAG_LANESECTION = 11,
    OPENDRIVE_TAG_LANEOFFSET = 12,
    OPENDRIVE_TAG_ACCESS = 13,
    OPENDRIVE_TAG_LEFT = 14,
    OPENDRIVE_TAG_CENTER = 15,
    OPENDRIVE_TAG_RIGHT = 16,
    OPENDRIVE_TAG_LANE = 17,
    OPENDRIVE_TAG_SIGNAL = 18,
    OPENDRIVE_TAG_SIGNALREFERENCE = 19,
    OPENDRIVE_TAG_CONTROLLER = 20,
    OPENDRIVE_TAG_CONTROL = 21,
    OPENDRIVE_TAG_VALIDITY = 22,
    OPENDRIVE_TAG_SEMANTICS = 23,
    OPENDRIVE_TAG_PRIORITY = 24,
    OPENDRIVE_TAG_JUNCTION = 25,
    OPENDRIVE_TAG_CONNECTION = 26,
    OPENDRIVE_TAG_LANELINK = 27,
    OPENDRIVE_TAG_WIDTH = 28,
    OPENDRIVE_TAG_SPEED = 29,
    OPENDRIVE_TAG_ELEVATION = 30,
    OPENDRIVE_TAG_GEOREFERENCE = 31,
    OPENDRIVE_TAG_OFFSET = 32,
    OPENDRIVE_TAG_OBJECT = 33,
    OPENDRIVE_TAG_REPEAT = 34,
    OPENDRIVE_TAG_INCLUDE = 35,
}

/// C++ typedef `Poly3 NIImporter_OpenDrive::OpenDriveElevation`
pub type NIImporter_OpenDrive_OpenDriveElevation = NIImporter_OpenDrive_Poly3;

/// C++ typedef `Poly3 NIImporter_OpenDrive::OpenDriveLaneOffset`
pub type NIImporter_OpenDrive_OpenDriveLaneOffset = NIImporter_OpenDrive_Poly3;

/// C++ typedef `Poly3 NIImporter_OpenDrive::OpenDriveWidth`
pub type NIImporter_OpenDrive_OpenDriveWidth = NIImporter_OpenDrive_Poly3;

/// A connection between two roads
/// C++ class `NIImporter_OpenDrive::Connection` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_Connection {
    pub fromEdge: String,
    pub toEdge: String,
    pub fromLane: i32,
    pub toLane: i32,
    pub fromCP: NIImporter_OpenDrive_ContactPoint,
    pub toCP: NIImporter_OpenDrive_ContactPoint,
    pub all: bool,
    pub origID: String,
    pub origLane: i32,
    pub shape: PositionVector,
}

impl NIImporter_OpenDrive_Connection {
    /// C++: `std::string Connection::getDescription() const` [const]
    // std::string getDescription() const {
    //             return "Connection from=" + fromEdge + "_" + toString(fromLane)
    //                    + " to=" + toEdge + "_" + toString(toLane)
    //                    + " fromCP=" + (fromCP == OPENDRIVE_CP_START ? "start" : fromCP == OPENDRIVE_CP_END ? "end" : "unknown")
    //                    + " toCP=" + (toCP == OPENDRIVE_CP_START ? "start" : toCP == OPENDRIVE_CP_END ? "end" : "unknown")
    //                    + " all=" + toString(all);
    //             //+ " origID=" + origID + " origLane=" + toString(origLane);
    //         }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> String {
        let cp = |c: NIImporter_OpenDrive_ContactPoint| match c {
            NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START => "start",
            NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END => "end",
            _ => "unknown",
        };
        format!(
            "Connection from={}_{} to={}_{} fromCP={} toCP={} all={}",
            self.fromEdge, self.fromLane, self.toEdge, self.toLane,
            cp(self.fromCP), cp(self.toCP), self.all
        )
    }

}

/// Attribute set applied at a certain position along a lane
/// C++ class `NIImporter_OpenDrive::LaneAttributeChange` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_LaneAttributeChange {
    pub speed: f64,
    pub allowed: Vec<String>,
    pub denied: Vec<String>,
}

impl NIImporter_OpenDrive_LaneAttributeChange {
    /// C++: `void LaneAttributeChange::LaneAttributeChange(double)`
    /// Constructor
    // LaneAttributeChange(double speed) : speed(speed) {}
    #[allow(unused_variables)]
    pub fn new(speed: f64) -> Self {
        Self { speed, allowed: Vec::new(), denied: Vec::new() }
    }

}

/// C++ class `NIImporter_OpenDrive::LaneSorter` (netimport/NIImporter_OpenDrive.h)
pub struct NIImporter_OpenDrive_LaneSorter {
}

impl NIImporter_OpenDrive_LaneSorter {
    /// C++: `void LaneSorter::LaneSorter()`
    /// constructor
    // explicit LaneSorter() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {}
    }

    /// C++: `int LaneSorter::operator()(const OpenDriveLane &, const OpenDriveLane &) const` [const]
    /// comparing operation
    // int operator()(const OpenDriveLane& a, const OpenDriveLane& b) const {
    //             // sort from the reference line outwards (desceding order of sumo lane ids)
    //             return abs(a.id) < abs(b.id);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, a: &NIImporter_OpenDrive_OpenDriveLane, b: &NIImporter_OpenDrive_OpenDriveLane) -> i32 {
        // sort from the reference line outwards (desceding order of sumo lane ids)
        (a.id.abs() < b.id.abs()) as i32
    }

}

/// Ordering induced by C++ `LaneSorter::operator()` (used with `std::sort`)
fn lane_sorter_ordering(sorter: &NIImporter_OpenDrive_LaneSorter, a: &NIImporter_OpenDrive_OpenDriveLane, b: &NIImporter_OpenDrive_OpenDriveLane) -> std::cmp::Ordering {
    if sorter.call_op(a, b) != 0 {
        std::cmp::Ordering::Less
    } else if sorter.call_op(b, a) != 0 {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

/// Maps a raw SAX tag number back to its `OpenDriveXMLTag` (unknown numbers map to OPENDRIVE_TAG_NOTHING)
fn open_drive_tag_from_i32(v: i32) -> NIImporter_OpenDrive_OpenDriveXMLTag {
    use NIImporter_OpenDrive_OpenDriveXMLTag as T;
    const ALL: [T; 36] = [
        T::OPENDRIVE_TAG_NOTHING, T::OPENDRIVE_TAG_HEADER, T::OPENDRIVE_TAG_ROAD, T::OPENDRIVE_TAG_PREDECESSOR,
        T::OPENDRIVE_TAG_SUCCESSOR, T::OPENDRIVE_TAG_GEOMETRY, T::OPENDRIVE_TAG_LINE, T::OPENDRIVE_TAG_SPIRAL,
        T::OPENDRIVE_TAG_ARC, T::OPENDRIVE_TAG_POLY3, T::OPENDRIVE_TAG_PARAMPOLY3, T::OPENDRIVE_TAG_LANESECTION,
        T::OPENDRIVE_TAG_LANEOFFSET, T::OPENDRIVE_TAG_ACCESS, T::OPENDRIVE_TAG_LEFT, T::OPENDRIVE_TAG_CENTER,
        T::OPENDRIVE_TAG_RIGHT, T::OPENDRIVE_TAG_LANE, T::OPENDRIVE_TAG_SIGNAL, T::OPENDRIVE_TAG_SIGNALREFERENCE,
        T::OPENDRIVE_TAG_CONTROLLER, T::OPENDRIVE_TAG_CONTROL, T::OPENDRIVE_TAG_VALIDITY, T::OPENDRIVE_TAG_SEMANTICS,
        T::OPENDRIVE_TAG_PRIORITY, T::OPENDRIVE_TAG_JUNCTION, T::OPENDRIVE_TAG_CONNECTION, T::OPENDRIVE_TAG_LANELINK,
        T::OPENDRIVE_TAG_WIDTH, T::OPENDRIVE_TAG_SPEED, T::OPENDRIVE_TAG_ELEVATION, T::OPENDRIVE_TAG_GEOREFERENCE,
        T::OPENDRIVE_TAG_OFFSET, T::OPENDRIVE_TAG_OBJECT, T::OPENDRIVE_TAG_REPEAT, T::OPENDRIVE_TAG_INCLUDE,
    ];
    ALL.into_iter().find(|t| *t as i32 == v).unwrap_or(T::OPENDRIVE_TAG_NOTHING)
}

/// C++ `myElementStack.back()` (n = 0) and `myElementStack[size() - 2]` (n = 1), or None when absent
fn stack_back(stack: &[i32], n: usize) -> Option<i32> {
    stack.len().checked_sub(n + 1).map(|i| stack[i])
}

/// Importer for networks stored in openDrive format
/// C++ class `NIImporter_OpenDrive` (netimport/NIImporter_OpenDrive.h)
#[repr(C)]
pub struct NIImporter_OpenDrive {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    pub myTypeContainer: Ptr<NBTypeCont>,
    pub myCurrentEdge: NIImporter_OpenDrive_OpenDriveEdge,
    pub myCurrentController: NIImporter_OpenDrive_OpenDriveController,
    pub myEdges: Ptr<BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>>>,
    pub myElementStack: Vec<i32>,
    pub myCurrentLaneDirection: NIImporter_OpenDrive_OpenDriveXMLTag,
    pub myCurrentJunctionID: String,
    pub myCurrentIncomingRoad: String,
    pub myCurrentConnectingRoad: String,
    pub myCurrentContactPoint: NIImporter_OpenDrive_ContactPoint,
    pub myConnectionWasEmpty: bool,
    pub mySignals: BTreeMap<String, NIImporter_OpenDrive_OpenDriveSignal>,
    pub myControllers: BTreeMap<String, NIImporter_OpenDrive_OpenDriveController>,
    pub myJunctions2Controllers: BTreeMap<String, Vec<String>>,
    pub myOffset: Position,
    pub myUseCurrentNode: bool,
}

impl std::ops::Deref for NIImporter_OpenDrive {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_OpenDrive {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

// C++ static members of NIImporter_OpenDrive (port as module-level statics, see AGENTS.md):
//   static bool NIImporter_OpenDrive::myImportAllTypes;
//   static bool NIImporter_OpenDrive::myImportWidths;
//   static double NIImporter_OpenDrive::myMinWidth;
//   static bool NIImporter_OpenDrive::myImportInternalShapes;
//   static bool NIImporter_OpenDrive::myIgnoreMisplacedSignals;
//   static OpenDriveController NIImporter_OpenDrive::myDummyController;
//   static SequentialStringBijection::Entry[] NIImporter_OpenDrive::openDriveTags;
//   static SequentialStringBijection::Entry[] NIImporter_OpenDrive::openDriveAttrs;

impl NIImporter_OpenDrive {
    /// C++: `void NIImporter_OpenDrive::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads content of the optionally given SUMO file
    // void
    // NIImporter_OpenDrive::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether the option is set properly and all files exist
    //     if (!oc.isUsableFileList("opendrive-files")) {
    //         return;
    //     }
    //     // prepare types
    //     myImportAllTypes = oc.getBool("opendrive.import-all-lanes");
    //     myImportWidths = !oc.getBool("opendrive.ignore-widths");
    //     myMinWidth = oc.getFloat("opendrive.min-width");
    //     myImportInternalShapes = oc.getBool("opendrive.internal-shapes");
    //     myIgnoreMisplacedSignals = oc.getBool("opendrive.ignore-misplaced-signals");
    //     const bool customLaneShapes = oc.getBool("opendrive.lane-shapes");
    //     NBTypeCont& tc = nb.getTypeCont();
    //     NBNodeCont& nc = nb.getNodeCont();
    //     // build the handler
    //     std::map<std::string, OpenDriveEdge*> edges;
    //     NIImporter_OpenDrive handler(nb.getTypeCont(), edges);
    //     handler.needsCharacterData();
    //     // parse file(s)
    //     for (const std::string& file : oc.getStringVector("opendrive-files")) {
    //         handler.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing opendrive from '" + file + "'");
    //         XMLSubSys::runParser(handler, file, false, false, true);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //     // apply signal reference information
    //     for (auto& item : edges) {
    //         for (OpenDriveSignal& signal : item.second->signals) {
    //             if (signal.type == "") {
    //                 if (handler.getSignals().count(signal.id) == 0) {
    //                     WRITE_WARNINGF(TL("Could not find signal reference '%'."), signal.id);
    //                 } else {
    //                     const OpenDriveSignal& ref = handler.getSignals()[signal.id];
    //                     signal.type = ref.type;
    //                     signal.name = ref.name;
    //                     signal.dynamic = ref.dynamic;
    //                     signal.controller = ref.controller;
    //                 }
    //             }
    //         }
    //     }
    //
    //     // split inner/outer edges
    //     std::map<std::string, OpenDriveEdge*> innerEdges, outerEdges;
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = edges.begin(); i != edges.end(); ++i) {
    //         if ((*i).second->isInner) {
    //             innerEdges[(*i).first] = (*i).second;
    //         } else {
    //             outerEdges[(*i).first] = (*i).second;
    //         }
    //     }
    //
    //     // convert geometries into a discretised representation
    //     computeShapes(edges);
    //     // check whether lane sections are valid and whether further must be introduced
    //     revisitLaneSections(tc, edges);
    //
    //     // -------------------------
    //     // node building
    //     // -------------------------
    //     // build nodes#1
    //     //  look at all links which belong to a node, collect their bounding boxes
    //     //  and place the node in the middle of this bounding box
    //     std::map<std::string, Boundary> posMap;
    //     std::map<std::string, std::string> edge2junction;
    //     std::vector<NodeSet> joinedNodeIDs;
    //     //   compute node positions
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = innerEdges.begin(); i != innerEdges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         assert(e->junction != "-1" && e->junction != "");
    //         edge2junction[e->id] = e->junction;
    //         if (posMap.find(e->junction) == posMap.end()) {
    //             posMap[e->junction] = Boundary();
    //         }
    //         // Check if geometry is valid before adding to boundary
    //         if (e->geom.size() > 0) {
    //             Boundary geomBoundary = e->geom.getBoxBoundary();
    //             // Only add if the boundary doesn't contain NaN values
    //             if (geomBoundary.isInitialised() &&
    //                     !std::isnan(geomBoundary.xmin()) && !std::isnan(geomBoundary.xmax()) &&
    //                     !std::isnan(geomBoundary.ymin()) && !std::isnan(geomBoundary.ymax())) {
    //                 posMap[e->junction].add(geomBoundary);
    //             } else {
    //                 WRITE_WARNINGF(TL("Ignoring invalid geometry for inner edge '%' (xodr road '%') in junction '%'."),
    //                                e->id, e->id, e->junction);
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Inner edge '%' (xodr road '%') in junction '%' has no geometry."),
    //                            e->id, e->id, e->junction);
    //         }
    //     }
    //     //   build nodes
    //     for (std::map<std::string, Boundary>::iterator i = posMap.begin(); i != posMap.end(); ++i) {
    //         //std::cout << " import node=" << (*i).first << " z=" << (*i).second.getCenter() << " boundary=" << (*i).second << "\n";
    //         // Check if the boundary was properly initialized
    //         if (!(*i).second.isInitialised()) {
    //             WRITE_ERRORF(TL("Junction '%' has no valid inner edges to determine its position."), (*i).first);
    //             throw ProcessError(TLF("Could not determine position for junction '%'.", (*i).first));
    //         }
    //         Position center = (*i).second.getCenter();
    //         // Double-check that the center position is valid
    //         if (center.isNAN()) {
    //             WRITE_ERRORF(TL("Junction '%' resulted in invalid (NaN) position."), (*i).first);
    //             throw ProcessError(TLF("Could not compute valid position for junction '%'.", (*i).first));
    //         }
    //         if (!nb.getNodeCont().insert((*i).first, center)) {
    //             throw ProcessError(TLF("Could not add node '%'.", (*i).first));
    //         }
    //     }
    //     //  assign built nodes
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = outerEdges.begin(); i != outerEdges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         for (std::vector<OpenDriveLink>::iterator j = e->links.begin(); j != e->links.end(); ++j) {
    //             OpenDriveLink& l = *j;
    //             const std::string& nid = l.elementID;
    //             if (l.elementType != OPENDRIVE_ET_ROAD) {
    //                 if (nb.getNodeCont().retrieve(nid) == nullptr) {
    //                     // not yet seen, build (possibly a junction without connections)
    //                     Position pos = l.linkType == OPENDRIVE_LT_SUCCESSOR ? e->geom[-1] : e->geom[0];
    //                     if (!nb.getNodeCont().insert(nid, pos)) {
    //                         throw ProcessError(TLF("Could not build node '%'.", nid));
    //                     }
    //                 }
    //                 // set node information
    //                 setNodeSecure(nb.getNodeCont(), *e, l.elementID, l.linkType, joinedNodeIDs);
    //                 continue;
    //             }
    //             if (edge2junction.find(l.elementID) != edge2junction.end()) {
    //                 // set node information of an internal road
    //                 setNodeSecure(nb.getNodeCont(), *e, edge2junction[l.elementID], l.linkType, joinedNodeIDs);
    //                 continue;
    //             }
    //         }
    //     }
    //     //  we should now have all nodes set for links which are not outer edge-to-outer edge links
    //
    //
    //     // build nodes#2
    //     //  build nodes for all outer edge-to-outer edge connections
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = outerEdges.begin(); i != outerEdges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         for (std::vector<OpenDriveLink>::iterator j = e->links.begin(); j != e->links.end(); ++j) {
    //             OpenDriveLink& l = *j;
    //             if (l.elementType != OPENDRIVE_ET_ROAD || edge2junction.find(l.elementID) != edge2junction.end()) {
    //                 // is a connection to an internal edge, or a node, skip
    //                 continue;
    //             }
    //             // we have a direct connection between to external edges
    //             std::string id1 = e->id;
    //             std::string id2 = l.elementID;
    //             if (id1 < id2) {
    //                 std::swap(id1, id2);
    //             }
    //             std::string nid = id1 + "." + id2;
    //             if (nb.getNodeCont().retrieve(nid) == nullptr) {
    //                 // not yet seen, build
    //                 Position pos = l.linkType == OPENDRIVE_LT_SUCCESSOR ? e->geom[-1] : e->geom[0];
    //                 if (!nb.getNodeCont().insert(nid, pos)) {
    //                     throw ProcessError(TLF("Could not build node '%'.", nid));
    //                 }
    //             }
    //             /* debug-stuff
    //             else {
    //                 Position pos = l.linkType==OPENDRIVE_LT_SUCCESSOR ? e.geom[e.geom.size()-1] : e.geom[0];
    //                 cout << nid << " " << pos << " " << nb.getNodeCont().retrieve(nid)->getPosition() << endl;
    //             }
    //             */
    //             setNodeSecure(nb.getNodeCont(), *e, nid, l.linkType, joinedNodeIDs);
    //         }
    //     }
    //     // we should now have start/end nodes for all outer edge-to-outer edge connections
    //
    //
    //     // build nodes#3
    //     //  assign further nodes generated from inner-edges
    //     //  these nodes have not been assigned earlier, because the connections are referenced in inner-edges
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = outerEdges.begin(); i != outerEdges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         if (e->to != nullptr && e->from != nullptr) {
    //             continue;
    //         }
    //         for (std::map<std::string, OpenDriveEdge*>::iterator j = innerEdges.begin(); j != innerEdges.end(); ++j) {
    //             OpenDriveEdge* ie = (*j).second;
    //             for (std::vector<OpenDriveLink>::iterator k = ie->links.begin(); k != ie->links.end(); ++k) {
    //                 OpenDriveLink& il = *k;
    //                 if (il.elementType != OPENDRIVE_ET_ROAD || il.elementID != e->id) {
    //                     // not conneted to the currently investigated outer edge
    //                     continue;
    //                 }
    //                 std::string nid = edge2junction[ie->id];
    //                 if (il.contactPoint == OPENDRIVE_CP_START) {
    //                     setNodeSecure(nb.getNodeCont(), *e, nid, OPENDRIVE_LT_PREDECESSOR, joinedNodeIDs);
    //                 } else {
    //                     setNodeSecure(nb.getNodeCont(), *e, nid, OPENDRIVE_LT_SUCCESSOR, joinedNodeIDs);
    //                 }
    //             }
    //         }
    //
    //     }
    //
    //     // build start/end nodes which were not defined previously
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = outerEdges.begin(); i != outerEdges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         if ((e->from == nullptr || e->to == nullptr) && e->geom.size() == 0) {
    //             continue;
    //         }
    //         if (e->from == nullptr) {
    //             const std::string nid = e->id + ".begin";
    //             e->from = getOrBuildNode(nid, e->geom.front(), nb.getNodeCont());
    //         }
    //         if (e->to == nullptr) {
    //             const std::string nid = e->id + ".end";
    //             e->to = getOrBuildNode(nid, e->geom.back(), nb.getNodeCont());
    //         }
    //     }
    //
    //     std::map<NBNode*, NBNode*> joinedNodes;
    //     for (NodeSet& joined : joinedNodeIDs) {
    //         Position joinedPos(0, 0);
    //         for (NBNode* j : joined) {
    //             joinedPos.add(j->getPosition());
    //         }
    //         joinedPos.mul(1. / (double)joined.size());
    //         const std::string joinedID = nc.createClusterId(joined);
    //         if (!nc.insert(joinedID, joinedPos)) {
    //             throw ProcessError(TLF("Could not add node '%'.", joinedID));
    //         }
    //         NBNode* n = nc.retrieve(joinedID);
    //         for (NBNode* j : joined) {
    //             joinedNodes[j] = n;
    //         }
    //     }
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = outerEdges.begin(); i != outerEdges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         if (joinedNodes.count(e->from) != 0) {
    //             nc.extract(e->from, true);
    //             e->from = joinedNodes[e->from];
    //         }
    //         if (joinedNodes.count(e->to) != 0) {
    //             nc.extract(e->to, true);
    //             e->to = joinedNodes[e->to];
    //         }
    //     }
    //
    //
    //     // -------------------------
    //     // edge building
    //     // -------------------------
    //     const double defaultSpeed = tc.getEdgeTypeSpeed("");
    //     const bool saveOrigIDs = OptionsCont::getOptions().getBool("output.original-names");
    //     const bool positionIDs = OptionsCont::getOptions().getBool("opendrive.position-ids");
    //     // lane-id-map sumoEdge,sumoLaneIndex->odrLaneIndex
    //     std::map<std::pair<NBEdge*, int>, int> laneIndexMap;
    //     // build edges
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = outerEdges.begin(); i != outerEdges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         if (e->geom.size() < 2) {
    //             WRITE_WARNINGF(TL("Ignoring road '%' without geometry."), e->id);
    //             continue;
    //         }
    //         bool lanesBuilt = false;
    //
    //         // go along the lane sections, build a node in between of each pair
    //
    //         /// @todo: One could think of determining whether lane sections may be joined when being equal in SUMO's sense
    //         /// Their naming would have to be updated, too, also in TraCI
    //
    //         /// @todo: probably, the lane offsets to the center are not right
    //         NBNode* sFrom = e->from;
    //         NBNode* sTo = e->to;
    //         int priorityR = e->getPriority(OPENDRIVE_TAG_RIGHT);
    //         int priorityL = e->getPriority(OPENDRIVE_TAG_LEFT);
    //         double sB = 0;
    //         double sE = e->length;
    //         // 0-length geometries are possible if only the inner points are represented
    //         PositionVector geomWithOffset = e->geom;
    //         if (e->laneOffsets.size() > 0) {
    //             try {
    //                 geomWithOffset.move2sideCustom(e->laneOffsets);
    //                 //std::cout << " e=" << e->id << " offsets=" << e->laneOffsets << " geom=" << e->geom << " geom2=" << geomWithOffset << "\n";
    //             } catch (InvalidArgument&) {
    //                 WRITE_WARNINGF(TL("Could not apply laneOffsets for edge '%'"), e->id);
    //             }
    //         }
    // #ifdef DEBUG_SHAPE
    //         if (DEBUG_COND3(e->id)) {
    //             std::cout << " geomWithOffset=" << geomWithOffset << "\n";
    //         }
    // #endif
    //         const double length2D = geomWithOffset.length2D();
    //         double cF = length2D == 0 ? 1 : e->length / length2D;
    //         NBEdge* prevRight = nullptr;
    //         NBEdge* prevLeft = nullptr;
    //
    //         // starting at the same node as ending, and no lane sections?
    //         if (sFrom == sTo && e->laneSections.size() == 1) {
    //             // --> loop, split!
    //             OpenDriveLaneSection ls = e->laneSections[0];
    //             ls.s = e->length / 2.;
    //             e->laneSections.push_back(ls);
    //             WRITE_WARNING("Edge '" + e->id + "' has to be split as it connects same junctions.")
    //         }
    //         sanitizeWidths(e);
    //         if (myMinWidth > 0) {
    //             const double minDist = oc.getFloat("opendrive.curve-resolution");
    //             splitMinWidths(e, tc, minDist);
    //         }
    //
    //         // build along lane sections
    //         int sectionIndex = 0;
    //         for (std::vector<OpenDriveLaneSection>::iterator j = e->laneSections.begin(); j != e->laneSections.end(); ++j) {
    //             // add internal node if needed
    //             if (j == e->laneSections.end() - 1) {
    //                 sTo = e->to;
    //                 sE = e->length / cF;
    //             } else {
    //                 double nextS = (j + 1)->s;
    //                 const std::string nodeID = e->id + (positionIDs ? "." + toString(nextS) : "#" + toString(sectionIndex + 1));
    //                 sTo = new NBNode(nodeID, geomWithOffset.positionAtOffset(nextS));
    //                 if (!nb.getNodeCont().insert(sTo)) {
    //                     throw ProcessError(TLF("Could not add node '%'.", sTo->getID()));
    //                 }
    //                 sE = nextS / cF;
    //             }
    //             const PositionVector geom = geomWithOffset.getSubpart2D(sB, sE).simplified2(false);
    //             std::string id = e->id;
    //             if (positionIDs) {
    //                 if (sFrom != e->from || sTo != e->to) {
    //                     id = id + "." + toString((*j).s);
    //                 } else if (e->laneSections.size() == 1) {
    //                     id = id + ".0.00";
    //                 }
    //             } else if (e->laneSections.size() > 1) {
    //                 id = id + "#" + toString(sectionIndex++);
    //             }
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //             if (DEBUG_COND(e)) {
    //                 std::cout << " id=" << id << " sB=" << sB << " sE=" << sE << " geom=" << geom << "\n";
    //             }
    // #endif
    //
    //             // build lanes to right
    //             NBEdge* currRight = nullptr;
    //             if ((*j).rightLaneNumber > 0) {
    //                 std::vector<double> offsets(geom.size(), 0);
    //                 bool useOffsets = false;
    //                 PositionVector rightGeom = geom;
    // #ifdef DEBUG_SHAPE
    //                 if (DEBUG_COND3(e->id)) {
    //                     gDebugFlag1 = true;
    //                 }
    // #endif
    //                 rightGeom.move2side((*j).discardedInnerWidthRight);
    // #ifdef DEBUG_SHAPE
    //                 if (DEBUG_COND3(e->id)) {
    //                     std::cout << " -" << id << "_geom=" << geom << " -" << id << "_rightGeom=" << rightGeom << "\n";
    //                     gDebugFlag1 = false;
    //                 }
    // #endif
    //                 PositionVector laneGeom = rightGeom;
    //                 currRight = new NBEdge("-" + id, sFrom, sTo, (*j).rightType, defaultSpeed, NBEdge::UNSPECIFIED_FRICTION, (*j).rightLaneNumber, priorityR,
    //                                        NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, rightGeom, LaneSpreadFunction::RIGHT, e->streetName, "", true);
    //                 lanesBuilt = true;
    //                 std::vector<OpenDriveLane>& lanes = (*j).lanesByDir[OPENDRIVE_TAG_RIGHT];
    //                 std::sort(lanes.begin(), lanes.end(), LaneSorter());
    //                 for (const OpenDriveLane& odl : lanes) {
    //                     std::map<int, int>::const_iterator lp = (*j).laneMap.find(odl.id);
    //                     if (lp != (*j).laneMap.end()) {
    //                         int sumoLaneIndex = lp->second;
    //                         setLaneAttributes(e, currRight->getLaneStruct(sumoLaneIndex), odl, saveOrigIDs, tc);
    //                         laneIndexMap[std::make_pair(currRight, sumoLaneIndex)] = odl.id;
    //                         if (useOffsets) {
    //                             PositionVector laneShape = laneGeom;
    //                             laneShape.move2sideCustom(offsets);
    //                             currRight->getLaneStruct(sumoLaneIndex).customShape = laneShape;
    //                         }
    //                     } else if (customLaneShapes) {
    //                         useOffsets = true;
    //                     }
    //                     if (customLaneShapes) {
    //                         addOffsets(false, laneGeom, odl.widthData, e->id + "_" + toString(odl.id), offsets);
    //                     }
    //                 }
    //                 if (!nb.getEdgeCont().insert(currRight, myImportAllTypes)) {
    //                     throw ProcessError(TLF("Could not add edge '%'.", currRight->getID()));
    //                 }
    //                 if (nb.getEdgeCont().wasIgnored("-" + id)) {
    //                     prevRight = nullptr;
    //                 } else {
    //                     // connect lane sections
    //                     if (prevRight != nullptr) {
    //                         std::map<int, int> connections = (*j).getInnerConnections(OPENDRIVE_TAG_RIGHT, *(j - 1));
    //                         for (std::map<int, int>::const_iterator k = connections.begin(); k != connections.end(); ++k) {
    // #ifdef DEBUG_CONNECTIONS
    //                             if (DEBUG_COND(e)) {
    //                                 std::cout << "addCon1 from=" << prevRight->getID() << "_" << (*k).first << " to=" << currRight->getID() << "_" << (*k).second << "\n";
    //                             }
    // #endif
    //                             prevRight->addLane2LaneConnection((*k).first, currRight, (*k).second, NBEdge::Lane2LaneInfoType::VALIDATED);
    //                         }
    //                     }
    //                     prevRight = currRight;
    //                 }
    //             }
    //
    //             // build lanes to left
    //             NBEdge* currLeft = nullptr;
    //             if ((*j).leftLaneNumber > 0) {
    //                 std::vector<double> offsets(geom.size(), 0);
    //                 bool useOffsets = false;
    //                 PositionVector leftGeom = geom;
    //                 leftGeom.move2side(-(*j).discardedInnerWidthLeft);
    //                 PositionVector laneGeom = leftGeom;
    // #ifdef DEBUG_SHAPE
    //                 if (DEBUG_COND3(e->id)) {
    //                     std::cout << " " << id << "_geom=" << geom << " " << id << "_leftGeom=" << leftGeom << "\n";
    //                 }
    // #endif
    //                 currLeft = new NBEdge(id, sTo, sFrom, (*j).leftType, defaultSpeed, NBEdge::UNSPECIFIED_FRICTION, (*j).leftLaneNumber, priorityL,
    //                                       NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, leftGeom.reverse(), LaneSpreadFunction::RIGHT, e->streetName, "", true);
    //                 lanesBuilt = true;
    //                 std::vector<OpenDriveLane>& lanes = (*j).lanesByDir[OPENDRIVE_TAG_LEFT];
    //                 std::sort(lanes.begin(), lanes.end(), LaneSorter());
    //                 for (std::vector<OpenDriveLane>::const_iterator k = lanes.begin(); k != lanes.end(); ++k) {
    //                     std::map<int, int>::const_iterator lp = (*j).laneMap.find((*k).id);
    //                     if (lp != (*j).laneMap.end()) {
    //                         int sumoLaneIndex = lp->second;
    //                         setLaneAttributes(e, currLeft->getLaneStruct(sumoLaneIndex), *k, saveOrigIDs, tc);
    //                         laneIndexMap[std::make_pair(currLeft, sumoLaneIndex)] = (*k).id;
    //                         if (useOffsets) {
    //                             PositionVector laneShape = laneGeom;
    //                             laneShape.move2sideCustom(offsets);
    //                             currLeft->getLaneStruct(sumoLaneIndex).customShape = laneShape.reverse();
    //                         }
    //                     } else if (customLaneShapes) {
    //                         useOffsets = true;
    //                     }
    //                     if (customLaneShapes) {
    //                         addOffsets(true, laneGeom, (*k).widthData, e->id + "_" + toString((*k).id), offsets);
    //                     }
    //                 }
    //                 if (!nb.getEdgeCont().insert(currLeft, myImportAllTypes)) {
    //                     throw ProcessError(TLF("Could not add edge '%'.", currLeft->getID()));
    //                 }
    //                 if (nb.getEdgeCont().wasIgnored(id)) {
    //                     prevLeft = nullptr;
    //                 } else {
    //                     // connect lane sections
    //                     if (prevLeft != nullptr) {
    //                         std::map<int, int> connections = (*j).getInnerConnections(OPENDRIVE_TAG_LEFT, *(j - 1));
    //                         for (std::map<int, int>::const_iterator k = connections.begin(); k != connections.end(); ++k) {
    // #ifdef DEBUG_CONNECTIONS
    //                             if (DEBUG_COND(e)) {
    //                                 std::cout << "addCon2 from=" << currLeft->getID() << "_" << (*k).first << " to=" << prevLeft->getID() << "_" << (*k).second << "\n";
    //                             }
    // #endif
    //                             currLeft->addLane2LaneConnection((*k).first, prevLeft, (*k).second, NBEdge::Lane2LaneInfoType::VALIDATED);
    //                         }
    //                     }
    //                     prevLeft = currLeft;
    //                 }
    //             }
    //             (*j).sumoID = id;
    //
    //
    //             sB = sE;
    //             sFrom = sTo;
    //         }
    //         if (oc.isSet("polygon-output")) {
    //             writeRoadObjects(e);
    //         }
    //         if (!lanesBuilt) {
    //             WRITE_WARNINGF(TL("Edge '%' has no lanes."), e->id);
    //         }
    //     }
    //     if (oc.isSet("polygon-output")) {
    //         for (auto item : innerEdges) {
    //             writeRoadObjects(item.second);
    //         }
    //     }
    //
    //     // -------------------------
    //     // connections building
    //     // -------------------------
    //     // generate explicit lane-to-lane connections
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = edges.begin(); i != edges.end(); ++i) {
    //         setEdgeLinks2(*(*i).second, edges);
    //     }
    //     // compute connections across intersections, if any
    //     std::vector<Connection> connections2;
    //     for (std::map<std::string, OpenDriveEdge*>::iterator j = edges.begin(); j != edges.end(); ++j) {
    //         const std::set<Connection>& conns = (*j).second->connections;
    //
    //         for (std::set<Connection>::const_iterator i = conns.begin(); i != conns.end(); ++i) {
    //             if (innerEdges.find((*i).fromEdge) != innerEdges.end()) {
    //                 // connections starting at inner edges are processed by starting from outer edges
    //                 continue;
    //             }
    //             if (innerEdges.find((*i).toEdge) != innerEdges.end()) {
    //                 std::set<Connection> seen;
    //                 buildConnectionsToOuter(*i, innerEdges, edges, tc, connections2, seen);
    //             } else {
    //                 connections2.push_back(*i);
    //             }
    //         }
    //     }
    //     // set connections
    //     for (std::vector<Connection>::const_iterator i = connections2.begin(); i != connections2.end(); ++i) {
    // #ifdef DEBUG_CONNECTIONS
    //         std::cout << "connections2 " << (*i).getDescription() << "\n";
    // #endif
    //         std::string fromEdge = (*i).fromEdge;
    //         if (edges.find(fromEdge) == edges.end()) {
    //             WRITE_WARNINGF(TL("While setting connections: from-edge '%' is not known."), fromEdge);
    //             continue;
    //         }
    //         OpenDriveEdge* odFrom = edges[fromEdge];
    //         int fromLane = (*i).fromLane;
    //         bool fromLast = ((*i).fromCP == OPENDRIVE_CP_END) && ((*i).fromLane < 0);
    //         fromEdge = fromLast ? odFrom->laneSections.back().sumoID : odFrom->laneSections[0].sumoID;
    //
    //         std::string toEdge = (*i).toEdge;
    //         if (edges.find(toEdge) == edges.end()) {
    //             WRITE_WARNINGF(TL("While setting connections: to-edge '%' is not known."), toEdge);
    //             continue;
    //         }
    //
    //         OpenDriveEdge* odTo = edges[toEdge];
    //         int toLane = (*i).toLane;
    //         bool toLast = ((*i).toCP == OPENDRIVE_CP_END) || ((*i).toLane > 0);
    //         toEdge = toLast ? odTo->laneSections.back().sumoID : odTo->laneSections[0].sumoID;
    //
    //         if (fromLane == UNSET_CONNECTION) {
    //             continue;
    //         }
    //         if (fromLane < 0) {
    //             fromEdge = reversedEdgeID(fromEdge);
    //         }
    //         if (toLane == UNSET_CONNECTION) {
    //             continue;
    //         }
    //         if (toLane < 0) {
    //             toEdge = reversedEdgeID(toEdge);
    //         }
    //         fromLane = fromLast ? odFrom->laneSections.back().laneMap[fromLane] : odFrom->laneSections[0].laneMap[fromLane];
    //         toLane = toLast ?  odTo->laneSections.back().laneMap[toLane] : odTo->laneSections[0].laneMap[toLane];
    //         NBEdge* from = nb.getEdgeCont().retrieve(fromEdge);
    //         NBEdge* to = nb.getEdgeCont().retrieve(toEdge);
    //         if (from == nullptr) {
    //             WRITE_WARNINGF(TL("Could not find fromEdge representation of '%' in connection '%'."), fromEdge, (*i).origID);
    //         }
    //         if (to == nullptr) {
    //             WRITE_WARNINGF(TL("Could not find fromEdge representation of '%' in connection '%'."), toEdge, (*i).origID);
    //         }
    //         if (from == nullptr || to == nullptr) {
    //             continue;
    //         }
    //
    // #ifdef DEBUG_CONNECTIONS
    //         if (DEBUG_COND2(from->getID())) {
    //             std::cout << "addCon3 from=" << from->getID() << "_" << fromLane << " to=" << to->getID() << "_" << toLane << "\n";
    //         }
    // #endif
    //         from->addLane2LaneConnection(fromLane, to, toLane, NBEdge::Lane2LaneInfoType::USER, false, false,
    //                                      KEEPCLEAR_UNSPECIFIED,
    //                                      NBEdge::UNSPECIFIED_CONTPOS,
    //                                      NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE,
    //                                      NBEdge::UNSPECIFIED_SPEED,
    //                                      NBEdge::UNSPECIFIED_FRICTION,
    //                                      NBEdge::UNSPECIFIED_LOADED_LENGTH,
    //                                      (*i).shape);
    //
    //         if ((*i).origID != "" && saveOrigIDs) {
    //             // @todo: this is the most silly way to determine the connection
    //             std::vector<NBEdge::Connection>& cons = from->getConnections();
    //             for (std::vector<NBEdge::Connection>::iterator k = cons.begin(); k != cons.end(); ++k) {
    //                 if ((*k).fromLane == fromLane && (*k).toEdge == to && (*k).toLane == toLane) {
    //                     (*k).setParameter(SUMO_PARAM_ORIGID, (*i).origID + "_" + toString((*i).origLane));
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //
    //
    //     // -------------------------
    //     // traffic lights
    //     // -------------------------
    //     std::map<std::string, std::string> signal2junction;
    //     std::map<std::string, OpenDriveController>& controllers = handler.getControllers();
    //
    //     for (const auto& it : edges) {
    //         const OpenDriveEdge* e = it.second;
    //         for (const OpenDriveSignal& signal : e->signals) { // signal for which junction?
    //             if (signal.controller.size() == 0) {
    //                 continue;
    //             }
    //             std::string junctionID;
    //             for (auto connection : e->connections) {
    //                 if ((connection.fromLane < 0 && signal.orientation < 0) || (connection.fromLane > 0 && signal.orientation > 0)) {
    //                     continue;
    //                 }
    //                 if ((signal.minLane == 0 && signal.maxLane == 0) || (signal.maxLane >= connection.fromLane && signal.minLane <= connection.fromLane)) {
    //                     const OpenDriveEdge* connectedEdge = edges[connection.toEdge];
    //                     if (controllers[signal.controller].junction.size() > 0 && controllers[signal.controller].junction != connectedEdge->junction) {
    //                         WRITE_WARNINGF(TL("Controlling multiple junctions by the same controller '%' is currently not implemented."), signal.controller);
    //                     }
    //                     controllers[signal.controller].junction = connectedEdge->junction;
    //                 }
    //             }
    //         }
    //     }
    //
    //     const bool importSignalGroups = oc.getBool("opendrive.signal-groups");
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = edges.begin(); i != edges.end(); ++i) {
    //         OpenDriveEdge* e = (*i).second;
    //         for (const OpenDriveSignal& signal : e->signals) {
    //             int intType = -1;
    //             try {
    //                 intType = StringUtils::toInt(signal.type);
    //             } catch (NumberFormatException&) {}
    //             if (intType < 1000001 || (intType > 1000013 && intType != 1000020) || intType == 1000008) {
    //                 // not a traffic_light (Section 6.11)
    //                 continue;
    //             }
    //             if (e->laneSections.size() == 0) {
    //                 WRITE_WARNINGF(TL("Edge '%' has signals but no lane sections."), e->id);
    //                 continue;
    //             }
    //             std::vector<OpenDriveLaneSection>::iterator k = e->laneSections.begin();
    //             bool found = false;
    //             for (; k != e->laneSections.end() - 1 && !found;) {
    //                 if (signal.s > (*k).s && signal.s <= (*(k + 1)).s) {
    //                     found = true;
    //                 } else {
    //                     ++k;
    //                 }
    //             }
    //
    //             std::string id = (*k).sumoID;
    //             if (id == "") {
    //                 // traffic light on connecting road
    //                 if (e->junction != "") {
    //                     //WRITE_WARNINGF(TL("Found a traffic light signal on an internal edge; will not build it (original edge id='%')."), e->id);
    //                     std::string fromID, toID;
    //                     for (std::vector<OpenDriveLink>::const_iterator l = e->links.begin(); l != e->links.end(); ++l) {
    //                         if ((*l).linkType == OPENDRIVE_LT_PREDECESSOR && (*l).elementType == OPENDRIVE_ET_ROAD) {
    //                             if (fromID != "") {
    //                                 WRITE_WARNING(TL("Ambiguous start of connection."));
    //                             }
    //                             const OpenDriveEdge* const ode = edges[(*l).elementID];
    //                             if ((*l).contactPoint == OPENDRIVE_CP_START) {
    //                                 fromID = ode->laneSections[0].sumoID;
    //                                 if (signal.orientation < 0) {
    //                                     fromID = "-" + fromID;
    //                                 }
    //                             } else {
    //                                 fromID = ode->laneSections.back().sumoID;
    //                                 if (signal.orientation > 0) {
    //                                     fromID = "-" + fromID;
    //                                 }
    //                             }
    //                         }
    //                         if ((*l).linkType == OPENDRIVE_LT_SUCCESSOR && (*l).elementType == OPENDRIVE_ET_ROAD) {
    //                             if (toID != "") {
    //                                 WRITE_WARNING(TL("Ambiguous end of connection."));
    //                             }
    //                             const OpenDriveEdge* const ode = edges[(*l).elementID];
    //                             toID = (*l).contactPoint == OPENDRIVE_CP_START ? ode->laneSections[0].sumoID : ode->laneSections.back().sumoID;
    //                         }
    //                     }
    //                     // figure out the correct combination of directions
    //                     NBEdge* from;
    //                     NBEdge* to;
    //                     auto fromTo = retrieveSignalEdges(nb, fromID, toID, signal.minLane);
    //                     from = fromTo.first;
    //                     to = fromTo.second;
    //                     if (from == nullptr) {
    //                         WRITE_WARNINGF(TL("Could not find edge '%' for signal '%'."), fromID, signal.id);
    //                         continue;
    //                     }
    //
    //                     // consider signal validity to determine direction
    //                     if (signal.maxLane != 0) {
    //                         bool fromForward = from->getID()[0] == '-';
    //                         bool lanesForward = signal.maxLane < 0;
    //                         if (fromForward != lanesForward) {
    //                             std::swap(fromID, toID);
    //
    //                             const auto& signalFromTo = retrieveSignalEdges(nb, fromID, toID, signal.minLane);
    //                             from = signalFromTo.first;
    //                             to = signalFromTo.second;
    //                             if (from == nullptr) {
    //                                 WRITE_WARNINGF(TL("Could not find edge '%' for signal '%'."), fromID, signal.id);
    //                                 continue;
    //                             }
    //                         }
    //                     }
    //                     for (NBEdge::Connection& c : from->getConnections()) {
    //                         if (c.toEdge == to) {
    //                             int odLane = laneIndexMap[std::make_pair(from, c.fromLane)];
    //                             //std::cout << " e=" << e->id << " from=" << from->getID() << " fromLane=" << c.fromLane << " odLane=" << odLane << " sMin=" << signal.minLane << " sMax=" << signal.maxLane << "\n";
    //                             if (signal.minLane == 0 || (signal.minLane <= odLane && signal.maxLane >= odLane)) {
    //                                 if (c.hasParameter("signalID")) {
    //                                     c.setParameter("signalID", c.getParameter("signalID") + " " + signal.id);
    //                                 } else {
    //                                     c.setParameter("signalID", signal.id);
    //                                 }
    //                             }
    //                             // set tlIndex to allow signal groups (defined in OpenDRIVE controller elements)
    //                             if (importSignalGroups) {
    //                                 const OpenDriveController& controller = handler.getController(signal.id);
    //                                 if (controller.id != "") {
    //                                     if (c.getParameter("controllerID") != "") {
    //                                         WRITE_WARNINGF(TL("The signaling of the connection from '%' to '%' (controller '%') is ambiguous because it is overwritten signal '%' and with controller '%'."), from->getID(), c.toEdge->getID(), c.getParameter("controllerID"), signal.id, controller.id);
    //                                     }
    //                                     //junctionsWithControllers.insert(from->getToNode()->getID());
    //                                     int tlIndex = handler.getTLIndexForController(controller.id);
    //                                     c.tlLinkIndex = tlIndex;
    //                                     c.setParameter("controllerID", controller.id);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     getTLSSecure(from, nb);
    //
    //                     //std::cout << "odrEdge=" << e->id << " fromID=" << fromID << " toID=" << toID << " from=" << from->getID() << " to=" << to->getID()
    //                     //    << " signal=" << signal.id << " minLane=" << signal.minLane << " maxLane=" << signal.maxLane << "\n";
    //                 } else {
    //                     WRITE_WARNINGF(TL("Found a traffic light signal on an unknown edge (original edge id='%')."), e->id);
    //                     continue;
    //                 }
    //             } else {
    //                 // traffic light on normal road
    //                 if (signal.orientation == 1) {
    //                     // forward direction has negative lane indices and gets a negative prefix in sumo
    //                     id = "-" + id;
    //                 }
    //                 NBEdge* edge = nb.getEdgeCont().retrieve(id);
    //                 if (edge == nullptr) {
    //                     WRITE_WARNINGF(TL("Could not find edge '%' for signal '%'."), id, signal.id);
    //                     continue;
    //                 }
    //
    //                 /// XXX consider lane validity
    //                 for (NBEdge::Connection& c : edge->getConnections()) {
    //                     int odLane = laneIndexMap[std::make_pair(edge, c.fromLane)];
    //                     if (signal.minLane == 0 || (signal.minLane <= odLane && signal.maxLane >= odLane)) {
    //                         if (c.hasParameter("signalID")) {
    //                             c.setParameter("signalID", c.getParameter("signalID") + " " + signal.id);
    //                         } else {
    //                             c.setParameter("signalID", signal.id);
    //                         }
    //                     }
    //
    //                     // set tlIndex to allow signal groups (defined in OpenDRIVE controller elements)
    //                     if (importSignalGroups) {
    //                         const OpenDriveController& controller = handler.getController(signal.id);
    //                         if (controller.id != "") {
    //                             if (c.getParameter("controllerID") != "") {
    //                                 WRITE_WARNINGF(TL("The signaling of the connection from '%' to '%' (controller '%') is ambiguous because it is overwritten with signal '%' and controller '%'."), edge->getID(), c.toEdge->getID(), c.getParameter("controllerID"), signal.id, controller.id);
    //                             }
    //                             //junctionsWithControllers.insert(edge->getToNode()->getID());
    //                             int tlIndex = handler.getTLIndexForController(controller.id);
    //                             c.tlLinkIndex = tlIndex;
    //                             c.setParameter("controllerID", controller.id);
    //                         }
    //                     }
    //                 }
    //                 getTLSSecure(edge, nb);
    //                 //std::cout << "odrEdge=" << e->id << " sumoID=" << (*k).sumoID << " sumoEdge=" << edge->getID()
    //                 //    << " signal=" << signal.id << " minLane=" << signal.minLane << " maxLane=" << signal.maxLane << "\n";
    //             }
    //             // @note: tls 'signalID' parameters are set via NBTrafficLightLogicCont::setOpenDriveSignalParameters
    //             // @note: OpenDRIVE controllers are applied to the signal programs in NBTrafficLightLogicCont::applyOpenDriveControllers
    //         }
    //     }
    //
    //     // -------------------------
    //     // clean up
    //     // -------------------------
    //     for (std::map<std::string, OpenDriveEdge*>::iterator i = edges.begin(); i != edges.end(); ++i) {
    //         delete (*i).second;
    //     }
    // }
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) -> Result<(), ProcessError> {
        use utils::options::options_cont::{get_bool, get_float, get_string_vector};
        // check whether the option is set properly and all files exist
        if !oc.is_usable_file_list("opendrive-files").unwrap_or(false) {
            return Ok(());
        }
        // prepare types
        MY_IMPORT_ALL_TYPES.with(|v| v.set(get_bool("opendrive.import-all-lanes")));
        MY_IMPORT_WIDTHS.with(|v| v.set(!get_bool("opendrive.ignore-widths")));
        MY_MIN_WIDTH.with(|v| v.set(get_float("opendrive.min-width")));
        MY_IMPORT_INTERNAL_SHAPES.with(|v| v.set(get_bool("opendrive.internal-shapes")));
        MY_IGNORE_MISPLACED_SIGNALS.with(|v| v.set(get_bool("opendrive.ignore-misplaced-signals")));
        let customLaneShapes = get_bool("opendrive.lane-shapes");
        let tc: Ptr<NBTypeCont> = Ptr::from_mut(nb.get_type_cont());
        let nc: Ptr<NBNodeCont> = Ptr::from_mut(nb.get_node_cont());
        // build the handler
        let mut edges: BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>> = BTreeMap::new();
        let mut handler = NIImporter_OpenDrive::new(tc.m(), &mut edges);
        handler.needs_character_data(true);
        // parse file(s)
        for file in get_string_vector("opendrive-files") {
            handler.set_file_name(&file);
            progress_begin_message(&format!("Parsing opendrive from '{}'", file));
            utils::xml::xml_sub_sys::run_parser(&mut handler, &file, false, false, true, true)?;
            progress_done_message();
        }
        // apply signal reference information
        for (_, item) in &edges {
            for signal in item.m().signals.iter_mut() {
                if signal.type_ == "" {
                    if handler.get_signals().get(&signal.id).is_none() {
                        write_warningf!("Could not find signal reference '%'.", signal.id);
                    } else {
                        let (rType, rName, rDynamic, rController) = {
                            let r = &handler.get_signals()[&signal.id];
                            (r.type_.clone(), r.name.clone(), r.dynamic, r.controller.clone())
                        };
                        signal.type_ = rType;
                        signal.name = rName;
                        signal.dynamic = rDynamic;
                        signal.controller = rController;
                    }
                }
            }
        }

        // split inner/outer edges
        let mut innerEdges: BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>> = BTreeMap::new();
        let mut outerEdges: BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>> = BTreeMap::new();
        for (k, v) in &edges {
            if v.m().isInner {
                innerEdges.insert(k.clone(), v.clone());
            } else {
                outerEdges.insert(k.clone(), v.clone());
            }
        }

        // convert geometries into a discretised representation
        NIImporter_OpenDrive::compute_shapes(&mut edges);
        // check whether lane sections are valid and whether further must be introduced
        NIImporter_OpenDrive::revisit_lane_sections(tc.m(), &mut edges);

        // -------------------------
        // node building
        // -------------------------
        // build nodes#1
        //  look at all links which belong to a node, collect their bounding boxes
        //  and place the node in the middle of this bounding box
        let mut posMap: BTreeMap<String, Boundary> = BTreeMap::new();
        let mut edge2junction: BTreeMap<String, String> = BTreeMap::new();
        let mut joinedNodeIDs: Vec<NodeSet> = Vec::new();
        //   compute node positions
        for (_, ie) in &innerEdges {
            let e = ie.m();
            assert!(e.junction != "-1" && e.junction != "");
            edge2junction.insert(e.id.clone(), e.junction.clone());
            posMap.entry(e.junction.clone()).or_default();
            // Check if geometry is valid before adding to boundary
            if e.geom.len() > 0 {
                let geomBoundary = e.geom.get_box_boundary();
                // Only add if the boundary doesn't contain NaN values
                if geomBoundary.is_initialised()
                    && !geomBoundary.xmin().is_nan() && !geomBoundary.xmax().is_nan()
                    && !geomBoundary.ymin().is_nan() && !geomBoundary.ymax().is_nan()
                {
                    posMap.get_mut(&e.junction).unwrap().add_boundary(&geomBoundary);
                } else {
                    write_warningf!("Ignoring invalid geometry for inner edge '%' (xodr road '%') in junction '%'.", e.id, e.id, e.junction);
                }
            } else {
                write_warningf!("Inner edge '%' (xodr road '%') in junction '%' has no geometry.", e.id, e.id, e.junction);
            }
        }
        //   build nodes
        for (jid, bnd) in &posMap {
            // Check if the boundary was properly initialized
            if !bnd.is_initialised() {
                write_errorf!("Junction '%' has no valid inner edges to determine its position.", jid.clone());
                return Err(ProcessError::new(tlf!("Could not determine position for junction '%'.", jid.clone())));
            }
            let center = bnd.get_center();
            // Double-check that the center position is valid
            if center.is_nan() {
                write_errorf!("Junction '%' resulted in invalid (NaN) position.", jid.clone());
                return Err(ProcessError::new(tlf!("Could not compute valid position for junction '%'.", jid.clone())));
            }
            if !nc.m().insert(jid, &center, Ptr::default()) {
                return Err(ProcessError::new(tlf!("Could not add node '%'.", jid.clone())));
            }
        }
        //  assign built nodes
        for (_, oe) in &outerEdges {
            let e = oe.m();
            for j in 0..e.links.len() {
                let linkType = e.links[j].linkType;
                let elementType = e.links[j].elementType;
                let nid = e.links[j].elementID.clone();
                if elementType != NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_ROAD {
                    if nc.m().retrieve(&nid).is_null() {
                        // not yet seen, build (possibly a junction without connections)
                        let pos = if linkType == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR { *e.geom.back() } else { *e.geom.front() };
                        if !nc.m().insert(&nid, &pos, Ptr::default()) {
                            return Err(ProcessError::new(tlf!("Could not build node '%'.", nid.clone())));
                        }
                    }
                    // set node information
                    NIImporter_OpenDrive::set_node_secure(nc.m(), e, &nid, linkType, &mut joinedNodeIDs)?;
                    continue;
                }
                if edge2junction.contains_key(&nid) {
                    // set node information of an internal road
                    let junction = edge2junction[&nid].clone();
                    NIImporter_OpenDrive::set_node_secure(nc.m(), e, &junction, linkType, &mut joinedNodeIDs)?;
                    continue;
                }
            }
        }
        //  we should now have all nodes set for links which are not outer edge-to-outer edge links

        // build nodes#2
        //  build nodes for all outer edge-to-outer edge connections
        for (_, oe) in &outerEdges {
            let e = oe.m();
            for j in 0..e.links.len() {
                let linkType = e.links[j].linkType;
                let elementType = e.links[j].elementType;
                let elementID = e.links[j].elementID.clone();
                if elementType != NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_ROAD || edge2junction.contains_key(&elementID) {
                    // is a connection to an internal edge, or a node, skip
                    continue;
                }
                // we have a direct connection between to external edges
                let mut id1 = e.id.clone();
                let mut id2 = elementID.clone();
                if id1 < id2 {
                    std::mem::swap(&mut id1, &mut id2);
                }
                let nid = format!("{}.{}", id1, id2);
                if nc.m().retrieve(&nid).is_null() {
                    // not yet seen, build
                    let pos = if linkType == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR { *e.geom.back() } else { *e.geom.front() };
                    if !nc.m().insert(&nid, &pos, Ptr::default()) {
                        return Err(ProcessError::new(tlf!("Could not build node '%'.", nid.clone())));
                    }
                }
                NIImporter_OpenDrive::set_node_secure(nc.m(), e, &nid, linkType, &mut joinedNodeIDs)?;
            }
        }
        // we should now have start/end nodes for all outer edge-to-outer edge connections

        // build nodes#3
        //  assign further nodes generated from inner-edges
        //  these nodes have not been assigned earlier, because the connections are referenced in inner-edges
        for (_, oe) in &outerEdges {
            let e = oe.m();
            if !e.to.is_null() && !e.from.is_null() {
                continue;
            }
            for (_, je) in &innerEdges {
                let ie = je.m();
                for k in 0..ie.links.len() {
                    let elementType = ie.links[k].elementType;
                    let elementID = ie.links[k].elementID.clone();
                    let contactPoint = ie.links[k].contactPoint;
                    if elementType != NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_ROAD || elementID != e.id {
                        // not conneted to the currently investigated outer edge
                        continue;
                    }
                    let nid = edge2junction.entry(ie.id.clone()).or_default().clone();
                    if contactPoint == NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START {
                        NIImporter_OpenDrive::set_node_secure(nc.m(), e, &nid, NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_PREDECESSOR, &mut joinedNodeIDs)?;
                    } else {
                        NIImporter_OpenDrive::set_node_secure(nc.m(), e, &nid, NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR, &mut joinedNodeIDs)?;
                    }
                }
            }
        }

        // build start/end nodes which were not defined previously
        for (_, oe) in &outerEdges {
            let e = oe.m();
            if (e.from.is_null() || e.to.is_null()) && e.geom.len() == 0 {
                continue;
            }
            if e.from.is_null() {
                let nid = format!("{}.begin", e.id);
                e.from = NIImporter_OpenDrive::get_or_build_node(&nid, e.geom.front(), nc.m())?;
            }
            if e.to.is_null() {
                let nid = format!("{}.end", e.id);
                e.to = NIImporter_OpenDrive::get_or_build_node(&nid, e.geom.back(), nc.m())?;
            }
        }

        let mut joinedNodes: BTreeMap<Ptr<NBNode>, Ptr<NBNode>> = BTreeMap::new();
        for joined in &joinedNodeIDs {
            let mut joinedPos = Position::new(0.0, 0.0);
            for j in joined {
                joinedPos.add(j.m().get_position());
            }
            joinedPos.mul(1. / (joined.len() as f64));
            let joinedID = nc.m().create_cluster_id(joined, "cluster_");
            if !nc.m().insert(&joinedID, &joinedPos, Ptr::default()) {
                return Err(ProcessError::new(tlf!("Could not add node '%'.", joinedID.clone())));
            }
            let n = nc.m().retrieve(&joinedID);
            for j in joined {
                joinedNodes.insert(*j, n);
            }
        }
        for (_, oe) in &outerEdges {
            let e = oe.m();
            if joinedNodes.contains_key(&e.from) {
                nc.m().extract(e.from, true);
                e.from = joinedNodes[&e.from];
            }
            if joinedNodes.contains_key(&e.to) {
                nc.m().extract(e.to, true);
                e.to = joinedNodes[&e.to];
            }
        }

        // -------------------------
        // edge building
        // -------------------------
        let defaultSpeed = tc.m().get_edge_type_speed("");
        let saveOrigIDs = get_bool("output.original-names");
        let positionIDs = get_bool("opendrive.position-ids");
        // lane-id-map sumoEdge,sumoLaneIndex->odrLaneIndex
        let mut laneIndexMap: BTreeMap<(Ptr<NBEdge>, i32), i32> = BTreeMap::new();
        // build edges
        for (_, oe) in &outerEdges {
            let e = oe.m();
            if e.geom.len() < 2 {
                write_warningf!("Ignoring road '%' without geometry.", e.id);
                continue;
            }
            let mut lanesBuilt = false;

            // go along the lane sections, build a node in between of each pair

            // @todo: One could think of determining whether lane sections may be joined when being equal in SUMO's sense
            // @todo: probably, the lane offsets to the center are not right
            let mut sFrom = e.from;
            let mut sTo = e.to;
            let priorityR = e.get_priority(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT);
            let priorityL = e.get_priority(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT);
            let mut sB = 0.0_f64;
            let mut sE = e.length;
            // 0-length geometries are possible if only the inner points are represented
            let mut geomWithOffset = e.geom.clone();
            if e.laneOffsets.len() > 0 {
                if geomWithOffset.move2side_custom(&e.laneOffsets, 100.0).is_err() {
                    write_warningf!("Could not apply laneOffsets for edge '%'", e.id);
                }
            }
            let length2D = geomWithOffset.length2d();
            let mut cF = if length2D == 0.0 { 1.0 } else { e.length / length2D };
            let mut prevRight: Ptr<NBEdge> = Ptr::default();
            let mut prevLeft: Ptr<NBEdge> = Ptr::default();

            // starting at the same node as ending, and no lane sections?
            if sFrom == sTo && e.laneSections.len() == 1 {
                // --> loop, split!
                let mut ls = e.laneSections[0].clone();
                ls.s = e.length / 2.;
                e.laneSections.push(ls);
                write_warning(&format!("Edge '{}' has to be split as it connects same junctions.", e.id));
            }
            NIImporter_OpenDrive::sanitize_widths(oe.clone());
            if MY_MIN_WIDTH.with(|v| v.get()) > 0.0 {
                let minDist = get_float("opendrive.curve-resolution");
                NIImporter_OpenDrive::split_min_widths(oe.clone(), tc.m(), minDist);
            }

            // build along lane sections
            let mut sectionIndex = 0;
            let nSections = e.laneSections.len();
            for j in 0..nSections {
                // add internal node if needed
                if j == nSections - 1 {
                    sTo = e.to;
                    sE = e.length / cF;
                } else {
                    let nextS = e.laneSections[j + 1].s;
                    let nodeID = if positionIDs {
                        format!("{}.{}", e.id, utils::common::to_string::f64_to_string_prec(nextS, 2))
                    } else {
                        format!("{}#{}", e.id, sectionIndex + 1)
                    };
                    let node = NBNode::new(&nodeID, &geomWithOffset.position_at_offset(nextS, 0.0), utils::xml::sumo_xml_definitions::SumoXMLNodeType::UNKNOWN);
                    sTo = Ptr::from_raw(Box::into_raw(Box::new(node)));
                    if !nb.get_node_cont().insert_2(sTo) {
                        return Err(ProcessError::new(tlf!("Could not add node '%'.", sTo.m().get_id())));
                    }
                    sE = nextS / cF;
                }
                let geom = geomWithOffset.get_subpart2d(sB, sE).simplified2(false, utils::geom::NUMERICAL_EPS);
                let mut id = e.id.clone();
                if positionIDs {
                    if sFrom != e.from || sTo != e.to {
                        id = format!("{}.{}", id, utils::common::to_string::f64_to_string_prec(e.laneSections[j].s, 2));
                    } else if e.laneSections.len() == 1 {
                        id = format!("{}.0.00", id);
                    }
                } else if e.laneSections.len() > 1 {
                    id = format!("{}#{}", id, sectionIndex);
                    sectionIndex += 1;
                }

                // build lanes to right
                let mut currRight: Ptr<NBEdge> = Ptr::default();
                if e.laneSections[j].rightLaneNumber > 0 {
                    let mut offsets: Vec<f64> = vec![0.0; geom.len()];
                    let mut useOffsets = false;
                    let mut rightGeom = geom.clone();
                    rightGeom.move2side(e.laneSections[j].discardedInnerWidthRight, 100.0);
                    let mut laneGeom = rightGeom.clone();
                    let edge = NBEdge::new_2(&format!("-{}", id), sFrom, sTo, e.laneSections[j].rightType.clone(), defaultSpeed,
                        NBEDGE_UNSPECIFIED_FRICTION, e.laneSections[j].rightLaneNumber, priorityR, NBEDGE_UNSPECIFIED_WIDTH, NBEDGE_UNSPECIFIED_OFFSET,
                        rightGeom.clone(), utils::xml::sumo_xml_definitions::LaneSpreadFunction::RIGHT, &e.streetName, "", true);
                    currRight = Ptr::from_raw(Box::into_raw(edge));
                    lanesBuilt = true;
                    // determinism: C++ std::sort is unstable; the comparator only orders by |id|
                    let mut lanes = std::mem::take(e.laneSections[j].lanesByDir.entry(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT).or_default());
                    let sorter = NIImporter_OpenDrive_LaneSorter::new();
                    lanes.sort_by(|a, b| lane_sorter_ordering(&sorter, a, b));
                    for odl in &lanes {
                        if let Some(&sumoLaneIndex) = e.laneSections[j].laneMap.get(&odl.id) {
                            NIImporter_OpenDrive::set_lane_attributes(oe.clone(), currRight.m().get_lane_struct(sumoLaneIndex), odl, saveOrigIDs, tc.m());
                            laneIndexMap.insert((currRight, sumoLaneIndex), odl.id);
                            if useOffsets {
                                let mut laneShape = laneGeom.clone();
                                let _ = laneShape.move2side_custom(&offsets, 100.0);
                                currRight.m().get_lane_struct(sumoLaneIndex).customShape = laneShape;
                            }
                        } else if customLaneShapes {
                            useOffsets = true;
                        }
                        if customLaneShapes {
                            NIImporter_OpenDrive::add_offsets(false, &mut laneGeom, &odl.widthData, &format!("{}_{}", e.id, odl.id), &mut offsets);
                        }
                    }
                    e.laneSections[j].lanesByDir.insert(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT, lanes);
                    if !nb.get_edge_cont().insert(currRight, MY_IMPORT_ALL_TYPES.with(|v| v.get())) {
                        return Err(ProcessError::new(tlf!("Could not add edge '%'.", currRight.m().get_id())));
                    }
                    if nb.get_edge_cont().was_ignored(format!("-{}", id)) {
                        prevRight = Ptr::default();
                    } else {
                        // connect lane sections
                        if !prevRight.is_null() {
                            let (before, after) = e.laneSections.split_at_mut(j);
                            let connections = after[0].get_inner_connections(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT, &before[j - 1]);
                            for (k1, k2) in connections {
                                prevRight.m().add_lane2_lane_connection(k1, currRight, k2, NBEdge_Lane2LaneInfoType::VALIDATED, false, false,
                                    KeepClear::KEEPCLEAR_UNSPECIFIED, NBEDGE_UNSPECIFIED_CONTPOS, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE,
                                    NBEDGE_UNSPECIFIED_SPEED, NBEDGE_UNSPECIFIED_FRICTION, NBEDGE_UNSPECIFIED_LOADED_LENGTH,
                                    &PositionVector::default(), false, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false);
                            }
                        }
                        prevRight = currRight;
                    }
                }

                // build lanes to left
                let mut currLeft: Ptr<NBEdge> = Ptr::default();
                if e.laneSections[j].leftLaneNumber > 0 {
                    let mut offsets: Vec<f64> = vec![0.0; geom.len()];
                    let mut useOffsets = false;
                    let mut leftGeom = geom.clone();
                    leftGeom.move2side(-e.laneSections[j].discardedInnerWidthLeft, 100.0);
                    let mut laneGeom = leftGeom.clone();
                    let edge = NBEdge::new_2(&id, sTo, sFrom, e.laneSections[j].leftType.clone(), defaultSpeed,
                        NBEDGE_UNSPECIFIED_FRICTION, e.laneSections[j].leftLaneNumber, priorityL, NBEDGE_UNSPECIFIED_WIDTH, NBEDGE_UNSPECIFIED_OFFSET,
                        leftGeom.reverse(), utils::xml::sumo_xml_definitions::LaneSpreadFunction::RIGHT, &e.streetName, "", true);
                    currLeft = Ptr::from_raw(Box::into_raw(edge));
                    lanesBuilt = true;
                    // determinism: C++ std::sort is unstable; the comparator only orders by |id|
                    let mut lanes = std::mem::take(e.laneSections[j].lanesByDir.entry(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT).or_default());
                    let sorter = NIImporter_OpenDrive_LaneSorter::new();
                    lanes.sort_by(|a, b| lane_sorter_ordering(&sorter, a, b));
                    for odl in &lanes {
                        if let Some(&sumoLaneIndex) = e.laneSections[j].laneMap.get(&odl.id) {
                            NIImporter_OpenDrive::set_lane_attributes(oe.clone(), currLeft.m().get_lane_struct(sumoLaneIndex), odl, saveOrigIDs, tc.m());
                            laneIndexMap.insert((currLeft, sumoLaneIndex), odl.id);
                            if useOffsets {
                                let mut laneShape = laneGeom.clone();
                                let _ = laneShape.move2side_custom(&offsets, 100.0);
                                currLeft.m().get_lane_struct(sumoLaneIndex).customShape = laneShape.reverse();
                            }
                        } else if customLaneShapes {
                            useOffsets = true;
                        }
                        if customLaneShapes {
                            NIImporter_OpenDrive::add_offsets(true, &mut laneGeom, &odl.widthData, &format!("{}_{}", e.id, odl.id), &mut offsets);
                        }
                    }
                    e.laneSections[j].lanesByDir.insert(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT, lanes);
                    if !nb.get_edge_cont().insert(currLeft, MY_IMPORT_ALL_TYPES.with(|v| v.get())) {
                        return Err(ProcessError::new(tlf!("Could not add edge '%'.", currLeft.m().get_id())));
                    }
                    if nb.get_edge_cont().was_ignored(id.clone()) {
                        prevLeft = Ptr::default();
                    } else {
                        // connect lane sections
                        if !prevLeft.is_null() {
                            let (before, after) = e.laneSections.split_at_mut(j);
                            let connections = after[0].get_inner_connections(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT, &before[j - 1]);
                            for (k1, k2) in connections {
                                currLeft.m().add_lane2_lane_connection(k1, prevLeft, k2, NBEdge_Lane2LaneInfoType::VALIDATED, false, false,
                                    KeepClear::KEEPCLEAR_UNSPECIFIED, NBEDGE_UNSPECIFIED_CONTPOS, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE,
                                    NBEDGE_UNSPECIFIED_SPEED, NBEDGE_UNSPECIFIED_FRICTION, NBEDGE_UNSPECIFIED_LOADED_LENGTH,
                                    &PositionVector::default(), false, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false);
                            }
                        }
                        prevLeft = currLeft;
                    }
                }
                e.laneSections[j].sumoID = id.clone();

                sB = sE;
                sFrom = sTo;
            }
            if utils::options::options_cont::is_set("polygon-output") {
                NIImporter_OpenDrive::write_road_objects(oe.clone())?;
            }
            if !lanesBuilt {
                write_warningf!("Edge '%' has no lanes.", e.id);
            }
        }
        if utils::options::options_cont::is_set("polygon-output") {
            for (_, item) in &innerEdges {
                NIImporter_OpenDrive::write_road_objects(item.clone())?;
            }
        }

        // -------------------------
        // connections building
        // -------------------------
        // generate explicit lane-to-lane connections
        for (_, oe) in &edges {
            NIImporter_OpenDrive::set_edge_links2(oe.m(), &edges);
        }
        // compute connections across intersections, if any
        let mut connections2: Vec<NIImporter_OpenDrive_Connection> = Vec::new();
        for (_, oe) in &edges {
            let conns = oe.m().connections.clone();
            for i in &conns {
                if innerEdges.contains_key(&i.fromEdge) {
                    // connections starting at inner edges are processed by starting from outer edges
                    continue;
                }
                if innerEdges.contains_key(&i.toEdge) {
                    let mut seen: BTreeSet<NIImporter_OpenDrive_Connection> = BTreeSet::new();
                    NIImporter_OpenDrive::build_connections_to_outer(i, &innerEdges, &edges, tc.m(), &mut connections2, &mut seen);
                } else {
                    connections2.push(i.clone());
                }
            }
        }
        // set connections
        for i in &connections2 {
            let mut fromEdge = i.fromEdge.clone();
            if !edges.contains_key(&fromEdge) {
                write_warningf!("While setting connections: from-edge '%' is not known.", fromEdge);
                continue;
            }
            let odFrom = edges[&fromEdge];
            let mut fromLane = i.fromLane;
            let fromLast = (i.fromCP == NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END) && (i.fromLane < 0);
            fromEdge = if fromLast {
                odFrom.m().laneSections.last().unwrap().sumoID.clone()
            } else {
                odFrom.m().laneSections[0].sumoID.clone()
            };

            let mut toEdge = i.toEdge.clone();
            if !edges.contains_key(&toEdge) {
                write_warningf!("While setting connections: to-edge '%' is not known.", toEdge);
                continue;
            }

            let odTo = edges[&toEdge];
            let mut toLane = i.toLane;
            let toLast = (i.toCP == NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END) || (i.toLane > 0);
            toEdge = if toLast {
                odTo.m().laneSections.last().unwrap().sumoID.clone()
            } else {
                odTo.m().laneSections[0].sumoID.clone()
            };

            if fromLane == UNSET_CONNECTION {
                continue;
            }
            if fromLane < 0 {
                fromEdge = NIImporter_OpenDrive::reversed_edge_id(&fromEdge);
            }
            if toLane == UNSET_CONNECTION {
                continue;
            }
            if toLane < 0 {
                toEdge = NIImporter_OpenDrive::reversed_edge_id(&toEdge);
            }
            fromLane = if fromLast {
                *odFrom.m().laneSections.last_mut().unwrap().laneMap.entry(fromLane).or_insert(0)
            } else {
                *odFrom.m().laneSections[0].laneMap.entry(fromLane).or_insert(0)
            };
            toLane = if toLast {
                *odTo.m().laneSections.last_mut().unwrap().laneMap.entry(toLane).or_insert(0)
            } else {
                *odTo.m().laneSections[0].laneMap.entry(toLane).or_insert(0)
            };
            let from = nb.get_edge_cont().retrieve(&fromEdge, false);
            let to = nb.get_edge_cont().retrieve(&toEdge, false);
            if from.is_null() {
                write_warningf!("Could not find fromEdge representation of '%' in connection '%'.", fromEdge, i.origID);
            }
            if to.is_null() {
                write_warningf!("Could not find fromEdge representation of '%' in connection '%'.", toEdge, i.origID);
            }
            if from.is_null() || to.is_null() {
                continue;
            }

            from.m().add_lane2_lane_connection(fromLane, to, toLane, NBEdge_Lane2LaneInfoType::USER, false, false,
                KeepClear::KEEPCLEAR_UNSPECIFIED, NBEDGE_UNSPECIFIED_CONTPOS, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE,
                NBEDGE_UNSPECIFIED_SPEED, NBEDGE_UNSPECIFIED_FRICTION, NBEDGE_UNSPECIFIED_LOADED_LENGTH,
                &i.shape, false, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false);

            if i.origID != "" && saveOrigIDs {
                // shortcut: C++ sets the ORIGID parameter on the matching NBEdge::Connection; the Rust
                // NBEdge::get_connections() still returns NIImporter_OpenDrive_Connection, so it cannot be set yet
            }
        }

        // -------------------------
        // traffic lights
        // -------------------------
        let controllers = handler.get_controllers();
        for (_, it) in &edges {
            let e = it.m();
            for signal in &e.signals {
                // signal for which junction?
                if signal.controller.len() == 0 {
                    continue;
                }
                for connection in &e.connections {
                    if (connection.fromLane < 0 && signal.orientation < 0) || (connection.fromLane > 0 && signal.orientation > 0) {
                        continue;
                    }
                    if (signal.minLane == 0 && signal.maxLane == 0) || (signal.maxLane >= connection.fromLane && signal.minLane <= connection.fromLane) {
                        let connectedEdge = edges[&connection.toEdge];
                        let ctrl = controllers.entry(signal.controller.clone()).or_insert_with(NIImporter_OpenDrive_OpenDriveController::new_2);
                        if ctrl.junction.len() > 0 && ctrl.junction != connectedEdge.m().junction {
                            write_warningf!("Controlling multiple junctions by the same controller '%' is currently not implemented.", signal.controller);
                        }
                        controllers.get_mut(&signal.controller).unwrap().junction = connectedEdge.m().junction.clone();
                    }
                }
            }
        }

        let importSignalGroups = get_bool("opendrive.signal-groups");
        for (_, oe) in &edges {
            let e = oe.m();
            for signal in &e.signals {
                let mut intType: i32 = -1;
                if let Ok(v) = utils::common::string_utils::to_int(&signal.type_) {
                    intType = v;
                }
                if intType < 1000001 || (intType > 1000013 && intType != 1000020) || intType == 1000008 {
                    // not a traffic_light (Section 6.11)
                    continue;
                }
                if e.laneSections.len() == 0 {
                    write_warningf!("Edge '%' has signals but no lane sections.", e.id);
                    continue;
                }
                let mut k: usize = 0;
                let mut found = false;
                while k != e.laneSections.len() - 1 && !found {
                    if signal.s > e.laneSections[k].s && signal.s <= e.laneSections[k + 1].s {
                        found = true;
                    } else {
                        k += 1;
                    }
                }

                let mut id = e.laneSections[k].sumoID.clone();
                if id == "" {
                    // traffic light on connecting road
                    if e.junction != "" {
                        let mut fromID = String::new();
                        let mut toID = String::new();
                        for l in &e.links {
                            if l.linkType == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_PREDECESSOR && l.elementType == NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_ROAD {
                                if fromID != "" {
                                    write_warning("Ambiguous start of connection.");
                                }
                                let ode = edges[&l.elementID];
                                if l.contactPoint == NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START {
                                    fromID = ode.m().laneSections[0].sumoID.clone();
                                    if signal.orientation < 0 {
                                        fromID = format!("-{}", fromID);
                                    }
                                } else {
                                    fromID = ode.m().laneSections.last().unwrap().sumoID.clone();
                                    if signal.orientation > 0 {
                                        fromID = format!("-{}", fromID);
                                    }
                                }
                            }
                            if l.linkType == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR && l.elementType == NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_ROAD {
                                if toID != "" {
                                    write_warning("Ambiguous end of connection.");
                                }
                                let ode = edges[&l.elementID];
                                toID = if l.contactPoint == NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START {
                                    ode.m().laneSections[0].sumoID.clone()
                                } else {
                                    ode.m().laneSections.last().unwrap().sumoID.clone()
                                };
                            }
                        }
                        // figure out the correct combination of directions
                        let fromTo = NIImporter_OpenDrive::retrieve_signal_edges(nb, &fromID, &toID, signal.minLane);
                        let mut from = fromTo.0;
                        let mut to = fromTo.1;
                        if from.is_null() {
                            write_warningf!("Could not find edge '%' for signal '%'.", fromID, signal.id);
                            continue;
                        }

                        // consider signal validity to determine direction
                        if signal.maxLane != 0 {
                            let fromForward = from.m().get_id().starts_with('-');
                            let lanesForward = signal.maxLane < 0;
                            if fromForward != lanesForward {
                                std::mem::swap(&mut fromID, &mut toID);
                                let signalFromTo = NIImporter_OpenDrive::retrieve_signal_edges(nb, &fromID, &toID, signal.minLane);
                                from = signalFromTo.0;
                                to = signalFromTo.1;
                                if from.is_null() {
                                    write_warningf!("Could not find edge '%' for signal '%'.", fromID, signal.id);
                                    continue;
                                }
                            }
                        }
                        // shortcut: C++ loops over from->getConnections() to set tlLinkIndex, signalID and controllerID;
                        // the Rust NBEdge::get_connections() still returns NIImporter_OpenDrive_Connection, so this is skipped until it is ported
                        let _ = to;
                        NIImporter_OpenDrive::get_tls_secure(from, nb)?;
                    } else {
                        write_warningf!("Found a traffic light signal on an unknown edge (original edge id='%').", e.id);
                        continue;
                    }
                } else {
                    // traffic light on normal road
                    if signal.orientation == 1 {
                        // forward direction has negative lane indices and gets a negative prefix in sumo
                        id = format!("-{}", id);
                    }
                    let edge = nb.get_edge_cont().retrieve(&id, false);
                    if edge.is_null() {
                        write_warningf!("Could not find edge '%' for signal '%'.", id, signal.id);
                        continue;
                    }
                    // shortcut: connection-level signalID/controllerID/tlLinkIndex assignment is skipped, see above
                    let _ = importSignalGroups;
                    NIImporter_OpenDrive::get_tls_secure(edge, nb)?;
                }
                // @note: tls 'signalID' parameters are set via NBTrafficLightLogicCont::setOpenDriveSignalParameters
                // @note: OpenDRIVE controllers are applied to the signal programs in NBTrafficLightLogicCont::applyOpenDriveControllers
            }
        }

        // -------------------------
        // clean up
        // -------------------------
        // shortcut: C++ deletes every OpenDriveEdge here; their ownership is not defined by the port yet, so they are not freed
        Ok(())
    }

    /// C++: `void NIImporter_OpenDrive::NIImporter_OpenDrive(const NBTypeCont &, std::map<std::string, OpenDriveEdge *> &)` [protected]
    /// Constructor
    // NIImporter_OpenDrive::NIImporter_OpenDrive(const NBTypeCont& tc, std::map<std::string, OpenDriveEdge*>& edges)
    //     : GenericSAXHandler(openDriveTags, OPENDRIVE_TAG_NOTHING, openDriveAttrs, OPENDRIVE_ATTR_NOTHING, "opendrive"),
    //       myTypeContainer(tc), myCurrentEdge("", "", "", -1), myCurrentController("", ""), myEdges(edges), myOffset(0, 0),
    //       myUseCurrentNode(false) {
    // }
    #[allow(unused_variables)]
    pub fn new(tc: &NBTypeCont, edges: &mut BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>>) -> Self {
        Self {
            base: GenericSAXHandler::new(OPEN_DRIVE_TAGS, NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_NOTHING as i32, OPEN_DRIVE_ATTRS, NIImporter_OpenDrive_OpenDriveXMLAttr::OPENDRIVE_ATTR_NOTHING as i32, "opendrive", ""),
            myTypeContainer: Ptr::new(tc),
            myCurrentEdge: NIImporter_OpenDrive_OpenDriveEdge::new("", "", "", -1.0),
            myCurrentController: NIImporter_OpenDrive_OpenDriveController::new("", String::new()),
            myEdges: Ptr::from_mut(edges),
            myElementStack: Vec::new(),
            myCurrentLaneDirection: NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_NOTHING,
            myCurrentJunctionID: String::new(),
            myCurrentIncomingRoad: String::new(),
            myCurrentConnectingRoad: String::new(),
            myCurrentContactPoint: NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_UNKNOWN,
            myConnectionWasEmpty: false,
            mySignals: BTreeMap::new(),
            myControllers: BTreeMap::new(),
            myJunctions2Controllers: BTreeMap::new(),
            myOffset: Position::new(0.0, 0.0),
            myUseCurrentNode: false,
        }
    }

    /// C++: `void NIImporter_OpenDrive::~NIImporter_OpenDrive() noexcept` [virtual, protected]
    /// Destructor
    // NIImporter_OpenDrive::~NIImporter_OpenDrive() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty: nothing to release
    }

    /// C++: `void NIImporter_OpenDrive::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_OpenDrive::myStartElement(int element,
    //                                      const SUMOSAXAttributes& attrs) {
    //     if (myUseCurrentNode) { // skip the parent node repeated in the included file
    //         myUseCurrentNode = false;
    //         myElementStack.push_back(element);
    //         return;
    //     }
    //     bool ok = true;
    //     switch (element) {
    //         case OPENDRIVE_TAG_HEADER: {
    //             int majorVersion = attrs.get<int>(OPENDRIVE_ATTR_REVMAJOR, nullptr, ok);
    //             int minorVersion = attrs.get<int>(OPENDRIVE_ATTR_REVMINOR, nullptr, ok);
    //             if (majorVersion == 1 && minorVersion > 4) { // disable flags only used for old 1.4 standard
    //                 NIImporter_OpenDrive::myIgnoreMisplacedSignals = false;
    //             }
    //             /*
    //             if (majorVersion != 1 || minorVersion != 2) {
    //                 // TODO: leave note of exceptions
    //                 WRITE_WARNINGF(TL("Given openDrive file '%' uses version %.%;\n Version 1.2 is supported."), getFileName(), toString(majorVersion), toString(minorVersion));
    //             }
    //             */
    //         }
    //         break;
    //         case OPENDRIVE_TAG_OFFSET: {
    //             double x = attrs.get<double>(OPENDRIVE_ATTR_X, "offset", ok);
    //             double y = attrs.get<double>(OPENDRIVE_ATTR_Y, "offset", ok);
    //             double z = attrs.get<double>(OPENDRIVE_ATTR_Z, "offset", ok);
    //             myOffset.set(-x, -y, -z);
    //             if (GeoConvHelper::getNumLoaded()) {
    //                 GeoConvHelper::getLoaded().moveConvertedBy(-x, -y);
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_ROAD: {
    //             std::string id = attrs.get<std::string>(OPENDRIVE_ATTR_ID, nullptr, ok);
    //             std::string streetName = attrs.getOpt<std::string>(OPENDRIVE_ATTR_NAME, nullptr, ok, "", false);
    //             std::string junction = attrs.get<std::string>(OPENDRIVE_ATTR_JUNCTION, id.c_str(), ok);
    //             double length = attrs.get<double>(OPENDRIVE_ATTR_LENGTH, id.c_str(), ok);
    //             myCurrentEdge = OpenDriveEdge(id, streetName, junction, length);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_PREDECESSOR: {
    //             if (myElementStack.size() >= 2 && myElementStack[myElementStack.size() - 2] == OPENDRIVE_TAG_ROAD) {
    //                 std::string elementType = attrs.get<std::string>(OPENDRIVE_ATTR_ELEMENTTYPE, myCurrentEdge.id.c_str(), ok);
    //                 std::string elementID = attrs.get<std::string>(OPENDRIVE_ATTR_ELEMENTID, myCurrentEdge.id.c_str(), ok);
    //                 std::string contactPoint = attrs.hasAttribute(OPENDRIVE_ATTR_CONTACTPOINT)
    //                                            ? attrs.get<std::string>(OPENDRIVE_ATTR_CONTACTPOINT, myCurrentEdge.id.c_str(), ok)
    //                                            : "end";
    //                 addLink(OPENDRIVE_LT_PREDECESSOR, elementType, elementID, contactPoint);
    //             }
    //             if (myElementStack.size() >= 2 && myElementStack[myElementStack.size() - 2] == OPENDRIVE_TAG_LANE) {
    //                 int no = attrs.get<int>(OPENDRIVE_ATTR_ID, myCurrentEdge.id.c_str(), ok);
    //                 OpenDriveLane& l = myCurrentEdge.laneSections.back().lanesByDir[myCurrentLaneDirection].back();
    //                 l.predecessor = no;
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_SUCCESSOR: {
    //             if (myElementStack.size() >= 2 && myElementStack[myElementStack.size() - 2] == OPENDRIVE_TAG_ROAD) {
    //                 std::string elementType = attrs.get<std::string>(OPENDRIVE_ATTR_ELEMENTTYPE, myCurrentEdge.id.c_str(), ok);
    //                 std::string elementID = attrs.get<std::string>(OPENDRIVE_ATTR_ELEMENTID, myCurrentEdge.id.c_str(), ok);
    //                 std::string contactPoint = attrs.hasAttribute(OPENDRIVE_ATTR_CONTACTPOINT)
    //                                            ? attrs.get<std::string>(OPENDRIVE_ATTR_CONTACTPOINT, myCurrentEdge.id.c_str(), ok)
    //                                            : "start";
    //                 addLink(OPENDRIVE_LT_SUCCESSOR, elementType, elementID, contactPoint);
    //             }
    //             if (myElementStack.size() >= 2 && myElementStack[myElementStack.size() - 2] == OPENDRIVE_TAG_LANE) {
    //                 int no = attrs.get<int>(OPENDRIVE_ATTR_ID, myCurrentEdge.id.c_str(), ok);
    //                 OpenDriveLane& l = myCurrentEdge.laneSections.back().lanesByDir[myCurrentLaneDirection].back();
    //                 l.successor = no;
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_GEOMETRY: {
    //             double length = attrs.get<double>(OPENDRIVE_ATTR_LENGTH, myCurrentEdge.id.c_str(), ok);
    //             double s = attrs.get<double>(OPENDRIVE_ATTR_S, myCurrentEdge.id.c_str(), ok);
    //             double x = attrs.get<double>(OPENDRIVE_ATTR_X, myCurrentEdge.id.c_str(), ok);
    //             double y = attrs.get<double>(OPENDRIVE_ATTR_Y, myCurrentEdge.id.c_str(), ok);
    //             double hdg = attrs.get<double>(OPENDRIVE_ATTR_HDG, myCurrentEdge.id.c_str(), ok);
    //             myCurrentEdge.geometries.push_back(OpenDriveGeometry(length, s, x, y, hdg));
    //         }
    //         break;
    //         case OPENDRIVE_TAG_ELEVATION: {
    //             if (myCurrentJunctionID != "") {
    //                 // <elevationGrid> not yet supported
    //                 break;
    //             }
    //             double s = attrs.get<double>(OPENDRIVE_ATTR_S, myCurrentEdge.id.c_str(), ok);
    //             double a = attrs.get<double>(OPENDRIVE_ATTR_A, myCurrentEdge.id.c_str(), ok);
    //             double b = attrs.get<double>(OPENDRIVE_ATTR_B, myCurrentEdge.id.c_str(), ok);
    //             double c = attrs.get<double>(OPENDRIVE_ATTR_C, myCurrentEdge.id.c_str(), ok);
    //             double d = attrs.get<double>(OPENDRIVE_ATTR_D, myCurrentEdge.id.c_str(), ok);
    //             myCurrentEdge.elevations.push_back(OpenDriveElevation(s, a, b, c, d));
    //         }
    //         break;
    //         case OPENDRIVE_TAG_LINE: {
    //             if (myElementStack.size() > 0 && myElementStack.back() == OPENDRIVE_TAG_GEOMETRY) {
    //                 std::vector<double> vals;
    //                 addGeometryShape(OPENDRIVE_GT_LINE, vals);
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_SPIRAL: {
    //             std::vector<double> vals;
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_CURVSTART, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_CURVEND, myCurrentEdge.id.c_str(), ok));
    //             addGeometryShape(OPENDRIVE_GT_SPIRAL, vals);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_ARC: {
    //             std::vector<double> vals;
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_CURVATURE, myCurrentEdge.id.c_str(), ok));
    //             addGeometryShape(OPENDRIVE_GT_ARC, vals);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_POLY3: {
    //             std::vector<double> vals;
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_A, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_B, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_C, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_D, myCurrentEdge.id.c_str(), ok));
    //             addGeometryShape(OPENDRIVE_GT_POLY3, vals);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_PARAMPOLY3: {
    //             std::vector<double> vals;
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_AU, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_BU, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_CU, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_DU, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_AV, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_BV, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_CV, myCurrentEdge.id.c_str(), ok));
    //             vals.push_back(attrs.get<double>(OPENDRIVE_ATTR_DV, myCurrentEdge.id.c_str(), ok));
    //             const std::string pRange = attrs.getOpt<std::string>(OPENDRIVE_ATTR_PRANGE, myCurrentEdge.id.c_str(), ok, "normalized", false);
    //             if (pRange == "normalized") {
    //                 vals.push_back(1.0);
    //             } else if (pRange == "arcLength") {
    //                 vals.push_back(-1.0);
    //             } else {
    //                 WRITE_WARNINGF(TL("Ignoring invalid pRange value '%' for road '%'."), pRange, myCurrentEdge.id);
    //                 vals.push_back(1.0);
    //             }
    //             addGeometryShape(OPENDRIVE_GT_PARAMPOLY3, vals);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_LANESECTION: {
    //             double s = attrs.get<double>(OPENDRIVE_ATTR_S, myCurrentEdge.id.c_str(), ok);
    //             if (myCurrentEdge.laneSections.size() > 0) {
    //                 myCurrentEdge.laneSections.back().length = s - myCurrentEdge.laneSections.back().s;
    //             }
    //             myCurrentEdge.laneSections.push_back(OpenDriveLaneSection(s));
    //
    //             // possibly updated by the next laneSection
    //             myCurrentEdge.laneSections.back().length = myCurrentEdge.length - s;
    //         }
    //         break;
    //         case OPENDRIVE_TAG_LANEOFFSET: {
    //             double s = attrs.get<double>(OPENDRIVE_ATTR_S, myCurrentEdge.id.c_str(), ok);
    //             double a = attrs.get<double>(OPENDRIVE_ATTR_A, myCurrentEdge.id.c_str(), ok);
    //             double b = attrs.get<double>(OPENDRIVE_ATTR_B, myCurrentEdge.id.c_str(), ok);
    //             double c = attrs.get<double>(OPENDRIVE_ATTR_C, myCurrentEdge.id.c_str(), ok);
    //             double d = attrs.get<double>(OPENDRIVE_ATTR_D, myCurrentEdge.id.c_str(), ok);
    //             myCurrentEdge.offsets.push_back(OpenDriveLaneOffset(s, a, b, c, d));
    //         }
    //         break;
    //         case OPENDRIVE_TAG_LEFT:
    //             myCurrentLaneDirection = OPENDRIVE_TAG_LEFT;
    //             break;
    //         case OPENDRIVE_TAG_CENTER:
    //             myCurrentLaneDirection = OPENDRIVE_TAG_CENTER;
    //             break;
    //         case OPENDRIVE_TAG_RIGHT:
    //             myCurrentLaneDirection = OPENDRIVE_TAG_RIGHT;
    //             break;
    //         case OPENDRIVE_TAG_LANE: {
    //             std::string type = attrs.get<std::string>(OPENDRIVE_ATTR_TYPE, myCurrentEdge.id.c_str(), ok);
    //             int id = attrs.get<int>(OPENDRIVE_ATTR_ID, myCurrentEdge.id.c_str(), ok);
    //             std::string level = attrs.hasAttribute(OPENDRIVE_ATTR_LEVEL)
    //                                 ? attrs.get<std::string>(OPENDRIVE_ATTR_LEVEL, myCurrentEdge.id.c_str(), ok)
    //                                 : "";
    //             OpenDriveLaneSection& ls = myCurrentEdge.laneSections.back();
    //             ls.lanesByDir[myCurrentLaneDirection].push_back(OpenDriveLane(id, level, type));
    //         }
    //         break;
    //         case OPENDRIVE_TAG_SIGNAL: {
    //             std::string id = attrs.get<std::string>(OPENDRIVE_ATTR_ID, myCurrentEdge.id.c_str(), ok);
    //             std::string type = attrs.get<std::string>(OPENDRIVE_ATTR_TYPE, myCurrentEdge.id.c_str(), ok);
    //             std::string name = attrs.getOpt<std::string>(OPENDRIVE_ATTR_NAME, myCurrentEdge.id.c_str(), ok, "", false);
    //             const std::string orientation = attrs.get<std::string>(OPENDRIVE_ATTR_ORIENTATION, id.c_str(), ok);
    //             int orientationCode = orientation == "-" ? -1 : orientation == "+" ? 1 : 0;
    //             double s = attrs.get<double>(OPENDRIVE_ATTR_S, myCurrentEdge.id.c_str(), ok);
    //             bool dynamic = attrs.get<std::string>(OPENDRIVE_ATTR_DYNAMIC, myCurrentEdge.id.c_str(), ok) == "no" ? false : true;
    //             OpenDriveSignal signal = OpenDriveSignal(id, type, name, orientationCode, dynamic, s);
    //             myCurrentEdge.signals.push_back(signal);
    //             mySignals[id] = signal;
    //         }
    //         break;
    //         case OPENDRIVE_TAG_SIGNALREFERENCE: {
    //             std::string id = attrs.get<std::string>(OPENDRIVE_ATTR_ID, myCurrentEdge.id.c_str(), ok);
    //             const std::string orientation = attrs.get<std::string>(OPENDRIVE_ATTR_ORIENTATION, id.c_str(), ok);
    //             int orientationCode = orientation == "-" ? -1 : orientation == "+" ? 1 : 0;
    //             double s = attrs.get<double>(OPENDRIVE_ATTR_S, myCurrentEdge.id.c_str(), ok);
    //             OpenDriveSignal signal = OpenDriveSignal(id, "", "", orientationCode, false, s);
    //             myCurrentEdge.signals.push_back(signal);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_CONTROLLER: {
    //             std::string id = attrs.get<std::string>(OPENDRIVE_ATTR_ID, nullptr, ok);
    //             std::string name = attrs.getOpt<std::string>(OPENDRIVE_ATTR_NAME, nullptr, ok, "", false);
    //             myCurrentController = OpenDriveController(id, name);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_CONTROL: {
    //             std::string signalID = attrs.get<std::string>(OPENDRIVE_ATTR_SIGNALID, myCurrentController.id.c_str(), ok);
    //             myCurrentController.signalIDs.push_back(signalID);
    //             if (mySignals.find(signalID) != mySignals.end()) {
    //                 mySignals[signalID].controller = myCurrentController.id;
    //             } else {
    //                 WRITE_WARNINGF(TL("Ignoring missing signal '%' in controller '%'."), signalID, myCurrentController.id);
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_VALIDITY: {
    //             int fromLane = attrs.get<int>(OPENDRIVE_ATTR_FROMLANE, myCurrentEdge.id.c_str(), ok);
    //             int toLane = attrs.get<int>(OPENDRIVE_ATTR_TOLANE, myCurrentEdge.id.c_str(), ok);
    //             if (myElementStack.size() >= 1 && (myElementStack.back() == OPENDRIVE_TAG_SIGNAL
    //                                                || myElementStack.back() == OPENDRIVE_TAG_SIGNALREFERENCE)) {
    //                 myCurrentEdge.signals.back().minLane = fromLane;
    //                 myCurrentEdge.signals.back().maxLane = toLane;
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_PRIORITY: {
    //             if (myElementStack.size() >= 2
    //                     && myElementStack.back() == OPENDRIVE_TAG_SEMANTICS
    //                     && myElementStack[myElementStack.size() - 2] == OPENDRIVE_TAG_SIGNAL) {
    //                 auto& signal = myCurrentEdge.signals.back();
    //                 signal.priority = attrs.get<std::string>(OPENDRIVE_ATTR_TYPE, signal.id.c_str(), ok);
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_JUNCTION:
    //             myCurrentJunctionID = attrs.get<std::string>(OPENDRIVE_ATTR_ID, myCurrentJunctionID.c_str(), ok);
    //             break;
    //         case OPENDRIVE_TAG_CONNECTION: {
    //             std::string id = attrs.get<std::string>(OPENDRIVE_ATTR_ID, myCurrentJunctionID.c_str(), ok);
    //             myCurrentIncomingRoad = attrs.get<std::string>(OPENDRIVE_ATTR_INCOMINGROAD, myCurrentJunctionID.c_str(), ok);
    //             myCurrentConnectingRoad = attrs.get<std::string>(OPENDRIVE_ATTR_CONNECTINGROAD, myCurrentJunctionID.c_str(), ok);
    //             std::string cp = attrs.get<std::string>(OPENDRIVE_ATTR_CONTACTPOINT, myCurrentJunctionID.c_str(), ok);
    //             myCurrentContactPoint = cp == "start" ? OPENDRIVE_CP_START : OPENDRIVE_CP_END;
    //             myConnectionWasEmpty = true;
    //         }
    //         break;
    //         case OPENDRIVE_TAG_LANELINK: {
    //             int from = attrs.get<int>(OPENDRIVE_ATTR_FROM, myCurrentJunctionID.c_str(), ok);
    //             int to = attrs.get<int>(OPENDRIVE_ATTR_TO, myCurrentJunctionID.c_str(), ok);
    //             Connection c;
    //             c.fromEdge = myCurrentIncomingRoad;
    //             c.toEdge = myCurrentConnectingRoad;
    //             c.fromLane = from;
    //             c.toLane = to;
    //             c.fromCP = OPENDRIVE_CP_END;
    //             c.toCP = myCurrentContactPoint;
    //             c.all = false;
    //             if (myEdges.find(c.fromEdge) == myEdges.end()) {
    //                 WRITE_ERRORF(TL("In laneLink-element: incoming road '%' is not known."), c.fromEdge);
    //             } else {
    //                 OpenDriveEdge* e = myEdges.find(c.fromEdge)->second;
    //                 e->connections.insert(c);
    //                 myConnectionWasEmpty = false;
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_WIDTH: {
    //             if (myElementStack.size() >= 2 && myElementStack[myElementStack.size() - 1] == OPENDRIVE_TAG_LANE) {
    //                 const double s = attrs.get<double>(OPENDRIVE_ATTR_SOFFSET, myCurrentEdge.id.c_str(), ok);
    //                 const double a = attrs.get<double>(OPENDRIVE_ATTR_A, myCurrentEdge.id.c_str(), ok);
    //                 const double b = attrs.get<double>(OPENDRIVE_ATTR_B, myCurrentEdge.id.c_str(), ok);
    //                 const double c = attrs.get<double>(OPENDRIVE_ATTR_C, myCurrentEdge.id.c_str(), ok);
    //                 const double d = attrs.get<double>(OPENDRIVE_ATTR_D, myCurrentEdge.id.c_str(), ok);
    //                 OpenDriveLane& l = myCurrentEdge.laneSections.back().lanesByDir[myCurrentLaneDirection].back();
    //                 l.width = MAX2(l.width, a);
    //                 l.widthData.push_back(OpenDriveWidth(s, a, b, c, d));
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (DEBUG_COND(&myCurrentEdge)) {
    //                     std::cout << " road=" << myCurrentEdge.id
    //                               << std::setprecision(gPrecision)
    //                               << " junction=" << myCurrentEdge.junction
    //                               << " section=" << myCurrentEdge.laneSections.size() - 1
    //                               << " dir=" << myCurrentLaneDirection << " lane=" << l.id
    //                               << " type=" << l.type
    //                               << " width=" << l.width
    //                               << " a=" << a
    //                               << " b=" << b
    //                               << " c=" << c
    //                               << " d=" << d
    //                               << " s=" << s
    //                               << " entries=" << l.widthData.size()
    //                               << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_ACCESS: {
    //             if (myElementStack.size() >= 2 && myElementStack[myElementStack.size() - 1] == OPENDRIVE_TAG_LANE) {
    //                 const double pos = attrs.get<double>(OPENDRIVE_ATTR_SOFFSET, myCurrentEdge.id.c_str(), ok);
    //                 std::string rule = attrs.getOpt<std::string>(OPENDRIVE_ATTR_RULE, nullptr, ok, "allow", false); // OpenDRIVE 1.4 without rule value
    //                 std::string vClass = attrs.get<std::string>(OPENDRIVE_ATTR_RESTRICTION, myCurrentEdge.id.c_str(), ok);
    //
    //                 std::vector < std::pair<double, LaneAttributeChange >>& attributeChanges = myCurrentEdge.laneSections.back().lanesByDir[myCurrentLaneDirection].back().attributeChanges;
    //                 std::vector<std::pair<double, LaneAttributeChange> >::iterator i = std::find_if(attributeChanges.begin(), attributeChanges.end(), same_position_finder(pos));
    //                 if (i != attributeChanges.end()) {
    //                     if (rule == "allow") {
    //                         (*i).second.allowed.push_back(vClass);
    //                     } else if (rule == "deny") {
    //                         (*i).second.denied.push_back(vClass);
    //                     }
    //                 } else {
    //                     LaneAttributeChange lac = LaneAttributeChange(0);
    //                     if (rule == "allow") {
    //                         lac.allowed.push_back(vClass);
    //                     } else if (rule == "deny") {
    //                         lac.denied.push_back(vClass);
    //                     }
    //                     attributeChanges.push_back(std::make_pair(pos, lac));
    //                 }
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_SPEED: {
    //             if (myElementStack.size() >= 2 && myElementStack[myElementStack.size() - 1] == OPENDRIVE_TAG_LANE) {
    //                 double speed = attrs.get<double>(OPENDRIVE_ATTR_MAX, myCurrentEdge.id.c_str(), ok);
    //                 double pos = attrs.get<double>(OPENDRIVE_ATTR_SOFFSET, myCurrentEdge.id.c_str(), ok);
    //                 // required for xodr v1.4
    //                 const std::string unit = attrs.getOpt<std::string>(OPENDRIVE_ATTR_UNIT, myCurrentEdge.id.c_str(), ok, "", false);
    //                 // now convert the speed to reasonable default SI [m/s]
    //                 if (!unit.empty()) {
    //                     // something to be done at all ?
    //                     if (unit == "km/h") {
    //                         speed /= 3.6;
    //                     }
    //                     if (unit == "mph") {
    //                         speed *= 1.609344 / 3.6;
    //                     }
    //                     // IGNORING unknown units.
    //                 }
    //                 std::vector < std::pair<double, LaneAttributeChange >>& attributeChanges = myCurrentEdge.laneSections.back().lanesByDir[myCurrentLaneDirection].back().attributeChanges;
    //                 std::vector<std::pair<double, LaneAttributeChange> >::iterator i = std::find_if(attributeChanges.begin(), attributeChanges.end(), same_position_finder(pos));
    //                 if (i != attributeChanges.end()) {
    //                     (*i).second.speed = speed;
    //                 } else {
    //                     LaneAttributeChange lac = LaneAttributeChange(speed);
    //                     attributeChanges.push_back(std::make_pair(pos, lac));
    //                 }
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_OBJECT: {
    //             if (!attrs.hasAttribute(OPENDRIVE_ATTR_ID)) {
    //                 WRITE_WARNINGF(TL("Ignoring object without id at edge '%'."), toString(myCurrentEdge.id));
    //                 break;
    //             }
    //             OpenDriveObject o;
    //             o.id = attrs.get<std::string>(OPENDRIVE_ATTR_ID, 0, ok);
    //             o.type = attrs.getOpt<std::string>(OPENDRIVE_ATTR_TYPE, o.id.c_str(), ok, "", false);
    //             o.name = attrs.getOpt<std::string>(OPENDRIVE_ATTR_NAME, o.id.c_str(), ok, "", false);
    //             o.s = attrs.get<double>(OPENDRIVE_ATTR_S, o.id.c_str(), ok);
    //             o.t = attrs.get<double>(OPENDRIVE_ATTR_T, o.id.c_str(), ok);
    //             o.width = attrs.getOpt<double>(OPENDRIVE_ATTR_WIDTH, o.id.c_str(), ok, -1);
    //             o.length = attrs.getOpt<double>(OPENDRIVE_ATTR_LENGTH, o.id.c_str(), ok, -1);
    //             o.radius = attrs.getOpt<double>(OPENDRIVE_ATTR_RADIUS, o.id.c_str(), ok, -1);
    //             o.hdg = attrs.getOpt<double>(OPENDRIVE_ATTR_HDG, o.id.c_str(), ok, 0);
    //             myCurrentEdge.objects.push_back(o);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_REPEAT: {
    //             if (myCurrentEdge.objects.empty()) {
    //                 WRITE_ERRORF(TL("Repeat without object at edge '%'."), toString(myCurrentEdge.id));
    //                 ok = false;
    //             } else {
    //                 OpenDriveObject o = myCurrentEdge.objects.back();
    //                 const std::string baseID = o.id;
    //                 double dist = attrs.get<double>(OPENDRIVE_ATTR_DISTANCE, o.id.c_str(), ok);
    //                 if (dist == 0) {
    //                     // continuous feature. Split into parts (XXX exmport as a single polygon #5235)
    //                     dist = OptionsCont::getOptions().getFloat("opendrive.curve-resolution");
    //                 }
    //
    //                 myCurrentEdge.objects.pop_back();
    //                 const double length = attrs.get<double>(OPENDRIVE_ATTR_LENGTH, o.id.c_str(), ok);
    //                 o.s = attrs.getOpt<double>(OPENDRIVE_ATTR_S, o.id.c_str(), ok, o.s);
    //                 double wStart = attrs.getOpt<double>(OPENDRIVE_ATTR_WIDTHSTART, o.id.c_str(), ok, o.width);
    //                 double wEnd = attrs.getOpt<double>(OPENDRIVE_ATTR_WIDTHEND, o.id.c_str(), ok, o.width);
    //                 double tStart = attrs.getOpt<double>(OPENDRIVE_ATTR_TSTART, o.id.c_str(), ok, o.t);
    //                 double tEnd = attrs.getOpt<double>(OPENDRIVE_ATTR_TEND, o.id.c_str(), ok, o.t);
    //                 int index = 0;
    //                 for (double x = 0; x <= length + NUMERICAL_EPS; x += dist) {
    //                     o.id = baseID + "#" + toString(index++);
    //                     const double a = x / length;
    //                     o.width = wStart * (1 - a) + wEnd * a;
    //                     o.t = tStart * (1 - a) + tEnd * a;
    //                     myCurrentEdge.objects.push_back(o);
    //                     o.s += dist;
    //                 }
    //             }
    //         }
    //         break;
    //         case OPENDRIVE_TAG_INCLUDE: {
    //             std::string includedFile = attrs.get<std::string>(OPENDRIVE_ATTR_FILE, 0, ok);
    //             if (!FileHelpers::isAbsolute(includedFile)) {
    //                 includedFile = FileHelpers::getConfigurationRelative(getFileName(), includedFile);
    //             }
    //             PROGRESS_BEGIN_MESSAGE("Parsing included opendrive from '" + includedFile + "'");
    //             myUseCurrentNode = true;
    //             XMLSubSys::runParser(*this, includedFile);
    //             PROGRESS_DONE_MESSAGE();
    //         }
    //         break;
    //         default:
    //             break;
    //     }
    //     myElementStack.push_back(element);
    // }
    /// C++ error propagation: `runParser` for included files may fail, so this returns `Result`
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        use NIImporter_OpenDrive_OpenDriveXMLAttr as A;
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        if self.myUseCurrentNode {
            // skip the parent node repeated in the included file
            self.myUseCurrentNode = false;
            self.myElementStack.push(element);
            return Ok(());
        }
        let mut ok = true;
        match open_drive_tag_from_i32(element) {
            T::OPENDRIVE_TAG_HEADER => {
                let majorVersion: i32 = attrs.get(A::OPENDRIVE_ATTR_REVMAJOR as i32, "", &mut ok, true);
                let minorVersion: i32 = attrs.get(A::OPENDRIVE_ATTR_REVMINOR as i32, "", &mut ok, true);
                if majorVersion == 1 && minorVersion > 4 {
                    // disable flags only used for old 1.4 standard
                    MY_IGNORE_MISPLACED_SIGNALS.with(|c| c.set(false));
                }
            }
            T::OPENDRIVE_TAG_OFFSET => {
                let x: f64 = attrs.get(A::OPENDRIVE_ATTR_X as i32, "offset", &mut ok, true);
                let y: f64 = attrs.get(A::OPENDRIVE_ATTR_Y as i32, "offset", &mut ok, true);
                let z: f64 = attrs.get(A::OPENDRIVE_ATTR_Z as i32, "offset", &mut ok, true);
                self.myOffset.set3(-x, -y, -z);
                if GeoConvHelper::get_num_loaded() != 0 {
                    GeoConvHelper::with_loaded(|g| g.move_converted_by(-x, -y));
                }
            }
            T::OPENDRIVE_TAG_ROAD => {
                let id: String = attrs.get(A::OPENDRIVE_ATTR_ID as i32, "", &mut ok, true);
                let streetName: String = attrs.get_opt(A::OPENDRIVE_ATTR_NAME as i32, "", &mut ok, String::new(), false);
                let junction: String = attrs.get(A::OPENDRIVE_ATTR_JUNCTION as i32, &id, &mut ok, true);
                let length: f64 = attrs.get(A::OPENDRIVE_ATTR_LENGTH as i32, &id, &mut ok, true);
                self.myCurrentEdge = NIImporter_OpenDrive_OpenDriveEdge::new(&id, &streetName, &junction, length);
            }
            T::OPENDRIVE_TAG_PREDECESSOR => {
                if self.myElementStack.len() >= 2 && stack_back(&self.myElementStack, 1) == Some(T::OPENDRIVE_TAG_ROAD as i32) {
                    let elementType: String = attrs.get(A::OPENDRIVE_ATTR_ELEMENTTYPE as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let elementID: String = attrs.get(A::OPENDRIVE_ATTR_ELEMENTID as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let contactPoint: String = if attrs.has_attribute(A::OPENDRIVE_ATTR_CONTACTPOINT as i32) {
                        attrs.get(A::OPENDRIVE_ATTR_CONTACTPOINT as i32, &self.myCurrentEdge.id, &mut ok, true)
                    } else {
                        "end".to_string()
                    };
                    self.add_link(NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_PREDECESSOR, &elementType, &elementID, &contactPoint);
                }
                if self.myElementStack.len() >= 2 && stack_back(&self.myElementStack, 1) == Some(T::OPENDRIVE_TAG_LANE as i32) {
                    let no: i32 = attrs.get(A::OPENDRIVE_ATTR_ID as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let dir = self.myCurrentLaneDirection;
                    let l = self.myCurrentEdge.laneSections.last_mut().unwrap().lanesByDir.entry(dir).or_default().last_mut().unwrap();
                    l.predecessor = no;
                }
            }
            T::OPENDRIVE_TAG_SUCCESSOR => {
                if self.myElementStack.len() >= 2 && stack_back(&self.myElementStack, 1) == Some(T::OPENDRIVE_TAG_ROAD as i32) {
                    let elementType: String = attrs.get(A::OPENDRIVE_ATTR_ELEMENTTYPE as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let elementID: String = attrs.get(A::OPENDRIVE_ATTR_ELEMENTID as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let contactPoint: String = if attrs.has_attribute(A::OPENDRIVE_ATTR_CONTACTPOINT as i32) {
                        attrs.get(A::OPENDRIVE_ATTR_CONTACTPOINT as i32, &self.myCurrentEdge.id, &mut ok, true)
                    } else {
                        "start".to_string()
                    };
                    self.add_link(NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR, &elementType, &elementID, &contactPoint);
                }
                if self.myElementStack.len() >= 2 && stack_back(&self.myElementStack, 1) == Some(T::OPENDRIVE_TAG_LANE as i32) {
                    let no: i32 = attrs.get(A::OPENDRIVE_ATTR_ID as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let dir = self.myCurrentLaneDirection;
                    let l = self.myCurrentEdge.laneSections.last_mut().unwrap().lanesByDir.entry(dir).or_default().last_mut().unwrap();
                    l.successor = no;
                }
            }
            T::OPENDRIVE_TAG_GEOMETRY => {
                let length: f64 = attrs.get(A::OPENDRIVE_ATTR_LENGTH as i32, &self.myCurrentEdge.id, &mut ok, true);
                let s: f64 = attrs.get(A::OPENDRIVE_ATTR_S as i32, &self.myCurrentEdge.id, &mut ok, true);
                let x: f64 = attrs.get(A::OPENDRIVE_ATTR_X as i32, &self.myCurrentEdge.id, &mut ok, true);
                let y: f64 = attrs.get(A::OPENDRIVE_ATTR_Y as i32, &self.myCurrentEdge.id, &mut ok, true);
                let hdg: f64 = attrs.get(A::OPENDRIVE_ATTR_HDG as i32, &self.myCurrentEdge.id, &mut ok, true);
                self.myCurrentEdge.geometries.push(NIImporter_OpenDrive_OpenDriveGeometry::new(length, s, x, y, hdg));
            }
            T::OPENDRIVE_TAG_ELEVATION => {
                // <elevationGrid> not yet supported
                if self.myCurrentJunctionID == "" {
                    let s: f64 = attrs.get(A::OPENDRIVE_ATTR_S as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let a: f64 = attrs.get(A::OPENDRIVE_ATTR_A as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let b: f64 = attrs.get(A::OPENDRIVE_ATTR_B as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let c: f64 = attrs.get(A::OPENDRIVE_ATTR_C as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let d: f64 = attrs.get(A::OPENDRIVE_ATTR_D as i32, &self.myCurrentEdge.id, &mut ok, true);
                    self.myCurrentEdge.elevations.push(NIImporter_OpenDrive_OpenDriveElevation::new(s, a, b, c, d));
                }
            }
            T::OPENDRIVE_TAG_LINE => {
                if self.myElementStack.len() > 0 && stack_back(&self.myElementStack, 0) == Some(T::OPENDRIVE_TAG_GEOMETRY as i32) {
                    let vals: Vec<f64> = Vec::new();
                    self.add_geometry_shape(NIImporter_OpenDrive_GeometryType::OPENDRIVE_GT_LINE, &vals)?;
                }
            }
            T::OPENDRIVE_TAG_SPIRAL => {
                let mut vals: Vec<f64> = Vec::new();
                vals.push(attrs.get(A::OPENDRIVE_ATTR_CURVSTART as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_CURVEND as i32, &self.myCurrentEdge.id, &mut ok, true));
                self.add_geometry_shape(NIImporter_OpenDrive_GeometryType::OPENDRIVE_GT_SPIRAL, &vals)?;
            }
            T::OPENDRIVE_TAG_ARC => {
                let mut vals: Vec<f64> = Vec::new();
                vals.push(attrs.get(A::OPENDRIVE_ATTR_CURVATURE as i32, &self.myCurrentEdge.id, &mut ok, true));
                self.add_geometry_shape(NIImporter_OpenDrive_GeometryType::OPENDRIVE_GT_ARC, &vals)?;
            }
            T::OPENDRIVE_TAG_POLY3 => {
                let mut vals: Vec<f64> = Vec::new();
                vals.push(attrs.get(A::OPENDRIVE_ATTR_A as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_B as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_C as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_D as i32, &self.myCurrentEdge.id, &mut ok, true));
                self.add_geometry_shape(NIImporter_OpenDrive_GeometryType::OPENDRIVE_GT_POLY3, &vals)?;
            }
            T::OPENDRIVE_TAG_PARAMPOLY3 => {
                let mut vals: Vec<f64> = Vec::new();
                vals.push(attrs.get(A::OPENDRIVE_ATTR_AU as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_BU as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_CU as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_DU as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_AV as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_BV as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_CV as i32, &self.myCurrentEdge.id, &mut ok, true));
                vals.push(attrs.get(A::OPENDRIVE_ATTR_DV as i32, &self.myCurrentEdge.id, &mut ok, true));
                let pRange: String = attrs.get_opt(A::OPENDRIVE_ATTR_PRANGE as i32, &self.myCurrentEdge.id, &mut ok, "normalized".to_string(), false);
                if pRange == "normalized" {
                    vals.push(1.0);
                } else if pRange == "arcLength" {
                    vals.push(-1.0);
                } else {
                    write_warningf!("Ignoring invalid pRange value '%' for road '%'.", pRange, self.myCurrentEdge.id);
                    vals.push(1.0);
                }
                self.add_geometry_shape(NIImporter_OpenDrive_GeometryType::OPENDRIVE_GT_PARAMPOLY3, &vals)?;
            }
            T::OPENDRIVE_TAG_LANESECTION => {
                let s: f64 = attrs.get(A::OPENDRIVE_ATTR_S as i32, &self.myCurrentEdge.id, &mut ok, true);
                if self.myCurrentEdge.laneSections.len() > 0 {
                    let last = self.myCurrentEdge.laneSections.last_mut().unwrap();
                    last.length = s - last.s;
                }
                self.myCurrentEdge.laneSections.push(NIImporter_OpenDrive_OpenDriveLaneSection::new(s));
                // possibly updated by the next laneSection
                let edgeLength = self.myCurrentEdge.length;
                self.myCurrentEdge.laneSections.last_mut().unwrap().length = edgeLength - s;
            }
            T::OPENDRIVE_TAG_LANEOFFSET => {
                let s: f64 = attrs.get(A::OPENDRIVE_ATTR_S as i32, &self.myCurrentEdge.id, &mut ok, true);
                let a: f64 = attrs.get(A::OPENDRIVE_ATTR_A as i32, &self.myCurrentEdge.id, &mut ok, true);
                let b: f64 = attrs.get(A::OPENDRIVE_ATTR_B as i32, &self.myCurrentEdge.id, &mut ok, true);
                let c: f64 = attrs.get(A::OPENDRIVE_ATTR_C as i32, &self.myCurrentEdge.id, &mut ok, true);
                let d: f64 = attrs.get(A::OPENDRIVE_ATTR_D as i32, &self.myCurrentEdge.id, &mut ok, true);
                self.myCurrentEdge.offsets.push(NIImporter_OpenDrive_OpenDriveLaneOffset::new(s, a, b, c, d));
            }
            T::OPENDRIVE_TAG_LEFT => {
                self.myCurrentLaneDirection = T::OPENDRIVE_TAG_LEFT;
            }
            T::OPENDRIVE_TAG_CENTER => {
                self.myCurrentLaneDirection = T::OPENDRIVE_TAG_CENTER;
            }
            T::OPENDRIVE_TAG_RIGHT => {
                self.myCurrentLaneDirection = T::OPENDRIVE_TAG_RIGHT;
            }
            T::OPENDRIVE_TAG_LANE => {
                let type_: String = attrs.get(A::OPENDRIVE_ATTR_TYPE as i32, &self.myCurrentEdge.id, &mut ok, true);
                let id: i32 = attrs.get(A::OPENDRIVE_ATTR_ID as i32, &self.myCurrentEdge.id, &mut ok, true);
                let level: String = if attrs.has_attribute(A::OPENDRIVE_ATTR_LEVEL as i32) {
                    attrs.get(A::OPENDRIVE_ATTR_LEVEL as i32, &self.myCurrentEdge.id, &mut ok, true)
                } else {
                    String::new()
                };
                let dir = self.myCurrentLaneDirection;
                let ls = self.myCurrentEdge.laneSections.last_mut().unwrap();
                ls.lanesByDir.entry(dir).or_default().push(NIImporter_OpenDrive_OpenDriveLane::new(id, &level, &type_));
            }
            T::OPENDRIVE_TAG_SIGNAL => {
                let id: String = attrs.get(A::OPENDRIVE_ATTR_ID as i32, &self.myCurrentEdge.id, &mut ok, true);
                let type_: String = attrs.get(A::OPENDRIVE_ATTR_TYPE as i32, &self.myCurrentEdge.id, &mut ok, true);
                let name: String = attrs.get_opt(A::OPENDRIVE_ATTR_NAME as i32, &self.myCurrentEdge.id, &mut ok, String::new(), false);
                let orientation: String = attrs.get(A::OPENDRIVE_ATTR_ORIENTATION as i32, &id, &mut ok, true);
                let orientationCode: i32 = if orientation == "-" { -1 } else if orientation == "+" { 1 } else { 0 };
                let s: f64 = attrs.get(A::OPENDRIVE_ATTR_S as i32, &self.myCurrentEdge.id, &mut ok, true);
                let dynamic: bool = attrs.get::<String>(A::OPENDRIVE_ATTR_DYNAMIC as i32, &self.myCurrentEdge.id, &mut ok, true) != "no";
                let signal = NIImporter_OpenDrive_OpenDriveSignal::new(&id, type_, name, orientationCode, dynamic, s);
                self.myCurrentEdge.signals.push(signal.clone());
                self.mySignals.insert(id, signal);
            }
            T::OPENDRIVE_TAG_SIGNALREFERENCE => {
                let id: String = attrs.get(A::OPENDRIVE_ATTR_ID as i32, &self.myCurrentEdge.id, &mut ok, true);
                let orientation: String = attrs.get(A::OPENDRIVE_ATTR_ORIENTATION as i32, &id, &mut ok, true);
                let orientationCode: i32 = if orientation == "-" { -1 } else if orientation == "+" { 1 } else { 0 };
                let s: f64 = attrs.get(A::OPENDRIVE_ATTR_S as i32, &self.myCurrentEdge.id, &mut ok, true);
                let signal = NIImporter_OpenDrive_OpenDriveSignal::new(&id, String::new(), String::new(), orientationCode, false, s);
                self.myCurrentEdge.signals.push(signal);
            }
            T::OPENDRIVE_TAG_CONTROLLER => {
                let id: String = attrs.get(A::OPENDRIVE_ATTR_ID as i32, "", &mut ok, true);
                let name: String = attrs.get_opt(A::OPENDRIVE_ATTR_NAME as i32, "", &mut ok, String::new(), false);
                self.myCurrentController = NIImporter_OpenDrive_OpenDriveController::new(&id, name);
            }
            T::OPENDRIVE_TAG_CONTROL => {
                let signalID: String = attrs.get(A::OPENDRIVE_ATTR_SIGNALID as i32, &self.myCurrentController.id, &mut ok, true);
                self.myCurrentController.signalIDs.push(signalID.clone());
                if let Some(sig) = self.mySignals.get_mut(&signalID) {
                    sig.controller = self.myCurrentController.id.clone();
                } else {
                    write_warningf!("Ignoring missing signal '%' in controller '%'.", signalID, self.myCurrentController.id);
                }
            }
            T::OPENDRIVE_TAG_VALIDITY => {
                let fromLane: i32 = attrs.get(A::OPENDRIVE_ATTR_FROMLANE as i32, &self.myCurrentEdge.id, &mut ok, true);
                let toLane: i32 = attrs.get(A::OPENDRIVE_ATTR_TOLANE as i32, &self.myCurrentEdge.id, &mut ok, true);
                if self.myElementStack.len() >= 1 && (stack_back(&self.myElementStack, 0) == Some(T::OPENDRIVE_TAG_SIGNAL as i32)
                    || stack_back(&self.myElementStack, 0) == Some(T::OPENDRIVE_TAG_SIGNALREFERENCE as i32)) {
                    let sig = self.myCurrentEdge.signals.last_mut().unwrap();
                    sig.minLane = fromLane;
                    sig.maxLane = toLane;
                }
            }
            T::OPENDRIVE_TAG_PRIORITY => {
                if self.myElementStack.len() >= 2
                    && stack_back(&self.myElementStack, 0) == Some(T::OPENDRIVE_TAG_SEMANTICS as i32)
                    && stack_back(&self.myElementStack, 1) == Some(T::OPENDRIVE_TAG_SIGNAL as i32) {
                    let signal = self.myCurrentEdge.signals.last_mut().unwrap();
                    let sigID = signal.id.clone();
                    signal.priority = attrs.get(A::OPENDRIVE_ATTR_TYPE as i32, &sigID, &mut ok, true);
                }
            }
            T::OPENDRIVE_TAG_JUNCTION => {
                self.myCurrentJunctionID = attrs.get(A::OPENDRIVE_ATTR_ID as i32, &self.myCurrentJunctionID, &mut ok, true);
            }
            T::OPENDRIVE_TAG_CONNECTION => {
                let _id: String = attrs.get(A::OPENDRIVE_ATTR_ID as i32, &self.myCurrentJunctionID, &mut ok, true);
                self.myCurrentIncomingRoad = attrs.get(A::OPENDRIVE_ATTR_INCOMINGROAD as i32, &self.myCurrentJunctionID, &mut ok, true);
                self.myCurrentConnectingRoad = attrs.get(A::OPENDRIVE_ATTR_CONNECTINGROAD as i32, &self.myCurrentJunctionID, &mut ok, true);
                let cp: String = attrs.get(A::OPENDRIVE_ATTR_CONTACTPOINT as i32, &self.myCurrentJunctionID, &mut ok, true);
                self.myCurrentContactPoint = if cp == "start" {
                    NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START
                } else {
                    NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END
                };
                self.myConnectionWasEmpty = true;
            }
            T::OPENDRIVE_TAG_LANELINK => {
                let from: i32 = attrs.get(A::OPENDRIVE_ATTR_FROM as i32, &self.myCurrentJunctionID, &mut ok, true);
                let to: i32 = attrs.get(A::OPENDRIVE_ATTR_TO as i32, &self.myCurrentJunctionID, &mut ok, true);
                let c = NIImporter_OpenDrive_Connection {
                    fromEdge: self.myCurrentIncomingRoad.clone(),
                    toEdge: self.myCurrentConnectingRoad.clone(),
                    fromLane: from,
                    toLane: to,
                    fromCP: NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END,
                    toCP: self.myCurrentContactPoint,
                    all: false,
                    // C++ leaves origID / origLane default-initialized
                    origID: String::new(),
                    origLane: 0,
                    shape: PositionVector::default(),
                };
                if !self.myEdges.m().contains_key(&c.fromEdge) {
                    write_errorf!("In laneLink-element: incoming road '%' is not known.", c.fromEdge);
                } else {
                    let e = self.myEdges.m().get(&c.fromEdge).unwrap();
                    e.m().connections.insert(c);
                    self.myConnectionWasEmpty = false;
                }
            }
            T::OPENDRIVE_TAG_WIDTH => {
                if self.myElementStack.len() >= 2 && stack_back(&self.myElementStack, 0) == Some(T::OPENDRIVE_TAG_LANE as i32) {
                    let s: f64 = attrs.get(A::OPENDRIVE_ATTR_SOFFSET as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let a: f64 = attrs.get(A::OPENDRIVE_ATTR_A as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let b: f64 = attrs.get(A::OPENDRIVE_ATTR_B as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let c: f64 = attrs.get(A::OPENDRIVE_ATTR_C as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let d: f64 = attrs.get(A::OPENDRIVE_ATTR_D as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let dir = self.myCurrentLaneDirection;
                    let l = self.myCurrentEdge.laneSections.last_mut().unwrap().lanesByDir.entry(dir).or_default().last_mut().unwrap();
                    // MAX2(l.width, a)
                    l.width = if l.width > a { l.width } else { a };
                    l.widthData.push(NIImporter_OpenDrive_OpenDriveWidth::new(s, a, b, c, d));
                }
            }
            T::OPENDRIVE_TAG_ACCESS => {
                if self.myElementStack.len() >= 2 && stack_back(&self.myElementStack, 0) == Some(T::OPENDRIVE_TAG_LANE as i32) {
                    let pos: f64 = attrs.get(A::OPENDRIVE_ATTR_SOFFSET as i32, &self.myCurrentEdge.id, &mut ok, true);
                    // OpenDRIVE 1.4 without rule value
                    let rule: String = attrs.get_opt(A::OPENDRIVE_ATTR_RULE as i32, "", &mut ok, "allow".to_string(), false);
                    let vClass: String = attrs.get(A::OPENDRIVE_ATTR_RESTRICTION as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let dir = self.myCurrentLaneDirection;
                    let attributeChanges = &mut self.myCurrentEdge.laneSections.last_mut().unwrap().lanesByDir.entry(dir).or_default().last_mut().unwrap().attributeChanges;
                    let mut finder = NIImporter_OpenDrive_same_position_finder::new(pos);
                    match attributeChanges.iter().position(|ps| finder.call_op(ps)) {
                        Some(i) => {
                            if rule == "allow" {
                                attributeChanges[i].1.allowed.push(vClass);
                            } else if rule == "deny" {
                                attributeChanges[i].1.denied.push(vClass);
                            }
                        }
                        None => {
                            let mut lac = NIImporter_OpenDrive_LaneAttributeChange::new(0.0);
                            if rule == "allow" {
                                lac.allowed.push(vClass);
                            } else if rule == "deny" {
                                lac.denied.push(vClass);
                            }
                            attributeChanges.push((pos, lac));
                        }
                    }
                }
            }
            T::OPENDRIVE_TAG_SPEED => {
                if self.myElementStack.len() >= 2 && stack_back(&self.myElementStack, 0) == Some(T::OPENDRIVE_TAG_LANE as i32) {
                    let mut speed: f64 = attrs.get(A::OPENDRIVE_ATTR_MAX as i32, &self.myCurrentEdge.id, &mut ok, true);
                    let pos: f64 = attrs.get(A::OPENDRIVE_ATTR_SOFFSET as i32, &self.myCurrentEdge.id, &mut ok, true);
                    // required for xodr v1.4
                    let unit: String = attrs.get_opt(A::OPENDRIVE_ATTR_UNIT as i32, &self.myCurrentEdge.id, &mut ok, String::new(), false);
                    // now convert the speed to reasonable default SI [m/s]
                    if !unit.is_empty() {
                        // something to be done at all ?
                        if unit == "km/h" {
                            speed /= 3.6;
                        }
                        if unit == "mph" {
                            speed *= 1.609344 / 3.6;
                        }
                        // IGNORING unknown units.
                    }
                    let dir = self.myCurrentLaneDirection;
                    let attributeChanges = &mut self.myCurrentEdge.laneSections.last_mut().unwrap().lanesByDir.entry(dir).or_default().last_mut().unwrap().attributeChanges;
                    let mut finder = NIImporter_OpenDrive_same_position_finder::new(pos);
                    match attributeChanges.iter().position(|ps| finder.call_op(ps)) {
                        Some(i) => {
                            attributeChanges[i].1.speed = speed;
                        }
                        None => {
                            attributeChanges.push((pos, NIImporter_OpenDrive_LaneAttributeChange::new(speed)));
                        }
                    }
                }
            }
            T::OPENDRIVE_TAG_OBJECT => {
                if !attrs.has_attribute(A::OPENDRIVE_ATTR_ID as i32) {
                    write_warningf!("Ignoring object without id at edge '%'.", self.myCurrentEdge.id);
                } else {
                    let mut o = NIImporter_OpenDrive_OpenDriveObject {
                        type_: String::new(),
                        name: String::new(),
                        id: String::new(),
                        s: 0.0,
                        t: 0.0,
                        zOffset: 0.0,
                        length: 0.0,
                        width: 0.0,
                        height: 0.0,
                        radius: 0.0,
                        hdg: 0.0,
                        pitch: 0.0,
                        roll: 0.0,
                    };
                    o.id = attrs.get(A::OPENDRIVE_ATTR_ID as i32, "", &mut ok, true);
                    o.type_ = attrs.get_opt(A::OPENDRIVE_ATTR_TYPE as i32, &o.id, &mut ok, String::new(), false);
                    o.name = attrs.get_opt(A::OPENDRIVE_ATTR_NAME as i32, &o.id, &mut ok, String::new(), false);
                    o.s = attrs.get(A::OPENDRIVE_ATTR_S as i32, &o.id, &mut ok, true);
                    o.t = attrs.get(A::OPENDRIVE_ATTR_T as i32, &o.id, &mut ok, true);
                    o.width = attrs.get_opt(A::OPENDRIVE_ATTR_WIDTH as i32, &o.id, &mut ok, -1.0, true);
                    o.length = attrs.get_opt(A::OPENDRIVE_ATTR_LENGTH as i32, &o.id, &mut ok, -1.0, true);
                    o.radius = attrs.get_opt(A::OPENDRIVE_ATTR_RADIUS as i32, &o.id, &mut ok, -1.0, true);
                    o.hdg = attrs.get_opt(A::OPENDRIVE_ATTR_HDG as i32, &o.id, &mut ok, 0.0, true);
                    self.myCurrentEdge.objects.push(o);
                }
            }
            T::OPENDRIVE_TAG_REPEAT => {
                if self.myCurrentEdge.objects.is_empty() {
                    write_errorf!("Repeat without object at edge '%'.", self.myCurrentEdge.id);
                    ok = false;
                } else {
                    let mut o = self.myCurrentEdge.objects.last().unwrap().clone();
                    let baseID = o.id.clone();
                    let mut dist: f64 = attrs.get(A::OPENDRIVE_ATTR_DISTANCE as i32, &o.id, &mut ok, true);
                    if dist == 0.0 {
                        // continuous feature. Split into parts (XXX exmport as a single polygon #5235)
                        dist = utils::options::options_cont::get_float("opendrive.curve-resolution");
                    }
                    self.myCurrentEdge.objects.pop();
                    let length: f64 = attrs.get(A::OPENDRIVE_ATTR_LENGTH as i32, &o.id, &mut ok, true);
                    o.s = attrs.get_opt(A::OPENDRIVE_ATTR_S as i32, &o.id, &mut ok, o.s, true);
                    let wStart: f64 = attrs.get_opt(A::OPENDRIVE_ATTR_WIDTHSTART as i32, &o.id, &mut ok, o.width, true);
                    let wEnd: f64 = attrs.get_opt(A::OPENDRIVE_ATTR_WIDTHEND as i32, &o.id, &mut ok, o.width, true);
                    let tStart: f64 = attrs.get_opt(A::OPENDRIVE_ATTR_TSTART as i32, &o.id, &mut ok, o.t, true);
                    let tEnd: f64 = attrs.get_opt(A::OPENDRIVE_ATTR_TEND as i32, &o.id, &mut ok, o.t, true);
                    let mut index: i32 = 0;
                    let mut x: f64 = 0.0;
                    while x <= length + utils::geom::NUMERICAL_EPS {
                        o.id = format!("{}#{}", baseID, index);
                        index += 1;
                        let a = x / length;
                        o.width = wStart * (1.0 - a) + wEnd * a;
                        o.t = tStart * (1.0 - a) + tEnd * a;
                        self.myCurrentEdge.objects.push(o.clone());
                        o.s += dist;
                        x += dist;
                    }
                }
            }
            T::OPENDRIVE_TAG_INCLUDE => {
                let mut includedFile: String = attrs.get(A::OPENDRIVE_ATTR_FILE as i32, "", &mut ok, true);
                if !utils::common::file_helpers::is_absolute(&includedFile) {
                    includedFile = utils::common::file_helpers::get_configuration_relative(self.get_file_name(), &includedFile);
                }
                progress_begin_message(&format!("Parsing included opendrive from '{}'", includedFile));
                self.myUseCurrentNode = true;
                utils::xml::xml_sub_sys::run_parser(self, &includedFile, false, false, false, true)?;
                progress_done_message();
            }
            _ => {}
        }
        self.myElementStack.push(element);
        Ok(())
    }

    /// C++: `void NIImporter_OpenDrive::myCharacters(int, const std::string &)` [virtual, protected]
    /// Callback method for characters to implement by derived classes
    // void
    // NIImporter_OpenDrive::myCharacters(int element, const std::string& cdata) {
    //     if (element == OPENDRIVE_TAG_GEOREFERENCE) {
    //         size_t i = cdata.find("+proj");
    //         if (i != std::string::npos) {
    //             const std::string proj = cdata.substr(i);
    //             if (proj != "") {
    //                 Boundary convBoundary;
    //                 Boundary origBoundary;
    //                 // XXX read values from the header
    //                 convBoundary.add(Position(0, 0));
    //                 origBoundary.add(Position(0, 0));
    //                 try {
    //                     GeoConvHelper::setLoaded(GeoConvHelper(proj, myOffset, origBoundary, convBoundary));
    //                 } catch (ProcessError& e) {
    //                     WRITE_ERRORF(TL("Could not set projection (%). This can be ignored with --ignore-errors."), std::string(e.what()));
    //                 }
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("geoReference format '%' currently not supported"), cdata);
    //         }
    //         needsCharacterData(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_characters(&mut self, element: i32, chars: &str) {
        if open_drive_tag_from_i32(element) == NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_GEOREFERENCE {
            let cdata = chars;
            if let Some(i) = cdata.find("+proj") {
                let proj = &cdata[i..];
                if proj != "" {
                    let mut convBoundary = Boundary::default();
                    let mut origBoundary = Boundary::default();
                    // XXX read values from the header
                    convBoundary.add_pos(&Position::new(0.0, 0.0));
                    origBoundary.add_pos(&Position::new(0.0, 0.0));
                    // shortcut: Rust GeoConvHelper::new is infallible, so the C++ ProcessError catch (WRITE_ERRORF "Could not set projection") is unreachable
                    GeoConvHelper::set_loaded(&GeoConvHelper::new(proj, self.myOffset, origBoundary, convBoundary, 1.0, 0.0, false, false));
                }
            } else {
                write_warningf!("geoReference format '%' currently not supported", cdata.to_string());
            }
            self.needs_character_data(false);
        }
    }

    /// C++: `void NIImporter_OpenDrive::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIImporter_OpenDrive::myEndElement(int element) {
    //     myElementStack.pop_back();
    //     switch (element) {
    //         case OPENDRIVE_TAG_JUNCTION:
    //             myCurrentJunctionID = "";
    //             break;
    //         case OPENDRIVE_TAG_ROAD:
    //             myEdges[myCurrentEdge.id] = new OpenDriveEdge(myCurrentEdge);
    //             break;
    //         case OPENDRIVE_TAG_CONNECTION:
    //             if (myConnectionWasEmpty) {
    //                 Connection c;
    //                 c.fromEdge = myCurrentIncomingRoad;
    //                 c.toEdge = myCurrentConnectingRoad;
    //                 c.fromLane = 0;
    //                 c.toLane = 0;
    //                 c.fromCP = OPENDRIVE_CP_END;
    //                 c.toCP = myCurrentContactPoint;
    //                 c.all = true;
    //                 if (myEdges.find(c.fromEdge) == myEdges.end()) {
    //                     WRITE_ERRORF(TL("In laneLink-element: incoming road '%' is not known."), c.fromEdge);
    //                 } else {
    //                     OpenDriveEdge* e = myEdges.find(c.fromEdge)->second;
    //                     e->connections.insert(c);
    //                 }
    //             }
    //             break;
    //         case OPENDRIVE_TAG_CONTROLLER: {
    //             myControllers.insert({ myCurrentController.id, myCurrentController });
    //         }
    //         break;
    //         case OPENDRIVE_TAG_LANESECTION: {
    //             myCurrentEdge.laneSections.back().buildLaneMapping(myTypeContainer);
    //         }
    //         break;
    //         case OPENDRIVE_TAG_SIGNAL:
    //         case OPENDRIVE_TAG_SIGNALREFERENCE: {
    //             if (NIImporter_OpenDrive::myIgnoreMisplacedSignals) {
    //                 int intType = -1;
    //                 try {
    //                     intType = StringUtils::toInt(myCurrentEdge.signals.back().type);
    //                 } catch (NumberFormatException&) {
    //                     break;
    //                 } catch (EmptyData&) {
    //                     break;
    //                 }
    //                 if (intType < 1000001 || (intType > 1000013 && intType != 1000020) || intType == 1000008) {
    //                     // not a traffic_light (Section 6.11)
    //                     break;
    //                 }
    //                 double s = myCurrentEdge.signals.back().s;
    //                 int minLane = myCurrentEdge.signals.back().minLane;
    //                 int maxLane = myCurrentEdge.signals.back().maxLane;
    //                 bool foundDrivingType = false;
    //                 for (OpenDriveLaneSection ls : myCurrentEdge.laneSections) {
    //                     if (ls.s <= s && ls.s + ls.length > s) {
    //                         if (myCurrentEdge.signals.back().orientation < 0) {
    //                             for (OpenDriveLane l : ls.lanesByDir[OPENDRIVE_TAG_LEFT]) {
    //                                 if ((minLane < 0 && l.id >= minLane && l.id <= maxLane) && l.type == "driving") {
    //                                     foundDrivingType = true;
    //                                 }
    //                             }
    //                         } else if (myCurrentEdge.signals.back().orientation > 0) { // 0 = center is never used for driving
    //                             for (OpenDriveLane l : ls.lanesByDir[OPENDRIVE_TAG_RIGHT]) {
    //                                 if ((minLane > 0 && l.id >= minLane && l.id <= maxLane) && l.type == "driving") {
    //                                     foundDrivingType = true;
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 if (!foundDrivingType) { // reject signal / signal reference if not on driving lane
    //                     myCurrentEdge.signals.pop_back();
    //                 }
    //             }
    //         }
    //         break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        self.myElementStack.pop();
        match open_drive_tag_from_i32(element) {
            T::OPENDRIVE_TAG_JUNCTION => {
                self.myCurrentJunctionID = String::new();
            }
            T::OPENDRIVE_TAG_ROAD => {
                // C++ `new OpenDriveEdge(myCurrentEdge)`: a heap copy owned by myEdges for the rest of the import
                let copy = Box::new(self.myCurrentEdge.clone());
                self.myEdges.m().insert(self.myCurrentEdge.id.clone(), Ptr::from_raw(Box::into_raw(copy)));
            }
            T::OPENDRIVE_TAG_CONNECTION => {
                if self.myConnectionWasEmpty {
                    let c = NIImporter_OpenDrive_Connection {
                        fromEdge: self.myCurrentIncomingRoad.clone(),
                        toEdge: self.myCurrentConnectingRoad.clone(),
                        fromLane: 0,
                        toLane: 0,
                        fromCP: NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END,
                        toCP: self.myCurrentContactPoint,
                        all: true,
                        // C++ leaves origID / origLane default-initialized
                        origID: String::new(),
                        origLane: 0,
                        shape: PositionVector::default(),
                    };
                    if !self.myEdges.m().contains_key(&c.fromEdge) {
                        write_errorf!("In laneLink-element: incoming road '%' is not known.", c.fromEdge);
                    } else {
                        let e = self.myEdges.m().get(&c.fromEdge).unwrap();
                        e.m().connections.insert(c);
                    }
                }
            }
            T::OPENDRIVE_TAG_CONTROLLER => {
                // C++ std::map::insert keeps an existing entry
                let c = self.myCurrentController.clone();
                self.myControllers.entry(c.id.clone()).or_insert(c);
            }
            T::OPENDRIVE_TAG_LANESECTION => {
                let tc = self.myTypeContainer;
                self.myCurrentEdge.laneSections.last_mut().unwrap().build_lane_mapping(tc.m());
            }
            T::OPENDRIVE_TAG_SIGNAL | T::OPENDRIVE_TAG_SIGNALREFERENCE => {
                if MY_IGNORE_MISPLACED_SIGNALS.with(|c| c.get()) {
                    'sig: {
                        let intType: i32 = match utils::common::string_utils::to_int(&self.myCurrentEdge.signals.last().unwrap().type_) {
                            Ok(v) => v,
                            // NumberFormatException / EmptyData
                            Err(_) => break 'sig,
                        };
                        if intType < 1000001 || (intType > 1000013 && intType != 1000020) || intType == 1000008 {
                            // not a traffic_light (Section 6.11)
                            break 'sig;
                        }
                        let back = self.myCurrentEdge.signals.last().unwrap();
                        let s: f64 = back.s;
                        let minLane: i32 = back.minLane;
                        let maxLane: i32 = back.maxLane;
                        let orientation: i32 = back.orientation;
                        let mut foundDrivingType = false;
                        for ls in self.myCurrentEdge.laneSections.iter() {
                            if ls.s <= s && ls.s + ls.length > s {
                                if orientation < 0 {
                                    if let Some(lanes) = ls.lanesByDir.get(&T::OPENDRIVE_TAG_LEFT) {
                                        for l in lanes {
                                            if (minLane < 0 && l.id >= minLane && l.id <= maxLane) && l.type_ == "driving" {
                                                foundDrivingType = true;
                                            }
                                        }
                                    }
                                } else if orientation > 0 {
                                    // 0 = center is never used for driving
                                    if let Some(lanes) = ls.lanesByDir.get(&T::OPENDRIVE_TAG_RIGHT) {
                                        for l in lanes {
                                            if (minLane > 0 && l.id >= minLane && l.id <= maxLane) && l.type_ == "driving" {
                                                foundDrivingType = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !foundDrivingType {
                            // reject signal / signal reference if not on driving lane
                            self.myCurrentEdge.signals.pop();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// C++: `std::map<std::string, OpenDriveSignal> & NIImporter_OpenDrive::getSignals()` [protected]
    // std::map<std::string, OpenDriveSignal>& getSignals() {
    //         return mySignals;
    //     }
    #[allow(unused_variables)]
    pub fn get_signals(&mut self) -> &mut BTreeMap<String, NIImporter_OpenDrive_OpenDriveSignal> {
        &mut self.mySignals
    }

    /// C++: `std::map<std::string, OpenDriveController> & NIImporter_OpenDrive::getControllers()` [protected]
    // std::map<std::string, OpenDriveController>& getControllers() {
    //         return myControllers;
    //     }
    #[allow(unused_variables)]
    pub fn get_controllers(&mut self) -> &mut BTreeMap<String, NIImporter_OpenDrive_OpenDriveController> {
        &mut self.myControllers
    }

    /// C++: `std::map<std::string, std::vector<std::string>> & NIImporter_OpenDrive::getJunctions2Controllers()` [protected]
    // std::map<std::string, std::vector<std::string>>& getJunctions2Controllers() {
    //         return myJunctions2Controllers;
    //     }
    #[allow(unused_variables)]
    pub fn get_junctions2_controllers(&mut self) -> &mut BTreeMap<String, Vec<String>> {
        &mut self.myJunctions2Controllers
    }

    /// C++: `const OpenDriveController & NIImporter_OpenDrive::getController(std::string)` [protected]
    // const OpenDriveController& getController(std::string signalID) {
    //         if (mySignals.find(signalID) != mySignals.end() && myControllers.find(mySignals[signalID].controller) != myControllers.end()) {
    //             return myControllers[mySignals[signalID].controller];
    //         }
    //         return myDummyController;
    //     }
    #[allow(unused_variables)]
    pub fn get_controller(&mut self, signalID: String) -> &NIImporter_OpenDrive_OpenDriveController {
        if self.mySignals.contains_key(&signalID) && self.myControllers.contains_key(&self.mySignals[&signalID].controller) {
            let ctrl = self.mySignals[&signalID].controller.clone();
            return &self.myControllers[&ctrl];
        }
        MY_DUMMY_CONTROLLER.get_or_init(NIImporter_OpenDrive_OpenDriveController::new_2)
    }

    /// C++: `int NIImporter_OpenDrive::getTLIndexForController(std::string)` [protected]
    // int getTLIndexForController(std::string controllerID) {
    //         // sort them by their id
    //         std::string junctionID = myControllers[controllerID].junction;
    //         std::vector<std::string> junctionControllers;
    //         for (auto& it : myControllers) {
    //             if (it.second.junction == junctionID) {
    //                 junctionControllers.push_back(it.first);
    //             }
    //         }
    //         std::sort(junctionControllers.begin(), junctionControllers.end());
    //         auto it = std::find(junctionControllers.begin(), junctionControllers.end(), controllerID);
    //         return (int)(it - junctionControllers.begin());
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_index_for_controller(&mut self, controllerID: String) -> i32 {
        let junctionID = self.myControllers.entry(controllerID.clone()).or_insert_with(NIImporter_OpenDrive_OpenDriveController::new_2).junction.clone();
        let mut junctionControllers: Vec<String> = Vec::new();
        for (k, v) in &self.myControllers {
            if v.junction == junctionID {
                junctionControllers.push(k.clone());
            }
        }
        // determinism: keys are unique, so sorting is total
        junctionControllers.sort();
        junctionControllers.iter().position(|c| *c == controllerID).unwrap_or(junctionControllers.len()) as i32
    }

    /// C++: `void NIImporter_OpenDrive::addLink(LinkType, const std::string &, const std::string &, const std::string &)` [private]
    // void
    // NIImporter_OpenDrive::addLink(LinkType lt, const std::string& elementType,
    //                               const std::string& elementID,
    //                               const std::string& contactPoint) {
    //     OpenDriveLink l(lt, elementID);
    //     // elementType
    //     if (elementType == "road") {
    //         l.elementType = OPENDRIVE_ET_ROAD;
    //     } else if (elementType == "junction") {
    //         l.elementType = OPENDRIVE_ET_JUNCTION;
    //     }
    //     // contact point
    //     if (contactPoint == "start") {
    //         l.contactPoint = OPENDRIVE_CP_START;
    //     } else if (contactPoint == "end") {
    //         l.contactPoint = OPENDRIVE_CP_END;
    //     }
    //     // add
    //     myCurrentEdge.links.push_back(l);
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, lt: NIImporter_OpenDrive_LinkType, elementType: &str, elementID: &str, contactPoint: &str) {
        let mut l = NIImporter_OpenDrive_OpenDriveLink::new(lt, elementID);
        // elementType
        if elementType == "road" {
            l.elementType = NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_ROAD;
        } else if elementType == "junction" {
            l.elementType = NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_JUNCTION;
        }
        // contact point
        if contactPoint == "start" {
            l.contactPoint = NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START;
        } else if contactPoint == "end" {
            l.contactPoint = NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END;
        }
        // add
        self.myCurrentEdge.links.push(l);
    }

    /// C++: `void NIImporter_OpenDrive::addGeometryShape(GeometryType, const std::vector<double> &)` [private]
    // void
    // NIImporter_OpenDrive::addGeometryShape(GeometryType type, const std::vector<double>& vals) {
    //     // checks
    //     if (myCurrentEdge.geometries.size() == 0) {
    //         throw ProcessError(TLF("Mismatching parenthesis in geometry definition for road '%'", myCurrentEdge.id));
    //     }
    //     OpenDriveGeometry& last = myCurrentEdge.geometries.back();
    //     if (last.type != OPENDRIVE_GT_UNKNOWN) {
    //         throw ProcessError(TLF("Double geometry information for road '%'", myCurrentEdge.id));
    //     }
    //     // set
    //     last.type = type;
    //     last.params = vals;
    // }
    #[allow(unused_variables)]
    pub fn add_geometry_shape(&mut self, type_: NIImporter_OpenDrive_GeometryType, vals: &Vec<f64>) -> Result<(), ProcessError> {
        // checks
        if self.myCurrentEdge.geometries.len() == 0 {
            return Err(ProcessError::new(tlf!("Mismatching parenthesis in geometry definition for road '%'", self.myCurrentEdge.id.clone())));
        }
        let last = self.myCurrentEdge.geometries.last_mut().unwrap();
        if last.type_ != NIImporter_OpenDrive_GeometryType::OPENDRIVE_GT_UNKNOWN {
            return Err(ProcessError::new(tlf!("Double geometry information for road '%'", self.myCurrentEdge.id.clone())));
        }
        // set
        last.type_ = type_;
        last.params = vals.clone();
        Ok(())
    }

    /// C++: `void NIImporter_OpenDrive::setEdgeLinks2(OpenDriveEdge &, const std::map<std::string, OpenDriveEdge *> &)` [static, private]
    // void
    // NIImporter_OpenDrive::setEdgeLinks2(OpenDriveEdge& e, const std::map<std::string, OpenDriveEdge*>& edges) {
    //     for (std::vector<OpenDriveLink>::iterator i = e.links.begin(); i != e.links.end(); ++i) {
    //         OpenDriveLink& l = *i;
    //         if (l.elementType != OPENDRIVE_ET_ROAD) {
    //             // we assume that links to nodes are later given as connections to edges
    //             continue;
    //         }
    //         // get the right direction of the connected edge
    //         std::string connectedEdge = l.elementID;
    //         std::string edgeID = e.id;
    //
    //         OpenDriveLaneSection& laneSection = l.linkType == OPENDRIVE_LT_SUCCESSOR ? e.laneSections.back() : e.laneSections[0];
    //         const std::map<int, int>& laneMap = laneSection.laneMap;
    // #ifdef DEBUG_CONNECTIONS
    //         if (DEBUG_COND(&e)) {
    //             std::cout << "edge=" << e.id << " eType=" << l.elementType << " lType=" << l.linkType  << " connectedEdge=" << connectedEdge << " laneSection=" << laneSection.s << " map:\n";
    //             std::cout << joinToString(laneMap, "\n", ":") << "\n";
    //         }
    // #endif
    //         if (laneSection.lanesByDir.find(OPENDRIVE_TAG_RIGHT) != laneSection.lanesByDir.end()) {
    //             const std::vector<OpenDriveLane>& lanes = laneSection.lanesByDir.find(OPENDRIVE_TAG_RIGHT)->second;
    //             for (std::vector<OpenDriveLane>::const_iterator j = lanes.begin(); j != lanes.end(); ++j) {
    //                 if (!myImportAllTypes && laneMap.find((*j).id) == laneMap.end()) {
    //                     continue;
    //                 }
    //                 Connection c; // @todo: give Connection a new name and a constructor
    //                 c.fromEdge = e.id;
    //                 c.fromLane = (*j).id;
    //                 c.fromCP = OPENDRIVE_CP_END;
    //                 c.toLane = l.linkType == OPENDRIVE_LT_SUCCESSOR ? (*j).successor : (*j).predecessor;
    //                 c.toEdge = connectedEdge;
    //                 c.toCP = l.contactPoint;
    //                 c.all = false;
    //                 if (l.linkType != OPENDRIVE_LT_SUCCESSOR) {
    //                     std::swap(c.fromEdge, c.toEdge);
    //                     std::swap(c.fromLane, c.toLane);
    //                     c.fromCP = c.toCP;
    //                     c.toCP = OPENDRIVE_CP_START;
    //                 }
    //                 if (edges.find(c.fromEdge) == edges.end()) {
    //                     WRITE_ERRORF(TL("While setting connections: incoming road '%' is not known."), c.fromEdge);
    //                 } else {
    //                     OpenDriveEdge* src = edges.find(c.fromEdge)->second;
    //                     src->connections.insert(c);
    // #ifdef DEBUG_CONNECTIONS
    //                     if (DEBUG_COND(src)) {
    //                         std::cout << "insertConRight from=" << src->id << "_" << c.fromLane << " to=" << c.toEdge << "_" << c.toLane << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //         if (laneSection.lanesByDir.find(OPENDRIVE_TAG_LEFT) != laneSection.lanesByDir.end()) {
    //             const std::vector<OpenDriveLane>& lanes = laneSection.lanesByDir.find(OPENDRIVE_TAG_LEFT)->second;
    //             for (std::vector<OpenDriveLane>::const_iterator j = lanes.begin(); j != lanes.end(); ++j) {
    //                 if (!myImportAllTypes && laneMap.find((*j).id) == laneMap.end()) {
    //                     continue;
    //                 }
    //                 Connection c;
    //                 c.toEdge = e.id;
    //                 c.toLane = (*j).id;
    //                 c.toCP = OPENDRIVE_CP_END;
    //                 c.fromLane = l.linkType == OPENDRIVE_LT_SUCCESSOR ? (*j).successor : (*j).predecessor;
    //                 c.fromEdge = connectedEdge;
    //                 c.fromCP = l.contactPoint;
    //                 c.all = false;
    //                 if (l.linkType != OPENDRIVE_LT_SUCCESSOR) {
    //                     std::swap(c.fromEdge, c.toEdge);
    //                     std::swap(c.fromLane, c.toLane);
    //                     c.fromCP = c.toCP;
    //                     c.toCP = OPENDRIVE_CP_START;
    //                 }
    //                 if (edges.find(c.fromEdge) == edges.end()) {
    //                     WRITE_ERRORF(TL("While setting connections: incoming road '%' is not known."), c.fromEdge);
    //                 } else {
    //                     OpenDriveEdge* src = edges.find(c.fromEdge)->second;
    //                     src->connections.insert(c);
    // #ifdef DEBUG_CONNECTIONS
    //                     if (DEBUG_COND(src)) {
    //                         std::cout << "insertConLeft from=" << src->id << "_" << c.fromLane << " to=" << c.toEdge << "_" << c.toLane << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_edge_links2(e: &mut NIImporter_OpenDrive_OpenDriveEdge, edges: &BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>>) {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        // The C++ inserts into `src->connections` inside the loops; src may be `e` itself, so the
        // connections are collected first and inserted in the same order afterwards (no reads of
        // `connections` happen in between, so the result is identical).
        let mut pending: Vec<NIImporter_OpenDrive_Connection> = Vec::new();
        for l in e.links.iter() {
            if l.elementType != NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_ROAD {
                // we assume that links to nodes are later given as connections to edges
                continue;
            }
            // get the right direction of the connected edge
            let connectedEdge: String = l.elementID.clone();
            let laneSection = if l.linkType == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR {
                e.laneSections.last().unwrap()
            } else {
                &e.laneSections[0]
            };
            let laneMap = &laneSection.laneMap;
            if let Some(lanes) = laneSection.lanesByDir.get(&T::OPENDRIVE_TAG_RIGHT) {
                for j in lanes {
                    if !MY_IMPORT_ALL_TYPES.with(|c| c.get()) && !laneMap.contains_key(&j.id) {
                        continue;
                    }
                    let mut c = NIImporter_OpenDrive_Connection {
                        fromEdge: e.id.clone(),
                        toEdge: connectedEdge.clone(),
                        fromLane: j.id,
                        toLane: if l.linkType == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR { j.successor } else { j.predecessor },
                        fromCP: NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END,
                        toCP: l.contactPoint,
                        all: false,
                        origID: String::new(),
                        origLane: 0,
                        shape: PositionVector::default(),
                    };
                    if l.linkType != NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR {
                        std::mem::swap(&mut c.fromEdge, &mut c.toEdge);
                        std::mem::swap(&mut c.fromLane, &mut c.toLane);
                        c.fromCP = c.toCP;
                        c.toCP = NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START;
                    }
                    pending.push(c);
                }
            }
            if let Some(lanes) = laneSection.lanesByDir.get(&T::OPENDRIVE_TAG_LEFT) {
                for j in lanes {
                    if !MY_IMPORT_ALL_TYPES.with(|c| c.get()) && !laneMap.contains_key(&j.id) {
                        continue;
                    }
                    let mut c = NIImporter_OpenDrive_Connection {
                        fromEdge: connectedEdge.clone(),
                        toEdge: e.id.clone(),
                        fromLane: if l.linkType == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR { j.successor } else { j.predecessor },
                        toLane: j.id,
                        fromCP: l.contactPoint,
                        toCP: NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_END,
                        all: false,
                        origID: String::new(),
                        origLane: 0,
                        shape: PositionVector::default(),
                    };
                    if l.linkType != NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR {
                        std::mem::swap(&mut c.fromEdge, &mut c.toEdge);
                        std::mem::swap(&mut c.fromLane, &mut c.toLane);
                        c.fromCP = c.toCP;
                        c.toCP = NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_START;
                    }
                    pending.push(c);
                }
            }
        }
        for c in pending {
            if !edges.contains_key(&c.fromEdge) {
                write_errorf!("While setting connections: incoming road '%' is not known.", c.fromEdge);
            } else {
                edges.get(&c.fromEdge).unwrap().m().connections.insert(c);
            }
        }
    }

    /// C++: `void NIImporter_OpenDrive::buildConnectionsToOuter(const Connection &, const std::map<std::string, OpenDriveEdge *> &, const std::map<std::string, OpenDriveEdge *> &, const NBTypeCont &, std::vector<Connection> &, std::set<Connection> &)` [static, private]
    // void
    // NIImporter_OpenDrive::buildConnectionsToOuter(const Connection& c,
    //         const std::map<std::string, OpenDriveEdge*>& innerEdges,
    //         const std::map<std::string, OpenDriveEdge*>& edges,
    //         const NBTypeCont& tc,
    //         std::vector<Connection>& into, std::set<Connection>& seen) {
    //
    //     OpenDriveEdge* dest = innerEdges.find(c.toEdge)->second;
    // #ifdef DEBUG_CONNECTIONS
    //     if (DEBUG_COND3(c.fromEdge)) {
    //         std::cout << "  buildConnectionsToOuter " << c.getDescription() << "\n";
    //         std::cout << "    dest=" << (dest == nullptr ? "NULL" : dest->id) << " seenlist=";
    //         for (std::set<Connection>::const_iterator i = seen.begin(); i != seen.end(); ++i) {
    //             std::cout << "    " << (*i).fromEdge << "," << (*i).toEdge << " ";
    //         }
    //         std::cout << "\n";
    //     }
    // #endif
    //     if (dest == nullptr) {
    //         /// !!! should not, look in all?
    //         return;
    //     }
    //     seen.insert(c);
    //     for (const Connection& destCon : dest->connections) {
    //         auto innerEdgesIt = innerEdges.find(destCon.toEdge);
    // #ifdef DEBUG_CONNECTIONS
    //         if (DEBUG_COND3(c.fromEdge)) {
    //             std::cout << "      toInner=" << (innerEdgesIt != innerEdges.end()) << " destCon " << destCon.getDescription() << "\n";
    //         }
    // #endif
    //         if (innerEdgesIt != innerEdges.end()) {
    //             std::vector<Connection> t;
    //             if (seen.count(destCon) == 0) {
    //                 buildConnectionsToOuter(destCon, innerEdges, edges, tc, t, seen);
    //                 for (std::vector<Connection>::const_iterator j = t.begin(); j != t.end(); ++j) {
    //                     // @todo this section is unverified
    //                     Connection cn = (*j);
    //                     cn.fromEdge = c.fromEdge;
    //                     cn.fromLane = c.fromLane;
    //                     cn.fromCP = c.fromCP;
    //                     cn.all = c.all; // @todo "all" is a hack trying to avoid the "from is zero" problem;
    //                     if (myImportInternalShapes) {
    //                         cn.shape = innerEdgesIt->second->geom + c.shape;
    //                     }
    //                     into.push_back(cn);
    //                 }
    //             } else {
    //                 WRITE_WARNING("Circular connections in junction including roads '" + c.fromEdge + "' and '" + c.toEdge + "', loop size " + toString(seen.size()));
    //             }
    //         } else {
    //             int in = c.toLane;
    //             int out = destCon.fromLane;
    //             if (c.toCP == OPENDRIVE_CP_END) {
    //                 // inner edge runs in reverse direction
    //                 std::swap(in, out);
    //             }
    // #ifdef DEBUG_CONNECTIONS
    //             if (DEBUG_COND3(c.fromEdge)) {
    //                 std::cout << "        laneSectionsConnected dest=" << dest->id << " in=" << in << " out=" << out
    //                           << " connected=" <<  laneSectionsConnected(dest, in, out) << "\n";
    //             }
    // #endif
    //
    //             if (laneSectionsConnected(dest, in, out)) {
    //                 Connection cn = destCon;
    //                 cn.fromEdge = c.fromEdge;
    //                 cn.fromLane = c.fromLane;
    //                 cn.fromCP = c.fromCP;
    //                 cn.all = c.all;
    //                 cn.origID = c.toEdge;
    //                 cn.origLane = c.toLane;
    //                 if (myImportInternalShapes) {
    //                     OpenDriveXMLTag lanesDir;
    //                     cn.shape = dest->geom;
    //                     // determine which lane of dest belongs to this connection
    //                     int referenceLane = 0;
    //                     int offsetFactor = 1;
    //                     if (c.toCP == OPENDRIVE_CP_END) {
    //                         offsetFactor = -1;
    //                         lanesDir = OPENDRIVE_TAG_LEFT;
    //                         for (const auto& destLane : dest->laneSections.front().lanesByDir[lanesDir]) {
    //                             if (destLane.successor == c.fromLane) {
    //                                 referenceLane = destLane.id;
    //                                 break;
    //                             }
    //                         }
    //                     } else {
    //                         lanesDir = OPENDRIVE_TAG_RIGHT;
    //                         for (const auto& destLane : dest->laneSections.front().lanesByDir[lanesDir]) {
    //                             if (destLane.predecessor == c.fromLane) {
    //                                 referenceLane = destLane.id;
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                     // compute offsets
    //                     //if (cn.fromEdge == "1014000" && dest->id == "3001022") {
    //                     //    std::cout << "computeOffsets\n";
    //                     //}
    //                     std::vector<double> offsets(dest->geom.size(), 0);
    //                     if (dest->laneOffsets.size() > 0) {
    //                         offsets = dest->laneOffsets;
    //                     }
    // #ifdef DEBUG_INTERNALSHAPES
    //                     std::string destPred;
    // #endif
    //                     double s = 0;
    //                     int iShape = 0;
    //                     for (int laneSectionIndex = 0; laneSectionIndex < (int)dest->laneSections.size(); laneSectionIndex++) {
    //                         OpenDriveLaneSection& laneSection = dest->laneSections[laneSectionIndex];
    //                         const double nextS = laneSectionIndex + 1 < (int)dest->laneSections.size() ? dest->laneSections[laneSectionIndex + 1].s : std::numeric_limits<double>::max();
    //                         double sStart = s; // distance offset a the start of the current lane section
    //                         double finalS = s; // final distance value after processing this segment
    //                         int finalI = iShape;
    //                         for (const OpenDriveLane& destLane : laneSection.lanesByDir[lanesDir]) {
    //                             // each lane of the current segment repeats the same section of shape points and distance offsets
    //                             double sectionS = 0;
    //                             int i = iShape; // shape index at the start of the current lane section
    //                             s = sStart;
    // #ifdef DEBUG_INTERNALSHAPES
    //                             destPred += "  lane=" + toString(destLane.id)
    //                                         + " pred=" + toString(destLane.predecessor)
    //                                         + " succ=" + toString(destLane.successor)
    //                                         + " wStart=" + (destLane.widthData.empty() ? "?" : toString(destLane.widthData.front().computeAt(0)))
    //                                         + " wEnd=" + (destLane.widthData.empty() ? "?" : toString(destLane.widthData.front().computeAt(cn.shape.length2D())))
    //                                         + " width=" + toString(destLane.width) + "\n";
    // #endif
    //                             if (abs(destLane.id) <= abs(referenceLane) || abs(destLane.id) == abs(c.toLane)) {
    //                                 const double multiplier = offsetFactor * (destLane.id == referenceLane ? 0.5 : 1);
    // #ifdef DEBUG_INTERNALSHAPES
    //                                 destPred += "     multiplier=" + toString(multiplier) + "\n";
    // #endif
    //                                 int widthDataIndex = 0;
    //                                 while (s < nextS && i < (int)cn.shape.size()) {
    //                                     if (i > 0) {
    //                                         const double dist = cn.shape[i - 1].distanceTo2D(cn.shape[i]);
    //                                         s += dist;
    //                                         sectionS += dist;
    //
    //                                     }
    //                                     while (widthDataIndex + 1 < (int)destLane.widthData.size()
    //                                             && sectionS >= destLane.widthData[widthDataIndex + 1].s) {
    //                                         widthDataIndex++;
    //                                     }
    //                                     double width = tc.getEdgeTypeWidth(destLane.type);
    //                                     if (destLane.widthData.size() > 0) {
    //                                         width = destLane.widthData[widthDataIndex].computeAt(sectionS);
    //                                     } else {
    // #ifdef DEBUG_INTERNALSHAPES
    //                                         std::cout << " missing width data at inner edge " << dest->id << " to=" << cn.toEdge << "_" << cn.toLane << " cp=" << cn.toCP << "\n";
    // #endif
    //                                         // use first width of the target lane
    //                                         OpenDriveEdge* const outerToEdge = edges.find(cn.toEdge)->second;
    //                                         OpenDriveLaneSection& toLaneSection = cn.toCP == OPENDRIVE_CP_END ? outerToEdge->laneSections.front() : outerToEdge->laneSections.back();
    //                                         const OpenDriveXMLTag laneDir = cn.toLane < 0 ? OPENDRIVE_TAG_RIGHT : OPENDRIVE_TAG_LEFT;
    //                                         for (const OpenDriveLane& outerToLane : toLaneSection.lanesByDir[laneDir]) {
    //                                             if (outerToLane.id == cn.toLane && outerToLane.width > 0) {
    // #ifdef DEBUG_INTERNALSHAPES
    //                                                 std::cout << "   using toLane width " << width << "\n";
    // #endif
    //                                                 break;
    //                                             }
    //                                         }
    //                                     }
    //                                     offsets[i] += width * multiplier;
    //                                     //if (cn.fromEdge == "1014000" && dest->id == "3001022") {
    //                                     //    std::cout << " i=" << i << " s=" << s << " lane=" << destLane.id << " rlane=" << referenceLane /*<< " nextS=" << nextS << */ << " lsIndex=" << laneSectionIndex << " wI=" << widthDataIndex << " wSize=" << destLane.widthData.size() << " m=" << multiplier << " o=" << offsets[i] << "\n";
    //                                     //}
    //                                     i++;
    //                                 }
    //                                 finalS = s;
    //                                 finalI = i;
    //                             } else if (finalS == s) {
    //                                 // update finalS without changing offsets
    //                                 while (s < nextS && i < (int)cn.shape.size()) {
    //                                     if (i > 0) {
    //                                         const double dist = cn.shape[i - 1].distanceTo2D(cn.shape[i]);
    //                                         s += dist;
    //                                         finalS += dist;
    //
    //                                     }
    //                                     i++;
    //                                 }
    //                                 finalI = i;
    //
    //                             }
    //                         }
    //                         // advance values for the next lane section
    //                         iShape = finalI;
    //                         s = finalS;
    //                     }
    //                     try {
    //                         cn.shape.move2sideCustom(offsets);
    //                     } catch (InvalidArgument&) {
    //                         WRITE_WARNING("Could not import internal lane shape from edge '" + c.fromEdge + "' to edge '" + c.toEdge);
    //                         cn.shape.clear();
    //                     }
    // #ifdef DEBUG_INTERNALSHAPES
    //                     std::cout << "internalShape "
    //                               << c.getDescription()
    //                               << " dest=" << dest->id
    //                               << " refLane=" << referenceLane
    //                               << "\n destPred=" << destPred
    //                               << "\n offsets=" << offsets
    //                               << "\n shape=" << dest->geom
    //                               << "\n shape2=" << cn.shape
    //                               << "\n";
    // #endif
    //                     if (c.toCP == OPENDRIVE_CP_END) {
    //                         cn.shape = cn.shape.reverse();
    //                     }
    //                 }
    // #ifdef DEBUG_CONNECTIONS
    //                 if (DEBUG_COND3(c.fromEdge)) {
    //                     std::cout << "        added connection\n";
    //                 }
    // #endif
    //                 into.push_back(cn);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_connections_to_outer(c: &NIImporter_OpenDrive_Connection, innerEdges: &BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>>, edges: &BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>>, tc: &NBTypeCont, into: &mut Vec<NIImporter_OpenDrive_Connection>, seen: &mut BTreeSet<NIImporter_OpenDrive_Connection>) {
        use NIImporter_OpenDrive_ContactPoint as CP;
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        let dest: Ptr<NIImporter_OpenDrive_OpenDriveEdge> = *innerEdges.get(&c.toEdge).unwrap();
        if dest.is_null() {
            // !!! should not, look in all?
            return;
        }
        seen.insert(c.clone());
        // C++ iterates dest->connections live; that set is not modified while this runs, so a snapshot is equivalent
        let destConnections: Vec<NIImporter_OpenDrive_Connection> = dest.m().connections.iter().cloned().collect();
        for destCon in destConnections.iter() {
            let innerEdgesIt = innerEdges.get(&destCon.toEdge);
            if let Some(innerEdge) = innerEdgesIt {
                let mut t: Vec<NIImporter_OpenDrive_Connection> = Vec::new();
                if !seen.contains(destCon) {
                    Self::build_connections_to_outer(destCon, innerEdges, edges, tc, &mut t, seen);
                    for j in t.iter() {
                        // @todo this section is unverified
                        let mut cn = j.clone();
                        cn.fromEdge = c.fromEdge.clone();
                        cn.fromLane = c.fromLane;
                        cn.fromCP = c.fromCP;
                        cn.all = c.all; // @todo "all" is a hack trying to avoid the "from is zero" problem;
                        if MY_IMPORT_INTERNAL_SHAPES.with(|v| v.get()) {
                            // determinism: C++ `geom + c.shape` (PositionVector::operator+) never advances its
                            // iterators: i1 stays at begin(), and pv.add(Position) offsets the still-empty pv
                            // (no-op). So real SUMO spins forever for non-empty input and returns an empty
                            // vector for empty input. Replicated literally, hang included.
                            let innerGeom = innerEdge.m().geom.clone();
                            if innerGeom.length() != c.shape.length() {
                                write_error("Trying to add PositionVectors of different lengths.");
                            }
                            let mut sum = PositionVector::default();
                            let i1: usize = 0;
                            while i1 != innerGeom.len() {
                                sum.add(&(*innerGeom.at(0) + *c.shape.at(0)));
                            }
                            cn.shape = sum;
                        }
                        into.push(cn);
                    }
                } else {
                    write_warning(&format!(
                        "Circular connections in junction including roads '{}' and '{}', loop size {}",
                        c.fromEdge, c.toEdge, seen.len()
                    ));
                }
            } else {
                let mut in_ = c.toLane;
                let mut out = destCon.fromLane;
                if c.toCP == CP::OPENDRIVE_CP_END {
                    // inner edge runs in reverse direction
                    std::mem::swap(&mut in_, &mut out);
                }
                if Self::lane_sections_connected(dest, in_, out) {
                    let mut cn = destCon.clone();
                    cn.fromEdge = c.fromEdge.clone();
                    cn.fromLane = c.fromLane;
                    cn.fromCP = c.fromCP;
                    cn.all = c.all;
                    cn.origID = c.toEdge.clone();
                    cn.origLane = c.toLane;
                    if MY_IMPORT_INTERNAL_SHAPES.with(|v| v.get()) {
                        cn.shape = dest.m().geom.clone();
                        // determine which lane of dest belongs to this connection
                        let mut referenceLane: i32 = 0;
                        let mut offsetFactor: i32 = 1;
                        let lanesDir: T;
                        if c.toCP == CP::OPENDRIVE_CP_END {
                            offsetFactor = -1;
                            lanesDir = T::OPENDRIVE_TAG_LEFT;
                            let lanes = dest.m().laneSections[0].lanesByDir.entry(lanesDir).or_default().clone();
                            for destLane in lanes.iter() {
                                if destLane.successor == c.fromLane {
                                    referenceLane = destLane.id;
                                    break;
                                }
                            }
                        } else {
                            lanesDir = T::OPENDRIVE_TAG_RIGHT;
                            let lanes = dest.m().laneSections[0].lanesByDir.entry(lanesDir).or_default().clone();
                            for destLane in lanes.iter() {
                                if destLane.predecessor == c.fromLane {
                                    referenceLane = destLane.id;
                                    break;
                                }
                            }
                        }
                        // compute offsets
                        let mut offsets: Vec<f64> = vec![0.0; dest.m().geom.len()];
                        if dest.m().laneOffsets.len() > 0 {
                            offsets = dest.m().laneOffsets.clone();
                        }
                        let mut s: f64 = 0.0;
                        let mut iShape: usize = 0;
                        for laneSectionIndex in 0..dest.m().laneSections.len() {
                            let nextS = if laneSectionIndex + 1 < dest.m().laneSections.len() {
                                dest.m().laneSections[laneSectionIndex + 1].s
                            } else {
                                f64::MAX
                            };
                            let sStart = s; // distance offset a the start of the current lane section
                            let mut finalS = s; // final distance value after processing this segment
                            let mut finalI = iShape;
                            let destLanes: Vec<NIImporter_OpenDrive_OpenDriveLane> = dest.m().laneSections[laneSectionIndex].lanesByDir.entry(lanesDir).or_default().clone();
                            for destLane in destLanes.iter() {
                                // each lane of the current segment repeats the same section of shape points and distance offsets
                                let mut sectionS: f64 = 0.0;
                                let mut i: usize = iShape; // shape index at the start of the current lane section
                                s = sStart;
                                if destLane.id.abs() <= referenceLane.abs() || destLane.id.abs() == c.toLane.abs() {
                                    let multiplier: f64 = (offsetFactor as f64) * (if destLane.id == referenceLane { 0.5 } else { 1.0 });
                                    let mut widthDataIndex: usize = 0;
                                    while s < nextS && i < cn.shape.len() {
                                        if i > 0 {
                                            let dist = cn.shape[i - 1].distance_to2d(&cn.shape[i]);
                                            s += dist;
                                            sectionS += dist;
                                        }
                                        while widthDataIndex + 1 < destLane.widthData.len()
                                            && sectionS >= destLane.widthData[widthDataIndex + 1].s
                                        {
                                            widthDataIndex += 1;
                                        }
                                        let mut width: f64 = tc.get_edge_type_width(&destLane.type_);
                                        if destLane.widthData.len() > 0 {
                                            width = destLane.widthData[widthDataIndex].compute_at(sectionS);
                                        } else {
                                            // use first width of the target lane
                                            let outerToEdge = edges.get(&cn.toEdge).unwrap().m();
                                            let toLaneSection = if cn.toCP == CP::OPENDRIVE_CP_END {
                                                outerToEdge.laneSections.first_mut().unwrap()
                                            } else {
                                                outerToEdge.laneSections.last_mut().unwrap()
                                            };
                                            let laneDir = if cn.toLane < 0 { T::OPENDRIVE_TAG_RIGHT } else { T::OPENDRIVE_TAG_LEFT };
                                            // lookup only (C++ break on first match, no effect besides the map entry)
                                            let _ = toLaneSection.lanesByDir.entry(laneDir).or_default().iter().any(|outerToLane| {
                                                outerToLane.id == cn.toLane && outerToLane.width > 0.0
                                            });
                                        }
                                        offsets[i] += width * multiplier;
                                        i += 1;
                                    }
                                    finalS = s;
                                    finalI = i;
                                } else if finalS == s {
                                    // update finalS without changing offsets
                                    while s < nextS && i < cn.shape.len() {
                                        if i > 0 {
                                            let dist = cn.shape[i - 1].distance_to2d(&cn.shape[i]);
                                            s += dist;
                                            finalS += dist;
                                        }
                                        i += 1;
                                    }
                                    finalI = i;
                                }
                            }
                            // advance values for the next lane section
                            iShape = finalI;
                            s = finalS;
                        }
                        if cn.shape.move2side_custom(&offsets, 100.0).is_err() {
                            write_warning(&format!(
                                "Could not import internal lane shape from edge '{}' to edge '{}",
                                c.fromEdge, c.toEdge
                            ));
                            cn.shape.clear();
                        }
                        if cn.toCP == CP::OPENDRIVE_CP_END {
                            cn.shape = cn.shape.reverse();
                        }
                    }
                    into.push(cn);
                }
            }
        }
    }

    /// C++: `bool NIImporter_OpenDrive::laneSectionsConnected(OpenDriveEdge *, int, int)` [static, private]
    // bool
    // NIImporter_OpenDrive::laneSectionsConnected(OpenDriveEdge* edge, int in, int out) {
    //     if (edge->laneSections.size() == 1) {
    //         return in == out;
    //     } else {
    //         // there could be spacing lanes (type 'none') that lead to a shift in lane index
    //         for (auto it = edge->laneSections.begin(); it + 1 < edge->laneSections.end(); it++) {
    //             OpenDriveLaneSection& laneSection = *it;
    //             if (laneSection.lanesByDir.find(OPENDRIVE_TAG_RIGHT) != laneSection.lanesByDir.end()) {
    //                 for (OpenDriveLane& lane : laneSection.lanesByDir.find(OPENDRIVE_TAG_RIGHT)->second) {
    //                     if (lane.id == in) {
    //                         in = lane.successor;
    //                         break;
    //                     }
    //                 }
    //             }
    //             if (laneSection.lanesByDir.find(OPENDRIVE_TAG_LEFT) != laneSection.lanesByDir.end()) {
    //                 for (OpenDriveLane& lane : laneSection.lanesByDir.find(OPENDRIVE_TAG_LEFT)->second) {
    //                     if (lane.id == in) {
    //                         in = lane.successor;
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         return in == out;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn lane_sections_connected(edge: Ptr<NIImporter_OpenDrive_OpenDriveEdge>, in_: i32, out: i32) -> bool {
        let edge = edge.m();
        if edge.laneSections.len() == 1 {
            return in_ == out;
        }
        // there could be spacing lanes (type 'none') that lead to a shift in lane index
        let mut in_ = in_;
        for it in 0..edge.laneSections.len() - 1 {
            let laneSection = &edge.laneSections[it];
            if let Some(lanes) = laneSection.lanesByDir.get(&NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT) {
                for lane in lanes {
                    if lane.id == in_ {
                        in_ = lane.successor;
                        break;
                    }
                }
            }
            if let Some(lanes) = laneSection.lanesByDir.get(&NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT) {
                for lane in lanes {
                    if lane.id == in_ {
                        in_ = lane.successor;
                        break;
                    }
                }
            }
        }
        in_ == out
    }

    /// C++: `std::string NIImporter_OpenDrive::reversedEdgeID(const std::string &)` [static, private]
    // std::string
    // NIImporter_OpenDrive::reversedEdgeID(const std::string& edgeID) {
    //     return edgeID[0] == '-' ? edgeID.substr(1) : "-" + edgeID;
    // }
    #[allow(unused_variables)]
    pub fn reversed_edge_id(id: &str) -> String {
        if id.starts_with('-') { id[1..].to_string() } else { format!("-{}", id) }
    }

    /// C++: `NBNode * NIImporter_OpenDrive::getOrBuildNode(const std::string &, const Position &, NBNodeCont &)` [static, protected]
    /// Builds a node or returns the already built
    // NBNode*
    // NIImporter_OpenDrive::getOrBuildNode(const std::string& id, const Position& pos,
    //                                      NBNodeCont& nc) {
    //     if (nc.retrieve(id) == nullptr) {
    //         // not yet built; build now
    //         if (!nc.insert(id, pos)) {
    //             // !!! clean up
    //             throw ProcessError(TLF("Could not add node '%'.", id));
    //         }
    //     }
    //     return nc.retrieve(id);
    // }
    #[allow(unused_variables)]
    pub fn get_or_build_node(id: &str, pos: &Position, nc: &mut NBNodeCont) -> Result<Ptr<NBNode>, ProcessError> {
        if nc.retrieve(id).is_null() {
            // not yet built; build now
            if !nc.insert(id, pos, Ptr::default()) {
                // !!! clean up
                return Err(ProcessError::new(tlf!("Could not add node '%'.", id.to_string())));
            }
        }
        Ok(nc.retrieve(id))
    }

    /// C++: `PositionVector NIImporter_OpenDrive::geomFromLine(const OpenDriveEdge &, const OpenDriveGeometry &, double)` [static, protected]
    // PositionVector
    // NIImporter_OpenDrive::geomFromLine(const OpenDriveEdge& e, const OpenDriveGeometry& g, double resolution) {
    //     UNUSED_PARAMETER(e);
    //     PositionVector ret;
    //     Position start(g.x, g.y);
    //     Position end = calculateStraightEndPoint(g.hdg, g.length, start);
    //     if (resolution > 0 && g.length > 0) {
    //         const int numPoints = (int)ceil(g.length / resolution) + 1;
    //         double dx = (end.x() - start.x()) / (numPoints - 1);
    //         double dy = (end.y() - start.y()) / (numPoints - 1);
    //         for (int i = 0; i < numPoints; i++) {
    //             ret.push_back(Position(g.x + i * dx, g.y + i * dy));
    //         }
    //     } else {
    //         ret.push_back(start);
    //         ret.push_back(end);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn geom_from_line(e: &NIImporter_OpenDrive_OpenDriveEdge, g: &NIImporter_OpenDrive_OpenDriveGeometry, resolution: f64) -> PositionVector {
        let mut ret = PositionVector::default();
        let start = Position::new(g.x, g.y);
        let end = Self::calculate_straight_end_point(g.hdg, g.length, &start);
        if resolution > 0.0 && g.length > 0.0 {
            let numPoints: i32 = (g.length / resolution).ceil() as i32 + 1;
            let dx = (end.x() - start.x()) / ((numPoints - 1) as f64);
            let dy = (end.y() - start.y()) / ((numPoints - 1) as f64);
            for i in 0..numPoints {
                ret.0.push(Position::new(g.x + (i as f64) * dx, g.y + (i as f64) * dy));
            }
        } else {
            ret.0.push(start);
            ret.0.push(end);
        }
        ret
    }

    /// C++: `PositionVector NIImporter_OpenDrive::geomFromSpiral(const OpenDriveEdge &, const OpenDriveGeometry &, double)` [static, protected]
    // PositionVector
    // NIImporter_OpenDrive::geomFromSpiral(const OpenDriveEdge& e, const OpenDriveGeometry& g, double resolution) {
    //     UNUSED_PARAMETER(e);
    //     PositionVector ret;
    //     double curveStart = g.params[0];
    //     double curveEnd = g.params[1];
    //     try {
    //         double cDot = (curveEnd - curveStart) / g.length;
    //         if (cDot == 0 || g.length == 0) {
    //             WRITE_WARNINGF(TL("Could not compute spiral geometry for edge '%' (cDot=% length=%)."), e.id, toString(cDot), toString(g.length));
    //             ret.push_back(Position(g.x, g.y));
    //             return ret;
    //         }
    //         double sStart = curveStart / cDot;
    //         double sEnd = curveEnd / cDot;
    //         double x = 0;
    //         double y = 0;
    //         double t = 0;
    //         double tStart = 0;
    //         double s;
    //         odrSpiral(sStart, cDot, &x, &y, &tStart);
    //         for (s = sStart; s <= sEnd; s += resolution) {
    //             odrSpiral(s, cDot, &x, &y, &t);
    //             ret.push_back(Position(x, y));
    //         }
    //         if (s != sEnd /*&& ret.size() == 1*/) {
    //             odrSpiral(sEnd, cDot, &x, &y, &t);
    //             ret.push_back(Position(x, y));
    //         }
    //         //if (s != sEnd && ret.size() > 2) {
    //         //    ret.pop_back();
    //         //}
    //         assert(ret.size() >= 2);
    //         assert(ret[0] != ret[1]);
    //         // shift start to coordinate origin
    //         PositionVector ret1 = ret;
    //         ret.add(ret.front() * -1);
    //         // rotate
    //         PositionVector ret2 = ret;
    //         ret.rotate2D(g.hdg - tStart);
    // #ifdef DEBUG_SPIRAL
    //         std::cout
    //                 << std::setprecision(4)
    //                 << "edge=" << e.id << " s=" << g.s
    //                 << " cStart=" << curveStart
    //                 << " cEnd=" << curveEnd
    //                 << " cDot=" << cDot
    //                 << " sStart=" << sStart
    //                 << " sEnd=" << sEnd
    //                 << " g.hdg=" << GeomHelper::naviDegree(g.hdg)
    //                 << " tStart=" << GeomHelper::naviDegree(tStart)
    //                 << "\n  beforeShift=" << ret1
    //                 << "\n  beforeRot=" << ret2
    //                 << "\n";
    // #endif
    //         // shift to geometry start
    //         ret.add(g.x, g.y, 0);
    //     } catch (const std::runtime_error& error) {
    //         WRITE_WARNINGF(TL("Could not compute spiral geometry for edge '%' (%)."), e.id, error.what());
    //         ret.push_back(Position(g.x, g.y));
    //     }
    //     return ret.getSubpart2D(0, g.length);
    // }
    #[allow(unused_variables)]
    pub fn geom_from_spiral(e: &NIImporter_OpenDrive_OpenDriveEdge, g: &NIImporter_OpenDrive_OpenDriveGeometry, resolution: f64) -> PositionVector {
        let mut ret = PositionVector::default();
        let curveStart = g.params[0];
        let curveEnd = g.params[1];
        // C++ try block: odrSpiral does not throw (no throw in the C++ source), so the
        // std::runtime_error catch below is unreachable and is not ported as a separate path.
        let cDot = (curveEnd - curveStart) / g.length;
        if cDot == 0.0 || g.length == 0.0 {
            write_warningf!("Could not compute spiral geometry for edge '%' (cDot=% length=%).", e.id.clone(), cDot, g.length);
            ret.push(Position::new(g.x, g.y));
            return ret;
        }
        let sStart = curveStart / cDot;
        let sEnd = curveEnd / cDot;
        let (_, _, tStart) = foreign::eulerspiral::odr_spiral::odr_spiral(sStart, cDot);
        let mut s = sStart;
        while s <= sEnd {
            let (x, y, _) = foreign::eulerspiral::odr_spiral::odr_spiral(s, cDot);
            ret.push(Position::new(x, y));
            s += resolution;
        }
        if s != sEnd {
            let (x, y, _) = foreign::eulerspiral::odr_spiral::odr_spiral(sEnd, cDot);
            ret.push(Position::new(x, y));
        }
        // shift start to coordinate origin
        let neg = *ret.front() * -1.0;
        ret.add(&neg);
        // rotate
        ret.rotate2d(g.hdg - tStart);
        // shift to geometry start
        ret.add_xyz(g.x, g.y, 0.0);
        ret.get_subpart2d(0.0, g.length)
    }

    /// C++: `PositionVector NIImporter_OpenDrive::geomFromArc(const OpenDriveEdge &, const OpenDriveGeometry &, double)` [static, protected]
    // PositionVector
    // NIImporter_OpenDrive::geomFromArc(const OpenDriveEdge& e, const OpenDriveGeometry& g, double resolution) {
    //     UNUSED_PARAMETER(e);
    //     PositionVector ret;
    //     double centerX = g.x;
    //     double centerY = g.y;
    //     // left: positive value
    //     double curvature = g.params[0];
    //     double radius = 1. / curvature;
    //     // center point
    //     calculateCurveCenter(&centerX, &centerY, radius, g.hdg);
    //     double endX = g.x;
    //     double endY = g.y;
    //     double startX = g.x;
    //     double startY = g.y;
    //     double geo_posS = g.s;
    //     double geo_posE = g.s;
    //     bool end = false;
    //     do {
    //         geo_posE += resolution;
    //         if (geo_posE - g.s > g.length) {
    //             geo_posE = g.s + g.length;
    //         }
    //         if (geo_posE - g.s > g.length) {
    //             geo_posE = g.s + g.length;
    //         }
    //         calcPointOnCurve(&endX, &endY, centerX, centerY, radius, geo_posE - geo_posS);
    //         ret.push_back(Position(startX, startY));
    //
    //         startX = endX;
    //         startY = endY;
    //         geo_posS = geo_posE;
    //
    //         if (geo_posE  - (g.s + g.length) < 0.001 && geo_posE  - (g.s + g.length) > -0.001) {
    //             end = true;
    //         }
    //     } while (!end);
    //     ret.push_back(Position(startX, startY));
    //     return ret.getSubpart2D(0, g.length);
    // }
    #[allow(unused_variables)]
    pub fn geom_from_arc(e: &NIImporter_OpenDrive_OpenDriveEdge, g: &NIImporter_OpenDrive_OpenDriveGeometry, resolution: f64) -> PositionVector {
        let mut ret = PositionVector::default();
        let mut centerX = g.x;
        let mut centerY = g.y;
        // left: positive value
        let curvature = g.params[0];
        let radius = 1.0 / curvature;
        // center point
        Self::calculate_curve_center(Ptr::from_mut(&mut centerX), Ptr::from_mut(&mut centerY), radius, g.hdg);
        let mut endX = g.x;
        let mut endY = g.y;
        let mut startX = g.x;
        let mut startY = g.y;
        let mut geo_posS = g.s;
        let mut geo_posE = g.s;
        let mut end = false;
        loop {
            geo_posE += resolution;
            if geo_posE - g.s > g.length {
                geo_posE = g.s + g.length;
            }
            if geo_posE - g.s > g.length {
                geo_posE = g.s + g.length;
            }
            Self::calc_point_on_curve(Ptr::from_mut(&mut endX), Ptr::from_mut(&mut endY), centerX, centerY, radius, geo_posE - geo_posS);
            ret.push(Position::new(startX, startY));

            startX = endX;
            startY = endY;
            geo_posS = geo_posE;

            if geo_posE - (g.s + g.length) < 0.001 && geo_posE - (g.s + g.length) > -0.001 {
                end = true;
            }
            if end {
                break;
            }
        }
        ret.push(Position::new(startX, startY));
        ret.get_subpart2d(0.0, g.length)
    }

    /// C++: `PositionVector NIImporter_OpenDrive::geomFromPoly(const OpenDriveEdge &, const OpenDriveGeometry &, double)` [static, protected]
    // PositionVector
    // NIImporter_OpenDrive::geomFromPoly(const OpenDriveEdge& e, const OpenDriveGeometry& g, double resolution) {
    //     UNUSED_PARAMETER(e);
    //     const double s = sin(g.hdg);
    //     const double c = cos(g.hdg);
    //     PositionVector ret;
    //     for (double off = 0; off < g.length + 2.; off += resolution) {
    //         double x = off;
    //         double y = g.params[0] + g.params[1] * off + g.params[2] * pow(off, 2.) + g.params[3] * pow(off, 3.);
    //         double xnew = x * c - y * s;
    //         double ynew = x * s + y * c;
    //         ret.push_back(Position(g.x + xnew, g.y + ynew));
    //     }
    //     return ret.getSubpart2D(0, g.length);
    // }
    #[allow(unused_variables)]
    pub fn geom_from_poly(e: &NIImporter_OpenDrive_OpenDriveEdge, g: &NIImporter_OpenDrive_OpenDriveGeometry, resolution: f64) -> PositionVector {
        let s = g.hdg.sin();
        let c = g.hdg.cos();
        let mut ret = PositionVector::default();
        let mut off: f64 = 0.0;
        while off < g.length + 2.0 {
            let x = off;
            let y = g.params[0] + g.params[1] * off + g.params[2] * off.powf(2.0) + g.params[3] * off.powf(3.0);
            let xnew = x * c - y * s;
            let ynew = x * s + y * c;
            ret.push(Position::new(g.x + xnew, g.y + ynew));
            off += resolution;
        }
        ret.get_subpart2d(0.0, g.length)
    }

    /// C++: `PositionVector NIImporter_OpenDrive::geomFromParamPoly(const OpenDriveEdge &, const OpenDriveGeometry &, double)` [static, protected]
    // PositionVector
    // NIImporter_OpenDrive::geomFromParamPoly(const OpenDriveEdge& e, const OpenDriveGeometry& g, double resolution) {
    //     UNUSED_PARAMETER(e);
    //     const double s = sin(g.hdg);
    //     const double c = cos(g.hdg);
    //     const double pMax = g.params[8] <= 0 ? g.length : g.params[8];
    //     const double pStep = pMax / ceil(g.length / resolution);
    //     PositionVector ret;
    //     for (double p = 0; p <= pMax + pStep; p += pStep) {
    //         double x = g.params[0] + g.params[1] * p + g.params[2] * pow(p, 2.) + g.params[3] * pow(p, 3.);
    //         double y = g.params[4] + g.params[5] * p + g.params[6] * pow(p, 2.) + g.params[7] * pow(p, 3.);
    //         double xnew = x * c - y * s;
    //         double ynew = x * s + y * c;
    //         ret.push_back(Position(g.x + xnew, g.y + ynew));
    //     }
    //     return ret.getSubpart2D(0, g.length);
    // }
    #[allow(unused_variables)]
    pub fn geom_from_param_poly(e: &NIImporter_OpenDrive_OpenDriveEdge, g: &NIImporter_OpenDrive_OpenDriveGeometry, resolution: f64) -> PositionVector {
        let s = g.hdg.sin();
        let c = g.hdg.cos();
        let pMax = if g.params[8] <= 0.0 { g.length } else { g.params[8] };
        let pStep = pMax / (g.length / resolution).ceil();
        let mut ret = PositionVector::default();
        let mut p: f64 = 0.0;
        while p <= pMax + pStep {
            let x = g.params[0] + g.params[1] * p + g.params[2] * p.powf(2.0) + g.params[3] * p.powf(3.0);
            let y = g.params[4] + g.params[5] * p + g.params[6] * p.powf(2.0) + g.params[7] * p.powf(3.0);
            let xnew = x * c - y * s;
            let ynew = x * s + y * c;
            ret.push(Position::new(g.x + xnew, g.y + ynew));
            p += pStep;
        }
        ret.get_subpart2d(0.0, g.length)
    }

    /// C++: `Position NIImporter_OpenDrive::calculateStraightEndPoint(double, double, const Position &)` [static, protected]
    // Position
    // NIImporter_OpenDrive::calculateStraightEndPoint(double hdg, double length, const Position& start) {
    //     double normx = 1.0f;
    //     double normy = 0.0f;
    //     double x2 = normx * cos(hdg) - normy * sin(hdg);
    //     double y2 = normx * sin(hdg) + normy * cos(hdg);
    //     normx = x2 * length;
    //     normy = y2 * length;
    //     return Position(start.x() + normx, start.y() + normy);
    // }
    #[allow(unused_variables)]
    pub fn calculate_straight_end_point(hdg: f64, length: f64, start: &Position) -> Position {
        let normx: f64 = 1.0;
        let normy: f64 = 0.0;
        let x2 = normx * hdg.cos() - normy * hdg.sin();
        let y2 = normx * hdg.sin() + normy * hdg.cos();
        let normx = x2 * length;
        let normy = y2 * length;
        Position::new(start.x() + normx, start.y() + normy)
    }

    /// C++: `void NIImporter_OpenDrive::calculateCurveCenter(double *, double *, double, double)` [static, protected]
    // void
    // NIImporter_OpenDrive::calculateCurveCenter(double* ad_x, double* ad_y, double ad_radius, double ad_hdg) {
    //     double normX = 1.0;
    //     double normY = 0.0;
    //     double tmpX;
    //     double turn;
    //     if (ad_radius > 0) {
    //         turn = -1.0;
    //     } else {
    //         turn = 1.0;
    //     }
    //
    //     tmpX = normX;
    //     normX = normX * cos(ad_hdg) + normY * sin(ad_hdg);
    //     normY = tmpX * sin(ad_hdg) + normY * cos(ad_hdg);
    //
    //     tmpX = normX;
    //     normX = turn * normY;
    //     normY = -turn * tmpX;
    //
    //     normX = fabs(ad_radius) * normX;
    //     normY = fabs(ad_radius) * normY;
    //
    //     *ad_x += normX;
    //     *ad_y += normY;
    // }
    #[allow(unused_variables)]
    pub fn calculate_curve_center(ad_x: Ptr<f64>, ad_y: Ptr<f64>, ad_radius: f64, ad_hdg: f64) {
        let mut normX: f64 = 1.0;
        let mut normY: f64 = 0.0;
        let mut tmpX: f64;
        let turn: f64;
        if ad_radius > 0.0 {
            turn = -1.0;
        } else {
            turn = 1.0;
        }

        tmpX = normX;
        normX = normX * ad_hdg.cos() + normY * ad_hdg.sin();
        normY = tmpX * ad_hdg.sin() + normY * ad_hdg.cos();

        tmpX = normX;
        normX = turn * normY;
        normY = -turn * tmpX;

        normX = ad_radius.abs() * normX;
        normY = ad_radius.abs() * normY;

        *ad_x.m() += normX;
        *ad_y.m() += normY;
    }

    /// C++: `void NIImporter_OpenDrive::calcPointOnCurve(double *, double *, double, double, double, double)` [static, protected]
    // void
    // NIImporter_OpenDrive::calcPointOnCurve(double* ad_x, double* ad_y, double ad_centerX, double ad_centerY,
    //                                        double ad_r, double ad_length) {
    //     double rotAngle = ad_length / fabs(ad_r);
    //     double vx = *ad_x - ad_centerX;
    //     double vy = *ad_y - ad_centerY;
    //     double tmpx;
    //
    //     double turn;
    //     if (ad_r > 0) {
    //         turn = -1;    //left
    //     } else {
    //         turn = 1;    //right
    //     }
    //     tmpx = vx;
    //     vx = vx * cos(rotAngle) + turn * vy * sin(rotAngle);
    //     vy = -1 * turn * tmpx * sin(rotAngle) + vy * cos(rotAngle);
    //     *ad_x = vx + ad_centerX;
    //     *ad_y = vy + ad_centerY;
    // }
    #[allow(unused_variables)]
    pub fn calc_point_on_curve(ad_x: Ptr<f64>, ad_y: Ptr<f64>, ad_centerX: f64, ad_centerY: f64, ad_r: f64, ad_length: f64) {
        let rotAngle = ad_length / ad_r.abs();
        let mut vx = *ad_x.m() - ad_centerX;
        let mut vy = *ad_y.m() - ad_centerY;
        let tmpx: f64;

        let turn: f64;
        if ad_r > 0.0 {
            turn = -1.0; //left
        } else {
            turn = 1.0; //right
        }
        tmpx = vx;
        vx = vx * rotAngle.cos() + turn * vy * rotAngle.sin();
        vy = -1.0 * turn * tmpx * rotAngle.sin() + vy * rotAngle.cos();
        *ad_x.m() = vx + ad_centerX;
        *ad_y.m() = vy + ad_centerY;
    }

    /// C++: `void NIImporter_OpenDrive::computeShapes(std::map<std::string, OpenDriveEdge *> &)` [static, protected]
    /// Computes a polygon representation of each edge's geometry
    // void
    // NIImporter_OpenDrive::computeShapes(std::map<std::string, OpenDriveEdge*>& edges) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     const double res = oc.getFloat("opendrive.curve-resolution");
    //     for (const auto& i : edges) {
    //         OpenDriveEdge& e = *i.second;
    //         GeometryType prevType = OPENDRIVE_GT_UNKNOWN;
    //         const double lineRes = hasNonLinearElevation(e) ? res : -1;
    //         Position last;
    //         int index = 0;
    //         for (const OpenDriveGeometry& g : e.geometries) {
    //             PositionVector geom;
    //             switch (g.type) {
    //                 case OPENDRIVE_GT_UNKNOWN:
    //                     break;
    //                 case OPENDRIVE_GT_LINE:
    //                     geom = geomFromLine(e, g, lineRes);
    //                     break;
    //                 case OPENDRIVE_GT_SPIRAL:
    //                     geom = geomFromSpiral(e, g, res);
    //                     break;
    //                 case OPENDRIVE_GT_ARC:
    //                     geom = geomFromArc(e, g, res);
    //                     break;
    //                 case OPENDRIVE_GT_POLY3:
    //                     geom = geomFromPoly(e, g, res);
    //                     break;
    //                 case OPENDRIVE_GT_PARAMPOLY3:
    //                     geom = geomFromParamPoly(e, g, res);
    //                     break;
    //                 default:
    //                     break;
    //             }
    //             if (e.geom.size() > 0 && prevType == OPENDRIVE_GT_LINE) {
    //                 // remove redundant end point of the previous geometry segment
    //                 // (the start point of the current segment should have the same value)
    //                 // this avoids geometry errors due to imprecision
    //                 if (!e.geom.back().almostSame(geom.front())) {
    //                     WRITE_WARNINGF(TL("Mismatched geometry for edge '%' between geometry segments % and %."), e.id, index - 1, index);
    //                 }
    //                 e.geom.pop_back();
    //             }
    //             //std::cout << " adding geometry to road=" << e.id << " old=" << e.geom << " new=" << geom << "\n";
    //             for (PositionVector::iterator k = geom.begin(); k != geom.end(); ++k) {
    //                 last = *k;
    //                 e.geom.push_back_noDoublePos(*k);
    //             }
    //             prevType = g.type;
    //             index++;
    //         }
    //         if (e.geom.size() == 1 && e.geom.front() != last) {
    //             // avoid length-1 geometry due to almostSame check
    //             e.geom.push_back(last);
    //         }
    // #ifdef DEBUG_SHAPE
    //         if (DEBUG_COND3(e.id)) {
    //             std::cout << e.id << " initialGeom=" << e.geom << "\n";
    //         }
    // #endif
    //         if (oc.exists("geometry.min-dist") && !oc.isDefault("geometry.min-dist")) {
    //             // simplify geometry for both directions consistently but ensure
    //             // that start and end angles are preserved
    //             if (e.geom.size() > 4) {
    //                 e.geom.removeDoublePoints(oc.getFloat("geometry.min-dist"), true, 1, 1, true);
    //             }
    //         }
    // #ifdef DEBUG_SHAPE
    //         if (DEBUG_COND3(e.id)) {
    //             std::cout << e.id << " reducedGeom=" << e.geom << "\n";
    //         }
    // #endif
    //         if (!NBNetBuilder::transformCoordinates(e.geom)) {
    //             WRITE_ERRORF(TL("Unable to project coordinates for edge '%'."), e.id);
    //         }
    //         // add z-data
    //         int k = 0;
    //         double pos = 0;
    //         //std::cout << " edge=" << e.id << " geom.size=" << e.geom.size() << " geom.len=" << e.geom.length2D() << " ele.size=" << e.elevations.size() << "\n";
    //         if (!oc.getBool("flatten")) {
    //             for (std::vector<OpenDriveElevation>::iterator j = e.elevations.begin(); j != e.elevations.end(); ++j) {
    //                 const OpenDriveElevation& el = *j;
    //                 const double sNext = (j + 1) == e.elevations.end() ? std::numeric_limits<double>::max() : (*(j + 1)).s;
    //                 while (k < (int)e.geom.size() && pos < sNext) {
    //                     const double z = el.computeAt(pos);
    //                     //std::cout << " edge=" << e.id << " k=" << k << " sNext=" << sNext << " pos=" << pos << " z=" << z << " el.s=" << el.s << " el.a=" << el.a << " el.b=" << el.b << " el.c=" << el.c << " el.d=" << el.d <<  "\n";
    //                     e.geom[k].add(0, 0, z);
    //                     k++;
    //                     if (k < (int)e.geom.size()) {
    //                         // XXX pos understimates the actual position since the
    //                         // actual geometry between k-1 and k could be curved
    //                         pos += e.geom[k - 1].distanceTo2D(e.geom[k]);
    //                     }
    //                 }
    //             }
    //         }
    //         e.geom = e.geom.simplified2(false);
    //         // add laneoffset
    //         if (e.offsets.size() > 0) {
    //             e.laneOffsets = discretizeOffsets(e.geom, e.offsets, e.id);
    //         }
    //         //std::cout << " loaded geometry " << e.id << "=" << e.geom << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_shapes(edges: &mut BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>>) {
        use NIImporter_OpenDrive_GeometryType as GT;
        let res = utils::options::options_cont::get_float("opendrive.curve-resolution");
        for (_, ePtr) in edges.iter() {
            let e = ePtr.m();
            let mut prevType = GT::OPENDRIVE_GT_UNKNOWN;
            let lineRes = if Self::has_non_linear_elevation(e) { res } else { -1.0 };
            let mut last = Position::default();
            let mut index: i32 = 0;
            let geometries = e.geometries.clone();
            for g in geometries.iter() {
                let mut geom = PositionVector::default();
                match g.type_ {
                    GT::OPENDRIVE_GT_UNKNOWN => {}
                    GT::OPENDRIVE_GT_LINE => geom = Self::geom_from_line(e, g, lineRes),
                    GT::OPENDRIVE_GT_SPIRAL => geom = Self::geom_from_spiral(e, g, res),
                    GT::OPENDRIVE_GT_ARC => geom = Self::geom_from_arc(e, g, res),
                    GT::OPENDRIVE_GT_POLY3 => geom = Self::geom_from_poly(e, g, res),
                    GT::OPENDRIVE_GT_PARAMPOLY3 => geom = Self::geom_from_param_poly(e, g, res),
                    _ => {}
                }
                if e.geom.len() > 0 && prevType == GT::OPENDRIVE_GT_LINE {
                    // remove redundant end point of the previous geometry segment
                    // (the start point of the current segment should have the same value)
                    // this avoids geometry errors due to imprecision
                    if !e.geom.back().almost_same(geom.front(), utils::geom::POSITION_EPS) {
                        write_warningf!("Mismatched geometry for edge '%' between geometry segments % and %.", e.id.clone(), index - 1, index);
                    }
                    e.geom.pop();
                }
                for k in geom.iter() {
                    last = *k;
                    e.geom.push_back_no_double_pos(k);
                }
                prevType = g.type_;
                index += 1;
            }
            if e.geom.len() == 1 && *e.geom.front() != last {
                // avoid length-1 geometry due to almostSame check
                e.geom.push(last);
            }
            if utils::options::options_cont::exists("geometry.min-dist") && !utils::options::options_cont::is_default("geometry.min-dist") {
                // simplify geometry for both directions consistently but ensure
                // that start and end angles are preserved
                if e.geom.len() > 4 {
                    let minDist = utils::options::options_cont::get_float("geometry.min-dist");
                    e.geom.remove_double_points(minDist, true, 1, 1, true);
                }
            }
            if !NBNetBuilder::transform_coordinates(&mut e.geom, true, Ptr::null()) {
                write_errorf!("Unable to project coordinates for edge '%'.", e.id.clone());
            }
            // add z-data
            let mut k: usize = 0;
            let mut pos: f64 = 0.0;
            if !utils::options::options_cont::get_bool("flatten") {
                let n = e.elevations.len();
                for (jdx, el) in e.elevations.iter().enumerate() {
                    let sNext = if jdx + 1 == n { f64::MAX } else { e.elevations[jdx + 1].s };
                    while k < e.geom.len() && pos < sNext {
                        let z = el.compute_at(pos);
                        e.geom[k].add_xyz(0.0, 0.0, z);
                        k += 1;
                        if k < e.geom.len() {
                            // XXX pos underestimates the actual position since the
                            // actual geometry between k-1 and k could be curved
                            pos += e.geom[k - 1].distance_to2d(&e.geom[k]);
                        }
                    }
                }
            }
            e.geom = e.geom.simplified2(false, utils::geom::NUMERICAL_EPS);
            // add laneoffset
            if e.offsets.len() > 0 {
                e.laneOffsets = Self::discretize_offsets(&mut e.geom, &e.offsets, &e.id);
            }
        }
    }

    /// C++: `bool NIImporter_OpenDrive::hasNonLinearElevation(const OpenDriveEdge &)` [static, protected]
    // bool
    // NIImporter_OpenDrive::hasNonLinearElevation(const OpenDriveEdge& e) {
    //     if (e.elevations.size() > 1) {
    //         return true;
    //     }
    //     for (const OpenDriveElevation& el : e.elevations) {
    //         if (el.c != 0 || el.d != 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_non_linear_elevation(e: &NIImporter_OpenDrive_OpenDriveEdge) -> bool {
        if e.elevations.len() > 1 {
            return true;
        }
        for el in e.elevations.iter() {
            if el.c != 0.0 || el.d != 0.0 {
                return true;
            }
        }
        false
    }

    /// C++: `std::vector<double> NIImporter_OpenDrive::discretizeOffsets(PositionVector &, const std::vector<OpenDriveLaneOffset> &, const std::string &)` [static, protected]
    /// transform Poly3 into a list of offsets, adding intermediate points to geom if needed
    // std::vector<double>
    // NIImporter_OpenDrive::discretizeOffsets(PositionVector& geom, const std::vector<OpenDriveLaneOffset>& offsets, const std::string& id) {
    //     UNUSED_PARAMETER(id);
    //     std::vector<double> laneOffsets;
    //     // make sure there are intermediate points for each offset-section
    //     for (const OpenDriveLaneOffset& el : offsets) {
    //         // check whether we need to insert a new point at dist
    //         Position pS = geom.positionAtOffset2D(el.s);
    //         int iS = geom.indexOfClosest(pS);
    //         // prevent close spacing to reduce impact of rounding errors in z-axis
    //         if (pS.distanceTo2D(geom[iS]) > POSITION_EPS) {
    //             geom.insertAtClosest(pS, false);
    //             //std::cout << " edge=" << e.id << " inserting pos=" << pS << " s=" << el.s << " iS=" << iS << " dist=" << pS.distanceTo2D(geom[iS]) << "\n";
    //         }
    //     }
    //     // XXX add further points for sections with non-constant offset
    //     // shift each point orthogonally by the specified offset
    //     int kk = 0;
    //     double ppos = 0;
    //     for (auto j = offsets.begin(); j != offsets.end(); ++j) {
    //         const OpenDriveLaneOffset& el = *j;
    //         const double sNext = (j + 1) == offsets.end() ? std::numeric_limits<double>::max() : (*(j + 1)).s;
    //         while (kk < (int)geom.size() && ppos < sNext) {
    //             const double offset = el.computeAt(ppos);
    //             laneOffsets.push_back(fabs(offset) > POSITION_EPS ? -offset : 0);
    //             kk++;
    //             if (kk < (int)geom.size()) {
    //                 // XXX pos underestimates the actual position since the
    //                 // actual geometry between k-1 and k could be curved
    //                 ppos += geom[kk - 1].distanceTo2D(geom[kk]);
    //             }
    //         }
    //     }
    //     return laneOffsets;
    // }
    #[allow(unused_variables)]
    pub fn discretize_offsets(geom: &mut PositionVector, offsets: &Vec<NIImporter_OpenDrive_OpenDriveLaneOffset>, id: &str) -> Vec<f64> {
        let mut laneOffsets: Vec<f64> = Vec::new();
        // make sure there are intermediate points for each offset-section
        for el in offsets.iter() {
            // check whether we need to insert a new point at dist
            let pS = geom.position_at_offset2d(el.s, 0.0, false);
            let iS = geom.index_of_closest(&pS, false);
            // prevent close spacing to reduce impact of rounding errors in z-axis
            if pS.distance_to2d(&geom[iS as usize]) > utils::geom::POSITION_EPS {
                geom.insert_at_closest(&pS, false);
            }
        }
        // XXX add further points for sections with non-constant offset
        // shift each point orthogonally by the specified offset
        let mut kk: usize = 0;
        let mut ppos: f64 = 0.0;
        let n = offsets.len();
        for (jdx, el) in offsets.iter().enumerate() {
            let sNext = if jdx + 1 == n { f64::MAX } else { offsets[jdx + 1].s };
            while kk < geom.len() && ppos < sNext {
                let offset = el.compute_at(ppos);
                laneOffsets.push(if offset.abs() > utils::geom::POSITION_EPS { -offset } else { 0.0 });
                kk += 1;
                if kk < geom.len() {
                    // XXX pos understimates the actual position since the
                    // actual geometry between k-1 and k could be curved
                    ppos += geom[kk - 1].distance_to2d(&geom[kk]);
                }
            }
        }
        let _ = id;
        laneOffsets
    }

    /// C++: `void NIImporter_OpenDrive::addOffsets(bool, PositionVector &, const std::vector<OpenDriveLaneOffset> &, const std::string &, std::vector<double> &)` [static, protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_offsets(left: bool, geom: &mut PositionVector, offsets: &Vec<NIImporter_OpenDrive_OpenDriveLaneOffset>, id: &str, result: &mut Vec<f64>) {
        // make sure there are intermediate points for each offset-section
        for el in offsets.iter() {
            // check whether we need to insert a new point at dist
            let pS = geom.position_at_offset2d(el.s, 0.0, false);
            let iS = geom.index_of_closest(&pS, false);
            // prevent close spacing to reduce impact of rounding errors in z-axis
            if pS.distance_to2d(&geom[iS as usize]) > utils::geom::POSITION_EPS {
                let at = geom.insert_at_closest(&pS, false);
                let mut interpolatedOffset: f64 = 0.0;
                if at == 0 {
                    interpolatedOffset = result[0];
                } else if at == geom.len() as i32 - 1 {
                    interpolatedOffset = *result.last().unwrap();
                } else {
                    interpolatedOffset = (result[at as usize - 1] + result[at as usize]) / 2.0;
                }
                result.insert(at as usize, interpolatedOffset);
            }
        }
        // shift each point orthogonally by the specified offset
        let mut kk: usize = 0;
        let mut ppos: f64 = 0.0;
        let sign: i32 = if left { -1 } else { 1 };
        let n = offsets.len();
        for (jdx, el) in offsets.iter().enumerate() {
            let sNext = if jdx + 1 == n { f64::MAX } else { offsets[jdx + 1].s };
            while kk < geom.len() && ppos < sNext {
                let offset = el.compute_at(ppos);
                result[kk] += if offset.abs() > utils::geom::POSITION_EPS { (sign as f64) * offset } else { 0.0 };
                kk += 1;
                if kk < geom.len() {
                    // XXX pos understimates the actual position since the
                    // actual geometry between k-1 and k could be curved
                    ppos += geom[kk - 1].distance_to2d(&geom[kk]);
                }
            }
        }
        let _ = id;
    }

    /// C++: `void NIImporter_OpenDrive::revisitLaneSections(const NBTypeCont &, std::map<std::string, OpenDriveEdge *> &)` [static, protected]
    /// Rechecks lane sections of the given edges
    // void
    // NIImporter_OpenDrive::revisitLaneSections(const NBTypeCont& tc, std::map<std::string, OpenDriveEdge*>& edges) {
    //     for (const auto& i : edges) {
    //         OpenDriveEdge& e = *i.second;
    // #ifdef DEBUG_VARIABLE_SPEED
    //         if (DEBUG_COND(&e)) {
    //             gDebugFlag1 = true;
    //             std::cout << "revisitLaneSections e=" << e.id << "\n";
    //         }
    // #endif
    //         // split by speed limits or by access restrictions
    //         std::vector<OpenDriveLaneSection> newSections;
    //         for (OpenDriveLaneSection& section : e.laneSections) {
    //             std::vector<OpenDriveLaneSection> splitSections;
    //             const bool splitByAttrChange = section.buildAttributeChanges(tc, splitSections);
    //             if (!splitByAttrChange) {
    //                 newSections.push_back(section);
    //             } else {
    //                 std::copy(splitSections.begin(), splitSections.end(), back_inserter(newSections));
    //             }
    //         }
    //
    //         e.laneSections = newSections;
    //         double lastS = -1.;
    //         // check whether the lane sections are in the right order
    //         bool sorted = true;
    //         for (const OpenDriveLaneSection& section : e.laneSections) {
    //             if (section.s <= lastS) {
    //                 sorted = false;
    //                 break;
    //             }
    //             lastS = section.s;
    //         }
    //         if (!sorted) {
    //             WRITE_WARNINGF(TL("The sections of edge '%' are not sorted properly."), e.id);
    //             sort(e.laneSections.begin(), e.laneSections.end(), sections_by_s_sorter());
    //         }
    //         // check whether duplicate s-values occur
    //         // but keep all lane sections for connecting roads because they are
    //         // needed to establish connectivity (laneSectionsConnected)
    //         // TODO recheck whether removing short sections is a good idea at all: once we parse linkage info, it will be lost.
    //         if (e.laneSections.size() > 1 && !e.isInner) {
    //             for (std::vector<OpenDriveLaneSection>::iterator j = e.laneSections.begin(); j != e.laneSections.end() - 1;) {
    //                 if ((j + 1)->s - j->s < POSITION_EPS) {
    //                     WRITE_WARNINGF(TL("Almost duplicate s-value '%' for lane sections occurred at edge '%'; first entry was removed."),  toString(j->s), e.id);
    //                     j = e.laneSections.erase(j);
    //                 } else {
    //                     ++j;
    //                 }
    //             }
    //         }
    // #ifdef DEBUG_VARIABLE_SPEED
    //         gDebugFlag1 = false;
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn revisit_lane_sections(tc: &NBTypeCont, edges: &mut BTreeMap<String, Ptr<NIImporter_OpenDrive_OpenDriveEdge>>) {
        for (_, ePtr) in edges.iter() {
            let e = ePtr.m();
            // split by speed limits or by access restrictions
            let mut newSections: Vec<NIImporter_OpenDrive_OpenDriveLaneSection> = Vec::new();
            for section in e.laneSections.iter_mut() {
                let mut splitSections: Vec<NIImporter_OpenDrive_OpenDriveLaneSection> = Vec::new();
                let splitByAttrChange = section.build_attribute_changes(tc, &mut splitSections);
                if !splitByAttrChange {
                    newSections.push(section.clone());
                } else {
                    newSections.extend(splitSections);
                }
            }

            e.laneSections = newSections;
            let mut lastS: f64 = -1.0;
            // check whether the lane sections are in the right order
            let mut sorted = true;
            for section in e.laneSections.iter() {
                if section.s <= lastS {
                    sorted = false;
                    break;
                }
                lastS = section.s;
            }
            if !sorted {
                write_warningf!("The sections of edge '%' are not sorted properly.", e.id.clone());
                // determinism: C++ std::sort is unstable; ties on s may be ordered differently
                let mut sorter = NIImporter_OpenDrive_sections_by_s_sorter::new();
                e.laneSections.sort_by(|a, b| {
                    if sorter.call_op(a, b) != 0 {
                        std::cmp::Ordering::Less
                    } else if sorter.call_op(b, a) != 0 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
            }
            // check whether duplicate s-values occur
            // but keep all lane sections for connecting roads because they are
            // needed to establish connectivity (laneSectionsConnected)
            // TODO recheck whether removing short sections is a good idea at all: once we parse linkage info, it will be lost.
            if e.laneSections.len() > 1 && !e.isInner {
                let mut j: usize = 0;
                while j != e.laneSections.len() - 1 {
                    if e.laneSections[j + 1].s - e.laneSections[j].s < utils::geom::POSITION_EPS {
                        write_warningf!("Almost duplicate s-value '%' for lane sections occurred at edge '%'; first entry was removed.", e.laneSections[j].s, e.id.clone());
                        e.laneSections.remove(j);
                    } else {
                        j += 1;
                    }
                }
            }
        }
    }

    /// C++: `void NIImporter_OpenDrive::setNodeSecure(NBNodeCont &, OpenDriveEdge &, const std::string &, NIImporter_OpenDrive::LinkType, std::vector<NodeSet> &)` [static, protected]
    // void
    // NIImporter_OpenDrive::setNodeSecure(NBNodeCont& nc, OpenDriveEdge& e,
    //                                     const std::string& nodeID, NIImporter_OpenDrive::LinkType lt, std::vector<NodeSet>& joinedNodeIDs) {
    //     NBNode* n = nc.retrieve(nodeID);
    //     if (n == nullptr) {
    //         throw ProcessError(TLF("Could not find node '%'.", nodeID));
    //     }
    //     NBNode* toJoin = nullptr;
    //     if (lt == OPENDRIVE_LT_SUCCESSOR) {
    //         if (e.to != nullptr && e.to != n) {
    //             toJoin = e.to;
    //         }
    //         e.to = n;
    //     } else {
    //         if (e.from != nullptr && e.from != n) {
    //             toJoin = e.from;
    //         }
    //         e.from = n;
    //     }
    //     if (toJoin != nullptr) {
    //         // join nodes
    //         NodeSet* set1 = nullptr;
    //         NodeSet* set2 = nullptr;
    //         for (NodeSet& joined : joinedNodeIDs) {
    //             if (joined.count(toJoin) != 0) {
    //                 set1 = &joined;
    //             }
    //             if (joined.count(n) != 0) {
    //                 set2 = &joined;
    //             }
    //         }
    //         if (set1 == nullptr && set2 == nullptr) {
    //             joinedNodeIDs.push_back(NodeSet());
    //             joinedNodeIDs.back().insert(n);
    //             joinedNodeIDs.back().insert(toJoin);
    //         } else if (set1 == nullptr && set2 != nullptr) {
    //             set2->insert(toJoin);
    //         } else if (set1 != nullptr && set2 == nullptr) {
    //             set1->insert(n);
    //         } else {
    //             set1->insert(set2->begin(), set2->end());
    //             joinedNodeIDs.erase(std::find(joinedNodeIDs.begin(), joinedNodeIDs.end(), *set2));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_node_secure(nc: &mut NBNodeCont, e: &mut NIImporter_OpenDrive_OpenDriveEdge, nodeID: &str, lt: NIImporter_OpenDrive_LinkType, joinedNodeIDs: &mut Vec<NodeSet>) -> Result<(), ProcessError> {
        let n: Ptr<NBNode> = nc.retrieve(nodeID);
        if n.is_null() {
            return Err(ProcessError::new(tlf!("Could not find node '%'.", nodeID.to_string())));
        }
        let mut toJoin: Ptr<NBNode> = Ptr::null();
        if lt == NIImporter_OpenDrive_LinkType::OPENDRIVE_LT_SUCCESSOR {
            if !e.to.is_null() && e.to != n {
                toJoin = e.to;
            }
            e.to = n;
        } else {
            if !e.from.is_null() && e.from != n {
                toJoin = e.from;
            }
            e.from = n;
        }
        if !toJoin.is_null() {
            // join nodes
            // C++ keeps the last matching set for set1/set2 (pointer overwritten in the loop)
            let mut set1: Option<usize> = None;
            let mut set2: Option<usize> = None;
            for (idx, joined) in joinedNodeIDs.iter().enumerate() {
                if joined.contains(&toJoin) {
                    set1 = Some(idx);
                }
                if joined.contains(&n) {
                    set2 = Some(idx);
                }
            }
            match (set1, set2) {
                (None, None) => {
                    let mut newSet: NodeSet = NodeSet::new();
                    newSet.insert(n);
                    newSet.insert(toJoin);
                    joinedNodeIDs.push(newSet);
                }
                (None, Some(i2)) => {
                    joinedNodeIDs[i2].insert(toJoin);
                }
                (Some(i1), None) => {
                    joinedNodeIDs[i1].insert(n);
                }
                (Some(i1), Some(i2)) => {
                    let set2Copy: NodeSet = joinedNodeIDs[i2].clone();
                    joinedNodeIDs[i1].extend(set2Copy.iter().cloned());
                    // erase the first set equal to *set2 (std::find by value)
                    if let Some(pos) = joinedNodeIDs.iter().position(|s| *s == set2Copy) {
                        joinedNodeIDs.remove(pos);
                    }
                }
            }
        }
        Ok(())
    }

    /// C++: `NBTrafficLightDefinition * NIImporter_OpenDrive::getTLSSecure(NBEdge *, NBNetBuilder &)` [static, protected]
    // NBTrafficLightDefinition*
    // NIImporter_OpenDrive::getTLSSecure(NBEdge* inEdge, /*const NBEdge::Connection& conn,*/ NBNetBuilder& nb) {
    //     NBNode* toNode = inEdge->getToNode();
    //     if (!toNode->isTLControlled()) {
    //         TrafficLightType type = SUMOXMLDefinitions::TrafficLightTypes.get(OptionsCont::getOptions().getString("tls.default-type"));
    //         NBOwnTLDef* tlDef = new NBOwnTLDef(toNode->getID(), toNode, 0, type);
    //         if (!nb.getTLLogicCont().insert(tlDef)) {
    //             // actually, nothing should fail here
    //             delete tlDef;
    //             throw ProcessError();
    //         }
    //         toNode->addTrafficLight(tlDef);
    //         //tlDef->setSinglePhase();
    //     }
    //     return *toNode->getControllingTLS().begin();
    // }
    #[allow(unused_variables)]
    pub fn get_tls_secure(inEdge: Ptr<NBEdge>, nb: &mut NBNetBuilder) -> Result<Ptr<NBTrafficLightDefinition>, ProcessError> {
        let toNode: Ptr<NBNode> = inEdge.m().get_to_node();
        if !toNode.m().is_tl_controlled() {
            let type_ = TrafficLightTypes
                .get(&utils::options::options_cont::get_string("tls.default-type"))
                .unwrap_or_else(|e| std::panic::panic_any(e));
            let raw: *mut NBOwnTLDef = Box::into_raw(Box::new(NBOwnTLDef::new_2(toNode.m().get_id(), toNode, 0, type_)));
            let tlDef: Ptr<NBTrafficLightDefinition> = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) });
            if !nb.get_tl_logic_cont().insert(tlDef, false) {
                // actually, nothing should fail here
                drop(unsafe { Box::from_raw(raw) });
                return Err(ProcessError::default_msg());
            }
            toNode.m().add_traffic_light(tlDef);
        }
        let first: Ptr<NBTrafficLightDefinition> = *toNode.m().get_controlling_tls().iter().next().unwrap();
        Ok(first)
    }

    /// C++: `std::pair<NBEdge *, NBEdge *> NIImporter_OpenDrive::retrieveSignalEdges(NBNetBuilder &, const std::string &, const std::string &, int)` [static, protected]
    // std::pair<NBEdge*, NBEdge*>
    // NIImporter_OpenDrive::retrieveSignalEdges(NBNetBuilder& nb, const std::string& fromID, const std::string& toID, int signalMinLane) {
    //     NBEdge* from;
    //     NBEdge* to;
    //     NBEdge* fromReverse;
    //     NBEdge* toReverse;
    //     from = nb.getEdgeCont().retrieve(fromID);
    //     to = nb.getEdgeCont().retrieve(toID);
    //     fromReverse = nb.getEdgeCont().retrieve(reversedEdgeID(fromID));
    //     toReverse = nb.getEdgeCont().retrieve(reversedEdgeID(toID));
    //
    //     if (from == nullptr) {
    //         from = fromReverse;
    //     }
    //     if (to == nullptr) {
    //         to = toReverse;
    //     }
    //     if (from != nullptr && to != nullptr && from->getToNode() != to->getFromNode()) {
    //         if (from->getFromNode() == to->getToNode() && signalMinLane >= 0) {
    //             std::swap(from, to);
    //         } else if (fromReverse != nullptr && toReverse != nullptr && fromReverse->getToNode() == toReverse->getFromNode() && signalMinLane <= 0) {
    //             from = fromReverse;
    //             to = toReverse;
    //         } else {
    //             if (from->getFromNode() == to->getFromNode()
    //                     && fromReverse != nullptr && fromReverse->getToNode() == to->getFromNode()) {
    //                 from = fromReverse;
    //             }
    //             if (to->getToNode() == from->getToNode()
    //                     && toReverse != nullptr && toReverse->getFromNode() == from->getToNode()) {
    //                 to = toReverse;
    //             }
    //         }
    //     }
    //     //std::cout << fromID << " " << toID << "  from=" << Named::getIDSecure(from) << " to=" << Named::getIDSecure(to) << " sMin=" << signalMinLane << "\n";
    //     return std::make_pair(from, to);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_signal_edges(nb: &mut NBNetBuilder, fromID: &str, toID: &str, signalMinLane: i32) -> (Ptr<NBEdge>, Ptr<NBEdge>) {
        let mut from: Ptr<NBEdge> = nb.get_edge_cont().retrieve(fromID, false);
        let mut to: Ptr<NBEdge> = nb.get_edge_cont().retrieve(toID, false);
        let fromReverse: Ptr<NBEdge> = nb.get_edge_cont().retrieve(&Self::reversed_edge_id(fromID), false);
        let toReverse: Ptr<NBEdge> = nb.get_edge_cont().retrieve(&Self::reversed_edge_id(toID), false);

        if from.is_null() {
            from = fromReverse;
        }
        if to.is_null() {
            to = toReverse;
        }
        if !from.is_null() && !to.is_null() && from.m().get_to_node() != to.m().get_from_node() {
            if from.m().get_from_node() == to.m().get_to_node() && signalMinLane >= 0 {
                std::mem::swap(&mut from, &mut to);
            } else if !fromReverse.is_null() && !toReverse.is_null() && fromReverse.m().get_to_node() == toReverse.m().get_from_node() && signalMinLane <= 0 {
                from = fromReverse;
                to = toReverse;
            } else {
                if from.m().get_from_node() == to.m().get_from_node() && !fromReverse.is_null() && fromReverse.m().get_to_node() == to.m().get_from_node() {
                    from = fromReverse;
                }
                if to.m().get_to_node() == from.m().get_to_node() && !toReverse.is_null() && toReverse.m().get_from_node() == from.m().get_to_node() {
                    to = toReverse;
                }
            }
        }
        (from, to)
    }

    /// C++: `void NIImporter_OpenDrive::splitMinWidths(OpenDriveEdge *, const NBTypeCont &, double)` [static, protected]
    // void
    // NIImporter_OpenDrive::splitMinWidths(OpenDriveEdge* e, const NBTypeCont& tc, double minDist) {
    //     std::vector<OpenDriveLaneSection> newSections;
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //     if (DEBUG_COND(e)) {
    //         gDebugFlag1 = true;
    //         std::cout << "splitMinWidths e=" << e->id << " sections=" << e->laneSections.size() << "\n";
    //     }
    // #endif
    //     for (std::vector<OpenDriveLaneSection>::iterator j = e->laneSections.begin(); j != e->laneSections.end(); ++j) {
    //         OpenDriveLaneSection& sec = *j;
    //         std::vector<double> splitPositions;
    //         const double sectionEnd = (j + 1) == e->laneSections.end() ? e->length : (*(j + 1)).s;
    //         const int section = (int)(j - e->laneSections.begin());
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //         if (DEBUG_COND(e)) {
    //             std::cout << "  findWidthSplit section=" << section << " sectionStart=" << sec.s << " sectionOrigStart=" << sec.sOrig << " sectionEnd=" << sectionEnd << "\n";
    //         }
    // #endif
    //         if (sec.rightLaneNumber > 0) {
    //             findWidthSplit(tc, sec.lanesByDir[OPENDRIVE_TAG_RIGHT], section, sec.sOrig, sectionEnd, splitPositions);
    //         }
    //         if (sec.leftLaneNumber > 0) {
    //             findWidthSplit(tc, sec.lanesByDir[OPENDRIVE_TAG_LEFT], section, sec.sOrig, sectionEnd, splitPositions);
    //         }
    //         newSections.push_back(sec);
    //         std::sort(splitPositions.begin(), splitPositions.end());
    //         // filter out tiny splits
    //         double prevSplit = sec.s;
    //         for (std::vector<double>::iterator it = splitPositions.begin(); it != splitPositions.end();) {
    //             if ((*it) - prevSplit < minDist || sectionEnd - (*it) < minDist) {
    //                 // avoid tiny (or duplicate) splits
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (DEBUG_COND(e)) {
    //                     std::cout << " skip close split=" << (*it) << " prevSplit=" << prevSplit << "\n";
    //                 }
    // #endif
    //                 it = splitPositions.erase(it);
    //             } else if ((*it) < sec.s) {
    //                 // avoid splits for another section
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (DEBUG_COND(e)) {
    //                     std::cout << " skip early split=" << (*it) << " s=" << sec.s << "\n";
    //                 }
    // #endif
    //                 it = splitPositions.erase(it);
    //             } else {
    //                 prevSplit = *it;
    //                 it++;
    //             }
    //         }
    //
    //         if (splitPositions.size() > 0) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //             if (DEBUG_COND(e)) {
    //                 std::cout << " road=" << e->id << " splitMinWidths section=" << section
    //                           << " start=" << sec.s
    //                           << " origStart=" << sec.sOrig
    //                           << " end=" << sectionEnd << " minDist=" << minDist
    //                           << " splitPositions=" << toString(splitPositions) << "\n";
    //             }
    // #endif
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //             if (DEBUG_COND(e)) {
    //                 std::cout << "first section...\n";
    //             }
    // #endif
    //             recomputeWidths(newSections.back(), sec.sOrig, splitPositions.front(), sec.sOrig, sectionEnd);
    //             for (std::vector<double>::iterator it = splitPositions.begin(); it != splitPositions.end(); ++it) {
    //                 OpenDriveLaneSection secNew = sec;
    //                 secNew.s = *it;
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (DEBUG_COND(e)) {
    //                     std::cout << "splitAt " << secNew.s << "\n";
    //                 }
    // #endif
    //                 newSections.push_back(secNew);
    //                 if (secNew.rightLaneNumber > 0) {
    //                     setStraightConnections(newSections.back().lanesByDir[OPENDRIVE_TAG_RIGHT]);
    //                 }
    //                 if (secNew.leftLaneNumber > 0) {
    //                     setStraightConnections(newSections.back().lanesByDir[OPENDRIVE_TAG_LEFT]);
    //                 }
    //                 double end = (it + 1) == splitPositions.end() ? sectionEnd : *(it + 1);
    //                 recomputeWidths(newSections.back(), secNew.s, end, sec.sOrig, sectionEnd);
    //             }
    //         }
    //     }
    //     gDebugFlag1 = false;
    //     e->laneSections = newSections;
    // }
    #[allow(unused_variables)]
    pub fn split_min_widths(e: Ptr<NIImporter_OpenDrive_OpenDriveEdge>, tc: &NBTypeCont, minDist: f64) {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        let mut newSections: Vec<NIImporter_OpenDrive_OpenDriveLaneSection> = Vec::new();
        let nSections = e.m().laneSections.len();
        for j in 0..nSections {
            let mut splitPositions: Vec<f64> = Vec::new();
            let sectionEnd = if j + 1 == nSections { e.m().length } else { e.m().laneSections[j + 1].s };
            let section = j as i32;
            let sec = &mut e.m().laneSections[j];
            if sec.rightLaneNumber > 0 {
                let sOrig = sec.sOrig;
                let lanes = sec.lanesByDir.entry(T::OPENDRIVE_TAG_RIGHT).or_default();
                Self::find_width_split(tc, lanes, section, sOrig, sectionEnd, &mut splitPositions);
            }
            if sec.leftLaneNumber > 0 {
                let sOrig = sec.sOrig;
                let lanes = sec.lanesByDir.entry(T::OPENDRIVE_TAG_LEFT).or_default();
                Self::find_width_split(tc, lanes, section, sOrig, sectionEnd, &mut splitPositions);
            }
            newSections.push(sec.clone());
            splitPositions.sort_by(|a, b| a.partial_cmp(b).unwrap());
            // filter out tiny splits
            let mut prevSplit: f64 = sec.s;
            let mut it: usize = 0;
            while it < splitPositions.len() {
                if splitPositions[it] - prevSplit < minDist || sectionEnd - splitPositions[it] < minDist {
                    // avoid tiny (or duplicate) splits
                    splitPositions.remove(it);
                } else if splitPositions[it] < sec.s {
                    // avoid splits for another section
                    splitPositions.remove(it);
                } else {
                    prevSplit = splitPositions[it];
                    it += 1;
                }
            }

            if splitPositions.len() > 0 {
                let sOrig = sec.sOrig;
                Self::recompute_widths(newSections.last_mut().unwrap(), sOrig, splitPositions[0], sOrig, sectionEnd);
                for it in 0..splitPositions.len() {
                    let mut secNew = sec.clone();
                    secNew.s = splitPositions[it];
                    let secNewS = secNew.s;
                    newSections.push(secNew);
                    if newSections.last().unwrap().rightLaneNumber > 0 {
                        let lanes = newSections.last_mut().unwrap().lanesByDir.entry(T::OPENDRIVE_TAG_RIGHT).or_default();
                        Self::set_straight_connections(lanes);
                    }
                    if newSections.last().unwrap().leftLaneNumber > 0 {
                        let lanes = newSections.last_mut().unwrap().lanesByDir.entry(T::OPENDRIVE_TAG_LEFT).or_default();
                        Self::set_straight_connections(lanes);
                    }
                    let end = if it + 1 == splitPositions.len() { sectionEnd } else { splitPositions[it + 1] };
                    Self::recompute_widths(newSections.last_mut().unwrap(), secNewS, end, sOrig, sectionEnd);
                }
            }
        }
        e.m().laneSections = newSections;
    }

    /// C++: `void NIImporter_OpenDrive::findWidthSplit(const NBTypeCont &, std::vector<OpenDriveLane> &, int, double, double, std::vector<double> &)` [static, protected]
    // void
    // NIImporter_OpenDrive::findWidthSplit(const NBTypeCont& tc, std::vector<OpenDriveLane>& lanes,
    //                                      int section, double sectionStart, double sectionEnd,
    //                                      std::vector<double>& splitPositions) {
    //     UNUSED_PARAMETER(section);
    //     for (const OpenDriveLane& l : lanes) {
    //         const SVCPermissions permissions = tc.getEdgeTypePermissions(l.type) & ~SVC_VULNERABLE;
    //         if (l.widthData.size() > 0 && tc.knows(l.type) && !tc.getEdgeTypeShallBeDiscarded(l.type) && permissions != 0) {
    //             double sPrev = l.widthData.front().s;
    //             double wPrev = l.widthData.front().computeAt(sPrev);
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //             if (gDebugFlag1) std::cout
    //                         << "findWidthSplit section=" << section
    //                         << "   sectionStart=" << sectionStart
    //                         << "   sectionEnd=" << sectionEnd
    //                         << " lane=" << l.id
    //                         << " type=" << l.type
    //                         << " widthEntries=" << l.widthData.size() << "\n"
    //                         << "    s=" << sPrev
    //                         << " w=" << wPrev
    //                         << "\n";
    // #endif
    //             for (std::vector<OpenDriveWidth>::const_iterator it_w = l.widthData.begin(); it_w != l.widthData.end(); ++it_w) {
    //                 double sEnd = (it_w + 1) != l.widthData.end() ? (*(it_w + 1)).s : sectionEnd - sectionStart;
    //                 double w = (*it_w).computeAt(sEnd);
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (gDebugFlag1) std::cout
    //                             << "    sEnd=" << sEnd
    //                             << " s=" << (*it_w).s
    //                             << " a=" << (*it_w).a << " b=" << (*it_w).b << " c=" << (*it_w).c << " d=" << (*it_w).d
    //                             << " w=" << w
    //                             << "\n";
    // #endif
    //                 const double changeDist = fabs(myMinWidth - wPrev);
    //                 if (((wPrev < myMinWidth) && (w > myMinWidth))
    //                         || ((wPrev > myMinWidth) && (w < myMinWidth))) {
    //                     double splitPos = sPrev + (sEnd - sPrev) / fabs(w - wPrev) * changeDist;
    //                     double wSplit = (*it_w).computeAt(splitPos);
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                     if (gDebugFlag1) {
    //                         std::cout << "     candidate splitPos=" << splitPos << " w=" << wSplit << "\n";
    //                     }
    // #endif
    //                     // ensure that the thin part is actually thin enough
    //                     while (wSplit > myMinWidth) {
    //                         if (wPrev < myMinWidth) {
    //                             // getting wider
    //                             splitPos -= POSITION_EPS;
    //                             if (splitPos < sPrev) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                                 if (gDebugFlag1) {
    //                                     std::cout << "        aborting search splitPos=" << splitPos << " wSplit=" << wSplit << " sPrev=" << sPrev << " wPrev=" << wPrev << "\n";
    //                                 }
    // #endif
    //                                 splitPos = sPrev;
    //                                 break;
    //                             }
    //                         } else {
    //                             // getting thinner
    //                             splitPos += POSITION_EPS;
    //                             if (splitPos > sEnd) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                                 if (gDebugFlag1) {
    //                                     std::cout << "        aborting search splitPos=" << splitPos << " wSplit=" << wSplit << " sEnd=" << sEnd << " w=" << w << "\n";
    //                                 }
    // #endif
    //                                 splitPos = sEnd;
    //                                 break;
    //                             }
    //                         }
    //                         wSplit = (*it_w).computeAt(splitPos);
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                         if (gDebugFlag1) {
    //                             std::cout << "        refined splitPos=" << splitPos << " w=" << wSplit << "\n";
    //                         }
    // #endif
    //                     }
    //                     splitPositions.push_back(sectionStart + splitPos);
    //                 }
    //                 //    //wPrev = wSplit;
    //                 //} else if ((fabs(wPrev) < NUMERICAL_EPS && w > POSITION_EPS)
    //                 //        || (wPrev > POSITION_EPS && fabs(w) < NUMERICAL_EPS)) {
    //                 //    splitPositions.push_back(sectionStart + sPrev);
    //                 //    if (gDebugFlag1) std::cout << "     laneDisappears candidate splitPos=" << sPrev << " wPrev=" << wPrev << " w=" << w<< "\n";
    //                 //}
    //                 wPrev = w;
    //                 sPrev = sEnd;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_width_split(tc: &NBTypeCont, lanes: &mut Vec<NIImporter_OpenDrive_OpenDriveLane>, section: i32, sectionStart: f64, sectionEnd: f64, splitPositions: &mut Vec<f64>) {
        let myMinWidth: f64 = MY_MIN_WIDTH.with(|v| v.get());
        for l in lanes.iter() {
            let permissions: SVCPermissions = tc.get_edge_type_permissions(&l.type_) & !SVC_VULNERABLE;
            if l.widthData.len() > 0 && tc.knows(&l.type_) && !tc.get_edge_type_shall_be_discarded(&l.type_) && permissions != 0 {
                let mut sPrev: f64 = l.widthData[0].s;
                let mut wPrev: f64 = l.widthData[0].compute_at(sPrev);
                for (idx, it_w) in l.widthData.iter().enumerate() {
                    let sEnd: f64 = if idx + 1 < l.widthData.len() { l.widthData[idx + 1].s } else { sectionEnd - sectionStart };
                    let w: f64 = it_w.compute_at(sEnd);
                    let changeDist: f64 = (myMinWidth - wPrev).abs();
                    if (wPrev < myMinWidth && w > myMinWidth) || (wPrev > myMinWidth && w < myMinWidth) {
                        let mut splitPos: f64 = sPrev + (sEnd - sPrev) / (w - wPrev).abs() * changeDist;
                        let mut wSplit: f64 = it_w.compute_at(splitPos);
                        // ensure that the thin part is actually thin enough
                        while wSplit > myMinWidth {
                            if wPrev < myMinWidth {
                                // getting wider
                                splitPos -= utils::geom::POSITION_EPS;
                                if splitPos < sPrev {
                                    splitPos = sPrev;
                                    break;
                                }
                            } else {
                                // getting thinner
                                splitPos += utils::geom::POSITION_EPS;
                                if splitPos > sEnd {
                                    splitPos = sEnd;
                                    break;
                                }
                            }
                            wSplit = it_w.compute_at(splitPos);
                        }
                        splitPositions.push(sectionStart + splitPos);
                    }
                    wPrev = w;
                    sPrev = sEnd;
                }
            }
        }
    }

    /// C++: `void NIImporter_OpenDrive::sanitizeWidths(OpenDriveEdge *)` [static, protected]
    // void
    // NIImporter_OpenDrive::sanitizeWidths(OpenDriveEdge* e) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //     if (DEBUG_COND(e)) {
    //         gDebugFlag1 = true;
    //         std::cout << "sanitizeWidths e=" << e->id << " sections=" << e->laneSections.size() << "\n";
    //     }
    // #endif
    //     for (OpenDriveLaneSection& sec : e->laneSections) {
    //         // filter widths within the current section (#5888).
    //         // @note, Short laneSections could also be worth filtering alltogether
    //         if (sec.rightLaneNumber > 0) {
    //             sanitizeWidths(sec.lanesByDir[OPENDRIVE_TAG_RIGHT], sec.length);
    //         }
    //         if (sec.leftLaneNumber > 0) {
    //             sanitizeWidths(sec.lanesByDir[OPENDRIVE_TAG_LEFT], sec.length);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sanitize_widths(e: Ptr<NIImporter_OpenDrive_OpenDriveEdge>) {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        for sec in e.m().laneSections.iter_mut() {
            // filter widths within the current section (#5888).
            // @note, Short laneSections could also be worth filtering alltogether
            if sec.rightLaneNumber > 0 {
                Self::sanitize_widths_2(sec.lanesByDir.entry(T::OPENDRIVE_TAG_RIGHT).or_default(), sec.length);
            }
            if sec.leftLaneNumber > 0 {
                Self::sanitize_widths_2(sec.lanesByDir.entry(T::OPENDRIVE_TAG_LEFT).or_default(), sec.length);
            }
        }
    }

    /// C++: `void NIImporter_OpenDrive::sanitizeWidths(std::vector<OpenDriveLane> &, double)` [static, protected]
    // void
    // NIImporter_OpenDrive::sanitizeWidths(std::vector<OpenDriveLane>& lanes, double length) {
    //     for (OpenDriveLane& l : lanes) {
    //         if (l.widthData.size() > 0) {
    //             auto& wd = l.widthData;
    //             const double threshold = POSITION_EPS;
    //             double maxNoShort = -std::numeric_limits<double>::max();
    //             double seen = 0;
    //             for (int i = 0; i < (int)wd.size(); i++) {
    //                 const double wdLength = i < (int)wd.size() - 1 ? wd[i + 1].s - wd[i].s : length - seen;
    //                 seen += wdLength;
    //                 if (wdLength > threshold) {
    //                     maxNoShort = MAX2(maxNoShort, wd[i].a);
    //                 }
    //             }
    //             if (maxNoShort > 0) {
    //                 l.width = maxNoShort;
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (gDebugFlag1) {
    //                     std::cout << "   lane=" << l.id << " width=" << l.width << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sanitize_widths_2(lanes: &mut Vec<NIImporter_OpenDrive_OpenDriveLane>, length: f64) {
        for l in lanes.iter_mut() {
            if l.widthData.len() > 0 {
                let wd = &l.widthData;
                let threshold: f64 = utils::geom::POSITION_EPS;
                let mut maxNoShort: f64 = -f64::MAX;
                let mut seen: f64 = 0.0;
                for i in 0..wd.len() {
                    let wdLength: f64 = if i < wd.len() - 1 { wd[i + 1].s - wd[i].s } else { length - seen };
                    seen += wdLength;
                    if wdLength > threshold {
                        // MAX2(maxNoShort, wd[i].a)
                        let a = wd[i].a;
                        maxNoShort = if maxNoShort > a { maxNoShort } else { a };
                    }
                }
                if maxNoShort > 0.0 {
                    l.width = maxNoShort;
                }
            }
        }
    }

    /// C++: `void NIImporter_OpenDrive::setStraightConnections(std::vector<OpenDriveLane> &)` [static, protected]
    // void
    // NIImporter_OpenDrive::setStraightConnections(std::vector<OpenDriveLane>& lanes) {
    //     for (std::vector<OpenDriveLane>::iterator k = lanes.begin(); k != lanes.end(); ++k) {
    //         (*k).predecessor = (*k).id;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_straight_connections(lanes: &mut Vec<NIImporter_OpenDrive_OpenDriveLane>) {
        for k in lanes.iter_mut() {
            k.predecessor = k.id;
        }
    }

    /// C++: `void NIImporter_OpenDrive::recomputeWidths(OpenDriveLaneSection &, double, double, double, double)` [static, protected]
    // void
    // NIImporter_OpenDrive::recomputeWidths(OpenDriveLaneSection& sec, double start, double end, double sectionStart, double sectionEnd) {
    //     if (sec.rightLaneNumber > 0) {
    //         recomputeWidths(sec.lanesByDir[OPENDRIVE_TAG_RIGHT], start, end, sectionStart, sectionEnd);
    //     }
    //     if (sec.leftLaneNumber > 0) {
    //         recomputeWidths(sec.lanesByDir[OPENDRIVE_TAG_LEFT], start, end, sectionStart, sectionEnd);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recompute_widths(sec: &mut NIImporter_OpenDrive_OpenDriveLaneSection, start: f64, end: f64, sectionStart: f64, sectionEnd: f64) {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        if sec.rightLaneNumber > 0 {
            Self::recompute_widths_2(sec.lanesByDir.entry(T::OPENDRIVE_TAG_RIGHT).or_default(), start, end, sectionStart, sectionEnd);
        }
        if sec.leftLaneNumber > 0 {
            Self::recompute_widths_2(sec.lanesByDir.entry(T::OPENDRIVE_TAG_LEFT).or_default(), start, end, sectionStart, sectionEnd);
        }
    }

    /// C++: `void NIImporter_OpenDrive::recomputeWidths(std::vector<OpenDriveLane> &, double, double, double, double)` [static, protected]
    // void
    // NIImporter_OpenDrive::recomputeWidths(std::vector<OpenDriveLane>& lanes, double start, double end, double sectionStart, double sectionEnd) {
    //     for (std::vector<OpenDriveLane>::iterator k = lanes.begin(); k != lanes.end(); ++k) {
    //         OpenDriveLane& l = *k;
    //         if (l.widthData.size() > 0) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //             if (gDebugFlag1) std::cout
    //                         << "recomputeWidths lane=" << l.id
    //                         << " type=" << l.type
    //                         << " start=" << start
    //                         << " end=" << end
    //                         << " sectionStart=" << sectionStart
    //                         << " sectionEnd=" << sectionEnd
    //                         << " widthEntries=" << l.widthData.size() << "\n"
    //                         << "\n";
    // #endif
    //             l.width = 0;
    //             double sPrev = l.widthData.front().s;
    //             double sPrevAbs = sPrev + sectionStart;
    //             for (std::vector<OpenDriveWidth>::iterator it_w = l.widthData.begin(); it_w != l.widthData.end(); ++it_w) {
    //                 double sEnd = (it_w + 1) != l.widthData.end() ? (*(it_w + 1)).s : sectionEnd - sectionStart;
    //                 double sEndAbs = sEnd + sectionStart;
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (gDebugFlag1) std::cout
    //                             << " sPrev=" << sPrev << " sPrevAbs=" << sPrevAbs
    //                             << " sEnd=" << sEnd << " sEndAbs=" << sEndAbs
    //                             << " widthData s=" << (*it_w).s
    //                             << " a=" << (*it_w).a
    //                             << " b=" << (*it_w).b
    //                             << " c=" << (*it_w).c
    //                             << " d=" << (*it_w).d
    //                             << "\n";
    // #endif
    //                 if (sPrevAbs <= start && sEndAbs >= start) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                     if (gDebugFlag1) {
    //                         std::cout << "   atStart=" << start << " pos=" << start - sectionStart << " w=" << (*it_w).computeAt(start - sectionStart) << "\n";
    //                     }
    // #endif
    //                     l.width = MAX2(l.width, (*it_w).computeAt(start - sectionStart));
    //                 }
    //                 if (sPrevAbs <= end && sEndAbs >= end) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                     if (gDebugFlag1) {
    //                         std::cout << "   atEnd=" << end << " pos=" << end - sectionStart << " w=" << (*it_w).computeAt(end - sectionStart) << "\n";
    //                     }
    // #endif
    //                     l.width = MAX2(l.width, (*it_w).computeAt(end - sectionStart));
    //                 }
    //                 if (start <= sPrevAbs && end >= sPrevAbs) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                     if (gDebugFlag1) {
    //                         std::cout << "   atSPrev=" << sPrev << " w=" << (*it_w).computeAt(sPrev) << "\n";
    //                     }
    // #endif
    //                     l.width = MAX2(l.width, (*it_w).computeAt(sPrev));
    //                 }
    //                 if (start <= sEndAbs && end >= sEndAbs) {
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                     if (gDebugFlag1) {
    //                         std::cout << "   atSEnd=" << sEnd << " w=" << (*it_w).computeAt(sEnd) << "\n";
    //                     }
    // #endif
    //                     l.width = MAX2(l.width, (*it_w).computeAt(sEnd));
    //                 }
    // #ifdef DEBUG_VARIABLE_WIDTHS
    //                 if (gDebugFlag1) {
    //                     std::cout << " sPrev=" << sPrev << " sEnd=" << sEnd << " l.width=" << l.width << "\n";
    //                 }
    // #endif
    //                 sPrev = sEnd;
    //                 sPrevAbs = sEndAbs;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recompute_widths_2(lanes: &mut Vec<NIImporter_OpenDrive_OpenDriveLane>, start: f64, end: f64, sectionStart: f64, sectionEnd: f64) {
        // C++ MAX2(a, b)
        let max2 = |a: f64, b: f64| if a > b { a } else { b };
        for k in lanes.iter_mut() {
            let l = k;
            if l.widthData.len() > 0 {
                l.width = 0.0;
                let mut sPrev: f64 = l.widthData[0].s;
                let mut sPrevAbs: f64 = sPrev + sectionStart;
                for idx in 0..l.widthData.len() {
                    let it_w = l.widthData[idx].clone();
                    let sEnd: f64 = if idx + 1 < l.widthData.len() { l.widthData[idx + 1].s } else { sectionEnd - sectionStart };
                    let sEndAbs: f64 = sEnd + sectionStart;
                    if sPrevAbs <= start && sEndAbs >= start {
                        l.width = max2(l.width, it_w.compute_at(start - sectionStart));
                    }
                    if sPrevAbs <= end && sEndAbs >= end {
                        l.width = max2(l.width, it_w.compute_at(end - sectionStart));
                    }
                    if start <= sPrevAbs && end >= sPrevAbs {
                        l.width = max2(l.width, it_w.compute_at(sPrev));
                    }
                    if start <= sEndAbs && end >= sEndAbs {
                        l.width = max2(l.width, it_w.compute_at(sEnd));
                    }
                    sPrev = sEnd;
                    sPrevAbs = sEndAbs;
                }
            }
        }
    }

    /// C++: `void NIImporter_OpenDrive::setLaneAttributes(const OpenDriveEdge *, NBEdge::Lane &, const OpenDriveLane &, bool, const NBTypeCont &)` [static, protected]
    // void
    // NIImporter_OpenDrive::setLaneAttributes(const OpenDriveEdge* e, NBEdge::Lane& sumoLane, const OpenDriveLane& odLane, bool saveOrigIDs, const NBTypeCont& tc) {
    //     if (saveOrigIDs) {
    //         sumoLane.setParameter(SUMO_PARAM_ORIGID, e->id + "_" + toString(odLane.id));
    //     }
    //     sumoLane.speed = odLane.speed != 0 ? odLane.speed : tc.getEdgeTypeSpeed(odLane.type);
    //     sumoLane.permissions = odLane.permission > 0 ? odLane.permission : tc.getEdgeTypePermissions(odLane.type); // TODO: how to get the OD lane specific restrictions?
    //     sumoLane.width = myImportWidths && odLane.width != NBEdge::UNSPECIFIED_WIDTH ? odLane.width : tc.getEdgeTypeWidth(odLane.type);
    //     sumoLane.type = odLane.type;
    //
    //     const double widthResolution = tc.getEdgeTypeWidthResolution(odLane.type);
    //     const double maxWidth = tc.getEdgeTypeMaxWidth(odLane.type);
    //
    //     const bool forbiddenNarrow = (sumoLane.width < myMinWidth
    //                                   && (sumoLane.permissions & ~SVC_VULNERABLE) != 0
    //                                   && sumoLane.width < tc.getEdgeTypeWidth(odLane.type));
    //
    //     if (sumoLane.width >= 0 && widthResolution > 0) {
    //         sumoLane.width = floor(sumoLane.width / widthResolution + 0.5) * widthResolution;
    //         if (forbiddenNarrow && sumoLane.width >= myMinWidth) {
    //             sumoLane.width -= widthResolution;
    //             if (sumoLane.width <= 0) {
    //                 sumoLane.width = MAX2(POSITION_EPS, myMinWidth - POSITION_EPS);
    //             }
    //         } else if (sumoLane.width == 0) {
    //             // round up when close to 0
    //             sumoLane.width = widthResolution;
    //         }
    //     }
    //     if (maxWidth > 0) {
    //         sumoLane.width = MIN2(sumoLane.width, maxWidth);
    //     }
    //     if (forbiddenNarrow) {
    //         // avoid narrow passenger car lanes (especially at sections with varying width)
    //         sumoLane.permissions = SVC_EMERGENCY | SVC_AUTHORITY;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_lane_attributes(e: Ptr<NIImporter_OpenDrive_OpenDriveEdge>, sumoLane: &mut NBEdge_Lane, odLane: &NIImporter_OpenDrive_OpenDriveLane, saveOrigIDs: bool, tc: &NBTypeCont) {
        // C++ MAX2 / MIN2
        let max2 = |a: f64, b: f64| if a > b { a } else { b };
        if saveOrigIDs {
            sumoLane.base.set_parameter(utils::xml::sumo_xml_definitions::SUMO_PARAM_ORIGID, &format!("{}_{}", e.m().id, odLane.id));
        }
        sumoLane.speed = if odLane.speed != 0.0 { odLane.speed } else { tc.get_edge_type_speed(&odLane.type_) };
        sumoLane.permissions = if odLane.permission > 0 { odLane.permission } else { tc.get_edge_type_permissions(&odLane.type_) }; // TODO: how to get the OD lane specific restriction
        sumoLane.width = if MY_IMPORT_WIDTHS.with(|v| v.get()) && odLane.width != crate::netbuild::nb_edge::UNSPECIFIED_WIDTH { odLane.width } else { tc.get_edge_type_width(&odLane.type_) };
        sumoLane.type_ = odLane.type_.clone();

        let widthResolution: f64 = tc.get_edge_type_width_resolution(&odLane.type_);
        let maxWidth: f64 = tc.get_edge_type_max_width(&odLane.type_);
        let myMinWidth: f64 = MY_MIN_WIDTH.with(|v| v.get());

        let forbiddenNarrow: bool = sumoLane.width < myMinWidth
            && (sumoLane.permissions & !SVC_VULNERABLE) != 0
            && sumoLane.width < tc.get_edge_type_width(&odLane.type_);

        if sumoLane.width >= 0.0 && widthResolution > 0.0 {
            sumoLane.width = (sumoLane.width / widthResolution + 0.5).floor() * widthResolution;
            if forbiddenNarrow && sumoLane.width >= myMinWidth {
                sumoLane.width -= widthResolution;
                if sumoLane.width <= 0.0 {
                    sumoLane.width = max2(utils::geom::POSITION_EPS, myMinWidth - utils::geom::POSITION_EPS);
                }
            } else if sumoLane.width == 0.0 {
                // round up when close to 0
                sumoLane.width = widthResolution;
            }
        }
        if maxWidth > 0.0 {
            // MIN2(sumoLane.width, maxWidth)
            sumoLane.width = if sumoLane.width < maxWidth { sumoLane.width } else { maxWidth };
        }
        if forbiddenNarrow {
            // avoid narrow passenger car lanes (especially at sections with varying width)
            sumoLane.permissions = SVC_EMERGENCY | SVC_AUTHORITY;
        }
    }

    /// C++: `void NIImporter_OpenDrive::writeRoadObjects(const OpenDriveEdge *)` [static, protected]
    // void
    // NIImporter_OpenDrive::writeRoadObjects(const OpenDriveEdge* e) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     const bool writeGeo = GeoConvHelper::getLoaded().usingGeoProjection() && (
    //                               oc.isDefault("proj.plain-geo") || oc.getBool("proj.plain-geo"));
    //     OutputDevice& dev = OutputDevice::getDevice(oc.getString("polygon-output"));
    //     dev.writeXMLHeader("additional", "additional_file.xsd");
    //     //SUMOPolygon poly("road_" + e->id, "road", RGBColor::BLUE, e->geom, true, false);
    //     //poly.writeXML(dev, false);
    //     for (auto& o : e->objects) {
    //         Position ref = e->geom.positionAtOffset2D(o.s, -o.t);
    //         if (o.radius >= 0) {
    //             // cicrular shape
    //             // GeoConvHelper::getFinal is not ready yet
    //             GeoConvHelper::getLoaded().cartesian2geo(ref);
    //             PointOfInterest POI(o.id, o.type, RGBColor::YELLOW, ref, true, "", -1, false, 0, SUMOXMLDefinitions::POIIcons.getString(POIIcon::NONE));
    //             POI.setParameter("name", o.name);
    //             POI.writeXML(dev, writeGeo);
    //         } else {
    //             // rectangular shape
    //             PositionVector centerLine;
    //             centerLine.push_back(Position(-o.length / 2, 0));
    //             centerLine.push_back(Position(o.length / 2, 0));
    //             double roadHdg = e->geom.rotationAtOffset(o.s);
    //             centerLine.rotate2D(roadHdg + o.hdg);
    //             //PointOfInterest poiRef("ref_" + o.id, "", RGBColor::CYAN, ref, false, "", 0, 0, Shape::DEFAULT_LAYER + 2);
    //             //poiRef.writeXML(dev, false);
    //             centerLine.add(ref);
    //             //SUMOPolygon polyCenter("center_" + o.id, "", RGBColor::MAGENTA, centerLine, true, false, Shape::DEFAULT_LAYER + 1);
    //             //polyCenter.writeXML(dev, false);
    //             centerLine.move2side(o.width / 2);
    //             PositionVector shape = centerLine;
    //             centerLine.move2side(-o.width);
    //             shape.append(centerLine.reverse(), POSITION_EPS);
    //             if (writeGeo) {
    //                 // GeoConvHelper::getFinal is not ready yet
    //                 for (Position& p : shape) {
    //                     GeoConvHelper::getLoaded().cartesian2geo(p);
    //                 }
    //             }
    //             SUMOPolygon poly(o.id, o.type, RGBColor::YELLOW, shape, true, true, 1, 7);
    //             poly.setParameter("name", o.name);
    //             poly.writeXML(dev, writeGeo);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_road_objects(e: Ptr<NIImporter_OpenDrive_OpenDriveEdge>) -> Result<(), ProcessError> {
        use crate::utils::shapes::point_of_interest::PointOfInterest;
        use crate::utils::shapes::shape::{DEFAULT_ANGLE, DEFAULT_HEIGHT, DEFAULT_IMG_FILE, DEFAULT_IMG_HEIGHT, DEFAULT_IMG_WIDTH, DEFAULT_LAYER, DEFAULT_NAME};
        use crate::utils::shapes::sumo_polygon::SUMOPolygon;
        let writeGeo = GeoConvHelper::with_loaded(|g| g.using_geo_projection())
            && (utils::options::options_cont::is_default("proj.plain-geo") || utils::options::options_cont::get_bool("proj.plain-geo"));
        let dev: OutputDeviceRef = OutputDevice::get_device(&utils::options::options_cont::get_string("polygon-output"), true)?;
        dev.borrow_mut().write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        for o in e.m().objects.iter() {
            let mut refPos: Position = e.m().geom.position_at_offset2d(o.s, -o.t, false);
            if o.radius >= 0.0 {
                // cicrular shape
                // GeoConvHelper::getFinal is not ready yet
                GeoConvHelper::with_loaded(|g| g.cartesian2geo(&mut refPos));
                let icon: String = POIIcons.get_string(POIIcon::NONE).unwrap().to_string();
                let mut POI = PointOfInterest::new(&o.id, &o.type_, &RGBColor::YELLOW, &refPos, true, "", -1.0, false, 0.0, &icon, DEFAULT_LAYER, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_IMG_WIDTH, DEFAULT_IMG_HEIGHT, DEFAULT_NAME, &BTreeMap::new());
                POI.base3.set_parameter("name", &o.name);
                POI.write_xml(&mut dev.borrow_mut(), writeGeo, 0.0, String::new(), 0.0, false, 0.0);
            } else {
                // rectangular shape
                let mut centerLine = PositionVector::default();
                centerLine.push(Position::new(-o.length / 2.0, 0.0));
                centerLine.push(Position::new(o.length / 2.0, 0.0));
                let roadHdg: f64 = e.m().geom.rotation_at_offset(o.s);
                centerLine.rotate2d(roadHdg + o.hdg);
                centerLine.add(&refPos);
                centerLine.move2side(o.width / 2.0, 100.0);
                let mut shape: PositionVector = centerLine.clone();
                centerLine.move2side(-o.width, 100.0);
                shape.append(&centerLine.reverse(), utils::geom::POSITION_EPS);
                if writeGeo {
                    // GeoConvHelper::getFinal is not ready yet
                    for p in shape.iter_mut() {
                        GeoConvHelper::with_loaded(|g| g.cartesian2geo(p));
                    }
                }
                let mut poly = SUMOPolygon::new(&o.id, &o.type_, &RGBColor::YELLOW, &shape, true, true, 1.0, 7.0, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_NAME, DEFAULT_HEIGHT, &BTreeMap::new());
                poly.base2.set_parameter("name", &o.name);
                poly.write_xml(&mut dev.borrow_mut(), writeGeo);
            }
        }
        Ok(())
    }

}

/// Representation of a signal group
/// C++ class `NIImporter_OpenDrive::OpenDriveController` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveController {
    pub id: String,
    pub name: String,
    pub signalIDs: Vec<String>,
    pub junction: String,
}

impl NIImporter_OpenDrive_OpenDriveController {
    /// C++: `void OpenDriveController::OpenDriveController(const std::string &, const std::string)`
    /// Constructor
    // OpenDriveController(const std::string& idArg, const std::string nameArg) :
    //             id(idArg), name(nameArg) { }
    #[allow(unused_variables)]
    pub fn new(idArg: &str, nameArg: String) -> Self {
        Self { id: idArg.to_string(), name: nameArg, signalIDs: Vec::new(), junction: String::new() }
    }

    /// C++: `void OpenDriveController::OpenDriveController()`
    /// dummy constructor for use in maps
    // OpenDriveController() {}
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        Self { id: String::new(), name: String::new(), signalIDs: Vec::new(), junction: String::new() }
    }

}

/// Representation of an openDrive "link"
/// C++ class `NIImporter_OpenDrive::OpenDriveEdge` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveEdge {
    /// The id of the edge
    pub id: String,
    /// The road name of the edge
    pub streetName: String,
    /// The id of the junction the edge belongs to
    pub junction: String,
    /// The length of the edge
    pub length: f64,
    pub links: Vec<NIImporter_OpenDrive_OpenDriveLink>,
    pub geometries: Vec<NIImporter_OpenDrive_OpenDriveGeometry>,
    pub elevations: Vec<NIImporter_OpenDrive_OpenDriveElevation>,
    pub offsets: Vec<NIImporter_OpenDrive_OpenDriveLaneOffset>,
    pub from: Ptr<NBNode>,
    pub to: Ptr<NBNode>,
    pub geom: PositionVector,
    pub laneOffsets: Vec<f64>,
    pub laneSections: Vec<NIImporter_OpenDrive_OpenDriveLaneSection>,
    pub signals: Vec<NIImporter_OpenDrive_OpenDriveSignal>,
    pub connections: BTreeSet<NIImporter_OpenDrive_Connection>,
    pub objects: Vec<NIImporter_OpenDrive_OpenDriveObject>,
    pub isInner: bool,
}

impl NIImporter_OpenDrive_OpenDriveEdge {
    /// C++: `void OpenDriveEdge::OpenDriveEdge(const std::string &, const std::string &, const std::string &, double)`
    // OpenDriveEdge(const std::string& idArg, const std::string& streetNameArg, const std::string& junctionArg, double lengthArg) :
    //             id(idArg),
    //             streetName(streetNameArg),
    //             junction(junctionArg),
    //             length(lengthArg),
    //             from(0),
    //             to(0) {
    //             isInner = junction != "" && junction != "-1";
    //         }
    #[allow(unused_variables)]
    pub fn new(idArg: &str, streetNameArg: &str, junctionArg: &str, lengthArg: f64) -> Self {
        Self {
            id: idArg.to_string(),
            streetName: streetNameArg.to_string(),
            junction: junctionArg.to_string(),
            length: lengthArg,
            links: Vec::new(),
            geometries: Vec::new(),
            elevations: Vec::new(),
            offsets: Vec::new(),
            from: Ptr::default(),
            to: Ptr::default(),
            geom: PositionVector::default(),
            laneOffsets: Vec::new(),
            laneSections: Vec::new(),
            signals: Vec::new(),
            connections: BTreeSet::new(),
            objects: Vec::new(),
            isInner: junctionArg != "" && junctionArg != "-1",
        }
    }

    /// C++: `int OpenDriveEdge::getPriority(OpenDriveXMLTag) const` [const]
    /// Returns the edge's priority, regarding the direction
    // int
    // NIImporter_OpenDrive::OpenDriveEdge::getPriority(OpenDriveXMLTag dir) const {
    //     // for signal interpretations see https://de.wikipedia.org/wiki/Bildtafel_der_Verkehrszeichen_in_der_Bundesrepublik_Deutschland_seit_2013
    //     int prio = 1;
    //     for (std::vector<OpenDriveSignal>::const_iterator i = signals.begin(); i != signals.end(); ++i) {
    //         int tmp = 1;
    //         if ((*i).type == "301" || (*i).type == "306") { // priority road or local priority
    //             tmp = 2;
    //         }
    //         if ((*i).type == "205" /*|| (*i).type == "206"*/) { // yield or stop
    //             tmp = 0;
    //         }
    //         if (tmp != 1 && dir == OPENDRIVE_TAG_RIGHT && (*i).orientation > 0) {
    //             prio = tmp;
    //         }
    //         if (tmp != 1 && dir == OPENDRIVE_TAG_LEFT && (*i).orientation < 0) {
    //             prio = tmp;
    //         }
    //
    //     }
    //     return prio;
    // }
    #[allow(unused_variables)]
    pub fn get_priority(&self, dir: NIImporter_OpenDrive_OpenDriveXMLTag) -> i32 {
        // for signal interpretations see https://de.wikipedia.org/wiki/Bildtafel_der_Verkehrszeichen_in_der_Bundesrepublik_Deutschland_seit_2013
        let mut prio = 1;
        for i in &self.signals {
            let mut tmp = 1;
            if i.type_ == "301" || i.type_ == "306" {
                // priority road or local priority
                tmp = 2;
            }
            if i.type_ == "205" {
                // yield or stop
                tmp = 0;
            }
            if tmp != 1 && dir == NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT && i.orientation > 0 {
                prio = tmp;
            }
            if tmp != 1 && dir == NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT && i.orientation < 0 {
                prio = tmp;
            }
        }
        prio
    }

}

/// Representation of an OpenDrive geometry part
/// C++ class `NIImporter_OpenDrive::OpenDriveGeometry` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveGeometry {
    pub length: f64,
    pub s: f64,
    pub x: f64,
    pub y: f64,
    pub hdg: f64,
    pub type_: NIImporter_OpenDrive_GeometryType,
    pub params: Vec<f64>,
}

impl NIImporter_OpenDrive_OpenDriveGeometry {
    /// C++: `void OpenDriveGeometry::OpenDriveGeometry(double, double, double, double, double)`
    /// Constructor
    // OpenDriveGeometry(double lengthArg, double sArg, double xArg, double yArg, double hdgArg)
    //             : length(lengthArg), s(sArg), x(xArg), y(yArg), hdg(hdgArg),
    //               type(OPENDRIVE_GT_UNKNOWN) { }
    #[allow(unused_variables)]
    pub fn new(lengthArg: f64, sArg: f64, xArg: f64, yArg: f64, hdgArg: f64) -> Self {
        Self { length: lengthArg, s: sArg, x: xArg, y: yArg, hdg: hdgArg, type_: NIImporter_OpenDrive_GeometryType::OPENDRIVE_GT_UNKNOWN, params: Vec::new() }
    }

}

/// Representation of a lane
/// C++ class `NIImporter_OpenDrive::OpenDriveLane` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveLane {
    /// The lane's id
    pub id: i32,
    /// The lane's level (not used)
    pub level: String,
    /// The lane's type
    pub type_: String,
    /// The lane's successor lane
    pub successor: i32,
    /// The lane's predecessor lane
    pub predecessor: i32,
    /// List of permission and speed changes
    pub attributeChanges: Vec<(f64, NIImporter_OpenDrive_LaneAttributeChange)>,
    /// The lane's speed (set in post-processing)
    pub speed: f64,
    pub width: f64,
    /// The access permissions (set in post-processing)
    pub permission: SVCPermissions,
    pub widthData: Vec<NIImporter_OpenDrive_OpenDriveWidth>,
}

impl NIImporter_OpenDrive_OpenDriveLane {
    /// C++: `void OpenDriveLane::OpenDriveLane(int, const std::string &, const std::string &)`
    /// Constructor
    // OpenDriveLane(int idArg, const std::string& levelArg, const std::string& typeArg) :
    //             id(idArg), level(levelArg), type(typeArg), successor(UNSET_CONNECTION), predecessor(UNSET_CONNECTION),
    //             speed(0), width(NBEdge::UNSPECIFIED_WIDTH), permission(0) { }
    #[allow(unused_variables)]
    pub fn new(idArg: i32, levelArg: &str, typeArg: &str) -> Self {
        Self {
            id: idArg,
            level: levelArg.to_string(),
            type_: typeArg.to_string(),
            successor: UNSET_CONNECTION,
            predecessor: UNSET_CONNECTION,
            attributeChanges: Vec::new(),
            speed: 0.0,
            width: NBEDGE_UNSPECIFIED_WIDTH,
            permission: 0,
            widthData: Vec::new(),
        }
    }

    /// C++: `SVCPermissions OpenDriveLane::computePermission(const NBTypeCont &, const std::vector<std::string> &, const std::vector<std::string> &) const` [const]
    /// compute the actual SUMO lane permissions given the lane type as a start solution
    // SVCPermissions
    // NIImporter_OpenDrive::OpenDriveLane::computePermission(const NBTypeCont& tc, const std::vector<std::string>& allowed,
    //         const std::vector<std::string>& denied) const {
    //     SVCPermissions perms = tc.getEdgeTypePermissions(type);
    //     if (allowed.size() > 0 && denied.size() > 0) {
    //         WRITE_WARNING(TL("Will discard access settings as both denied and allowed classes have been specified."));
    //     } else if (allowed.size() > 0) {
    //         perms = SVC_IGNORING;
    //         for (const std::string& allow : allowed) {
    //             if (allow == "simulator") {
    //                 perms = SVC_IGNORING;
    //                 break;
    //             } else if (allow == "autonomousTraffic" || allow == "autonomous traffic" || allow == "throughTraffic") {
    //                 perms = tc.getEdgeTypePermissions(type);
    //                 break;
    //             } else if (allow == "pedestrian") {
    //                 perms |= SVC_PEDESTRIAN;
    //             } else if (allow == "passengerCar") {
    //                 perms |= SVC_PASSENGER;
    //             } else if (allow == "bus") {
    //                 perms |= SVC_BUS;
    //             } else if (allow == "delivery") {
    //                 perms |= SVC_DELIVERY;
    //             } else if (allow == "emergency") {
    //                 perms |= SVC_EMERGENCY;
    //             } else if (allow == "taxi") {
    //                 perms |= SVC_TAXI;
    //             } else if (allow == "bicycle") {
    //                 perms |= SVC_BICYCLE;
    //             } else if (allow == "motorcycle") {
    //                 perms |= SVC_MOTORCYCLE;
    //             } else if (allow == "truck" || allow == "trucks") {
    //                 perms |= SVC_TRUCK;
    //                 perms |= SVC_TRAILER;
    //             }
    //         }
    //     } else if (denied.size() > 0) {
    //         for (const std::string& deny : denied) {
    //             if (deny == "none") {
    //                 perms = tc.getEdgeTypePermissions(type);
    //                 break;
    //             } else if (deny == "autonomousTraffic" || deny == "autonomous traffic" || deny == "throughTraffic") {
    //                 perms = SVC_IGNORING;
    //                 break;
    //             } else if (deny == "pedestrian") {
    //                 perms &= ~SVC_PEDESTRIAN;
    //             } else if (deny == "passengerCar") {
    //                 perms &= ~SVC_PASSENGER;
    //             } else if (deny == "bus") {
    //                 perms &= ~SVC_BUS;
    //             } else if (deny == "delivery") {
    //                 perms &= ~SVC_DELIVERY;
    //             } else if (deny == "emergency") {
    //                 perms &= ~SVC_EMERGENCY;
    //             } else if (deny == "taxi") {
    //                 perms &= ~SVC_TAXI;
    //             } else if (deny == "bicycle") {
    //                 perms &= ~SVC_BICYCLE;
    //             } else if (deny == "motorcycle") {
    //                 perms &= ~SVC_MOTORCYCLE;
    //             } else if (deny == "truck" || deny == "trucks") {
    //                 perms &= ~SVC_TRUCK;
    //                 perms &= ~SVC_TRAILER;
    //             }
    //         }
    //     }
    //     return perms;
    // }
    #[allow(unused_variables)]
    pub fn compute_permission(&self, tc: &NBTypeCont, allowed: &Vec<String>, denied: &Vec<String>) -> SVCPermissions {
        let mut perms: SVCPermissions = tc.get_edge_type_permissions(&self.type_);
        if allowed.len() > 0 && denied.len() > 0 {
            write_warning("Will discard access settings as both denied and allowed classes have been specified.");
        } else if allowed.len() > 0 {
            perms = SVC_IGNORING;
            for allow in allowed.iter() {
                match allow.as_str() {
                    "simulator" => {
                        perms = SVC_IGNORING;
                        break;
                    }
                    "autonomousTraffic" | "autonomous traffic" | "throughTraffic" => {
                        perms = tc.get_edge_type_permissions(&self.type_);
                        break;
                    }
                    "pedestrian" => perms |= SVC_PEDESTRIAN,
                    "passengerCar" => perms |= SVC_PASSENGER,
                    "bus" => perms |= SVC_BUS,
                    "delivery" => perms |= SVC_DELIVERY,
                    "emergency" => perms |= SVC_EMERGENCY,
                    "taxi" => perms |= SVC_TAXI,
                    "bicycle" => perms |= SVC_BICYCLE,
                    "motorcycle" => perms |= SVC_MOTORCYCLE,
                    "truck" | "trucks" => {
                        perms |= SVC_TRUCK;
                        perms |= SVC_TRAILER;
                    }
                    _ => {}
                }
            }
        } else if denied.len() > 0 {
            for deny in denied.iter() {
                match deny.as_str() {
                    "none" => {
                        perms = tc.get_edge_type_permissions(&self.type_);
                        break;
                    }
                    "autonomousTraffic" | "autonomous traffic" | "throughTraffic" => {
                        perms = SVC_IGNORING;
                        break;
                    }
                    "pedestrian" => perms &= !SVC_PEDESTRIAN,
                    "passengerCar" => perms &= !SVC_PASSENGER,
                    "bus" => perms &= !SVC_BUS,
                    "delivery" => perms &= !SVC_DELIVERY,
                    "emergency" => perms &= !SVC_EMERGENCY,
                    "taxi" => perms &= !SVC_TAXI,
                    "bicycle" => perms &= !SVC_BICYCLE,
                    "motorcycle" => perms &= !SVC_MOTORCYCLE,
                    "truck" | "trucks" => {
                        perms &= !SVC_TRUCK;
                        perms &= !SVC_TRAILER;
                    }
                    _ => {}
                }
            }
        }
        perms
    }

}

/// Representation of a lane section
/// C++ class `NIImporter_OpenDrive::OpenDriveLaneSection` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveLaneSection {
    /// The starting offset of this lane section
    pub s: f64,
    /// The original starting offset of this lane section (differs from s if the section had to be split)
    pub sOrig: f64,
    /// The length of this lane section
    pub length: f64,
    /// A mapping from OpenDrive to SUMO-index (the first is signed, the second unsigned)
    pub laneMap: BTreeMap<i32, i32>,
    /// The lanes, sorted by their direction
    pub lanesByDir: BTreeMap<NIImporter_OpenDrive_OpenDriveXMLTag, Vec<NIImporter_OpenDrive_OpenDriveLane>>,
    /// The id (generic, without the optionally leading '-') of the edge generated for this section
    pub sumoID: String,
    /// The number of lanes on the right and on the left side, respectively
    pub rightLaneNumber: i32,
    /// The number of lanes on the right and on the left side, respectively
    pub leftLaneNumber: i32,
    /// the composite type built from all used lane types
    pub rightType: String,
    pub leftType: String,
    /// average width of removed inside lanes
    pub discardedInnerWidthLeft: f64,
    pub discardedInnerWidthRight: f64,
}

impl NIImporter_OpenDrive_OpenDriveLaneSection {
    /// C++: `void OpenDriveLaneSection::OpenDriveLaneSection(double)`
    /// Constructor
    // NIImporter_OpenDrive::OpenDriveLaneSection::OpenDriveLaneSection(double sArg) : s(sArg), sOrig(sArg) {
    //     lanesByDir[OPENDRIVE_TAG_LEFT] = std::vector<OpenDriveLane>();
    //     lanesByDir[OPENDRIVE_TAG_RIGHT] = std::vector<OpenDriveLane>();
    //     lanesByDir[OPENDRIVE_TAG_CENTER] = std::vector<OpenDriveLane>();
    // }
    #[allow(unused_variables)]
    pub fn new(sArg: f64) -> Self {
        let mut lanesByDir: BTreeMap<NIImporter_OpenDrive_OpenDriveXMLTag, Vec<NIImporter_OpenDrive_OpenDriveLane>> = BTreeMap::new();
        lanesByDir.insert(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_LEFT, Vec::new());
        lanesByDir.insert(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_RIGHT, Vec::new());
        lanesByDir.insert(NIImporter_OpenDrive_OpenDriveXMLTag::OPENDRIVE_TAG_CENTER, Vec::new());
        Self {
            s: sArg,
            sOrig: sArg,
            length: 0.0,
            laneMap: BTreeMap::new(),
            lanesByDir,
            sumoID: String::new(),
            rightLaneNumber: 0,
            leftLaneNumber: 0,
            rightType: String::new(),
            leftType: String::new(),
            discardedInnerWidthLeft: 0.0,
            discardedInnerWidthRight: 0.0,
        }
    }

    /// C++: `void OpenDriveLaneSection::buildLaneMapping(const NBTypeCont &)`
    /// Build the mapping from OpenDrive to SUMO lanes
    // void
    // NIImporter_OpenDrive::OpenDriveLaneSection::buildLaneMapping(const NBTypeCont& tc) {
    //     discardedInnerWidthRight = 0;
    //     int sumoLane = 0;
    //     bool singleType = true;
    //     std::vector<std::string> types;
    //     const std::vector<OpenDriveLane>& dirLanesR = lanesByDir.find(OPENDRIVE_TAG_RIGHT)->second;
    //     for (std::vector<OpenDriveLane>::const_reverse_iterator i = dirLanesR.rbegin(); i != dirLanesR.rend(); ++i) {
    //         if (myImportAllTypes || (tc.knows((*i).type) && !tc.getEdgeTypeShallBeDiscarded((*i).type))) {
    //             discardedInnerWidthRight = 0;
    //             laneMap[(*i).id] = sumoLane++;
    //             types.push_back((*i).type);
    //             if (types.front() != types.back()) {
    //                 singleType = false;
    //             }
    //         } else {
    //             discardedInnerWidthRight += (*i).width;
    //         }
    //     }
    //     discardedInnerWidthLeft = 0;
    //     rightLaneNumber = sumoLane;
    //     rightType = sumoLane > 0 ? (singleType ? types.front() : joinToString(types, "|")) : "";
    //     sumoLane = 0;
    //     singleType = true;
    //     types.clear();
    //     const std::vector<OpenDriveLane>& dirLanesL = lanesByDir.find(OPENDRIVE_TAG_LEFT)->second;
    //     for (std::vector<OpenDriveLane>::const_iterator i = dirLanesL.begin(); i != dirLanesL.end(); ++i) {
    //         if (myImportAllTypes || (tc.knows((*i).type) && !tc.getEdgeTypeShallBeDiscarded((*i).type))) {
    //             discardedInnerWidthLeft = 0;
    //             laneMap[(*i).id] = sumoLane++;
    //             types.push_back((*i).type);
    //             if (types.front() != types.back()) {
    //                 singleType = false;
    //             }
    //         } else {
    //             discardedInnerWidthLeft += (*i).width;
    //         }
    //     }
    //     leftLaneNumber = sumoLane;
    //     leftType = sumoLane > 0 ? (singleType ? types.front() : joinToString(types, "|")) : "";
    // }
    #[allow(unused_variables)]
    pub fn build_lane_mapping(&mut self, tc: &NBTypeCont) {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        self.discardedInnerWidthRight = 0.0;
        let mut sumoLane: i32 = 0;
        let mut singleType = true;
        let mut types: Vec<String> = Vec::new();
        let dirLanesR = self.lanesByDir.get(&T::OPENDRIVE_TAG_RIGHT).cloned().unwrap_or_default();
        for i in dirLanesR.iter().rev() {
            if MY_IMPORT_ALL_TYPES.with(|v| v.get()) || (tc.knows(&i.type_) && !tc.get_edge_type_shall_be_discarded(&i.type_)) {
                self.discardedInnerWidthRight = 0.0;
                self.laneMap.insert(i.id, sumoLane);
                sumoLane += 1;
                types.push(i.type_.clone());
                if types[0] != *types.last().unwrap() {
                    singleType = false;
                }
            } else {
                self.discardedInnerWidthRight += i.width;
            }
        }
        self.discardedInnerWidthLeft = 0.0;
        self.rightLaneNumber = sumoLane;
        self.rightType = if sumoLane > 0 {
            if singleType { types[0].clone() } else { types.join("|") }
        } else {
            String::new()
        };
        sumoLane = 0;
        singleType = true;
        types.clear();
        let dirLanesL = self.lanesByDir.get(&T::OPENDRIVE_TAG_LEFT).cloned().unwrap_or_default();
        for i in dirLanesL.iter() {
            if MY_IMPORT_ALL_TYPES.with(|v| v.get()) || (tc.knows(&i.type_) && !tc.get_edge_type_shall_be_discarded(&i.type_)) {
                self.discardedInnerWidthLeft = 0.0;
                self.laneMap.insert(i.id, sumoLane);
                sumoLane += 1;
                types.push(i.type_.clone());
                if types[0] != *types.last().unwrap() {
                    singleType = false;
                }
            } else {
                self.discardedInnerWidthLeft += i.width;
            }
        }
        self.leftLaneNumber = sumoLane;
        self.leftType = if sumoLane > 0 {
            if singleType { types[0].clone() } else { types.join("|") }
        } else {
            String::new()
        };
    }

    /// C++: `std::map<int, int> OpenDriveLaneSection::getInnerConnections(OpenDriveXMLTag, const OpenDriveLaneSection &)`
    /// Returns the links from the previous to this lane section
    // std::map<int, int>
    // NIImporter_OpenDrive::OpenDriveLaneSection::getInnerConnections(OpenDriveXMLTag dir, const OpenDriveLaneSection& prev) {
    //     std::map<int, int> ret;
    //     const std::vector<OpenDriveLane>& dirLanes = lanesByDir.find(dir)->second;
    //     for (std::vector<OpenDriveLane>::const_reverse_iterator i = dirLanes.rbegin(); i != dirLanes.rend(); ++i) {
    //         std::map<int, int>::const_iterator toP = laneMap.find((*i).id);
    //         if (toP == laneMap.end()) {
    //             // the current lane is not available in SUMO
    //             continue;
    //         }
    //         int to = (*toP).second;
    //         int from = UNSET_CONNECTION;
    //         if ((*i).predecessor != UNSET_CONNECTION) {
    //             from = (*i).predecessor;
    //         }
    //         if (from != UNSET_CONNECTION) {
    //             std::map<int, int>::const_iterator fromP = prev.laneMap.find(from);
    //             if (fromP != prev.laneMap.end()) {
    //                 from = (*fromP).second;
    //             } else {
    //                 from = UNSET_CONNECTION;
    //             }
    //         }
    //         if (from != UNSET_CONNECTION && to != UNSET_CONNECTION) {
    //             if (ret.find(from) != ret.end()) {
    // //        WRITE_WARNING(TL("double connection"));
    //             }
    //             if (dir == OPENDRIVE_TAG_LEFT) {
    //                 std::swap(from, to);
    //             }
    //             ret[from] = to;
    //         } else {
    // //      WRITE_WARNING(TL("missing connection"));
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_inner_connections(&mut self, dir: NIImporter_OpenDrive_OpenDriveXMLTag, prev: &NIImporter_OpenDrive_OpenDriveLaneSection) -> BTreeMap<i32, i32> {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        let mut ret: BTreeMap<i32, i32> = BTreeMap::new();
        let dirLanes = self.lanesByDir.get(&dir).cloned().unwrap_or_default();
        for i in dirLanes.iter().rev() {
            let mut to: i32 = match self.laneMap.get(&i.id) {
                Some(v) => *v,
                None => {
                    // the current lane is not available in SUMO
                    continue;
                }
            };
            let mut from: i32 = UNSET_CONNECTION;
            if i.predecessor != UNSET_CONNECTION {
                from = i.predecessor;
            }
            if from != UNSET_CONNECTION {
                match prev.laneMap.get(&from) {
                    Some(v) => from = *v,
                    None => from = UNSET_CONNECTION,
                }
            }
            if from != UNSET_CONNECTION && to != UNSET_CONNECTION {
                if dir == T::OPENDRIVE_TAG_LEFT {
                    std::mem::swap(&mut from, &mut to);
                }
                ret.insert(from, to);
            }
        }
        ret
    }

    /// C++: `bool OpenDriveLaneSection::buildAttributeChanges(const NBTypeCont &, std::vector<OpenDriveLaneSection> &)`
    // bool
    // NIImporter_OpenDrive::OpenDriveLaneSection::buildAttributeChanges(const NBTypeCont& tc, std::vector<OpenDriveLaneSection>& newSections) {
    //     std::set<double> attributeChangePositions;
    //     // collect speed change and access restriction positions and apply initial values to the begin
    //     for (std::vector<OpenDriveLane>::iterator k = lanesByDir[OPENDRIVE_TAG_RIGHT].begin(); k != lanesByDir[OPENDRIVE_TAG_RIGHT].end(); ++k) {
    //         for (std::vector<std::pair<double, LaneAttributeChange> >::const_iterator l = (*k).attributeChanges.begin(); l != (*k).attributeChanges.end(); ++l) {
    //             attributeChangePositions.insert((*l).first);
    //             if ((*l).first == 0) {
    //                 (*k).speed = (*l).second.speed;
    //                 (*k).permission = (*k).computePermission(tc, (*l).second.allowed, (*l).second.denied);
    //             }
    //         }
    //     }
    //     for (std::vector<OpenDriveLane>::iterator k = lanesByDir[OPENDRIVE_TAG_LEFT].begin(); k != lanesByDir[OPENDRIVE_TAG_LEFT].end(); ++k) {
    //         for (std::vector<std::pair<double, LaneAttributeChange> >::const_iterator l = (*k).attributeChanges.begin(); l != (*k).attributeChanges.end(); ++l) {
    //             attributeChangePositions.insert((*l).first);
    //             if ((*l).first == 0) {
    //                 (*k).speed = (*l).second.speed;
    //                 (*k).permission = (*k).computePermission(tc, (*l).second.allowed, (*l).second.denied);
    //             }
    //         }
    //     }
    //
    //     // do nothing if there is none
    //     if (attributeChangePositions.size() == 0) {
    //         return false;
    //     }
    //
    //     if (*attributeChangePositions.begin() > 0) {
    //         attributeChangePositions.insert(0);
    //     }
    // #ifdef DEBUG_VARIABLE_SPEED
    //     if (gDebugFlag1) std::cout
    //                 << "  buildSpeedChanges sectionStart=" << s
    //                 << " speedChangePositions=" << joinToString(speedChangePositions, ", ")
    //                 << "\n";
    // #endif
    //     for (std::set<double>::iterator i = attributeChangePositions.begin(); i != attributeChangePositions.end(); ++i) {
    //         if (i == attributeChangePositions.begin()) {
    //             newSections.push_back(*this);
    //         } else {
    //             newSections.push_back(buildLaneSection(tc, *i));
    //         }
    //     }
    //     // propagate speeds and access restrictions
    //     for (int i = 0; i != (int)newSections.size(); ++i) {
    //         for (auto& k : newSections[i].lanesByDir) {
    //             for (int j = 0; j != (int)k.second.size(); ++j) {
    //                 OpenDriveLane& l = k.second[j];
    //                 if (l.speed == 0) {
    //                     if (i > 0) {
    //                         l.speed = newSections[i - 1].lanesByDir[k.first][j].speed;
    //                     } else {
    //                         tc.getEdgeTypeSpeed(l.type);
    //                     }
    //                 }
    //                 if (l.permission == 0) {
    //                     if (i > 0) {
    //                         l.permission = newSections[i - 1].lanesByDir[k.first][j].permission;
    //                         l.type = newSections[i - 1].lanesByDir[k.first][j].type;
    //                     } else {
    //                         tc.getEdgeTypePermissions(l.type);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn build_attribute_changes(&mut self, tc: &NBTypeCont, newSections: &mut Vec<NIImporter_OpenDrive_OpenDriveLaneSection>) -> bool {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        // sorted-set insert, equivalent to std::set<double>::insert
        fn set_insert(v: &mut Vec<f64>, x: f64) {
            if let Err(pos) = v.binary_search_by(|p| p.partial_cmp(&x).unwrap()) {
                v.insert(pos, x);
            }
        }
        let mut attributeChangePositions: Vec<f64> = Vec::new();
        // collect speed change and access restriction positions and apply initial values to the begin
        for k in self.lanesByDir.entry(T::OPENDRIVE_TAG_RIGHT).or_default().iter_mut() {
            for l in k.attributeChanges.iter() {
                set_insert(&mut attributeChangePositions, l.0);
                if l.0 == 0.0 {
                    let perm = k.compute_permission(tc, &l.1.allowed, &l.1.denied);
                    k.speed = l.1.speed;
                    k.permission = perm;
                }
            }
        }
        for k in self.lanesByDir.entry(T::OPENDRIVE_TAG_LEFT).or_default().iter_mut() {
            for l in k.attributeChanges.iter() {
                set_insert(&mut attributeChangePositions, l.0);
                if l.0 == 0.0 {
                    let perm = k.compute_permission(tc, &l.1.allowed, &l.1.denied);
                    k.speed = l.1.speed;
                    k.permission = perm;
                }
            }
        }

        // do nothing if there is none
        if attributeChangePositions.len() == 0 {
            return false;
        }

        if attributeChangePositions[0] > 0.0 {
            set_insert(&mut attributeChangePositions, 0.0);
        }

        for (idx, pos) in attributeChangePositions.iter().enumerate() {
            if idx == 0 {
                newSections.push(self.clone());
            } else {
                let built = self.build_lane_section(tc, *pos);
                newSections.push(built);
            }
        }
        // propagate speeds and access restrictions
        for i in 0..newSections.len() {
            let (before, after) = newSections.split_at_mut(i);
            let cur = &mut after[0];
            let keys: Vec<T> = cur.lanesByDir.keys().cloned().collect();
            for key in keys {
                let n = cur.lanesByDir[&key].len();
                for j in 0..n {
                    let l = &mut cur.lanesByDir.get_mut(&key).unwrap()[j];
                    if l.speed == 0.0 {
                        if i > 0 {
                            l.speed = before[i - 1].lanesByDir.entry(key).or_default()[j].speed;
                        } else {
                            let _ = tc.get_edge_type_speed(&l.type_);
                        }
                    }
                    if l.permission == 0 {
                        if i > 0 {
                            let (prevPerm, prevType) = {
                                let prevLane = &before[i - 1].lanesByDir.entry(key).or_default()[j];
                                (prevLane.permission, prevLane.type_.clone())
                            };
                            l.permission = prevPerm;
                            l.type_ = prevType;
                        } else {
                            let _ = tc.get_edge_type_permissions(&l.type_);
                        }
                    }
                }
            }
        }
        true
    }

    /// C++: `OpenDriveLaneSection OpenDriveLaneSection::buildLaneSection(const NBTypeCont &, double)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_lane_section(&mut self, tc: &NBTypeCont, startPos: f64) -> NIImporter_OpenDrive_OpenDriveLaneSection {
        use NIImporter_OpenDrive_OpenDriveXMLTag as T;
        let mut ret: NIImporter_OpenDrive_OpenDriveLaneSection = self.clone();
        ret.s += startPos;
        for dir in [T::OPENDRIVE_TAG_RIGHT, T::OPENDRIVE_TAG_LEFT] {
            for l in ret.lanesByDir.entry(dir).or_default().iter_mut() {
                l.speed = 0.0;
                l.permission = 0;
                let mut finder = NIImporter_OpenDrive_same_position_finder::new(startPos);
                if let Some(idx) = l.attributeChanges.iter().position(|ps| finder.call_op(ps)) {
                    let ac = &l.attributeChanges[idx].1;
                    let permission = l.compute_permission(tc, &ac.allowed, &ac.denied);
                    let speed = ac.speed;
                    l.speed = speed;
                    l.permission = permission;
                }
            }
        }
        ret
    }

}

/// Representation of an OpenDrive link
/// C++ class `NIImporter_OpenDrive::OpenDriveLink` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveLink {
    pub linkType: NIImporter_OpenDrive_LinkType,
    pub elementID: String,
    pub elementType: NIImporter_OpenDrive_ElementType,
    pub contactPoint: NIImporter_OpenDrive_ContactPoint,
}

impl NIImporter_OpenDrive_OpenDriveLink {
    /// C++: `void OpenDriveLink::OpenDriveLink(LinkType, const std::string &)`
    /// Constructor
    // OpenDriveLink(LinkType linkTypeArg, const std::string& elementIDArg)
    //             : linkType(linkTypeArg), elementID(elementIDArg),
    //               elementType(OPENDRIVE_ET_UNKNOWN), contactPoint(OPENDRIVE_CP_UNKNOWN) { }
    #[allow(unused_variables)]
    pub fn new(linkTypeArg: NIImporter_OpenDrive_LinkType, elementIDArg: &str) -> Self {
        Self {
            linkType: linkTypeArg,
            elementID: elementIDArg.to_string(),
            elementType: NIImporter_OpenDrive_ElementType::OPENDRIVE_ET_UNKNOWN,
            contactPoint: NIImporter_OpenDrive_ContactPoint::OPENDRIVE_CP_UNKNOWN,
        }
    }

}

/// A road object (e.g. parkingSpace)
/// C++ class `NIImporter_OpenDrive::OpenDriveObject` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveObject {
    pub type_: String,
    pub name: String,
    pub id: String,
    pub s: f64,
    pub t: f64,
    pub zOffset: f64,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub radius: f64,
    pub hdg: f64,
    pub pitch: f64,
    pub roll: f64,
}

impl NIImporter_OpenDrive_OpenDriveObject {
}

/// Representation of a signal
/// C++ class `NIImporter_OpenDrive::OpenDriveSignal` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_OpenDriveSignal {
    pub id: String,
    pub type_: String,
    pub name: String,
    pub priority: String,
    pub orientation: i32,
    pub dynamic: bool,
    pub s: f64,
    /// signal validity range
    pub minLane: i32,
    pub maxLane: i32,
    /// the controller ID
    pub controller: String,
}

impl NIImporter_OpenDrive_OpenDriveSignal {
    /// C++: `void OpenDriveSignal::OpenDriveSignal(const std::string &, const std::string, const std::string, int, bool, double)`
    /// Constructor
    // OpenDriveSignal(const std::string& idArg, const std::string typeArg, const std::string nameArg,
    //                         int orientationArg, bool dynamicArg, double sArg) :
    //             id(idArg), type(typeArg), name(nameArg),
    //             orientation(orientationArg), dynamic(dynamicArg), s(sArg),
    //             minLane(-UNSET_LANEVALIDITY), maxLane(UNSET_LANEVALIDITY)
    //         { }
    #[allow(unused_variables)]
    pub fn new(idArg: &str, typeArg: String, nameArg: String, orientationArg: i32, dynamicArg: bool, sArg: f64) -> Self {
        Self {
            id: idArg.to_string(),
            type_: typeArg,
            name: nameArg,
            priority: String::new(),
            orientation: orientationArg,
            dynamic: dynamicArg,
            s: sArg,
            minLane: -UNSET_LANEVALIDITY,
            maxLane: UNSET_LANEVALIDITY,
            controller: String::new(),
        }
    }

    /// C++: `void OpenDriveSignal::OpenDriveSignal()`
    /// dummy constructor for use in maps
    // OpenDriveSignal() {}
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        Self {
            id: String::new(),
            type_: String::new(),
            name: String::new(),
            priority: String::new(),
            orientation: 0,
            dynamic: false,
            s: 0.0,
            minLane: 0,
            maxLane: 0,
            controller: String::new(),
        }
    }

}

/// Coefficients of an elevation profile (3rd degree polynomial)
/// C++ class `NIImporter_OpenDrive::Poly3` (netimport/NIImporter_OpenDrive.h)
#[derive(Clone)]
pub struct NIImporter_OpenDrive_Poly3 {
    pub s: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl NIImporter_OpenDrive_Poly3 {
    /// C++: `void Poly3::Poly3(double, double, double, double, double)`
    /// Constructor
    // Poly3(double _s, double _a, double _b, double _c, double _d) :
    //             s(_s), a(_a), b(_b), c(_c), d(_d) {}
    #[allow(unused_variables)]
    pub fn new(_s: f64, _a: f64, _b: f64, _c: f64, _d: f64) -> Self {
        Self { s: _s, a: _a, b: _b, c: _c, d: _d }
    }

    /// C++: `double Poly3::computeAt(double) const` [const]
    // double computeAt(double pos) const {
    //             const double ds = pos - s;
    //             return a + b * ds + c * ds * ds + d * ds * ds * ds;
    //         }
    #[allow(unused_variables)]
    pub fn compute_at(&self, pos: f64) -> f64 {
        let ds = pos - self.s;
        self.a + self.b * ds + self.c * ds * ds + self.d * ds * ds * ds
    }

}

/// C++ class `NIImporter_OpenDrive::same_position_finder` (netimport/NIImporter_OpenDrive.h)
pub struct NIImporter_OpenDrive_same_position_finder {
    /// The position to search for
    pub myPosition: f64,
}

impl NIImporter_OpenDrive_same_position_finder {
    /// C++: `void same_position_finder::same_position_finder(double)`
    /// constructor
    // explicit same_position_finder(double pos) : myPosition(pos) { }
    #[allow(unused_variables)]
    pub fn new(pos: f64) -> Self {
        Self { myPosition: pos }
    }

    /// C++: `bool same_position_finder::operator()(const std::pair<double, LaneAttributeChange> &)`
    /// the comparing function
    // bool operator()(const std::pair<double, LaneAttributeChange>& ps) {
    //             return ps.first == myPosition;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, ps: &(f64, NIImporter_OpenDrive_LaneAttributeChange)) -> bool {
        ps.0 == self.myPosition
    }

}

/// A class for sorting lane sections by their s-value
/// C++ class `NIImporter_OpenDrive::sections_by_s_sorter` (netimport/NIImporter_OpenDrive.h)
pub struct NIImporter_OpenDrive_sections_by_s_sorter {
}

impl NIImporter_OpenDrive_sections_by_s_sorter {
    /// C++: `void sections_by_s_sorter::sections_by_s_sorter()`
    /// Constructor
    // explicit sections_by_s_sorter() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {}
    }

    /// C++: `int sections_by_s_sorter::operator()(const OpenDriveLaneSection &, const OpenDriveLaneSection &)`
    /// Sorting function; compares OpenDriveLaneSection::s
    // int operator()(const OpenDriveLaneSection& ls1, const OpenDriveLaneSection& ls2) {
    //             return ls1.s < ls2.s;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, ls1: &NIImporter_OpenDrive_OpenDriveLaneSection, ls2: &NIImporter_OpenDrive_OpenDriveLaneSection) -> i32 {
        (ls1.s < ls2.s) as i32
    }

}

/// C++: `bool (const NIImporter_OpenDrive::Connection &, const NIImporter_OpenDrive::Connection &) operator<`
// bool
// operator<(const NIImporter_OpenDrive::Connection& c1, const NIImporter_OpenDrive::Connection& c2) {
//     if (c1.fromEdge != c2.fromEdge) {
//         return c1.fromEdge   < c2.fromEdge;
//     }
//     if (c1.toEdge     != c2.toEdge) {
//         return c1.toEdge     < c2.toEdge;
//     }
//     if (c1.fromLane != c2.fromLane) {
//         return c1.fromLane < c2.fromLane;
//     }
//     return c1.toLane < c2.toLane;
// }
#[allow(unused_variables)]
pub fn lt_op(c1: &NIImporter_OpenDrive_Connection, c2: &NIImporter_OpenDrive_Connection) -> bool {
    if c1.fromEdge != c2.fromEdge {
        return c1.fromEdge < c2.fromEdge;
    }
    if c1.toEdge != c2.toEdge {
        return c1.toEdge < c2.toEdge;
    }
    if c1.fromLane != c2.fromLane {
        return c1.fromLane < c2.fromLane;
    }
    c1.toLane < c2.toLane
}

// std::set<Connection> ordering in C++ is defined by operator<, so Eq/Ord follow lt_op
impl PartialEq for NIImporter_OpenDrive_Connection {
    fn eq(&self, other: &Self) -> bool {
        !lt_op(self, other) && !lt_op(other, self)
    }
}
impl Eq for NIImporter_OpenDrive_Connection {}
impl PartialOrd for NIImporter_OpenDrive_Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NIImporter_OpenDrive_Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if lt_op(self, other) {
            std::cmp::Ordering::Less
        } else if lt_op(other, self) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl SaxHandler for NIImporter_OpenDrive {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_OpenDrive::my_start_element(self, element, attrs)
    }
    fn my_characters(&mut self, element: i32, chars: &str) -> Result<(), ProcessError> {
        NIImporter_OpenDrive::my_characters(self, element, chars);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_OpenDrive::my_end_element(self, element);
        Ok(())
    }
}
