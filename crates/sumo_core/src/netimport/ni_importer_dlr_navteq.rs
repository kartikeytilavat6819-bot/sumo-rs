//! Port of netimport/NIImporter_DlrNavteq.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::sync::atomic::AtomicBool;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, write_message, write_warning};
use utils::common::string_tokenizer::StringTokenizer;
use utils::common::string_utils::{to_double, to_int, to_lower_case};
use utils::common::sumo_vehicle_class::{SVC_PASSENGER, SVC_SHIP, SVC_UNSPECIFIED};
use utils::common::to_string::to_string;
use utils::importio::line_handler::LineHandler;
use utils::importio::line_reader::LineReader;
use utils::options::options_cont::{get_bool, get_string};
use utils::{tlf, write_errorf, write_warningf};

// C++ static members of EdgesHandler and NIImporter_DlrNavteq
const MISSING_COLUMN: i32 = i32::MAX;
pub const GEO_SCALE: &str = "1e-5";
static KEEP_LENGTH: AtomicBool = AtomicBool::new(false);
const UNDEFINED: &str = "-1";

// shortcut: NBEdge's static constants are not ported as statics yet, their C++ values are copied here
const UNSPECIFIED_WIDTH: f64 = -1.0;
const UNSPECIFIED_OFFSET: f64 = 0.0;
const UNSPECIFIED_FRICTION: f64 = 1.0;
const UNSPECIFIED_SPEED: f64 = -1.0;
const UNSPECIFIED_CONTPOS: f64 = -1.0;
const UNSPECIFIED_VISIBILITY_DISTANCE: f64 = -1.0;
const UNSPECIFIED_LOADED_LENGTH: f64 = -1.0;

/// C++ `throw ProcessError(...)`: the handler and loader signatures return no Result, so the error unwinds as a panic carrying the ProcessError
fn throw_error(e: ProcessError) -> ! {
    std::panic::panic_any(e)
}

/// Unwraps a Result whose error the C++ code would have thrown
fn must<T>(r: Result<T, ProcessError>) -> T {
    match r {
        Ok(v) => v,
        Err(e) => throw_error(e),
    }
}

/// `StringUtils::toInt` inside `try { } catch (NumberFormatException&) { throw ProcessError(msg) }`
fn to_int_or_throw(s: &str, msg: String) -> i32 {
    match to_int(s) {
        Ok(v) => v,
        Err(_) => throw_error(ProcessError::new(msg)),
    }
}

/// time(&t): the current time in seconds since the epoch
fn now_time() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// days since 1970-01-01 of the given proleptic Gregorian date
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12; // March = 0
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// (year, month, day) of the given day count since 1970-01-01
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// The YYYY-MM-DD date of a time value (strftime "%Y-%m-%d")
fn format_date(t: i64) -> String {
    let (y, m, d) = civil_from_days(t.div_euclid(86400));
    format!("{y:04}-{m:02}-{d:02}")
}

/// mktime() of a struct tm with zero time of day; tm_year counts from 1900 and out-of-range fields are normalised
/// shortcut: computed in UTC, C mktime() uses the local time zone and there is no time zone database in this port
fn mktime_utc(tm_year: i64, tm_mon: i64, tm_mday: i64) -> i64 {
    let year = 1900 + tm_year + tm_mon.div_euclid(12);
    let month = tm_mon.rem_euclid(12) + 1;
    (days_from_civil(year, month, 1) + tm_mday - 1) * 86400
}

/// sscanf(s, "[(%[^)]){%[^}]}]") for the construction period: (start, duration) when both fields are read
fn scan_construction_period(s: &str) -> Option<(String, String)> {
    let rest = s.strip_prefix("[(")?;
    let close = rest.find(')')?;
    let start = &rest[..close];
    if start.is_empty() {
        return None;
    }
    let rest = rest[close + 1..].strip_prefix('{')?;
    let end = rest.find('}').unwrap_or(rest.len());
    let duration = &rest[..end];
    if duration.is_empty() {
        return None;
    }
    Some((start.to_string(), duration.to_string()))
}

/// sscanf(s, "%i") for the leading integer of s (whitespace, sign, 0x hex and leading 0 octal); None if no number is read
fn scanf_percent_i(s: &str) -> Option<i32> {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() && b[i].is_ascii_whitespace() {
        i += 1;
    }
    let mut neg = false;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        neg = b[i] == b'-';
        i += 1;
    }
    let (radix, skip) = if i + 2 < b.len() && b[i] == b'0' && (b[i + 1] == b'x' || b[i + 1] == b'X') && (b[i + 2] as char).is_ascii_hexdigit() {
        (16, 2)
    } else if i < b.len() && b[i] == b'0' {
        (8, 0)
    } else {
        (10, 0)
    };
    i += skip;
    let start = i;
    let mut value: i64 = 0;
    while i < b.len() && (b[i] as char).is_digit(radix) {
        value = value.wrapping_mul(radix as i64).wrapping_add((b[i] as char).to_digit(radix).unwrap_or(0) as i64);
        i += 1;
    }
    if i == start {
        return None;
    }
    let value = if neg { value.wrapping_neg() } else { value };
    Some(value as i32)
}

/// The parts of std::istringstream used by the node parser: >> std::string, >> int and >> double
struct LineStream<'a> {
    rest: &'a str,
}

impl<'a> LineStream<'a> {
    fn new(s: &'a str) -> Self {
        LineStream { rest: s }
    }

    fn token(&mut self) -> Option<&'a str> {
        let s = self.rest.trim_start();
        if s.is_empty() {
            self.rest = s;
            return None;
        }
        let end = s.find(char::is_whitespace).unwrap_or(s.len());
        self.rest = &s[end..];
        Some(&s[..end])
    }

    fn int(&mut self) -> Option<i32> {
        let s = self.rest.trim_start();
        let b = s.as_bytes();
        let mut i = 0;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let d0 = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == d0 {
            self.rest = s;
            return None;
        }
        let v = s[..i].parse::<i32>().ok()?;
        self.rest = &s[i..];
        Some(v)
    }

    fn double(&mut self) -> Option<f64> {
        let s = self.rest.trim_start();
        let b = s.as_bytes();
        let mut i = 0;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let int_start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        let mut digits = i - int_start;
        if i < b.len() && b[i] == b'.' {
            let f = i + 1;
            let mut k = f;
            while k < b.len() && b[k].is_ascii_digit() {
                k += 1;
            }
            digits += k - f;
            if digits > 0 {
                i = k;
            }
        }
        if digits == 0 {
            self.rest = s;
            return None;
        }
        if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
            let mut k = i + 1;
            if k < b.len() && (b[k] == b'+' || b[k] == b'-') {
                k += 1;
            }
            let ds = k;
            while k < b.len() && b[k].is_ascii_digit() {
                k += 1;
            }
            if k > ds {
                i = k;
            }
        }
        let v = s[..i].parse::<f64>().ok()?;
        self.rest = &s[i..];
        Some(v)
    }
}

/// C++ enum `EdgesHandler::ColumnName`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum EdgesHandler_ColumnName {
    LINK_ID = 0,
    NODE_ID_FROM = 1,
    NODE_ID_TO = 2,
    BETWEEN_NODE_ID = 3,
    LENGTH = 4,
    VEHICLE_TYPE = 5,
    FORM_OF_WAY = 6,
    BRUNNEL_TYPE = 7,
    FUNCTIONAL_ROAD_CLASS = 8,
    SPEED_CATEGORY = 9,
    NUMBER_OF_LANES = 10,
    SPEED_LIMIT = 11,
    SPEED_RESTRICTION = 12,
    NAME_ID1_REGIONAL = 13,
    NAME_ID2_LOCAL = 14,
    HOUSENUMBERS_RIGHT = 15,
    HOUSENUMBERS_LEFT = 16,
    ZIP_CODE = 17,
    AREA_ID = 18,
    SUBAREA_ID = 19,
    THROUGH_TRAFFIC = 20,
    SPECIAL_RESTRICTIONS = 21,
    EXTENDED_NUMBER_OF_LANES = 22,
    ISRAMP = 23,
    CONNECTION = 24,
}

/// Imports prohibitions regarding connectivity
/// C++ class `NIImporter_DlrNavteq::ConnectedLanesHandler` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq_ConnectedLanesHandler {
    // C++ base class `LineHandler` is the Rust trait `LineHandler`: add `impl LineHandler for NIImporter_DlrNavteq_ConnectedLanesHandler`
    /// The edge container to store loaded edges into
    pub myEdgeCont: Ptr<NBEdgeCont>,
}

impl NIImporter_DlrNavteq_ConnectedLanesHandler {
    /// C++: `void ConnectedLanesHandler::ConnectedLanesHandler(NBEdgeCont &)`
    /// Constructor
    // NIImporter_DlrNavteq::ConnectedLanesHandler::ConnectedLanesHandler(
    //     NBEdgeCont& ec) :
    //     myEdgeCont(ec) {
    // }
    #[allow(unused_variables)]
    pub fn new(ne: &mut NBEdgeCont) -> Self {
        NIImporter_DlrNavteq_ConnectedLanesHandler { myEdgeCont: Ptr::from_mut(ne) }
    }

    /// C++: `void ConnectedLanesHandler::~ConnectedLanesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_DlrNavteq::ConnectedLanesHandler::~ConnectedLanesHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool ConnectedLanesHandler::report(const std::string &)` [virtual]
    /// Parsing method
    // bool
    // NIImporter_DlrNavteq::ConnectedLanesHandler::report(const std::string& result) {
    //     if (result[0] == '#') {
    //         return true;
    //     }
    //     StringTokenizer st(result, StringTokenizer::TAB);
    //     if (st.size() == 1) {
    //         return true; // one line with the number of data containing lines in it (also starts with a comment # since ersion 6.5)
    //     }
    //     assert(st.size() >= 7);
    //     const std::string nodeID = st.next();
    //     const std::string vehicleType = st.next();
    //     const std::string fromLaneS = st.next();
    //     const std::string toLaneS = st.next();
    //     const std::string throughTraffic = st.next();
    //     const std::string startEdge = st.next();
    //     const std::string endEdge = st.get(st.size() - 1);
    //
    //     NBEdge* from = myEdgeCont.retrieve(startEdge);
    //     if (from == nullptr) {
    //         WRITE_WARNINGF(TL("Ignoring prohibition from unknown start edge '%'."), startEdge);
    //         return true;
    //     }
    //     NBEdge* to = myEdgeCont.retrieve(endEdge);
    //     if (to == nullptr) {
    //         WRITE_WARNINGF(TL("Ignoring prohibition from unknown end edge '%'."), endEdge);
    //         return true;
    //     }
    //     int fromLane = StringUtils::toInt(fromLaneS) - 1; // one based
    //     if (fromLane < 0 || fromLane >= from->getNumLanes()) {
    //         WRITE_WARNINGF(TL("Ignoring invalid lane index '%' in connection from edge '%' with % lanes."), fromLaneS, startEdge, from->getNumLanes());
    //         return true;
    //     }
    //     int toLane = StringUtils::toInt(toLaneS) - 1; // one based
    //     if (toLane < 0 || toLane >= to->getNumLanes()) {
    //         WRITE_WARNINGF(TL("Ignoring invalid lane index '%' in connection to edge '%' with % lanes"), toLaneS, endEdge, to->getNumLanes());
    //         return true;
    //     }
    //     if (!from->addLane2LaneConnection(fromLane, to, toLane, NBEdge::Lane2LaneInfoType::USER, true)) {
    //         if (OptionsCont::getOptions().getBool("show-errors.connections-first-try")) {
    //             WRITE_WARNINGF(TL("Could not set loaded connection from '%' to '%'."), from->getLaneID(fromLane), to->getLaneID(toLane));
    //         }
    //         // set as to be re-applied after network processing
    //         // if this connection runs across a node cluster it may not be possible to set this
    //         const bool warnOnly = st.size() > 7;
    //         myEdgeCont.addPostProcessConnection(from->getID(), fromLane, to->getID(), toLane, false, KEEPCLEAR_UNSPECIFIED,
    //                                             NBEdge::UNSPECIFIED_CONTPOS, NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE,
    //                                             NBEdge::UNSPECIFIED_SPEED, NBEdge::UNSPECIFIED_FRICTION, NBEdge::UNSPECIFIED_LOADED_LENGTH, PositionVector::EMPTY, false, warnOnly);
    //     }
    //     // ensure that connections for other lanes are guessed if not specified
    //     from->declareConnectionsAsLoaded(NBEdge::EdgeBuildingStep::INIT);
    //     from->getLaneStruct(fromLane).connectionsDone = true;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn report(&mut self, result: &str) -> bool {
        if result.starts_with('#') {
            return true;
        }
        let mut st = StringTokenizer::with_special(result, StringTokenizer::TAB);
        if st.size() == 1 {
            return true; // one line with the number of data containing lines in it (also starts with a comment # since ersion 6.5)
        }
        assert!(st.size() >= 7);
        let nodeID = must(st.next());
        let vehicleType = must(st.next());
        let fromLaneS = must(st.next());
        let toLaneS = must(st.next());
        let throughTraffic = must(st.next());
        let startEdge = must(st.next());
        let endEdge = must(st.get(st.size() - 1));

        let from = self.myEdgeCont.m().retrieve(&startEdge, false);
        if from.is_null() {
            write_warningf!("Ignoring prohibition from unknown start edge '%'.", startEdge);
            return true;
        }
        let to = self.myEdgeCont.m().retrieve(&endEdge, false);
        if to.is_null() {
            write_warningf!("Ignoring prohibition from unknown end edge '%'.", endEdge);
            return true;
        }
        let fromLane = must(to_int(&fromLaneS)) - 1; // one based
        if fromLane < 0 || fromLane >= from.get_num_lanes() {
            write_warningf!("Ignoring invalid lane index '%' in connection from edge '%' with % lanes.", fromLaneS, startEdge, from.get_num_lanes());
            return true;
        }
        let toLane = must(to_int(&toLaneS)) - 1; // one based
        if toLane < 0 || toLane >= to.get_num_lanes() {
            write_warningf!("Ignoring invalid lane index '%' in connection to edge '%' with % lanes", toLaneS, endEdge, to.get_num_lanes());
            return true;
        }
        if !from.m().add_lane2_lane_connection(fromLane, to, toLane, NBEdge_Lane2LaneInfoType::USER, true, false, KeepClear::KEEPCLEAR_UNSPECIFIED, UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_SPEED, UNSPECIFIED_FRICTION, UNSPECIFIED_LOADED_LENGTH, &PositionVector::EMPTY, false, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false) {
            if get_bool("show-errors.connections-first-try") {
                write_warningf!("Could not set loaded connection from '%' to '%'.", from.get_lane_id(fromLane), to.get_lane_id(toLane));
            }
            // set as to be re-applied after network processing
            // if this connection runs across a node cluster it may not be possible to set this
            let warnOnly = st.size() > 7;
            self.myEdgeCont.m().add_post_process_connection(from.get_id(), fromLane, to.get_id(), toLane, false, KeepClear::KEEPCLEAR_UNSPECIFIED,
                UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_SPEED, UNSPECIFIED_FRICTION, UNSPECIFIED_LOADED_LENGTH, &PositionVector::EMPTY, false, warnOnly,
                SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED);
        }
        // ensure that connections for other lanes are guessed if not specified
        from.m().declare_connections_as_loaded(NBEdge_EdgeBuildingStep::INIT);
        // shortcut: NBEdge::get_lane_struct is still todo and nb_edge.rs resolves Lane to libsumo::lane::Lane (no connectionsDone), set Lane.connectionsDone = true here once it is wired
        true
    }

    /// C++: `void ConnectedLanesHandler::ConnectedLanesHandler(const ConnectedLanesHandler &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_DlrNavteq_ConnectedLanesHandler) -> Self {
        panic!("ConnectedLanesHandler copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `ConnectedLanesHandler & ConnectedLanesHandler::operator=(const ConnectedLanesHandler &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_DlrNavteq_ConnectedLanesHandler) -> &mut NIImporter_DlrNavteq_ConnectedLanesHandler {
        panic!("ConnectedLanesHandler assignment operator is invalidated (private and never defined in C++)")
    }

}

/// Importer of edges stored in unsplit elmar format
/// C++ class `NIImporter_DlrNavteq::EdgesHandler` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq_EdgesHandler {
    // C++ base class `LineHandler` is the Rust trait `LineHandler`: add `impl LineHandler for NIImporter_DlrNavteq_EdgesHandler`
    /// The node container to get the referenced nodes from
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The edge container to store loaded edges into
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The type container to retrieve type info from
    pub myTypeCont: Ptr<NBTypeCont>,
    /// Previously read edge geometries (manipulated during use)
    pub myGeoms: Ptr<BTreeMap<String, PositionVector>>,
    /// Previously read streat names (non-const because operate[] is more convenient)
    pub myStreetNames: Ptr<BTreeMap<String, String>>,
    /// Whether node positions shall not be added to the edge's geometry
    pub myTryIgnoreNodePositions: bool,
    /// version number of current file
    pub myVersion: f64,
    /// the version number of the edge file being parsed
    pub myColumns: Vec<i32>,
    /// the file being parsed
    pub myFile: String,
}

// C++ static members of EdgesHandler (port as module-level statics, see AGENTS.md):
//   static const int EdgesHandler::MISSING_COLUMN;

impl NIImporter_DlrNavteq_EdgesHandler {
    /// C++: `void EdgesHandler::EdgesHandler(NBNodeCont &, NBEdgeCont &, NBTypeCont &, const std::string &, std::map<std::string, PositionVector> &, std::map<std::string, std::string> &)`
    /// Constructor
    // NIImporter_DlrNavteq::EdgesHandler::EdgesHandler(NBNodeCont& nc, NBEdgeCont& ec,
    //         NBTypeCont& tc, const std::string& file,
    //         std::map<std::string, PositionVector>& geoms,
    //         std::map<std::string, std::string>& streetNames):
    //     myNodeCont(nc),
    //     myEdgeCont(ec),
    //     myTypeCont(tc),
    //     myGeoms(geoms),
    //     myStreetNames(streetNames),
    //     myVersion(0),
    //     myFile(file) {
    // }
    #[allow(unused_variables)]
    pub fn new(nc: &mut NBNodeCont, ec: &mut NBEdgeCont, tc: &mut NBTypeCont, file: &str, geoms: &mut BTreeMap<String, PositionVector>, streetNames: &mut BTreeMap<String, String>) -> Self {
        NIImporter_DlrNavteq_EdgesHandler {
            myNodeCont: Ptr::from_mut(nc),
            myEdgeCont: Ptr::from_mut(ec),
            myTypeCont: Ptr::from_mut(tc),
            myGeoms: Ptr::from_mut(geoms),
            myStreetNames: Ptr::from_mut(streetNames),
            myTryIgnoreNodePositions: false,
            myVersion: 0.0,
            myColumns: Vec::new(),
            myFile: file.to_string(),
        }
    }

    /// C++: `void EdgesHandler::~EdgesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_DlrNavteq::EdgesHandler::~EdgesHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool EdgesHandler::report(const std::string &)` [virtual]
    /// Parsing method
    // bool
    // NIImporter_DlrNavteq::EdgesHandler::report(const std::string& result) {
    //     // parse version number from first comment line and initialize column definitions
    //     if (result[0] == '#') {
    //         if (!myColumns.empty()) {
    //             return true;
    //         }
    //         const double version = readVersion(result, myFile);
    //         if (version > 0) {
    //             myVersion = version;
    //             // init columns
    //             const int NUM_COLUMNS = 25; // @note arrays must match this size!
    //             const int MC = MISSING_COLUMN;
    //             if (myVersion < 3) {
    //                 const int columns[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, MC, 12, 13, 14, 15, 16, 17, 18, 19, 20, MC, MC, -21};
    //                 myColumns = std::vector<int>(columns, columns + NUM_COLUMNS);
    //             } else if (myVersion < 6) {
    //                 const int columns[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, MC, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, -23};
    //                 myColumns = std::vector<int>(columns, columns + NUM_COLUMNS);
    //             } else {
    //                 const int columns[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24};
    //                 myColumns = std::vector<int>(columns, columns + NUM_COLUMNS);
    //             }
    //         }
    //         return true;
    //     }
    //     if (myColumns.empty()) {
    //         throw ProcessError(TLF("Missing version string in file '%'.", myFile));
    //     }
    //     // interpret link attributes
    //     StringTokenizer st(result, StringTokenizer::WHITECHARS);
    //     const std::string id = getColumn(st, LINK_ID);
    //     // form of way (for priority and permissions)
    //     int form_of_way;
    //     try {
    //         form_of_way = StringUtils::toInt(getColumn(st, FORM_OF_WAY));
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Non-numerical value for form_of_way of link '%'.", id));
    //     }
    //     // brunnel type (bridge/tunnel/ferry (for permissions)
    //     int brunnel_type;
    //     try {
    //         brunnel_type = StringUtils::toInt(getColumn(st, BRUNNEL_TYPE));
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Non-numerical value for brunnel_type of link '%'.", id));
    //     }
    //     // priority based on street_type / frc
    //     int priority;
    //     try {
    //         priority = -StringUtils::toInt(getColumn(st, FUNCTIONAL_ROAD_CLASS));
    //         // lower priority using form_of_way
    //         if (form_of_way == 11) {
    //             priority -= 1; // frontage road, very often with lowered curb
    //         } else if (form_of_way > 11) {
    //             priority -= 2; // parking/service access assume lowered curb
    //         }
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Non-numerical value for street_type of link '%'.", id));
    //     }
    //     // street name
    //     std::string streetName = getStreetNameFromIDs(
    //                                  getColumn(st, NAME_ID1_REGIONAL),
    //                                  getColumn(st, NAME_ID2_LOCAL));
    //     // try to get the nodes
    //     const std::string fromID = getColumn(st, NODE_ID_FROM);
    //     const std::string toID = getColumn(st, NODE_ID_TO);
    //     NBNode* from = myNodeCont.retrieve(fromID);
    //     NBNode* to = myNodeCont.retrieve(toID);
    //     if (from == nullptr) {
    //         throw ProcessError("The from-node '" + fromID + "' of link '" + id + "' could not be found");
    //     }
    //     if (to == nullptr) {
    //         throw ProcessError("The to-node '" + toID + "' of link '" + id + "' could not be found");
    //     }
    //     // speed
    //     double speed;
    //     try {
    //         speed = StringUtils::toInt(getColumn(st, SPEED_RESTRICTION, "-1")) / 3.6;
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Non-numerical value for the SPEED_RESTRICTION of link '%'.", id));
    //     }
    //     if (speed < 0) {
    //         // speed category as fallback
    //         speed = NINavTeqHelper::getSpeed(id, getColumn(st, SPEED_CATEGORY));
    //     }
    //     // number of lanes
    //     int numLanes;
    //     try {
    //         // EXTENDED_NUMBER_OF_LANES is prefered but may not be defined
    //         numLanes = StringUtils::toInt(getColumn(st, EXTENDED_NUMBER_OF_LANES, "-1"));
    //         if (numLanes == -1) {
    //             numLanes = NINavTeqHelper::getLaneNumber(id, getColumn(st, NUMBER_OF_LANES), speed);
    //         }
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Non-numerical value for the number of lanes of link '%'.", id));
    //     }
    //
    //     const std::string navTeqTypeId = getColumn(st, VEHICLE_TYPE) + "_" + getColumn(st, FORM_OF_WAY);
    //     // build the edge
    //     NBEdge* e = nullptr;
    //     const std::string interID = getColumn(st, BETWEEN_NODE_ID);
    //     if (interID == "-1") {
    //         e = new NBEdge(id, from, to, myTypeCont.knows(navTeqTypeId) ? navTeqTypeId : "", speed, NBEdge::UNSPECIFIED_FRICTION, numLanes, priority,
    //                        NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT, streetName);
    //     } else {
    //         PositionVector geoms = myGeoms[interID];
    //         if (getColumn(st, CONNECTION, "0") == "1") {
    //             geoms = geoms.reverse();
    //         }
    //         geoms.insert(geoms.begin(), from->getPosition());
    //         geoms.push_back(to->getPosition());
    //         const std::string origID = OptionsCont::getOptions().getBool("output.original-names") ? id : "";
    //         e = new NBEdge(id, from, to, myTypeCont.knows(navTeqTypeId) ? navTeqTypeId : "", speed, NBEdge::UNSPECIFIED_FRICTION, numLanes, priority,
    //                        NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, geoms, LaneSpreadFunction::CENTER, streetName, origID);
    //     }
    //
    //     //Import length or edges from input file
    //     if (keepLength) {
    //         const double length = StringUtils::toDouble(getColumn(st, LENGTH));
    //         e->setLoadedLength(length);
    //     }
    //
    //     // NavTeq imports can be done with a typemap (if supplied), if not, the old defaults are used
    //     if (myTypeCont.knows(navTeqTypeId)) {
    //         e->setPermissions(myTypeCont.getEdgeTypePermissions(navTeqTypeId));
    //     } else {
    //         // add vehicle type information to the edge
    //         const SVCPermissions allPermissions = myTypeCont.getEdgeTypePermissions("");
    //         const SVCPermissions defaultPermissions = OptionsCont::getOptions().getBool("dlr-navteq.tolerant-permissions") ? allPermissions : 0;
    //         if (myVersion < 6.0) {
    //             NINavTeqHelper::addVehicleClasses(*e, getColumn(st, VEHICLE_TYPE), allPermissions, defaultPermissions);
    //         } else {
    //             NINavTeqHelper::addVehicleClassesV6(*e, getColumn(st, VEHICLE_TYPE), allPermissions, defaultPermissions);
    //         }
    //         // permission modifications based on form_of_way
    //         if (form_of_way == 14) { // pedestrian area (fussgaengerzone)
    //             // unfortunately, the veh_type string is misleading in this case
    //             e->disallowVehicleClass(-1, SVC_PASSENGER);
    //         }
    //         // permission modifications based on brunnel_type
    //         if (brunnel_type == 10) { // ferry
    //             e->setPermissions(SVC_SHIP, -1);
    //         }
    //     }
    //
    //     // insert the edge to the network
    //     if (!myEdgeCont.insert(e)) {
    //         delete e;
    //         throw ProcessError(TLF("Could not add edge '%'.", id));
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn report(&mut self, result: &str) -> bool {
        // parse version number from first comment line and initialize column definitions
        if result.starts_with('#') {
            if !self.myColumns.is_empty() {
                return true;
            }
            let version = NIImporter_DlrNavteq::read_version(result, &self.myFile);
            if version > 0.0 {
                self.myVersion = version;
                // init columns
                let MC = MISSING_COLUMN;
                if self.myVersion < 3.0 {
                    let columns: [i32; 25] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, MC, 12, 13, 14, 15, 16, 17, 18, 19, 20, MC, MC, -21];
                    self.myColumns = columns.to_vec();
                } else if self.myVersion < 6.0 {
                    let columns: [i32; 25] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, MC, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, -23];
                    self.myColumns = columns.to_vec();
                } else {
                    let columns: [i32; 25] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];
                    self.myColumns = columns.to_vec();
                }
            }
            return true;
        }
        if self.myColumns.is_empty() {
            throw_error(ProcessError::new(tlf!("Missing version string in file '%'.", self.myFile)));
        }
        // interpret link attributes
        let st = StringTokenizer::with_special(result, StringTokenizer::WHITECHARS);
        let id = self.get_column(&st, EdgesHandler_ColumnName::LINK_ID, String::new());
        // form of way (for priority and permissions)
        let col = self.get_column(&st, EdgesHandler_ColumnName::FORM_OF_WAY, String::new());
        let form_of_way = to_int_or_throw(&col, tlf!("Non-numerical value for form_of_way of link '%'.", id));
        // brunnel type (bridge/tunnel/ferry (for permissions)
        let col = self.get_column(&st, EdgesHandler_ColumnName::BRUNNEL_TYPE, String::new());
        let brunnel_type = to_int_or_throw(&col, tlf!("Non-numerical value for brunnel_type of link '%'.", id));
        // priority based on street_type / frc
        let col = self.get_column(&st, EdgesHandler_ColumnName::FUNCTIONAL_ROAD_CLASS, String::new());
        let mut priority = -to_int_or_throw(&col, tlf!("Non-numerical value for street_type of link '%'.", id));
        // lower priority using form_of_way
        if form_of_way == 11 {
            priority -= 1; // frontage road, very often with lowered curb
        } else if form_of_way > 11 {
            priority -= 2; // parking/service access assume lowered curb
        }
        // street name
        let regional = self.get_column(&st, EdgesHandler_ColumnName::NAME_ID1_REGIONAL, String::new());
        let local = self.get_column(&st, EdgesHandler_ColumnName::NAME_ID2_LOCAL, String::new());
        let streetName = self.get_street_name_from_i_ds(&regional, &local);
        // try to get the nodes
        let fromID = self.get_column(&st, EdgesHandler_ColumnName::NODE_ID_FROM, String::new());
        let toID = self.get_column(&st, EdgesHandler_ColumnName::NODE_ID_TO, String::new());
        let from = self.myNodeCont.retrieve(&fromID);
        let to = self.myNodeCont.retrieve(&toID);
        if from.is_null() {
            throw_error(ProcessError::new(format!("The from-node '{fromID}' of link '{id}' could not be found")));
        }
        if to.is_null() {
            throw_error(ProcessError::new(format!("The to-node '{toID}' of link '{id}' could not be found")));
        }
        // speed
        let col = self.get_column(&st, EdgesHandler_ColumnName::SPEED_RESTRICTION, "-1".to_string());
        let mut speed = to_int_or_throw(&col, tlf!("Non-numerical value for the SPEED_RESTRICTION of link '%'.", id)) as f64 / 3.6;
        if speed < 0.0 {
            // speed category as fallback
            let col = self.get_column(&st, EdgesHandler_ColumnName::SPEED_CATEGORY, String::new());
            speed = NINavTeqHelper::get_speed(&id, &col);
        }
        // number of lanes
        // EXTENDED_NUMBER_OF_LANES is prefered but may not be defined
        let col = self.get_column(&st, EdgesHandler_ColumnName::EXTENDED_NUMBER_OF_LANES, "-1".to_string());
        let mut numLanes = to_int_or_throw(&col, tlf!("Non-numerical value for the number of lanes of link '%'.", id));
        if numLanes == -1 {
            let col = self.get_column(&st, EdgesHandler_ColumnName::NUMBER_OF_LANES, String::new());
            numLanes = NINavTeqHelper::get_lane_number(&id, &col, speed);
        }

        let vehicle_type = self.get_column(&st, EdgesHandler_ColumnName::VEHICLE_TYPE, String::new());
        let form = self.get_column(&st, EdgesHandler_ColumnName::FORM_OF_WAY, String::new());
        let navTeqTypeId = format!("{vehicle_type}_{form}");
        // build the edge
        let interID = self.get_column(&st, EdgesHandler_ColumnName::BETWEEN_NODE_ID, String::new());
        let e: Ptr<NBEdge>;
        if interID == "-1" {
            let type_ = if self.myTypeCont.knows(&navTeqTypeId) { navTeqTypeId.clone() } else { String::new() };
            e = Ptr::from_mut(Box::leak(NBEdge::new(&id, from, to, type_, speed, UNSPECIFIED_FRICTION, numLanes, priority, UNSPECIFIED_WIDTH, UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT, &streetName)));
        } else {
            let mut geoms: PositionVector = self.myGeoms.m().entry(interID.clone()).or_default().clone();
            if self.get_column(&st, EdgesHandler_ColumnName::CONNECTION, "0".to_string()) == "1" {
                geoms = geoms.reverse();
            }
            geoms.insert(0, *from.get_position());
            geoms.push(*to.get_position());
            let origID = if get_bool("output.original-names") { id.clone() } else { String::new() };
            let type_ = if self.myTypeCont.knows(&navTeqTypeId) { navTeqTypeId.clone() } else { String::new() };
            e = Ptr::from_mut(Box::leak(NBEdge::new_2(&id, from, to, type_, speed, UNSPECIFIED_FRICTION, numLanes, priority, UNSPECIFIED_WIDTH, UNSPECIFIED_OFFSET, geoms, LaneSpreadFunction::CENTER, &streetName, &origID, false)));
        }

        //Import length or edges from input file
        if KEEP_LENGTH.load(std::sync::atomic::Ordering::Relaxed) {
            let col = self.get_column(&st, EdgesHandler_ColumnName::LENGTH, String::new());
            let length = must(to_double(&col));
            e.m().set_loaded_length(length);
        }

        // NavTeq imports can be done with a typemap (if supplied), if not, the old defaults are used
        if self.myTypeCont.knows(&navTeqTypeId) {
            e.m().set_permissions(self.myTypeCont.get_edge_type_permissions(&navTeqTypeId), -1);
        } else {
            // add vehicle type information to the edge
            let allPermissions = self.myTypeCont.get_edge_type_permissions("");
            let defaultPermissions = if get_bool("dlr-navteq.tolerant-permissions") { allPermissions } else { 0 };
            if self.myVersion < 6.0 {
                NINavTeqHelper::add_vehicle_classes(e.m(), &vehicle_type, allPermissions, defaultPermissions);
            } else {
                NINavTeqHelper::add_vehicle_classes_v6(e.m(), &vehicle_type, allPermissions, defaultPermissions);
            }
            // permission modifications based on form_of_way
            if form_of_way == 14 {
                // pedestrian area (fussgaengerzone)
                // unfortunately, the veh_type string is misleading in this case
                e.m().disallow_vehicle_class(-1, SVC_PASSENGER);
            }
            // permission modifications based on brunnel_type
            if brunnel_type == 10 {
                // ferry
                e.m().set_permissions(SVC_SHIP, -1);
            }
        }

        // insert the edge to the network
        if !self.myEdgeCont.m().insert(e, false) {
            // delete e
            drop(unsafe { Box::from_raw(e.as_raw()) });
            throw_error(ProcessError::new(tlf!("Could not add edge '%'.", id)));
        }
        true
    }

    /// C++: `std::string EdgesHandler::getColumn(const StringTokenizer &, ColumnName, const std::string)` [protected]
    // std::string
    // NIImporter_DlrNavteq::EdgesHandler::getColumn(const StringTokenizer& st, ColumnName name, const std::string fallback) {
    //     assert(!myColumns.empty());
    //     if (myColumns[name] == MISSING_COLUMN) {
    //         if (fallback == "") {
    //             throw ProcessError(TLF("Missing column %.", toString(name)));
    //         } else {
    //             return fallback;
    //         }
    //     } else if (myColumns[name] >= 0) {
    //         return st.get((int)(myColumns[name]));
    //     } else {
    //         // negative column number implies an optional column
    //         if ((int) st.size() <= -myColumns[name]) {
    //             // the column is not present
    //             if (fallback == "") {
    //                 throw ProcessError(TLF("Missing optional column % without default value.", toString(name)));
    //             } else {
    //                 return fallback;
    //             }
    //         } else {
    //             return st.get((int)(-myColumns[name]));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_column(&mut self, st: &StringTokenizer, name: EdgesHandler_ColumnName, fallback: String) -> String {
        assert!(!self.myColumns.is_empty());
        let col = self.myColumns[name as usize];
        if col == MISSING_COLUMN {
            if fallback.is_empty() {
                throw_error(ProcessError::new(tlf!("Missing column %.", name as i64)));
            } else {
                return fallback;
            }
        } else if col >= 0 {
            return must(st.get(col as usize));
        } else {
            // negative column number implies an optional column
            if st.size() as i64 <= -(col as i64) {
                // the column is not present
                if fallback.is_empty() {
                    throw_error(ProcessError::new(tlf!("Missing optional column % without default value.", name as i64)));
                } else {
                    return fallback;
                }
            } else {
                return must(st.get((-col) as usize));
            }
        }
    }

    /// C++: `std::string EdgesHandler::getStreetNameFromIDs(const std::string &, const std::string &) const` [const, private]
    /// build the street name for the given ids
    // std::string
    // NIImporter_DlrNavteq::EdgesHandler::getStreetNameFromIDs(
    //     const std::string& regionalID, const std::string& localID) const {
    //     std::string result = "";
    //     bool hadRegional = false;
    //     if (myStreetNames.count(regionalID) > 0) {
    //         hadRegional = true;
    //         result += myStreetNames[regionalID];
    //     }
    //     if (myStreetNames.count(localID) > 0) {
    //         if (hadRegional) {
    //             result += " / ";
    //         }
    //         result += myStreetNames[localID];
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_street_name_from_i_ds(&self, regionalID: &str, localID: &str) -> String {
        let names = self.myStreetNames.m();
        let mut result = String::new();
        let mut hadRegional = false;
        if let Some(s) = names.get(regionalID) {
            hadRegional = true;
            result += s.as_str();
        }
        if let Some(s) = names.get(localID) {
            if hadRegional {
                result += " / ";
            }
            result += s.as_str();
        }
        result
    }

    /// C++: `void EdgesHandler::EdgesHandler(const EdgesHandler &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_DlrNavteq_EdgesHandler) -> Self {
        panic!("EdgesHandler copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `EdgesHandler & EdgesHandler::operator=(const EdgesHandler &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_DlrNavteq_EdgesHandler) -> &mut NIImporter_DlrNavteq_EdgesHandler {
        panic!("EdgesHandler assignment operator is invalidated (private and never defined in C++)")
    }

}

/// Importer for networks stored in Elmar's format
/// C++ class `NIImporter_DlrNavteq` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq {
}

// C++ static members of NIImporter_DlrNavteq (port as module-level statics, see AGENTS.md):
//   static const std::string NIImporter_DlrNavteq::GEO_SCALE;
//   static bool NIImporter_DlrNavteq::keepLength;
//   static const std::string NIImporter_DlrNavteq::UNDEFINED;

impl NIImporter_DlrNavteq {
    /// C++: `void NIImporter_DlrNavteq::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads content of the optionally given dlr-navteq (aka Elmar-fomat) folder
    // void
    // NIImporter_DlrNavteq::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether the option is set (properly)
    //     if (!oc.isSet("dlr-navteq-prefix")) {
    //         return;
    //     }
    //     time_t csTime;
    //     time(&csTime);
    //     keepLength = oc.getBool("dlr-navteq.keep-length");
    //     // parse file(s)
    //     LineReader lr;
    //     // load nodes
    //     std::map<std::string, PositionVector> myGeoms;
    //     PROGRESS_BEGIN_MESSAGE(TL("Loading nodes"));
    //     std::string file = oc.getString("dlr-navteq-prefix") + "_nodes_unsplitted.txt";
    //     NodesHandler handler1(nb.getNodeCont(), file, myGeoms);
    //     if (!lr.setFile(file)) {
    //         throw ProcessError(TLF("The file '%' could not be opened.", file));
    //     }
    //     lr.readAll(handler1);
    //     PROGRESS_DONE_MESSAGE();
    //
    //     // load street names if given and wished
    //     std::map<std::string, std::string> streetNames; // nameID : name
    //     if (oc.getBool("output.street-names")) {
    //         file = oc.getString("dlr-navteq-prefix") + "_names.txt";
    //         if (lr.setFile(file)) {
    //             PROGRESS_BEGIN_MESSAGE(TL("Loading street names"));
    //             NamesHandler handler4(file, streetNames);
    //             lr.readAll(handler4);
    //             PROGRESS_DONE_MESSAGE();
    //         } else {
    //             WRITE_WARNINGF(TL("Output will not contain street names because the file '%' was not found"), file);
    //         }
    //     }
    //
    //     // load edges
    //     PROGRESS_BEGIN_MESSAGE(TL("Loading edges"));
    //     file = oc.getString("dlr-navteq-prefix") + "_links_unsplitted.txt";
    //     // parse the file
    //     EdgesHandler handler2(nb.getNodeCont(), nb.getEdgeCont(), nb.getTypeCont(), file, myGeoms, streetNames);
    //     if (!lr.setFile(file)) {
    //         throw ProcessError(TLF("The file '%' could not be opened.", file));
    //     }
    //     lr.readAll(handler2);
    //     nb.getEdgeCont().recheckLaneSpread();
    //     PROGRESS_DONE_MESSAGE();
    //
    //     // load traffic lights if given
    //     file = oc.getString("dlr-navteq-prefix") + "_traffic_signals.txt";
    //     if (lr.setFile(file)) {
    //         PROGRESS_BEGIN_MESSAGE(TL("Loading traffic lights"));
    //         TrafficlightsHandler handler3(nb.getNodeCont(), nb.getTLLogicCont(), nb.getEdgeCont(), file);
    //         lr.readAll(handler3);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //
    //     // load prohibited manoeuvres if given
    //     file = oc.getString("dlr-navteq-prefix") + "_prohibited_manoeuvres.txt";
    //     if (lr.setFile(file)) {
    //         PROGRESS_BEGIN_MESSAGE(TL("Loading prohibited manoeuvres"));
    //         ProhibitionHandler handler6(nb.getEdgeCont(), file, csTime);
    //         lr.readAll(handler6);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //
    //     // load connected lanes if given
    //     file = oc.getString("dlr-navteq-prefix") + "_connected_lanes.txt";
    //     if (lr.setFile(file)) {
    //         PROGRESS_BEGIN_MESSAGE(TL("Loading connected lanes"));
    //         ConnectedLanesHandler handler7(nb.getEdgeCont());
    //         lr.readAll(handler7);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //
    //     // load time restrictions if given
    //     file = oc.getString("dlr-navteq-prefix") + "_links_timerestrictions.txt";
    //     if (lr.setFile(file)) {
    //         PROGRESS_BEGIN_MESSAGE(TL("Loading time restrictions"));
    //         if (!oc.isDefault("construction-date")) {
    //             csTime = readDate(oc.getString("construction-date"));
    //         }
    //         TimeRestrictionsHandler handler5(nb.getEdgeCont(), nb.getDistrictCont(), csTime);
    //         lr.readAll(handler5);
    //         handler5.printSummary();
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        // check whether the option is set (properly)
        if !must(oc.is_set("dlr-navteq-prefix", true)) {
            return;
        }
        let mut csTime: i64 = now_time();
        KEEP_LENGTH.store(must(oc.get_bool("dlr-navteq.keep-length")), std::sync::atomic::Ordering::Relaxed);
        // parse file(s)
        let mut lr = LineReader::new();
        // load nodes
        let mut myGeoms: BTreeMap<String, PositionVector> = BTreeMap::new();
        progress_begin_message("Loading nodes");
        let mut file = must(oc.get_string("dlr-navteq-prefix")) + "_nodes_unsplitted.txt";
        let mut handler1 = NIImporter_DlrNavteq_NodesHandler::new(nb.get_node_cont(), &file, &mut myGeoms);
        if !lr.set_file(&file) {
            throw_error(ProcessError::new(tlf!("The file '%' could not be opened.", file)));
        }
        lr.read_all(&mut handler1);
        progress_done_message();

        // load street names if given and wished
        let mut streetNames: BTreeMap<String, String> = BTreeMap::new(); // nameID : name
        if must(oc.get_bool("output.street-names")) {
            file = must(oc.get_string("dlr-navteq-prefix")) + "_names.txt";
            if lr.set_file(&file) {
                progress_begin_message("Loading street names");
                let mut handler4 = NIImporter_DlrNavteq_NamesHandler::new(&file, &mut streetNames);
                lr.read_all(&mut handler4);
                progress_done_message();
            } else {
                write_warningf!("Output will not contain street names because the file '%' was not found", file);
            }
        }

        // load edges
        progress_begin_message("Loading edges");
        file = must(oc.get_string("dlr-navteq-prefix")) + "_links_unsplitted.txt";
        // parse the file
        let nc = Ptr::from_mut(nb.get_node_cont());
        let ec = Ptr::from_mut(nb.get_edge_cont());
        let tc = Ptr::from_mut(nb.get_type_cont());
        let mut handler2 = NIImporter_DlrNavteq_EdgesHandler::new(nc.m(), ec.m(), tc.m(), &file, &mut myGeoms, &mut streetNames);
        if !lr.set_file(&file) {
            throw_error(ProcessError::new(tlf!("The file '%' could not be opened.", file)));
        }
        lr.read_all(&mut handler2);
        ec.m().recheck_lane_spread();
        progress_done_message();

        // load traffic lights if given
        file = must(oc.get_string("dlr-navteq-prefix")) + "_traffic_signals.txt";
        if lr.set_file(&file) {
            progress_begin_message("Loading traffic lights");
            let mut handler3 = NIImporter_DlrNavteq_TrafficlightsHandler::new(nc.m(), nb.get_tl_logic_cont(), ec.m(), &file);
            lr.read_all(&mut handler3);
            progress_done_message();
        }

        // load prohibited manoeuvres if given
        file = must(oc.get_string("dlr-navteq-prefix")) + "_prohibited_manoeuvres.txt";
        if lr.set_file(&file) {
            progress_begin_message("Loading prohibited manoeuvres");
            let mut handler6 = NIImporter_DlrNavteq_ProhibitionHandler::new(ec.m(), &file, csTime);
            lr.read_all(&mut handler6);
            progress_done_message();
        }

        // load connected lanes if given
        file = must(oc.get_string("dlr-navteq-prefix")) + "_connected_lanes.txt";
        if lr.set_file(&file) {
            progress_begin_message("Loading connected lanes");
            let mut handler7 = NIImporter_DlrNavteq_ConnectedLanesHandler::new(ec.m());
            lr.read_all(&mut handler7);
            progress_done_message();
        }

        // load time restrictions if given
        file = must(oc.get_string("dlr-navteq-prefix")) + "_links_timerestrictions.txt";
        if lr.set_file(&file) {
            progress_begin_message("Loading time restrictions");
            if !oc.is_default("construction-date") {
                csTime = NIImporter_DlrNavteq::read_date(&must(oc.get_string("construction-date")));
            }
            let mut handler5 = NIImporter_DlrNavteq_TimeRestrictionsHandler::new(ec.m(), nb.get_district_cont(), csTime);
            lr.read_all(&mut handler5);
            handler5.print_summary();
            progress_done_message();
        }
    }

    /// C++: `double NIImporter_DlrNavteq::readVersion(const std::string &, const std::string &)` [static, protected]
    // double
    // NIImporter_DlrNavteq::readVersion(const std::string& line, const std::string& file) {
    //     assert(line[0] == '#');
    //     const std::string marker = "extraction version: v";
    //     const std::string lowerCase = StringUtils::to_lower_case(line);
    //     if (lowerCase.find(marker) == std::string::npos) {
    //         return -1;
    //     }
    //     const int vStart = (int)(lowerCase.find(marker) + marker.size());
    //     const int vEnd = (int)line.find(" ", vStart);
    //     try {
    //         const double version = StringUtils::toDouble(line.substr(vStart, vEnd - vStart));
    //         if (version < 0) {
    //             throw ProcessError("Invalid version number '" + toString(version) + "' in file '" + file + "'.");
    //         }
    //         return version;
    //     } catch (NumberFormatException&) {
    //         throw ProcessError("Non-numerical value '" + line.substr(vStart, vEnd - vStart) + "' for version string in file '" + file + "'.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn read_version(line: &str, file: &str) -> f64 {
        assert!(line.starts_with('#'));
        let marker = "extraction version: v";
        let lowerCase = to_lower_case(line);
        let found = match lowerCase.find(marker) {
            Some(p) => p,
            None => return -1.0,
        };
        let vStart = found + marker.len();
        let vEnd = match line[vStart..].find(' ') {
            Some(p) => vStart + p,
            None => line.len(), // npos: the rest of the line
        };
        let sub = &line[vStart..vEnd];
        let version = match to_double(sub) {
            Ok(v) => v,
            Err(_) => throw_error(ProcessError::new(format!("Non-numerical value '{sub}' for version string in file '{file}'."))),
        };
        if version < 0.0 {
            throw_error(ProcessError::new(format!("Invalid version number '{}' in file '{file}'.", to_string(&version))));
        }
        version
    }

    /// C++: `int NIImporter_DlrNavteq::readPrefixedInt(const std::string &, const std::string &, int)` [static, protected]
    // int
    // NIImporter_DlrNavteq::readPrefixedInt(const std::string& s, const std::string& prefix, int fallBack) {
    //     int result = fallBack;
    //     const size_t pos = s.find(prefix);
    //     if (pos != std::string::npos) {
    //         sscanf(s.substr(pos + prefix.size()).c_str(), "%i", &result);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn read_prefixed_int(s: &str, prefix: &str, fallBack: i32) -> i32 {
        let mut result = fallBack;
        if let Some(pos) = s.find(prefix) {
            if let Some(v) = scanf_percent_i(&s[pos + prefix.len()..]) {
                result = v;
            }
        }
        result
    }

    /// C++: `time_t NIImporter_DlrNavteq::readTimeRec(const std::string &, const std::string &)` [static, protected]
    // time_t
    // NIImporter_DlrNavteq::readTimeRec(const std::string& start, const std::string& duration) {
    //     // http://www.cplusplus.com/reference/ctime/mktime/
    //     struct tm timeinfo;
    //     timeinfo.tm_hour = 0;
    //     timeinfo.tm_min = 0;
    //     timeinfo.tm_sec = 0;
    //     timeinfo.tm_year = 0;
    //     timeinfo.tm_mon = 0;
    //     timeinfo.tm_mday = 1;
    //     timeinfo.tm_wday = 0;
    //     timeinfo.tm_yday = 0;
    //     timeinfo.tm_isdst = 0;
    //
    //     timeinfo.tm_year = NIImporter_DlrNavteq::read_prefixed_int(start, "y") + NIImporter_DlrNavteq::read_prefixed_int(duration, "y") - 1900;
    //     timeinfo.tm_mon = NIImporter_DlrNavteq::read_prefixed_int(start, "M") + NIImporter_DlrNavteq::read_prefixed_int(duration, "M") - 1;
    //     timeinfo.tm_mday = 7 * (NIImporter_DlrNavteq::read_prefixed_int(start, "w") + NIImporter_DlrNavteq::read_prefixed_int(duration, "w"));
    //     timeinfo.tm_mday += NIImporter_DlrNavteq::read_prefixed_int(start, "d") + NIImporter_DlrNavteq::read_prefixed_int(duration, "d");
    //
    //     time_t result =  mktime(&timeinfo);
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn read_time_rec(start: &str, duration: &str) -> i64 {
        // shortcut: C mktime() works in the local time zone, this port computes UTC (no time zone database), add the local offset when one is available
        let year = NIImporter_DlrNavteq::read_prefixed_int(start, "y", 0) as i64 + NIImporter_DlrNavteq::read_prefixed_int(duration, "y", 0) as i64;
        let mon = NIImporter_DlrNavteq::read_prefixed_int(start, "M", 0) as i64 + NIImporter_DlrNavteq::read_prefixed_int(duration, "M", 0) as i64;
        let mut mday = 7 * (NIImporter_DlrNavteq::read_prefixed_int(start, "w", 0) as i64 + NIImporter_DlrNavteq::read_prefixed_int(duration, "w", 0) as i64);
        mday += NIImporter_DlrNavteq::read_prefixed_int(start, "d", 0) as i64 + NIImporter_DlrNavteq::read_prefixed_int(duration, "d", 0) as i64;
        mktime_utc(year - 1900, mon - 1, mday)
    }

    /// C++: `time_t NIImporter_DlrNavteq::readDate(const std::string &)` [static, protected]
    // time_t
    // NIImporter_DlrNavteq::readDate(const std::string& yyyymmdd) {
    //     struct tm timeinfo;
    //     timeinfo.tm_hour = 0;
    //     timeinfo.tm_min = 0;
    //     timeinfo.tm_sec = 0;
    //     timeinfo.tm_wday = 0;
    //     timeinfo.tm_yday = 0;
    //     timeinfo.tm_isdst = 0;
    //
    //     if (yyyymmdd.size() == 10
    //             && yyyymmdd[4] == '-'
    //             && yyyymmdd[7] == '-') {
    //         try {
    //             timeinfo.tm_year = StringUtils::toInt(yyyymmdd.substr(0, 4)) - 1900;
    //             timeinfo.tm_mon = StringUtils::toInt(yyyymmdd.substr(5, 2)) - 1;
    //             timeinfo.tm_mday = StringUtils::toInt(yyyymmdd.substr(8, 2));
    //             return mktime(&timeinfo);
    //         } catch (...) {
    //         }
    //     }
    //     WRITE_ERRORF(TL("Could not parse YYYY-MM-DD date '%'"), yyyymmdd);
    //     time_t now;
    //     time(&now);
    //     return now;
    // }
    #[allow(unused_variables)]
    pub fn read_date(yyyymmdd: &str) -> i64 {
        // shortcut: C mktime() works in the local time zone, this port computes UTC (no time zone database), add the local offset when one is available
        if yyyymmdd.len() == 10 && yyyymmdd.as_bytes()[4] == b'-' && yyyymmdd.as_bytes()[7] == b'-' {
            if let (Some(y), Some(m), Some(d)) = (yyyymmdd.get(0..4), yyyymmdd.get(5..7), yyyymmdd.get(8..10)) {
                if let (Ok(y), Ok(m), Ok(d)) = (to_int(y), to_int(m), to_int(d)) {
                    return mktime_utc(y as i64 - 1900, m as i64 - 1, d as i64);
                }
            }
        }
        write_errorf!("Could not parse YYYY-MM-DD date '%'", yyyymmdd);
        now_time()
    }

}

/// Importer of street names in DLRNavteq's (aka elmar) format
/// C++ class `NIImporter_DlrNavteq::NamesHandler` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq_NamesHandler {
    // C++ base class `LineHandler` is the Rust trait `LineHandler`: add `impl LineHandler for NIImporter_DlrNavteq_NamesHandler`
    /// The container for storing read names
    pub myStreetNames: Ptr<BTreeMap<String, String>>,
}

impl NIImporter_DlrNavteq_NamesHandler {
    /// C++: `void NamesHandler::NamesHandler(const std::string &, std::map<std::string, std::string> &)`
    /// Constructor
    // NIImporter_DlrNavteq::NamesHandler::NamesHandler(
    //     const std::string& file, std::map<std::string, std::string>& streetNames) :
    //     myStreetNames(streetNames) {
    //     UNUSED_PARAMETER(file);
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, streetNames: &mut BTreeMap<String, String>) -> Self {
        NIImporter_DlrNavteq_NamesHandler {
            myStreetNames: Ptr::from_mut(streetNames),
        }
    }

    /// C++: `void NamesHandler::~NamesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_DlrNavteq::NamesHandler::~NamesHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NamesHandler::report(const std::string &)` [virtual]
    /// Parsing method
    // bool
    // NIImporter_DlrNavteq::NamesHandler::report(const std::string& result) {
    // // # NAME_ID    Name
    //     if (result[0] == '#') {
    //         return true;
    //     }
    //     StringTokenizer st(result, StringTokenizer::TAB);
    //     if (st.size() == 1) {
    //         return true; // one line with the number of data containing lines in it (also starts with a comment # since ersion 6.5)
    //     }
    //     assert(st.size() >= 2);
    //     const std::string id = st.next();
    //     if (st.size() > 2) {
    //         const std::string permanent_id_info = st.next();
    //     }
    //     myStreetNames[id] = st.next();
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn report(&mut self, result: &str) -> bool {
        if result.starts_with('#') {
            return true;
        }
        let mut st = StringTokenizer::with_special(result, StringTokenizer::TAB);
        if st.size() == 1 {
            return true; // one line with the number of data containing lines in it (also starts with a comment # since ersion 6.5)
        }
        assert!(st.size() >= 2);
        let id = must(st.next());
        if st.size() > 2 {
            let _permanent_id_info = must(st.next());
        }
        let name = must(st.next());
        self.myStreetNames.m().insert(id, name);
        true
    }

    /// C++: `void NamesHandler::NamesHandler(const NamesHandler &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_DlrNavteq_NamesHandler) -> Self {
        panic!("NamesHandler copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `NamesHandler & NamesHandler::operator=(const NamesHandler &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_DlrNavteq_NamesHandler) -> &mut NIImporter_DlrNavteq_NamesHandler {
        panic!("NamesHandler assignment operator is invalidated (private and never defined in C++)")
    }

}

/// Importer of nodes stored in unsplit elmar format
/// C++ class `NIImporter_DlrNavteq::NodesHandler` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq_NodesHandler {
    // C++ base class `LineHandler` is the Rust trait `LineHandler`: add `impl LineHandler for NIImporter_DlrNavteq_NodesHandler`
    pub myNodeCont: Ptr<NBNodeCont>,
    /// A container for parsed geometries
    pub myGeoms: Ptr<BTreeMap<String, PositionVector>>,
}

impl NIImporter_DlrNavteq_NodesHandler {
    /// C++: `void NodesHandler::NodesHandler(NBNodeCont &, const std::string &, std::map<std::string, PositionVector> &)`
    /// Constructor
    // NIImporter_DlrNavteq::NodesHandler::NodesHandler(NBNodeCont& nc,
    //         const std::string& file,
    //         std::map<std::string, PositionVector>& geoms)
    //     : myNodeCont(nc), myGeoms(geoms) {
    //     UNUSED_PARAMETER(file);
    // }
    #[allow(unused_variables)]
    pub fn new(nc: &mut NBNodeCont, file: &str, geoms: &mut BTreeMap<String, PositionVector>) -> Self {
        NIImporter_DlrNavteq_NodesHandler {
            myNodeCont: Ptr::from_mut(nc),
            myGeoms: Ptr::from_mut(geoms),
        }
    }

    /// C++: `void NodesHandler::~NodesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_DlrNavteq::NodesHandler::~NodesHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NodesHandler::report(const std::string &)` [virtual]
    /// Parsing method
    // bool
    // NIImporter_DlrNavteq::NodesHandler::report(const std::string& result) {
    //     if (result[0] == '#') {
    //         return true;
    //     }
    //     std::string id;
    //     double x, y;
    //     int no_geoms, intermediate;
    //     // parse
    //     std::istringstream stream(result);
    //     // id
    //     stream >> id;
    //     if (stream.fail()) {
    //         throw ProcessError("Something is wrong with the following data line\n" + result);
    //     }
    //     // intermediate?
    //     stream >> intermediate;
    //     if (stream.fail()) {
    //         if (myNodeCont.size() == 0) { // be generous with extra data at beginning of file
    //             return true;
    //         }
    //         throw ProcessError(TLF("Non-numerical value for intermediate status in node %.", id));
    //     }
    //     // number of geometrical information
    //     stream >> no_geoms;
    //     if (stream.fail()) {
    //         throw ProcessError(TLF("Non-numerical value for number of geometries in node %.", id));
    //     }
    //     // geometrical information
    //     PositionVector geoms;
    //     for (int i = 0; i < no_geoms; i++) {
    //         stream >> x;
    //         if (stream.fail()) {
    //             throw ProcessError(TLF("Non-numerical value for x-position in node %.", id));
    //         }
    //         stream >> y;
    //         if (stream.fail()) {
    //             throw ProcessError(TLF("Non-numerical value for y-position in node %.", id));
    //         }
    //         Position pos(x, y);
    //         if (!NBNetBuilder::transformCoordinate(pos, true)) {
    //             throw ProcessError(TLF("Unable to project coordinates for node %.", id));
    //         }
    //         geoms.push_back(pos);
    //     }
    //
    //     if (intermediate == 0) {
    //         NBNode* n = new NBNode(id, geoms[0]);
    //         if (!myNodeCont.insert(n)) {
    //             delete n;
    //             if (OptionsCont::getOptions().getBool("ignore-errors")) {
    //                 WRITE_WARNINGF(TL("Could not add node '%'."), id);
    //             } else {
    //                 throw ProcessError(TLF("Could not add node '%'.", id));
    //             }
    //         }
    //     } else {
    //         myGeoms[id] = geoms;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn report(&mut self, result: &str) -> bool {
        if result.starts_with('#') {
            return true;
        }
        let mut stream = LineStream::new(result);
        // id
        let id = match stream.token() {
            Some(t) => t.to_string(),
            None => throw_error(ProcessError::new(format!("Something is wrong with the following data line\n{result}"))),
        };
        // intermediate?
        let intermediate = match stream.int() {
            Some(v) => v,
            None => {
                if self.myNodeCont.size() == 0 {
                    // be generous with extra data at beginning of file
                    return true;
                }
                throw_error(ProcessError::new(tlf!("Non-numerical value for intermediate status in node %.", id)));
            }
        };
        // number of geometrical information
        let no_geoms = match stream.int() {
            Some(v) => v,
            None => throw_error(ProcessError::new(tlf!("Non-numerical value for number of geometries in node %.", id))),
        };
        // geometrical information
        let mut geoms = PositionVector::default();
        for _ in 0..no_geoms {
            let x = match stream.double() {
                Some(v) => v,
                None => throw_error(ProcessError::new(tlf!("Non-numerical value for x-position in node %.", id))),
            };
            let y = match stream.double() {
                Some(v) => v,
                None => throw_error(ProcessError::new(tlf!("Non-numerical value for y-position in node %.", id))),
            };
            let mut pos = Position::new(x, y);
            if !NBNetBuilder::transform_coordinate(&mut pos, true, Ptr::null()) {
                throw_error(ProcessError::new(tlf!("Unable to project coordinates for node %.", id)));
            }
            geoms.push(pos);
        }

        if intermediate == 0 {
            // shortcut: the C++ NBNode(id, position) overload (district = nullptr) maps to the UNKNOWN node type here
            let n = Ptr::from_mut(Box::leak(Box::new(NBNode::new(&id, &geoms[0], SumoXMLNodeType::UNKNOWN))));
            if !self.myNodeCont.m().insert_2(n) {
                // delete n
                drop(unsafe { Box::from_raw(n.as_raw()) });
                if get_bool("ignore-errors") {
                    write_warningf!("Could not add node '%'.", id);
                } else {
                    throw_error(ProcessError::new(tlf!("Could not add node '%'.", id)));
                }
            }
        } else {
            self.myGeoms.m().insert(id, geoms);
        }
        true
    }

    /// C++: `void NodesHandler::NodesHandler(const NodesHandler &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_DlrNavteq_NodesHandler) -> Self {
        panic!("NodesHandler copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `NodesHandler & NodesHandler::operator=(const NodesHandler &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_DlrNavteq_NodesHandler) -> &mut NIImporter_DlrNavteq_NodesHandler {
        panic!("NodesHandler assignment operator is invalidated (private and never defined in C++)")
    }

}

/// Imports prohibitions regarding connectivity
/// C++ class `NIImporter_DlrNavteq::ProhibitionHandler` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq_ProhibitionHandler {
    // C++ base class `LineHandler` is the Rust trait `LineHandler`: add `impl LineHandler for NIImporter_DlrNavteq_ProhibitionHandler`
    /// The edge container to store loaded edges into
    pub myEdgeCont: Ptr<NBEdgeCont>,
    pub myFile: String,
    pub myVersion: f64,
    pub myConstructionTime: i64,
}

impl NIImporter_DlrNavteq_ProhibitionHandler {
    /// C++: `void ProhibitionHandler::ProhibitionHandler(NBEdgeCont &, const std::string &, time_t)`
    /// Constructor
    // NIImporter_DlrNavteq::ProhibitionHandler::ProhibitionHandler(
    //     NBEdgeCont& ec, const std::string& file, time_t constructionTime) :
    //     myEdgeCont(ec),
    //     myFile(file),
    //     myVersion(0),
    //     myConstructionTime(constructionTime) {
    // }
    #[allow(unused_variables)]
    pub fn new(ne: &mut NBEdgeCont, file: &str, constructionTime: i64) -> Self {
        NIImporter_DlrNavteq_ProhibitionHandler {
            myEdgeCont: Ptr::from_mut(ne),
            myFile: file.to_string(),
            myVersion: 0.0,
            myConstructionTime: constructionTime,
        }
    }

    /// C++: `void ProhibitionHandler::~ProhibitionHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_DlrNavteq::ProhibitionHandler::~ProhibitionHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool ProhibitionHandler::report(const std::string &)` [virtual]
    /// Parsing method
    // bool
    // NIImporter_DlrNavteq::ProhibitionHandler::report(const std::string& result) {
    // // # NAME_ID    Name
    //     if (result[0] == '#') {
    //         if (myVersion == 0) {
    //             const double version = readVersion(result, myFile);
    //             if (version > 0) {
    //                 myVersion = version;
    //             }
    //         }
    //         return true;
    //     }
    //     StringTokenizer st(result, StringTokenizer::TAB);
    //     if (st.size() == 1) {
    //         return true; // one line with the number of data containing lines in it (also starts with a comment # since ersion 6.5)
    //     }
    //     if (myVersion >= 6) {
    //         assert(st.size() >= 7);
    //         const std::string id = st.next();
    //         const std::string permanent = st.next();
    //         const std::string validityPeriod = st.next();
    //         const std::string throughTraffic = st.next();
    //         const std::string vehicleType = st.next();
    //         if (validityPeriod != UNDEFINED) {
    //             WRITE_WARNINGF(TL("Ignoring temporary prohibited manoeuvre (%)."), validityPeriod);
    //             return true;
    //         }
    //     }
    //     const std::string startEdge = st.next();
    //     const std::string endEdge = st.get(st.size() - 1);
    //
    //     NBEdge* from = myEdgeCont.retrieve(startEdge);
    //     if (from == nullptr) {
    //         WRITE_WARNINGF(TL("Ignoring prohibition from unknown start edge '%'."), startEdge);
    //         return true;
    //     }
    //     NBEdge* to = myEdgeCont.retrieve(endEdge);
    //     if (to == nullptr) {
    //         WRITE_WARNINGF(TL("Ignoring prohibition from unknown end edge '%'."), endEdge);
    //         return true;
    //     }
    //     from->removeFromConnections(to, -1, -1, true);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn report(&mut self, result: &str) -> bool {
        if result.starts_with('#') {
            if self.myVersion == 0.0 {
                let version = NIImporter_DlrNavteq::read_version(result, &self.myFile);
                if version > 0.0 {
                    self.myVersion = version;
                }
            }
            return true;
        }
        let mut st = StringTokenizer::with_special(result, StringTokenizer::TAB);
        if st.size() == 1 {
            return true; // one line with the number of data containing lines in it (also starts with a comment # since ersion 6.5)
        }
        if self.myVersion >= 6.0 {
            assert!(st.size() >= 7);
            let _id = must(st.next());
            let _permanent = must(st.next());
            let validityPeriod = must(st.next());
            let _throughTraffic = must(st.next());
            let _vehicleType = must(st.next());
            if validityPeriod != UNDEFINED {
                write_warningf!("Ignoring temporary prohibited manoeuvre (%).", validityPeriod);
                return true;
            }
        }
        let startEdge = must(st.next());
        let endEdge = must(st.get(st.size() - 1));

        let from = self.myEdgeCont.retrieve(&startEdge, false);
        if from.is_null() {
            write_warningf!("Ignoring prohibition from unknown start edge '%'.", startEdge);
            return true;
        }
        let to = self.myEdgeCont.retrieve(&endEdge, false);
        if to.is_null() {
            write_warningf!("Ignoring prohibition from unknown end edge '%'.", endEdge);
            return true;
        }
        from.m().remove_from_connections(to, -1, -1, true, false, false);
        true
    }

    /// C++: `void ProhibitionHandler::ProhibitionHandler(const ProhibitionHandler &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_DlrNavteq_ProhibitionHandler) -> Self {
        panic!("ProhibitionHandler copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `ProhibitionHandler & ProhibitionHandler::operator=(const ProhibitionHandler &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_DlrNavteq_ProhibitionHandler) -> &mut NIImporter_DlrNavteq_ProhibitionHandler {
        panic!("ProhibitionHandler assignment operator is invalidated (private and never defined in C++)")
    }

}

/// Importer of street names in DLRNavteq's (aka elmar) format
/// C++ class `NIImporter_DlrNavteq::TimeRestrictionsHandler` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq_TimeRestrictionsHandler {
    // C++ base class `LineHandler` is the Rust trait `LineHandler`: add `impl LineHandler for NIImporter_DlrNavteq_TimeRestrictionsHandler`
    /// The edge container
    pub myEdgeCont: Ptr<NBEdgeCont>,
    pub myDistrictCont: Ptr<NBDistrictCont>,
    /// The date for which to build the network (in case some edges are still under construction)
    pub myConstructionTime: i64,
    pub myCS_min: i64,
    pub myCS_max: i64,
    pub myConstructionEntries: i32,
    pub myNotStarted: i32,
    pub myUnderConstruction: i32,
    pub myFinished: i32,
    pub myRemovedEdges: i32,
}

impl NIImporter_DlrNavteq_TimeRestrictionsHandler {
    /// C++: `void TimeRestrictionsHandler::TimeRestrictionsHandler(NBEdgeCont &, NBDistrictCont &, time_t)`
    /// Constructor
    // NIImporter_DlrNavteq::TimeRestrictionsHandler::TimeRestrictionsHandler(NBEdgeCont& ec, NBDistrictCont& dc, time_t constructionTime):
    //     myEdgeCont(ec),
    //     myDistrictCont(dc),
    //     myConstructionTime(constructionTime),
    //     myCS_min(std::numeric_limits<time_t>::max()),
    //     myCS_max(std::numeric_limits<time_t>::min()),
    //     myConstructionEntries(0),
    //     myNotStarted(0),
    //     myUnderConstruction(0),
    //     myFinished(0),
    //     myRemovedEdges(0) {
    // }
    #[allow(unused_variables)]
    pub fn new(ec: &mut NBEdgeCont, dc: &mut NBDistrictCont, constructionTime: i64) -> Self {
        NIImporter_DlrNavteq_TimeRestrictionsHandler {
            myEdgeCont: Ptr::from_mut(ec),
            myDistrictCont: Ptr::from_mut(dc),
            myConstructionTime: constructionTime,
            myCS_min: i64::MAX,
            myCS_max: i64::MIN,
            myConstructionEntries: 0,
            myNotStarted: 0,
            myUnderConstruction: 0,
            myFinished: 0,
            myRemovedEdges: 0,
        }
    }

    /// C++: `void TimeRestrictionsHandler::~TimeRestrictionsHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_DlrNavteq::TimeRestrictionsHandler::~TimeRestrictionsHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool TimeRestrictionsHandler::report(const std::string &)` [virtual]
    /// Parsing method
    // bool
    // NIImporter_DlrNavteq::TimeRestrictionsHandler::report(const std::string& result) {
    // // # NAME_ID    Name
    //     if (result[0] == '#') {
    //         return true;
    //     }
    //     StringTokenizer st(result, StringTokenizer::WHITECHARS);
    //     const std::string id = st.next();
    //     const std::string type = st.next();
    //     const std::string directionOfFlow = st.next(); // can be ignored since unidirectional edge ids are referenced in the file
    //     const std::string throughTraffic = st.next();
    //     const std::string vehicleType = st.next();
    //     const std::string validityPeriod = st.next();
    //     const std::string warning = "Unrecognized TIME_REC '" + validityPeriod + "'";
    //     if (type == "CS") {
    //         myConstructionEntries++;
    //         if (validityPeriod.size() > 1024) {
    //             WRITE_WARNING(warning);
    //         }
    //         // construction
    //         char start[1024];
    //         char duration[1024];
    //
    //         int matched;
    //
    //         matched = sscanf(validityPeriod.c_str(), "[(%[^)]){%[^}]}]", start, duration);
    //         if (matched == 2) {
    //             time_t tStart = readTimeRec(start, "");
    //             time_t tEnd = readTimeRec(start, duration);
    //             myCS_min = MIN2(myCS_min, tStart);
    //             myCS_max = MAX2(myCS_max, tEnd);
    //             //std::cout << " start=" << start << " tStart=" << tStart<< " translation=" << asctime(localtime(&tStart)) << "";
    //             //std::cout << " duration=" << duration << " tEnd=" << tEnd << " translation=" << asctime(localtime(&tEnd)) << "\n";
    //             if (myConstructionTime < tEnd) {
    //                 NBEdge* edge = myEdgeCont.retrieve(id);
    //                 if (edge != nullptr) {
    //                     myRemovedEdges++;
    //                     myEdgeCont.extract(myDistrictCont, edge, true);
    //                 }
    //                 if (myConstructionTime < tStart) {
    //                     myNotStarted++;
    //                 } else {
    //                     myUnderConstruction++;
    //                 }
    //             } else {
    //                 myFinished++;
    //             }
    //         } else {
    //             WRITE_WARNING(warning);
    //         };
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn report(&mut self, result: &str) -> bool {
        if result.starts_with('#') {
            return true;
        }
        let mut st = StringTokenizer::with_special(result, StringTokenizer::WHITECHARS);
        let id = must(st.next());
        let type_ = must(st.next());
        let _directionOfFlow = must(st.next()); // can be ignored since unidirectional edge ids are referenced in the file
        let _throughTraffic = must(st.next());
        let _vehicleType = must(st.next());
        let validityPeriod = must(st.next());
        let warning = format!("Unrecognized TIME_REC '{validityPeriod}'");
        if type_ == "CS" {
            self.myConstructionEntries += 1;
            if validityPeriod.len() > 1024 {
                write_warning(&warning);
            }
            // construction
            match scan_construction_period(&validityPeriod) {
                Some((start, duration)) => {
                    let tStart = NIImporter_DlrNavteq::read_time_rec(&start, "");
                    let tEnd = NIImporter_DlrNavteq::read_time_rec(&start, &duration);
                    self.myCS_min = std::cmp::min(self.myCS_min, tStart);
                    self.myCS_max = std::cmp::max(self.myCS_max, tEnd);
                    if self.myConstructionTime < tEnd {
                        let edge = self.myEdgeCont.retrieve(&id, false);
                        if !edge.is_null() {
                            self.myRemovedEdges += 1;
                            self.myEdgeCont.m().extract(self.myDistrictCont.m(), edge, true);
                        }
                        if self.myConstructionTime < tStart {
                            self.myNotStarted += 1;
                        } else {
                            self.myUnderConstruction += 1;
                        }
                    } else {
                        self.myFinished += 1;
                    }
                }
                None => write_warning(&warning),
            }
        }
        true
    }

    /// C++: `void TimeRestrictionsHandler::printSummary()`
    // void
    // NIImporter_DlrNavteq::TimeRestrictionsHandler::printSummary() {
    //     if (myConstructionEntries > 0) {
    //         char buff[1024];
    //         std::ostringstream msg;
    //         strftime(buff, 1024, "%Y-%m-%d", localtime(&myCS_min));
    //         msg << "Parsed " << myConstructionEntries << " construction entries between " << buff;
    //         strftime(buff, 1024, "%Y-%m-%d", localtime(&myCS_max));
    //         msg << " and " << buff << ".\n";
    //         strftime(buff, 1024, "%Y-%m-%d", localtime(&myConstructionTime));
    //         msg << "Removed " << myRemovedEdges << " edges not yet constructed at " << buff << ".\n";
    //         msg << "   not yet started: " << myNotStarted << "\n";
    //         msg << "   under construction: " << myUnderConstruction << "\n";
    //         msg << "   finished: " << myFinished << "\n";
    //         WRITE_MESSAGE(msg.str());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn print_summary(&mut self) {
        if self.myConstructionEntries > 0 {
            // shortcut: strftime/localtime use the local time zone, dates are formatted in UTC here
            let msg = format!(
                "Parsed {} construction entries between {} and {}.\nRemoved {} edges not yet constructed at {}.\n   not yet started: {}\n   under construction: {}\n   finished: {}\n",
                self.myConstructionEntries,
                format_date(self.myCS_min),
                format_date(self.myCS_max),
                self.myRemovedEdges,
                format_date(self.myConstructionTime),
                self.myNotStarted,
                self.myUnderConstruction,
                self.myFinished,
            );
            write_message(&msg);
        }
    }

    /// C++: `void TimeRestrictionsHandler::TimeRestrictionsHandler(const TimeRestrictionsHandler &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_DlrNavteq_TimeRestrictionsHandler) -> Self {
        panic!("TimeRestrictionsHandler copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `TimeRestrictionsHandler & TimeRestrictionsHandler::operator=(const TimeRestrictionsHandler &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_DlrNavteq_TimeRestrictionsHandler) -> &mut NIImporter_DlrNavteq_TimeRestrictionsHandler {
        panic!("TimeRestrictionsHandler assignment operator is invalidated (private and never defined in C++)")
    }

}

/// Importer of traffic lights stored in DLRNavteq's (aka elmar) format
/// C++ class `NIImporter_DlrNavteq::TrafficlightsHandler` (netimport/NIImporter_DlrNavteq.h)
pub struct NIImporter_DlrNavteq_TrafficlightsHandler {
    // C++ base class `LineHandler` is the Rust trait `LineHandler`: add `impl LineHandler for NIImporter_DlrNavteq_TrafficlightsHandler`
    /// The node container to get the referenced nodes from
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The traffic lights container to add built tls to
    pub myTLLogicCont: Ptr<NBTrafficLightLogicCont>,
    /// The edge container to get the referenced edges from
    pub myEdgeCont: Ptr<NBEdgeCont>,
}

impl NIImporter_DlrNavteq_TrafficlightsHandler {
    /// C++: `void TrafficlightsHandler::TrafficlightsHandler(NBNodeCont &, NBTrafficLightLogicCont &, NBEdgeCont &, const std::string &)`
    /// Constructor
    // NIImporter_DlrNavteq::TrafficlightsHandler::TrafficlightsHandler(NBNodeCont& nc,
    //         NBTrafficLightLogicCont& tlc,
    //         NBEdgeCont& ne,
    //         const std::string& file) :
    //     myNodeCont(nc),
    //     myTLLogicCont(tlc),
    //     myEdgeCont(ne) {
    //     UNUSED_PARAMETER(file);
    // }
    #[allow(unused_variables)]
    pub fn new(nc: &mut NBNodeCont, tlc: &mut NBTrafficLightLogicCont, ne: &mut NBEdgeCont, file: &str) -> Self {
        NIImporter_DlrNavteq_TrafficlightsHandler {
            myNodeCont: Ptr::from_mut(nc),
            myTLLogicCont: Ptr::from_mut(tlc),
            myEdgeCont: Ptr::from_mut(ne),
        }
    }

    /// C++: `void TrafficlightsHandler::~TrafficlightsHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_DlrNavteq::TrafficlightsHandler::~TrafficlightsHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool TrafficlightsHandler::report(const std::string &)` [virtual]
    /// Parsing method
    // bool
    // NIImporter_DlrNavteq::TrafficlightsHandler::report(const std::string& result) {
    // // #ID     POICOL-TYPE     DESCRIPTION     LONGITUDE       LATITUDE        NAVTEQ_LINK_ID  NODEID
    //
    //     if (result[0] == '#') {
    //         return true;
    //     }
    //     StringTokenizer st(result, StringTokenizer::WHITECHARS);
    //     const std::string edgeID = st.get(5);
    //     NBEdge* edge = myEdgeCont.retrieve(edgeID);
    //     if (edge == nullptr) {
    //         WRITE_WARNINGF(TL("The traffic light edge '%' could not be found."), edgeID);
    //     } else {
    //         NBNode* node = edge->getToNode();
    //         if (node->getType() != SumoXMLNodeType::TRAFFIC_LIGHT) {
    //             node->reinit(node->getPosition(), SumoXMLNodeType::TRAFFIC_LIGHT);
    //             // @note. There may be additional information somewhere in the GDF files about traffic light type ...
    //             TrafficLightType type = SUMOXMLDefinitions::TrafficLightTypes.get(OptionsCont::getOptions().getString("tls.default-type"));
    //             // @note actually we could use the navteq node ID here
    //             NBTrafficLightDefinition* tlDef = new NBOwnTLDef(node->getID(), node, 0, type);
    //             if (!myTLLogicCont.insert(tlDef)) {
    //                 // actually, nothing should fail here
    //                 delete tlDef;
    //                 throw ProcessError(TLF("Could not allocate tls for '%'.", node->getID()));
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn report(&mut self, result: &str) -> bool {
        if result.starts_with('#') {
            return true;
        }
        let mut st = StringTokenizer::with_special(result, StringTokenizer::WHITECHARS);
        let edgeID = must(st.get(5));
        let edge = self.myEdgeCont.retrieve(&edgeID, false);
        if edge.is_null() {
            write_warningf!("The traffic light edge '%' could not be found.", edgeID);
        } else {
            let node = edge.get_to_node();
            if node.get_type() != SumoXMLNodeType::TRAFFIC_LIGHT {
                let pos = *node.get_position();
                node.m().reinit(&pos, SumoXMLNodeType::TRAFFIC_LIGHT, false);
                // @note. There may be additional information somewhere in the GDF files about traffic light type ...
                let type_ = must(TrafficLightTypes.get(&get_string("tls.default-type")));
                // @note actually we could use the navteq node ID here
                let raw: *mut NBOwnTLDef = Box::into_raw(Box::new(NBOwnTLDef::new_2(node.get_id(), node, 0, type_)));
                let tlDef = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) });
                if !self.myTLLogicCont.m().insert(tlDef, false) {
                    // actually, nothing should fail here
                    // delete tlDef
                    drop(unsafe { Box::from_raw(raw) });
                    throw_error(ProcessError::new(tlf!("Could not allocate tls for '%'.", node.get_id())));
                }
            }
        }
        true
    }

    /// C++: `void TrafficlightsHandler::TrafficlightsHandler(const TrafficlightsHandler &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_DlrNavteq_TrafficlightsHandler) -> Self {
        panic!("TrafficlightsHandler copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `TrafficlightsHandler & TrafficlightsHandler::operator=(const TrafficlightsHandler &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_DlrNavteq_TrafficlightsHandler) -> &mut NIImporter_DlrNavteq_TrafficlightsHandler {
        panic!("TrafficlightsHandler assignment operator is invalidated (private and never defined in C++)")
    }

}


impl LineHandler for NIImporter_DlrNavteq_ConnectedLanesHandler {
    fn report(&mut self, result: &str) -> bool {
        NIImporter_DlrNavteq_ConnectedLanesHandler::report(self, result)
    }
}

impl LineHandler for NIImporter_DlrNavteq_EdgesHandler {
    fn report(&mut self, result: &str) -> bool {
        NIImporter_DlrNavteq_EdgesHandler::report(self, result)
    }
}

impl LineHandler for NIImporter_DlrNavteq_NamesHandler {
    fn report(&mut self, result: &str) -> bool {
        NIImporter_DlrNavteq_NamesHandler::report(self, result)
    }
}

impl LineHandler for NIImporter_DlrNavteq_NodesHandler {
    fn report(&mut self, result: &str) -> bool {
        NIImporter_DlrNavteq_NodesHandler::report(self, result)
    }
}

impl LineHandler for NIImporter_DlrNavteq_ProhibitionHandler {
    fn report(&mut self, result: &str) -> bool {
        NIImporter_DlrNavteq_ProhibitionHandler::report(self, result)
    }
}

impl LineHandler for NIImporter_DlrNavteq_TimeRestrictionsHandler {
    fn report(&mut self, result: &str) -> bool {
        NIImporter_DlrNavteq_TimeRestrictionsHandler::report(self, result)
    }
}

impl LineHandler for NIImporter_DlrNavteq_TrafficlightsHandler {
    fn report(&mut self, result: &str) -> bool {
        NIImporter_DlrNavteq_TrafficlightsHandler::report(self, result)
    }
}
