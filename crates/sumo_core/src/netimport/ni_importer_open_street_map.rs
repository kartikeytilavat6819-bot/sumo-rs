//! Port of netimport/NIImporter_OpenStreetMap.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::util_exceptions::ProcessError;
use utils::xml::sumo_sax_reader::SUMOSAXReader;
use utils::xml::sumo_sax_handler;
use utils::xml::generic_sax_handler::{GenericSAXHandler, SaxHandler};
use utils::{write_errorf, write_messagef, write_warningf};
use utils::common::std_defs::{g_precision, SUMO_CONST_LANE_WIDTH};
use utils::common::string_utils::starts_with;
use utils::common::to_string::join_to_string_prec;
use utils::common::string_utils::{escape_xml, is_bool, to_bool, to_long};

/// C++ `NBTrafficLightDefinition::OSM_DIRECTION`
const OSM_DIRECTION: &str = "osm:direction";

/// C++ `toString(LaneSpreadFunction)`: the first name in SUMOXMLDefinitions' table for the value
fn lane_spread_str(l: LaneSpreadFunction) -> &'static str {
    match l {
        LaneSpreadFunction::RIGHT => "right",
        LaneSpreadFunction::ROADCENTER => "roadCenter",
        LaneSpreadFunction::CENTER => "center",
        LaneSpreadFunction::SPREAD_UNKNOWN => "unknown",
    }
}

/// C++ `toString(SumoXMLAttr)`: the attribute name from the SUMOXML attribute table
fn attr_name(a: i32) -> &'static str {
    utils::xml::sumo_xml_definitions::ATTRS.iter().find(|(_, v)| *v == a).map(|(n, _)| *n).unwrap_or("")
}

// C++ NBEdge::UNSPECIFIED_* constants
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;
const NBEDGE_UNSPECIFIED_FRICTION: f64 = 1.0;
const NBEDGE_UNSPECIFIED_OFFSET: f64 = 0.0;
// shortcut: NBTrafficLightDefinition::OSM_SIGNAL_DIRECTION is not ported as a const yet (nb_traffic_light_definition.rs keeps it as a comment); mirrors NBTrafficLightDefinition.cpp:49, switch to the ported constant when it lands
const OSM_SIGNAL_DIRECTION: &str = "railway:signal:direction";

/// C++ static `NBNode::FORWARD` (netbuild/NBNode.cpp); NBNode statics are not ported as consts yet.
const NBNODE_FORWARD: i32 = 1;
/// C++ static `NBNode::BACKWARD` (netbuild/NBNode.cpp).
#[allow(dead_code)]
const NBNODE_BACKWARD: i32 = -1;
/// C++ static `NBEdge::TURN_SIGN_SHIFT_BUS` etc. (netbuild/NBEdge.h); NBEdge statics are not ported as consts yet.
const NBEDGE_TURN_SIGN_SHIFT_BUS: i32 = 8;
const NBEDGE_TURN_SIGN_SHIFT_TAXI: i32 = 16;
const NBEDGE_TURN_SIGN_SHIFT_BICYCLE: i32 = 24;

// C++ MAX2/MIN2 macros (ties return the second operand for MAX2, as in the macro)
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// SAX reader results: C++ lets the ProcessError propagate, so rethrow it as one (netconvert reports it)
fn sax_result(r: Result<bool, ProcessError>) -> bool {
    r.unwrap_or_else(|e| std::panic::panic_any(e))
}

/// SysUtils::getCurrentMillis()
fn current_millis() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// PROGRESS_BEGIN_TIME_MESSAGE(msg): prints the message and returns the start time
fn progress_begin_time_message(msg: &str) -> i64 {
    utils::common::msg_handler::progress_begin_message(msg);
    current_millis()
}

/// PROGRESS_TIME_MESSAGE(before)
fn progress_time_since(before: i64) {
    utils::common::msg_handler::progress_time_message(current_millis() - before);
}

/// C++ enum `NIImporter_OpenStreetMap::ChangeType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenStreetMap_ChangeType {
    CHANGE_YES = 0,
    CHANGE_NO_LEFT = 1,
    CHANGE_NO_RIGHT = 2,
    CHANGE_NO = 3,
}

/// C++ enum `NIImporter_OpenStreetMap::ParkingType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenStreetMap_ParkingType {
    PARKING_NONE = 0,
    PARKING_LEFT = 1,
    PARKING_RIGHT = 2,
    PARKING_BOTH = 3,
    PARKING_UNKNOWN = 4,
    PARKING_FORBIDDEN = 8,
    PARKING_PERPENDICULAR = 16,
    PARKING_DIAGONAL = 32,
}

/// C++ enum `NIImporter_OpenStreetMap::PlacementType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenStreetMap_PlacementType {
    NONE = 0,
    LEFT_OF = 1,
    RIGHT_OF = 2,
    MIDDLE_OF = 3,
}

/// C++ enum `RelationHandler::RestrictionType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum RelationHandler_RestrictionType {
    ONLY = 0,
    NO = 1,
    UNKNOWN = 2,
}

/// C++ enum `NIImporter_OpenStreetMap::SignalType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenStreetMap_SignalType {
    SIGNAL_NONE = 0,
    SIGNAL_MAIN = 1,
    SIGNAL_ETCS_BLOCK = 2,
}

/// C++ enum `NIImporter_OpenStreetMap::WayType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_OpenStreetMap_WayType {
    WAY_NONE = 0,
    WAY_FORWARD = 1,
    WAY_BACKWARD = 2,
    WAY_BOTH = 3,
    WAY_UNKNOWN = 4,
    WAY_PREFER_FORWARD = 8,
    WAY_PREFER_BACKWARD = 16,
}

/// C++ static `NIImporter_OpenStreetMap::MAXSPEED_UNGIVEN`
pub const MAXSPEED_UNGIVEN: f64 = -1.0;
/// C++ static `NIImporter_OpenStreetMap::INVALID_ID`
pub const INVALID_ID: i64 = i64::MAX;
/// C++ static `NIImporter_OpenStreetMap::compoundTypeSeparator`
pub const compoundTypeSeparator: &str = "|";

thread_local! {
    /// C++ static `NIImporter_OpenStreetMap::myAllAttributes`
    pub static myAllAttributes: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// C++ static `NIImporter_OpenStreetMap::myExtraAttributes`
    pub static myExtraAttributes: std::cell::RefCell<BTreeSet<String>> = std::cell::RefCell::new(BTreeSet::new());
}

/// Functor which compares two Edges
/// C++ class `NIImporter_OpenStreetMap::CompareEdges` (netimport/NIImporter_OpenStreetMap.cpp)
pub struct NIImporter_OpenStreetMap_CompareEdges {
}

impl NIImporter_OpenStreetMap_CompareEdges {
    /// C++: `bool CompareEdges::operator()(const Edge *, const Edge *) const` [const]
    // bool operator()(const Edge* e1, const Edge* e2) const {
    //         if (e1->myHighWayType != e2->myHighWayType) {
    //             return e1->myHighWayType > e2->myHighWayType;
    //         }
    //         if (e1->myNoLanes != e2->myNoLanes) {
    //             return e1->myNoLanes > e2->myNoLanes;
    //         }
    //         if (e1->myNoLanesForward != e2->myNoLanesForward) {
    //             return e1->myNoLanesForward > e2->myNoLanesForward;
    //         }
    //         if (e1->myMaxSpeed != e2->myMaxSpeed) {
    //             return e1->myMaxSpeed > e2->myMaxSpeed;
    //         }
    //         if (e1->myIsOneWay != e2->myIsOneWay) {
    //             return e1->myIsOneWay > e2->myIsOneWay;
    //         }
    //         if (e1->myPlacement != e2->myPlacement) {
    //             return (int)e1->myPlacement > (int)e2->myPlacement;
    //         }
    //         if (e1->myPlacementLane != e2->myPlacementLane) {
    //             return e1->myPlacementLane > e2->myPlacementLane;
    //         }
    //         return e1->myCurrentNodes > e2->myCurrentNodes;
    //     }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NIImporter_OpenStreetMap_Edge>, e2: Ptr<NIImporter_OpenStreetMap_Edge>) -> bool {
        if e1.myHighWayType != e2.myHighWayType {
            return e1.myHighWayType > e2.myHighWayType;
        }
        if e1.myNoLanes != e2.myNoLanes {
            return e1.myNoLanes > e2.myNoLanes;
        }
        if e1.myNoLanesForward != e2.myNoLanesForward {
            return e1.myNoLanesForward > e2.myNoLanesForward;
        }
        if e1.myMaxSpeed != e2.myMaxSpeed {
            return e1.myMaxSpeed > e2.myMaxSpeed;
        }
        if e1.myIsOneWay != e2.myIsOneWay {
            return e1.myIsOneWay > e2.myIsOneWay;
        }
        if e1.myPlacement != e2.myPlacement {
            return (e1.myPlacement as i64) > (e2.myPlacement as i64);
        }
        if e1.myPlacementLane != e2.myPlacementLane {
            return e1.myPlacementLane > e2.myPlacementLane;
        }
        e1.myCurrentNodes > e2.myCurrentNodes
    }

}

/// Functor which compares two NIOSMNodes according to their coordinates
/// C++ class `NIImporter_OpenStreetMap::CompareNodes` (netimport/NIImporter_OpenStreetMap.h)
pub struct NIImporter_OpenStreetMap_CompareNodes {
}

/// shortcut: NBPTStop is held as Rc but C++ mutates it in place; mutate through the Rc pointer until stop containers hold Ptr<NBPTStop>
#[allow(clippy::mut_from_ref)]
fn pt_stop_mut(stop: &Rc<NBPTStop>) -> &mut NBPTStop {
    unsafe { &mut *(Rc::as_ptr(stop) as *mut NBPTStop) }
}

/// A node pointer ordered by CompareNodes, so a BTreeSet behaves like C++ `std::set<NIOSMNode *, CompareNodes>`
/// (find/insert match on coordinates, not on pointer value).
#[derive(Clone, Copy)]
pub struct NIImporter_OpenStreetMap_NodeByCoord(pub Ptr<NIImporter_OpenStreetMap_NIOSMNode>);

impl PartialEq for NIImporter_OpenStreetMap_NodeByCoord {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for NIImporter_OpenStreetMap_NodeByCoord {}

impl PartialOrd for NIImporter_OpenStreetMap_NodeByCoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NIImporter_OpenStreetMap_NodeByCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = NIImporter_OpenStreetMap_CompareNodes {};
        if cmp.call_op(self.0, other.0) {
            std::cmp::Ordering::Less
        } else if cmp.call_op(other.0, self.0) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl NIImporter_OpenStreetMap_CompareNodes {
    /// C++: `bool CompareNodes::operator()(const NIOSMNode *, const NIOSMNode *) const` [const]
    // bool operator()(const NIOSMNode* n1, const NIOSMNode* n2) const {
    //             return (n1->lat > n2->lat) || (n1->lat == n2->lat && n1->lon > n2->lon);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, n1: Ptr<NIImporter_OpenStreetMap_NIOSMNode>, n2: Ptr<NIImporter_OpenStreetMap_NIOSMNode>) -> bool {
        (n1.lat > n2.lat) || (n1.lat == n2.lat && n1.lon > n2.lon)
    }

}

/// An internal definition of a loaded edge
/// C++ class `NIImporter_OpenStreetMap::Edge` (netimport/NIImporter_OpenStreetMap.h)
#[repr(C)]
pub struct NIImporter_OpenStreetMap_Edge {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The edge's id
    pub id: i64,
    /// The edge's street name
    pub streetName: String,
    /// The edge's track name
    pub ref_: String,
    /// number of lanes, or -1 if unknown
    pub myNoLanes: i32,
    /// number of lanes in forward direction or 0 if unknown, negative if backwards lanes are meant
    pub myNoLanesForward: i32,
    /// maximum speed in km/h, or MAXSPEED_UNGIVEN
    pub myMaxSpeed: f64,
    /// maximum speed in km/h, or MAXSPEED_UNGIVEN
    pub myMaxSpeedBackward: f64,
    /// Extra permissions added from tags instead of highway type
    pub myExtraAllowed: SVCPermissions,
    /// Extra permissions prohibited from tags instead of highway type
    pub myExtraDisallowed: SVCPermissions,
    /// The type, stored in "highway" key
    pub myHighWayType: String,
    /// Information whether this is an one-way road
    pub myIsOneWay: String,
    /// Information about the kind of cycleway along this road
    pub myCyclewayType: i64,
    /// Information about the kind of busway along this road
    pub myBuswayType: i64,
    /// Information about the kind of sidwalk along this road
    pub mySidewalkType: i64,
    /// Information about the direction(s) of railway usage
    pub myRailDirection: i32,
    /// Information about road-side parking
    pub myParkingType: i32,
    /// Information about change prohibitions (forward direction
    pub myChangeForward: i32,
    /// Information about change prohibitions (backward direction
    pub myChangeBackward: i32,
    /// (optional) information about whether the forward lanes are designated to some SVCs
    pub myDesignatedLaneForward: Vec<bool>,
    /// (optional) information about whether the backward lanes are designated to some SVCs
    pub myDesignatedLaneBackward: Vec<bool>,
    /// (optional) information about additional allowed SVCs on forward lane(s)
    pub myAllowedLaneForward: Vec<SVCPermissions>,
    /// (optional) information about additional allowed SVCs on backward lane(s)
    pub myAllowedLaneBackward: Vec<SVCPermissions>,
    /// (optional) information about additional disallowed SVCs on forward lane(s)
    pub myDisallowedLaneForward: Vec<SVCPermissions>,
    /// (optional) information about additional disallowed SVCs on backward lane(s)
    pub myDisallowedLaneBackward: Vec<SVCPermissions>,
    /// Information about the relative z-ordering of ways
    pub myLayer: i32,
    /// The list of nodes this edge is made of
    pub myCurrentNodes: Vec<i64>,
    /// Information whether this is a road
    pub myCurrentIsRoad: bool,
    /// Information whether this road is part of a roundabout
    pub myAmInRoundabout: bool,
    /// Additionally tagged information
    pub myExtraTags: BTreeMap<String, String>,
    /// turning direction (arrows printed on the road)
    pub myTurnSignsForward: Vec<i32>,
    pub myTurnSignsBackward: Vec<i32>,
    /// placement of the OSM way geometry relative to lanes
    pub myPlacement: NIImporter_OpenStreetMap_PlacementType,
    /// 1-based lane index for placement specification
    pub myPlacementLane: i32,
    /// Information on lane width
    pub myWidthLanesForward: Vec<f64>,
    pub myWidthLanesBackward: Vec<f64>,
    pub myWidth: f64,
}

impl std::ops::Deref for NIImporter_OpenStreetMap_Edge {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_OpenStreetMap_Edge {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NIImporter_OpenStreetMap_Edge {
    /// C++: `void Edge::Edge(long long)`
    // explicit Edge(long long int _id) :
    //             id(_id), myNoLanes(-1), myNoLanesForward(0),
    //             myMaxSpeed(MAXSPEED_UNGIVEN),
    //             myMaxSpeedBackward(MAXSPEED_UNGIVEN),
    //             myExtraAllowed(0),
    //             myExtraDisallowed(0),
    //             myCyclewayType(WAY_UNKNOWN), // building of extra lane depends on bikelaneWidth of loaded typemap
    //             myBuswayType(WAY_NONE), // buslanes are always built when declared
    //             mySidewalkType(WAY_UNKNOWN), // building of extra lanes depends on sidewalkWidth of loaded typemap
    //             myRailDirection(WAY_UNKNOWN), // store direction(s) of railway usage
    //             myParkingType(PARKING_NONE), // parking areas exported optionally
    //             myChangeForward(CHANGE_YES),
    //             myChangeBackward(CHANGE_YES),
    //             myLayer(0), // layer is non-zero only in conflict areas
    //             myCurrentIsRoad(false),
    //             myAmInRoundabout(false),
    //             myPlacement(PlacementType::NONE),
    //             myPlacementLane(-1),
    //             myWidth(-1)
    //         { }
    #[allow(unused_variables)]
    pub fn new(_id: i64) -> Self {
        NIImporter_OpenStreetMap_Edge {
            base: Parameterised::new(),
            id: _id,
            streetName: String::new(),
            ref_: String::new(),
            myNoLanes: -1,
            myNoLanesForward: 0,
            myMaxSpeed: MAXSPEED_UNGIVEN,
            myMaxSpeedBackward: MAXSPEED_UNGIVEN,
            myExtraAllowed: 0,
            myExtraDisallowed: 0,
            myHighWayType: String::new(),
            myIsOneWay: String::new(),
            myCyclewayType: NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64,
            myBuswayType: NIImporter_OpenStreetMap_WayType::WAY_NONE as i64,
            mySidewalkType: NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64,
            myRailDirection: 4, // WAY_UNKNOWN
            myParkingType: NIImporter_OpenStreetMap_ParkingType::PARKING_NONE as i32,
            myChangeForward: NIImporter_OpenStreetMap_ChangeType::CHANGE_YES as i32,
            myChangeBackward: NIImporter_OpenStreetMap_ChangeType::CHANGE_YES as i32,
            myDesignatedLaneForward: Vec::new(),
            myDesignatedLaneBackward: Vec::new(),
            myAllowedLaneForward: Vec::new(),
            myAllowedLaneBackward: Vec::new(),
            myDisallowedLaneForward: Vec::new(),
            myDisallowedLaneBackward: Vec::new(),
            myLayer: 0,
            myCurrentNodes: Vec::new(),
            myCurrentIsRoad: false,
            myAmInRoundabout: false,
            myExtraTags: BTreeMap::new(),
            myTurnSignsForward: Vec::new(),
            myTurnSignsBackward: Vec::new(),
            myPlacement: NIImporter_OpenStreetMap_PlacementType::NONE,
            myPlacementLane: -1,
            myWidthLanesForward: Vec::new(),
            myWidthLanesBackward: Vec::new(),
            myWidth: -1.0,
        }
    }

    /// C++: `void Edge::~Edge() noexcept` [virtual]
    // virtual ~Edge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // Rust drops fields automatically; the C++ destructor body is empty.
    }

    /// C++: `Edge & Edge::operator=(const Edge &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_OpenStreetMap_Edge) -> &mut NIImporter_OpenStreetMap_Edge {
        // C++ declares this private and never defines it, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::Edge::operator= is private and undefined in C++")
    }

}

/// A class which extracts OSM-edges from a parsed OSM-file
/// C++ class `NIImporter_OpenStreetMap::EdgesHandler` (netimport/NIImporter_OpenStreetMap.h)
#[repr(C)]
pub struct NIImporter_OpenStreetMap_EdgesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The previously parsed nodes
    pub myOSMNodes: Ptr<BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>>,
    /// A map of built edges
    pub myEdgeMap: Ptr<BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>>,
    /// A map of built edges
    pub myPlatformShapesMap: Ptr<BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>>,
    /// The currently built edge
    pub myCurrentEdge: Ptr<NIImporter_OpenStreetMap_Edge>,
    /// A map of non-numeric speed descriptions to their numeric values
    pub mySpeedMap: BTreeMap<String, f64>,
    pub myTypeCont: Ptr<NBTypeCont>,
}

impl std::ops::Deref for NIImporter_OpenStreetMap_EdgesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_OpenStreetMap_EdgesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_OpenStreetMap_EdgesHandler {
    /// C++: `void EdgesHandler::EdgesHandler(const std::map<long long, NIOSMNode *> &, std::map<long long, Edge *> &, std::map<long long, Edge *> &, const NBTypeCont &)`
    /// Constructor
    // NIImporter_OpenStreetMap::EdgesHandler::EdgesHandler(
    //     const std::map<long long int, NIOSMNode*>& osmNodes,
    //     std::map<long long int, Edge*>& toFill, std::map<long long int, Edge*>& platformShapes,
    //     const NBTypeCont& tc):
    //     SUMOSAXHandler("osm - file"),
    //     myOSMNodes(osmNodes),
    //     myEdgeMap(toFill),
    //     myPlatformShapesMap(platformShapes),
    //     myTypeCont(tc) {
    //
    //     const double unlimitedSpeed = OptionsCont::getOptions().getFloat("osm.speedlimit-none");
    //
    //     mySpeedMap["nan"] = MAXSPEED_UNGIVEN;
    //     mySpeedMap["sign"] = MAXSPEED_UNGIVEN;
    //     mySpeedMap["signals"] = MAXSPEED_UNGIVEN;
    //     mySpeedMap["none"] = unlimitedSpeed;
    //     mySpeedMap["no"] = unlimitedSpeed;
    //     mySpeedMap["walk"] = 5. / 3.6;
    //     // https://wiki.openstreetmap.org/wiki/Key:source:maxspeed#Commonly_used_values
    //     mySpeedMap["AT:urban"] = 50. / 3.6;
    //     mySpeedMap["AT:rural"] = 100. / 3.6;
    //     mySpeedMap["AT:trunk"] = 100. / 3.6;
    //     mySpeedMap["AT:motorway"] = 130. / 3.6;
    //     mySpeedMap["AU:urban"] = 50. / 3.6;
    //     mySpeedMap["BE:urban"] = 50. / 3.6;
    //     mySpeedMap["BE:zone"] = 30. / 3.6;
    //     mySpeedMap["BE:motorway"] = 120. / 3.6;
    //     mySpeedMap["BE:zone30"] = 30. / 3.6;
    //     mySpeedMap["BE-VLG:rural"] = 70. / 3.6;
    //     mySpeedMap["BE-WAL:rural"] = 90. / 3.6;
    //     mySpeedMap["BE:school"] = 30. / 3.6;
    //     mySpeedMap["CZ:motorway"] = 130. / 3.6;
    //     mySpeedMap["CZ:trunk"] = 110. / 3.6;
    //     mySpeedMap["CZ:rural"] = 90. / 3.6;
    //     mySpeedMap["CZ:urban_motorway"] = 80. / 3.6;
    //     mySpeedMap["CZ:urban_trunk"] = 80. / 3.6;
    //     mySpeedMap["CZ:urban"] = 50. / 3.6;
    //     mySpeedMap["DE:motorway"] = unlimitedSpeed;
    //     mySpeedMap["DE:rural"] = 100. / 3.6;
    //     mySpeedMap["DE:urban"] = 50. / 3.6;
    //     mySpeedMap["DE:bicycle_road"] = 30. / 3.6;
    //     mySpeedMap["DK:motorway"] = 130. / 3.6;
    //     mySpeedMap["DK:rural"] = 80. / 3.6;
    //     mySpeedMap["DK:urban"] = 50. / 3.6;
    //     mySpeedMap["EE:urban"] = 50. / 3.6;
    //     mySpeedMap["EE:rural"] = 90. / 3.6;
    //     mySpeedMap["ES:urban"] = 50. / 3.6;
    //     mySpeedMap["ES:zone30"] = 30. / 3.6;
    //     mySpeedMap["FR:motorway"] = 130. / 3.6; // 110 (raining)
    //     mySpeedMap["FR:rural"] = 80. / 3.6;
    //     mySpeedMap["FR:urban"] = 50. / 3.6;
    //     mySpeedMap["FR:zone30"] = 30. / 3.6;
    //     mySpeedMap["HU:living_street"] = 20. / 3.6;
    //     mySpeedMap["HU:motorway"] = 130. / 3.6;
    //     mySpeedMap["HU:rural"] = 90. / 3.6;
    //     mySpeedMap["HU:trunk"] = 110. / 3.6;
    //     mySpeedMap["HU:urban"] = 50. / 3.6;
    //     mySpeedMap["IT:rural"] = 90. / 3.6;
    //     mySpeedMap["IT:motorway"] = 130. / 3.6;
    //     mySpeedMap["IT:urban"] = 50. / 3.6;
    //     mySpeedMap["JP:nsl"] = 60. / 3.6;
    //     mySpeedMap["JP:express"] = 100. / 3.6;
    //     mySpeedMap["LT:rural"] = 90. / 3.6;
    //     mySpeedMap["LT:urban"] = 50. / 3.6;
    //     mySpeedMap["NO:rural"] = 80. / 3.6;
    //     mySpeedMap["NO:urban"] = 50. / 3.6;
    //     mySpeedMap["ON:urban"] = 50. / 3.6;
    //     mySpeedMap["ON:rural"] = 80. / 3.6;
    //     mySpeedMap["PT:motorway"] = 120. / 3.6;
    //     mySpeedMap["PT:rural"] = 90. / 3.6;
    //     mySpeedMap["PT:trunk"] = 100. / 3.6;
    //     mySpeedMap["PT:urban"] = 50. / 3.6;
    //     mySpeedMap["RO:motorway"] = 130. / 3.6;
    //     mySpeedMap["RO:rural"] = 90. / 3.6;
    //     mySpeedMap["RO:trunk"] = 100. / 3.6;
    //     mySpeedMap["RO:urban"] = 50. / 3.6;
    //     mySpeedMap["RS:living_street"] = 30. / 3.6;
    //     mySpeedMap["RS:motorway"] = 130. / 3.6;
    //     mySpeedMap["RS:rural"] = 80. / 3.6;
    //     mySpeedMap["RS:trunk"] = 100. / 3.6;
    //     mySpeedMap["RS:urban"] = 50. / 3.6;
    //     mySpeedMap["RU:living_street"] = 20. / 3.6;
    //     mySpeedMap["RU:urban"] = 60. / 3.6;
    //     mySpeedMap["RU:rural"] = 90. / 3.6;
    //     mySpeedMap["RU:motorway"] = 110. / 3.6;
    //     const double seventy = StringUtils::parseSpeed("70mph");
    //     const double sixty = StringUtils::parseSpeed("60mph");
    //     mySpeedMap["GB:motorway"] = seventy;
    //     mySpeedMap["GB:nsl_dual"] = seventy;
    //     mySpeedMap["GB:nsl_single"] = sixty;
    //     mySpeedMap["UK:motorway"] = seventy;
    //     mySpeedMap["UK:nsl_dual"] = seventy;
    //     mySpeedMap["UK:nsl_single"] = sixty;
    //     mySpeedMap["UZ:living_street"] = 30. / 3.6;
    //     mySpeedMap["UZ:urban"] = 70. / 3.6;
    //     mySpeedMap["UZ:rural"] = 100. / 3.6;
    //     mySpeedMap["UZ:motorway"] = 110. / 3.6;
    // }
    #[allow(unused_variables)]
    pub fn new(osmNodes: &BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>, toFill: &mut BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>, platformShapes: &mut BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>, tc: &NBTypeCont) -> Self {
        let unlimitedSpeed = utils::options::options_cont::get_float("osm.speedlimit-none");
        let seventy = utils::common::string_utils::parse_speed("70mph", true).unwrap();
        let sixty = utils::common::string_utils::parse_speed("60mph", true).unwrap();
        let mySpeedMap: BTreeMap<String, f64> = BTreeMap::from([
            ("nan".to_string(), MAXSPEED_UNGIVEN),
            ("sign".to_string(), MAXSPEED_UNGIVEN),
            ("signals".to_string(), MAXSPEED_UNGIVEN),
            ("none".to_string(), unlimitedSpeed),
            ("no".to_string(), unlimitedSpeed),
            ("walk".to_string(), 5. / 3.6),
            ("AT:urban".to_string(), 50. / 3.6),
            ("AT:rural".to_string(), 100. / 3.6),
            ("AT:trunk".to_string(), 100. / 3.6),
            ("AT:motorway".to_string(), 130. / 3.6),
            ("AU:urban".to_string(), 50. / 3.6),
            ("BE:urban".to_string(), 50. / 3.6),
            ("BE:zone".to_string(), 30. / 3.6),
            ("BE:motorway".to_string(), 120. / 3.6),
            ("BE:zone30".to_string(), 30. / 3.6),
            ("BE-VLG:rural".to_string(), 70. / 3.6),
            ("BE-WAL:rural".to_string(), 90. / 3.6),
            ("BE:school".to_string(), 30. / 3.6),
            ("CZ:motorway".to_string(), 130. / 3.6),
            ("CZ:trunk".to_string(), 110. / 3.6),
            ("CZ:rural".to_string(), 90. / 3.6),
            ("CZ:urban_motorway".to_string(), 80. / 3.6),
            ("CZ:urban_trunk".to_string(), 80. / 3.6),
            ("CZ:urban".to_string(), 50. / 3.6),
            ("DE:motorway".to_string(), unlimitedSpeed),
            ("DE:rural".to_string(), 100. / 3.6),
            ("DE:urban".to_string(), 50. / 3.6),
            ("DE:bicycle_road".to_string(), 30. / 3.6),
            ("DK:motorway".to_string(), 130. / 3.6),
            ("DK:rural".to_string(), 80. / 3.6),
            ("DK:urban".to_string(), 50. / 3.6),
            ("EE:urban".to_string(), 50. / 3.6),
            ("EE:rural".to_string(), 90. / 3.6),
            ("ES:urban".to_string(), 50. / 3.6),
            ("ES:zone30".to_string(), 30. / 3.6),
            ("FR:motorway".to_string(), 130. / 3.6),
            ("FR:rural".to_string(), 80. / 3.6),
            ("FR:urban".to_string(), 50. / 3.6),
            ("FR:zone30".to_string(), 30. / 3.6),
            ("HU:living_street".to_string(), 20. / 3.6),
            ("HU:motorway".to_string(), 130. / 3.6),
            ("HU:rural".to_string(), 90. / 3.6),
            ("HU:trunk".to_string(), 110. / 3.6),
            ("HU:urban".to_string(), 50. / 3.6),
            ("IT:rural".to_string(), 90. / 3.6),
            ("IT:motorway".to_string(), 130. / 3.6),
            ("IT:urban".to_string(), 50. / 3.6),
            ("JP:nsl".to_string(), 60. / 3.6),
            ("JP:express".to_string(), 100. / 3.6),
            ("LT:rural".to_string(), 90. / 3.6),
            ("LT:urban".to_string(), 50. / 3.6),
            ("NO:rural".to_string(), 80. / 3.6),
            ("NO:urban".to_string(), 50. / 3.6),
            ("ON:urban".to_string(), 50. / 3.6),
            ("ON:rural".to_string(), 80. / 3.6),
            ("PT:motorway".to_string(), 120. / 3.6),
            ("PT:rural".to_string(), 90. / 3.6),
            ("PT:trunk".to_string(), 100. / 3.6),
            ("PT:urban".to_string(), 50. / 3.6),
            ("RO:motorway".to_string(), 130. / 3.6),
            ("RO:rural".to_string(), 90. / 3.6),
            ("RO:trunk".to_string(), 100. / 3.6),
            ("RO:urban".to_string(), 50. / 3.6),
            ("RS:living_street".to_string(), 30. / 3.6),
            ("RS:motorway".to_string(), 130. / 3.6),
            ("RS:rural".to_string(), 80. / 3.6),
            ("RS:trunk".to_string(), 100. / 3.6),
            ("RS:urban".to_string(), 50. / 3.6),
            ("RU:living_street".to_string(), 20. / 3.6),
            ("RU:urban".to_string(), 60. / 3.6),
            ("RU:rural".to_string(), 90. / 3.6),
            ("RU:motorway".to_string(), 110. / 3.6),
            ("GB:motorway".to_string(), seventy),
            ("GB:nsl_dual".to_string(), seventy),
            ("GB:nsl_single".to_string(), sixty),
            ("UK:motorway".to_string(), seventy),
            ("UK:nsl_dual".to_string(), seventy),
            ("UK:nsl_single".to_string(), sixty),
            ("UZ:living_street".to_string(), 30. / 3.6),
            ("UZ:urban".to_string(), 70. / 3.6),
            ("UZ:rural".to_string(), 100. / 3.6),
            ("UZ:motorway".to_string(), 110. / 3.6),
        ]);
        NIImporter_OpenStreetMap_EdgesHandler {
            base: utils::xml::sumo_sax_handler::new("osm - file", ""),
            myOSMNodes: Ptr::new(osmNodes),
            myEdgeMap: Ptr::from_mut(toFill),
            myPlatformShapesMap: Ptr::from_mut(platformShapes),
            myCurrentEdge: Ptr::null(),
            mySpeedMap,
            myTypeCont: Ptr::new(tc),
        }
    }

    /// C++: `void EdgesHandler::~EdgesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_OpenStreetMap::EdgesHandler::~EdgesHandler()
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty; Rust drops the members.
    }

    /// C++: `void EdgesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_OpenStreetMap::EdgesHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_WAY) {
    //         bool ok = true;
    //         const long long int id = attrs.get<long long int>(SUMO_ATTR_ID, nullptr, ok);
    //         const std::string& action = attrs.getOpt<std::string>(SUMO_ATTR_ACTION, nullptr, ok);
    //         if (action == "delete" || !ok) {
    //             myCurrentEdge = nullptr;
    //             return;
    //         }
    //         myCurrentEdge = new Edge(id);
    //     }
    //     // parse "nd" (node) elements
    //     if (element == SUMO_TAG_ND && myCurrentEdge != nullptr) {
    //         bool ok = true;
    //         long long int ref = attrs.get<long long int>(SUMO_ATTR_REF, nullptr, ok);
    //         if (ok) {
    //             auto node = myOSMNodes.find(ref);
    //             if (node == myOSMNodes.end()) {
    //                 WRITE_WARNINGF(TL("The referenced geometry information (ref='%') is not known"), toString(ref));
    //                 return;
    //             }
    //
    //             ref = node->second->id; // node may have been substituted
    //             if (myCurrentEdge->myCurrentNodes.empty() ||
    //                     myCurrentEdge->myCurrentNodes.back() != ref) { // avoid consecutive duplicates
    //                 myCurrentEdge->myCurrentNodes.push_back(ref);
    //             }
    //
    //         }
    //     }
    //     if (element == SUMO_TAG_TAG && myCurrentEdge != nullptr) {
    //         bool ok = true;
    //         std::string key = attrs.get<std::string>(SUMO_ATTR_K, toString(myCurrentEdge->id).c_str(), ok, false);
    //         if (key.size() > 6 && StringUtils::startsWith(key, "busway:")) {
    //             // handle special busway keys
    //             const std::string buswaySpec = key.substr(7);
    //             key = "busway";
    //             if (buswaySpec == "right") {
    //                 myCurrentEdge->myBuswayType = (WayType)(myCurrentEdge->myBuswayType | WAY_FORWARD);
    //             } else if (buswaySpec == "left") {
    //                 myCurrentEdge->myBuswayType = (WayType)(myCurrentEdge->myBuswayType | WAY_BACKWARD);
    //             } else if (buswaySpec == "both") {
    //                 myCurrentEdge->myBuswayType = (WayType)(myCurrentEdge->myBuswayType | WAY_BOTH);
    //             } else {
    //                 key = "ignore";
    //             }
    //         }
    //         if (myAllAttributes && (myExtraAttributes.count(key) != 0 || myExtraAttributes.size() == 0)) {
    //             const std::string info = "way=" + toString(myCurrentEdge->id) + ", k=" + key;
    //             myCurrentEdge->setParameter(key, attrs.get<std::string>(SUMO_ATTR_V, info.c_str(), ok, false));
    //         }
    //         // we check whether the key is relevant (and we really need to transcode the value) to avoid hitting #1636
    //         if (!StringUtils::endsWith(key, "way")
    //                 && !StringUtils::startsWith(key, "lanes")
    //                 && key != "maxspeed" && key != "maxspeed:type"
    //                 && key != "zone:maxspeed"
    //                 && key != "maxspeed:forward" && key != "maxspeed:backward"
    //                 && key != "junction" && key != "name" && key != "tracks" && key != "layer"
    //                 && key != "route"
    //                 && !StringUtils::startsWith(key, "cycleway")
    //                 && !StringUtils::startsWith(key, "sidewalk")
    //                 && key != "ref"
    //                 && key != "highspeed"
    //                 && !StringUtils::startsWith(key, "parking")
    //                 && !StringUtils::startsWith(key, "change")
    //                 && !StringUtils::startsWith(key, "vehicle:lanes")
    //                 && key != "postal_code"
    //                 && key != "railway:preferred_direction"
    //                 && key != "railway:bidirectional"
    //                 && key != "railway:track_ref"
    //                 && key != "usage"
    //                 && key != "access"
    //                 && key != "emergency"
    //                 && key != "service"
    //                 && key != "electrified"
    //                 && key != "segregated"
    //                 && key != "bus"
    //                 && key != "psv"
    //                 && key != "foot"
    //                 && key != "bicycle"
    //                 && key != "oneway:bicycle"
    //                 && key != "oneway:bus"
    //                 && key != "oneway:psv"
    //                 && key != "placement"
    //                 && key != "bus:lanes"
    //                 && key != "bus:lanes:forward"
    //                 && key != "bus:lanes:backward"
    //                 && key != "psv:lanes"
    //                 && key != "psv:lanes:forward"
    //                 && key != "psv:lanes:backward"
    //                 && key != "bicycle:lanes"
    //                 && key != "bicycle:lanes:forward"
    //                 && key != "bicycle:lanes:backward"
    //                 && !StringUtils::startsWith(key, "width")
    //                 && !(StringUtils::startsWith(key, "turn:") && key.find(":lanes") != std::string::npos)
    //                 && key != "public_transport") {
    //             return;
    //         }
    //         const std::string value = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentEdge->id).c_str(), ok, false);
    //
    //         if (key == "highway" || key == "railway" || key == "waterway" || StringUtils::startsWith(key, "cycleway")
    //                 || key == "busway" || key == "route" || StringUtils::startsWith(key, "sidewalk") || key == "highspeed"
    //                 || key == "aeroway" || key == "aerialway" || key == "usage" || key == "service") {
    //             // build type id
    //             if (key != "highway" || myTypeCont.knows(key + "." + value)) {
    //                 myCurrentEdge->myCurrentIsRoad = true;
    //             }
    //             // special cycleway stuff https://wiki.openstreetmap.org/wiki/Key:cycleway
    //             if (key == "cycleway") {
    //                 if (value == "no" || value == "none" || value == "separate") {
    //                     myCurrentEdge->myCyclewayType = WAY_NONE;
    //                 } else if (value == "both") {
    //                     myCurrentEdge->myCyclewayType = WAY_BOTH;
    //                 } else if (value == "right") {
    //                     myCurrentEdge->myCyclewayType = WAY_FORWARD;
    //                 } else if (value == "left") {
    //                     myCurrentEdge->myCyclewayType = WAY_BACKWARD;
    //                 } else if (value == "opposite_track") {
    //                     myCurrentEdge->myCyclewayType = WAY_BACKWARD;
    //                 } else if (value == "opposite_lane") {
    //                     myCurrentEdge->myCyclewayType = WAY_BACKWARD;
    //                 } else if (value == "opposite") {
    //                     // according to the wiki ref above, this should rather be a bidi lane, see #13438
    //                     myCurrentEdge->myCyclewayType = WAY_BACKWARD;
    //                 }
    //             }
    //             if (key == "cycleway:left") {
    //                 if (myCurrentEdge->myCyclewayType == WAY_UNKNOWN) {
    //                     myCurrentEdge->myCyclewayType = WAY_NONE;
    //                 }
    //                 if (value == "yes" || value == "lane" || value == "track") {
    //                     myCurrentEdge->myCyclewayType = (WayType)(myCurrentEdge->myCyclewayType | WAY_BACKWARD);
    //                 }
    //                 key = "cycleway"; // for type adaption
    //             }
    //             if (key == "cycleway:right") {
    //                 if (myCurrentEdge->myCyclewayType == WAY_UNKNOWN) {
    //                     myCurrentEdge->myCyclewayType = WAY_NONE;
    //                 }
    //                 if (value == "yes" || value == "lane" || value == "track") {
    //                     myCurrentEdge->myCyclewayType = (WayType)(myCurrentEdge->myCyclewayType | WAY_FORWARD);
    //                 }
    //                 key = "cycleway"; // for type adaption
    //             }
    //             if (key == "cycleway:both") {
    //                 if (myCurrentEdge->myCyclewayType == WAY_UNKNOWN) {
    //                     if (value == "no" || value == "none" || value == "separate") {
    //                         myCurrentEdge->myCyclewayType = WAY_NONE;
    //                     }
    //                     if (value == "yes" || value == "lane" || value == "track") {
    //                         myCurrentEdge->myCyclewayType = WAY_BOTH;
    //                     }
    //                 }
    //                 key = "cycleway"; // for type adaption
    //             }
    //             if (key == "cycleway" && value != "lane" && value != "track" && value != "opposite_track" && value != "opposite_lane") {
    //                 // typemap covers only the lane and track cases
    //                 return;
    //             }
    //             if (StringUtils::startsWith(key, "cycleway:")) {
    //                 // no need to extend the type id for other cycleway sub tags
    //                 return;
    //             }
    //             // special sidewalk stuff
    //             if (key == "sidewalk") {
    //                 if (value == "no" || value == "none" || value == "separate") {
    //                     myCurrentEdge->mySidewalkType = WAY_NONE;
    //                     if (value == "separate") {
    //                         myCurrentEdge->myExtraDisallowed |= SVC_PEDESTRIAN;
    //                     }
    //                 } else if (value == "both" || value == "yes") {
    //                     myCurrentEdge->mySidewalkType = WAY_BOTH;
    //                 } else if (value == "right") {
    //                     myCurrentEdge->mySidewalkType = WAY_FORWARD;
    //                 } else if (value == "left") {
    //                     myCurrentEdge->mySidewalkType = WAY_BACKWARD;
    //                 }
    //             }
    //             if (key == "sidewalk:left") {
    //                 if (myCurrentEdge->mySidewalkType == WAY_UNKNOWN) {
    //                     myCurrentEdge->mySidewalkType = WAY_NONE;
    //                 }
    //                 if (value == "yes") {
    //                     myCurrentEdge->mySidewalkType = (WayType)(myCurrentEdge->mySidewalkType | WAY_BACKWARD);
    //                 }
    //             }
    //             if (key == "sidewalk:right") {
    //                 if (myCurrentEdge->mySidewalkType == WAY_UNKNOWN) {
    //                     myCurrentEdge->mySidewalkType = WAY_NONE;
    //                 }
    //                 if (value == "yes") {
    //                     myCurrentEdge->mySidewalkType = (WayType)(myCurrentEdge->mySidewalkType | WAY_FORWARD);
    //                 }
    //                 if (value == "separate") {
    //                     myCurrentEdge->myExtraDisallowed |= SVC_PEDESTRIAN;
    //                 }
    //             }
    //             if (key == "sidewalk:both") {
    //                 if (myCurrentEdge->mySidewalkType == WAY_UNKNOWN) {
    //                     if (value == "no" || value == "none" || value == "separate") {
    //                         myCurrentEdge->mySidewalkType = WAY_NONE;
    //                         if (value == "separate") {
    //                             myCurrentEdge->myExtraDisallowed |= SVC_PEDESTRIAN;
    //                         }
    //                     }
    //                     if (value == "yes") {
    //                         myCurrentEdge->mySidewalkType = WAY_BOTH;
    //                     }
    //                 }
    //             }
    //             if (StringUtils::startsWith(key, "sidewalk")) {
    //                 // no need to extend the type id
    //                 return;
    //             }
    //             // special busway stuff
    //             if (key == "busway") {
    //                 if (value == "no") {
    //                     return;
    //                 }
    //                 if (value == "opposite_track") {
    //                     myCurrentEdge->myBuswayType = WAY_BACKWARD;
    //                 } else if (value == "opposite_lane") {
    //                     myCurrentEdge->myBuswayType = WAY_BACKWARD;
    //                 }
    //                 // no need to extend the type id
    //                 return;
    //             }
    //             std::string singleTypeID = key + "." + value;
    //             if (key == "highspeed") {
    //                 if (value == "no") {
    //                     return;
    //                 }
    //                 singleTypeID = "railway.highspeed";
    //             }
    //             addType(singleTypeID);
    //
    //         } else if (key == "bus" || key == "psv") {
    //             // 'psv' includes taxi in the UK but not in germany
    //             try {
    //                 if (StringUtils::toBool(value)) {
    //                     myCurrentEdge->myExtraAllowed |= SVC_BUS;
    //                     addType(key);
    //                 } else {
    //                     myCurrentEdge->myExtraDisallowed |= SVC_BUS;
    //                 }
    //             } catch (const BoolFormatException&) {
    //                 myCurrentEdge->myExtraAllowed |= SVC_BUS;
    //                 addType(key);
    //             }
    //         } else if (key == "emergency") {
    //             try {
    //                 if (StringUtils::toBool(value)) {
    //                     myCurrentEdge->myExtraAllowed |= SVC_AUTHORITY | SVC_EMERGENCY;
    //                 }
    //             } catch (const BoolFormatException&) {
    //                 myCurrentEdge->myExtraAllowed |= SVC_AUTHORITY | SVC_EMERGENCY;
    //             }
    //         } else if (key == "access") {
    //             if (value == "no") {
    //                 myCurrentEdge->myExtraDisallowed |= ~(SVC_PUBLIC_CLASSES | SVC_EMERGENCY | SVC_AUTHORITY);
    //             }
    //         } else if (StringUtils::startsWith(key, "width:lanes")) {
    //             try {
    //                 const std::vector<std::string> values = StringTokenizer(value, "|").getVector();
    //                 std::vector<double> widthLanes;
    //                 for (std::string width : values) {
    //                     const double parsedWidth = width == "" ? -1 : StringUtils::parseDist(width);
    //                     widthLanes.push_back(parsedWidth);
    //                 }
    //
    //                 if (key == "width:lanes" || key == "width:lanes:forward") {
    //                     myCurrentEdge->myWidthLanesForward = widthLanes;
    //                 } else if (key == "width:lanes:backward") {
    //                     myCurrentEdge->myWidthLanesBackward = widthLanes;
    //                 } else {
    //                     WRITE_WARNINGF(TL("Using default lane width for edge '%' as key '%' could not be parsed."), toString(myCurrentEdge->id), key);
    //                 }
    //             } catch (const NumberFormatException&) {
    //                 WRITE_WARNINGF(TL("Using default lane width for edge '%' as value '%' could not be parsed."), toString(myCurrentEdge->id), value);
    //             }
    //         } else if (key == "width") {
    //             try {
    //                 myCurrentEdge->myWidth = StringUtils::parseDist(value);
    //             } catch (const NumberFormatException&) {
    //                 WRITE_WARNINGF(TL("Using default width for edge '%' as value '%' could not be parsed."), toString(myCurrentEdge->id), value);
    //             }
    //         } else if (key == "foot") {
    //             if (value == "use_sidepath" || value == "no") {
    //                 myCurrentEdge->myExtraDisallowed |= SVC_PEDESTRIAN;
    //             } else if (value == "yes" || value == "designated" || value == "permissive") {
    //                 myCurrentEdge->myExtraAllowed |= SVC_PEDESTRIAN;
    //             }
    //         } else if (key == "bicycle") {
    //             if (value == "use_sidepath" || value == "no") {
    //                 myCurrentEdge->myExtraDisallowed |= SVC_BICYCLE;
    //             } else if (value == "yes" || value == "designated" || value == "permissive") {
    //                 myCurrentEdge->myExtraAllowed |= SVC_BICYCLE;
    //             }
    //         } else if (key == "oneway:bicycle") {
    //             myCurrentEdge->myExtraTags["oneway:bicycle"] = value;
    //         } else if (key == "oneway:bus" || key == "oneway:psv") {
    //             if (value == "no") {
    //                 // need to add a bus way in reversed direction of way
    //                 myCurrentEdge->myBuswayType = WAY_BACKWARD;
    //             }
    //         } else if (key == "placement") {
    //             if (!interpretPlacement(value, myCurrentEdge->myPlacement, myCurrentEdge->myPlacementLane)) {
    //                 WRITE_WARNINGF(TL("Ignoring unsupported placement value '%' for edge '%'."), value, myCurrentEdge->id);
    //             }
    //         } else if (key == "lanes") {
    //             try {
    //                 myCurrentEdge->myNoLanes = StringUtils::toInt(value);
    //             } catch (NumberFormatException&) {
    //                 // might be a list of values
    //                 StringTokenizer st(value, ";", true);
    //                 std::vector<std::string> list = st.getVector();
    //                 if (list.size() >= 2) {
    //                     int minLanes = std::numeric_limits<int>::max();
    //                     try {
    //                         for (auto& i : list) {
    //                             const int numLanes = StringUtils::toInt(StringUtils::prune(i));
    //                             minLanes = MIN2(minLanes, numLanes);
    //                         }
    //                         myCurrentEdge->myNoLanes = minLanes;
    //                         WRITE_WARNINGF(TL("Using minimum lane number from list (%) for edge '%'."), value, toString(myCurrentEdge->id));
    //                     } catch (NumberFormatException&) {
    //                         WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in edge '%'."), key, value, myCurrentEdge->id);
    //                     }
    //                 }
    //             } catch (EmptyData&) {
    //                 WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in edge '%'."), key, value, myCurrentEdge->id);
    //             }
    //         } else if (key == "lanes:forward") {
    //             try {
    //                 const int numLanes = StringUtils::toInt(value);
    //                 if (myCurrentEdge->myNoLanesForward < 0 && myCurrentEdge->myNoLanes < 0) {
    //                     // fix lane count in case only lanes:forward and lanes:backward are set
    //                     myCurrentEdge->myNoLanes = numLanes - myCurrentEdge->myNoLanesForward;
    //                 }
    //                 myCurrentEdge->myNoLanesForward = numLanes;
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in edge '%'."), key, value, myCurrentEdge->id);
    //             }
    //         } else if (key == "lanes:backward") {
    //             try {
    //                 const int numLanes = StringUtils::toInt(value);
    //                 if (myCurrentEdge->myNoLanesForward > 0 && myCurrentEdge->myNoLanes < 0) {
    //                     // fix lane count in case only lanes:forward and lanes:backward are set
    //                     myCurrentEdge->myNoLanes = numLanes + myCurrentEdge->myNoLanesForward;
    //                 }
    //                 // denote backwards count with a negative sign
    //                 myCurrentEdge->myNoLanesForward = -numLanes;
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in edge '%'."), key, value, myCurrentEdge->id);
    //             }
    //         } else if (myCurrentEdge->myMaxSpeed == MAXSPEED_UNGIVEN &&
    //                    (key == "maxspeed" || key == "maxspeed:type" || key == "maxspeed:forward" || key == "zone:maxspeed")) {
    //             // both 'maxspeed' and 'maxspeed:type' may be given so we must take care not to overwrite an already seen value
    //             myCurrentEdge->myMaxSpeed = interpretSpeed(key, value);
    //         } else if (key == "maxspeed:backward" && myCurrentEdge->myMaxSpeedBackward == MAXSPEED_UNGIVEN) {
    //             myCurrentEdge->myMaxSpeedBackward = interpretSpeed(key, value);
    //         } else if (key == "junction") {
    //             if ((value == "roundabout" || value == "circular") && myCurrentEdge->myIsOneWay.empty()) {
    //                 myCurrentEdge->myIsOneWay = "yes";
    //             }
    //             if (value == "roundabout") {
    //                 myCurrentEdge->myAmInRoundabout = true;
    //             }
    //         } else if (key == "oneway") {
    //             myCurrentEdge->myIsOneWay = value;
    //         } else if (key == "name") {
    //             myCurrentEdge->streetName = value;
    //         } else if (key == "ref") {
    //             myCurrentEdge->ref = value;
    //             myCurrentEdge->setParameter("ref", value);
    //         } else if (key == "layer") {
    //             try {
    //                 myCurrentEdge->myLayer = StringUtils::toInt(value);
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in edge '%'."), key, value, myCurrentEdge->id);
    //             }
    //         } else if (key == "tracks") {
    //             try {
    //                 if (StringUtils::toInt(value) == 1) {
    //                     myCurrentEdge->myIsOneWay = "true";
    //                 } else {
    //                     WRITE_WARNINGF(TL("Ignoring track count % for edge '%'."), value, myCurrentEdge->id);
    //                 }
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in edge '%'."), key, value, myCurrentEdge->id);
    //             }
    //         } else if (key == "railway:preferred_direction") {
    //             if (value == "both") {
    //                 myCurrentEdge->myRailDirection = WAY_BOTH | WAY_PREFER_FORWARD | WAY_PREFER_BACKWARD;
    //             } else if (value == "backward") {
    //                 myCurrentEdge->myRailDirection = (myCurrentEdge->myRailDirection | WAY_BACKWARD | WAY_PREFER_BACKWARD) & ~WAY_UNKNOWN;
    //             } else if (value == "forward") {
    //                 myCurrentEdge->myRailDirection = (myCurrentEdge->myRailDirection | WAY_FORWARD | WAY_PREFER_FORWARD) & ~WAY_UNKNOWN;
    //             }
    //         } else if (key == "railway:bidirectional") {
    //             if (value == "regular") {
    //                 myCurrentEdge->myRailDirection = (myCurrentEdge->myRailDirection | WAY_BOTH) & ~WAY_UNKNOWN;
    //             }
    //         } else if (key == "electrified" || key == "segregated") {
    //             if (value != "no") {
    //                 myCurrentEdge->myExtraTags[key] = value;
    //             }
    //         } else if (key == "railway:track_ref") {
    //             myCurrentEdge->setParameter(key, value);
    //         } else if (key == "public_transport" && value == "platform") {
    //             myCurrentEdge->myExtraTags["platform"] = "yes";
    //         } else if ((key == "parking:both" || key == "parking:lane:both") && !StringUtils::startsWith(value, "no")) {
    //             myCurrentEdge->myParkingType |= PARKING_BOTH;
    //         } else if ((key == "parking:left" || key == "parking:lane:left") && !StringUtils::startsWith(value, "no")) {
    //             myCurrentEdge->myParkingType |= PARKING_LEFT;
    //         } else if ((key == "parking:right" || key == "parking:lane:right") && !StringUtils::startsWith(value, "no")) {
    //             myCurrentEdge->myParkingType |= PARKING_RIGHT;
    //         } else if (key == "change" || key == "change:lanes") {
    //             myCurrentEdge->myChangeForward = myCurrentEdge->myChangeBackward = interpretChangeType(value);
    //         } else if (key == "change:forward" || key == "change:lanes:forward") {
    //             myCurrentEdge->myChangeForward = interpretChangeType(value);
    //         } else if (key == "change:backward" || key == "change:lanes:backward") {
    //             myCurrentEdge->myChangeBackward = interpretChangeType(value);
    //         } else if (key == "vehicle:lanes" || key == "vehicle:lanes:forward") {
    //             interpretLaneUse(value, SVC_PASSENGER, true);
    //             interpretLaneUse(value, SVC_PRIVATE, true);
    //         } else if (key == "vehicle:lanes:backward") {
    //             interpretLaneUse(value, SVC_PASSENGER, false);
    //             interpretLaneUse(value, SVC_PRIVATE, false);
    //         } else if (key == "bus:lanes" || key == "bus:lanes:forward") {
    //             interpretLaneUse(value, SVC_BUS, true);
    //         } else if (key == "bus:lanes:backward") {
    //             interpretLaneUse(value, SVC_BUS, false);
    //         } else if (key == "psv:lanes" || key == "psv:lanes:forward") {
    //             interpretLaneUse(value, SVC_BUS, true);
    //             interpretLaneUse(value, SVC_TAXI, true);
    //         } else if (key == "psv:lanes:backward") {
    //             interpretLaneUse(value, SVC_BUS, false);
    //             interpretLaneUse(value, SVC_TAXI, false);
    //         } else if (key == "bicycle:lanes" || key == "bicycle:lanes:forward") {
    //             interpretLaneUse(value, SVC_BICYCLE, true);
    //         } else if (key == "bicycle:lanes:backward") {
    //             interpretLaneUse(value, SVC_BICYCLE, false);
    //         } else if (StringUtils::startsWith(key, "turn:") && key.find(":lanes") != std::string::npos) {
    //             int shift = 0;
    //             // use the first 8 bit to encode permitted directions for all classes
    //             // and the successive 8 bit blocks for selected classes
    //             if (StringUtils::startsWith(key, "turn:bus") || StringUtils::startsWith(key, "turn:psv:")) {
    //                 shift = NBEdge::TURN_SIGN_SHIFT_BUS;
    //             } else if (StringUtils::startsWith(key, "turn:taxi")) {
    //                 shift = NBEdge::TURN_SIGN_SHIFT_TAXI;
    //             } else if (StringUtils::startsWith(key, "turn:bicycle")) {
    //                 shift = NBEdge::TURN_SIGN_SHIFT_BICYCLE;
    //             }
    //             const std::vector<std::string> values = StringTokenizer(value, "|").getVector();
    //             std::vector<int> turnCodes;
    //             for (std::string codeList : values) {
    //                 const std::vector<std::string> codes = StringTokenizer(codeList, ";").getVector();
    //                 int turnCode = 0;
    //                 if (codes.size() == 0) {
    //                     turnCode = (int)LinkDirection::STRAIGHT;
    //                 }
    //                 for (std::string code : codes) {
    //                     if (code == "" || code == "none" || code == "through") {
    //                         turnCode |= (int)LinkDirection::STRAIGHT << shift ;
    //                     } else if (code == "left" || code == "sharp_left") {
    //                         turnCode |= (int)LinkDirection::LEFT << shift;
    //                     } else if (code == "right" || code == "sharp_right") {
    //                         turnCode |= (int)LinkDirection::RIGHT << shift;
    //                     } else if (code == "slight_left") {
    //                         turnCode |= (int)LinkDirection::PARTLEFT << shift;
    //                     } else if (code == "slight_right") {
    //                         turnCode |= (int)LinkDirection::PARTRIGHT << shift;
    //                     } else if (code == "reverse") {
    //                         turnCode |= (int)LinkDirection::TURN << shift;
    //                     } else if (code == "merge_to_left" || code == "merge_to_right") {
    //                         turnCode |= (int)LinkDirection::NODIR << shift;
    //                     }
    //                 }
    //                 turnCodes.push_back(turnCode);
    //             }
    //             if (StringUtils::endsWith(key, "lanes") || StringUtils::endsWith(key, "lanes:forward")) {
    //                 mergeTurnSigns(myCurrentEdge->myTurnSignsForward, turnCodes);
    //             } else if (StringUtils::endsWith(key, "lanes:backward")) {
    //                 mergeTurnSigns(myCurrentEdge->myTurnSignsBackward, turnCodes);
    //             } else if (StringUtils::endsWith(key, "lanes:both_ways")) {
    //                 mergeTurnSigns(myCurrentEdge->myTurnSignsForward, turnCodes);
    //                 mergeTurnSigns(myCurrentEdge->myTurnSignsBackward, turnCodes);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element == SUMO_TAG_WAY {
            let mut ok = true;
            let id: i64 = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
            let action: String = attrs.get_opt(SUMO_ATTR_ACTION, "", &mut ok, String::new(), true);
            if action == "delete" || !ok {
                self.myCurrentEdge = Ptr::null();
                return;
            }
            self.myCurrentEdge = Ptr::from_raw(Box::into_raw(Box::new(NIImporter_OpenStreetMap_Edge::new(id))));
        }
        // parse "nd" (node) elements
        if element == SUMO_TAG_ND && !self.myCurrentEdge.is_null() {
            let mut ok = true;
            let mut r: i64 = attrs.get(SUMO_ATTR_REF, "", &mut ok, true);
            if ok {
                let Some(node) = (*self.myOSMNodes).get(&r) else {
                    write_warningf!("The referenced geometry information (ref='%') is not known", r);
                    return;
                };
                r = node.id; // node may have been substituted
                let ce = self.myCurrentEdge;
                if ce.myCurrentNodes.last() != Some(&r) { // avoid consecutive duplicates
                    ce.m().myCurrentNodes.push(r);
                }
            }
        }
        if element == SUMO_TAG_TAG && !self.myCurrentEdge.is_null() {
            let mut ok = true;
            let ce = self.myCurrentEdge;
            let mut key: String = attrs.get(SUMO_ATTR_K, &ce.id.to_string(), &mut ok, false);
            if key.len() > 6 && utils::common::string_utils::starts_with(&key, "busway:") {
                // handle special busway keys
                let buswaySpec = key[7..].to_string();
                key = "busway".to_string();
                if buswaySpec == "right" {
                    ce.m().myBuswayType |= NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i64;
                } else if buswaySpec == "left" {
                    ce.m().myBuswayType |= NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                } else if buswaySpec == "both" {
                    ce.m().myBuswayType |= NIImporter_OpenStreetMap_WayType::WAY_BOTH as i64;
                } else {
                    key = "ignore".to_string();
                }
            }
            if myAllAttributes.with(|c| c.get()) && myExtraAttributes.with(|s| s.borrow().contains(&key) || s.borrow().is_empty()) {
                let info = format!("way={}, k={}", ce.id, key);
                let v: String = attrs.get(SUMO_ATTR_V, &info, &mut ok, false);
                ce.m().set_parameter(&key, &v);
            }
            // we check whether the key is relevant (and we really need to transcode the value) to avoid hitting #1636
            if !utils::common::string_utils::ends_with(&key, "way")
                && !utils::common::string_utils::starts_with(&key, "lanes")
                && key != "maxspeed" && key != "maxspeed:type"
                && key != "zone:maxspeed"
                && key != "maxspeed:forward" && key != "maxspeed:backward"
                && key != "junction" && key != "name" && key != "tracks" && key != "layer"
                && key != "route"
                && !utils::common::string_utils::starts_with(&key, "cycleway")
                && !utils::common::string_utils::starts_with(&key, "sidewalk")
                && key != "ref"
                && key != "highspeed"
                && !utils::common::string_utils::starts_with(&key, "parking")
                && !utils::common::string_utils::starts_with(&key, "change")
                && !utils::common::string_utils::starts_with(&key, "vehicle:lanes")
                && key != "postal_code"
                && key != "railway:preferred_direction"
                && key != "railway:bidirectional"
                && key != "railway:track_ref"
                && key != "usage"
                && key != "access"
                && key != "emergency"
                && key != "service"
                && key != "electrified"
                && key != "segregated"
                && key != "bus"
                && key != "psv"
                && key != "foot"
                && key != "bicycle"
                && key != "oneway:bicycle"
                && key != "oneway:bus"
                && key != "oneway:psv"
                && key != "placement"
                && key != "bus:lanes"
                && key != "bus:lanes:forward"
                && key != "bus:lanes:backward"
                && key != "psv:lanes"
                && key != "psv:lanes:forward"
                && key != "psv:lanes:backward"
                && key != "bicycle:lanes"
                && key != "bicycle:lanes:forward"
                && key != "bicycle:lanes:backward"
                && !utils::common::string_utils::starts_with(&key, "width")
                && !(utils::common::string_utils::starts_with(&key, "turn:") && key.contains(":lanes"))
                && key != "public_transport" {
                return;
            }
            let value: String = attrs.get(SUMO_ATTR_V, &ce.id.to_string(), &mut ok, false);

            if key == "highway" || key == "railway" || key == "waterway" || utils::common::string_utils::starts_with(&key, "cycleway")
                || key == "busway" || key == "route" || utils::common::string_utils::starts_with(&key, "sidewalk") || key == "highspeed"
                || key == "aeroway" || key == "aerialway" || key == "usage" || key == "service" {
                // build type id
                if key != "highway" || self.myTypeCont.knows(&format!("{key}.{value}")) {
                    ce.m().myCurrentIsRoad = true;
                }
                // special cycleway stuff https://wiki.openstreetmap.org/wiki/Key:cycleway
                if key == "cycleway" {
                    if value == "no" || value == "none" || value == "separate" {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                    } else if value == "both" {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_BOTH as i64;
                    } else if value == "right" {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i64;
                    } else if value == "left" {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    } else if value == "opposite_track" {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    } else if value == "opposite_lane" {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    } else if value == "opposite" {
                        // according to the wiki ref above, this should rather be a bidi lane, see #13438
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    }
                }
                if key == "cycleway:left" {
                    if ce.myCyclewayType == NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64 {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                    }
                    if value == "yes" || value == "lane" || value == "track" {
                        ce.m().myCyclewayType |= NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    }
                    key = "cycleway".to_string(); // for type adaption
                }
                if key == "cycleway:right" {
                    if ce.myCyclewayType == NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64 {
                        ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                    }
                    if value == "yes" || value == "lane" || value == "track" {
                        ce.m().myCyclewayType |= NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i64;
                    }
                    key = "cycleway".to_string(); // for type adaption
                }
                if key == "cycleway:both" {
                    if ce.myCyclewayType == NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64 {
                        if value == "no" || value == "none" || value == "separate" {
                            ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                        }
                        if value == "yes" || value == "lane" || value == "track" {
                            ce.m().myCyclewayType = NIImporter_OpenStreetMap_WayType::WAY_BOTH as i64;
                        }
                    }
                    key = "cycleway".to_string(); // for type adaption
                }
                if key == "cycleway" && value != "lane" && value != "track" && value != "opposite_track" && value != "opposite_lane" {
                    // typemap covers only the lane and track cases
                    return;
                }
                if utils::common::string_utils::starts_with(&key, "cycleway:") {
                    // no need to extend the type id for other cycleway sub tags
                    return;
                }
                // special sidewalk stuff
                if key == "sidewalk" {
                    if value == "no" || value == "none" || value == "separate" {
                        ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                        if value == "separate" {
                            ce.m().myExtraDisallowed |= SVC_PEDESTRIAN;
                        }
                    } else if value == "both" || value == "yes" {
                        ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_BOTH as i64;
                    } else if value == "right" {
                        ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i64;
                    } else if value == "left" {
                        ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    }
                }
                if key == "sidewalk:left" {
                    if ce.mySidewalkType == NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64 {
                        ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                    }
                    if value == "yes" {
                        ce.m().mySidewalkType |= NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    }
                }
                if key == "sidewalk:right" {
                    if ce.mySidewalkType == NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64 {
                        ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                    }
                    if value == "yes" {
                        ce.m().mySidewalkType |= NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i64;
                    }
                    if value == "separate" {
                        ce.m().myExtraDisallowed |= SVC_PEDESTRIAN;
                    }
                }
                if key == "sidewalk:both" {
                    if ce.mySidewalkType == NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64 {
                        if value == "no" || value == "none" || value == "separate" {
                            ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
                            if value == "separate" {
                                ce.m().myExtraDisallowed |= SVC_PEDESTRIAN;
                            }
                        }
                        if value == "yes" {
                            ce.m().mySidewalkType = NIImporter_OpenStreetMap_WayType::WAY_BOTH as i64;
                        }
                    }
                }
                if utils::common::string_utils::starts_with(&key, "sidewalk") {
                    // no need to extend the type id
                    return;
                }
                // special busway stuff
                if key == "busway" {
                    if value == "no" {
                        return;
                    }
                    if value == "opposite_track" {
                        ce.m().myBuswayType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    } else if value == "opposite_lane" {
                        ce.m().myBuswayType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                    }
                    // no need to extend the type id
                    return;
                }
                let mut singleTypeID = format!("{key}.{value}");
                if key == "highspeed" {
                    if value == "no" {
                        return;
                    }
                    singleTypeID = "railway.highspeed".to_string();
                }
                self.add_type(&singleTypeID);

            } else if key == "bus" || key == "psv" {
                // 'psv' includes taxi in the UK but not in germany
                match utils::common::string_utils::to_bool(&value) {
                    Ok(true) => {
                        ce.m().myExtraAllowed |= SVC_BUS;
                        self.add_type(&key);
                    }
                    Ok(false) => {
                        ce.m().myExtraDisallowed |= SVC_BUS;
                    }
                    Err(ProcessError::BoolFormat(_)) => {
                        ce.m().myExtraAllowed |= SVC_BUS;
                        self.add_type(&key);
                    }
                    Err(e) => panic!("{e}"),
                }
            } else if key == "emergency" {
                match utils::common::string_utils::to_bool(&value) {
                    Ok(true) => {
                        ce.m().myExtraAllowed |= SVC_AUTHORITY | SVC_EMERGENCY;
                    }
                    Ok(false) => {}
                    Err(ProcessError::BoolFormat(_)) => {
                        ce.m().myExtraAllowed |= SVC_AUTHORITY | SVC_EMERGENCY;
                    }
                    Err(e) => panic!("{e}"),
                }
            } else if key == "access" {
                if value == "no" {
                    ce.m().myExtraDisallowed |= !(SVC_PUBLIC_CLASSES | SVC_EMERGENCY | SVC_AUTHORITY);
                }
            } else if utils::common::string_utils::starts_with(&key, "width:lanes") {
                let values = utils::common::string_tokenizer::StringTokenizer::with_token(&value, "|", false).get_vector();
                let mut widthLanes: Vec<f64> = Vec::new();
                let mut parsed_ok = true;
                for width in values {
                    let parsedWidth = if width.is_empty() { Ok(-1.0) } else { utils::common::string_utils::parse_dist(&width) };
                    match parsedWidth {
                        Ok(w) => widthLanes.push(w),
                        Err(_) => {
                            parsed_ok = false;
                            break;
                        }
                    }
                }
                if !parsed_ok {
                    write_warningf!("Using default lane width for edge '%' as value '%' could not be parsed.", ce.id, value);
                } else if key == "width:lanes" || key == "width:lanes:forward" {
                    ce.m().myWidthLanesForward = widthLanes;
                } else if key == "width:lanes:backward" {
                    ce.m().myWidthLanesBackward = widthLanes;
                } else {
                    write_warningf!("Using default lane width for edge '%' as key '%' could not be parsed.", ce.id, key);
                }
            } else if key == "width" {
                match utils::common::string_utils::parse_dist(&value) {
                    Ok(w) => ce.m().myWidth = w,
                    Err(_) => {
                        write_warningf!("Using default width for edge '%' as value '%' could not be parsed.", ce.id, value);
                    }
                }
            } else if key == "foot" {
                if value == "use_sidepath" || value == "no" {
                    ce.m().myExtraDisallowed |= SVC_PEDESTRIAN;
                } else if value == "yes" || value == "designated" || value == "permissive" {
                    ce.m().myExtraAllowed |= SVC_PEDESTRIAN;
                }
            } else if key == "bicycle" {
                if value == "use_sidepath" || value == "no" {
                    ce.m().myExtraDisallowed |= SVC_BICYCLE;
                } else if value == "yes" || value == "designated" || value == "permissive" {
                    ce.m().myExtraAllowed |= SVC_BICYCLE;
                }
            } else if key == "oneway:bicycle" {
                ce.m().myExtraTags.insert("oneway:bicycle".to_string(), value);
            } else if key == "oneway:bus" || key == "oneway:psv" {
                if value == "no" {
                    // need to add a bus way in reversed direction of way
                    ce.m().myBuswayType = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
                }
            } else if key == "placement" {
                let e = ce.m();
                if !self.interpret_placement(&value, &mut e.myPlacement, &mut e.myPlacementLane) {
                    write_warningf!("Ignoring unsupported placement value '%' for edge '%'.", value, ce.id);
                }
            } else if key == "lanes" {
                match utils::common::string_utils::to_int(&value) {
                    Ok(n) => ce.m().myNoLanes = n,
                    Err(ProcessError::NumberFormat(_)) => {
                        // might be a list of values
                        let mut st = utils::common::string_tokenizer::StringTokenizer::with_token(&value, ";", true);
                        let list = st.get_vector();
                        if list.len() >= 2 {
                            let mut minLanes = i32::MAX;
                            let mut failed = false;
                            for i in &list {
                                match utils::common::string_utils::to_int(&utils::common::string_utils::prune(i)) {
                                    Ok(numLanes) => minLanes = minLanes.min(numLanes),
                                    Err(_) => {
                                        failed = true;
                                        break;
                                    }
                                }
                            }
                            if failed {
                                write_warningf!("Value of key '%' is not numeric ('%') in edge '%'.", key, value, ce.id);
                            } else {
                                ce.m().myNoLanes = minLanes;
                                write_warningf!("Using minimum lane number from list (%) for edge '%'.", value, ce.id);
                            }
                        }
                    }
                    Err(_) => {
                        write_warningf!("Value of key '%' is not numeric ('%') in edge '%'.", key, value, ce.id);
                    }
                }
            } else if key == "lanes:forward" {
                match utils::common::string_utils::to_int(&value) {
                    Ok(numLanes) => {
                        let e = ce.m();
                        if e.myNoLanesForward < 0 && e.myNoLanes < 0 {
                            // fix lane count in case only lanes:forward and lanes:backward are set
                            e.myNoLanes = numLanes - e.myNoLanesForward;
                        }
                        e.myNoLanesForward = numLanes;
                    }
                    Err(_) => {
                        write_warningf!("Value of key '%' is not numeric ('%') in edge '%'.", key, value, ce.id);
                    }
                }
            } else if key == "lanes:backward" {
                match utils::common::string_utils::to_int(&value) {
                    Ok(numLanes) => {
                        let e = ce.m();
                        if e.myNoLanesForward > 0 && e.myNoLanes < 0 {
                            // fix lane count in case only lanes:forward and lanes:backward are set
                            e.myNoLanes = numLanes + e.myNoLanesForward;
                        }
                        // denote backwards count with a negative sign
                        e.myNoLanesForward = -numLanes;
                    }
                    Err(_) => {
                        write_warningf!("Value of key '%' is not numeric ('%') in edge '%'.", key, value, ce.id);
                    }
                }
            } else if ce.myMaxSpeed == MAXSPEED_UNGIVEN
                && (key == "maxspeed" || key == "maxspeed:type" || key == "maxspeed:forward" || key == "zone:maxspeed") {
                // both 'maxspeed' and 'maxspeed:type' may be given so we must take care not to overwrite an already seen value
                let s = self.interpret_speed(&key, value.clone());
                ce.m().myMaxSpeed = s;
            } else if key == "maxspeed:backward" && ce.myMaxSpeedBackward == MAXSPEED_UNGIVEN {
                let s = self.interpret_speed(&key, value.clone());
                ce.m().myMaxSpeedBackward = s;
            } else if key == "junction" {
                if (value == "roundabout" || value == "circular") && ce.myIsOneWay.is_empty() {
                    ce.m().myIsOneWay = "yes".to_string();
                }
                if value == "roundabout" {
                    ce.m().myAmInRoundabout = true;
                }
            } else if key == "oneway" {
                ce.m().myIsOneWay = value;
            } else if key == "name" {
                ce.m().streetName = value;
            } else if key == "ref" {
                ce.m().ref_ = value.clone();
                ce.m().set_parameter("ref", &value);
            } else if key == "layer" {
                match utils::common::string_utils::to_int(&value) {
                    Ok(n) => ce.m().myLayer = n,
                    Err(_) => {
                        write_warningf!("Value of key '%' is not numeric ('%') in edge '%'.", key, value, ce.id);
                    }
                }
            } else if key == "tracks" {
                match utils::common::string_utils::to_int(&value) {
                    Ok(1) => {
                        ce.m().myIsOneWay = "true".to_string();
                    }
                    Ok(_) => {
                        write_warningf!("Ignoring track count % for edge '%'.", value, ce.id);
                    }
                    Err(_) => {
                        write_warningf!("Value of key '%' is not numeric ('%') in edge '%'.", key, value, ce.id);
                    }
                }
            } else if key == "railway:preferred_direction" {
                if value == "both" {
                    ce.m().myRailDirection = NIImporter_OpenStreetMap_WayType::WAY_BOTH as i32
                        | NIImporter_OpenStreetMap_WayType::WAY_PREFER_FORWARD as i32
                        | NIImporter_OpenStreetMap_WayType::WAY_PREFER_BACKWARD as i32;
                } else if value == "backward" {
                    let e = ce.m();
                    e.myRailDirection = (e.myRailDirection
                        | NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i32
                        | NIImporter_OpenStreetMap_WayType::WAY_PREFER_BACKWARD as i32)
                        & !(NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i32);
                } else if value == "forward" {
                    let e = ce.m();
                    e.myRailDirection = (e.myRailDirection
                        | NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i32
                        | NIImporter_OpenStreetMap_WayType::WAY_PREFER_FORWARD as i32)
                        & !(NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i32);
                }
            } else if key == "railway:bidirectional" {
                if value == "regular" {
                    let e = ce.m();
                    e.myRailDirection = (e.myRailDirection | NIImporter_OpenStreetMap_WayType::WAY_BOTH as i32)
                        & !(NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i32);
                }
            } else if key == "electrified" || key == "segregated" {
                if value != "no" {
                    ce.m().myExtraTags.insert(key.clone(), value);
                }
            } else if key == "railway:track_ref" {
                ce.m().set_parameter(&key, &value);
            } else if key == "public_transport" && value == "platform" {
                ce.m().myExtraTags.insert("platform".to_string(), "yes".to_string());
            } else if (key == "parking:both" || key == "parking:lane:both") && !utils::common::string_utils::starts_with(&value, "no") {
                ce.m().myParkingType |= NIImporter_OpenStreetMap_ParkingType::PARKING_BOTH as i32;
            } else if (key == "parking:left" || key == "parking:lane:left") && !utils::common::string_utils::starts_with(&value, "no") {
                ce.m().myParkingType |= NIImporter_OpenStreetMap_ParkingType::PARKING_LEFT as i32;
            } else if (key == "parking:right" || key == "parking:lane:right") && !utils::common::string_utils::starts_with(&value, "no") {
                ce.m().myParkingType |= NIImporter_OpenStreetMap_ParkingType::PARKING_RIGHT as i32;
            } else if key == "change" || key == "change:lanes" {
                let c = self.interpret_change_type(&value);
                ce.m().myChangeForward = c;
                ce.m().myChangeBackward = c;
            } else if key == "change:forward" || key == "change:lanes:forward" {
                ce.m().myChangeForward = self.interpret_change_type(&value);
            } else if key == "change:backward" || key == "change:lanes:backward" {
                ce.m().myChangeBackward = self.interpret_change_type(&value);
            } else if key == "vehicle:lanes" || key == "vehicle:lanes:forward" {
                self.interpret_lane_use(&value, SVC_PASSENGER, true);
                self.interpret_lane_use(&value, SVC_PRIVATE, true);
            } else if key == "vehicle:lanes:backward" {
                self.interpret_lane_use(&value, SVC_PASSENGER, false);
                self.interpret_lane_use(&value, SVC_PRIVATE, false);
            } else if key == "bus:lanes" || key == "bus:lanes:forward" {
                self.interpret_lane_use(&value, SVC_BUS, true);
            } else if key == "bus:lanes:backward" {
                self.interpret_lane_use(&value, SVC_BUS, false);
            } else if key == "psv:lanes" || key == "psv:lanes:forward" {
                self.interpret_lane_use(&value, SVC_BUS, true);
                self.interpret_lane_use(&value, SVC_TAXI, true);
            } else if key == "psv:lanes:backward" {
                self.interpret_lane_use(&value, SVC_BUS, false);
                self.interpret_lane_use(&value, SVC_TAXI, false);
            } else if key == "bicycle:lanes" || key == "bicycle:lanes:forward" {
                self.interpret_lane_use(&value, SVC_BICYCLE, true);
            } else if key == "bicycle:lanes:backward" {
                self.interpret_lane_use(&value, SVC_BICYCLE, false);
            } else if utils::common::string_utils::starts_with(&key, "turn:") && key.contains(":lanes") {
                let mut shift: i32 = 0;
                // use the first 8 bit to encode permitted directions for all classes
                // and the successive 8 bit blocks for selected classes
                if utils::common::string_utils::starts_with(&key, "turn:bus") || utils::common::string_utils::starts_with(&key, "turn:psv:") {
                    shift = NBEDGE_TURN_SIGN_SHIFT_BUS;
                } else if utils::common::string_utils::starts_with(&key, "turn:taxi") {
                    shift = NBEDGE_TURN_SIGN_SHIFT_TAXI;
                } else if utils::common::string_utils::starts_with(&key, "turn:bicycle") {
                    shift = NBEDGE_TURN_SIGN_SHIFT_BICYCLE;
                }
                let values = utils::common::string_tokenizer::StringTokenizer::with_token(&value, "|", false).get_vector();
                let mut turnCodes: Vec<i32> = Vec::new();
                for codeList in values {
                    let codes = utils::common::string_tokenizer::StringTokenizer::with_token(&codeList, ";", false).get_vector();
                    let mut turnCode: i32 = 0;
                    if codes.is_empty() {
                        turnCode = LinkDirection::STRAIGHT as i32;
                    }
                    for code in &codes {
                        if code.is_empty() || code == "none" || code == "through" {
                            turnCode |= (LinkDirection::STRAIGHT as i32) << shift;
                        } else if code == "left" || code == "sharp_left" {
                            turnCode |= (LinkDirection::LEFT as i32) << shift;
                        } else if code == "right" || code == "sharp_right" {
                            turnCode |= (LinkDirection::RIGHT as i32) << shift;
                        } else if code == "slight_left" {
                            turnCode |= (LinkDirection::PARTLEFT as i32) << shift;
                        } else if code == "slight_right" {
                            turnCode |= (LinkDirection::PARTRIGHT as i32) << shift;
                        } else if code == "reverse" {
                            turnCode |= (LinkDirection::TURN as i32) << shift;
                        } else if code == "merge_to_left" || code == "merge_to_right" {
                            turnCode |= (LinkDirection::NODIR as i32) << shift;
                        }
                    }
                    turnCodes.push(turnCode);
                }
                if utils::common::string_utils::ends_with(&key, "lanes") || utils::common::string_utils::ends_with(&key, "lanes:forward") {
                    NIImporter_OpenStreetMap::merge_turn_signs(&mut ce.m().myTurnSignsForward, turnCodes);
                } else if utils::common::string_utils::ends_with(&key, "lanes:backward") {
                    NIImporter_OpenStreetMap::merge_turn_signs(&mut ce.m().myTurnSignsBackward, turnCodes);
                } else if utils::common::string_utils::ends_with(&key, "lanes:both_ways") {
                    NIImporter_OpenStreetMap::merge_turn_signs(&mut ce.m().myTurnSignsForward, turnCodes.clone());
                    NIImporter_OpenStreetMap::merge_turn_signs(&mut ce.m().myTurnSignsBackward, turnCodes);
                }
            }
        }
    }

    /// C++: `void EdgesHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIImporter_OpenStreetMap::EdgesHandler::myEndElement(int element) {
    //     if (element == SUMO_TAG_WAY && myCurrentEdge != nullptr) {
    //         if (myCurrentEdge->myCurrentIsRoad) {
    //             const auto insertionIt = myEdgeMap.lower_bound(myCurrentEdge->id);
    //             if (insertionIt == myEdgeMap.end() || insertionIt->first != myCurrentEdge->id) {
    //                 // assume we are loading multiple files, so we won't report duplicate edges
    //                 myEdgeMap.emplace_hint(insertionIt, myCurrentEdge->id, myCurrentEdge);
    //             } else {
    //                 delete myCurrentEdge;
    //             }
    //         } else if (myCurrentEdge->myExtraTags.count("platform") != 0) {
    //             const auto insertionIt = myPlatformShapesMap.lower_bound(myCurrentEdge->id);
    //             if (insertionIt == myPlatformShapesMap.end() || insertionIt->first != myCurrentEdge->id) {
    //                 // assume we are loading multiple files, so we won't report duplicate platforms
    //                 myPlatformShapesMap.emplace_hint(insertionIt, myCurrentEdge->id, myCurrentEdge);
    //             } else {
    //                 delete myCurrentEdge;
    //             }
    //         } else {
    //             delete myCurrentEdge;
    //         }
    //         myCurrentEdge = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_WAY && !self.myCurrentEdge.is_null() {
            let ce = self.myCurrentEdge;
            if ce.myCurrentIsRoad {
                let id = ce.id;
                if !self.myEdgeMap.m().contains_key(&id) {
                    // assume we are loading multiple files, so we won't report duplicate edges
                    self.myEdgeMap.m().insert(id, ce);
                } else {
                    // SAFETY: the Edge was created with Box::into_raw in my_start_element and is not shared yet.
                    drop(unsafe { Box::from_raw(ce.as_raw()) });
                }
            } else if ce.myExtraTags.contains_key("platform") {
                let id = ce.id;
                if !self.myPlatformShapesMap.m().contains_key(&id) {
                    // assume we are loading multiple files, so we won't report duplicate platforms
                    self.myPlatformShapesMap.m().insert(id, ce);
                } else {
                    // SAFETY: as above.
                    drop(unsafe { Box::from_raw(ce.as_raw()) });
                }
            } else {
                // SAFETY: as above.
                drop(unsafe { Box::from_raw(ce.as_raw()) });
            }
            self.myCurrentEdge = Ptr::null();
        }
    }

    /// C++: `double EdgesHandler::interpretSpeed(const std::string &, std::string)` [protected]
    // double
    // NIImporter_OpenStreetMap::EdgesHandler::interpretSpeed(const std::string& key, std::string value) {
    //     if (mySpeedMap.find(value) != mySpeedMap.end()) {
    //         return mySpeedMap[value];
    //     } else {
    //         // handle symbolic names of the form DE:30 / DE:zone30
    //         if (value.size() > 3 && value[2] == ':') {
    //             if (value.substr(3, 4) == "zone") {
    //                 value = value.substr(7);
    //             } else {
    //                 value = value.substr(3);
    //             }
    //         }
    //         try {
    //             return StringUtils::parseSpeed(value);
    //         } catch (...) {
    //             WRITE_WARNING("Value of key '" + key + "' is not numeric ('" + value + "') in edge '" +
    //                           toString(myCurrentEdge->id) + "'.");
    //             return MAXSPEED_UNGIVEN;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn interpret_speed(&mut self, key: &str, value: String) -> f64 {
        if let Some(v) = self.mySpeedMap.get(&value) {
            return *v;
        }
        // handle symbolic names of the form DE:30 / DE:zone30
        let mut value = value;
        if value.len() > 3 && value.as_bytes()[2] == b':' {
            if value.as_bytes().get(3..7) == Some(b"zone".as_slice()) {
                value = String::from_utf8_lossy(&value.as_bytes()[7..]).into_owned();
            } else {
                value = String::from_utf8_lossy(&value.as_bytes()[3..]).into_owned();
            }
        }
        match utils::common::string_utils::parse_speed(&value, true) {
            Ok(s) => s,
            Err(_) => {
                utils::common::msg_handler::write_warning(&format!("Value of key '{}' is not numeric ('{}') in edge '{}'.", key, value, self.myCurrentEdge.id));
                MAXSPEED_UNGIVEN
            }
        }
    }

    /// C++: `int EdgesHandler::interpretChangeType(const std::string &) const` [const, protected]
    // int
    // NIImporter_OpenStreetMap::EdgesHandler::interpretChangeType(const std::string& value) const {
    //     int result = 0;
    //     const std::vector<std::string> values = StringTokenizer(value, "|").getVector();
    //     for (const std::string& val : values) {
    //         if (val == "no") {
    //             result += CHANGE_NO;
    //         } else if (val == "not_left") {
    //             result += CHANGE_NO_LEFT;
    //         } else if (val == "not_right") {
    //             result += CHANGE_NO_RIGHT;
    //         }
    //         result = result << 2;
    //     }
    //     // last shift was superfluous
    //     result = result >> 2;
    //
    //     if (values.size() > 1) {
    //         result += 2 << 29; // mark multi-value input
    //     }
    //     //std::cout << " way=" << myCurrentEdge->id << " value=" << value << " result=" << std::bitset<32>(result) << "\n";
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn interpret_change_type(&self, value: &str) -> i32 {
        let mut result: i32 = 0;
        let values = utils::common::string_tokenizer::StringTokenizer::with_token(value, "|", false).get_vector();
        for val in &values {
            if val == "no" {
                result += NIImporter_OpenStreetMap_ChangeType::CHANGE_NO as i32;
            } else if val == "not_left" {
                result += NIImporter_OpenStreetMap_ChangeType::CHANGE_NO_LEFT as i32;
            } else if val == "not_right" {
                result += NIImporter_OpenStreetMap_ChangeType::CHANGE_NO_RIGHT as i32;
            }
            result <<= 2;
        }
        // last shift was superfluous
        result >>= 2;

        if values.len() > 1 {
            result += 2 << 29; // mark multi-value input
        }
        result
    }

    /// C++: `void EdgesHandler::interpretLaneUse(const std::string &, SUMOVehicleClass, const bool) const` [const, protected]
    // void
    // NIImporter_OpenStreetMap::EdgesHandler::interpretLaneUse(const std::string& value, SUMOVehicleClass svc, const bool forward) const {
    //     const std::vector<std::string> values = StringTokenizer(value, "|").getVector();
    //     std::vector<bool>& designated = forward ? myCurrentEdge->myDesignatedLaneForward : myCurrentEdge->myDesignatedLaneBackward;
    //     std::vector<SVCPermissions>& allowed = forward ? myCurrentEdge->myAllowedLaneForward : myCurrentEdge->myAllowedLaneBackward;
    //     std::vector<SVCPermissions>& disallowed = forward ? myCurrentEdge->myDisallowedLaneForward : myCurrentEdge->myDisallowedLaneBackward;
    //     designated.resize(MAX2(designated.size(), values.size()), false);
    //     allowed.resize(MAX2(allowed.size(), values.size()), SVC_IGNORING);
    //     disallowed.resize(MAX2(disallowed.size(), values.size()), SVC_IGNORING);
    //     int i = 0;
    //     for (const std::string& val : values) {
    //         if (val == "yes" || val == "permissive") {
    //             allowed[i] |= svc;
    //         } else if (val == "lane" || val == "designated") {
    //             allowed[i] |= svc;
    //             designated[i] = true;
    //         } else if (val == "no") {
    //             disallowed[i] |= svc;
    //         } else {
    //             WRITE_WARNINGF(TL("Unknown lane use specifier '%' ignored for way '%'"), val, myCurrentEdge->id);
    //         }
    //         i++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn interpret_lane_use(&self, value: &str, svc: SUMOVehicleClass, forward: bool) {
        let values = utils::common::string_tokenizer::StringTokenizer::with_token(value, "|", false).get_vector();
        let e = self.myCurrentEdge.m();
        let (designated, allowed, disallowed) = if forward {
            (&mut e.myDesignatedLaneForward, &mut e.myAllowedLaneForward, &mut e.myDisallowedLaneForward)
        } else {
            (&mut e.myDesignatedLaneBackward, &mut e.myAllowedLaneBackward, &mut e.myDisallowedLaneBackward)
        };
        let n = designated.len().max(values.len());
        designated.resize(n, false);
        let n = allowed.len().max(values.len());
        allowed.resize(n, SVC_IGNORING);
        let n = disallowed.len().max(values.len());
        disallowed.resize(n, SVC_IGNORING);
        for (i, val) in values.iter().enumerate() {
            if val == "yes" || val == "permissive" {
                allowed[i] |= svc;
            } else if val == "lane" || val == "designated" {
                allowed[i] |= svc;
                designated[i] = true;
            } else if val == "no" {
                disallowed[i] |= svc;
            } else {
                write_warningf!("Unknown lane use specifier '%' ignored for way '%'", val, e.id);
            }
        }
    }

    /// C++: `bool EdgesHandler::interpretPlacement(const std::string &, NIImporter_OpenStreetMap::PlacementType &, int &) const` [const, protected]
    // bool
    // NIImporter_OpenStreetMap::EdgesHandler::interpretPlacement(const std::string& value, NIImporter_OpenStreetMap::PlacementType& placement, int& laneIndex) const {
    //     placement = NIImporter_OpenStreetMap::PlacementType::NONE;
    //     laneIndex = -1;
    //     const std::vector<std::string> tokens = StringTokenizer(value, ":").getVector();
    //     if (tokens.size() != 2) {
    //         return false;
    //     }
    //     const std::string where = StringUtils::prune(tokens[0]);
    //     if (where == "left_of") {
    //         placement = NIImporter_OpenStreetMap::PlacementType::LEFT_OF;
    //     } else if (where == "right_of") {
    //         placement = NIImporter_OpenStreetMap::PlacementType::RIGHT_OF;
    //     } else if (where == "middle_of") {
    //         placement = NIImporter_OpenStreetMap::PlacementType::MIDDLE_OF;
    //     } else {
    //         return false;
    //     }
    //     try {
    //         laneIndex = StringUtils::toInt(StringUtils::prune(tokens[1]));
    //     } catch (ProcessError&) {
    //         placement = NIImporter_OpenStreetMap::PlacementType::NONE;
    //         laneIndex = -1;
    //         return false;
    //     }
    //     if (laneIndex <= 0) {
    //         placement = NIImporter_OpenStreetMap::PlacementType::NONE;
    //         laneIndex = -1;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn interpret_placement(&self, value: &str, placement: &mut NIImporter_OpenStreetMap_PlacementType, laneIndex: &mut i32) -> bool {
        *placement = NIImporter_OpenStreetMap_PlacementType::NONE;
        *laneIndex = -1;
        let tokens = utils::common::string_tokenizer::StringTokenizer::with_token(value, ":", false).get_vector();
        if tokens.len() != 2 {
            return false;
        }
        let where_ = utils::common::string_utils::prune(&tokens[0]);
        if where_ == "left_of" {
            *placement = NIImporter_OpenStreetMap_PlacementType::LEFT_OF;
        } else if where_ == "right_of" {
            *placement = NIImporter_OpenStreetMap_PlacementType::RIGHT_OF;
        } else if where_ == "middle_of" {
            *placement = NIImporter_OpenStreetMap_PlacementType::MIDDLE_OF;
        } else {
            return false;
        }
        match utils::common::string_utils::to_int(&utils::common::string_utils::prune(&tokens[1])) {
            Ok(idx) => *laneIndex = idx,
            Err(_) => {
                *placement = NIImporter_OpenStreetMap_PlacementType::NONE;
                *laneIndex = -1;
                return false;
            }
        }
        if *laneIndex <= 0 {
            *placement = NIImporter_OpenStreetMap_PlacementType::NONE;
            *laneIndex = -1;
            return false;
        }
        true
    }

    /// C++: `void EdgesHandler::addType(const std::string &)` [protected]
    // void
    // NIImporter_OpenStreetMap::EdgesHandler::addType(const std::string& singleTypeID) {
    //     // special case: never build compound type for highspeed rail
    //     if (!myCurrentEdge->myHighWayType.empty() && singleTypeID != "railway.highspeed") {
    //         if (myCurrentEdge->myHighWayType == "railway.highspeed") {
    //             return;
    //         }
    //         // osm-ways may be used by more than one mode (eg railway.tram + highway.residential. this is relevant for multimodal traffic)
    //         // we create a new type for this kind of situation which must then be resolved in insertEdge()
    //         std::vector<std::string> types = StringTokenizer(myCurrentEdge->myHighWayType,
    //                                          compoundTypeSeparator).getVector();
    //         types.push_back(singleTypeID);
    //         myCurrentEdge->myHighWayType = joinToStringSorting(types, compoundTypeSeparator);
    //     } else {
    //         myCurrentEdge->myHighWayType = singleTypeID;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_type(&mut self, singleTypeID: &str) {
        let e = self.myCurrentEdge.m();
        // special case: never build compound type for highspeed rail
        if !e.myHighWayType.is_empty() && singleTypeID != "railway.highspeed" {
            if e.myHighWayType == "railway.highspeed" {
                return;
            }
            // osm-ways may be used by more than one mode (eg railway.tram + highway.residential. this is relevant for multimodal traffic)
            // we create a new type for this kind of situation which must then be resolved in insertEdge()
            let mut types = utils::common::string_tokenizer::StringTokenizer::with_token(&e.myHighWayType, compoundTypeSeparator, false).get_vector();
            types.push(singleTypeID.to_string());
            e.myHighWayType = utils::common::to_string::join_to_string_sorting(&types, compoundTypeSeparator);
        } else {
            e.myHighWayType = singleTypeID.to_string();
        }
    }

    /// C++: `void EdgesHandler::EdgesHandler(const EdgesHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_OpenStreetMap_EdgesHandler) -> Self {
        // C++ declares the copy constructor private and never defines it, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::EdgesHandler copy constructor is private and undefined in C++")
    }

    /// C++: `EdgesHandler & EdgesHandler::operator=(const EdgesHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_OpenStreetMap_EdgesHandler) -> &mut NIImporter_OpenStreetMap_EdgesHandler {
        // C++ declares the assignment operator private and never defines it, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::EdgesHandler::operator= is private and undefined in C++")
    }

}

/// C++ class `RelationHandler::NIIPTPlatform` (netimport/NIImporter_OpenStreetMap.h)
#[derive(Clone, Copy)]
pub struct RelationHandler_NIIPTPlatform {
    pub ref_: i64,
    pub isWay: bool,
}

impl RelationHandler_NIIPTPlatform {
}

/// Importer for networks stored in OpenStreetMap format
/// C++ class `NIImporter_OpenStreetMap` (netimport/NIImporter_OpenStreetMap.h)
pub struct NIImporter_OpenStreetMap {
    /// the map from OSM node ids to actual nodes
    pub myOSMNodes: BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>,
    /// the set of unique nodes used in NodesHandler, used when freeing memory
    pub myUniqueNodes: BTreeSet<NIImporter_OpenStreetMap_NodeByCoord>, // C++: std::set<NIOSMNode *, CompareNodes>
    /// the map from OSM way ids to edge objects
    pub myEdges: BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>,
    /// the map from OSM way ids to platform shapes
    pub myPlatformShapes: BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>,
    /// The compounds types that do not contain known types
    pub myUnusableTypes: BTreeSet<String>,
    /// The compound types that have already been mapped to other known types
    pub myKnownCompoundTypes: BTreeMap<String, String>,
    /// import lane specific access restrictions
    pub myImportLaneAccess: bool,
    /// import sidewalks
    pub myImportSidewalks: bool,
    /// import sidewalks
    pub myOnewayDualSidewalk: bool,
    /// import bike path specific permissions and directions
    pub myImportBikeAccess: bool,
    /// import crossings
    pub myImportCrossings: bool,
    /// import turning signals (turn:lanes) to guide connection building
    pub myImportTurnSigns: bool,
    /// whether edges should carry information on the use of typemap defaults
    pub myAnnotateDefaults: bool,
    /// number of placement skips due to non-explicit oneway
    pub myPlacementSkippedNonExplicitOneWay: i32,
    /// number of placement skips due to generated opposite-direction auxiliary edges
    pub myPlacementSkippedAuxOppositeDirection: i32,
}

// C++ static members of NIImporter_OpenStreetMap (port as module-level statics, see AGENTS.md):
//   static const std::string NIImporter_OpenStreetMap::compoundTypeSeparator;
//   static bool NIImporter_OpenStreetMap::myAllAttributes;
//   static std::set<std::string> NIImporter_OpenStreetMap::myExtraAttributes;
//   static const double NIImporter_OpenStreetMap::MAXSPEED_UNGIVEN;
//   static const long long NIImporter_OpenStreetMap::INVALID_ID;

impl NIImporter_OpenStreetMap {
    /// C++: `void NIImporter_OpenStreetMap::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads content of the optionally given OSM file
    // void
    // NIImporter_OpenStreetMap::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     NIImporter_OpenStreetMap importer;
    //     importer.load(oc, nb);
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        let mut importer = NIImporter_OpenStreetMap::new();
        importer.load(oc, nb);
    }

    /// C++: `SUMOVehicleClass NIImporter_OpenStreetMap::interpretTransportType(const std::string &, NIOSMNode *)` [static]
    /// translate osm transport designations into sumo vehicle class
    // SUMOVehicleClass
    // NIImporter_OpenStreetMap::interpretTransportType(const std::string& type, NIOSMNode* toSet) {
    //     SUMOVehicleClass result = SVC_IGNORING;
    //     if (type == "train") {
    //         result = SVC_RAIL;
    //     } else if (type == "subway") {
    //         result = SVC_SUBWAY;
    //     } else if (type == "aerialway") {
    //         result = SVC_CABLE_CAR;
    //     } else if (type == "light_rail" || type == "monorail") {
    //         result = SVC_RAIL_URBAN;
    //     } else if (type == "share_taxi") {
    //         result = SVC_TAXI;
    //     } else if (type == "minibus") {
    //         result = SVC_BUS;
    //     } else if (type == "trolleybus") {
    //         result = SVC_BUS;
    //     } else if (SumoVehicleClassStrings.hasString(type)) {
    //         result = SumoVehicleClassStrings.get(type);
    //     }
    //     std::string stop = "";
    //     if (result == SVC_TRAM) {
    //         stop = ".tram";
    //     } else if (result == SVC_BUS) {
    //         stop = ".bus";
    //     } else if (isRailway(result)) {
    //         stop = ".train";
    //     }
    //     if (toSet != nullptr && result != SVC_IGNORING) {
    //         toSet->permissions |= result;
    //         toSet->ptStopLength = OptionsCont::getOptions().getFloat("osm.stop-output.length" + stop);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn interpret_transport_type(type_: &str, toSet: Ptr<NIImporter_OpenStreetMap_NIOSMNode>) -> SUMOVehicleClass {
        use utils::common::sumo_vehicle_class::*;
        let mut result: SUMOVehicleClass = SVC_IGNORING;
        if type_ == "train" {
            result = SVC_RAIL;
        } else if type_ == "subway" {
            result = SVC_SUBWAY;
        } else if type_ == "aerialway" {
            result = SVC_CABLE_CAR;
        } else if type_ == "light_rail" || type_ == "monorail" {
            result = SVC_RAIL_URBAN;
        } else if type_ == "share_taxi" {
            result = SVC_TAXI;
        } else if type_ == "minibus" {
            result = SVC_BUS;
        } else if type_ == "trolleybus" {
            result = SVC_BUS;
        } else if let Ok(v) = SUMO_VEHICLE_CLASS_STRINGS.get(type_) {
            result = v;
        }
        let mut stop = String::new();
        if result == SVC_TRAM {
            stop = ".tram".to_string();
        } else if result == SVC_BUS {
            stop = ".bus".to_string();
        } else if is_railway(result) {
            stop = ".train".to_string();
        }
        if !toSet.is_null() && result != SVC_IGNORING {
            toSet.m().permissions |= result;
            toSet.m().ptStopLength = utils::options::options_cont::get_float(&format!("osm.stop-output.length{stop}"));
        }
        result
    }

    /// C++: `void NIImporter_OpenStreetMap::NIImporter_OpenStreetMap()` [protected]
    // NIImporter_OpenStreetMap::NIImporter_OpenStreetMap()
    #[allow(unused_variables)]
    pub fn new() -> Self {
        // C++ `= default`; the bool options are read in load() before use, so false is a placeholder.
        NIImporter_OpenStreetMap {
            myOSMNodes: BTreeMap::new(),
            myUniqueNodes: BTreeSet::new(),
            myEdges: BTreeMap::new(),
            myPlatformShapes: BTreeMap::new(),
            myUnusableTypes: BTreeSet::new(),
            myKnownCompoundTypes: BTreeMap::new(),
            myImportLaneAccess: false,
            myImportSidewalks: false,
            myOnewayDualSidewalk: false,
            myImportBikeAccess: false,
            myImportCrossings: false,
            myImportTurnSigns: false,
            myAnnotateDefaults: false,
            myPlacementSkippedNonExplicitOneWay: 0,
            myPlacementSkippedAuxOppositeDirection: 0,
        }
    }

    /// C++: `void NIImporter_OpenStreetMap::~NIImporter_OpenStreetMap() noexcept` [protected]
    // NIImporter_OpenStreetMap::~NIImporter_OpenStreetMap() {
    //     // delete nodes
    //     for (auto myUniqueNode : myUniqueNodes) {
    //         delete myUniqueNode;
    //     }
    //     // delete edges
    //     for (auto& myEdge : myEdges) {
    //         delete myEdge.second;
    //     }
    //     // delete platform shapes
    //     for (auto& myPlatformShape : myPlatformShapes) {
    //         delete myPlatformShape.second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // delete nodes
        for myUniqueNode in self.myUniqueNodes.iter() {
            // SAFETY: nodes were allocated with Box::into_raw and are only freed here.
            drop(unsafe { Box::from_raw(myUniqueNode.0.as_raw()) });
        }
        // delete edges
        for myEdge in self.myEdges.values() {
            // SAFETY: edges were allocated with Box::into_raw and are only freed here.
            drop(unsafe { Box::from_raw(myEdge.as_raw()) });
        }
        // delete platform shapes
        for myPlatformShape in self.myPlatformShapes.values() {
            // SAFETY: as above.
            drop(unsafe { Box::from_raw(myPlatformShape.as_raw()) });
        }
    }

    /// C++: `void NIImporter_OpenStreetMap::load(const OptionsCont &, NBNetBuilder &)` [protected]
    // void
    // NIImporter_OpenStreetMap::load(const OptionsCont& oc, NBNetBuilder& nb) {
    //     if (!oc.isSet("osm-files")) {
    //         return;
    //     }
    //     const std::vector<std::string> files = oc.getStringVector("osm-files");
    //     std::vector<SUMOSAXReader*> readers;
    //
    //     myImportLaneAccess = oc.getBool("osm.lane-access");
    //     myImportTurnSigns = oc.getBool("osm.turn-lanes");
    //     myImportSidewalks = oc.getBool("osm.sidewalks");
    //     myImportBikeAccess = oc.getBool("osm.bike-access");
    //     myImportCrossings = oc.getBool("osm.crossings");
    //     myOnewayDualSidewalk = oc.getBool("osm.oneway-reverse-sidewalk");
    //     myAnnotateDefaults = oc.getBool("osm.annotate-defaults");
    //     myETCS_movingBlock = oc.getBool("osm.etcs-moving-block");
    //     myPlacementSkippedNonExplicitOneWay = 0;
    //     myPlacementSkippedAuxOppositeDirection = 0;
    //
    //     myAllAttributes = OptionsCont::getOptions().getBool("osm.all-attributes");
    //     std::vector<std::string> extra = OptionsCont::getOptions().getStringVector("osm.extra-attributes");
    //     myExtraAttributes.insert(extra.begin(), extra.end());
    //     if (myExtraAttributes.count("all") != 0) {
    //         // import all
    //         myExtraAttributes.clear();
    //     }
    //
    //     // load nodes, first
    //     NodesHandler nodesHandler(myOSMNodes, myUniqueNodes, oc);
    //     for (const std::string& file : files) {
    //         if (!FileHelpers::isReadable(file)) {
    //             WRITE_ERRORF(TL("Could not open osm-file '%'."), file);
    //             return;
    //         }
    //         nodesHandler.setFileName(file);
    //         nodesHandler.resetHierarchy();
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing nodes from osm-file '" + file + "'");
    //         readers.push_back(XMLSubSys::getSAXReader(nodesHandler));
    //         if (!readers.back()->parseFirst(file) || !readers.back()->parseSection(SUMO_TAG_NODE) ||
    //                 MsgHandler::getErrorInstance()->wasInformed()) {
    //             return;
    //         }
    //         if (nodesHandler.getDuplicateNodes() > 0) {
    //             WRITE_MESSAGEF(TL("Found and substituted % osm nodes."), toString(nodesHandler.getDuplicateNodes()));
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //
    //     // load edges, then
    //     EdgesHandler edgesHandler(myOSMNodes, myEdges, myPlatformShapes, nb.getTypeCont());
    //     int idx = 0;
    //     for (const std::string& file : files) {
    //         edgesHandler.setFileName(file);
    //         readers[idx]->setHandler(edgesHandler);
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing edges from osm-file '" + file + "'");
    //         if (!readers[idx]->parseSection(SUMO_TAG_WAY)) {
    //             // eof already reached, no relations
    //             delete readers[idx];
    //             readers[idx] = nullptr;
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //         idx++;
    //     }
    //
    //     /* Remove duplicate edges with the same shape and attributes */
    //     if (!oc.getBool("osm.skip-duplicates-check")) {
    //         int numRemoved = 0;
    //         PROGRESS_BEGIN_MESSAGE(TL("Removing duplicate edges"));
    //         if (myEdges.size() > 1) {
    //             std::set<const Edge*, CompareEdges> dupsFinder;
    //             for (auto it = myEdges.begin(); it != myEdges.end();) {
    //                 if (dupsFinder.count(it->second) > 0) {
    //                     numRemoved++;
    //                     delete it->second;
    //                     myEdges.erase(it++);
    //                 } else {
    //                     dupsFinder.insert(it->second);
    //                     it++;
    //                 }
    //             }
    //         }
    //         if (numRemoved > 0) {
    //             WRITE_MESSAGEF(TL("Removed % duplicate osm edges."), toString(numRemoved));
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //
    //     /* Mark which nodes are used (by edges or traffic lights).
    //      * This is necessary to detect which OpenStreetMap nodes are for
    //      * geometry only */
    //     std::map<long long int, int> nodeUsage;
    //     // Mark which nodes are used by edges (begin and end)
    //     for (const auto& edgeIt : myEdges) {
    //         assert(edgeIt.second->myCurrentIsRoad);
    //         for (const long long int node : edgeIt.second->myCurrentNodes) {
    //             nodeUsage[node]++;
    //         }
    //     }
    //     // Mark which nodes are used by traffic lights or are pedestrian crossings
    //     for (const auto& nodesIt : myOSMNodes) {
    //         if (nodesIt.second->tlsControlled || nodesIt.second->railwaySignal != SIGNAL_NONE || (nodesIt.second->pedestrianCrossing && myImportCrossings) /* || nodesIt->second->railwayCrossing*/) {
    //             // If the key is not found in the map, the value is automatically
    //             // initialized with 0.
    //             nodeUsage[nodesIt.first]++;
    //         }
    //     }
    //
    //     /* Instantiate edges
    //      * Only those nodes in the middle of an edge which are used by more than
    //      * one edge are instantiated. Other nodes are considered as geometry nodes. */
    //     NBNodeCont& nc = nb.getNodeCont();
    //     NBTrafficLightLogicCont& tlsc = nb.getTLLogicCont();
    //     for (const auto& edgeIt : myEdges) {
    //         Edge* const e = edgeIt.second;
    //         if (!e->myCurrentIsRoad) {
    //             continue;
    //         }
    //         if (e->myCurrentNodes.size() < 2) {
    //             WRITE_WARNINGF(TL("Discarding way '%' because it has only % node(s)"), e->id, e->myCurrentNodes.size());
    //             continue;
    //         }
    //         extendRailwayDistances(e, nb.getTypeCont());
    //         // build nodes;
    //         //  - the from- and to-nodes must be built in any case
    //         //  - the in-between nodes are only built if more than one edge references them
    //         NBNode* first = insertNodeChecking(e->myCurrentNodes.front(), nc, tlsc);
    //         NBNode* last = insertNodeChecking(e->myCurrentNodes.back(), nc, tlsc);
    //         NBNode* currentFrom = first;
    //         int running = 0;
    //         std::vector<long long int> passed;
    //         for (auto j = e->myCurrentNodes.begin(); j != e->myCurrentNodes.end(); ++j) {
    //             passed.push_back(*j);
    //             if (nodeUsage[*j] > 1 && j != e->myCurrentNodes.end() - 1 && j != e->myCurrentNodes.begin()) {
    //                 NBNode* currentTo = insertNodeChecking(*j, nc, tlsc);
    //                 running = insertEdge(e, running, currentFrom, currentTo, passed, nb, first, last);
    //                 currentFrom = currentTo;
    //                 passed.clear();
    //                 passed.push_back(*j);
    //             }
    //         }
    //         if (running == 0) {
    //             running = -1;
    //         }
    //         insertEdge(e, running, currentFrom, last, passed, nb, first, last);
    //     }
    //     if (myPlacementSkippedNonExplicitOneWay > 0 || myPlacementSkippedAuxOppositeDirection > 0) {
    //         WRITE_MESSAGEF(TL("Skipped applying OSM placement on % edge(s): % due to non-explicit one-way and % due to opposite-direction auxiliary edges."),
    //                        myPlacementSkippedNonExplicitOneWay + myPlacementSkippedAuxOppositeDirection,
    //                        myPlacementSkippedNonExplicitOneWay,
    //                        myPlacementSkippedAuxOppositeDirection);
    //     }
    //
    //     /* Collect edges which explicitly are part of a roundabout and store the edges of each
    //      * detected roundabout */
    //     nb.getEdgeCont().extractRoundabouts();
    //
    //     if (myImportCrossings) {
    //         /* After edges are instantiated
    //          * nodes are parsed again to add pedestrian crossings to them
    //          * This is only executed if crossings are imported and not guessed */
    //         const double crossingWidth = OptionsCont::getOptions().getFloat("default.crossing-width");
    //
    //         for (auto item : nodeUsage) {
    //             NIOSMNode* osmNode = myOSMNodes.find(item.first)->second;
    //             if (osmNode->pedestrianCrossing) {
    //                 NBNode* n = osmNode->node;
    //                 EdgeVector incomingEdges = n->getIncomingEdges();
    //                 EdgeVector outgoingEdges = n->getOutgoingEdges();
    //                 size_t incomingEdgesNo = incomingEdges.size();
    //                 size_t outgoingEdgesNo = outgoingEdges.size();
    //
    //                 for (size_t i = 0; i < incomingEdgesNo; i++) {
    //                     /* Check if incoming edge has driving lanes(and sidewalks)
    //                      * if not, ignore
    //                      * if yes, check if there is a corresponding outgoing edge for the opposite direction
    //                      *   -> if yes, check if it has driving lanes
    //                      *          --> if yes, do the crossing
    //                      *          --> if no, only do the crossing with the incoming edge (usually one lane roads with two sidewalks)
    //                      *   -> if not, do nothing as we don't have a sidewalk in the opposite direction */
    //                     auto const iEdge = incomingEdges[i];
    //
    //                     if (iEdge->getFirstNonPedestrianLaneIndex(NBNode::FORWARD) > -1
    //                             && iEdge->getSpecialLane(SVC_PEDESTRIAN) > -1) {
    //                         std::string const& iEdgeId = iEdge->getID();
    //                         std::size_t const m = iEdgeId.find_first_of("#");
    //                         std::string const& iWayId = iEdgeId.substr(0, m);
    //                         for (size_t j = 0; j < outgoingEdgesNo; j++) {
    //                             auto const oEdge = outgoingEdges[j];
    //                             // Searching for a corresponding outgoing edge (based on OSM way identifier)
    //                             // with at least a pedestrian lane, going in the opposite direction
    //                             if (oEdge->getID().find(iWayId) != std::string::npos
    //                                     && oEdge->getSpecialLane(SVC_PEDESTRIAN) > -1
    //                                     && oEdge->getID().rfind(iWayId, 0) != 0) {
    //                                 EdgeVector edgeVector = EdgeVector{ iEdge };
    //                                 if (oEdge->getFirstNonPedestrianLaneIndex(NBNode::FORWARD) > -1) {
    //                                     edgeVector.push_back(oEdge);
    //                                 }
    //
    //                                 if (!n->checkCrossingDuplicated(edgeVector)) {
    //                                     n->addCrossing(edgeVector, crossingWidth, false);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 for (size_t i = 0; i < outgoingEdgesNo; i++) {
    //                     // Same checks as above for loop, but for outgoing edges
    //                     auto const oEdge = outgoingEdges[i];
    //
    //                     if (oEdge->getFirstNonPedestrianLaneIndex(NBNode::FORWARD) > -1
    //                             && oEdge->getSpecialLane(SVC_PEDESTRIAN) > -1) {
    //                         std::string const& oEdgeId = oEdge->getID();
    //                         std::size_t const m = oEdgeId.find_first_of("#");
    //                         std::string const& iWayId = oEdgeId.substr(0, m);
    //                         for (size_t j = 0; j < incomingEdgesNo; j++) {
    //                             auto const iEdge = incomingEdges[j];
    //                             if (iEdge->getID().find(iWayId) != std::string::npos
    //                                     && iEdge->getSpecialLane(SVC_PEDESTRIAN) > -1
    //                                     && iEdge->getID().rfind(iWayId, 0) != 0) {
    //                                 EdgeVector edgeVector = EdgeVector{ oEdge };
    //                                 if (iEdge->getFirstNonPedestrianLaneIndex(NBNode::FORWARD) > -1) {
    //                                     edgeVector.push_back(iEdge);
    //                                 }
    //
    //                                 if (!n->checkCrossingDuplicated(edgeVector)) {
    //                                     n->addCrossing(edgeVector, crossingWidth, false);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     const double layerElevation = oc.getFloat("osm.layer-elevation");
    //     if (layerElevation > 0) {
    //         reconstructLayerElevation(layerElevation, nb);
    //     }
    //
    //     // revise pt stops; remove stops on deleted edges (but assign them to the
    //     // opposite direction if possible
    //     nb.getPTStopCont().cleanupDeleted(nb.getEdgeCont(), true);
    //
    //     // load relations (after edges are built since we want to apply
    //     // turn-restrictions directly to NBEdges)
    //     RelationHandler relationHandler(myOSMNodes, myEdges, &(nb.getPTStopCont()), myPlatformShapes,
    //                                     &nb.getPTLineCont(), oc);
    //     idx = 0;
    //     for (const std::string& file : files) {
    //         if (readers[idx] != nullptr) {
    //             relationHandler.setFileName(file);
    //             readers[idx]->setHandler(relationHandler);
    //             const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing relations from osm-file '" + file + "'");
    //             readers[idx]->parseSection(SUMO_TAG_RELATION);
    //             PROGRESS_TIME_MESSAGE(before);
    //             delete readers[idx];
    //         }
    //         idx++;
    //     }
    //
    //     // declare additional stops that are not anchored to a (road)-way or route relation
    //     std::set<std::string> stopNames;
    //     for (const auto& item : nb.getPTStopCont().getStops()) {
    //         stopNames.insert(item.second->getName());
    //     }
    //     for (const auto& item : myOSMNodes) {
    //         const NIOSMNode* n = item.second;
    //         if (n->ptStopPosition && stopNames.count(n->name) == 0) {
    //             Position ptPos(n->lon, n->lat, n->ele);
    //             if (!NBNetBuilder::transformCoordinate(ptPos)) {
    //                 WRITE_ERRORF("Unable to project coordinates for node '%'.", n->id);
    //             }
    //             SumoXMLTag element = isRailway(n->permissions) ? SUMO_TAG_TRAIN_STOP : SUMO_TAG_BUS_STOP;
    //             std::shared_ptr<NBPTStop> ptStop = std::make_shared<NBPTStop>(element, toString(n->id), ptPos, "", "", n->ptStopLength, n->name, n->permissions);
    //             nb.getPTStopCont().insert(ptStop, true);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load(&mut self, oc: &OptionsCont, nb: &mut NBNetBuilder) {
        use utils::common::msg_handler::*;
        use utils::options::options_cont as opt;
        if !utils::options::options_cont::is_set("osm-files") {
            return;
        }
        let files = utils::options::options_cont::get_string_vector("osm-files");
        let mut readers: Vec<Option<SUMOSAXReader>> = Vec::new();

        self.myImportLaneAccess = oc.get_bool("osm.lane-access").unwrap();
        self.myImportTurnSigns = oc.get_bool("osm.turn-lanes").unwrap();
        self.myImportSidewalks = oc.get_bool("osm.sidewalks").unwrap();
        self.myImportBikeAccess = oc.get_bool("osm.bike-access").unwrap();
        self.myImportCrossings = oc.get_bool("osm.crossings").unwrap();
        self.myOnewayDualSidewalk = oc.get_bool("osm.oneway-reverse-sidewalk").unwrap();
        self.myAnnotateDefaults = oc.get_bool("osm.annotate-defaults").unwrap();
        self.myPlacementSkippedNonExplicitOneWay = 0;
        self.myPlacementSkippedAuxOppositeDirection = 0;

        myAllAttributes.with(|c| c.set(utils::options::options_cont::get_bool("osm.all-attributes")));
        let extra = utils::options::options_cont::get_string_vector("osm.extra-attributes");
        myExtraAttributes.with(|s| {
            s.borrow_mut().extend(extra.iter().cloned());
            if s.borrow().contains("all") {
                // import all
                s.borrow_mut().clear();
            }
        });

        // load nodes, first
        let mut nodesHandler = NIImporter_OpenStreetMap_NodesHandler::new(&mut self.myOSMNodes, &mut self.myUniqueNodes, oc);
        for file in &files {
            if !utils::common::file_helpers::is_readable(file) {
                write_errorf!("Could not open osm-file '%'.", file);
                return;
            }
            nodesHandler.set_file_name(file);
            nodesHandler.reset_hierarchy();
            let before = progress_begin_time_message(&format!("Parsing nodes from osm-file '{file}'"));
            readers.push(Some(utils::xml::xml_sub_sys::get_sax_reader(false, false)));
            let reader = readers.last_mut().unwrap().as_mut().unwrap();
            if !sax_result(reader.parse_first(&mut nodesHandler, file))
                || !sax_result(reader.parse_section(&mut nodesHandler, SUMO_TAG_NODE))
                || with_instance(MsgType::Error, |h| h.was_informed())
            {
                return;
            }
            if nodesHandler.get_duplicate_nodes() > 0 {
                write_messagef!("Found and substituted % osm nodes.", nodesHandler.get_duplicate_nodes());
            }
            progress_time_since(before);
        }

        // load edges, then
        let mut edgesHandler = NIImporter_OpenStreetMap_EdgesHandler::new(&self.myOSMNodes, &mut self.myEdges, &mut self.myPlatformShapes, nb.get_type_cont());
        for (idx, file) in files.iter().enumerate() {
            edgesHandler.set_file_name(file);
            let before = progress_begin_time_message(&format!("Parsing edges from osm-file '{file}'"));
            let reader = readers[idx].as_mut().unwrap();
            if !sax_result(reader.parse_section(&mut edgesHandler, SUMO_TAG_WAY)) {
                // eof already reached, no relations
                readers[idx] = None;
            }
            progress_time_since(before);
        }

        /* Remove duplicate edges with the same shape and attributes */
        if !oc.get_bool("osm.skip-duplicates-check").unwrap() {
            let mut numRemoved = 0;
            progress_begin_message("Removing duplicate edges");
            if self.myEdges.len() > 1 {
                // std::set<const Edge*, CompareEdges> dupsFinder; kept sorted by the comparator
                let cmp = NIImporter_OpenStreetMap_CompareEdges {};
                let mut dupsFinder: Vec<Ptr<NIImporter_OpenStreetMap_Edge>> = Vec::new();
                let keys: Vec<i64> = self.myEdges.keys().cloned().collect();
                for key in keys {
                    let e = self.myEdges[&key];
                    let found = dupsFinder.binary_search_by(|x| {
                        if cmp.call_op(*x, e) {
                            std::cmp::Ordering::Less
                        } else if cmp.call_op(e, *x) {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    });
                    match found {
                        Ok(_) => {
                            numRemoved += 1;
                            self.myEdges.remove(&key);
                            // SAFETY: the edge was allocated with Box::into_raw and is no longer referenced by the map.
                            drop(unsafe { Box::from_raw(e.as_raw()) });
                        }
                        Err(pos) => dupsFinder.insert(pos, e),
                    }
                }
            }
            if numRemoved > 0 {
                write_messagef!("Removed % duplicate osm edges.", numRemoved);
            }
            progress_done_message();
        }

        /* Mark which nodes are used (by edges or traffic lights).
         * This is necessary to detect which OpenStreetMap nodes are for
         * geometry only */
        let mut nodeUsage: BTreeMap<i64, i32> = BTreeMap::new();
        // Mark which nodes are used by edges (begin and end)
        for edgeIt in self.myEdges.values() {
            assert!(edgeIt.myCurrentIsRoad);
            for node in &edgeIt.myCurrentNodes {
                *nodeUsage.entry(*node).or_insert(0) += 1;
            }
        }
        // Mark which nodes are used by traffic lights or are pedestrian crossings
        for (id, nodesIt) in &self.myOSMNodes {
            if nodesIt.tlsControlled || nodesIt.railwaySignal != NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 || (nodesIt.pedestrianCrossing && self.myImportCrossings) {
                // If the key is not found in the map, the value is automatically initialized with 0.
                *nodeUsage.entry(*id).or_insert(0) += 1;
            }
        }

        /* Instantiate edges
         * Only those nodes in the middle of an edge which are used by more than
         * one edge are instantiated. Other nodes are considered as geometry nodes. */
        // The node and traffic light containers are reached through raw pointers because
        // nb is also passed to the helpers below (C++ holds both references at once).
        let nc: Ptr<NBNodeCont> = Ptr::from_mut(nb.get_node_cont());
        let tlsc: Ptr<NBTrafficLightLogicCont> = Ptr::from_mut(nb.get_tl_logic_cont());
        let edges: Vec<Ptr<NIImporter_OpenStreetMap_Edge>> = self.myEdges.values().cloned().collect();
        for e in edges {
            if !e.myCurrentIsRoad {
                continue;
            }
            if e.myCurrentNodes.len() < 2 {
                write_warningf!("Discarding way '%' because it has only % node(s)", e.id, e.myCurrentNodes.len());
                continue;
            }
            self.extend_railway_distances(e, nb.get_type_cont());
            // build nodes;
            //  - the from- and to-nodes must be built in any case
            //  - the in-between nodes are only built if more than one edge references them
            let first = self.insert_node_checking(e.myCurrentNodes[0], nc.m(), tlsc.m());
            let last = self.insert_node_checking(*e.myCurrentNodes.last().unwrap(), nc.m(), tlsc.m());
            let mut currentFrom = first;
            let mut running: i32 = 0;
            let mut passed: Vec<i64> = Vec::new();
            let n = e.myCurrentNodes.len();
            for j in 0..n {
                let node = e.myCurrentNodes[j];
                passed.push(node);
                if *nodeUsage.get(&node).unwrap_or(&0) > 1 && j != n - 1 && j != 0 {
                    let currentTo = self.insert_node_checking(node, nc.m(), tlsc.m());
                    running = self.insert_edge(e, running, currentFrom, currentTo, &passed, nb, first, last);
                    currentFrom = currentTo;
                    passed.clear();
                    passed.push(node);
                }
            }
            if running == 0 {
                running = -1;
            }
            self.insert_edge(e, running, currentFrom, last, &passed, nb, first, last);
        }
        if self.myPlacementSkippedNonExplicitOneWay > 0 || self.myPlacementSkippedAuxOppositeDirection > 0 {
            write_messagef!(
                "Skipped applying OSM placement on % edge(s): % due to non-explicit one-way and % due to opposite-direction auxiliary edges.",
                self.myPlacementSkippedNonExplicitOneWay + self.myPlacementSkippedAuxOppositeDirection,
                self.myPlacementSkippedNonExplicitOneWay,
                self.myPlacementSkippedAuxOppositeDirection
            );
        }

        /* Collect edges which explicitly are part of a roundabout and store the edges of each
         * detected roundabout */
        nb.get_edge_cont().extract_roundabouts();

        if self.myImportCrossings {
            /* After edges are instantiated
             * nodes are parsed again to add pedestrian crossings to them
             * This is only executed if crossings are imported and not guessed */
            let crossingWidth = utils::options::options_cont::get_float("default.crossing-width");
            for item in nodeUsage.keys() {
                let osmNode = self.myOSMNodes[item];
                if osmNode.pedestrianCrossing {
                    let n = osmNode.node;
                    let incomingEdges: Vec<Ptr<NBEdge>> = n.get_incoming_edges().clone();
                    let outgoingEdges: Vec<Ptr<NBEdge>> = n.get_outgoing_edges().clone();
                    let incomingEdgesNo = incomingEdges.len();
                    let outgoingEdgesNo = outgoingEdges.len();

                    for i in 0..incomingEdgesNo {
                        /* Check if incoming edge has driving lanes(and sidewalks)
                         * if not, ignore
                         * if yes, check if there is a corresponding outgoing edge for the opposite direction
                         *   -> if yes, check if it has driving lanes
                         *          --> if yes, do the crossing
                         *          --> if no, only do the crossing with the incoming edge (usually one lane roads with two sidewalks)
                         *   -> if not, do nothing as we don't have a sidewalk in the opposite direction */
                        let iEdge = incomingEdges[i];

                        if iEdge.get_first_non_pedestrian_lane_index(NBNODE_FORWARD, false) > -1 && iEdge.get_special_lane(SVC_PEDESTRIAN) > -1 {
                            let iEdgeId = iEdge.get_id().to_string();
                            let m = iEdgeId.find('#').unwrap_or(iEdgeId.len());
                            let iWayId = iEdgeId[..m].to_string();
                            for j in 0..outgoingEdgesNo {
                                let oEdge = outgoingEdges[j];
                                // with at least a pedestrian lane, going in the opposite direction
                                if oEdge.get_id().contains(iWayId.as_str())
                                    && oEdge.get_special_lane(SVC_PEDESTRIAN) > -1
                                    && !oEdge.get_id().starts_with(iWayId.as_str())
                                {
                                    let mut edgeVector = vec![iEdge];
                                    if oEdge.get_first_non_pedestrian_lane_index(NBNODE_FORWARD, false) > -1 {
                                        edgeVector.push(oEdge);
                                    }

                                    if !n.m().check_crossing_duplicated(edgeVector.clone()) {
                                        n.m().add_crossing(edgeVector, crossingWidth, false, -1, -1, &PositionVector::EMPTY, false, Ptr::null());
                                    }
                                }
                            }
                        }
                    }
                    for i in 0..outgoingEdgesNo {
                        // Same checks as above for loop, but for outgoing edges
                        let oEdge = outgoingEdges[i];

                        if oEdge.get_first_non_pedestrian_lane_index(NBNODE_FORWARD, false) > -1 && oEdge.get_special_lane(SVC_PEDESTRIAN) > -1 {
                            let oEdgeId = oEdge.get_id().to_string();
                            let m = oEdgeId.find('#').unwrap_or(oEdgeId.len());
                            let iWayId = oEdgeId[..m].to_string();
                            for j in 0..incomingEdgesNo {
                                let iEdge = incomingEdges[j];
                                if iEdge.get_id().contains(iWayId.as_str())
                                    && iEdge.get_special_lane(SVC_PEDESTRIAN) > -1
                                    && !iEdge.get_id().starts_with(iWayId.as_str())
                                {
                                    let mut edgeVector = vec![oEdge];
                                    if iEdge.get_first_non_pedestrian_lane_index(NBNODE_FORWARD, false) > -1 {
                                        edgeVector.push(iEdge);
                                    }

                                    if !n.m().check_crossing_duplicated(edgeVector.clone()) {
                                        n.m().add_crossing(edgeVector, crossingWidth, false, -1, -1, &PositionVector::EMPTY, false, Ptr::null());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let layerElevation = utils::options::options_cont::get_float("osm.layer-elevation");
        if layerElevation > 0.0 {
            self.reconstruct_layer_elevation(layerElevation, nb);
        }

        // revise pt stops; remove stops on deleted edges (but assign them to the
        // opposite direction if possible
        let ec: Ptr<NBEdgeCont> = Ptr::from_mut(nb.get_edge_cont());
        nb.get_pt_stop_cont().cleanup_deleted(ec.m());

        // load relations (after edges are built since we want to apply
        // turn-restrictions directly to NBEdges)
        let ptStopCont: Ptr<NBPTStopCont> = Ptr::from_mut(nb.get_pt_stop_cont());
        let ptLineCont: Ptr<NBPTLineCont> = Ptr::from_mut(nb.get_pt_line_cont());
        let mut relationHandler = NIImporter_OpenStreetMap_RelationHandler::new(&self.myOSMNodes, &self.myEdges, ptStopCont, &self.myPlatformShapes, ptLineCont, oc);
        for (idx, file) in files.iter().enumerate() {
            if readers[idx].is_some() {
                relationHandler.set_file_name(file);
                let before = progress_begin_time_message(&format!("Parsing relations from osm-file '{file}'"));
                let reader = readers[idx].as_mut().unwrap();
                sax_result(reader.parse_section(&mut relationHandler, SUMO_TAG_RELATION));
                progress_time_since(before);
                readers[idx] = None;
            }
        }

        // declare additional stops that are not anchored to a (road)-way or route relation
        let mut stopNames: BTreeSet<String> = BTreeSet::new();
        for item in nb.get_pt_stop_cont().get_stops().values() {
            stopNames.insert(item.get_name());
        }
        for item in self.myOSMNodes.values() {
            let n = *item;
            if n.ptStopPosition && !stopNames.contains(&n.name) {
                let mut ptPos = Position::new3(n.lon, n.lat, n.ele);
                if !NBNetBuilder::transform_coordinate(&mut ptPos, true, Ptr::null()) {
                    write_errorf!("Unable to project coordinates for node '%'.", n.id);
                }
                let element = if utils::common::sumo_vehicle_class::is_railway(n.permissions) { SUMO_TAG_TRAIN_STOP } else { SUMO_TAG_BUS_STOP };
                let ptStop = Rc::new(NBPTStop::new(
                    element,
                    n.id.to_string(),
                    ptPos,
                    String::new(),
                    String::new(),
                    n.ptStopLength,
                    n.name.clone(),
                    n.permissions,
                    0.0,
                    RGBColor::default(),
                    -1.0,
                ));
                nb.get_pt_stop_cont().insert(ptStop, true);
            }
        }
    }

    /// C++: `NBNode * NIImporter_OpenStreetMap::insertNodeChecking(long long, NBNodeCont &, NBTrafficLightLogicCont &)` [private]
    /// Builds an NBNode
    // NBNode*
    // NIImporter_OpenStreetMap::insertNodeChecking(long long int id, NBNodeCont& nc, NBTrafficLightLogicCont& tlsc) {
    //     NBNode* node = nc.retrieve(toString(id));
    //     if (node == nullptr) {
    //         NIOSMNode* n = myOSMNodes.find(id)->second;
    //         Position pos(n->lon, n->lat, n->ele);
    //         if (!NBNetBuilder::transformCoordinate(pos, true)) {
    //             WRITE_ERRORF("Unable to project coordinates for junction '%'.", id);
    //             return nullptr;
    //         }
    //         node = new NBNode(toString(id), pos);
    //         if (!nc.insert(node)) {
    //             WRITE_ERRORF(TL("Could not insert junction '%'."), toString(id));
    //             delete node;
    //             return nullptr;
    //         }
    //         n->node = node;
    //         if (n->railwayCrossing) {
    //             if (n->getParameter("crossing:barrier", "no") != "no"
    //                     || n->getParameter("crossing.light") == "yes"
    //                     || n->tlsControlled) {
    //                 node->reinit(pos, SumoXMLNodeType::RAIL_CROSSING);
    //             }
    //         } else if (n->railwaySignal != SIGNAL_NONE) {
    //             node->reinit(pos, SumoXMLNodeType::RAIL_SIGNAL);
    //             if ((n->railwaySignal & SIGNAL_MAIN) == 0) {
    //                 node->setParameter("virtual", "true");
    //             }
    //             if (myETCS_movingBlock && (n->railwaySignal & SIGNAL_ETCS_BLOCK) != 0) {
    //                 node->setParameter("moving-block", "true");
    //             }
    //         } else if (n->tlsControlled) {
    //             // ok, this node is a traffic light node where no other nodes
    //             //  participate
    //             // @note: The OSM-community has not settled on a schema for differentiating between fixed and actuated lights
    //             TrafficLightType type = SUMOXMLDefinitions::TrafficLightTypes.get(
    //                                         OptionsCont::getOptions().getString("tls.default-type"));
    //             NBOwnTLDef* tlDef = new NBOwnTLDef(toString(id), node, 0, type);
    //             if (!tlsc.insert(tlDef)) {
    //                 // actually, nothing should fail here
    //                 delete tlDef;
    //                 throw ProcessError(TLF("Could not allocate tls '%'.", toString(id)));
    //             }
    //         }
    //         if (n->railwayBufferStop) {
    //             node->setParameter("buffer_stop", "true");
    //             node->setFringeType(FringeType::INNER);
    //         }
    //         if (n->railwaySignal != SIGNAL_NONE) {
    //             if (n->myRailDirection == WAY_FORWARD) {
    //                 node->setParameter(OSM_SIGNAL_DIRECTION, "forward");
    //             } else if (n->myRailDirection == WAY_BACKWARD) {
    //                 node->setParameter(OSM_SIGNAL_DIRECTION, "backward");
    //             }
    //         }
    //         node->updateParameters(n->getParametersMap());
    //     }
    //     return node;
    // }
    #[allow(unused_variables)]
    pub fn insert_node_checking(&mut self, id: i64, nc: &mut NBNodeCont, tlsc: &mut NBTrafficLightLogicCont) -> Ptr<NBNode> {
        let mut node = nc.retrieve(&id.to_string());
        if node.is_null() {
            let n = self.myOSMNodes[&id];
            let mut pos = Position::new3(n.lon, n.lat, n.ele);
            if !NBNetBuilder::transform_coordinate(&mut pos, true, Ptr::null()) {
                write_errorf!("Unable to project coordinates for junction '%'.", id);
                return Ptr::null();
            }
            node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(&id.to_string(), &pos, SumoXMLNodeType::UNKNOWN))));
            if !nc.insert_2(node) {
                write_errorf!("Could not insert junction '%'.", id);
                // SAFETY: the node was just allocated and was not inserted anywhere.
                drop(unsafe { Box::from_raw(node.as_raw()) });
                return Ptr::null();
            }
            n.m().node = node;
            if n.railwayCrossing {
                if n.get_parameter("crossing:barrier", "no") != "no" || n.get_parameter("crossing.light", "") == "yes" || n.tlsControlled {
                    node.m().reinit(&pos, SumoXMLNodeType::RAIL_CROSSING, false);
                }
            } else if n.railwaySignal != NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 {
                node.m().reinit(&pos, SumoXMLNodeType::RAIL_SIGNAL, false);
                if (n.railwaySignal & NIImporter_OpenStreetMap_SignalType::SIGNAL_MAIN as i32) == 0 {
                    node.m().base2.set_parameter("virtual", "true");
                }
            } else if n.tlsControlled {
                // ok, this node is a traffic light node where no other nodes
                //  participate
                // @note: The OSM-community has not settled on a schema for differentiating between fixed and actuated lights
                let type_ = TrafficLightTypes
                    .get(&utils::options::options_cont::get_string("tls.default-type"))
                    .unwrap_or(TrafficLightType::INVALID);
                let tlDef: &mut NBOwnTLDef = Box::leak(Box::new(NBOwnTLDef::new_2(&id.to_string(), node, 0, type_)));
                node.m().add_traffic_light(Ptr::from_mut(&mut tlDef.base));
                if !tlsc.insert(Ptr::from_mut(&mut tlDef.base), false) {
                    // actually, nothing should fail here
                    panic!("Could not allocate tls '{id}'.");
                }
            }
            if n.railwayBufferStop {
                node.m().base2.set_parameter("buffer_stop", "true");
                node.m().set_fringe_type(FringeType::INNER);
            }
            if n.railwaySignal != NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 {
                if n.myRailDirection == NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i32 {
                    node.m().base2.set_parameter(OSM_SIGNAL_DIRECTION, "forward");
                } else if n.myRailDirection == NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i32 {
                    node.m().base2.set_parameter(OSM_SIGNAL_DIRECTION, "backward");
                }
            }
            let params = n.get_parameters_map().clone();
            node.m().base2.update_parameters(&params);
        }
        node
    }

    /// C++: `int NIImporter_OpenStreetMap::insertEdge(Edge *, int, NBNode *, NBNode *, const std::vector<long long> &, NBNetBuilder &, const NBNode *, const NBNode *)` [private]
    /// Builds an NBEdge
    // int
    // NIImporter_OpenStreetMap::insertEdge(Edge* e, int index, NBNode* from, NBNode* to,
    //                                      const std::vector<long long int>& passed, NBNetBuilder& nb,
    //                                      const NBNode* first, const NBNode* last) {
    //     NBNodeCont& nc = nb.getNodeCont();
    //     NBEdgeCont& ec = nb.getEdgeCont();
    //     NBTypeCont& tc = nb.getTypeCont();
    //     NBPTStopCont& sc = nb.getPTStopCont();
    //
    //     NBTrafficLightLogicCont& tlsc = nb.getTLLogicCont();
    //     // patch the id
    //     std::string id = toString(e->id);
    //     if (from == nullptr || to == nullptr) {
    //         WRITE_ERRORF("Discarding edge '%' because the nodes could not be built.", id);
    //         return index;
    //     }
    //     if (index >= 0) {
    //         id = id + "#" + toString(index);
    //     } else {
    //         index = 0;
    //     }
    //     if (from == to) {
    //         assert(passed.size() >= 2);
    //         if (passed.size() == 2) {
    //             WRITE_WARNINGF(TL("Discarding edge '%' which connects two identical nodes without geometry."), id);
    //             return index;
    //         }
    //         // in the special case of a looped way split again using passed
    //         int intermediateIndex = (int) passed.size() / 2;
    //         NBNode* intermediate = insertNodeChecking(passed[intermediateIndex], nc, tlsc);
    //         std::vector<long long int> part1(passed.begin(), passed.begin() + intermediateIndex + 1);
    //         std::vector<long long int> part2(passed.begin() + intermediateIndex, passed.end());
    //         index = insertEdge(e, index, from, intermediate, part1, nb, first, last);
    //         return insertEdge(e, index, intermediate, to, part2, nb, first, last);
    //     }
    //     const int newIndex = index + 1;
    //     const std::string type = usableType(e->myHighWayType, id, tc);
    //     if (type == "") {  // we do not want to import it
    //         return newIndex;
    //     }
    //     std::string routingType = "";
    //     int numLanesForward = tc.getEdgeTypeNumLanes(type);
    //     int numLanesBackward = tc.getEdgeTypeNumLanes(type);
    //     double speed = tc.getEdgeTypeSpeed(type);
    //     bool defaultsToOneWay = tc.getEdgeTypeIsOneWay(type);
    //     const SVCPermissions defaultPermissions = tc.getEdgeTypePermissions(type);
    //     SVCPermissions extra = myImportBikeAccess ? e->myExtraAllowed : (e->myExtraAllowed & ~SVC_BICYCLE);
    //     const SVCPermissions extraDis = myImportBikeAccess ? e->myExtraDisallowed : (e->myExtraDisallowed & ~SVC_BICYCLE);
    //     std::vector<SumoXMLAttr> defaults;
    //     // extra permissions are more specific than extra prohibitions except for buses (which come from the less specific psv tag)
    //     if ((extraDis & SVC_BUS) && (extra & SVC_BUS)) {
    //         extra = extra & ~SVC_BUS;
    //     }
    //     SVCPermissions permissions = (defaultPermissions & ~extraDis) | extra;
    //     if (!myImportBikeAccess && permissions == (SVC_PEDESTRIAN | SVC_BICYCLE)
    //             && (e->myExtraDisallowed & SVC_BICYCLE) != 0
    //             && (e->myExtraAllowed & SVC_BICYCLE) == 0) {
    //         // remove bicyle permissions where they affect network building the most
    //         permissions = SVC_PEDESTRIAN;
    //         defaultsToOneWay = true;
    //     }
    //     if (defaultPermissions == SVC_SHIP) {
    //         // extra permission apply to the ships operating on the route rather than the waterway
    //         permissions = defaultPermissions;
    //     }
    //     if (defaultsToOneWay && defaultPermissions == SVC_PEDESTRIAN && (permissions & (~SVC_PEDESTRIAN)) != 0) {
    //         defaultsToOneWay = false;
    //     }
    //     if ((permissions & SVC_RAIL) != 0 && e->myExtraTags.count("electrified") != 0) {
    //         permissions |= (SVC_RAIL_ELECTRIC | SVC_RAIL_FAST);
    //     }
    //
    //     // convert the shape
    //     PositionVector shape;
    //     double distanceStart = myOSMNodes[passed.front()]->positionMeters;
    //     double distanceEnd = myOSMNodes[passed.back()]->positionMeters;
    //     const bool useDistance = distanceStart != std::numeric_limits<double>::max() && distanceEnd != std::numeric_limits<double>::max();
    //     if (useDistance) {
    //         // negative sign denotes counting in the other direction
    //         if (distanceStart < distanceEnd) {
    //             distanceEnd *= -1;
    //         } else {
    //             distanceStart *= -1;
    //         }
    //     } else {
    //         distanceStart = 0;
    //         distanceEnd = 0;
    //     }
    //     // get additional direction information
    //     int nodeDirection = WAY_UNKNOWN;
    //     const NIOSMNode* fn = myOSMNodes.find(StringUtils::toLong(from->getID()))->second;
    //     const NIOSMNode* ft = myOSMNodes.find(StringUtils::toLong(to->getID()))->second;
    //     if (fn->railwaySignal != SIGNAL_NONE) {
    //         nodeDirection |= fn->myRailDirection;
    //     }
    //     if (ft->railwaySignal != SIGNAL_NONE) {
    //         nodeDirection |= ft->myRailDirection;
    //     }
    //
    //     std::vector<std::shared_ptr<NBPTStop> > ptStops;
    //     for (long long i : passed) {
    //         NIOSMNode* n = myOSMNodes.find(i)->second;
    //         // recheck permissions, maybe they got assigned to a strange edge, see #11656
    //         if (n->ptStopPosition && (n->permissions == 0 || (permissions & n->permissions) != 0)) {
    //             std::shared_ptr<NBPTStop> existingPtStop = sc.get(toString(n->id));
    //             if (existingPtStop != nullptr) {
    //                 existingPtStop->registerAdditionalEdge(toString(e->id), id);
    //             } else {
    //                 Position ptPos(n->lon, n->lat, n->ele);
    //                 if (!NBNetBuilder::transformCoordinate(ptPos)) {
    //                     WRITE_ERRORF("Unable to project coordinates for node '%'.", n->id);
    //                 }
    //                 SumoXMLTag element = isRailway(n->permissions) ? SUMO_TAG_TRAIN_STOP : SUMO_TAG_BUS_STOP;
    //                 ptStops.push_back(std::make_shared<NBPTStop>(element, toString(n->id), ptPos, id, toString(e->id), n->ptStopLength, n->name, n->permissions));
    //                 sc.insert(ptStops.back());
    //             }
    //         }
    //         if (n->railwaySignal != SIGNAL_NONE) {
    //             nodeDirection |= n->myRailDirection;
    //         }
    //         Position pos(n->lon, n->lat, n->ele);
    //         shape.push_back(pos);
    //     }
    //     //if (e->id == DEBUGID) {
    //     //    std::cout
    //     //            << " id=" << id << " from=" << from->getID() << " fromRailDirection=" << myOSMNodes.find(StringUtils::toLong(from->getID()))->second->myRailDirection
    //     //            << " to=" << to->getID() << " toRailDirection=" << myOSMNodes.find(StringUtils::toLong(to->getID()))->second->myRailDirection
    //     //            << " origRailDirection=" << e->myRailDirection
    //     //            << " nodeDirection=" << nodeDirection
    //     //            << "\n";
    //     //}
    //     if (e->myRailDirection == WAY_UNKNOWN && (nodeDirection & WAY_BACKWARD) != 0) {
    //         // legacy behavior seems to have handled missing tags quite well
    //         e->myRailDirection = WAY_BOTH;
    //         //std::cout << " id=" << id << " newRailDir=" << e->myRailDirection << "\n";
    //     } else if (nodeDirection != WAY_UNKNOWN) {
    //         // additional direction information can just be added
    //         e->myRailDirection = (e->myRailDirection | nodeDirection) & ~WAY_UNKNOWN;
    //     }
    //
    //     if (!NBNetBuilder::transformCoordinates(shape)) {
    //         WRITE_ERRORF("Unable to project coordinates for edge '%'.", id);
    //     }
    //
    //     SVCPermissions forwardPermissions = permissions;
    //     SVCPermissions backwardPermissions = permissions;
    //     const std::string streetName = isRailway(permissions) && e->ref != "" ? e->ref : e->streetName;
    //     if (streetName == e->ref) {
    //         e->unsetParameter("ref"); // avoid superfluous param for railways
    //     }
    //     double forwardWidth = tc.getEdgeTypeWidth(type);
    //     double backwardWidth = tc.getEdgeTypeWidth(type);
    //     double sidewalkWidth = tc.getEdgeTypeSidewalkWidth(type);
    //     bool addSidewalk = sidewalkWidth != NBEdge::UNSPECIFIED_WIDTH;
    //     if (myImportSidewalks) {
    //         if (addSidewalk) {
    //             // only use sidewalk width from typemap but don't add sidewalks
    //             // unless OSM specifies them
    //             addSidewalk = false;
    //         } else {
    //             sidewalkWidth = OptionsCont::getOptions().getFloat("default.sidewalk-width");
    //         }
    //     }
    //     double bikeLaneWidth = tc.getEdgeTypeBikeLaneWidth(type);
    //     const std::string& onewayBike = e->myExtraTags["oneway:bicycle"];
    //     if (onewayBike == "false" || onewayBike == "no" || onewayBike == "0") {
    //         e->myCyclewayType = e->myCyclewayType == WAY_UNKNOWN ? WAY_BACKWARD : (WayType)(e->myCyclewayType | WAY_BACKWARD);
    //     }
    //
    //     const bool addBikeLane = bikeLaneWidth != NBEdge::UNSPECIFIED_WIDTH ||
    //                              (myImportBikeAccess && (((e->myCyclewayType & WAY_BOTH) != 0 || e->myExtraTags.count("segregated") != 0) &&
    //                                      !(e->myCyclewayType == WAY_BACKWARD && (e->myBuswayType & WAY_BOTH) != 0)));
    //     if (addBikeLane && bikeLaneWidth == NBEdge::UNSPECIFIED_WIDTH) {
    //         bikeLaneWidth = OptionsCont::getOptions().getFloat("default.bikelane-width");
    //     }
    //     // check directions
    //     bool addForward = true;
    //     bool addBackward = true;
    //     const bool explicitOneWay = StringUtils::isBool(e->myIsOneWay) && StringUtils::toBool(e->myIsOneWay);
    //     const bool explicitTwoWay = StringUtils::isBool(e->myIsOneWay) && !StringUtils::toBool(e->myIsOneWay);
    //     if ((explicitOneWay || (defaultsToOneWay && (!explicitTwoWay || isRailway(permissions)))) && (e->myRailDirection & WAY_BACKWARD) == 0) {
    //         addBackward = false;
    //     }
    //     if (e->myIsOneWay == "-1" || e->myIsOneWay == "reverse"
    //             || ((e->myRailDirection & WAY_BACKWARD) != 0 && (e->myRailDirection & WAY_FORWARD) == 0)) {
    //         // one-way in reversed direction of way
    //         addForward = false;
    //         addBackward = true;
    //     }
    //     if (!e->myIsOneWay.empty() && !explicitOneWay && !explicitTwoWay && e->myIsOneWay != "-1" && e->myIsOneWay != "reverse") {
    //         WRITE_WARNINGF(TL("New value for oneway found: %"), e->myIsOneWay);
    //     }
    //     if ((permissions == SVC_BICYCLE || permissions == (SVC_BICYCLE | SVC_PEDESTRIAN) || permissions == SVC_PEDESTRIAN)) {
    //         if (addBackward && (onewayBike == "true" || onewayBike == "yes" || onewayBike == "1")) {
    //             addBackward = false;
    //         }
    //         if (addForward && (onewayBike == "reverse" || onewayBike == "-1")) {
    //             addForward = false;
    //         }
    //         if (!addBackward && (onewayBike == "false" || onewayBike == "no" || onewayBike == "0")) {
    //             addBackward = true;
    //         }
    //     }
    //     // if we had been able to extract the number of lanes, override the highway type default
    //     if (e->myNoLanes > 0) {
    //         if (addForward && !addBackward) {
    //             numLanesForward = e->myNoLanesForward > 0 ? e->myNoLanesForward : e->myNoLanes;
    //         } else if (!addForward && addBackward) {
    //             numLanesBackward = e->myNoLanesForward < 0 ? -e->myNoLanesForward : e->myNoLanes;
    //         } else {
    //             if (e->myNoLanesForward > 0) {
    //                 numLanesForward = e->myNoLanesForward;
    //             } else if (e->myNoLanesForward < 0) {
    //                 numLanesForward = e->myNoLanes + e->myNoLanesForward;
    //             } else {
    //                 numLanesForward = (int) std::ceil(e->myNoLanes / 2.0);
    //             }
    //             numLanesBackward = e->myNoLanes - numLanesForward;
    //             // sometimes ways are tagged according to their physical width of a single
    //             // lane but they are intended for traffic in both directions
    //             numLanesForward = MAX2(1, numLanesForward);
    //             numLanesBackward = MAX2(1, numLanesBackward);
    //         }
    //     } else if (e->myNoLanes == 0) {
    //         WRITE_WARNINGF(TL("Skipping edge '%' because it has zero lanes."), id);
    //         return newIndex;
    //     } else {
    //         // the total number of lanes is not known but at least one direction
    //         if (e->myNoLanesForward > 0) {
    //             numLanesForward = e->myNoLanesForward;
    //         } else if ((e->myBuswayType & WAY_FORWARD) != 0 && (extraDis & SVC_PASSENGER) == 0) {
    //             // if we have a busway lane, yet cars may drive this implies at least two lanes
    //             numLanesForward = MAX2(numLanesForward, 2);
    //         }
    //         if (e->myNoLanesForward < 0) {
    //             numLanesBackward = -e->myNoLanesForward;
    //         } else if ((e->myBuswayType & WAY_BACKWARD) != 0 && (extraDis & SVC_PASSENGER) == 0) {
    //             // if we have a busway lane, yet cars may drive this implies at least two lanes
    //             numLanesBackward = MAX2(numLanesForward, 2);
    //         }
    //         if (myAnnotateDefaults && e->myNoLanesForward == 0) {
    //             defaults.push_back(SUMO_ATTR_NUMLANES);
    //         }
    //     }
    //     // deal with busways that run in the opposite direction of a one-way street
    //     if (!addForward && (e->myBuswayType & WAY_FORWARD) != 0) {
    //         addForward = true;
    //         forwardPermissions = SVC_BUS;
    //         numLanesForward = 1;
    //     }
    //     if (!addBackward && (e->myBuswayType & WAY_BACKWARD) != 0) {
    //         addBackward = true;
    //         backwardPermissions = SVC_BUS;
    //         numLanesBackward = 1;
    //     }
    //     // width is meant for raw lane count before adding sidewalks or cycleways
    //     const int taggedLanes = (addForward ? numLanesForward : 0) + (addBackward ? numLanesBackward : 0);
    //     if (e->myWidth > 0 && e->myWidthLanesForward.size() == 0 && e->myWidthLanesBackward.size() == 0 && taggedLanes != 0
    //             && !OptionsCont::getOptions().getBool("ignore-widths")) {
    //         // width is tagged excluding sidewalks and cycleways
    //         forwardWidth = e->myWidth / taggedLanes;
    //         backwardWidth = forwardWidth;
    //     }
    //
    //     // if we had been able to extract the maximum speed, override the type's default
    //     if (e->myMaxSpeed != MAXSPEED_UNGIVEN) {
    //         speed = e->myMaxSpeed;
    //     } else if (myAnnotateDefaults) {
    //         defaults.push_back(SUMO_ATTR_SPEED);
    //     }
    //     double speedBackward = speed;
    //     if (e->myMaxSpeedBackward != MAXSPEED_UNGIVEN) {
    //         speedBackward = e->myMaxSpeedBackward;
    //         addBackward = true;
    //     }
    //     if (speed <= 0 || speedBackward <= 0) {
    //         WRITE_WARNINGF(TL("Skipping edge '%' because it has speed %."), id, speed);
    //         return newIndex;
    //     }
    //     if (e->myNoLanes == 1 && addForward && addBackward) {
    //         // narrow road which now receives a total of 2 lanes but has less capacity than implied
    //         if (e->myWidth < 0 && e->myWidthLanesForward.size() == 0 && e->myWidthLanesBackward.size() == 0) {
    //             if (forwardWidth == NBEdge::UNSPECIFIED_WIDTH) {
    //                 forwardWidth = SUMO_const_laneWidth;
    //             }
    //             if (backwardWidth == NBEdge::UNSPECIFIED_WIDTH) {
    //                 backwardWidth = SUMO_const_laneWidth;
    //             }
    //             forwardWidth /= 2;
    //             backwardWidth /= 2;
    //         }
    //         if (e->myWidth < 5) {
    //             routingType = "narrow";
    //         }
    //     }
    //     // deal with cycleways that run in the opposite direction of a one-way street
    //     WayType cyclewayType = e->myCyclewayType; // make a copy because we do some temporary modifications
    //     if (addBikeLane) {
    //         if (!addForward && (cyclewayType & WAY_FORWARD) != 0) {
    //             addForward = true;
    //             forwardPermissions = SVC_BICYCLE;
    //             forwardWidth = bikeLaneWidth;
    //             numLanesForward = 1;
    //             // do not add an additional cycle lane
    //             cyclewayType = (WayType)(cyclewayType & ~WAY_FORWARD);
    //         }
    //         if (!addBackward && (cyclewayType & WAY_BACKWARD) != 0) {
    //             addBackward = true;
    //             backwardPermissions = SVC_BICYCLE;
    //             backwardWidth = bikeLaneWidth;
    //             numLanesBackward = 1;
    //             // do not add an additional cycle lane
    //             cyclewayType = (WayType)(cyclewayType & ~WAY_BACKWARD);
    //         }
    //     }
    //     // deal with sidewalks that run in the opposite direction of a one-way street
    //     WayType sidewalkType = e->mySidewalkType; // make a copy because we do some temporary modifications
    //     if (sidewalkType == WAY_UNKNOWN && (e->myExtraAllowed & SVC_PEDESTRIAN) != 0 && (permissions & SVC_PASSENGER) != 0) {
    //         // do not assume shared space unless sidewalk is actively disabled
    //         if (myOnewayDualSidewalk) {
    //             sidewalkType = WAY_BOTH;
    //         }
    //     }
    //     if (addSidewalk || (myImportSidewalks && (permissions & SVC_ROAD_CLASSES) != 0 && defaultPermissions != SVC_PEDESTRIAN)) {
    //         if (!addForward && (sidewalkType & WAY_FORWARD) != 0) {
    //             addForward = true;
    //             forwardPermissions = SVC_PEDESTRIAN;
    //             forwardWidth = tc.getEdgeTypeSidewalkWidth(type);
    //             numLanesForward = 1;
    //             // do not add an additional sidewalk
    //             sidewalkType = (WayType)(sidewalkType & ~WAY_FORWARD);  //clang tidy thinks "!WAY_FORWARD" is always false
    //         } else if (addSidewalk && addForward && (sidewalkType & WAY_BOTH) == 0
    //                    && numLanesForward == 1 && numLanesBackward <= 1
    //                    && (e->myExtraDisallowed & SVC_PEDESTRIAN) == 0) {
    //             // our typemap says pedestrians should walk here but the data says
    //             // there is no sidewalk at all. If the road is small, pedestrians can just walk
    //             // on the road
    //             forwardPermissions |= SVC_PEDESTRIAN;
    //         }
    //         if (!addBackward && (sidewalkType & WAY_BACKWARD) != 0) {
    //             addBackward = true;
    //             backwardPermissions = SVC_PEDESTRIAN;
    //             backwardWidth = tc.getEdgeTypeSidewalkWidth(type);
    //             numLanesBackward = 1;
    //             // do not add an additional cycle lane
    //             sidewalkType = (WayType)(sidewalkType & ~WAY_BACKWARD); //clang tidy thinks "!WAY_BACKWARD" is always false
    //         } else if (addSidewalk && addBackward && (sidewalkType & WAY_BOTH) == 0
    //                    && numLanesBackward == 1 && numLanesForward <= 1
    //                    && (e->myExtraDisallowed & SVC_PEDESTRIAN) == 0) {
    //             // our typemap says pedestrians should walk here but the data says
    //             // there is no sidewalk at all. If the road is small, pedestrians can just walk
    //             // on the road
    //             backwardPermissions |= SVC_PEDESTRIAN;
    //         }
    //     }
    //
    //     bool applyPlacement = false;
    //     double placementOffset = 0;
    //     if (e->myPlacement != PlacementType::NONE) {
    //         if (!explicitOneWay) {
    //             if (index <= 0) {
    //                 myPlacementSkippedNonExplicitOneWay++;
    //             }
    //         } else if (!(addForward && !addBackward)) {
    //             if (index <= 0) {
    //                 myPlacementSkippedAuxOppositeDirection++;
    //             }
    //         } else if (numLanesForward <= 0) {
    //             if (index <= 0) {
    //                 WRITE_WARNINGF(TL("Ignoring placement for edge '%' because lane count is invalid."), id);
    //             }
    //         } else {
    //             const double defaultPlacementWidth = forwardWidth == NBEdge::UNSPECIFIED_WIDTH || forwardWidth <= 0
    //                                                  ? SUMO_const_laneWidth : forwardWidth;
    //             std::vector<double> laneWidths((size_t)numLanesForward, defaultPlacementWidth);
    //             if (!OptionsCont::getOptions().getBool("ignore-widths")
    //                     && (int)e->myWidthLanesForward.size() == numLanesForward) {
    //                 for (int i = 0; i < numLanesForward; ++i) {
    //                     laneWidths[(size_t)i] = e->myWidthLanesForward[(size_t)i] > 0
    //                                             ? e->myWidthLanesForward[(size_t)i] : defaultPlacementWidth;
    //                 }
    //             }
    //             if (e->myPlacementLane < 1 || e->myPlacementLane > numLanesForward) {
    //                 if (index <= 0) {
    //                     WRITE_WARNINGF(TL("Ignoring placement for edge '%' because lane index '%' is out of range [1, %]."),
    //                                    id, e->myPlacementLane, numLanesForward);
    //                 }
    //             } else {
    //                 const int laneIndex = e->myPlacementLane - 1;
    //                 double leftOffset = 0;
    //                 for (int i = 0; i < laneIndex; ++i) {
    //                     leftOffset += laneWidths[(size_t)i];
    //                 }
    //                 double placementRefOffset = leftOffset;
    //                 if (e->myPlacement == PlacementType::RIGHT_OF) {
    //                     placementRefOffset += laneWidths[(size_t)laneIndex];
    //                 } else if (e->myPlacement == PlacementType::MIDDLE_OF) {
    //                     placementRefOffset += laneWidths[(size_t)laneIndex] / 2.;
    //                 }
    //                 double totalWidth = 0;
    //                 for (double laneWidth : laneWidths) {
    //                     totalWidth += laneWidth;
    //                 }
    //                 placementOffset = totalWidth / 2. - placementRefOffset;
    //                 applyPlacement = true;
    //             }
    //         }
    //     }
    //     if (applyPlacement && fabs(placementOffset) > POSITION_EPS) {
    //         try {
    //             shape.move2side(placementOffset);
    //         } catch (InvalidArgument&) {
    //             if (index <= 0) {
    //                 WRITE_WARNINGF(TL("Ignoring placement for edge '%' because offset shape computation failed."), id);
    //             }
    //             applyPlacement = false;
    //         }
    //     }
    //
    //     const std::string origID = OptionsCont::getOptions().getBool("output.original-names") ? toString(e->id) : "";
    //     const bool lefthand = OptionsCont::getOptions().getBool("lefthand");
    //     const int offsetFactor = lefthand ? -1 : 1;
    //     LaneSpreadFunction lsf = ((addBackward || OptionsCont::getOptions().getBool("osm.oneway-spread-right")) &&
    //             ((!isRailway(permissions) || (permissions == SVC_CABLE_CAR && e->myRailDirection == WAY_UNKNOWN)))
    //             ? LaneSpreadFunction::RIGHT : LaneSpreadFunction::CENTER);
    //     if (addBackward && lsf == LaneSpreadFunction::RIGHT && OptionsCont::getOptions().getString("default.spreadtype") == toString(LaneSpreadFunction::ROADCENTER)) {
    //         lsf = LaneSpreadFunction::ROADCENTER;
    //     }
    //     if (addForward && addBackward && lsf == LaneSpreadFunction::RIGHT && explicitOneWay) {
    //         lsf = LaneSpreadFunction::ROADCENTER;
    //     }
    //     if (tc.getEdgeTypeSpreadType(type) != LaneSpreadFunction::SPREAD_UNKNOWN) {
    //         // user defined value overrides defaults
    //         lsf = tc.getEdgeTypeSpreadType(type);
    //     }
    //     if (applyPlacement) {
    //         // placement references the directional edge centerline for one-way edges
    //         lsf = LaneSpreadFunction::CENTER;
    //     }
    //     if (defaults.size() > 0) {
    //         e->setParameter("osmDefaults", joinToString(defaults, " "));
    //     }
    //
    //     id = StringUtils::escapeXML(id);
    //     const std::string reverseID = "-" + id;
    //     const bool markOSMDirection =  from->getType() == SumoXMLNodeType::RAIL_SIGNAL || to->getType() == SumoXMLNodeType::RAIL_SIGNAL;
    //     if (addForward) {
    //         assert(numLanesForward > 0);
    //         NBEdge* nbe = new NBEdge(id, from, to, type, speed, NBEdge::UNSPECIFIED_FRICTION, numLanesForward, tc.getEdgeTypePriority(type),
    //                                  forwardWidth, NBEdge::UNSPECIFIED_OFFSET, shape, lsf,
    //                                  StringUtils::escapeXML(streetName), origID, true);
    //         if (markOSMDirection) {
    //             nbe->setParameter(NBTrafficLightDefinition::OSM_DIRECTION, "forward");
    //         }
    //         nbe->setPermissions(forwardPermissions, -1);
    //         if ((e->myBuswayType & WAY_FORWARD) != 0) {
    //             nbe->setPermissions(SVC_BUS, 0);
    //         }
    //         applyChangeProhibition(nbe, e->myChangeForward);
    //         applyLaneUse(nbe, e, true);
    //         applyTurnSigns(nbe, e->myTurnSignsForward);
    //         nbe->setTurnSignTarget(last->getID());
    //         if (addBikeLane && (cyclewayType == WAY_UNKNOWN || (cyclewayType & WAY_FORWARD) != 0)) {
    //             nbe->addBikeLane(bikeLaneWidth * offsetFactor);
    //         } else if (nbe->getPermissions(0) == SVC_BUS) {
    //             // bikes drive on buslanes if no separate cycle lane is available
    //             nbe->setPermissions(SVC_BUS | SVC_BICYCLE, 0);
    //         }
    //         if ((addSidewalk && (sidewalkType == WAY_UNKNOWN || (sidewalkType & WAY_FORWARD) != 0))
    //                 || (myImportSidewalks && (sidewalkType & WAY_FORWARD) != 0 && defaultPermissions != SVC_PEDESTRIAN)) {
    //             nbe->addSidewalk(sidewalkWidth * offsetFactor);
    //         }
    //         if (!addBackward && (e->myExtraAllowed & SVC_PEDESTRIAN) != 0 && (nbe->getPermissions(0) & SVC_PEDESTRIAN) == 0) {
    //             // Pedestrians are explicitly allowed (maybe through foot="yes") but did not get a sidewalk (maybe through sidewalk="no").
    //             // Since we do not have a backward edge, we need to make sure they can at least walk somewhere, see #14124
    //             nbe->setPermissions(nbe->getPermissions(0) | SVC_PEDESTRIAN, 0);
    //         }
    //         nbe->updateParameters(e->getParametersMap());
    //         nbe->setDistance(distanceStart);
    //         if (e->myAmInRoundabout) {
    //             // ensure roundabout edges have the precedence
    //             nbe->setJunctionPriority(to, NBEdge::JunctionPriority::ROUNDABOUT);
    //             nbe->setJunctionPriority(from, NBEdge::JunctionPriority::ROUNDABOUT);
    //         }
    //
    //         // process forward lanes width
    //         const int numForwardLanesFromWidthKey = (int)e->myWidthLanesForward.size();
    //         if (numForwardLanesFromWidthKey > 0 && !OptionsCont::getOptions().getBool("ignore-widths")) {
    //             if ((int)nbe->getLanes().size() != numForwardLanesFromWidthKey) {
    //                 WRITE_WARNINGF(TL("Forward lanes count for edge '%' ('%') is not matching the number of lanes defined in width:lanes:forward key ('%'). Using default width values."),
    //                                id, nbe->getLanes().size(), numForwardLanesFromWidthKey);
    //             } else {
    //                 for (int i = 0; i < numForwardLanesFromWidthKey; i++) {
    //                     const double actualWidth = e->myWidthLanesForward[i] <= 0 ? forwardWidth : e->myWidthLanesForward[i];
    //                     const int laneIndex = lefthand ? i : numForwardLanesFromWidthKey - i - 1;
    //                     nbe->setLaneWidth(laneIndex, actualWidth);
    //                 }
    //             }
    //         }
    //         if ((e->myRailDirection & WAY_PREFER_FORWARD) != 0 && isRailway(forwardPermissions)) {
    //             nbe->setRoutingType("4");
    //         } else {
    //             nbe->setRoutingType(routingType);
    //         }
    //
    //         if (!ec.insert(nbe)) {
    //             delete nbe;
    //             throw ProcessError(TLF("Could not add edge '%'.", id));
    //         }
    //     }
    //     if (addBackward) {
    //         assert(numLanesBackward > 0);
    //         NBEdge* nbe = new NBEdge(reverseID, to, from, type, speedBackward, NBEdge::UNSPECIFIED_FRICTION, numLanesBackward, tc.getEdgeTypePriority(type),
    //                                  backwardWidth, NBEdge::UNSPECIFIED_OFFSET, shape.reverse(), lsf,
    //                                  StringUtils::escapeXML(streetName), origID, true);
    //         if (markOSMDirection) {
    //             nbe->setParameter(NBTrafficLightDefinition::OSM_DIRECTION, "backward");
    //         }
    //         nbe->setPermissions(backwardPermissions);
    //         if ((e->myBuswayType & WAY_BACKWARD) != 0) {
    //             nbe->setPermissions(SVC_BUS, 0);
    //         }
    //         applyChangeProhibition(nbe, e->myChangeBackward);
    //         applyLaneUse(nbe, e, false);
    //         applyTurnSigns(nbe, e->myTurnSignsBackward);
    //         nbe->setTurnSignTarget(first->getID());
    //         if (addBikeLane && (cyclewayType == WAY_UNKNOWN || (cyclewayType & WAY_BACKWARD) != 0)) {
    //             nbe->addBikeLane(bikeLaneWidth * offsetFactor);
    //         } else if (nbe->getPermissions(0) == SVC_BUS) {
    //             // bikes drive on buslanes if no separate cycle lane is available
    //             nbe->setPermissions(SVC_BUS | SVC_BICYCLE, 0);
    //         }
    //         if ((addSidewalk && (sidewalkType == WAY_UNKNOWN || (sidewalkType & WAY_BACKWARD) != 0))
    //                 || (myImportSidewalks && (sidewalkType & WAY_BACKWARD) != 0 && defaultPermissions != SVC_PEDESTRIAN)) {
    //             nbe->addSidewalk(sidewalkWidth * offsetFactor);
    //         }
    //         nbe->updateParameters(e->getParametersMap());
    //         nbe->setDistance(distanceEnd);
    //         if (e->myAmInRoundabout) {
    //             // ensure roundabout edges have the precedence
    //             nbe->setJunctionPriority(from, NBEdge::JunctionPriority::ROUNDABOUT);
    //             nbe->setJunctionPriority(to, NBEdge::JunctionPriority::ROUNDABOUT);
    //         }
    //         // process backward lanes width
    //         const int numBackwardLanesFromWidthKey = (int)e->myWidthLanesBackward.size();
    //         if (numBackwardLanesFromWidthKey > 0 && !OptionsCont::getOptions().getBool("ignore-widths")) {
    //             if ((int)nbe->getLanes().size() != numBackwardLanesFromWidthKey) {
    //                 WRITE_WARNINGF(TL("Backward lanes count for edge '%' ('%') is not matching the number of lanes defined in width:lanes:backward key ('%'). Using default width values."),
    //                                id, nbe->getLanes().size(), numBackwardLanesFromWidthKey);
    //             } else {
    //                 for (int i = 0; i < numBackwardLanesFromWidthKey; i++) {
    //                     const double actualWidth = e->myWidthLanesBackward[i] <= 0 ? backwardWidth : e->myWidthLanesBackward[i];
    //                     const int laneIndex = lefthand ? i : numBackwardLanesFromWidthKey - i - 1;
    //                     nbe->setLaneWidth(laneIndex, actualWidth);
    //                 }
    //             }
    //         }
    //         if ((e->myRailDirection & WAY_PREFER_BACKWARD) != 0 && isRailway(backwardPermissions)) {
    //             nbe->setRoutingType("4");
    //         } else {
    //             nbe->setRoutingType(routingType);
    //         }
    //
    //         if (!ec.insert(nbe)) {
    //             delete nbe;
    //             throw ProcessError(TLF("Could not add edge '-%'.", id));
    //         }
    //     }
    //     if ((e->myParkingType & PARKING_BOTH) != 0 && OptionsCont::getOptions().isSet("parking-output")) {
    //         if ((e->myParkingType & PARKING_RIGHT) != 0) {
    //             if (addForward) {
    //                 nb.getParkingCont().push_back(NBParking(id, id));
    //             } else {
    //                 /// XXX parking area should be added on the left side of a reverse one-way street
    //                 if ((e->myParkingType & PARKING_LEFT) == 0 && !addBackward) {
    //                     /// put it on the wrong side (better than nothing)
    //                     nb.getParkingCont().push_back(NBParking(reverseID, reverseID));
    //                 }
    //             }
    //         }
    //         if ((e->myParkingType & PARKING_LEFT) != 0) {
    //             if (addBackward) {
    //                 nb.getParkingCont().push_back(NBParking(reverseID, reverseID));
    //             } else {
    //                 /// XXX parking area should be added on the left side of an one-way street
    //                 if ((e->myParkingType & PARKING_RIGHT) == 0 && !addForward) {
    //                     /// put it on the wrong side (better than nothing)
    //                     nb.getParkingCont().push_back(NBParking(id, id));
    //                 }
    //             }
    //         }
    //     }
    //     return newIndex;
    // }
    #[allow(unused_variables)]
    pub fn insert_edge(&mut self, e: Ptr<NIImporter_OpenStreetMap_Edge>, index: i32, from: Ptr<NBNode>, to: Ptr<NBNode>, passed: &Vec<i64>, nb: &mut NBNetBuilder, first: Ptr<NBNode>, last: Ptr<NBNode>) -> i32 {
        let WAY_NONE: i64 = NIImporter_OpenStreetMap_WayType::WAY_NONE as i64;
        let WAY_FORWARD: i64 = NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i64;
        let WAY_BACKWARD: i64 = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i64;
        let WAY_BOTH: i64 = NIImporter_OpenStreetMap_WayType::WAY_BOTH as i64;
        let WAY_UNKNOWN: i64 = NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i64;
        let WAY_PREFER_FORWARD: i64 = NIImporter_OpenStreetMap_WayType::WAY_PREFER_FORWARD as i64;
        let WAY_PREFER_BACKWARD: i64 = NIImporter_OpenStreetMap_WayType::WAY_PREFER_BACKWARD as i64;
        let PARKING_LEFT: i32 = NIImporter_OpenStreetMap_ParkingType::PARKING_LEFT as i32;
        let PARKING_RIGHT: i32 = NIImporter_OpenStreetMap_ParkingType::PARKING_RIGHT as i32;
        let PARKING_BOTH: i32 = NIImporter_OpenStreetMap_ParkingType::PARKING_BOTH as i32;
        let nc: Ptr<NBNodeCont> = Ptr::from_mut(nb.get_node_cont());
        let ec: Ptr<NBEdgeCont> = Ptr::from_mut(nb.get_edge_cont());
        let tc: Ptr<NBTypeCont> = Ptr::from_mut(nb.get_type_cont());
        let sc: Ptr<NBPTStopCont> = Ptr::from_mut(nb.get_pt_stop_cont());
        let tlsc: Ptr<NBTrafficLightLogicCont> = Ptr::from_mut(nb.get_tl_logic_cont());
        // patch the id
        let mut id = e.id.to_string();
        if from.is_null() || to.is_null() {
            write_errorf!("Discarding edge '%' because the nodes could not be built.", id);
            return index;
        }
        let mut index = index;
        if index >= 0 {
            id = format!("{id}#{index}");
        } else {
            index = 0;
        }
        if from == to {
            assert!(passed.len() >= 2);
            if passed.len() == 2 {
                write_warningf!("Discarding edge '%' which connects two identical nodes without geometry.", id);
                return index;
            }
            // in the special case of a looped way split again using passed
            let intermediateIndex = passed.len() / 2;
            let intermediate = self.insert_node_checking(passed[intermediateIndex], nc.m(), tlsc.m());
            let part1: Vec<i64> = passed[..=intermediateIndex].to_vec();
            let part2: Vec<i64> = passed[intermediateIndex..].to_vec();
            index = self.insert_edge(e, index, from, intermediate, &part1, nb, first, last);
            return self.insert_edge(e, index, intermediate, to, &part2, nb, first, last);
        }
        let newIndex = index + 1;
        let type_ = self.usable_type(&e.myHighWayType, &id, tc.m());
        if type_ == "" {
            // we do not want to import it
            return newIndex;
        }
        let mut routingType = String::new();
        let mut numLanesForward = tc.get_edge_type_num_lanes(&type_);
        let mut numLanesBackward = tc.get_edge_type_num_lanes(&type_);
        let mut speed = tc.get_edge_type_speed(&type_);
        let mut defaultsToOneWay = tc.get_edge_type_is_one_way(&type_);
        let defaultPermissions: SVCPermissions = tc.get_edge_type_permissions(&type_);
        let mut extra: SVCPermissions = if self.myImportBikeAccess { e.myExtraAllowed } else { e.myExtraAllowed & !SVC_BICYCLE };
        let extraDis: SVCPermissions = if self.myImportBikeAccess { e.myExtraDisallowed } else { e.myExtraDisallowed & !SVC_BICYCLE };
        let mut defaults: Vec<i32> = Vec::new(); // SumoXMLAttr values
        // extra permissions are more specific than extra prohibitions except for buses (which come from the less specific psv tag)
        if (extraDis & SVC_BUS) != 0 && (extra & SVC_BUS) != 0 {
            extra = extra & !SVC_BUS;
        }
        let mut permissions: SVCPermissions = (defaultPermissions & !extraDis) | extra;
        if !self.myImportBikeAccess && permissions == (SVC_PEDESTRIAN | SVC_BICYCLE) && (e.myExtraDisallowed & SVC_BICYCLE) != 0 && (e.myExtraAllowed & SVC_BICYCLE) == 0 {
            // remove bicyle permissions where they affect network building the most
            permissions = SVC_PEDESTRIAN;
            defaultsToOneWay = true;
        }
        if defaultPermissions == SVC_SHIP {
            // extra permission apply to the ships operating on the route rather than the waterway
            permissions = defaultPermissions;
        }
        if defaultsToOneWay && defaultPermissions == SVC_PEDESTRIAN && (permissions & (!SVC_PEDESTRIAN)) != 0 {
            defaultsToOneWay = false;
        }
        if (permissions & SVC_RAIL) != 0 && e.myExtraTags.contains_key("electrified") {
            permissions |= SVC_RAIL_ELECTRIC | SVC_RAIL_FAST;
        }

        // convert the shape
        let mut shape = PositionVector(Vec::new());
        let mut distanceStart = self.myOSMNodes[&passed[0]].positionMeters;
        let mut distanceEnd = self.myOSMNodes[&passed[passed.len() - 1]].positionMeters;
        let useDistance = distanceStart != f64::MAX && distanceEnd != f64::MAX;
        if useDistance {
            // negative sign denotes counting in the other direction
            if distanceStart < distanceEnd {
                distanceEnd *= -1.0;
            } else {
                distanceStart *= -1.0;
            }
        } else {
            distanceStart = 0.0;
            distanceEnd = 0.0;
        }

        // get additional direction information
        let mut nodeDirection: i64 = WAY_UNKNOWN;
        let fromOSM = self.myOSMNodes[&to_long(from.base.get_id()).unwrap_or_else(|err| panic!("{err}"))];
        let toOSM = self.myOSMNodes[&to_long(to.base.get_id()).unwrap_or_else(|err| panic!("{err}"))];
        if fromOSM.railwaySignal != NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 {
            nodeDirection |= fromOSM.myRailDirection as i64;
        }
        if toOSM.railwaySignal != NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 {
            nodeDirection |= toOSM.myRailDirection as i64;
        }

        let mut ptStops: Vec<Rc<NBPTStop>> = Vec::new();
        for i in passed.iter() {
            let n = self.myOSMNodes[i];
            // recheck permissions, maybe they got assigned to a strange edge, see #11656
            if n.ptStopPosition && (n.permissions == 0 || (permissions & n.permissions) != 0) {
                let existingPtStop: Option<Rc<NBPTStop>> = sc.get_stops().get(&n.id.to_string()).cloned();
                if let Some(existingPtStop) = existingPtStop {
                    pt_stop_mut(&existingPtStop).register_additional_edge(e.id.to_string(), id.clone());
                } else {
                    let mut ptPos = Position::new3(n.lon, n.lat, n.ele);
                    if !NBNetBuilder::transform_coordinate(&mut ptPos, true, Ptr::null()) {
                        write_errorf!("Unable to project coordinates for node '%'.", n.id);
                    }
                    let element = if is_railway(n.permissions) { SUMO_TAG_TRAIN_STOP } else { SUMO_TAG_BUS_STOP };
                    let stop: Rc<NBPTStop> = Rc::new(NBPTStop::new(
                        element, n.id.to_string(), ptPos, id.clone(), e.id.to_string(), n.ptStopLength, n.name.clone(), n.permissions,
                        0.0, RGBColor::new_valid(false), -1.0,
                    ));
                    ptStops.push(stop.clone());
                    sc.m().insert(stop, false);
                }
            }
            if n.railwaySignal != NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 {
                nodeDirection |= n.myRailDirection as i64;
            }
            let pos = Position::new3(n.lon, n.lat, n.ele);
            shape.push(pos);
        }
        //if (e->id == DEBUGID) { ... debug output omitted in C++ }
        if e.myRailDirection as i64 == WAY_UNKNOWN && (nodeDirection & WAY_BACKWARD) != 0 {
            // legacy behavior seems to have handled missing tags quite well
            e.m().myRailDirection = WAY_BOTH as i32;
        } else if nodeDirection != WAY_UNKNOWN {
            // additional direction information can just be added
            e.m().myRailDirection = ((e.myRailDirection as i64 | nodeDirection) & !WAY_UNKNOWN) as i32;
        }

        if !NBNetBuilder::transform_coordinates(&mut shape, true, Ptr::null()) {
            write_errorf!("Unable to project coordinates for edge '%'.", id);
        }

        let mut forwardPermissions = permissions;
        let mut backwardPermissions = permissions;
        let streetName: String = if is_railway(permissions) && e.ref_ != "" { e.ref_.clone() } else { e.streetName.clone() };
        if streetName == e.ref_ {
            e.m().unset_parameter("ref"); // avoid superfluous param for railways
        }
        let mut forwardWidth = tc.get_edge_type_width(&type_);
        let mut backwardWidth = tc.get_edge_type_width(&type_);
        let mut sidewalkWidth = tc.get_edge_type_sidewalk_width(&type_);
        let mut addSidewalk = sidewalkWidth != NBEDGE_UNSPECIFIED_WIDTH;
        if self.myImportSidewalks {
            if addSidewalk {
                // only use sidewalk width from typemap but don't add sidewalks
                // unless OSM specifies them
                addSidewalk = false;
            } else {
                sidewalkWidth = utils::options::options_cont::get_float("default.sidewalk-width");
            }
        }
        let mut bikeLaneWidth = tc.get_edge_type_bike_lane_width(&type_);
        let onewayBike: String = e.m().myExtraTags.entry("oneway:bicycle".to_string()).or_default().clone();
        if onewayBike == "false" || onewayBike == "no" || onewayBike == "0" {
            e.m().myCyclewayType = if e.myCyclewayType == WAY_UNKNOWN { WAY_BACKWARD } else { e.myCyclewayType | WAY_BACKWARD };
        }

        let addBikeLane = bikeLaneWidth != NBEDGE_UNSPECIFIED_WIDTH
            || (self.myImportBikeAccess && (((e.myCyclewayType & WAY_BOTH) != 0 || e.myExtraTags.contains_key("segregated"))
                && !(e.myCyclewayType == WAY_BACKWARD && (e.myBuswayType & WAY_BOTH) != 0)));
        if addBikeLane && bikeLaneWidth == NBEDGE_UNSPECIFIED_WIDTH {
            bikeLaneWidth = utils::options::options_cont::get_float("default.bikelane-width");
        }
        // check directions
        let mut addForward = true;
        let mut addBackward = true;
        let isBoolOneWay = is_bool(&e.myIsOneWay);
        let oneWayValue = isBoolOneWay && to_bool(&e.myIsOneWay).unwrap_or(false);
        let explicitOneWay = isBoolOneWay && oneWayValue;
        let explicitTwoWay = isBoolOneWay && !oneWayValue;
        if (explicitOneWay || (defaultsToOneWay && (!explicitTwoWay || is_railway(permissions)))) && (e.myRailDirection as i64 & WAY_BACKWARD) == 0 {
            addBackward = false;
        }
        if e.myIsOneWay == "-1" || e.myIsOneWay == "reverse" || ((e.myRailDirection as i64 & WAY_BACKWARD) != 0 && (e.myRailDirection as i64 & WAY_FORWARD) == 0) {
            // one-way in reversed direction of way
            addForward = false;
            addBackward = true;
        }
        if !e.myIsOneWay.is_empty() && !explicitOneWay && !explicitTwoWay && e.myIsOneWay != "-1" && e.myIsOneWay != "reverse" {
            write_warningf!("New value for oneway found: %", e.myIsOneWay.clone());
        }
        if permissions == SVC_BICYCLE || permissions == (SVC_BICYCLE | SVC_PEDESTRIAN) || permissions == SVC_PEDESTRIAN {
            if addBackward && (onewayBike == "true" || onewayBike == "yes" || onewayBike == "1") {
                addBackward = false;
            }
            if addForward && (onewayBike == "reverse" || onewayBike == "-1") {
                addForward = false;
            }
            if !addBackward && (onewayBike == "false" || onewayBike == "no" || onewayBike == "0") {
                addBackward = true;
            }
        }
        // if we had been able to extract the number of lanes, override the highway type default
        if e.myNoLanes > 0 {
            if addForward && !addBackward {
                numLanesForward = if e.myNoLanesForward > 0 { e.myNoLanesForward } else { e.myNoLanes };
            } else if !addForward && addBackward {
                numLanesBackward = if e.myNoLanesForward < 0 { -e.myNoLanesForward } else { e.myNoLanes };
            } else {
                if e.myNoLanesForward > 0 {
                    numLanesForward = e.myNoLanesForward;
                } else if e.myNoLanesForward < 0 {
                    numLanesForward = e.myNoLanes + e.myNoLanesForward;
                } else {
                    numLanesForward = (e.myNoLanes as f64 / 2.0).ceil() as i32;
                }
                numLanesBackward = e.myNoLanes - numLanesForward;
                // sometimes ways are tagged according to their physical width of a single
                // lane but they are intended for traffic in both directions
                numLanesForward = std::cmp::max(1, numLanesForward);
                numLanesBackward = std::cmp::max(1, numLanesBackward);
            }
        } else if e.myNoLanes == 0 {
            write_warningf!("Skipping edge '%' because it has zero lanes.", id);
            return newIndex;
        } else {
            // the total number of lanes is not known but at least one direction
            if e.myNoLanesForward > 0 {
                numLanesForward = e.myNoLanesForward;
            } else if (e.myBuswayType & WAY_FORWARD) != 0 && (extraDis & SVC_PASSENGER) == 0 {
                // if we have a busway lane, yet cars may drive this implies at least two lanes
                numLanesForward = std::cmp::max(numLanesForward, 2);
            }
            if e.myNoLanesForward < 0 {
                numLanesBackward = -e.myNoLanesForward;
            } else if (e.myBuswayType & WAY_BACKWARD) != 0 && (extraDis & SVC_PASSENGER) == 0 {
                // if we have a busway lane, yet cars may drive this implies at least two lanes
                numLanesBackward = std::cmp::max(numLanesForward, 2);
            }
            if self.myAnnotateDefaults && e.myNoLanesForward == 0 {
                defaults.push(SUMO_ATTR_NUMLANES);
            }
        }
        // deal with busways that run in the opposite direction of a one-way street
        if !addForward && (e.myBuswayType & WAY_FORWARD) != 0 {
            addForward = true;
            forwardPermissions = SVC_BUS;
            numLanesForward = 1;
        }
        if !addBackward && (e.myBuswayType & WAY_BACKWARD) != 0 {
            addBackward = true;
            backwardPermissions = SVC_BUS;
            numLanesBackward = 1;
        }
        // width is meant for raw lane count before adding sidewalks or cycleways
        let taggedLanes = (if addForward { numLanesForward } else { 0 }) + (if addBackward { numLanesBackward } else { 0 });
        if e.myWidth > 0.0 && e.myWidthLanesForward.is_empty() && e.myWidthLanesBackward.is_empty() && taggedLanes != 0 && !utils::options::options_cont::get_bool("ignore-widths") {
            // width is tagged excluding sidewalks and cycleways
            forwardWidth = e.myWidth / taggedLanes as f64;
            backwardWidth = forwardWidth;
        }

        // if we had been able to extract the maximum speed, override the type's default
        if e.myMaxSpeed != MAXSPEED_UNGIVEN {
            speed = e.myMaxSpeed;
        } else if self.myAnnotateDefaults {
            defaults.push(SUMO_ATTR_SPEED);
        }
        let mut speedBackward = speed;
        if e.myMaxSpeedBackward != MAXSPEED_UNGIVEN {
            speedBackward = e.myMaxSpeedBackward;
            addBackward = true;
        }
        if speed <= 0.0 || speedBackward <= 0.0 {
            write_warningf!("Skipping edge '%' because it has speed %.", id, speed);
            return newIndex;
        }
        if e.myNoLanes == 1 && addForward && addBackward {
            // narrow road which now receives a total of 2 lanes but has less capacity than implied
            if e.myWidth < 0.0 && e.myWidthLanesForward.is_empty() && e.myWidthLanesBackward.is_empty() {
                if forwardWidth == NBEDGE_UNSPECIFIED_WIDTH {
                    forwardWidth = SUMO_CONST_LANE_WIDTH;
                }
                if backwardWidth == NBEDGE_UNSPECIFIED_WIDTH {
                    backwardWidth = SUMO_CONST_LANE_WIDTH;
                }
                forwardWidth /= 2.0;
                backwardWidth /= 2.0;
            }
            if e.myWidth < 5.0 {
                routingType = "narrow".to_string();
            }
        }
        // deal with cycleways that run in the opposite direction of a one-way street
        let mut cyclewayType: i64 = e.myCyclewayType; // make a copy because we do some temporary modifications
        if addBikeLane {
            if !addForward && (cyclewayType & WAY_FORWARD) != 0 {
                addForward = true;
                forwardPermissions = SVC_BICYCLE;
                forwardWidth = bikeLaneWidth;
                numLanesForward = 1;
                // do not add an additional cycle lane
                cyclewayType &= !WAY_FORWARD;
            }
            if !addBackward && (cyclewayType & WAY_BACKWARD) != 0 {
                addBackward = true;
                backwardPermissions = SVC_BICYCLE;
                backwardWidth = bikeLaneWidth;
                numLanesBackward = 1;
                // do not add an additional cycle lane
                cyclewayType &= !WAY_BACKWARD;
            }
        }
        // deal with sidewalks that run in the opposite direction of a one-way street
        let mut sidewalkType: i64 = e.mySidewalkType; // make a copy because we do some temporary modifications
        if sidewalkType == WAY_UNKNOWN && (e.myExtraAllowed & SVC_PEDESTRIAN) != 0 && (permissions & SVC_PASSENGER) != 0 {
            // do not assume shared space unless sidewalk is actively disabled
            if self.myOnewayDualSidewalk {
                sidewalkType = WAY_BOTH;
            }
        }
        if addSidewalk || (self.myImportSidewalks && (permissions & SVC_ROAD_CLASSES) != 0 && defaultPermissions != SVC_PEDESTRIAN) {
            if !addForward && (sidewalkType & WAY_FORWARD) != 0 {
                addForward = true;
                forwardPermissions = SVC_PEDESTRIAN;
                forwardWidth = tc.get_edge_type_sidewalk_width(&type_);
                numLanesForward = 1;
                // do not add an additional sidewalk
                sidewalkType &= !WAY_FORWARD;
            } else if addSidewalk && addForward && (sidewalkType & WAY_BOTH) == 0 && numLanesForward == 1 && numLanesBackward <= 1 && (e.myExtraDisallowed & SVC_PEDESTRIAN) == 0 {
                // our typemap says pedestrians should walk here but the data says
                // there is no sidewalk at all. If the road is small, pedestrians can just walk
                // on the road
                forwardPermissions |= SVC_PEDESTRIAN;
            }
            if !addBackward && (sidewalkType & WAY_BACKWARD) != 0 {
                addBackward = true;
                backwardPermissions = SVC_PEDESTRIAN;
                backwardWidth = tc.get_edge_type_sidewalk_width(&type_);
                numLanesBackward = 1;
                // do not add an additional cycle lane
                sidewalkType &= !WAY_BACKWARD;
            } else if addSidewalk && addBackward && (sidewalkType & WAY_BOTH) == 0 && numLanesBackward == 1 && numLanesForward <= 1 && (e.myExtraDisallowed & SVC_PEDESTRIAN) == 0 {
                // our typemap says pedestrians should walk here but the data says
                // there is no sidewalk at all. If the road is small, pedestrians can just walk
                // on the road
                backwardPermissions |= SVC_PEDESTRIAN;
            }
        }

        let mut applyPlacement = false;
        let mut placementOffset: f64 = 0.0;
        if e.myPlacement != NIImporter_OpenStreetMap_PlacementType::NONE {
            if !explicitOneWay {
                if index <= 0 {
                    self.myPlacementSkippedNonExplicitOneWay += 1;
                }
            } else if !(addForward && !addBackward) {
                if index <= 0 {
                    self.myPlacementSkippedAuxOppositeDirection += 1;
                }
            } else if numLanesForward <= 0 {
                if index <= 0 {
                    write_warningf!("Ignoring placement for edge '%' because lane count is invalid.", id);
                }
            } else {
                let defaultPlacementWidth = if forwardWidth == NBEDGE_UNSPECIFIED_WIDTH || forwardWidth <= 0.0 { SUMO_CONST_LANE_WIDTH } else { forwardWidth };
                let mut laneWidths: Vec<f64> = vec![defaultPlacementWidth; numLanesForward as usize];
                if !utils::options::options_cont::get_bool("ignore-widths") && e.myWidthLanesForward.len() as i32 == numLanesForward {
                    for i in 0..numLanesForward as usize {
                        laneWidths[i] = if e.myWidthLanesForward[i] > 0.0 { e.myWidthLanesForward[i] } else { defaultPlacementWidth };
                    }
                }
                if e.myPlacementLane < 1 || e.myPlacementLane > numLanesForward {
                    if index <= 0 {
                        write_warningf!("Ignoring placement for edge '%' because lane index '%' is out of range [1, %].", id, e.myPlacementLane, numLanesForward);
                    }
                } else {
                    let laneIndex = (e.myPlacementLane - 1) as usize;
                    let mut leftOffset = 0.0;
                    for i in 0..laneIndex {
                        leftOffset += laneWidths[i];
                    }
                    let mut placementRefOffset = leftOffset;
                    if e.myPlacement == NIImporter_OpenStreetMap_PlacementType::RIGHT_OF {
                        placementRefOffset += laneWidths[laneIndex];
                    } else if e.myPlacement == NIImporter_OpenStreetMap_PlacementType::MIDDLE_OF {
                        placementRefOffset += laneWidths[laneIndex] / 2.0;
                    }
                    let mut totalWidth = 0.0;
                    for laneWidth in laneWidths.iter() {
                        totalWidth += laneWidth;
                    }
                    placementOffset = totalWidth / 2.0 - placementRefOffset;
                    applyPlacement = true;
                }
            }
        }
        if applyPlacement && placementOffset.abs() > utils::geom::POSITION_EPS {
            // shortcut: C++ catches InvalidArgument from move2side and skips placement; the Rust move2side has no Result, so a failure here is not caught, upgrade when move2side returns Result
            shape.move2side(placementOffset, 100.0);
        }

        let origID = if utils::options::options_cont::get_bool("output.original-names") { e.id.to_string() } else { String::new() };
        let lefthand = utils::options::options_cont::get_bool("lefthand");
        let offsetFactor: i32 = if lefthand { -1 } else { 1 };
        let mut lsf = if (addBackward || utils::options::options_cont::get_bool("osm.oneway-spread-right"))
            && (!is_railway(permissions) || (permissions == SVC_CABLE_CAR && e.myRailDirection as i64 == WAY_UNKNOWN))
        {
            LaneSpreadFunction::RIGHT
        } else {
            LaneSpreadFunction::CENTER
        };
        if addBackward && lsf == LaneSpreadFunction::RIGHT && utils::options::options_cont::get_string("default.spreadtype") == lane_spread_str(LaneSpreadFunction::ROADCENTER) {
            lsf = LaneSpreadFunction::ROADCENTER;
        }
        if addForward && addBackward && lsf == LaneSpreadFunction::RIGHT && explicitOneWay {
            lsf = LaneSpreadFunction::ROADCENTER;
        }
        if tc.get_edge_type_spread_type(&type_) != LaneSpreadFunction::SPREAD_UNKNOWN {
            // user defined value overrides defaults
            lsf = tc.get_edge_type_spread_type(&type_);
        }
        if applyPlacement {
            // placement references the directional edge centerline for one-way edges
            lsf = LaneSpreadFunction::CENTER;
        }
        let defaultsStr: String = defaults.iter().map(|a| attr_name(*a)).collect::<Vec<_>>().join(" ");
        if defaults.len() > 0 {
            e.m().set_parameter("osmDefaults", &defaultsStr);
        }

        id = escape_xml(&id, false);
        let reverseID = format!("-{id}");
        let markOSMDirection = from.get_type() == SumoXMLNodeType::RAIL_SIGNAL || to.get_type() == SumoXMLNodeType::RAIL_SIGNAL;
        if addForward {
            assert!(numLanesForward > 0);
            let nbe: Ptr<NBEdge> = Ptr::from_raw(Box::into_raw(NBEdge::new_2(
                &id, from, to, type_.clone(), speed, NBEDGE_UNSPECIFIED_FRICTION, numLanesForward, tc.get_edge_type_priority(&type_),
                forwardWidth, NBEDGE_UNSPECIFIED_OFFSET, shape.clone(), lsf, &escape_xml(&streetName, false), &origID, true,
            )));
            if markOSMDirection {
                nbe.m().base2.set_parameter(OSM_DIRECTION, "forward");
            }
            nbe.m().set_permissions(forwardPermissions, -1);
            if (e.myBuswayType & WAY_FORWARD) != 0 {
                nbe.m().set_permissions(SVC_BUS, 0);
            }
            NIImporter_OpenStreetMap::apply_change_prohibition(nbe, e.myChangeForward);
            self.apply_lane_use(nbe, e, true);
            self.apply_turn_signs(nbe, &e.myTurnSignsForward);
            nbe.m().set_turn_sign_target(last.base.get_id());
            if addBikeLane && (cyclewayType == WAY_UNKNOWN || (cyclewayType & WAY_FORWARD) != 0) {
                nbe.m().add_bike_lane(bikeLaneWidth * offsetFactor as f64);
            } else if nbe.get_permissions(0) == SVC_BUS {
                // bikes drive on buslanes if no separate cycle lane is available
                nbe.m().set_permissions(SVC_BUS | SVC_BICYCLE, 0);
            }
            if (addSidewalk && (sidewalkType == WAY_UNKNOWN || (sidewalkType & WAY_FORWARD) != 0))
                || (self.myImportSidewalks && (sidewalkType & WAY_FORWARD) != 0 && defaultPermissions != SVC_PEDESTRIAN)
            {
                nbe.m().add_sidewalk(sidewalkWidth * offsetFactor as f64);
            }
            if !addBackward && (e.myExtraAllowed & SVC_PEDESTRIAN) != 0 && (nbe.get_permissions(0) & SVC_PEDESTRIAN) == 0 {
                // Pedestrians are explicitly allowed (maybe through foot="yes") but did not get a sidewalk (maybe through sidewalk="no").
                // Since we do not have a backward edge, we need to make sure they can at least walk somewhere, see #14124
                nbe.m().set_permissions(nbe.get_permissions(0) | SVC_PEDESTRIAN, 0);
            }
            let params = e.get_parameters_map().clone();
            nbe.m().base2.update_parameters(&params);
            nbe.m().set_distance(distanceStart);
            if e.myAmInRoundabout {
                // ensure roundabout edges have the precedence
                nbe.m().set_junction_priority(to, NBEdge_JunctionPriority::ROUNDABOUT as i32);
                nbe.m().set_junction_priority(from, NBEdge_JunctionPriority::ROUNDABOUT as i32);
            }

            // process forward lanes width
            let numForwardLanesFromWidthKey = e.myWidthLanesForward.len() as i32;
            if numForwardLanesFromWidthKey > 0 && !utils::options::options_cont::get_bool("ignore-widths") {
                if nbe.get_lanes().len() as i32 != numForwardLanesFromWidthKey {
                    write_warningf!(
                        "Forward lanes count for edge '%' ('%') is not matching the number of lanes defined in width:lanes:forward key ('%'). Using default width values.",
                        id, nbe.get_lanes().len(), numForwardLanesFromWidthKey
                    );
                } else {
                    for i in 0..numForwardLanesFromWidthKey {
                        let actualWidth = if e.myWidthLanesForward[i as usize] <= 0.0 { forwardWidth } else { e.myWidthLanesForward[i as usize] };
                        let laneIndex = if lefthand { i } else { numForwardLanesFromWidthKey - i - 1 };
                        nbe.m().set_lane_width(laneIndex, actualWidth);
                    }
                }
            }
            if (e.myRailDirection as i64 & WAY_PREFER_FORWARD) != 0 && is_railway(forwardPermissions) {
                nbe.m().set_routing_type("4");
            } else {
                nbe.m().set_routing_type(&routingType);
            }

            if !ec.m().insert(nbe, false) {
                // SAFETY: the edge was rejected by the container, so nothing else holds it.
                drop(unsafe { Box::from_raw(nbe.as_raw()) });
                // shortcut: C++ throws ProcessError here; this fn returns i32, so panic with the same text instead
                panic!("{}", ProcessError::new(format!("Could not add edge '{id}'.")));
            }
        }
        if addBackward {
            assert!(numLanesBackward > 0);
            let nbe: Ptr<NBEdge> = Ptr::from_raw(Box::into_raw(NBEdge::new_2(
                &reverseID, to, from, type_.clone(), speedBackward, NBEDGE_UNSPECIFIED_FRICTION, numLanesBackward, tc.get_edge_type_priority(&type_),
                backwardWidth, NBEDGE_UNSPECIFIED_OFFSET, shape.reverse(), lsf, &escape_xml(&streetName, false), &origID, true,
            )));
            if markOSMDirection {
                nbe.m().base2.set_parameter(OSM_DIRECTION, "backward");
            }
            nbe.m().set_permissions(backwardPermissions, -1);
            if (e.myBuswayType & WAY_BACKWARD) != 0 {
                nbe.m().set_permissions(SVC_BUS, 0);
            }
            NIImporter_OpenStreetMap::apply_change_prohibition(nbe, e.myChangeBackward);
            self.apply_lane_use(nbe, e, false);
            self.apply_turn_signs(nbe, &e.myTurnSignsBackward);
            nbe.m().set_turn_sign_target(first.base.get_id());
            if addBikeLane && (cyclewayType == WAY_UNKNOWN || (cyclewayType & WAY_BACKWARD) != 0) {
                nbe.m().add_bike_lane(bikeLaneWidth * offsetFactor as f64);
            } else if nbe.get_permissions(0) == SVC_BUS {
                // bikes drive on buslanes if no separate cycle lane is available
                nbe.m().set_permissions(SVC_BUS | SVC_BICYCLE, 0);
            }
            if (addSidewalk && (sidewalkType == WAY_UNKNOWN || (sidewalkType & WAY_BACKWARD) != 0))
                || (self.myImportSidewalks && (sidewalkType & WAY_BACKWARD) != 0 && defaultPermissions != SVC_PEDESTRIAN)
            {
                nbe.m().add_sidewalk(sidewalkWidth * offsetFactor as f64);
            }
            let params = e.get_parameters_map().clone();
            nbe.m().base2.update_parameters(&params);
            nbe.m().set_distance(distanceEnd);
            if e.myAmInRoundabout {
                // ensure roundabout edges have the precedence
                nbe.m().set_junction_priority(from, NBEdge_JunctionPriority::ROUNDABOUT as i32);
                nbe.m().set_junction_priority(to, NBEdge_JunctionPriority::ROUNDABOUT as i32);
            }

            // process backward lanes width
            let numBackwardLanesFromWidthKey = e.myWidthLanesBackward.len() as i32;
            if numBackwardLanesFromWidthKey > 0 && !utils::options::options_cont::get_bool("ignore-widths") {
                if nbe.get_lanes().len() as i32 != numBackwardLanesFromWidthKey {
                    write_warningf!(
                        "Backward lanes count for edge '%' ('%') is not matching the number of lanes defined in width:lanes:backward key ('%'). Using default width values.",
                        id, nbe.get_lanes().len(), numBackwardLanesFromWidthKey
                    );
                } else {
                    for i in 0..numBackwardLanesFromWidthKey {
                        let actualWidth = if e.myWidthLanesBackward[i as usize] <= 0.0 { backwardWidth } else { e.myWidthLanesBackward[i as usize] };
                        let laneIndex = if lefthand { i } else { numBackwardLanesFromWidthKey - i - 1 };
                        nbe.m().set_lane_width(laneIndex, actualWidth);
                    }
                }
            }
            if (e.myRailDirection as i64 & WAY_PREFER_BACKWARD) != 0 && is_railway(backwardPermissions) {
                nbe.m().set_routing_type("4");
            } else {
                nbe.m().set_routing_type(&routingType);
            }

            if !ec.m().insert(nbe, false) {
                // SAFETY: the edge was rejected by the container, so nothing else holds it.
                drop(unsafe { Box::from_raw(nbe.as_raw()) });
                // shortcut: C++ throws ProcessError here; this fn returns i32, so panic with the same text instead
                panic!("{}", ProcessError::new(format!("Could not add edge '-{id}'.")));
            }
        }
        if (e.myParkingType & PARKING_BOTH) != 0 && utils::options::options_cont::is_set("parking-output") {
            if (e.myParkingType & PARKING_RIGHT) != 0 {
                if addForward {
                    nb.get_parking_cont().push(NBParking::new(&id, &id, ""));
                } else {
                    // XXX parking area should be added on the left side of a reverse one-way street
                    if (e.myParkingType & PARKING_LEFT) == 0 && !addBackward {
                        // put it on the wrong side (better than nothing)
                        nb.get_parking_cont().push(NBParking::new(&reverseID, &reverseID, ""));
                    }
                }
            }
            if (e.myParkingType & PARKING_LEFT) != 0 {
                if addBackward {
                    nb.get_parking_cont().push(NBParking::new(&reverseID, &reverseID, ""));
                } else {
                    // XXX parking area should be added on the left side of an one-way street
                    if (e.myParkingType & PARKING_RIGHT) == 0 && !addForward {
                        // put it on the wrong side (better than nothing)
                        nb.get_parking_cont().push(NBParking::new(&id, &id, ""));
                    }
                }
            }
        }
        newIndex
    }

    /// C++: `void NIImporter_OpenStreetMap::reconstructLayerElevation(double, NBNetBuilder &)` [private]
    /// reconstruct elevation from layer info
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn reconstruct_layer_elevation(&mut self, layerElevation: f64, nb: &mut NBNetBuilder) {
        let nc: Ptr<NBNodeCont> = Ptr::from_mut(nb.get_node_cont());
        // reconstruct elevation from layer info
        // build a map of raising and lowering forces (attractor and distance)
        // for all nodes unknownElevation
        let mut layerForces: BTreeMap<Ptr<NBNode>, Vec<(f64, f64)>> = BTreeMap::new();

        // collect all nodes that belong to a way with layer information
        let mut knownElevation: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        for myEdge in self.myEdges.values() {
            let e = *myEdge;
            if e.myLayer != 0 {
                for j in e.myCurrentNodes.iter() {
                    let node = nc.m().retrieve(&j.to_string());
                    if !node.is_null() {
                        knownElevation.insert(node);
                        layerForces.entry(node).or_default().push((e.myLayer as f64 * layerElevation, utils::geom::POSITION_EPS));
                    }
                }
            }
        }
        // layer data only provides a lower bound on elevation since it is used to
        // resolve the relation among overlapping ways.
        // Perform a sanity check for steep inclines and raise the knownElevation if necessary
        let mut knownEleMax: BTreeMap<Ptr<NBNode>, f64> = BTreeMap::new();
        for it in knownElevation.iter() {
            let mut eleMax = -f64::MAX;
            let primaryLayers = layerForces.entry(*it).or_default().clone();
            for primaryLayer in primaryLayers.iter() {
                eleMax = max2(eleMax, primaryLayer.0);
            }
            knownEleMax.insert(*it, eleMax);
        }
        let gradeThreshold = utils::options::options_cont::get_float("osm.layer-elevation.max-grade") / 100.0;
        let mut changed = true;
        while changed {
            changed = false;
            for it in knownElevation.iter() {
                let neighbors = self.get_neighboring_nodes(*it, knownEleMax[it] / gradeThreshold * 3.0, &knownElevation);
                for (neighborNode, neighborVal) in neighbors.iter() {
                    if knownElevation.contains(neighborNode) {
                        let grade = (knownEleMax[it] - knownEleMax[neighborNode]).abs() / max2(utils::geom::POSITION_EPS, neighborVal.0);
                        if grade > gradeThreshold * 50.0 / 3.6 / neighborVal.1 {
                            // raise the lower node to the higher level
                            let eleMax = max2(knownEleMax[it], knownEleMax[neighborNode]);
                            if knownEleMax[it] < eleMax {
                                knownEleMax.insert(*it, eleMax);
                            } else {
                                knownEleMax.insert(*neighborNode, eleMax);
                            }
                            changed = true;
                        }
                    }
                }
            }
        }

        // collect all nodes within a grade-dependent range around knownElevation-nodes and apply knowElevation forces
        let mut unknownElevation: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        for it in knownElevation.iter() {
            let eleMax = knownEleMax[it];
            let maxDist = eleMax.abs() * 100.0 / layerElevation;
            let neighbors = self.get_neighboring_nodes(*it, maxDist, &knownElevation);
            for (neighborNode, neighborVal) in neighbors.iter() {
                if !knownElevation.contains(neighborNode) {
                    unknownElevation.insert(*neighborNode);
                    layerForces.entry(*neighborNode).or_default().push((eleMax, neighborVal.0));
                }
            }
        }

        // apply forces to ground-level nodes (neither in knownElevation nor unknownElevation)
        for it in unknownElevation.iter() {
            let mut eleMax = -f64::MAX;
            let primaryLayers = layerForces.entry(*it).or_default().clone();
            for primaryLayer in primaryLayers.iter() {
                eleMax = max2(eleMax, primaryLayer.0);
            }
            let maxDist = eleMax.abs() * 100.0 / layerElevation;
            let neighbors = self.get_neighboring_nodes(*it, maxDist, &knownElevation);
            for (neighborNode, neighborVal) in neighbors.iter() {
                if !knownElevation.contains(neighborNode) && !unknownElevation.contains(neighborNode) {
                    layerForces.entry(*it).or_default().push((0.0, neighborVal.0));
                }
            }
        }
        // compute the elevation for each node as the weighted average of all forces
        let mut nodeElevation: BTreeMap<Ptr<NBNode>, f64> = BTreeMap::new();
        for (layerForceNode, forces) in layerForces.iter() {
            if knownElevation.contains(layerForceNode) {
                // use the maximum value
                nodeElevation.insert(*layerForceNode, knownEleMax[layerForceNode]);
            } else if forces.len() == 1 {
                nodeElevation.insert(*layerForceNode, forces[0].0);
            } else {
                // use the weighted sum
                let mut distSum = 0.0;
                for force in forces.iter() {
                    distSum += force.1;
                }
                let mut weightSum = 0.0;
                let mut elevation = 0.0;
                for force in forces.iter() {
                    let weight = (distSum - force.1) / distSum;
                    weightSum += weight;
                    elevation += force.0 * weight;
                }
                nodeElevation.insert(*layerForceNode, elevation / weightSum);
            }
        }
        // apply node elevations
        for (n, z) in nodeElevation.iter() {
            let n = *n;
            let newPos = n.get_position().clone() + Position::new3(0.0, 0.0, *z);
            let t = n.get_type();
            n.m().reinit(&newPos, t, false);
        }

        // apply way elevation to all edges that had layer information
        let edges: Vec<Ptr<NBEdge>> = nb.get_edge_cont().myEdges.values().copied().collect();
        for edge in edges {
            let geom = edge.get_geometry().clone();
            let length = geom.length2d();
            let zFrom = *nodeElevation.entry(edge.get_from_node()).or_insert(0.0);
            let zTo = *nodeElevation.entry(edge.get_to_node()).or_insert(0.0);
            // XXX if the from- or to-node was part of multiple ways with
            // different layers, reconstruct the layer value from origID
            let mut dist = 0.0;
            let mut newGeom = PositionVector(Vec::new());
            for (idx, itPos) in geom.iter().enumerate() {
                if idx != 0 {
                    dist += itPos.distance_to2d(&geom[idx - 1]);
                }
                newGeom.push(*itPos + Position::new3(0.0, 0.0, zFrom + (zTo - zFrom) * dist / length));
            }
            edge.m().set_geometry(&newGeom, false);
        }
    }

    /// C++: `std::map<NBNode *, std::pair<double, double>> NIImporter_OpenStreetMap::getNeighboringNodes(NBNode *, double, const std::set<NBNode *> &)` [private]
    /// collect neighboring nodes with their road distance and maximum between-speed. Search does not continue beyond knownElevation-nodes
    // std::map<NBNode*, std::pair<double, double> >
    // NIImporter_OpenStreetMap::getNeighboringNodes(NBNode* node, double maxDist, const std::set<NBNode*>& knownElevation) {
    //     std::map<NBNode*, std::pair<double, double> > result;
    //     std::set<NBNode*> visited;
    //     std::vector<NBNode*> open;
    //     open.push_back(node);
    //     result[node] = std::make_pair(0, 0);
    //     while (!open.empty()) {
    //         NBNode* n = open.back();
    //         open.pop_back();
    //         if (visited.count(n) != 0) {
    //             continue;
    //         }
    //         visited.insert(n);
    //         const EdgeVector& edges = n->getEdges();
    //         for (auto e : edges) {
    //             NBNode* s = nullptr;
    //             if (n->hasIncoming(e)) {
    //                 s = e->getFromNode();
    //             } else {
    //                 s = e->getToNode();
    //             }
    //             const double dist = result[n].first + e->getGeometry().length2D();
    //             const double speed = MAX2(e->getSpeed(), result[n].second);
    //             if (result.count(s) == 0) {
    //                 result[s] = std::make_pair(dist, speed);
    //             } else {
    //                 result[s] = std::make_pair(MIN2(dist, result[s].first), MAX2(speed, result[s].second));
    //             }
    //             if (dist < maxDist && knownElevation.count(s) == 0) {
    //                 open.push_back(s);
    //             }
    //         }
    //     }
    //     result.erase(node);
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_neighboring_nodes(&mut self, node: Ptr<NBNode>, maxDist: f64, knownElevation: &BTreeSet<Ptr<NBNode>>) -> BTreeMap<Ptr<NBNode>, (f64, f64)> {
        let mut result: BTreeMap<Ptr<NBNode>, (f64, f64)> = BTreeMap::new();
        let mut visited: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let mut open: Vec<Ptr<NBNode>> = Vec::new();
        open.push(node);
        result.insert(node, (0.0, 0.0));
        while let Some(n) = open.pop() {
            if visited.contains(&n) {
                continue;
            }
            visited.insert(n);
            let edges = n.get_edges().clone();
            for e in edges.iter() {
                let s = if n.has_incoming(*e) { e.get_from_node() } else { e.get_to_node() };
                let dist = result[&n].0 + e.get_geometry().length2d();
                let speed = max2(e.get_speed(), result[&n].1);
                if !result.contains_key(&s) {
                    result.insert(s, (dist, speed));
                } else {
                    let cur = result[&s];
                    result.insert(s, (min2(dist, cur.0), max2(speed, cur.1)));
                }
                if dist < maxDist && !knownElevation.contains(&s) {
                    open.push(s);
                }
            }
        }
        result.remove(&node);
        result
    }

    /// C++: `std::string NIImporter_OpenStreetMap::usableType(const std::string &, const std::string &, NBTypeCont &)` [private]
    /// check whether the type is known or consists of known type compounds. return empty string otherwise
    // std::string
    // NIImporter_OpenStreetMap::usableType(const std::string& type, const std::string& id, NBTypeCont& tc) {
    //     if (tc.knows(type)) {
    //         return type;
    //     }
    //     if (myUnusableTypes.count(type) > 0) {
    //         return "";
    //     }
    //     if (myKnownCompoundTypes.count(type) > 0) {
    //         return myKnownCompoundTypes[type];
    //     }
    //     // this edge has a type which does not yet exist in the TypeContainer
    //     StringTokenizer tok = StringTokenizer(type, compoundTypeSeparator);
    //     std::vector<std::string> types;
    //     while (tok.hasNext()) {
    //         std::string t = tok.next();
    //         if (tc.knows(t)) {
    //             if (std::find(types.begin(), types.end(), t) == types.end()) {
    //                 types.push_back(t);
    //             }
    //         } else if (tok.size() > 1) {
    //             if (!StringUtils::startsWith(t, "service.")) {
    //                 WRITE_WARNINGF(TL("Discarding unknown compound '%' in type '%' (first occurrence for edge '%')."), t, type, id);
    //             }
    //         }
    //     }
    //     if (types.empty()) {
    //         if (!StringUtils::startsWith(type, "service.")) {
    //             WRITE_WARNINGF(TL("Discarding unusable type '%' (first occurrence for edge '%')."), type, id);
    //         }
    //         myUnusableTypes.insert(type);
    //         return "";
    //     }
    //     const std::string newType = joinToString(types, "|");
    //     if (tc.knows(newType)) {
    //         myKnownCompoundTypes[type] = newType;
    //         return newType;
    //     } else if (myKnownCompoundTypes.count(newType) > 0) {
    //         return myKnownCompoundTypes[newType];
    //     } else {
    //         // build a new type by merging all values
    //         int numLanes = 0;
    //         double maxSpeed = 0;
    //         int prio = 0;
    //         double width = NBEdge::UNSPECIFIED_WIDTH;
    //         double sidewalkWidth = NBEdge::UNSPECIFIED_WIDTH;
    //         double bikelaneWidth = NBEdge::UNSPECIFIED_WIDTH;
    //         bool defaultIsOneWay = true;
    //         SVCPermissions permissions = 0;
    //         LaneSpreadFunction spreadType = LaneSpreadFunction::RIGHT;
    //         bool discard = true;
    //         bool hadDiscard = false;
    //         for (auto& type2 : types) {
    //             if (!tc.getEdgeTypeShallBeDiscarded(type2)) {
    //                 numLanes = MAX2(numLanes, tc.getEdgeTypeNumLanes(type2));
    //                 maxSpeed = MAX2(maxSpeed, tc.getEdgeTypeSpeed(type2));
    //                 prio = MAX2(prio, tc.getEdgeTypePriority(type2));
    //                 defaultIsOneWay &= tc.getEdgeTypeIsOneWay(type2);
    //                 //std::cout << "merging component " << type2 << " into type " << newType << " allows=" << getVehicleClassNames(tc.getPermissions(type2)) << " oneway=" << defaultIsOneWay << "\n";
    //                 permissions |= tc.getEdgeTypePermissions(type2);
    //                 spreadType = tc.getEdgeTypeSpreadType(type2);
    //                 width = MAX2(width, tc.getEdgeTypeWidth(type2));
    //                 sidewalkWidth = MAX2(sidewalkWidth, tc.getEdgeTypeSidewalkWidth(type2));
    //                 bikelaneWidth = MAX2(bikelaneWidth, tc.getEdgeTypeBikeLaneWidth(type2));
    //                 discard = false;
    //             } else {
    //                 hadDiscard = true;
    //             }
    //         }
    //         if (hadDiscard && permissions == 0) {
    //             discard = true;
    //         }
    //         if (discard) {
    //             WRITE_WARNINGF(TL("Discarding compound type '%' (first occurrence for edge '%')."), newType, id);
    //             myUnusableTypes.insert(newType);
    //             return "";
    //         }
    //         if (width != NBEdge::UNSPECIFIED_WIDTH) {
    //             width = MAX2(width, SUMO_const_laneWidth);
    //         }
    //         // ensure pedestrians don't run into trains
    //         if (sidewalkWidth == NBEdge::UNSPECIFIED_WIDTH
    //                 && (permissions & SVC_PEDESTRIAN) != 0
    //                 && (permissions & SVC_RAIL_CLASSES) != 0) {
    //             //std::cout << "patching sidewalk for type '" << newType << "' which allows=" << getVehicleClassNames(permissions) << "\n";
    //             sidewalkWidth = OptionsCont::getOptions().getFloat("default.sidewalk-width");
    //         }
    //
    //         WRITE_MESSAGEF(TL("Adding new type '%' (first occurrence for edge '%')."), type, id);
    //         tc.insertEdgeType(newType, numLanes, maxSpeed, prio, permissions, spreadType, width,
    //                           defaultIsOneWay, sidewalkWidth, bikelaneWidth, 0, 0, 0);
    //         for (auto& type3 : types) {
    //             if (!tc.getEdgeTypeShallBeDiscarded(type3)) {
    //                 tc.copyEdgeTypeRestrictionsAndAttrs(type3, newType);
    //             }
    //         }
    //         myKnownCompoundTypes[type] = newType;
    //         return newType;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn usable_type(&mut self, type_: &str, id: &str, tc: &mut NBTypeCont) -> String {
        if tc.knows(type_) {
            return type_.to_string();
        }
        if self.myUnusableTypes.contains(type_) {
            return String::new();
        }
        if self.myKnownCompoundTypes.contains_key(type_) {
            return self.myKnownCompoundTypes[type_].clone();
        }
        // this edge has a type which does not yet exist in the TypeContainer
        let mut tok = StringTokenizer::with_token(type_, compoundTypeSeparator, false);
        let mut types: Vec<String> = Vec::new();
        while tok.has_next() {
            let t = tok.next().unwrap_or_else(|e| panic!("{e}"));
            if tc.knows(&t) {
                if !types.contains(&t) {
                    types.push(t);
                }
            } else if tok.size() > 1 {
                if !starts_with(&t, "service.") {
                    write_warningf!("Discarding unknown compound '%' in type '%' (first occurrence for edge '%').", t, type_, id);
                }
            }
        }
        if types.is_empty() {
            if !starts_with(type_, "service.") {
                write_warningf!("Discarding unusable type '%' (first occurrence for edge '%').", type_, id);
            }
            self.myUnusableTypes.insert(type_.to_string());
            return String::new();
        }
        let newType = types.join(compoundTypeSeparator);
        if tc.knows(&newType) {
            self.myKnownCompoundTypes.insert(type_.to_string(), newType.clone());
            return newType;
        } else if self.myKnownCompoundTypes.contains_key(&newType) {
            return self.myKnownCompoundTypes[&newType].clone();
        } else {
            // build a new type by merging all values
            let mut numLanes: i32 = 0;
            let mut maxSpeed: f64 = 0.0;
            let mut prio: i32 = 0;
            let mut width: f64 = NBEDGE_UNSPECIFIED_WIDTH;
            let mut sidewalkWidth: f64 = NBEDGE_UNSPECIFIED_WIDTH;
            let mut bikelaneWidth: f64 = NBEDGE_UNSPECIFIED_WIDTH;
            let mut defaultIsOneWay = true;
            let mut permissions: SVCPermissions = 0;
            let mut spreadType = LaneSpreadFunction::RIGHT;
            let mut discard = true;
            let mut hadDiscard = false;
            for type2 in types.iter() {
                if !tc.get_edge_type_shall_be_discarded(type2) {
                    numLanes = std::cmp::max(numLanes, tc.get_edge_type_num_lanes(type2));
                    maxSpeed = max2(maxSpeed, tc.get_edge_type_speed(type2));
                    prio = std::cmp::max(prio, tc.get_edge_type_priority(type2));
                    defaultIsOneWay &= tc.get_edge_type_is_one_way(type2);
                    permissions |= tc.get_edge_type_permissions(type2);
                    spreadType = tc.get_edge_type_spread_type(type2);
                    width = max2(width, tc.get_edge_type_width(type2));
                    sidewalkWidth = max2(sidewalkWidth, tc.get_edge_type_sidewalk_width(type2));
                    bikelaneWidth = max2(bikelaneWidth, tc.get_edge_type_bike_lane_width(type2));
                    discard = false;
                } else {
                    hadDiscard = true;
                }
            }
            if hadDiscard && permissions == 0 {
                discard = true;
            }
            if discard {
                write_warningf!("Discarding compound type '%' (first occurrence for edge '%').", newType, id);
                self.myUnusableTypes.insert(newType.clone());
                return String::new();
            }
            if width != NBEDGE_UNSPECIFIED_WIDTH {
                width = max2(width, SUMO_CONST_LANE_WIDTH);
            }
            // ensure pedestrians don't run into trains
            if sidewalkWidth == NBEDGE_UNSPECIFIED_WIDTH && (permissions & SVC_PEDESTRIAN) != 0 && (permissions & SVC_RAIL_CLASSES) != 0 {
                sidewalkWidth = utils::options::options_cont::get_float("default.sidewalk-width");
            }

            write_messagef!("Adding new type '%' (first occurrence for edge '%').", type_, id);
            tc.insert_edge_type(&newType, numLanes, maxSpeed, prio, permissions, spreadType, width, defaultIsOneWay, sidewalkWidth, bikelaneWidth, 0.0, 0.0, 0.0);
            for type3 in types.iter() {
                if !tc.get_edge_type_shall_be_discarded(type3) {
                    tc.copy_edge_type_restrictions_and_attrs(type3, &newType);
                }
            }
            self.myKnownCompoundTypes.insert(type_.to_string(), newType.clone());
            newType
        }
    }

    /// C++: `void NIImporter_OpenStreetMap::extendRailwayDistances(Edge *, NBTypeCont &)` [private]
    /// extend kilometrage data for all nodes along railway
    // void
    // NIImporter_OpenStreetMap::extendRailwayDistances(Edge* e, NBTypeCont& tc) {
    //     const std::string id = toString(e->id);
    //     std::string type = usableType(e->myHighWayType, id, tc);
    //     if (type != "" && isRailway(tc.getEdgeTypePermissions(type))) {
    //         std::vector<NIOSMNode*> nodes;
    //         std::vector<double> usablePositions;
    //         std::vector<int> usableIndex;
    //         for (long long int n : e->myCurrentNodes) {
    //             NIOSMNode* node = myOSMNodes[n];
    //             node->positionMeters = interpretDistance(node);
    //             if (node->positionMeters != std::numeric_limits<double>::max()) {
    //                 usablePositions.push_back(node->positionMeters);
    //                 usableIndex.push_back((int)nodes.size());
    //             }
    //             nodes.push_back(node);
    //         }
    //         if (usablePositions.size() == 0) {
    //             return;
    //         } else {
    //             bool forward = true;
    //             if (usablePositions.size() == 1) {
    //                 WRITE_WARNINGF(TL("Ambiguous railway kilometrage direction for way '%' (assuming forward)"), id);
    //             } else {
    //                 forward = usablePositions.front() < usablePositions.back();
    //             }
    //             // check for consistency
    //             for (int i = 1; i < (int)usablePositions.size(); i++) {
    //                 if ((usablePositions[i - 1] < usablePositions[i]) != forward) {
    //                     WRITE_WARNINGF(TL("Inconsistent railway kilometrage direction for way '%': % (skipping)"), id, toString(usablePositions));
    //                     return;
    //                 }
    //             }
    //             if (nodes.size() > usablePositions.size()) {
    //                 // complete missing values
    //                 PositionVector shape;
    //                 for (NIOSMNode* node : nodes) {
    //                     shape.push_back(Position(node->lon, node->lat, 0));
    //                 }
    //                 if (!NBNetBuilder::transformCoordinates(shape)) {
    //                     return; // error will be given later
    //                 }
    //                 double sign = forward ? 1 : -1;
    //                 // extend backward before first usable value
    //                 for (int i = usableIndex.front() - 1; i >= 0; i--) {
    //                     nodes[i]->positionMeters = nodes[i + 1]->positionMeters - sign * shape[i].distanceTo2D(shape[i + 1]);
    //                 }
    //                 // extend forward
    //                 for (int i = usableIndex.front() + 1; i < (int)nodes.size(); i++) {
    //                     if (nodes[i]->positionMeters == std::numeric_limits<double>::max()) {
    //                         nodes[i]->positionMeters = nodes[i - 1]->positionMeters + sign * shape[i].distanceTo2D(shape[i - 1]);
    //                     }
    //                 }
    //                 //std::cout << " way=" << id << " usable=" << toString(usablePositions) << "\n indices=" << toString(usableIndex)
    //                 //    << " final:\n";
    //                 //for (auto n : nodes) {
    //                 //    std::cout << "    " << n->id << " " << n->positionMeters << " " << n->position<< "\n";
    //                 //}
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn extend_railway_distances(&mut self, e: Ptr<NIImporter_OpenStreetMap_Edge>, tc: &mut NBTypeCont) {
        let id = e.id.to_string();
        let type_ = self.usable_type(&e.myHighWayType, &id, tc);
        if type_ != "" && is_railway(tc.get_edge_type_permissions(&type_)) {
            let mut nodes: Vec<Ptr<NIImporter_OpenStreetMap_NIOSMNode>> = Vec::new();
            let mut usablePositions: Vec<f64> = Vec::new();
            let mut usableIndex: Vec<usize> = Vec::new();
            for n in e.myCurrentNodes.iter() {
                let node = self.myOSMNodes[n];
                node.m().positionMeters = Self::interpret_distance(node);
                if node.positionMeters != f64::MAX {
                    usablePositions.push(node.positionMeters);
                    usableIndex.push(nodes.len());
                }
                nodes.push(node);
            }
            if usablePositions.is_empty() {
                return;
            }
            let mut forward = true;
            if usablePositions.len() == 1 {
                write_warningf!("Ambiguous railway kilometrage direction for way '%' (assuming forward)", id);
            } else {
                forward = usablePositions[0] < usablePositions[usablePositions.len() - 1];
            }
            // check for consistency
            for i in 1..usablePositions.len() {
                if (usablePositions[i - 1] < usablePositions[i]) != forward {
                    write_warningf!("Inconsistent railway kilometrage direction for way '%': % (skipping)", id, join_to_string_prec(&usablePositions, " ", g_precision()));
                    return;
                }
            }
            if nodes.len() > usablePositions.len() {
                // complete missing values
                let mut shape = PositionVector(Vec::new());
                for node in nodes.iter() {
                    shape.push(Position::new3(node.lon, node.lat, 0.0));
                }
                if !NBNetBuilder::transform_coordinates(&mut shape, true, Ptr::null()) {
                    return; // error will be given later
                }
                let sign = if forward { 1.0 } else { -1.0 };
                // extend backward before first usable value
                let first = usableIndex[0];
                for i in (0..first).rev() {
                    nodes[i].m().positionMeters = nodes[i + 1].positionMeters - sign * shape[i].distance_to2d(&shape[i + 1]);
                }
                // extend forward
                for i in (first + 1)..nodes.len() {
                    if nodes[i].positionMeters == f64::MAX {
                        nodes[i].m().positionMeters = nodes[i - 1].positionMeters + sign * shape[i].distance_to2d(&shape[i - 1]);
                    }
                }
            }
        }
    }

    /// C++: `double NIImporter_OpenStreetMap::interpretDistance(NIOSMNode *)` [static, private]
    /// read distance value from node and return value in m
    // double
    // NIImporter_OpenStreetMap::interpretDistance(NIOSMNode* node) {
    //     if (node->position.size() > 0) {
    //         try {
    //             if (StringUtils::startsWith(node->position, "mi:")) {
    //                 return StringUtils::toDouble(node->position.substr(3)) * 1609.344; // meters per mile
    //             } else {
    //                 return StringUtils::toDouble(node->position) * 1000;
    //             }
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Value of railway:position is not numeric ('%') in node '%'."), node->position, toString(node->id));
    //         }
    //     }
    //     return std::numeric_limits<double>::max();
    // }
    #[allow(unused_variables)]
    pub fn interpret_distance(node: Ptr<NIImporter_OpenStreetMap_NIOSMNode>) -> f64 {
        if !node.position.is_empty() {
            let parsed = if starts_with(&node.position, "mi:") {
                utils::common::string_utils::to_double(&node.position[3..]).map(|v| v * 1609.344) // meters per mile
            } else {
                utils::common::string_utils::to_double(&node.position).map(|v| v * 1000.0)
            };
            match parsed {
                Ok(v) => return v,
                Err(_) => {
                    write_warningf!("Value of railway:position is not numeric ('%') in node '%'.", node.position, node.id.to_string());
                }
            }
        }
        f64::MAX
    }

    /// C++: `void NIImporter_OpenStreetMap::applyChangeProhibition(NBEdge *, int)` [static, protected]
    // void
    // NIImporter_OpenStreetMap::applyChangeProhibition(NBEdge* e, int changeProhibition) {
    //     bool multiLane = changeProhibition > 3;
    //     //std::cout << "applyChangeProhibition e=" << e->getID() << " changeProhibition=" << std::bitset<32>(changeProhibition) << " val=" << changeProhibition << "\n";
    //     for (int lane = 0; changeProhibition > 0 && lane < e->getNumLanes(); lane++) {
    //         int code = changeProhibition % 4; // only look at the last 2 bits
    //         SVCPermissions changeLeft = (code & CHANGE_NO_LEFT) == 0 ? SVCAll : (SVCPermissions)SVC_AUTHORITY;
    //         SVCPermissions changeRight = (code & CHANGE_NO_RIGHT) == 0 ? SVCAll : (SVCPermissions)SVC_AUTHORITY;
    //         e->setPermittedChanging(lane, changeLeft, changeRight);
    //         if (multiLane) {
    //             changeProhibition = changeProhibition >> 2;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_change_prohibition(e: Ptr<NBEdge>, changeProhibition: i32) {
        let mut changeProhibition = changeProhibition;
        let multiLane = changeProhibition > 3;
        let mut lane: i32 = 0;
        while changeProhibition > 0 && lane < e.get_num_lanes() {
            let code = changeProhibition % 4; // only look at the last 2 bits
            let changeLeft: SVCPermissions = if (code & NIImporter_OpenStreetMap_ChangeType::CHANGE_NO_LEFT as i32) == 0 { SVC_ALL } else { SVC_AUTHORITY };
            let changeRight: SVCPermissions = if (code & NIImporter_OpenStreetMap_ChangeType::CHANGE_NO_RIGHT as i32) == 0 { SVC_ALL } else { SVC_AUTHORITY };
            e.m().set_permitted_changing(lane, changeLeft, changeRight);
            if multiLane {
                changeProhibition = changeProhibition >> 2;
            }
            lane += 1;
        }
    }

    /// C++: `void NIImporter_OpenStreetMap::applyLaneUse(NBEdge *, NIImporter_OpenStreetMap::Edge *, const bool)` [protected]
    /// Applies lane use information from `nie` to `e`.
    // void
    // NIImporter_OpenStreetMap::applyLaneUse(NBEdge* e, NIImporter_OpenStreetMap::Edge* nie, const bool forward) {
    //     if (myImportLaneAccess) {
    //         const int numLanes = e->getNumLanes();
    //         const bool lefthand = OptionsCont::getOptions().getBool("lefthand");
    //         const std::vector<bool>& designated = forward ? nie->myDesignatedLaneForward : nie->myDesignatedLaneBackward;
    //         const std::vector<SVCPermissions>& allowed = forward ? nie->myAllowedLaneForward : nie->myAllowedLaneBackward;
    //         const std::vector<SVCPermissions>& disallowed = forward ? nie->myDisallowedLaneForward : nie->myDisallowedLaneBackward;
    //         for (int lane = 0; lane < numLanes; lane++) {
    //             // laneUse stores from left to right
    //             const int i = lefthand ? lane : numLanes - 1 - lane;
    //             // Extra allowed SVCs for this lane or none if no info was present for the lane
    //             const SVCPermissions extraAllowed = i < (int)allowed.size() ? allowed[i] : (SVCPermissions)SVC_IGNORING;
    //             // Extra disallowed SVCs for this lane or none if no info was present for the lane
    //             const SVCPermissions extraDisallowed = i < (int)disallowed.size() ? disallowed[i] : (SVCPermissions)SVC_IGNORING;
    //             if (i < (int)designated.size() && designated[i]) {
    //                 // if designated, delete all permissions
    //                 e->setPermissions(SVC_IGNORING, lane);
    //                 e->preferVehicleClass(lane, extraAllowed);
    //             }
    //             e->setPermissions((e->getPermissions(lane) | extraAllowed) & (~extraDisallowed), lane);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_lane_use(&mut self, e: Ptr<NBEdge>, nie: Ptr<NIImporter_OpenStreetMap_Edge>, forward: bool) {
        if self.myImportLaneAccess {
            let numLanes = e.get_num_lanes();
            let lefthand = utils::options::options_cont::get_bool("lefthand");
            let designated: &Vec<bool> = if forward { &nie.myDesignatedLaneForward } else { &nie.myDesignatedLaneBackward };
            let allowed: &Vec<SVCPermissions> = if forward { &nie.myAllowedLaneForward } else { &nie.myAllowedLaneBackward };
            let disallowed: &Vec<SVCPermissions> = if forward { &nie.myDisallowedLaneForward } else { &nie.myDisallowedLaneBackward };
            for lane in 0..numLanes {
                // laneUse stores from left to right
                let i = if lefthand { lane } else { numLanes - 1 - lane };
                // Extra allowed SVCs for this lane or none if no info was present for the lane
                let extraAllowed: SVCPermissions = if (i as usize) < allowed.len() { allowed[i as usize] } else { SVC_IGNORING };
                // Extra disallowed SVCs for this lane or none if no info was present for the lane
                let extraDisallowed: SVCPermissions = if (i as usize) < disallowed.len() { disallowed[i as usize] } else { SVC_IGNORING };
                if (i as usize) < designated.len() && designated[i as usize] {
                    // if designated, delete all permissions
                    e.m().set_permissions(SVC_IGNORING, lane);
                    e.m().prefer_vehicle_class(lane, extraAllowed);
                }
                let permissions = (e.get_permissions(lane) | extraAllowed) & (!extraDisallowed);
                e.m().set_permissions(permissions, lane);
            }
        }
    }

    /// C++: `void NIImporter_OpenStreetMap::mergeTurnSigns(std::vector<int> &, std::vector<int>)` [static, protected]
    // void
    // NIImporter_OpenStreetMap::mergeTurnSigns(std::vector<int>& signs, std::vector<int> signs2) {
    //     if (signs.empty()) {
    //         signs.insert(signs.begin(), signs2.begin(), signs2.end());
    //     } else {
    //         for (int i = 0; i < (int)MIN2(signs.size(), signs2.size()); i++) {
    //             signs[i] |= signs2[i];
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn merge_turn_signs(signs: &mut Vec<i32>, signs2: Vec<i32>) {
        if signs.is_empty() {
            signs.extend(signs2);
        } else {
            for i in 0..std::cmp::min(signs.len(), signs2.len()) {
                signs[i] |= signs2[i];
            }
        }
    }

    /// C++: `void NIImporter_OpenStreetMap::applyTurnSigns(NBEdge *, const std::vector<int> &)` [protected]
    // void
    // NIImporter_OpenStreetMap::applyTurnSigns(NBEdge* e, const std::vector<int>& turnSigns) {
    //     if (myImportTurnSigns && turnSigns.size() > 0) {
    //         // no sidewalks and bike lanes have been added yet
    //         if ((int)turnSigns.size() == e->getNumLanes()) {
    //             //std::cout << "apply turnSigns for " << e->getID() << " turnSigns=" << toString(turnSigns) << "\n";
    //             for (int i = 0; i < (int)turnSigns.size(); i++) {
    //                 // laneUse stores from left to right
    //                 const int laneIndex = e->getNumLanes() - 1 - i;
    //                 NBEdge::Lane& lane = e->getLaneStruct(laneIndex);
    //                 lane.turnSigns = turnSigns[i];
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Ignoring turn sign information for % lanes on edge % with % driving lanes"), turnSigns.size(), e->getID(), e->getNumLanes());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_turn_signs(&mut self, e: Ptr<NBEdge>, turnSigns: &Vec<i32>) {
        if self.myImportTurnSigns && turnSigns.len() > 0 {
            // no sidewalks and bike lanes have been added yet
            if turnSigns.len() as i32 == e.get_num_lanes() {
                for i in 0..turnSigns.len() {
                    // laneUse stores from left to right
                    let laneIndex = e.get_num_lanes() - 1 - i as i32;
                    let lane = e.m().get_lane_struct(laneIndex);
                    lane.turnSigns = turnSigns[i];
                }
            } else {
                write_warningf!("Ignoring turn sign information for % lanes on edge % with % driving lanes", turnSigns.len(), e.get_id(), e.get_num_lanes());
            }
        }
    }

}

/// An internal representation of an OSM-node
/// C++ class `NIImporter_OpenStreetMap::NIOSMNode` (netimport/NIImporter_OpenStreetMap.h)
#[repr(C)]
pub struct NIImporter_OpenStreetMap_NIOSMNode {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The node's id
    pub id: i64,
    /// The longitude the node is located at
    pub lon: f64,
    /// The latitude the node is located at
    pub lat: f64,
    /// The elevation of this node
    pub ele: f64,
    /// Whether this is a tls controlled junction
    pub tlsControlled: bool,
    /// Whether this is a pedestrian crossing
    pub pedestrianCrossing: bool,
    /// Whether this is a railway crossing
    pub railwayCrossing: bool,
    /// Whether this is a railway (main) signal
    pub railwaySignal: i32,
    /// Whether this is a railway buffer stop
    pub railwayBufferStop: bool,
    /// Whether this is a public transport stop position
    pub ptStopPosition: bool,
    /// The length of the pt stop
    pub ptStopLength: f64,
    /// The name of the node
    pub name: String,
    /// type of pt stop
    pub permissions: SVCPermissions,
    /// kilometrage/mileage
    pub position: String,
    /// position converted to m (using highest precision available)
    pub positionMeters: f64,
    /// Information about the direction(s) of railway usage
    pub myRailDirection: i32,
    /// the NBNode that was instantiated
    pub node: Ptr<NBNode>,
}

impl std::ops::Deref for NIImporter_OpenStreetMap_NIOSMNode {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_OpenStreetMap_NIOSMNode {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NIImporter_OpenStreetMap_NIOSMNode {
    /// C++: `void NIOSMNode::NIOSMNode(long long, double, double)`
    // NIOSMNode(long long int _id, double _lon, double _lat)
    //             :
    //             id(_id), lon(_lon), lat(_lat), ele(0.),
    //             tlsControlled(false),
    //             pedestrianCrossing(false),
    //             railwayCrossing(false),
    //             railwaySignal(SIGNAL_NONE),
    //             railwayBufferStop(false),
    //             ptStopPosition(false), ptStopLength(0), name(""),
    //             permissions(SVC_IGNORING),
    //             positionMeters(std::numeric_limits<double>::max()),
    //             myRailDirection(WAY_UNKNOWN),
    //             node(nullptr) { }
    #[allow(unused_variables)]
    pub fn new(_id: i64, _lon: f64, _lat: f64) -> Self {
        Self {
            base: Parameterised::new(),
            id: _id,
            lon: _lon,
            lat: _lat,
            ele: 0.0,
            tlsControlled: false,
            pedestrianCrossing: false,
            railwayCrossing: false,
            railwaySignal: NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32,
            railwayBufferStop: false,
            ptStopPosition: false,
            ptStopLength: 0.0,
            name: String::new(),
            permissions: SVC_IGNORING,
            position: String::new(),
            positionMeters: f64::MAX,
            myRailDirection: NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN as i32,
            node: Ptr::null(),
        }
    }

    /// C++: `NIOSMNode & NIOSMNode::operator=(const NIOSMNode &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_OpenStreetMap_NIOSMNode) -> &mut NIImporter_OpenStreetMap_NIOSMNode {
        // C++ declares this as `= delete`, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::NIOSMNode::operator= is deleted in C++")
    }

}

/// A class which extracts OSM-nodes from a parsed OSM-file
/// C++ class `NIImporter_OpenStreetMap::NodesHandler` (netimport/NIImporter_OpenStreetMap.h)
#[repr(C)]
pub struct NIImporter_OpenStreetMap_NodesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The nodes container to fill
    pub myToFill: Ptr<BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>>,
    /// id of the currently parsed node
    pub myLastNodeID: String,
    /// the currently parsed node
    pub myCurrentNode: Ptr<NIImporter_OpenStreetMap_NIOSMNode>,
    pub myIsStation: bool,
    pub myRailwayRef: String,
    /// The current hierarchy level
    pub myHierarchyLevel: i32,
    /// the set of unique nodes (used for duplicate detection/substitution)
    pub myUniqueNodes: Ptr<BTreeSet<NIImporter_OpenStreetMap_NodeByCoord>>, // C++: std::set<NIOSMNode *, CompareNodes> &
    /// whether elevation data should be imported
    pub myImportElevation: bool,
    /// custom requirements for rail signal tagging
    pub myRailSignalRules: StringVector,
    /// number of duplicate nodes
    pub myDuplicateNodes: i32,
    /// the options
    pub myOptionsCont: Ptr<OptionsCont>,
}

impl std::ops::Deref for NIImporter_OpenStreetMap_NodesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_OpenStreetMap_NodesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_OpenStreetMap_NodesHandler {
    /// C++: `void NodesHandler::NodesHandler(std::map<long long, NIOSMNode *> &, std::set<NIOSMNode *, CompareNodes> &, const OptionsCont &)`
    /// Constructor
    // NIImporter_OpenStreetMap::NodesHandler::NodesHandler(std::map<long long int, NIOSMNode*>& toFill,
    //         std::set<NIOSMNode*, CompareNodes>& uniqueNodes, const OptionsCont& oc) :
    //     SUMOSAXHandler("osm - file"),
    //     myToFill(toFill),
    //     myCurrentNode(nullptr),
    //     myIsStation(false),
    //     myHierarchyLevel(0),
    //     myUniqueNodes(uniqueNodes),
    //     myImportElevation(oc.getBool("osm.elevation")),
    //     myDuplicateNodes(0),
    //     myOptionsCont(oc) {
    //     // init rail signal rules
    //     for (std::string kv : oc.getStringVector("osm.railsignals")) {
    //         if (kv == "DEFAULT") {
    //             myRailSignalRules.push_back("railway:signal:main=");
    //             myRailSignalRules.push_back("railway:signal:combined=");
    //         } else if (kv == "ALL") {
    //             myRailSignalRules.push_back("railway=signal");
    //         } else {
    //             myRailSignalRules.push_back("railway:signal:" + kv);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(toFill: &mut BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>, uniqueNodes: &mut BTreeSet<NIImporter_OpenStreetMap_NodeByCoord>, cont: &OptionsCont) -> Self {
        let mut myRailSignalRules: StringVector = Vec::new();
        // init rail signal rules
        for kv in cont.get_string_vector("osm.railsignals").unwrap_or_else(|e| panic!("{e}")) {
            if kv == "DEFAULT" {
                myRailSignalRules.push("railway:signal:main=".to_string());
                myRailSignalRules.push("railway:signal:combined=".to_string());
            } else if kv == "ALL" {
                myRailSignalRules.push("railway=signal".to_string());
            } else {
                myRailSignalRules.push(format!("railway:signal:{kv}"));
            }
        }
        NIImporter_OpenStreetMap_NodesHandler {
            base: utils::xml::sumo_sax_handler::new("osm - file", ""),
            myToFill: Ptr::from_mut(toFill),
            myLastNodeID: String::new(),
            myCurrentNode: Ptr::null(),
            myIsStation: false,
            myRailwayRef: String::new(),
            myHierarchyLevel: 0,
            myUniqueNodes: Ptr::from_mut(uniqueNodes),
            myImportElevation: cont.get_bool("osm.elevation").unwrap_or_else(|e| panic!("{e}")),
            myRailSignalRules,
            myDuplicateNodes: 0,
            myOptionsCont: Ptr::new(cont),
        }
    }

    /// C++: `void NodesHandler::~NodesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_OpenStreetMap::NodesHandler::~NodesHandler()
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is defaulted; Rust drops the members.
    }

    /// C++: `int NodesHandler::getDuplicateNodes() const` [const]
    // int getDuplicateNodes() const {
    //             return myDuplicateNodes;
    //         }
    #[allow(unused_variables)]
    pub fn get_duplicate_nodes(&self) -> i32 {
        self.myDuplicateNodes
    }

    /// C++: `void NodesHandler::resetHierarchy()`
    // void resetHierarchy() {
    //             myHierarchyLevel = 0;
    //         }
    #[allow(unused_variables)]
    pub fn reset_hierarchy(&mut self) {
        self.myHierarchyLevel = 0;
    }

    /// C++: `void NodesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_OpenStreetMap::NodesHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     ++myHierarchyLevel;
    //     if (element == SUMO_TAG_NODE) {
    //         bool ok = true;
    //         myLastNodeID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //         if (myHierarchyLevel != 2) {
    //             WRITE_ERROR("Node element on wrong XML hierarchy level (id='" + myLastNodeID +
    //                         "', level='" + toString(myHierarchyLevel) + "').");
    //             return;
    //         }
    //         const std::string& action = attrs.getOpt<std::string>(SUMO_ATTR_ACTION, myLastNodeID.c_str(), ok);
    //         if (action == "delete" || !ok) {
    //             return;
    //         }
    //         try {
    //             // we do not use attrs.get here to save some time on parsing
    //             const long long int id = StringUtils::toLong(myLastNodeID);
    //             myCurrentNode = nullptr;
    //             const auto insertionIt = myToFill.lower_bound(id);
    //             if (insertionIt == myToFill.end() || insertionIt->first != id) {
    //                 // assume we are loading multiple files, so we won't report duplicate nodes
    //                 const double tlon = attrs.get<double>(SUMO_ATTR_LON, myLastNodeID.c_str(), ok);
    //                 const double tlat = attrs.get<double>(SUMO_ATTR_LAT, myLastNodeID.c_str(), ok);
    //                 if (!ok) {
    //                     return;
    //                 }
    //                 myCurrentNode = new NIOSMNode(id, tlon, tlat);
    //                 auto similarNode = myUniqueNodes.find(myCurrentNode);
    //                 if (similarNode == myUniqueNodes.end()) {
    //                     myUniqueNodes.insert(myCurrentNode);
    //                 } else {
    //                     delete myCurrentNode;
    //                     myCurrentNode = *similarNode;
    //                     myDuplicateNodes++;
    //                 }
    //                 myToFill.emplace_hint(insertionIt, id, myCurrentNode);
    //             }
    //         } catch (FormatException&) {
    //             WRITE_ERROR(TL("Attribute 'id' in the definition of a node is not of type long long int."));
    //             return;
    //         }
    //     }
    //     if (element == SUMO_TAG_TAG && myCurrentNode != nullptr) {
    //         if (myHierarchyLevel != 3) {
    //             WRITE_ERROR(TL("Tag element on wrong XML hierarchy level."));
    //             return;
    //         }
    //         bool ok = true;
    //         const std::string& key = attrs.get<std::string>(SUMO_ATTR_K, myLastNodeID.c_str(), ok, false);
    //         // we check whether the key is relevant (and we really need to transcode the value) to avoid hitting #1636
    //         if (key == "highway" || key == "ele" || key == "crossing" || key == "railway" || key == "public_transport"
    //                 || key == "name" || key == "train" || key == "bus" || key == "tram" || key == "light_rail" || key == "subway" || key == "station" || key == "noexit"
    //                 || key == "crossing:barrier"
    //                 || key == "crossing:light"
    //                 || key == "railway:ref"
    //                 || StringUtils::startsWith(key, "railway:signal")
    //                 || StringUtils::startsWith(key, "railway:position")
    //            ) {
    //             const std::string& value = attrs.get<std::string>(SUMO_ATTR_V, myLastNodeID.c_str(), ok, false);
    //             if (key == "highway" && value.find("traffic_signal") != std::string::npos) {
    //                 myCurrentNode->tlsControlled = true;
    //             } else if (key == "crossing" && value.find("traffic_signals") != std::string::npos) {
    //                 myCurrentNode->tlsControlled = true;
    //             } else if (key == "highway" && value.find("crossing") != std::string::npos) {
    //                 myCurrentNode->pedestrianCrossing = true;
    //             } else if ((key == "noexit" && value == "yes")
    //                        || (key == "railway" && value == "buffer_stop")) {
    //                 myCurrentNode->railwayBufferStop = true;
    //             } else if (key == "railway" && value.find("crossing") != std::string::npos) {
    //                 myCurrentNode->railwayCrossing = true;
    //             } else if (key == "crossing:barrier") {
    //                 myCurrentNode->setParameter("crossing:barrier", value);
    //             } else if (key == "crossing:light") {
    //                 myCurrentNode->setParameter("crossing:light", value);
    //             } else if (key == "railway:signal:direction") {
    //                 if (value == "both") {
    //                     myCurrentNode->myRailDirection = WAY_BOTH;
    //                 } else if (value == "backward") {
    //                     myCurrentNode->myRailDirection = WAY_BACKWARD;
    //                 } else if (value == "forward") {
    //                     myCurrentNode->myRailDirection = WAY_FORWARD;
    //                 }
    //             } else if (StringUtils::startsWith(key, "railway:signal") || (key == "railway" && value == "signal")) {
    //                 std::string kv = key + "=" + value;
    //                 std::string kglob = key + "=";
    //                 if (key == "railway:signal:train_protection" && value == "DE-ESO:ne14") {
    //                     myCurrentNode->railwaySignal |= SIGNAL_ETCS_BLOCK;
    //                 } else if (myCurrentNode->railwaySignal == SIGNAL_NONE) {
    //                     if ((std::find(myRailSignalRules.begin(), myRailSignalRules.end(), kv) != myRailSignalRules.end())
    //                             || (std::find(myRailSignalRules.begin(), myRailSignalRules.end(), kglob) != myRailSignalRules.end())) {
    //                         myCurrentNode->railwaySignal = SIGNAL_MAIN;
    //                     }
    //                 }
    //             } else if (StringUtils::startsWith(key, "railway:position") && value.size() > myCurrentNode->position.size()) {
    //                 // use the entry with the highest precision (more digits)
    //                 myCurrentNode->position = value;
    //             } else if ((key == "public_transport" && value == "stop_position") ||
    //                        (key == "highway" && value == "bus_stop")) {
    //                 myCurrentNode->ptStopPosition = true;
    //                 if (myCurrentNode->ptStopLength == 0) {
    //                     // default length
    //                     myCurrentNode->ptStopLength = myOptionsCont.getFloat("osm.stop-output.length");
    //                 }
    //             } else if (key == "name") {
    //                 myCurrentNode->name = value;
    //             } else if (myImportElevation && key == "ele") {
    //                 try {
    //                     const double elevation = StringUtils::parseDist(value);
    //                     if (std::isnan(elevation)) {
    //                         WRITE_WARNINGF(TL("Value of key '%' is invalid ('%') in node '%'."), key, value, myLastNodeID);
    //                     } else {
    //                         myCurrentNode->ele = elevation;
    //                     }
    //                 } catch (...) {
    //                     WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in node '%'."), key, value, myLastNodeID);
    //                 }
    //             } else if (key == "station") {
    //                 interpretTransportType(value, myCurrentNode);
    //                 myIsStation = true;
    //             } else if (key == "railway:ref") {
    //                 myRailwayRef = value;
    //             } else {
    //                 // v="yes"
    //                 interpretTransportType(key, myCurrentNode);
    //             }
    //         }
    //         if (myAllAttributes && (myExtraAttributes.count(key) != 0 || myExtraAttributes.size() == 0)) {
    //             const std::string info = "node=" + toString(myCurrentNode->id) + ", k=" + key;
    //             myCurrentNode->setParameter(key, attrs.get<std::string>(SUMO_ATTR_V, info.c_str(), ok, false));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myHierarchyLevel += 1;
        if element == SUMO_TAG_NODE {
            let mut ok = true;
            self.myLastNodeID = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
            if self.myHierarchyLevel != 2 {
                utils::common::msg_handler::write_error(&format!("Node element on wrong XML hierarchy level (id='{}', level='{}').", self.myLastNodeID, self.myHierarchyLevel));
                return;
            }
            let action: String = attrs.get_opt(SUMO_ATTR_ACTION, &self.myLastNodeID, &mut ok, String::new(), true);
            if action == "delete" || !ok {
                return;
            }
            // we do not use attrs.get here to save some time on parsing
            let id: i64 = match utils::common::string_utils::to_long(&self.myLastNodeID) {
                Ok(v) => v,
                Err(_) => {
                    utils::common::msg_handler::write_error("Attribute 'id' in the definition of a node is not of type long long int.");
                    return;
                }
            };
            self.myCurrentNode = Ptr::null();
            if !self.myToFill.contains_key(&id) {
                // assume we are loading multiple files, so we won't report duplicate nodes
                let tlon: f64 = attrs.get(SUMO_ATTR_LON, &self.myLastNodeID, &mut ok, true);
                let tlat: f64 = attrs.get(SUMO_ATTR_LAT, &self.myLastNodeID, &mut ok, true);
                if !ok {
                    return;
                }
                let mut current: Ptr<NIImporter_OpenStreetMap_NIOSMNode> = Ptr::from_raw(Box::into_raw(Box::new(NIImporter_OpenStreetMap_NIOSMNode::new(id, tlon, tlat))));
                self.myCurrentNode = current;
                let key = NIImporter_OpenStreetMap_NodeByCoord(current);
                let uniqueNodes = self.myUniqueNodes.m();
                if let Some(similarNode) = uniqueNodes.get(&key).copied() {
                    // SAFETY: the freshly allocated node was never inserted anywhere, so it is freed here.
                    drop(unsafe { Box::from_raw(current.as_raw()) });
                    current = similarNode.0;
                    self.myCurrentNode = current;
                    self.myDuplicateNodes += 1;
                } else {
                    uniqueNodes.insert(key);
                }
                self.myToFill.m().insert(id, current);
            }
        }
        if element == SUMO_TAG_TAG && !self.myCurrentNode.is_null() {
            if self.myHierarchyLevel != 3 {
                utils::common::msg_handler::write_error("Tag element on wrong XML hierarchy level.");
                return;
            }
            let mut ok = true;
            let node = self.myCurrentNode;
            let key: String = attrs.get(SUMO_ATTR_K, &self.myLastNodeID, &mut ok, false);
            // we check whether the key is relevant (and we really need to transcode the value) to avoid hitting #1636
            if key == "highway" || key == "ele" || key == "crossing" || key == "railway" || key == "public_transport"
                || key == "name" || key == "train" || key == "bus" || key == "tram" || key == "light_rail" || key == "subway" || key == "station" || key == "noexit"
                || key == "crossing:barrier"
                || key == "crossing:light"
                || key == "railway:ref"
                || starts_with(&key, "railway:signal")
                || starts_with(&key, "railway:position")
            {
                let value: String = attrs.get(SUMO_ATTR_V, &self.myLastNodeID, &mut ok, false);
                if key == "highway" && value.contains("traffic_signal") {
                    node.m().tlsControlled = true;
                } else if key == "crossing" && value.contains("traffic_signals") {
                    node.m().tlsControlled = true;
                } else if key == "highway" && value.contains("crossing") {
                    node.m().pedestrianCrossing = true;
                } else if (key == "noexit" && value == "yes") || (key == "railway" && value == "buffer_stop") {
                    node.m().railwayBufferStop = true;
                } else if key == "railway" && value.contains("crossing") {
                    node.m().railwayCrossing = true;
                } else if key == "crossing:barrier" {
                    node.m().set_parameter("crossing:barrier", &value);
                } else if key == "crossing:light" {
                    node.m().set_parameter("crossing:light", &value);
                } else if key == "railway:signal:direction" {
                    if value == "both" {
                        node.m().myRailDirection = NIImporter_OpenStreetMap_WayType::WAY_BOTH as i32;
                    } else if value == "backward" {
                        node.m().myRailDirection = NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i32;
                    } else if value == "forward" {
                        node.m().myRailDirection = NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i32;
                    }
                } else if starts_with(&key, "railway:signal") || (key == "railway" && value == "signal") {
                    let kv = format!("{key}={value}");
                    let kglob = format!("{key}=");
                    if key == "railway:signal:train_protection" && value == "DE-ESO:ne14" {
                        node.m().railwaySignal |= NIImporter_OpenStreetMap_SignalType::SIGNAL_ETCS_BLOCK as i32;
                    } else if node.railwaySignal == NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 {
                        if self.myRailSignalRules.contains(&kv) || self.myRailSignalRules.contains(&kglob) {
                            node.m().railwaySignal = NIImporter_OpenStreetMap_SignalType::SIGNAL_MAIN as i32;
                        }
                    }
                } else if starts_with(&key, "railway:position") && value.len() > node.position.len() {
                    // use the entry with the highest precision (more digits)
                    node.m().position = value;
                } else if (key == "public_transport" && value == "stop_position") || (key == "highway" && value == "bus_stop") {
                    node.m().ptStopPosition = true;
                    if node.ptStopLength == 0.0 {
                        // default length
                        node.m().ptStopLength = self.myOptionsCont.get_float("osm.stop-output.length").unwrap_or_else(|e| panic!("{e}"));
                    }
                } else if key == "name" {
                    node.m().name = value;
                } else if self.myImportElevation && key == "ele" {
                    match utils::common::string_utils::parse_dist(&value) {
                        Ok(elevation) => {
                            if elevation.is_nan() {
                                write_warningf!("Value of key '%' is invalid ('%') in node '%'.", key, value, self.myLastNodeID);
                            } else {
                                node.m().ele = elevation;
                            }
                        }
                        Err(_) => {
                            write_warningf!("Value of key '%' is not numeric ('%') in node '%'.", key, value, self.myLastNodeID);
                        }
                    }
                } else if key == "station" {
                    NIImporter_OpenStreetMap::interpret_transport_type(&value, node);
                    self.myIsStation = true;
                } else if key == "railway:ref" {
                    self.myRailwayRef = value;
                } else {
                    // v="yes"
                    NIImporter_OpenStreetMap::interpret_transport_type(&key, node);
                }
            }
            if myAllAttributes.with(|c| c.get()) && myExtraAttributes.with(|s| s.borrow().contains(&key) || s.borrow().is_empty()) {
                let info = format!("node={}, k={}", node.id, key);
                let v: String = attrs.get(SUMO_ATTR_V, &info, &mut ok, false);
                node.m().set_parameter(&key, &v);
            }
        }
    }

    /// C++: `void NodesHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIImporter_OpenStreetMap::NodesHandler::myEndElement(int element) {
    //     if (element == SUMO_TAG_NODE && myHierarchyLevel == 2) {
    //         if (myIsStation && myRailwayRef != "") {
    //             myCurrentNode->setParameter("railway:ref", myRailwayRef);
    //         }
    //         myCurrentNode = nullptr;
    //         myIsStation = false;
    //         myRailwayRef = "";
    //     }
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_NODE && self.myHierarchyLevel == 2 {
            if self.myIsStation && self.myRailwayRef != "" {
                self.myCurrentNode.m().set_parameter("railway:ref", &self.myRailwayRef);
            }
            self.myCurrentNode = Ptr::null();
            self.myIsStation = false;
            self.myRailwayRef = String::new();
        }
        self.myHierarchyLevel -= 1;
    }

    /// C++: `void NodesHandler::NodesHandler(const NodesHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_OpenStreetMap_NodesHandler) -> Self {
        // C++ declares the copy constructor private and never defines it, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::NodesHandler copy constructor is private and undefined in C++")
    }

    /// C++: `NodesHandler & NodesHandler::operator=(const NodesHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_OpenStreetMap_NodesHandler) -> &mut NIImporter_OpenStreetMap_NodesHandler {
        // C++ declares the assignment operator private and never defines it, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::NodesHandler::operator= is private and undefined in C++")
    }

}

/// A class which extracts relevant relation information from a parsed OSM-file - turn restrictions
/// C++ class `NIImporter_OpenStreetMap::RelationHandler` (netimport/NIImporter_OpenStreetMap.h)
#[repr(C)]
pub struct NIImporter_OpenStreetMap_RelationHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The previously parsed nodes
    pub myOSMNodes: Ptr<BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>>,
    /// The previously parsed edges
    pub myOSMEdges: Ptr<BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>>,
    /// The previously parsed platform shapes
    pub myPlatformShapes: Ptr<BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>>,
    /// The previously filled pt stop container
    pub myNBPTStopCont: Ptr<NBPTStopCont>,
    /// PT Line container to be filled
    pub myNBPTLineCont: Ptr<NBPTLineCont>,
    /// The currently parsed relation
    pub myCurrentRelation: i64,
    /// whether the currently parsed relation is a restriction
    pub myIsRestriction: bool,
    /// exceptions to the restriction currenlty being parsed
    pub myRestrictionException: SVCPermissions,
    /// the origination way for the current restriction
    pub myFromWay: i64,
    /// the destination way for the current restriction
    pub myToWay: i64,
    /// the via node/way for the current restriction
    pub myViaNode: i64,
    pub myViaWay: i64,
    /// the station node for the current stop_area
    pub myStation: i64,
    /// the options cont
    pub myOptionsCont: Ptr<OptionsCont>,
    pub myRestrictionType: RelationHandler_RestrictionType,
    /// bus stop references
    pub myStops: Vec<i64>,
    /// myStops which are actually platforms (in case there is no stop_position)
    pub myPlatformStops: BTreeSet<i64>,
    /// bus stop platforms
    pub myPlatforms: Vec<RelationHandler_NIIPTPlatform>,
    /// ways in pt line references
    pub myWays: Vec<i64>,
    /// indicates whether current relation is a pt stop area
    pub myIsStopArea: bool,
    /// indicates whether current relation is a route
    pub myIsRoute: bool,
    /// indicates whether current relation is a pt route
    pub myPTRouteType: String,
    /// official route color
    pub myRouteColor: RGBColor,
    /// name of the relation
    pub myName: String,
    /// ref of the pt line
    pub myRef: String,
    /// service interval of the pt line in minutes
    pub myInterval: i32,
    /// night service information of the pt line
    pub myNightService: String,
    /// the map from stop area member to stop_area id
    pub myStopAreas: BTreeMap<i64, i64>,
}

impl std::ops::Deref for NIImporter_OpenStreetMap_RelationHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_OpenStreetMap_RelationHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_OpenStreetMap_RelationHandler {
    /// C++: `void RelationHandler::RelationHandler(const std::map<long long, NIOSMNode *> &, const std::map<long long, Edge *> &, NBPTStopCont *, const std::map<long long, Edge *> &, NBPTLineCont *, const OptionsCont &)`
    /// Constructor
    // NIImporter_OpenStreetMap::RelationHandler::RelationHandler(
    //     const std::map<long long int, NIOSMNode*>& osmNodes,
    //     const std::map<long long int, Edge*>& osmEdges, NBPTStopCont* nbptStopCont,
    //     const std::map<long long int, Edge*>& platformShapes,
    //     NBPTLineCont* nbptLineCont,
    //     const OptionsCont& oc) :
    //     SUMOSAXHandler("osm - file"),
    //     myOSMNodes(osmNodes),
    //     myOSMEdges(osmEdges),
    //     myPlatformShapes(platformShapes),
    //     myNBPTStopCont(nbptStopCont),
    //     myNBPTLineCont(nbptLineCont),
    //     myOptionsCont(oc) {
    //     resetValues();
    // }
    #[allow(unused_variables)]
    pub fn new(osmNodes: &BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>, osmEdges: &BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>, nbptStopCont: Ptr<NBPTStopCont>, platfromShapes: &BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_Edge>>, nbptLineCont: Ptr<NBPTLineCont>, oc: &OptionsCont) -> Self {
        let mut handler = NIImporter_OpenStreetMap_RelationHandler {
            base: utils::xml::sumo_sax_handler::new("osm - file", ""),
            myOSMNodes: Ptr::new(osmNodes),
            myOSMEdges: Ptr::new(osmEdges),
            myPlatformShapes: Ptr::new(platfromShapes),
            myNBPTStopCont: nbptStopCont,
            myNBPTLineCont: nbptLineCont,
            // the remaining members are set by resetValues() below
            myCurrentRelation: INVALID_ID,
            myIsRestriction: false,
            myRestrictionException: SVC_IGNORING,
            myFromWay: INVALID_ID,
            myToWay: INVALID_ID,
            myViaNode: INVALID_ID,
            myViaWay: INVALID_ID,
            myStation: INVALID_ID,
            myOptionsCont: Ptr::new(oc),
            myRestrictionType: RelationHandler_RestrictionType::UNKNOWN,
            myStops: Vec::new(),
            myPlatformStops: BTreeSet::new(),
            myPlatforms: Vec::new(),
            myWays: Vec::new(),
            myIsStopArea: false,
            myIsRoute: false,
            myPTRouteType: String::new(),
            myRouteColor: RGBColor::default(),
            myName: String::new(),
            myRef: String::new(),
            myInterval: -1,
            myNightService: String::new(),
            myStopAreas: BTreeMap::new(),
        };
        handler.reset_values();
        handler
    }

    /// C++: `void RelationHandler::~RelationHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_OpenStreetMap::RelationHandler::~RelationHandler()
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is defaulted; Rust drops the members.
    }

    /// C++: `void RelationHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_OpenStreetMap::RelationHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_RELATION) {
    //         bool ok = true;
    //         myCurrentRelation = attrs.get<long long int>(SUMO_ATTR_ID, nullptr, ok);
    //         const std::string& action = attrs.getOpt<std::string>(SUMO_ATTR_ACTION, nullptr, ok);
    //         if (action == "delete" || !ok) {
    //             myCurrentRelation = INVALID_ID;
    //         }
    //         myName = "";
    //         myRef = "";
    //         myInterval = -1;
    //         myNightService = "";
    //         return;
    //     }
    //     if (myCurrentRelation == INVALID_ID) {
    //         return;
    //     }
    //     if (element == SUMO_TAG_MEMBER) {
    //         bool ok = true;
    //         std::string role = attrs.hasAttribute("role") ? attrs.getStringSecure("role", "") : "";
    //         const long long int ref = attrs.get<long long int>(SUMO_ATTR_REF, nullptr, ok);
    //         if (role == "via") {
    //             // u-turns for divided ways may be given with 2 via-nodes or 1 via-way
    //             std::string memberType = attrs.get<std::string>(SUMO_ATTR_TYPE, nullptr, ok);
    //             if (memberType == "way" && checkEdgeRef(ref)) {
    //                 myViaWay = ref;
    //             } else if (memberType == "node") {
    //                 if (myOSMNodes.find(ref) != myOSMNodes.end()) {
    //                     myViaNode = ref;
    //                 } else {
    //                     WRITE_WARNINGF(TL("No node found for reference '%' in relation '%'."), toString(ref), toString(myCurrentRelation));
    //                 }
    //             }
    //         } else if (role == "from" && checkEdgeRef(ref)) {
    //             myFromWay = ref;
    //         } else if (role == "to" && checkEdgeRef(ref)) {
    //             myToWay = ref;
    //         } else if (StringUtils::startsWith(role, "stop")) {
    //             // permit _entry_only and _exit_only variants
    //             myStops.push_back(ref);
    //         } else if (StringUtils::startsWith(role, "platform")) {
    //             // permit _entry_only and _exit_only variants
    //             std::string memberType = attrs.get<std::string>(SUMO_ATTR_TYPE, nullptr, ok);
    //             if (memberType == "way") {
    //                 const std::map<long long int, NIImporter_OpenStreetMap::Edge*>::const_iterator& wayIt = myPlatformShapes.find(ref);
    //                 if (wayIt != myPlatformShapes.end()) {
    //                     NIIPTPlatform platform;
    //                     platform.isWay = true;
    //                     platform.ref = ref;
    //                     myPlatforms.push_back(platform);
    //                 }
    //             } else if (memberType == "node") {
    //                 // myIsStopArea may not be set yet
    //                 myStops.push_back(ref);
    //                 myPlatformStops.insert(ref);
    //                 NIIPTPlatform platform;
    //                 platform.isWay = false;
    //                 platform.ref = ref;
    //                 myPlatforms.push_back(platform);
    //             }
    //
    //         } else if (role == "station") {
    //             myStation = ref;
    //         } else if (role.empty()) {
    //             std::string memberType = attrs.get<std::string>(SUMO_ATTR_TYPE, nullptr, ok);
    //             if (memberType == "way") {
    //                 myWays.push_back(ref);
    //             } else if (memberType == "node") {
    //                 auto it = myOSMNodes.find(ref);
    //                 if (it != myOSMNodes.end() && it->second->hasParameter("railway:ref")) {
    //                     myStation = ref;
    //                 } else {
    //                     myStops.push_back(ref);
    //                 }
    //             }
    //         }
    //         return;
    //     }
    //     // parse values
    //     if (element == SUMO_TAG_TAG) {
    //         bool ok = true;
    //         std::string key = attrs.get<std::string>(SUMO_ATTR_K, toString(myCurrentRelation).c_str(), ok, false);
    //         // we check whether the key is relevant (and we really need to transcode the value) to avoid hitting #1636
    //         if (key == "type" || key == "restriction") {
    //             std::string value = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //             if (key == "type" && value == "restriction") {
    //                 myIsRestriction = true;
    //                 return;
    //             }
    //             if (key == "type" && value == "route") {
    //                 myIsRoute = true;
    //                 return;
    //             }
    //             if (key == "restriction") {
    //                 // @note: the 'right/left/straight' part is ignored since the information is
    //                 // redundantly encoded in the 'from', 'to' and 'via' members
    //                 if (value.substr(0, 5) == "only_") {
    //                     myRestrictionType = RestrictionType::ONLY;
    //                 } else if (value.substr(0, 3) == "no_") {
    //                     myRestrictionType = RestrictionType::NO;
    //                 } else {
    //                     WRITE_WARNINGF(TL("Found unknown restriction type '%' in relation '%'"), value, toString(myCurrentRelation));
    //                 }
    //                 return;
    //             }
    //         } else if (key == "except") {
    //             std::string value = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //             for (const std::string& v : StringTokenizer(value, ";").getVector()) {
    //                 if (v == "psv") {
    //                     myRestrictionException |= SVC_BUS;
    //                 } else if (v == "bicycle") {
    //                     myRestrictionException |= SVC_BICYCLE;
    //                 } else if (v == "hgv") {
    //                     myRestrictionException |= SVC_TRUCK | SVC_TRAILER;
    //                 } else if (v == "motorcar") {
    //                     myRestrictionException |= SVC_PASSENGER | SVC_TAXI;
    //                 } else if (v == "emergency") {
    //                     myRestrictionException |= SVC_EMERGENCY;
    //                 }
    //             }
    //         } else if (key == "public_transport") {
    //             std::string value = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //             if (value == "stop_area") {
    //                 myIsStopArea = true;
    //             }
    //         } else if (key == "route") {
    //             std::string value = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //             if (value == "train" || value == "subway" || value == "light_rail" || value == "monorail" || value == "tram" || value == "bus"
    //                     || value == "trolleybus" || value == "aerialway" || value == "ferry" || value == "share_taxi" || value == "minibus") {
    //                 myPTRouteType = value;
    //             }
    //
    //         } else if (key == "name") {
    //             myName = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //         } else if (key == "colour") {
    //             std::string value = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //             try {
    //                 myRouteColor = RGBColor::parseColor(value);
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Invalid color value '%' in relation %"), value, myCurrentRelation);
    //             }
    //         } else if (key == "ref") {
    //             myRef = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //         } else if (key == "interval" || key == "headway") {
    //             myInterval = attrs.get<int>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //         } else if (key == "by_night") {
    //             myNightService = attrs.get<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, false);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element == SUMO_TAG_RELATION {
            let mut ok = true;
            self.myCurrentRelation = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
            let action: String = attrs.get_opt(SUMO_ATTR_ACTION, "", &mut ok, String::new(), true);
            if action == "delete" || !ok {
                self.myCurrentRelation = INVALID_ID;
            }
            self.myName = String::new();
            self.myRef = String::new();
            self.myInterval = -1;
            self.myNightService = String::new();
            return;
        }
        if self.myCurrentRelation == INVALID_ID {
            return;
        }
        if element == SUMO_TAG_MEMBER {
            let mut ok = true;
            let role: String = if attrs.has_attribute_name("role") { attrs.get_string_secure_name("role", "") } else { String::new() };
            let ref_: i64 = attrs.get(SUMO_ATTR_REF, "", &mut ok, true);
            if role == "via" {
                // u-turns for divided ways may be given with 2 via-nodes or 1 via-way
                let memberType: String = attrs.get(SUMO_ATTR_TYPE, "", &mut ok, true);
                if memberType == "way" && self.check_edge_ref(ref_) {
                    self.myViaWay = ref_;
                } else if memberType == "node" {
                    if self.myOSMNodes.contains_key(&ref_) {
                        self.myViaNode = ref_;
                    } else {
                        write_warningf!("No node found for reference '%' in relation '%'.", ref_, self.myCurrentRelation);
                    }
                }
            } else if role == "from" && self.check_edge_ref(ref_) {
                self.myFromWay = ref_;
            } else if role == "to" && self.check_edge_ref(ref_) {
                self.myToWay = ref_;
            } else if starts_with(&role, "stop") {
                // permit _entry_only and _exit_only variants
                self.myStops.push(ref_);
            } else if starts_with(&role, "platform") {
                // permit _entry_only and _exit_only variants
                let memberType: String = attrs.get(SUMO_ATTR_TYPE, "", &mut ok, true);
                if memberType == "way" {
                    if self.myPlatformShapes.contains_key(&ref_) {
                        self.myPlatforms.push(RelationHandler_NIIPTPlatform { ref_, isWay: true });
                    }
                } else if memberType == "node" {
                    // myIsStopArea may not be set yet
                    self.myStops.push(ref_);
                    self.myPlatformStops.insert(ref_);
                    self.myPlatforms.push(RelationHandler_NIIPTPlatform { ref_, isWay: false });
                }
            } else if role == "station" {
                self.myStation = ref_;
            } else if role.is_empty() {
                let memberType: String = attrs.get(SUMO_ATTR_TYPE, "", &mut ok, true);
                if memberType == "way" {
                    self.myWays.push(ref_);
                } else if memberType == "node" {
                    let hasRailwayRef = (*self.myOSMNodes).get(&ref_).is_some_and(|n| n.has_parameter("railway:ref"));
                    if hasRailwayRef {
                        self.myStation = ref_;
                    } else {
                        self.myStops.push(ref_);
                    }
                }
            }
            return;
        }
        // parse values
        if element == SUMO_TAG_TAG {
            let mut ok = true;
            let relation = self.myCurrentRelation.to_string();
            let key: String = attrs.get(SUMO_ATTR_K, &relation, &mut ok, false);
            // we check whether the key is relevant (and we really need to transcode the value) to avoid hitting #1636
            if key == "type" || key == "restriction" {
                let value: String = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
                if key == "type" && value == "restriction" {
                    self.myIsRestriction = true;
                    return;
                }
                if key == "type" && value == "route" {
                    self.myIsRoute = true;
                    return;
                }
                if key == "restriction" {
                    // @note: the 'right/left/straight' part is ignored since the information is
                    // redundantly encoded in the 'from', 'to' and 'via' members
                    if value.get(0..5) == Some("only_") {
                        self.myRestrictionType = RelationHandler_RestrictionType::ONLY;
                    } else if value.get(0..3) == Some("no_") {
                        self.myRestrictionType = RelationHandler_RestrictionType::NO;
                    } else {
                        write_warningf!("Found unknown restriction type '%' in relation '%'", value, self.myCurrentRelation);
                    }
                    return;
                }
            } else if key == "except" {
                let value: String = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
                let mut tok = StringTokenizer::with_token(&value, ";", false);
                for v in tok.get_vector() {
                    if v == "psv" {
                        self.myRestrictionException |= SVC_BUS;
                    } else if v == "bicycle" {
                        self.myRestrictionException |= SVC_BICYCLE;
                    } else if v == "hgv" {
                        self.myRestrictionException |= SVC_TRUCK | SVC_TRAILER;
                    } else if v == "motorcar" {
                        self.myRestrictionException |= SVC_PASSENGER | SVC_TAXI;
                    } else if v == "emergency" {
                        self.myRestrictionException |= SVC_EMERGENCY;
                    }
                }
            } else if key == "public_transport" {
                let value: String = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
                if value == "stop_area" {
                    self.myIsStopArea = true;
                }
            } else if key == "route" {
                let value: String = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
                if value == "train" || value == "subway" || value == "light_rail" || value == "monorail" || value == "tram" || value == "bus"
                    || value == "trolleybus" || value == "aerialway" || value == "ferry" || value == "share_taxi" || value == "minibus" {
                    self.myPTRouteType = value;
                }
            } else if key == "name" {
                self.myName = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
            } else if key == "colour" {
                let value: String = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
                match utils::common::rgb_color::RGBColor::parse_color(&value) {
                    Ok(color) => self.myRouteColor = color,
                    Err(_) => {
                        write_warningf!("Invalid color value '%' in relation %", value, self.myCurrentRelation);
                    }
                }
            } else if key == "ref" {
                self.myRef = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
            } else if key == "interval" || key == "headway" {
                self.myInterval = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
            } else if key == "by_night" {
                self.myNightService = attrs.get(SUMO_ATTR_V, &relation, &mut ok, false);
            }
        }
    }

    /// C++: `void RelationHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIImporter_OpenStreetMap::RelationHandler::myEndElement(int element) {
    //     if (element == SUMO_TAG_RELATION) {
    //         if (myIsRestriction) {
    //             assert(myCurrentRelation != INVALID_ID);
    //             bool ok = true;
    //             if (myRestrictionType == RestrictionType::UNKNOWN) {
    //                 WRITE_WARNINGF(TL("Ignoring restriction relation '%' with unknown type."), toString(myCurrentRelation));
    //                 ok = false;
    //             }
    //             if (myFromWay == INVALID_ID) {
    //                 WRITE_WARNINGF(TL("Ignoring restriction relation '%' with unknown from-way."), toString(myCurrentRelation));
    //                 ok = false;
    //             }
    //             if (myToWay == INVALID_ID) {
    //                 WRITE_WARNINGF(TL("Ignoring restriction relation '%' with unknown to-way."), toString(myCurrentRelation));
    //                 ok = false;
    //             }
    //             if (myViaNode == INVALID_ID && myViaWay == INVALID_ID) {
    //                 WRITE_WARNINGF(TL("Ignoring restriction relation '%' with unknown via."), toString(myCurrentRelation));
    //                 ok = false;
    //             }
    //             if (ok && !applyRestriction()) {
    //                 WRITE_WARNINGF(TL("Ignoring restriction relation '%'."), toString(myCurrentRelation));
    //             }
    //         } else if (myIsStopArea) {
    //             for (long long ref : myStops) {
    //                 myStopAreas[ref] = myCurrentRelation;
    //                 if (myOSMNodes.find(ref) == myOSMNodes.end()) {
    //                     //WRITE_WARNING(
    //                     //    "Referenced node: '" + toString(ref) + "' in relation: '" + toString(myCurrentRelation)
    //                     //    + "' does not exist. Probably OSM file is incomplete.");
    //                     continue;
    //                 }
    //
    //                 NIOSMNode* n = myOSMNodes.find(ref)->second;
    //                 std::shared_ptr<NBPTStop> ptStop = myNBPTStopCont->get(toString(n->id));
    //                 if (ptStop == nullptr) {
    //                     //WRITE_WARNING(
    //                     //    "Relation '" + toString(myCurrentRelation) + "' refers to a non existing pt stop at node: '"
    //                     //    + toString(n->id) + "'. Probably OSM file is incomplete.");
    //                     continue;
    //                 }
    //                 for (NIIPTPlatform& myPlatform : myPlatforms) {
    //                     if (myPlatform.isWay) {
    //                         assert(myPlatformShapes.find(myPlatform.ref) != myPlatformShapes.end()); //already tested earlier
    //                         Edge* edge = (*myPlatformShapes.find(myPlatform.ref)).second;
    //                         if (edge->myCurrentNodes.size() > 1 && edge->myCurrentNodes[0] == *(edge->myCurrentNodes.end() - 1)) {
    //                             WRITE_WARNINGF(TL("Platform '%' in relation: '%' is given as polygon, which currently is not supported."), myPlatform.ref, myCurrentRelation);
    //                             continue;
    //
    //                         }
    //                         PositionVector p;
    //                         for (auto nodeRef : edge->myCurrentNodes) {
    //                             if (myOSMNodes.find(nodeRef) == myOSMNodes.end()) {
    //                                 //WRITE_WARNING(
    //                                 //    "Referenced node: '" + toString(ref) + "' in relation: '" + toString(myCurrentRelation)
    //                                 //    + "' does not exist. Probably OSM file is incomplete.");
    //                                 continue;
    //                             }
    //                             NIOSMNode* pNode = myOSMNodes.find(nodeRef)->second;
    //                             Position pNodePos(pNode->lon, pNode->lat, pNode->ele);
    //                             if (!NBNetBuilder::transformCoordinate(pNodePos)) {
    //                                 WRITE_ERRORF("Unable to project coordinates for node '%'.", pNode->id);
    //                                 continue;
    //                             }
    //                             p.push_back(pNodePos);
    //                         }
    //                         if (p.size() == 0) {
    //                             WRITE_WARNINGF(TL("Referenced platform: '%' in relation: '%' is corrupt. Probably OSM file is incomplete."),
    //                                            toString(myPlatform.ref), toString(myCurrentRelation));
    //                             continue;
    //                         }
    //                         NBPTPlatform platform(p[(int)p.size() / 2], p.length());
    //                         ptStop->addPlatformCand(platform);
    //                     } else {
    //                         if (myOSMNodes.find(myPlatform.ref) == myOSMNodes.end()) {
    //                             //WRITE_WARNING(
    //                             //    "Referenced node: '" + toString(ref) + "' in relation: '" + toString(myCurrentRelation)
    //                             //    + "' does not exist. Probably OSM file is incomplete.");
    //                             continue;
    //                         }
    //                         NIOSMNode* pNode = myOSMNodes.find(myPlatform.ref)->second;
    //                         Position platformPos(pNode->lon, pNode->lat, pNode->ele);
    //                         if (!NBNetBuilder::transformCoordinate(platformPos)) {
    //                             WRITE_ERRORF("Unable to project coordinates for node '%'.", pNode->id);
    //                         }
    //                         NBPTPlatform platform(platformPos, myOptionsCont.getFloat("osm.stop-output.length"));
    //                         ptStop->addPlatformCand(platform);
    //
    //                     }
    //                 }
    //                 ptStop->setIsMultipleStopPositions(myStops.size() > 1, myCurrentRelation);
    //                 if (myStation != INVALID_ID) {
    //                     const auto& nodeIt = myOSMNodes.find(myStation);
    //                     if (nodeIt != myOSMNodes.end()) {
    //                         NIOSMNode* station = nodeIt->second;
    //                         if (station != nullptr) {
    //                             if (station->hasParameter("railway:ref")) {
    //                                 ptStop->setParameter("stationRef", station->getParameter("railway:ref"));
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         } else if (myPTRouteType != "" && myIsRoute) {
    //             NBPTLine* ptLine = new NBPTLine(toString(myCurrentRelation), myName, myPTRouteType, myRef, myInterval, myNightService,
    //                                             interpretTransportType(myPTRouteType), myRouteColor);
    //             int consecutiveGap = false;
    //             int missingBefore = 0;
    //             int missingAfter = 0;
    //             for (long long ref : myStops) {
    //                 const auto& nodeIt = myOSMNodes.find(ref);
    //                 if (nodeIt == myOSMNodes.end()) {
    //                     if (ptLine->getStops().empty()) {
    //                         missingBefore++;
    //                     } else {
    //                         missingAfter++;
    //                         consecutiveGap++;
    //                     }
    //                     continue;
    //                 }
    //                 // give some slack for single missing stops
    //                 if (consecutiveGap > 1) {
    //                     WRITE_WARNINGF(TL("PT line '%' in relation % has a gap of % stops, only keeping first part."), myName, myCurrentRelation, consecutiveGap);
    //                     missingAfter = (int)myStops.size() - missingBefore - (int)ptLine->getStops().size();
    //                     break;
    //                 }
    //                 // reset gap
    //                 consecutiveGap = 0;
    //
    //                 const NIOSMNode* const n = nodeIt->second;
    //                 std::shared_ptr<NBPTStop> ptStop = myNBPTStopCont->get(toString(n->id));
    //                 if (ptStop == nullptr) {
    //                     // loose stop, which must later be mapped onto a line way
    //                     Position ptPos(n->lon, n->lat, n->ele);
    //                     if (!NBNetBuilder::transformCoordinate(ptPos)) {
    //                         WRITE_ERRORF("Unable to project coordinates for node '%'.", n->id);
    //                     }
    //                     const SumoXMLTag stopElement = isRailway(n->permissions) ? SUMO_TAG_TRAIN_STOP : SUMO_TAG_BUS_STOP;
    //                     ptStop = std::make_shared<NBPTStop>(stopElement, toString(n->id), ptPos, "", "", n->ptStopLength, n->name, n->permissions);
    //                     myNBPTStopCont->insert(ptStop);
    //                     if (myStopAreas.count(n->id)) {
    //                         ptStop->setIsMultipleStopPositions(false, myStopAreas[n->id]);
    //                     }
    //                     if (myPlatformStops.count(n->id) > 0) {
    //                         ptStop->setIsPlatform();
    //                     }
    //                 }
    //                 ptLine->addPTStop(ptStop);
    //             }
    //             for (long long& myWay : myWays) {
    //                 auto entr = myOSMEdges.find(myWay);
    //                 if (entr != myOSMEdges.end()) {
    //                     Edge* edge = entr->second;
    //                     for (long long& myCurrentNode : edge->myCurrentNodes) {
    //                         ptLine->addWayNode(myWay, myCurrentNode);
    //                     }
    //                 }
    //             }
    //             ptLine->setNumOfStops((int)myStops.size(), missingBefore, missingAfter);
    //             if (ptLine->getStops().empty()) {
    //                 WRITE_WARNINGF(TL("PT line in relation % with no stops ignored. Probably OSM file is incomplete."), myCurrentRelation);
    //                 delete ptLine;
    //                 resetValues();
    //                 return;
    //             }
    //             if (!myNBPTLineCont->insert(ptLine)) {
    //                 WRITE_WARNINGF(TL("Ignoring duplicate PT line '%'."), myCurrentRelation);
    //                 delete ptLine;
    //             }
    //         }
    //         // other relations might use similar subelements so reset in any case
    //         resetValues();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_RELATION {
            if self.myIsRestriction {
                assert!(self.myCurrentRelation != INVALID_ID);
                let mut ok = true;
                if self.myRestrictionType == RelationHandler_RestrictionType::UNKNOWN {
                    write_warningf!("Ignoring restriction relation '%' with unknown type.", self.myCurrentRelation);
                    ok = false;
                }
                if self.myFromWay == INVALID_ID {
                    write_warningf!("Ignoring restriction relation '%' with unknown from-way.", self.myCurrentRelation);
                    ok = false;
                }
                if self.myToWay == INVALID_ID {
                    write_warningf!("Ignoring restriction relation '%' with unknown to-way.", self.myCurrentRelation);
                    ok = false;
                }
                if self.myViaNode == INVALID_ID && self.myViaWay == INVALID_ID {
                    write_warningf!("Ignoring restriction relation '%' with unknown via.", self.myCurrentRelation);
                    ok = false;
                }
                if ok && !self.apply_restriction() {
                    write_warningf!("Ignoring restriction relation '%'.", self.myCurrentRelation);
                }
            } else if self.myIsStopArea {
                for ref_ in self.myStops.clone() {
                    self.myStopAreas.insert(ref_, self.myCurrentRelation);
                    if !self.myOSMNodes.contains_key(&ref_) {
                        //WRITE_WARNING(
                        //    "Referenced node: '" + toString(ref) + "' in relation: '" + toString(myCurrentRelation)
                        //    + "' does not exist. Probably OSM file is incomplete.");
                        continue;
                    }

                    let n = self.myOSMNodes[&ref_];
                    let ptStop: Rc<NBPTStop> = match self.myNBPTStopCont.get_stops().get(&n.id.to_string()).cloned() {
                        Some(stop) => stop,
                        None => {
                            //WRITE_WARNING(
                            //    "Relation '" + toString(myCurrentRelation) + "' refers to a non existing pt stop at node: '"
                            //    + toString(n->id) + "'. Probably OSM file is incomplete.");
                            continue;
                        }
                    };
                    for myPlatform in self.myPlatforms.clone() {
                        if myPlatform.isWay {
                            // assert(myPlatformShapes.find(myPlatform.ref) != myPlatformShapes.end()); //already tested earlier
                            let edge: Ptr<NIImporter_OpenStreetMap_Edge> = self.myPlatformShapes[&myPlatform.ref_];
                            let numNodes = edge.myCurrentNodes.len();
                            if numNodes > 1 && edge.myCurrentNodes[0] == edge.myCurrentNodes[numNodes - 1] {
                                write_warningf!("Platform '%' in relation: '%' is given as polygon, which currently is not supported.", myPlatform.ref_, self.myCurrentRelation);
                                continue;
                            }
                            let mut p = PositionVector(Vec::new());
                            for nodeRef in edge.myCurrentNodes.iter() {
                                if !self.myOSMNodes.contains_key(nodeRef) {
                                    //WRITE_WARNING(
                                    //    "Referenced node: '" + toString(ref) + "' in relation: '" + toString(myCurrentRelation)
                                    //    + "' does not exist. Probably OSM file is incomplete.");
                                    continue;
                                }
                                let pNode = self.myOSMNodes[nodeRef];
                                let mut pNodePos = Position::new3(pNode.lon, pNode.lat, pNode.ele);
                                if !NBNetBuilder::transform_coordinate(&mut pNodePos, true, Ptr::null()) {
                                    write_errorf!("Unable to project coordinates for node '%'.", pNode.id);
                                    continue;
                                }
                                p.push(pNodePos);
                            }
                            if p.len() == 0 {
                                write_warningf!("Referenced platform: '%' in relation: '%' is corrupt. Probably OSM file is incomplete.", myPlatform.ref_.to_string(), self.myCurrentRelation.to_string());
                                continue;
                            }
                            let platform = NBPTPlatform::new(p[p.len() / 2], p.length());
                            pt_stop_mut(&ptStop).add_platform_cand(platform);
                        } else {
                            if !self.myOSMNodes.contains_key(&myPlatform.ref_) {
                                //WRITE_WARNING(
                                //    "Referenced node: '" + toString(ref) + "' in relation: '" + toString(myCurrentRelation)
                                //    + "' does not exist. Probably OSM file is incomplete.");
                                continue;
                            }
                            let pNode = self.myOSMNodes[&myPlatform.ref_];
                            let mut platformPos = Position::new3(pNode.lon, pNode.lat, pNode.ele);
                            if !NBNetBuilder::transform_coordinate(&mut platformPos, true, Ptr::null()) {
                                write_errorf!("Unable to project coordinates for node '%'.", pNode.id);
                            }
                            let platform = NBPTPlatform::new(platformPos, utils::options::options_cont::get_float("osm.stop-output.length"));
                            pt_stop_mut(&ptStop).add_platform_cand(platform);
                        }
                    }
                    pt_stop_mut(&ptStop).set_is_multiple_stop_positions(self.myStops.len() > 1, self.myCurrentRelation);
                    if self.myStation != INVALID_ID {
                        if let Some(station) = (*self.myOSMNodes).get(&self.myStation) {
                            if station.has_parameter("railway:ref") {
                                let stationRef = station.get_parameter("railway:ref", "");
                                pt_stop_mut(&ptStop).set_parameter("stationRef", &stationRef);
                            }
                        }
                    }
                }
            } else if self.myPTRouteType != "" && self.myIsRoute {
                let vClass = NIImporter_OpenStreetMap::interpret_transport_type(&self.myPTRouteType, Ptr::null());
                let ptLine: Ptr<NBPTLine> = Ptr::from_raw(Box::into_raw(Box::new(NBPTLine::new(
                    &self.myCurrentRelation.to_string(), &self.myName, &self.myPTRouteType, &self.myRef, self.myInterval, &self.myNightService, vClass, self.myRouteColor,
                ))));
                let mut consecutiveGap: i32 = 0;
                let mut missingBefore: i32 = 0;
                let mut missingAfter: i32 = 0;
                for ref_ in self.myStops.clone() {
                    let Some(n) = (*self.myOSMNodes).get(&ref_).copied() else {
                        if ptLine.m().get_stops().is_empty() {
                            missingBefore += 1;
                        } else {
                            missingAfter += 1;
                            consecutiveGap += 1;
                        }
                        continue;
                    };
                    // give some slack for single missing stops
                    if consecutiveGap > 1 {
                        write_warningf!("PT line '%' in relation % has a gap of % stops, only keeping first part.", self.myName, self.myCurrentRelation, consecutiveGap);
                        missingAfter = self.myStops.len() as i32 - missingBefore - ptLine.m().get_stops().len() as i32;
                        break;
                    }
                    // reset gap
                    consecutiveGap = 0;

                    let mut ptStop: Option<Rc<NBPTStop>> = self.myNBPTStopCont.get_stops().get(&n.id.to_string()).cloned();
                    if ptStop.is_none() {
                        // loose stop, which must later be mapped onto a line way
                        let mut ptPos = Position::new3(n.lon, n.lat, n.ele);
                        if !NBNetBuilder::transform_coordinate(&mut ptPos, true, Ptr::null()) {
                            write_errorf!("Unable to project coordinates for node '%'.", n.id);
                        }
                        let stopElement = if is_railway(n.permissions) { SUMO_TAG_TRAIN_STOP } else { SUMO_TAG_BUS_STOP };
                        let newStop: Rc<NBPTStop> = Rc::new(NBPTStop::new(
                            stopElement, n.id.to_string(), ptPos, String::new(), String::new(), n.ptStopLength, n.name.clone(), n.permissions,
                            0.0, RGBColor::new_valid(false), -1.0,
                        ));
                        self.myNBPTStopCont.m().insert(newStop.clone(), false);
                        if self.myStopAreas.contains_key(&n.id) {
                            pt_stop_mut(&newStop).set_is_multiple_stop_positions(false, self.myStopAreas[&n.id]);
                        }
                        if self.myPlatformStops.contains(&n.id) {
                            pt_stop_mut(&newStop).set_is_platform();
                        }
                        ptStop = Some(newStop);
                    }
                    ptLine.m().add_pt_stop(ptStop.unwrap());
                }
                for myWay in self.myWays.clone() {
                    if let Some(edge) = (*self.myOSMEdges).get(&myWay).copied() {
                        for myCurrentNode in edge.myCurrentNodes.iter() {
                            ptLine.m().add_way_node(myWay, *myCurrentNode);
                        }
                    }
                }
                ptLine.m().set_num_of_stops(self.myStops.len() as i32, missingBefore, missingAfter);
                if ptLine.m().get_stops().is_empty() {
                    write_warningf!("PT line in relation % with no stops ignored. Probably OSM file is incomplete.", self.myCurrentRelation);
                    // SAFETY: the line was allocated with Box::into_raw above and was not inserted anywhere.
                    drop(unsafe { Box::from_raw(ptLine.as_raw()) });
                    self.reset_values();
                    return;
                }
                if !self.myNBPTLineCont.m().insert(ptLine) {
                    write_warningf!("Ignoring duplicate PT line '%'.", self.myCurrentRelation);
                    // SAFETY: the line was rejected by the container, so nothing else holds it.
                    drop(unsafe { Box::from_raw(ptLine.as_raw()) });
                }
            }
            // other relations might use similar subelements so reset in any case
            self.reset_values();
        }
    }

    /// C++: `void RelationHandler::resetValues()` [private]
    /// reset members to their defaults for parsing a new relation
    // void
    // NIImporter_OpenStreetMap::RelationHandler::resetValues() {
    //     myCurrentRelation = INVALID_ID;
    //     myIsRestriction = false;
    //     myRestrictionException = SVC_IGNORING;
    //     myFromWay = INVALID_ID;
    //     myToWay = INVALID_ID;
    //     myViaNode = INVALID_ID;
    //     myViaWay = INVALID_ID;
    //     myStation = INVALID_ID;
    //     myRestrictionType = RestrictionType::UNKNOWN;
    //     myPlatforms.clear();
    //     myStops.clear();
    //     myPlatformStops.clear();
    //     myWays.clear();
    //     myIsStopArea = false;
    //     myIsRoute = false;
    //     myPTRouteType = "";
    //     myRouteColor.setValid(false);
    // }
    #[allow(unused_variables)]
    pub fn reset_values(&mut self) {
        self.myCurrentRelation = INVALID_ID;
        self.myIsRestriction = false;
        self.myRestrictionException = SVC_IGNORING;
        self.myFromWay = INVALID_ID;
        self.myToWay = INVALID_ID;
        self.myViaNode = INVALID_ID;
        self.myViaWay = INVALID_ID;
        self.myStation = INVALID_ID;
        self.myRestrictionType = RelationHandler_RestrictionType::UNKNOWN;
        self.myPlatforms.clear();
        self.myStops.clear();
        self.myPlatformStops.clear();
        self.myWays.clear();
        self.myIsStopArea = false;
        self.myIsRoute = false;
        self.myPTRouteType = String::new();
        self.myRouteColor.set_valid(false);
    }

    /// C++: `bool RelationHandler::checkEdgeRef(long long) const` [const, private]
    /// check whether a referenced way has a corresponding edge
    // bool
    // NIImporter_OpenStreetMap::RelationHandler::checkEdgeRef(long long int ref) const {
    //     if (myOSMEdges.find(ref) != myOSMEdges.end()) {
    //         return true;
    //     }
    //     WRITE_WARNINGF(TL("No way found for reference '%' in relation '%'"), toString(ref), toString(myCurrentRelation));
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_edge_ref(&self, ref_: i64) -> bool {
        if self.myOSMEdges.contains_key(&ref_) {
            return true;
        }
        write_warningf!("No way found for reference '%' in relation '%'", ref_, self.myCurrentRelation);
        false
    }

    /// C++: `bool RelationHandler::applyRestriction() const` [const, private]
    /// try to apply the parsed restriction and return whether successful
    // bool
    // NIImporter_OpenStreetMap::RelationHandler::applyRestriction() const {
    //     // since OSM ways are bidirectional we need the via to figure out which direction was meant
    //     if (myViaNode != INVALID_ID) {
    //         NBNode* viaNode = myOSMNodes.find(myViaNode)->second->node;
    //         if (viaNode == nullptr) {
    //             WRITE_WARNINGF(TL("Via-node '%' was not instantiated"), toString(myViaNode));
    //             return false;
    //         }
    //         NBEdge* from = findEdgeRef(myFromWay, viaNode->getIncomingEdges());
    //         NBEdge* to = findEdgeRef(myToWay, viaNode->getOutgoingEdges());
    //         if (from == nullptr) {
    //             WRITE_WARNINGF(TL("from-edge '%' of restriction relation could not be determined"), toString(myFromWay));
    //             return false;
    //         }
    //         if (to == nullptr) {
    //             WRITE_WARNINGF(TL("to-edge '%' of restriction relation could not be determined"), toString(myToWay));
    //             return false;
    //         }
    //         if (myRestrictionType == RestrictionType::ONLY) {
    //             from->addEdge2EdgeConnection(to, true);
    //             // make sure that these connections remain disabled even if network
    //             // modifications (ramps.guess) reset existing connections
    //             for (NBEdge* cand : from->getToNode()->getOutgoingEdges()) {
    //                 if (!from->isConnectedTo(cand)) {
    //                     if (myRestrictionException == SVC_IGNORING) {
    //                         from->removeFromConnections(cand, -1, -1, true);
    //                     } else {
    //                         from->addEdge2EdgeConnection(cand, true, myRestrictionException);
    //                     }
    //                 }
    //             }
    //         } else {
    //             if (myRestrictionException == SVC_IGNORING) {
    //                 from->removeFromConnections(to, -1, -1, true);
    //             } else {
    //                 from->addEdge2EdgeConnection(to, true, myRestrictionException);
    //                 for (NBEdge* cand : from->getToNode()->getOutgoingEdges()) {
    //                     if (!from->isConnectedTo(cand)) {
    //                         from->addEdge2EdgeConnection(cand, true);
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         // XXX interpreting via-ways or via-node lists not yet implemented
    //         WRITE_WARNINGF(TL("direction of restriction relation could not be determined%"), "");
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn apply_restriction(&self) -> bool {
        // since OSM ways are bidirectional we need the via to figure out which direction was meant
        if self.myViaNode != INVALID_ID {
            let viaNode = self.myOSMNodes[&self.myViaNode].node;
            if viaNode.is_null() {
                write_warningf!("Via-node '%' was not instantiated", self.myViaNode);
                return false;
            }
            let from = self.find_edge_ref(self.myFromWay, viaNode.get_incoming_edges());
            let to = self.find_edge_ref(self.myToWay, viaNode.get_outgoing_edges());
            if from.is_null() {
                write_warningf!("from-edge '%' of restriction relation could not be determined", self.myFromWay);
                return false;
            }
            if to.is_null() {
                write_warningf!("to-edge '%' of restriction relation could not be determined", self.myToWay);
                return false;
            }
            if self.myRestrictionType == RelationHandler_RestrictionType::ONLY {
                from.m().add_edge2_edge_connection(to, true, SVC_UNSPECIFIED);
                // make sure that these connections remain disabled even if network
                // modifications (ramps.guess) reset existing connections
                for cand in from.get_to_node().get_outgoing_edges().clone() {
                    if !from.is_connected_to(cand, false) {
                        if self.myRestrictionException == SVC_IGNORING {
                            from.m().remove_from_connections(cand, -1, -1, true, false, false);
                        } else {
                            from.m().add_edge2_edge_connection(cand, true, self.myRestrictionException);
                        }
                    }
                }
            } else {
                if self.myRestrictionException == SVC_IGNORING {
                    from.m().remove_from_connections(to, -1, -1, true, false, false);
                } else {
                    from.m().add_edge2_edge_connection(to, true, self.myRestrictionException);
                    for cand in from.get_to_node().get_outgoing_edges().clone() {
                        if !from.is_connected_to(cand, false) {
                            from.m().add_edge2_edge_connection(cand, true, SVC_UNSPECIFIED);
                        }
                    }
                }
            }
        } else {
            // XXX interpreting via-ways or via-node lists not yet implemented
            write_warningf!("direction of restriction relation could not be determined%", "");
            return false;
        }
        true
    }

    /// C++: `NBEdge * RelationHandler::findEdgeRef(long long, const std::vector<NBEdge *> &) const` [const, private]
    /// try to find the way segment among candidates
    // NBEdge*
    // NIImporter_OpenStreetMap::RelationHandler::findEdgeRef(long long int wayRef,
    //         const std::vector<NBEdge*>& candidates) const {
    //     const std::string prefix = toString(wayRef);
    //     const std::string backPrefix = "-" + prefix;
    //     NBEdge* result = nullptr;
    //     int found = 0;
    //     for (auto candidate : candidates) {
    //         if ((candidate->getID().substr(0, prefix.size()) == prefix) ||
    //                 (candidate->getID().substr(0, backPrefix.size()) == backPrefix)) {
    //             result = candidate;
    //             found++;
    //         }
    //     }
    //     if (found > 1) {
    //         WRITE_WARNINGF(TL("Ambiguous way reference '%' in restriction relation"), prefix);
    //         result = nullptr;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn find_edge_ref(&self, wayRef: i64, candidates: &Vec<Ptr<NBEdge>>) -> Ptr<NBEdge> {
        let prefix = wayRef.to_string();
        let backPrefix = format!("-{prefix}");
        let mut result: Ptr<NBEdge> = Ptr::null();
        let mut found = 0;
        for candidate in candidates.iter() {
            let id = candidate.get_id();
            if id.starts_with(&prefix) || id.starts_with(&backPrefix) {
                result = *candidate;
                found += 1;
            }
        }
        if found > 1 {
            write_warningf!("Ambiguous way reference '%' in restriction relation", prefix);
            result = Ptr::null();
        }
        result
    }

    /// C++: `void RelationHandler::RelationHandler(const RelationHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_OpenStreetMap_RelationHandler) -> Self {
        // C++ declares the copy constructor private and never defines it, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::RelationHandler copy constructor is private and undefined in C++")
    }

    /// C++: `RelationHandler & RelationHandler::operator=(const RelationHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_OpenStreetMap_RelationHandler) -> &mut NIImporter_OpenStreetMap_RelationHandler {
        // C++ declares the assignment operator private and never defines it, so no caller can reach it.
        panic!("NIImporter_OpenStreetMap::RelationHandler::operator= is private and undefined in C++")
    }

}

// The C++ handlers derive from SUMOSAXHandler; these forward the trait hooks to the inherent C++ overrides.
impl SaxHandler for NIImporter_OpenStreetMap_NodesHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_OpenStreetMap_NodesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_OpenStreetMap_NodesHandler::my_end_element(self, element);
        Ok(())
    }
}

impl SaxHandler for NIImporter_OpenStreetMap_EdgesHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_OpenStreetMap_EdgesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_OpenStreetMap_EdgesHandler::my_end_element(self, element);
        Ok(())
    }
}

impl SaxHandler for NIImporter_OpenStreetMap_RelationHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_OpenStreetMap_RelationHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_OpenStreetMap_RelationHandler::my_end_element(self, element);
        Ok(())
    }
}
