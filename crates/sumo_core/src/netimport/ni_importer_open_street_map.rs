//! Port of netimport/NIImporter_OpenStreetMap.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::util_exceptions::ProcessError;
use utils::xml::sumo_sax_reader::SUMOSAXReader;
use utils::xml::sumo_sax_handler;
use utils::xml::generic_sax_handler::{GenericSAXHandler, SaxHandler};
use utils::{write_errorf, write_messagef, write_warningf};

/// C++ static `NBNode::FORWARD` (netbuild/NBNode.cpp); NBNode statics are not ported as consts yet.
const NBNODE_FORWARD: i32 = 1;
/// C++ static `NBNode::BACKWARD` (netbuild/NBNode.cpp).
#[allow(dead_code)]
const NBNODE_BACKWARD: i32 = -1;
/// C++ static `NBEdge::TURN_SIGN_SHIFT_BUS` etc. (netbuild/NBEdge.h); NBEdge statics are not ported as consts yet.
const NBEDGE_TURN_SIGN_SHIFT_BUS: i32 = 8;
const NBEDGE_TURN_SIGN_SHIFT_TAXI: i32 = 16;
const NBEDGE_TURN_SIGN_SHIFT_BICYCLE: i32 = 24;

/// SAX reader results: a ProcessError is thrown in C++ and propagates to the caller, so panic here.
fn sax_result(r: Result<bool, ProcessError>) -> bool {
    r.unwrap_or_else(|e| panic!("{e}"))
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
    pub myCyclewayType: NIImporter_OpenStreetMap_WayType,
    /// Information about the kind of busway along this road
    pub myBuswayType: NIImporter_OpenStreetMap_WayType,
    /// Information about the kind of sidwalk along this road
    pub mySidewalkType: NIImporter_OpenStreetMap_WayType,
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
            myCyclewayType: NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN,
            myBuswayType: NIImporter_OpenStreetMap_WayType::WAY_NONE,
            mySidewalkType: NIImporter_OpenStreetMap_WayType::WAY_UNKNOWN,
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
                let Some(node) = self.myOSMNodes.get(&r) else {
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
                write_warning(&format!("Value of key '{}' is not numeric ('{}') in edge '{}'.", key, value, self.myCurrentEdge.id));
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
    pub myUniqueNodes: BTreeSet<Ptr<NIImporter_OpenStreetMap_NIOSMNode>>, // C++: std::set<NIOSMNode *, CompareNodes>
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
    /// whether ETCS signals default to moving block
    pub myETCS_movingBlock: bool,
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
            myETCS_movingBlock: false,
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
            drop(unsafe { Box::from_raw(myUniqueNode.as_raw()) });
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
        if !opt::is_set("osm-files") {
            return;
        }
        let files = opt::get_string_vector("osm-files");
        let mut readers: Vec<Option<SUMOSAXReader>> = Vec::new();

        self.myImportLaneAccess = oc.get_bool("osm.lane-access");
        self.myImportTurnSigns = oc.get_bool("osm.turn-lanes");
        self.myImportSidewalks = oc.get_bool("osm.sidewalks");
        self.myImportBikeAccess = oc.get_bool("osm.bike-access");
        self.myImportCrossings = oc.get_bool("osm.crossings");
        self.myOnewayDualSidewalk = oc.get_bool("osm.oneway-reverse-sidewalk");
        self.myAnnotateDefaults = oc.get_bool("osm.annotate-defaults");
        self.myETCS_movingBlock = oc.get_bool("osm.etcs-moving-block");
        self.myPlacementSkippedNonExplicitOneWay = 0;
        self.myPlacementSkippedAuxOppositeDirection = 0;

        myAllAttributes.with(|c| c.set(opt::get_bool("osm.all-attributes")));
        let extra = opt::get_string_vector("osm.extra-attributes");
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
        if !oc.get_bool("osm.skip-duplicates-check") {
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
            let crossingWidth = opt::get_float("default.crossing-width");
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

        let layerElevation = opt::get_float("osm.layer-elevation");
        if layerElevation > 0.0 {
            self.reconstruct_layer_elevation(layerElevation, nb);
        }

        // revise pt stops; remove stops on deleted edges (but assign them to the
        // opposite direction if possible
        let ec: Ptr<NBEdgeCont> = Ptr::from_mut(nb.get_edge_cont());
        nb.get_pt_stop_cont().cleanup_deleted(ec.m(), true);

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
                let mut ptPos = Position::new(n.lon, n.lat, n.ele);
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
    //                 node->setParameter(NBTrafficLightDefinition::OSM_SIGNAL_DIRECTION, "forward");
    //             } else if (n->myRailDirection == WAY_BACKWARD) {
    //                 node->setParameter(NBTrafficLightDefinition::OSM_SIGNAL_DIRECTION, "backward");
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
            let mut pos = Position::new(n.lon, n.lat, n.ele);
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
                    node.m().set_parameter("virtual", "true");
                }
                if self.myETCS_movingBlock && (n.railwaySignal & NIImporter_OpenStreetMap_SignalType::SIGNAL_ETCS_BLOCK as i32) != 0 {
                    node.m().set_parameter("moving-block", "true");
                }
            } else if n.tlsControlled {
                // ok, this node is a traffic light node where no other nodes
                //  participate
                // @note: The OSM-community has not settled on a schema for differentiating between fixed and actuated lights
                let type_ = TrafficLightTypes
                    .get(&utils::options::options_cont::get_string("tls.default-type"))
                    .unwrap_or(TrafficLightType::INVALID);
                let tlDef: &mut NBOwnTLDef = Box::leak(Box::new(NBOwnTLDef::new_2(&id.to_string(), node, 0, type_)));
                if !tlsc.insert(Ptr::from_mut(&mut tlDef.base), false) {
                    // actually, nothing should fail here
                    panic!("Could not allocate tls '{id}'.");
                }
            }
            if n.railwayBufferStop {
                node.m().set_parameter("buffer_stop", "true");
                node.m().set_fringe_type(FringeType::INNER);
            }
            if n.railwaySignal != NIImporter_OpenStreetMap_SignalType::SIGNAL_NONE as i32 {
                if n.myRailDirection == NIImporter_OpenStreetMap_WayType::WAY_FORWARD as i32 {
                    node.m().set_parameter(NBTrafficLightDefinition::OSM_SIGNAL_DIRECTION, "forward");
                } else if n.myRailDirection == NIImporter_OpenStreetMap_WayType::WAY_BACKWARD as i32 {
                    node.m().set_parameter(NBTrafficLightDefinition::OSM_SIGNAL_DIRECTION, "backward");
                }
            }
            let params = n.get_parameters_map().clone();
            node.m().update_parameters(&params);
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
        todo!("port NIImporter_OpenStreetMap::insertEdge")
    }

    /// C++: `void NIImporter_OpenStreetMap::reconstructLayerElevation(double, NBNetBuilder &)` [private]
    /// reconstruct elevation from layer info
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn reconstruct_layer_elevation(&mut self, layerElevation: f64, nb: &mut NBNetBuilder) {
        todo!("port NIImporter_OpenStreetMap::reconstructLayerElevation")
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
        todo!("port NIImporter_OpenStreetMap::getNeighboringNodes")
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
        todo!("port NIImporter_OpenStreetMap::usableType")
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
        todo!("port NIImporter_OpenStreetMap::extendRailwayDistances")
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
        todo!("port NIImporter_OpenStreetMap::interpretDistance")
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
        todo!("port NIImporter_OpenStreetMap::applyChangeProhibition")
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
        todo!("port NIImporter_OpenStreetMap::applyLaneUse")
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
        todo!("port NIImporter_OpenStreetMap::mergeTurnSigns")
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
        todo!("port NIImporter_OpenStreetMap::applyTurnSigns")
    }

}

/// An internal representation of an OSM-node
/// C++ class `NIImporter_OpenStreetMap::NIOSMNode` (netimport/NIImporter_OpenStreetMap.h)
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
    pub myRailDirection: NIImporter_OpenStreetMap_WayType,
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
        todo!("port NIOSMNode::NIOSMNode")
    }

    /// C++: `NIOSMNode & NIOSMNode::operator=(const NIOSMNode &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_OpenStreetMap_NIOSMNode) -> &mut NIImporter_OpenStreetMap_NIOSMNode {
        todo!("port NIOSMNode::operator=")
    }

}

/// A class which extracts OSM-nodes from a parsed OSM-file
/// C++ class `NIImporter_OpenStreetMap::NodesHandler` (netimport/NIImporter_OpenStreetMap.h)
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
    pub myUniqueNodes: Ptr<BTreeSet<Ptr<NIImporter_OpenStreetMap_NIOSMNode>>>, // C++: std::set<NIOSMNode *, CompareNodes> &
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
    pub fn new(toFill: &mut BTreeMap<i64, Ptr<NIImporter_OpenStreetMap_NIOSMNode>>, uniqueNodes: &mut BTreeSet<Ptr<NIImporter_OpenStreetMap_NIOSMNode>>, cont: &OptionsCont) -> Self {
        todo!("port NodesHandler::NodesHandler")
    }

    /// C++: `void NodesHandler::~NodesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_OpenStreetMap::NodesHandler::~NodesHandler()
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NodesHandler::~NodesHandler")
    }

    /// C++: `int NodesHandler::getDuplicateNodes() const` [const]
    // int getDuplicateNodes() const {
    //             return myDuplicateNodes;
    //         }
    #[allow(unused_variables)]
    pub fn get_duplicate_nodes(&self) -> i32 {
        todo!("port NodesHandler::getDuplicateNodes")
    }

    /// C++: `void NodesHandler::resetHierarchy()`
    // void resetHierarchy() {
    //             myHierarchyLevel = 0;
    //         }
    #[allow(unused_variables)]
    pub fn reset_hierarchy(&mut self) {
        todo!("port NodesHandler::resetHierarchy")
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
        todo!("port NodesHandler::myStartElement")
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
        todo!("port NodesHandler::myEndElement")
    }

    /// C++: `void NodesHandler::NodesHandler(const NodesHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_DlrNavteq_NodesHandler) -> Self {
        todo!("port NodesHandler::NodesHandler")
    }

    /// C++: `NodesHandler & NodesHandler::operator=(const NodesHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_DlrNavteq_NodesHandler) -> &mut NIImporter_DlrNavteq_NodesHandler {
        todo!("port NodesHandler::operator=")
    }

}

/// A class which extracts relevant relation information from a parsed OSM-file - turn restrictions
/// C++ class `NIImporter_OpenStreetMap::RelationHandler` (netimport/NIImporter_OpenStreetMap.h)
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
        todo!("port RelationHandler::RelationHandler")
    }

    /// C++: `void RelationHandler::~RelationHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_OpenStreetMap::RelationHandler::~RelationHandler()
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RelationHandler::~RelationHandler")
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
        todo!("port RelationHandler::myStartElement")
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
        todo!("port RelationHandler::myEndElement")
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
        todo!("port RelationHandler::resetValues")
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
        todo!("port RelationHandler::checkEdgeRef")
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
        todo!("port RelationHandler::applyRestriction")
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
        todo!("port RelationHandler::findEdgeRef")
    }

    /// C++: `void RelationHandler::RelationHandler(const RelationHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_OpenStreetMap_RelationHandler) -> Self {
        todo!("port RelationHandler::RelationHandler")
    }

    /// C++: `RelationHandler & RelationHandler::operator=(const RelationHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_OpenStreetMap_RelationHandler) -> &mut NIImporter_OpenStreetMap_RelationHandler {
        todo!("port RelationHandler::operator=")
    }

}

