//! Port of netimport/NIImporter_SUMO.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::NETWORK_VERSION;
use std::cell::RefCell;
use utils::common::msg_handler::{progress_begin_message, progress_time_message, write_error, write_warning};

// C++ static members NBEdge::UNSPECIFIED_* and NBConnection::InvalidTlIndex (values from NBEdge.cpp / NBConnection.cpp)
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;
const NBEDGE_UNSPECIFIED_OFFSET: f64 = 0.0;
const NBEDGE_UNSPECIFIED_FRICTION: f64 = 1.0;
const NBCONNECTION_INVALID_TL_INDEX: i32 = -1;

thread_local! {
    /// Traffic light definitions loaded from SUMO XML, keyed by the address of their NBTrafficLightDefinition base
    static LOADED_TL_DEFS: RefCell<BTreeMap<usize, Ptr<NBLoadedSUMOTLDef>>> = RefCell::new(BTreeMap::new());
}

/// C++ dynamic_cast<NBLoadedSUMOTLDef*>(def): null when the definition was not loaded from SUMO XML
fn loaded_tl_def(def: Ptr<NBTrafficLightDefinition>) -> Ptr<NBLoadedSUMOTLDef> {
    let key = def.as_raw() as usize;
    LOADED_TL_DEFS.with(|m| m.borrow().get(&key).copied().unwrap_or(Ptr::null()))
}

/// A connection description.
/// C++ class `NIImporter_SUMO::Connection` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO_Connection {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The id of the target edge
    pub toEdgeID: String,
    /// The index of the target lane
    pub toLaneIdx: i32,
    /// The id of the traffic light that controls this connection
    pub tlID: String,
    /// The index of this connection within the controlling traffic light
    pub tlLinkIndex: i32,
    pub tlLinkIndex2: i32,
    /// Information about being definitely free to drive (on-ramps)
    pub mayDefinitelyPass: bool,
    pub keepClear: bool,
    /// custom position for internal junction on this connection
    pub contPos: f64,
    /// custom foe visibility for connection
    pub visibility: f64,
    /// custom permissions for connection
    pub permissions: SVCPermissions,
    /// custom lane changing permissions for connection
    pub changeLeft: SVCPermissions,
    /// custom lane changing permissions for connection
    pub changeRight: SVCPermissions,
    /// custom speed for connection
    pub speed: f64,
    /// custom friction for connection
    pub friction: f64,
    /// custom length for connection
    pub customLength: f64,
    /// custom shape connection
    pub customShape: PositionVector,
    /// if set to true, This connection will not be TLS-controlled despite its node being controlled.
    pub uncontrolled: bool,
    /// Whether this connection is an indirect left turn
    pub indirectLeft: bool,
    /// optional edge type
    pub edgeType: String,
}

impl std::ops::Deref for NIImporter_SUMO_Connection {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_SUMO_Connection {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NIImporter_SUMO_Connection {
}

/// Describes a pedestrian crossing
/// C++ class `NIImporter_SUMO::Crossing` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO_Crossing {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    pub edgeID: String,
    pub crossingEdges: Vec<String>,
    pub width: f64,
    pub priority: bool,
    pub customShape: PositionVector,
    pub customTLIndex: i32,
    pub customTLIndex2: i32,
}

impl std::ops::Deref for NIImporter_SUMO_Crossing {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_SUMO_Crossing {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NIImporter_SUMO_Crossing {
    /// C++: `void Crossing::Crossing(const std::string &)`
    // Crossing(const std::string& _edgeID) :
    //             edgeID(_edgeID), customTLIndex(-1), customTLIndex2(-1) {}
    #[allow(unused_variables)]
    pub fn new(_edgeID: &str) -> Self {
        Self {
            base: Parameterised::new(),
            edgeID: _edgeID.to_string(),
            crossingEdges: Vec::new(),
            // C++ leaves width/priority uninitialised; zero-initialise them
            width: 0.0,
            priority: false,
            customShape: PositionVector::default(),
            customTLIndex: -1,
            customTLIndex2: -1,
        }
    }

}

/// Describes the values found in an edge's definition and this edge's lanes
/// C++ class `NIImporter_SUMO::EdgeAttrs` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO_EdgeAttrs {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// This edge's id
    pub id: String,
    /// This edge's street name
    pub streetName: String,
    /// This edge's type
    pub type_: String,
    /// This edge's routing type
    pub routingType: String,
    /// This edge's function
    pub func: SumoXMLEdgeFunc,
    /// The node this edge starts at
    pub fromNode: String,
    /// The node this edge ends at
    pub toNode: String,
    /// This edges's shape
    pub shape: PositionVector,
    /// The length of the edge if set explicitly
    pub length: f64,
    /// This edge's priority
    pub priority: i32,
    /// The maximum velocity allowed on this edge (!!!)
    pub maxSpeed: f64,
    /// This edge's lanes
    pub lanes: Vec<Ptr<NIImporter_SUMO_LaneAttrs>>,
    /// The built edge
    pub builtEdge: Ptr<NBEdge>,
    /// The lane spread function
    pub lsf: LaneSpreadFunction,
    /// This edge's vehicle specific stop offsets (used for lanes, that do not have a specified stopOffset)
    pub edgeStopOffset: StopOffset,
    /// The position at the start of this edge (kilometrage/mileage)
    pub distance: f64,
    /// the bidi edge
    pub bidi: String,
}

impl std::ops::Deref for NIImporter_SUMO_EdgeAttrs {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_SUMO_EdgeAttrs {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NIImporter_SUMO_EdgeAttrs {
}

/// Describes the values found in a junction
/// C++ class `NIImporter_SUMO::JunctionAttrs` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO_JunctionAttrs {
    pub node: Ptr<NBNode>,
    pub response: Vec<String>,
}

impl NIImporter_SUMO_JunctionAttrs {
}

/// Describes the values found in a lane's definition
/// C++ class `NIImporter_SUMO::LaneAttrs` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO_LaneAttrs {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The maximum velocity allowed on this lane
    pub maxSpeed: f64,
    /// The friction on this lane
    pub friction: f64,
    /// This lane's shape (may be custom)
    pub shape: PositionVector,
    /// This lane's connections
    pub connections: Vec<NIImporter_SUMO_Connection>,
    /// This lane's allowed vehicle classes
    pub allow: String,
    /// This lane's disallowed vehicle classes
    pub disallow: String,
    /// This lane's vehicle classes allowed to change left
    pub changeLeft: String,
    /// This lane's vehicle classes allowed to change right
    pub changeRight: String,
    /// The width of this lane
    pub width: f64,
    /// This lane's offset from the intersection
    pub endOffset: f64,
    /// This lane's vehicle specific stop offsets
    pub laneStopOffset: StopOffset,
    /// Whether this lane is an acceleration lane
    pub accelRamp: bool,
    /// This lane's opposite lane
    pub oppositeID: String,
    /// Whether this lane has a custom shape
    pub customShape: bool,
    /// the type of this lane
    pub type_: String,
}

impl std::ops::Deref for NIImporter_SUMO_LaneAttrs {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_SUMO_LaneAttrs {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NIImporter_SUMO_LaneAttrs {
}

/// Importer for networks stored in SUMO format
/// C++ class `NIImporter_SUMO` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// Loaded edge definitions
    pub myEdges: BTreeMap<String, Ptr<NIImporter_SUMO_EdgeAttrs>>,
    /// Loaded prohibitions
    pub myProhibitions: Vec<NIImporter_SUMO_Prohibition>,
    /// The network builder to fill
    pub myNetBuilder: Ptr<NBNetBuilder>,
    /// The node container to fill
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The node container to fill
    pub myTLLCont: Ptr<NBTrafficLightLogicCont>,
    /// The handler for parsing edge types and restrictions
    pub myTypesHandler: NIXMLTypesHandler,
    /// The currently parsed edge's definition (to add loaded lanes to)
    pub myCurrentEdge: Ptr<NIImporter_SUMO_EdgeAttrs>,
    /// The currently parsed junction definition to help in reconstructing crossings
    pub myCurrentJunction: NIImporter_SUMO_JunctionAttrs,
    /// The currently parsed lanes's definition (to add the shape to)
    pub myCurrentLane: Ptr<NIImporter_SUMO_LaneAttrs>,
    /// The currently parsed traffic light
    pub myCurrentTL: Ptr<NBLoadedSUMOTLDef>,
    /// The coordinate transformation which was used to build the loaded network.
    pub myLocation: Ptr<GeoConvHelper>,
    /// The pedestrian crossings found in the network
    pub myPedestrianCrossings: BTreeMap<String, Vec<NIImporter_SUMO_Crossing>>,
    /// Map from walkingArea edge IDs to custom shapes
    pub myWACustomShapes: BTreeMap<String, NIImporter_SUMO_WalkingAreaParsedCustomShape>,
    /// element to receive parameters
    pub myLastParameterised: Vec<Ptr<Parameterised>>,
    /// the loaded network version (MMVersion = std::pair<int, double>)
    pub myNetworkVersion: (i32, f64),
    /// whether the loaded network contains internal lanes
    pub myHaveSeenInternalEdge: bool,
    /// whether the loaded network was built for lefthand traffic
    pub myAmLefthand: bool,
    /// whether the written network should have a different "handedness" (LHT/RHT) than the loaded network
    pub myChangeLefthand: bool,
    /// the level of corner detail in the loaded network
    pub myCornerDetail: i32,
    /// the level of geometry detail for internal lanes in the loaded network
    pub myLinkDetail: i32,
    /// whether all lanes of an edge should have the same stop line
    pub myRectLaneCut: bool,
    /// whether walkingareas must be built
    pub myWalkingAreas: bool,
    /// whether turning speed was limited in the network
    pub myLimitTurnSpeed: f64,
    /// whether foe-relationships where checked at lane-level
    pub myCheckLaneFoesAll: bool,
    pub myCheckLaneFoesRoundabout: bool,
    /// whether some right-of-way checks at traffic light junctions should be disabled
    pub myTlsIgnoreInternalJunctionJam: bool,
    /// default spreadType defined in the network
    pub myDefaultSpreadType: String,
    /// overlap option for loaded network
    pub myGeomAvoidOverlap: bool,
    /// higherSpeed option for loaded network
    pub myJunctionsHigherSpeed: bool,
    /// custom settings for internal junction computation
    pub myInternalJunctionsVehicleWidth: f64,
    /// custom settings for junction shape computation
    pub myJunctionsMinimalShape: bool,
    pub myJunctionsEndpointShape: bool,
    /// loaded roundabout edges
    pub myRoundabouts: Vec<Vec<String>>,
    /// list of node id with rail signals (no NBTrafficLightDefinition exists)
    pub myRailSignals: BTreeSet<String>,
    /// list of parameter keys to discard
    pub myDiscardableParams: BTreeSet<String>,
}

impl std::ops::Deref for NIImporter_SUMO {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_SUMO {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl SaxHandler for NIImporter_SUMO {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_SUMO::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_SUMO::my_end_element(self, element);
        Ok(())
    }
}

impl NIImporter_SUMO {
    /// C++: `void NIImporter_SUMO::loadNetwork(OptionsCont &, NBNetBuilder &)` [static]
    /// Loads content of the optionally given SUMO file
    // void
    // NIImporter_SUMO::loadNetwork(OptionsCont& oc, NBNetBuilder& nb) {
    //     NIImporter_SUMO importer(nb);
    //     importer._loadNetwork(oc);
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &mut OptionsCont, nb: &mut NBNetBuilder) {
        let mut importer = NIImporter_SUMO::new(nb);
        importer.load_network_2(oc);
        importer.drop_();
    }

    /// C++: `NBLoadedSUMOTLDef * NIImporter_SUMO::initTrafficLightLogic(const SUMOSAXAttributes &, NBLoadedSUMOTLDef *)` [static]
    /// begins the reading of a traffic lights logic
    // NBLoadedSUMOTLDef*
    // NIImporter_SUMO::initTrafficLightLogic(const SUMOSAXAttributes& attrs, NBLoadedSUMOTLDef* currentTL) {
    //     if (currentTL) {
    //         WRITE_ERRORF(TL("Definition of tl-logic '%' was not finished."), currentTL->getID());
    //         return nullptr;
    //     }
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     SUMOTime offset = attrs.getOptOffsetReporting(SUMO_ATTR_OFFSET, id.c_str(), ok, 0);
    //     std::string programID = attrs.getOpt<std::string>(SUMO_ATTR_PROGRAMID, id.c_str(), ok, "<unknown>");
    //     std::string typeS = attrs.get<std::string>(SUMO_ATTR_TYPE, nullptr, ok);
    //     TrafficLightType type;
    //     if (SUMOXMLDefinitions::TrafficLightTypes.hasString(typeS)) {
    //         type = SUMOXMLDefinitions::TrafficLightTypes.get(typeS);
    //     } else {
    //         WRITE_ERRORF(TL("Unknown traffic light type '%' for tlLogic '%'."), typeS, id);
    //         return nullptr;
    //     }
    //     if (ok) {
    //         return new NBLoadedSUMOTLDef(id, programID, offset, type);
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_traffic_light_logic(attrs: &SUMOSAXAttributes, currentTL: Ptr<NBLoadedSUMOTLDef>) -> Ptr<NBLoadedSUMOTLDef> {
        if !currentTL.is_null() {
            write_error(&format!("Definition of tl-logic '{}' was not finished.", currentTL.get().unwrap().get_id()));
            return Ptr::null();
        }
        let mut ok = true;
        let id: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        let offset: SUMOTime = attrs.get_opt_offset_reporting(SUMO_ATTR_OFFSET, &id, &mut ok, 0, true);
        let programID: String = attrs.get_opt(SUMO_ATTR_PROGRAMID, &id, &mut ok, "<unknown>".to_string(), true);
        let typeS: String = attrs.get(SUMO_ATTR_TYPE, "", &mut ok, true);
        let type_: TrafficLightType;
        if TrafficLightTypes.has_string(&typeS) {
            type_ = TrafficLightTypes.get(&typeS).unwrap();
        } else {
            write_error(&format!("Unknown traffic light type '{}' for tlLogic '{}'.", typeS, id));
            return Ptr::null();
        }
        if ok {
            Ptr::from_raw(Box::into_raw(Box::new(NBLoadedSUMOTLDef::new(&id, &programID, offset, type_))))
        } else {
            Ptr::null()
        }
    }

    /// C++: `void NIImporter_SUMO::addPhase(const SUMOSAXAttributes &, NBLoadedSUMOTLDef *)` [static]
    /// adds a phase to the traffic lights logic currently build
    // void
    // NIImporter_SUMO::addPhase(const SUMOSAXAttributes& attrs, NBLoadedSUMOTLDef* currentTL) {
    //     if (!currentTL) {
    //         WRITE_ERROR(TL("found phase without tl-logic"));
    //         return;
    //     }
    //     const std::string& id = currentTL->getID();
    //     bool ok = true;
    //     std::string state = attrs.get<std::string>(SUMO_ATTR_STATE, id.c_str(), ok);
    //     SUMOTime duration = TIME2STEPS(attrs.get<double>(SUMO_ATTR_DURATION, id.c_str(), ok));
    //     if (duration < 0) {
    //         WRITE_ERRORF(TL("Phase duration for tl-logic '%/%' must be positive."), id, currentTL->getProgramID());
    //         return;
    //     }
    //     // if the traffic light is an actuated traffic light, try to get the minimum and maximum durations and ends
    //     std::vector<int> nextPhases = attrs.getOpt<std::vector<int> >(SUMO_ATTR_NEXT, id.c_str(), ok);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, nullptr, ok);
    //     // Specific from actuated
    //     const SUMOTime minDuration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MINDURATION, id.c_str(), ok, NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //     const SUMOTime maxDuration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MAXDURATION, id.c_str(), ok, NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //     const SUMOTime earliestEnd = attrs.getOptSUMOTimeReporting(SUMO_ATTR_EARLIEST_END, id.c_str(), ok, NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //     const SUMOTime latestEnd = attrs.getOptSUMOTimeReporting(SUMO_ATTR_LATEST_END, id.c_str(), ok, NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //     // specific von NEMA
    //     const SUMOTime vehExt = attrs.getOptSUMOTimeReporting(SUMO_ATTR_VEHICLEEXTENSION, id.c_str(), ok, NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //     const SUMOTime yellow = attrs.getOptSUMOTimeReporting(SUMO_ATTR_YELLOW, id.c_str(), ok, NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //     const SUMOTime red = attrs.getOptSUMOTimeReporting(SUMO_ATTR_RED, id.c_str(), ok, NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //     if (ok) {
    //         currentTL->addPhase(duration, state, minDuration, maxDuration, earliestEnd, latestEnd, vehExt, yellow, red, nextPhases, name);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_phase(attrs: &SUMOSAXAttributes, currentTL: Ptr<NBLoadedSUMOTLDef>) {
        if currentTL.is_null() {
            write_error("found phase without tl-logic");
            return;
        }
        let tl = currentTL.m();
        let id: String = tl.get_id().to_string();
        let mut ok = true;
        let state: String = attrs.get(SUMO_ATTR_STATE, &id, &mut ok, true);
        let duration: SUMOTime = utils::common::time::time2steps(attrs.get::<f64>(SUMO_ATTR_DURATION, &id, &mut ok, true));
        if duration < 0 {
            write_error(&format!("Phase duration for tl-logic '{}/{}' must be positive.", id, tl.get_program_id()));
            return;
        }
        // if the traffic light is an actuated traffic light, try to get the minimum and maximum durations and ends
        let nextPhases: Vec<i32> = attrs.get_opt(SUMO_ATTR_NEXT, &id, &mut ok, Vec::new(), true);
        let name: String = attrs.get_opt(SUMO_ATTR_NAME, "", &mut ok, String::new(), true);
        // Specific from actuated
        const UNSPECIFIED_DURATION: SUMOTime = -1; // NBTrafficLightDefinition::UNSPECIFIED_DURATION
        let minDuration: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MINDURATION, &id, &mut ok, UNSPECIFIED_DURATION, true);
        let maxDuration: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MAXDURATION, &id, &mut ok, UNSPECIFIED_DURATION, true);
        let earliestEnd: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_EARLIEST_END, &id, &mut ok, UNSPECIFIED_DURATION, true);
        let latestEnd: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_LATEST_END, &id, &mut ok, UNSPECIFIED_DURATION, true);
        // specific von NEMA
        let vehExt: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_VEHICLEEXTENSION, &id, &mut ok, UNSPECIFIED_DURATION, true);
        let yellow: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_YELLOW, &id, &mut ok, UNSPECIFIED_DURATION, true);
        let red: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_RED, &id, &mut ok, UNSPECIFIED_DURATION, true);
        if ok {
            tl.add_phase(duration, &state, minDuration, maxDuration, earliestEnd, latestEnd, vehExt, yellow, red, &nextPhases, &name);
        }
    }

    /// C++: `GeoConvHelper * NIImporter_SUMO::loadLocation(const SUMOSAXAttributes &, bool)` [static]
    /// Parses network location description and registers it with GeoConveHelper::setLoaded
    // GeoConvHelper*
    // NIImporter_SUMO::loadLocation(const SUMOSAXAttributes& attrs, bool setLoaded) {
    //     // @todo refactor parsing of location since its duplicated in NLHandler and PCNetProjectionLoader
    //     bool ok = true;
    //     GeoConvHelper* result = nullptr;
    //     const Position offset = attrs.get<Position>(SUMO_ATTR_NET_OFFSET, nullptr, ok);
    //     const Boundary convBoundary = attrs.get<Boundary>(SUMO_ATTR_CONV_BOUNDARY, nullptr, ok);
    //     const Boundary origBoundary = attrs.get<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, nullptr, ok);
    //     const std::string proj = attrs.get<std::string>(SUMO_ATTR_ORIG_PROJ, nullptr, ok);
    //     if (ok) {
    //         result = new GeoConvHelper(proj, offset, origBoundary, convBoundary);
    //         result->resolveAbstractProjection();
    //         if (setLoaded) {
    //             GeoConvHelper::setLoaded(*result);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn load_location(attrs: &SUMOSAXAttributes, setLoaded: bool) -> Ptr<GeoConvHelper> {
        let mut ok = true;
        let offset: Position = attrs.get(SUMO_ATTR_NET_OFFSET, "", &mut ok, true);
        let convBoundary: Boundary = attrs.get(SUMO_ATTR_CONV_BOUNDARY, "", &mut ok, true);
        let origBoundary: Boundary = attrs.get(SUMO_ATTR_ORIG_BOUNDARY, "", &mut ok, true);
        let proj: String = attrs.get(SUMO_ATTR_ORIG_PROJ, "", &mut ok, true);
        if ok {
            let mut result = GeoConvHelper::new(&proj, offset, origBoundary, convBoundary, 1.0, 0.0, false, false);
            result.resolve_abstract_projection();
            if setLoaded {
                GeoConvHelper::set_loaded(&result);
            }
            Ptr::from_raw(Box::into_raw(Box::new(result)))
        } else {
            Ptr::null()
        }
    }

    /// C++: `void NIImporter_SUMO::NIImporter_SUMO(NBNetBuilder &)` [protected]
    /// Constructor
    // NIImporter_SUMO::NIImporter_SUMO(NBNetBuilder& nb)
    //     : SUMOSAXHandler("sumo-network"),
    //       myNetBuilder(nb),
    //       myNodeCont(nb.getNodeCont()),
    //       myTLLCont(nb.getTLLogicCont()),
    //       myTypesHandler(nb.getTypeCont()),
    //       myCurrentEdge(nullptr),
    //       myCurrentLane(nullptr),
    //       myCurrentTL(nullptr),
    //       myLocation(nullptr),
    //       myNetworkVersion(0, 0),
    //       myHaveSeenInternalEdge(false),
    //       myAmLefthand(false),
    //       myChangeLefthand(false),
    //       myCornerDetail(0),
    //       myLinkDetail(-1),
    //       myRectLaneCut(false),
    //       myWalkingAreas(false),
    //       myLimitTurnSpeed(-1),
    //       myCheckLaneFoesAll(false),
    //       myCheckLaneFoesRoundabout(true),
    //       myTlsIgnoreInternalJunctionJam(false),
    //       myDefaultSpreadType(toString(LaneSpreadFunction::RIGHT)),
    //       myGeomAvoidOverlap(false),
    //       myJunctionsHigherSpeed(false),
    //       myInternalJunctionsVehicleWidth(OptionsCont::getOptions().getFloat("internal-junctions.vehicle-width")),
    //       myJunctionsMinimalShape(OptionsCont::getOptions().getBool("junctions.minimal-shape")),
    //       myJunctionsEndpointShape(OptionsCont::getOptions().getBool("junctions.endpoint-shape")) {
    // }
    #[allow(unused_variables)]
    pub fn new(nb: &mut NBNetBuilder) -> Self {
        let net_builder = Ptr::from_mut(nb);
        let node_cont = Ptr::from_mut(nb.get_node_cont());
        let tll_cont = Ptr::from_mut(nb.get_tl_logic_cont());
        let types_handler = NIXMLTypesHandler::new(nb.get_type_cont());
        Self {
            base: utils::xml::sumo_sax_handler::new("sumo-network", ""),
            myEdges: BTreeMap::new(),
            myProhibitions: Vec::new(),
            myNetBuilder: net_builder,
            myNodeCont: node_cont,
            myTLLCont: tll_cont,
            myTypesHandler: types_handler,
            myCurrentEdge: Ptr::null(),
            myCurrentJunction: NIImporter_SUMO_JunctionAttrs { node: Ptr::null(), response: Vec::new() },
            myCurrentLane: Ptr::null(),
            myCurrentTL: Ptr::null(),
            myLocation: Ptr::null(),
            myPedestrianCrossings: BTreeMap::new(),
            myWACustomShapes: BTreeMap::new(),
            myLastParameterised: Vec::new(),
                    myNetworkVersion: (0, 0.0),
            myHaveSeenInternalEdge: false,
            myAmLefthand: false,
            myChangeLefthand: false,
            myCornerDetail: 0,
            myLinkDetail: -1,
            myRectLaneCut: false,
            myWalkingAreas: false,
            myLimitTurnSpeed: -1.0,
            myCheckLaneFoesAll: false,
            myCheckLaneFoesRoundabout: true,
            myTlsIgnoreInternalJunctionJam: false,
            myDefaultSpreadType: LaneSpreadFunctions.get_string(LaneSpreadFunction::RIGHT).to_string(),
            myGeomAvoidOverlap: false,
            myJunctionsHigherSpeed: false,
            myInternalJunctionsVehicleWidth: utils::options::options_cont::get_float("internal-junctions.vehicle-width"),
            myJunctionsMinimalShape: utils::options::options_cont::get_bool("junctions.minimal-shape"),
            myJunctionsEndpointShape: utils::options::options_cont::get_bool("junctions.endpoint-shape"),
            myRoundabouts: Vec::new(),
            myRailSignals: BTreeSet::new(),
            myDiscardableParams: BTreeSet::new(),
        }
    }

    /// C++: `void NIImporter_SUMO::~NIImporter_SUMO() noexcept` [virtual, protected]
    /// Destructor
    // NIImporter_SUMO::~NIImporter_SUMO() {
    //     for (std::map<std::string, EdgeAttrs*>::const_iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         EdgeAttrs* ed = (*i).second;
    //         for (std::vector<LaneAttrs*>::const_iterator j = ed->lanes.begin(); j != ed->lanes.end(); ++j) {
    //             delete *j;
    //         }
    //         delete ed;
    //     }
    //     delete myLocation;
    //     if (myCurrentEdge != nullptr) {
    //         for (LaneAttrs* const lane : myCurrentEdge->lanes) {
    //             delete lane;
    //         }
    //         delete myCurrentEdge;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // Frees everything the C++ destructor frees. Every pointer here was allocated with Box::into_raw.
        unsafe {
            for ed in self.myEdges.values() {
                let ed = *ed;
                for lane in ed.m().lanes.iter() {
                    drop(Box::from_raw(lane.as_raw()));
                }
                drop(Box::from_raw(ed.as_raw()));
            }
            if !self.myLocation.is_null() {
                drop(Box::from_raw(self.myLocation.as_raw()));
            }
            if !self.myCurrentEdge.is_null() {
                let cur = self.myCurrentEdge;
                for lane in cur.m().lanes.iter() {
                    drop(Box::from_raw(lane.as_raw()));
                }
                drop(Box::from_raw(cur.as_raw()));
            }
        }
    }

    /// C++: `void NIImporter_SUMO::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_SUMO::myStartElement(int element,
    //                                 const SUMOSAXAttributes& attrs) {
    //     /* our goal is to reproduce the input net faithfully
    //      * there are different types of objects in the netfile:
    //      * 1) those which must be loaded into NBNetBuilder-Containers for processing
    //      * 2) those which can be ignored because they are recomputed based on group 1
    //      * 3) those which are of no concern to NBNetBuilder but should be exposed to
    //      *      netedit. We will probably have to patch NBNetBuilder to contain them
    //      *      and hand them over to netedit
    //      *    alternative idea: those shouldn't really be contained within the
    //      *    network but rather in separate files. teach netedit how to open those
    //      *    (POI?)
    //      * 4) those which are of concern neither to NBNetBuilder nor netedit and
    //      *    must be copied over - need to patch NBNetBuilder for this.
    //      *    copy unknown by default
    //      */
    //     switch (element) {
    //         case SUMO_TAG_NET: {
    //             bool ok;
    //             myNetworkVersion = StringUtils::toVersion(attrs.get<std::string>(SUMO_ATTR_VERSION, nullptr, ok, false));
    //             myAmLefthand = attrs.getOpt<bool>(SUMO_ATTR_LEFTHAND, nullptr, ok, false);
    //             myCornerDetail = attrs.getOpt<int>(SUMO_ATTR_CORNERDETAIL, nullptr, ok, 0);
    //             myLinkDetail = attrs.getOpt<int>(SUMO_ATTR_LINKDETAIL, nullptr, ok, -1);
    //             myRectLaneCut = attrs.getOpt<bool>(SUMO_ATTR_RECTANGULAR_LANE_CUT, nullptr, ok, false);
    //             myWalkingAreas = attrs.getOpt<bool>(SUMO_ATTR_WALKINGAREAS, nullptr, ok, false);
    //             myLimitTurnSpeed = attrs.getOpt<double>(SUMO_ATTR_LIMIT_TURN_SPEED, nullptr, ok, -1);
    //             myCheckLaneFoesAll = attrs.getOpt<bool>(SUMO_ATTR_CHECKLANEFOES_ALL, nullptr, ok, false);
    //             myCheckLaneFoesRoundabout = attrs.getOpt<bool>(SUMO_ATTR_CHECKLANEFOES_ROUNDABOUT, nullptr, ok, true);
    //             myTlsIgnoreInternalJunctionJam = attrs.getOpt<bool>(SUMO_ATTR_TLS_IGNORE_INTERNAL_JUNCTION_JAM, nullptr, ok, false);
    //             myDefaultSpreadType = attrs.getOpt<std::string>(SUMO_ATTR_SPREADTYPE, nullptr, ok, myDefaultSpreadType);
    //             myGeomAvoidOverlap = attrs.getOpt<bool>(SUMO_ATTR_AVOID_OVERLAP, nullptr, ok, myGeomAvoidOverlap);
    //             myJunctionsHigherSpeed = attrs.getOpt<bool>(SUMO_ATTR_HIGHER_SPEED, nullptr, ok, myJunctionsHigherSpeed);
    //             myInternalJunctionsVehicleWidth = attrs.getOpt<double>(SUMO_ATTR_INTERNAL_JUNCTIONS_VEHICLE_WIDTH, nullptr, ok, myInternalJunctionsVehicleWidth);
    //             myJunctionsMinimalShape = attrs.getOpt<bool>(SUMO_ATTR_JUNCTIONS_MINIMAL_SHAPE, nullptr, ok, myJunctionsMinimalShape);
    //             myJunctionsEndpointShape = attrs.getOpt<bool>(SUMO_ATTR_JUNCTIONS_ENDPOINT_SHAPE, nullptr, ok, myJunctionsEndpointShape);
    //             // derived
    //             const OptionsCont& oc = OptionsCont::getOptions();
    //             myChangeLefthand = !oc.isDefault("lefthand") && (oc.getBool("lefthand") != myAmLefthand);
    //
    //             break;
    //         }
    //         case SUMO_TAG_EDGE:
    //             addEdge(attrs);
    //             break;
    //         case SUMO_TAG_LANE:
    //             addLane(attrs);
    //             break;
    //         case SUMO_TAG_STOPOFFSET: {
    //             bool ok = true;
    //             addStopOffsets(attrs, ok);
    //         }
    //         break;
    //         case SUMO_TAG_NEIGH:
    //             myCurrentLane->oppositeID = attrs.getString(SUMO_ATTR_LANE);
    //             break;
    //         case SUMO_TAG_JUNCTION:
    //             addJunction(attrs);
    //             break;
    //         case SUMO_TAG_REQUEST:
    //             addRequest(attrs);
    //             break;
    //         case SUMO_TAG_CONNECTION:
    //             addConnection(attrs);
    //             break;
    //         case SUMO_TAG_TLLOGIC:
    //             myCurrentTL = initTrafficLightLogic(attrs, myCurrentTL);
    //             if (myCurrentTL) {
    //                 myLastParameterised.push_back(myCurrentTL);
    //             }
    //             break;
    //         case SUMO_TAG_PHASE:
    //             addPhase(attrs, myCurrentTL);
    //             break;
    //         case SUMO_TAG_LOCATION:
    //             delete myLocation;
    //             myLocation = loadLocation(attrs);
    //             break;
    //         case SUMO_TAG_PROHIBITION:
    //             addProhibition(attrs);
    //             break;
    //         case SUMO_TAG_ROUNDABOUT:
    //             addRoundabout(attrs);
    //             break;
    //         case SUMO_TAG_PARAM:
    //             if (myLastParameterised.size() != 0) {
    //                 bool ok = true;
    //                 const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //                 if (myDiscardableParams.count(key) == 0) {
    //                     // circumventing empty string test
    //                     const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //                     myLastParameterised.back()->setParameter(key, val);
    //                 }
    //             }
    //             break;
    //         default:
    //             myTypesHandler.myStartElement(element, attrs);
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        match element {
            SUMO_TAG_NET => {
                let mut ok = true;
                let version: String = attrs.get(SUMO_ATTR_VERSION, "", &mut ok, false);
                match utils::common::string_utils::to_version(&version) {
                    Ok(v) => self.myNetworkVersion = v,
                    Err(e) => write_error(&e.to_string()),
                }
                self.myAmLefthand = attrs.get_opt(SUMO_ATTR_LEFTHAND, "", &mut ok, false, true);
                self.myCornerDetail = attrs.get_opt(SUMO_ATTR_CORNERDETAIL, "", &mut ok, 0, true);
                self.myLinkDetail = attrs.get_opt(SUMO_ATTR_LINKDETAIL, "", &mut ok, -1, true);
                self.myRectLaneCut = attrs.get_opt(SUMO_ATTR_RECTANGULAR_LANE_CUT, "", &mut ok, false, true);
                self.myWalkingAreas = attrs.get_opt(SUMO_ATTR_WALKINGAREAS, "", &mut ok, false, true);
                self.myLimitTurnSpeed = attrs.get_opt(SUMO_ATTR_LIMIT_TURN_SPEED, "", &mut ok, -1.0, true);
                self.myCheckLaneFoesAll = attrs.get_opt(SUMO_ATTR_CHECKLANEFOES_ALL, "", &mut ok, false, true);
                self.myCheckLaneFoesRoundabout = attrs.get_opt(SUMO_ATTR_CHECKLANEFOES_ROUNDABOUT, "", &mut ok, true, true);
                self.myTlsIgnoreInternalJunctionJam = attrs.get_opt(SUMO_ATTR_TLS_IGNORE_INTERNAL_JUNCTION_JAM, "", &mut ok, false, true);
                self.myDefaultSpreadType = attrs.get_opt(SUMO_ATTR_SPREADTYPE, "", &mut ok, self.myDefaultSpreadType.clone(), true);
                self.myGeomAvoidOverlap = attrs.get_opt(SUMO_ATTR_AVOID_OVERLAP, "", &mut ok, self.myGeomAvoidOverlap, true);
                self.myJunctionsHigherSpeed = attrs.get_opt(SUMO_ATTR_HIGHER_SPEED, "", &mut ok, self.myJunctionsHigherSpeed, true);
                self.myInternalJunctionsVehicleWidth = attrs.get_opt(SUMO_ATTR_INTERNAL_JUNCTIONS_VEHICLE_WIDTH, "", &mut ok, self.myInternalJunctionsVehicleWidth, true);
                self.myJunctionsMinimalShape = attrs.get_opt(SUMO_ATTR_JUNCTIONS_MINIMAL_SHAPE, "", &mut ok, self.myJunctionsMinimalShape, true);
                self.myJunctionsEndpointShape = attrs.get_opt(SUMO_ATTR_JUNCTIONS_ENDPOINT_SHAPE, "", &mut ok, self.myJunctionsEndpointShape, true);
                // derived
                self.myChangeLefthand = !utils::options::options_cont::is_default("lefthand") && (utils::options::options_cont::get_bool("lefthand") != self.myAmLefthand);
            }
            SUMO_TAG_EDGE => self.add_edge(attrs),
            SUMO_TAG_LANE => self.add_lane(attrs),
            SUMO_TAG_STOPOFFSET => {
                let mut ok = true;
                self.add_stop_offsets(attrs, &mut ok);
            }
            SUMO_TAG_NEIGH => {
                self.myCurrentLane.m().oppositeID = attrs.get_string(SUMO_ATTR_LANE);
            }
            SUMO_TAG_JUNCTION => self.add_junction(attrs),
            SUMO_TAG_REQUEST => self.add_request(attrs),
            SUMO_TAG_CONNECTION => self.add_connection(attrs),
            SUMO_TAG_TLLOGIC => {
                self.myCurrentTL = Self::init_traffic_light_logic(attrs, self.myCurrentTL);
                if !self.myCurrentTL.is_null() {
                    self.myLastParameterised.push(Ptr::from_mut(&mut self.myCurrentTL.m().base2));
                }
            }
            SUMO_TAG_PHASE => Self::add_phase(attrs, self.myCurrentTL),
            SUMO_TAG_LOCATION => {
                if !self.myLocation.is_null() {
                    unsafe { drop(Box::from_raw(self.myLocation.as_raw())) };
                }
                self.myLocation = Self::load_location(attrs, true);
            }
            SUMO_TAG_PROHIBITION => self.add_prohibition(attrs),
            SUMO_TAG_ROUNDABOUT => self.add_roundabout(attrs),
            SUMO_TAG_PARAM => {
                if !self.myLastParameterised.is_empty() {
                    let mut ok = true;
                    let key: String = attrs.get(SUMO_ATTR_KEY, "", &mut ok, true);
                    if !self.myDiscardableParams.contains(&key) {
                        // circumventing empty string test
                        let val: String = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
                        self.myLastParameterised.last().unwrap().m().set_parameter(&key, &val);
                    }
                }
            }
            _ => {
                self.myTypesHandler.my_start_element(element, attrs);
            }
        }
    }

    /// C++: `void NIImporter_SUMO::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIImporter_SUMO::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_EDGE:
    //             if (myCurrentEdge != nullptr) {
    //                 if (myEdges.find(myCurrentEdge->id) != myEdges.end()) {
    //                     WRITE_WARNINGF(TL("Edge '%' occurred at least twice in the input."), myCurrentEdge->id);
    //                     for (LaneAttrs* const lane : myCurrentEdge->lanes) {
    //                         delete lane;
    //                     }
    //                     delete myCurrentEdge;
    //                 } else {
    //                     myEdges[myCurrentEdge->id] = myCurrentEdge;
    //                 }
    //                 myCurrentEdge = nullptr;
    //                 myLastParameterised.pop_back();
    //             }
    //             break;
    //         case SUMO_TAG_LANE:
    //             if (myCurrentEdge != nullptr && myCurrentLane != nullptr) {
    //                 myCurrentEdge->maxSpeed = MAX2(myCurrentEdge->maxSpeed, myCurrentLane->maxSpeed);
    //                 myCurrentEdge->lanes.push_back(myCurrentLane);
    //                 myLastParameterised.pop_back();
    //             }
    //             myCurrentLane = nullptr;
    //             break;
    //         case SUMO_TAG_TLLOGIC:
    //             if (myCurrentTL == nullptr) {
    //                 WRITE_ERROR(TL("Unmatched closing tag for tl-logic."));
    //             } else {
    //                 if (!myTLLCont.insert(myCurrentTL)) {
    //                     WRITE_WARNINGF(TL("Could not add program '%' for traffic light '%'"), myCurrentTL->getProgramID(), myCurrentTL->getID());
    //                     delete myCurrentTL;
    //                 }
    //                 myCurrentTL = nullptr;
    //                 myLastParameterised.pop_back();
    //             }
    //             break;
    //         case SUMO_TAG_JUNCTION:
    //             if (myCurrentJunction.node != nullptr) {
    //                 myLastParameterised.pop_back();
    //             }
    //             break;
    //         case SUMO_TAG_CONNECTION:
    //             // !!! this just avoids a crash but is not a real check that it was a connection
    //             if (!myLastParameterised.empty()) {
    //                 myLastParameterised.pop_back();
    //             }
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            SUMO_TAG_EDGE => {
                if !self.myCurrentEdge.is_null() {
                    let cur = self.myCurrentEdge;
                    if self.myEdges.contains_key(&cur.m().id) {
                        write_warning(&format!("Edge '{}' occurred at least twice in the input.", cur.m().id));
                        unsafe {
                            for lane in cur.m().lanes.iter() {
                                drop(Box::from_raw(lane.as_raw()));
                            }
                            drop(Box::from_raw(cur.as_raw()));
                        }
                    } else {
                        self.myEdges.insert(cur.m().id.clone(), cur);
                    }
                    self.myCurrentEdge = Ptr::null();
                    self.myLastParameterised.pop();
                }
            }
            SUMO_TAG_LANE => {
                if !self.myCurrentEdge.is_null() && !self.myCurrentLane.is_null() {
                    let edge = self.myCurrentEdge.m();
                    let laneMaxSpeed = self.myCurrentLane.m().maxSpeed;
                    // MAX2(a, b)
                    edge.maxSpeed = if edge.maxSpeed > laneMaxSpeed { edge.maxSpeed } else { laneMaxSpeed };
                    edge.lanes.push(self.myCurrentLane);
                    self.myLastParameterised.pop();
                }
                self.myCurrentLane = Ptr::null();
            }
            SUMO_TAG_TLLOGIC => {
                if self.myCurrentTL.is_null() {
                    write_error("Unmatched closing tag for tl-logic.");
                } else {
                    let tl = self.myCurrentTL;
                    if !self.myTLLCont.m().insert(Ptr::from_mut(&mut tl.m().base), false) {
                        write_warning(&format!("Could not add program '{}' for traffic light '{}'", tl.m().get_program_id(), tl.m().get_id()));
                        unsafe { drop(Box::from_raw(tl.as_raw())) };
                    } else {
                        let key = Ptr::from_mut(&mut tl.m().base).as_raw() as usize;
                        LOADED_TL_DEFS.with(|m| m.borrow_mut().insert(key, tl));
                    }
                    self.myCurrentTL = Ptr::null();
                    self.myLastParameterised.pop();
                }
            }
            SUMO_TAG_JUNCTION => {
                if !self.myCurrentJunction.node.is_null() {
                    self.myLastParameterised.pop();
                }
            }
            SUMO_TAG_CONNECTION => {
                // !!! this just avoids a crash but is not a real check that it was a connection
                if !self.myLastParameterised.is_empty() {
                    self.myLastParameterised.pop();
                }
            }
            _ => {}
        }
    }

    /// C++: `void NIImporter_SUMO::_loadNetwork(OptionsCont &)` [private]
    /// load the network
    // void
    // NIImporter_SUMO::_loadNetwork(OptionsCont& oc) {
    //     // check whether the option is set (properly)
    //     if (!oc.isUsableFileList("sumo-net-file")) {
    //         return;
    //     }
    //     const std::vector<std::string> discardableParams = oc.getStringVector("discard-params");
    //     myDiscardableParams.insert(discardableParams.begin(), discardableParams.end());
    //     // parse file(s)
    //     const std::vector<std::string> files = oc.getStringVector("sumo-net-file");
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         if (!FileHelpers::isReadable(*file)) {
    //             WRITE_ERRORF(TL("Could not open sumo-net-file '%'."), *file);
    //             return;
    //         }
    //         setFileName(*file);
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing sumo-net from '" + *file + "'");
    //         XMLSubSys::runParser(*this, *file, true);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     // build edges
    //     const double maxSegmentLength = oc.getFloat("geometry.max-segment-length");
    //     for (std::map<std::string, EdgeAttrs*>::const_iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         EdgeAttrs* ed = (*i).second;
    //         // skip internal edges
    //         if (ed->func == SumoXMLEdgeFunc::INTERNAL || ed->func == SumoXMLEdgeFunc::CROSSING || ed->func == SumoXMLEdgeFunc::WALKINGAREA) {
    //             continue;
    //         }
    //         // get and check the nodes
    //         NBNode* from = myNodeCont.retrieve(ed->fromNode);
    //         NBNode* to = myNodeCont.retrieve(ed->toNode);
    //         if (from == nullptr) {
    //             WRITE_ERRORF(TL("Edge's '%' from-node '%' is not known."), ed->id, ed->fromNode);
    //             continue;
    //         }
    //         if (to == nullptr) {
    //             WRITE_ERRORF(TL("Edge's '%' to-node '%' is not known."), ed->id, ed->toNode);
    //             continue;
    //         }
    //         if (from == to) {
    //             WRITE_ERRORF(TL("Edge's '%' from-node and to-node '%' are identical."), ed->id, ed->toNode);
    //             continue;
    //         }
    //         if (ed->shape.size() == 0 && maxSegmentLength > 0) {
    //             ed->shape.push_back(from->getPosition());
    //             ed->shape.push_back(to->getPosition());
    //             // shape is already cartesian but we must use a copy because the original will be modified
    //             NBNetBuilder::addGeometrySegments(ed->shape, PositionVector(ed->shape), maxSegmentLength);
    //         }
    //         // build and insert the edge
    //         NBEdge* e = new NBEdge(ed->id, from, to,
    //                                ed->type, ed->maxSpeed, NBEdge::UNSPECIFIED_FRICTION,
    //                                (int) ed->lanes.size(),
    //                                ed->priority, NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET,
    //                                ed->shape, ed->lsf, ed->streetName, "", true); // always use tryIgnoreNodePositions to keep original shape
    //         e->setLoadedLength(ed->length);
    //         e->updateParameters(ed->getParametersMap());
    //         e->setDistance(ed->distance);
    //         if (!myNetBuilder.getEdgeCont().insert(e)) {
    //             WRITE_ERRORF(TL("Could not insert edge '%'."), ed->id);
    //             delete e;
    //             continue;
    //         }
    //         ed->builtEdge = myNetBuilder.getEdgeCont().retrieve(ed->id);
    //         if (ed->builtEdge != nullptr) {
    //             ed->builtEdge->setEdgeStopOffset(-1, ed->edgeStopOffset);
    //             ed->builtEdge->setBidi(ed->bidi != "");
    //             ed->builtEdge->setRoutingType(ed->routingType);
    //         }
    //     }
    //     // assign further lane attributes (edges are built)
    //     EdgeVector toRemove;
    //     const bool dismissVclasses = oc.getBool("dismiss-vclasses");
    //     for (std::map<std::string, EdgeAttrs*>::const_iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         EdgeAttrs* ed = (*i).second;
    //         NBEdge* nbe = ed->builtEdge;
    //         if (nbe == nullptr) { // inner edge or removed by explicit list, vclass, ...
    //             continue;
    //         }
    //         const SumoXMLNodeType toType = nbe->getToNode()->getType();
    //         for (int fromLaneIndex = 0; fromLaneIndex < (int) ed->lanes.size(); ++fromLaneIndex) {
    //             LaneAttrs* lane = ed->lanes[fromLaneIndex];
    //             // connections
    //             const std::vector<Connection>& connections = lane->connections;
    //             for (const Connection& c : connections) {
    //                 if (myEdges.count(c.toEdgeID) == 0) {
    //                     WRITE_ERRORF(TL("Unknown edge '%' given in connection."), c.toEdgeID);
    //                     continue;
    //                 }
    //                 NBEdge* toEdge = myEdges[c.toEdgeID]->builtEdge;
    //                 if (toEdge == nullptr) { // removed by explicit list, vclass, ...
    //                     continue;
    //                 }
    //                 if (toEdge->getFromNode() != nbe->getToNode()) { // inconsistency may occur when merging networks
    //                     WRITE_WARNINGF("Removing invalid connection from edge '%' to edge '%'", nbe->getID(), toEdge->getID());
    //                     continue;
    //                 }
    //                 // patch attribute uncontrolled for legacy networks where it is not set explicitly
    //                 bool uncontrolled = c.uncontrolled;
    //
    //                 if ((NBNode::isTrafficLight(toType) || toType == SumoXMLNodeType::RAIL_SIGNAL)
    //                         && c.tlLinkIndex == NBConnection::InvalidTlIndex) {
    //                     uncontrolled = true;
    //                 }
    //                 nbe->addLane2LaneConnection(
    //                     fromLaneIndex, toEdge, c.toLaneIdx, NBEdge::Lane2LaneInfoType::VALIDATED,
    //                     true, c.mayDefinitelyPass, c.keepClear ? KEEPCLEAR_TRUE : KEEPCLEAR_FALSE,
    //                     c.contPos, c.visibility, c.speed, c.friction, c.customLength, c.customShape, uncontrolled, c.permissions, c.indirectLeft, c.edgeType, c.changeLeft, c.changeRight);
    //                 if (c.getParametersMap().size() > 0) {
    //                     nbe->getConnectionRef(fromLaneIndex, toEdge, c.toLaneIdx).updateParameters(c.getParametersMap());
    //                 }
    //                 // maybe we have a tls-controlled connection
    //                 if (c.tlID != "" && myRailSignals.count(c.tlID) == 0) {
    //                     const std::map<std::string, NBTrafficLightDefinition*>& programs = myTLLCont.getPrograms(c.tlID);
    //                     if (programs.size() > 0) {
    //                         std::map<std::string, NBTrafficLightDefinition*>::const_iterator it;
    //                         for (it = programs.begin(); it != programs.end(); it++) {
    //                             NBLoadedSUMOTLDef* tlDef = dynamic_cast<NBLoadedSUMOTLDef*>(it->second);
    //                             if (tlDef) {
    //                                 tlDef->addConnection(nbe, toEdge, fromLaneIndex, c.toLaneIdx, c.tlLinkIndex, c.tlLinkIndex2, false);
    //                             } else {
    //                                 throw ProcessError("Corrupt traffic light definition '" + c.tlID + "' (program '" + it->first + "')");
    //                             }
    //                         }
    //                     } else {
    //                         WRITE_ERRORF(TL("The traffic light '%' is not known."), c.tlID);
    //                     }
    //                 }
    //             }
    //             // allow/disallow XXX preferred
    //             if (!dismissVclasses) {
    //                 nbe->setPermissions(parseVehicleClasses(lane->allow, lane->disallow, myNetworkVersion), fromLaneIndex);
    //             }
    //             nbe->setPermittedChanging(fromLaneIndex, parseVehicleClasses(lane->changeLeft, ""), parseVehicleClasses(lane->changeRight, ""));
    //             // width, offset
    //             nbe->setLaneWidth(fromLaneIndex, lane->width);
    //             nbe->setEndOffset(fromLaneIndex, lane->endOffset);
    //             nbe->setSpeed(fromLaneIndex, lane->maxSpeed);
    //             nbe->setFriction(fromLaneIndex, lane->friction);
    //             nbe->setAcceleration(fromLaneIndex, lane->accelRamp);
    //             nbe->getLaneStruct(fromLaneIndex).oppositeID = lane->oppositeID;
    //             nbe->getLaneStruct(fromLaneIndex).type = lane->type;
    //             nbe->getLaneStruct(fromLaneIndex).updateParameters(lane->getParametersMap());
    //             if (lane->customShape) {
    //                 nbe->setLaneShape(fromLaneIndex, lane->shape);
    //             }
    //             // stop offset for lane
    //             bool stopOffsetSet = false;
    //             if (lane->laneStopOffset.isDefined() || nbe->getEdgeStopOffset().isDefined()) {
    //                 // apply lane-specific stopOffset (might be none as well)
    //                 stopOffsetSet = nbe->setEdgeStopOffset(fromLaneIndex, lane->laneStopOffset);
    //             }
    //             if (!stopOffsetSet) {
    //                 // apply default stop offset to lane
    //                 nbe->setEdgeStopOffset(fromLaneIndex, nbe->getEdgeStopOffset());
    //             }
    //         }
    //         nbe->declareConnectionsAsLoaded();
    //         if (!nbe->hasLaneSpecificWidth() && nbe->getLanes()[0].width != NBEdge::UNSPECIFIED_WIDTH) {
    //             nbe->setLaneWidth(-1, nbe->getLaneWidth(0));
    //         }
    //         if (!nbe->hasLaneSpecificEndOffset() && nbe->getEndOffset(0) != NBEdge::UNSPECIFIED_OFFSET) {
    //             nbe->setEndOffset(-1, nbe->getEndOffset(0));
    //         }
    //         if (!nbe->hasLaneSpecificStopOffsets() && nbe->getEdgeStopOffset().isDefined()) {
    //             nbe->setEdgeStopOffset(-1, nbe->getEdgeStopOffset());
    //         }
    //         // check again after permissions are set
    //         if (myNetBuilder.getEdgeCont().ignoreFilterMatch(nbe)) {
    //             myNetBuilder.getEdgeCont().ignore(nbe->getID());
    //             toRemove.push_back(nbe);
    //         }
    //     }
    //     for (EdgeVector::iterator i = toRemove.begin(); i != toRemove.end(); ++i) {
    //         myNetBuilder.getEdgeCont().erase(myNetBuilder.getDistrictCont(), *i);
    //     }
    //     // insert loaded prohibitions
    //     for (std::vector<Prohibition>::const_iterator it = myProhibitions.begin(); it != myProhibitions.end(); it++) {
    //         NBEdge* prohibitedFrom = myEdges[it->prohibitedFrom]->builtEdge;
    //         NBEdge* prohibitedTo = myEdges[it->prohibitedTo]->builtEdge;
    //         NBEdge* prohibitorFrom = myEdges[it->prohibitorFrom]->builtEdge;
    //         NBEdge* prohibitorTo = myEdges[it->prohibitorTo]->builtEdge;
    //         if (prohibitedFrom == nullptr) {
    //             WRITE_WARNINGF(TL("Edge '%' in prohibition was not built."), it->prohibitedFrom);
    //         } else if (prohibitedTo == nullptr) {
    //             WRITE_WARNINGF(TL("Edge '%' in prohibition was not built."), it->prohibitedTo);
    //         } else if (prohibitorFrom == nullptr) {
    //             WRITE_WARNINGF(TL("Edge '%' in prohibition was not built."), it->prohibitorFrom);
    //         } else if (prohibitorTo == nullptr) {
    //             WRITE_WARNINGF(TL("Edge '%' in prohibition was not built."), it->prohibitorTo);
    //         } else {
    //             NBNode* n = prohibitedFrom->getToNode();
    //             n->addSortedLinkFoes(
    //                 NBConnection(prohibitorFrom, prohibitorTo),
    //                 NBConnection(prohibitedFrom, prohibitedTo));
    //         }
    //     }
    //     if (oc.isWriteable("no-internal-links") && oc.isDefault("no-internal-links") && !myHaveSeenInternalEdge) {
    //         oc.set("no-internal-links", "true");
    //     }
    //     if (oc.isWriteable("lefthand") && oc.isDefault("lefthand") && myAmLefthand) {
    //         oc.set("lefthand", toString(myAmLefthand));
    //     }
    //     if (oc.isWriteable("junctions.corner-detail") && oc.isDefault("junctions.corner-detail") && oc.getInt("junctions.corner-detail") != myCornerDetail) {
    //         oc.set("junctions.corner-detail", toString(myCornerDetail));
    //     }
    //     if (oc.isWriteable("junctions.internal-link-detail") && oc.isDefault("junctions.internal-link-detail") && oc.getInt("junctions.internal-link-detail") != myLinkDetail && myLinkDetail > 0) {
    //         oc.set("junctions.internal-link-detail", toString(myLinkDetail));
    //     }
    //     if (oc.isWriteable("rectangular-lane-cut") && oc.isDefault("rectangular-lane-cut") && oc.getBool("rectangular-lane-cut") != myRectLaneCut) {
    //         oc.set("rectangular-lane-cut", toString(myRectLaneCut));
    //     }
    //     if (oc.isWriteable("walkingareas") && oc.isDefault("walkingareas") && oc.getBool("walkingareas") != myWalkingAreas) {
    //         oc.set("walkingareas", toString(myWalkingAreas));
    //     }
    //     if (oc.isWriteable("junctions.limit-turn-speed") && oc.isDefault("junctions.limit-turn-speed") && oc.getFloat("junctions.limit-turn-speed") != myLimitTurnSpeed) {
    //         oc.set("junctions.limit-turn-speed", toString(myLimitTurnSpeed));
    //     }
    //     if (oc.isWriteable("check-lane-foes.all") && oc.isDefault("check-lane-foes.all") && oc.getBool("check-lane-foes.all") != myCheckLaneFoesAll) {
    //         oc.set("check-lane-foes.all", toString(myCheckLaneFoesAll));
    //     }
    //     if (oc.isWriteable("check-lane-foes.roundabout") && oc.isDefault("check-lane-foes.roundabout") && oc.getBool("check-lane-foes.roundabout") != myCheckLaneFoesRoundabout) {
    //         oc.set("check-lane-foes.roundabout", toString(myCheckLaneFoesRoundabout));
    //     }
    //     if (oc.isWriteable("tls.ignore-internal-junction-jam") && oc.isDefault("tls.ignore-internal-junction-jam") && oc.getBool("tls.ignore-internal-junction-jam") != myTlsIgnoreInternalJunctionJam) {
    //         oc.set("tls.ignore-internal-junction-jam", toString(myTlsIgnoreInternalJunctionJam));
    //     }
    //     if (oc.isWriteable("default.spreadtype") && oc.isDefault("default.spreadtype") && oc.getString("default.spreadtype") != myDefaultSpreadType) {
    //         oc.set("default.spreadtype", myDefaultSpreadType);
    //     }
    //     if (oc.isWriteable("geometry.avoid-overlap") && oc.isDefault("geometry.avoid-overlap") && oc.getBool("geometry.avoid-overlap") != myGeomAvoidOverlap) {
    //         oc.set("geometry.avoid-overlap", toString(myGeomAvoidOverlap));
    //     }
    //     if (oc.isWriteable("junctions.higher-speed") && oc.isDefault("junctions.higher-speed") && oc.getBool("junctions.higher-speed") != myJunctionsHigherSpeed) {
    //         oc.set("junctions.higher-speed", toString(myJunctionsHigherSpeed));
    //     }
    //     if (oc.isWriteable("internal-junctions.vehicle-width") && oc.isDefault("internal-junctions.vehicle-width") && oc.getFloat("internal-junctions.vehicle-width") != myInternalJunctionsVehicleWidth) {
    //         oc.set("internal-junctions.vehicle-width", toString(myInternalJunctionsVehicleWidth));
    //     }
    //     if (oc.isWriteable("junctions.minimal-shape") && oc.isDefault("junctions.minimal-shape") && oc.getBool("junctions.minimal-shape") != myJunctionsMinimalShape) {
    //         oc.set("junctions.minimal-shape", toString(myJunctionsMinimalShape));
    //     }
    //     if (oc.isWriteable("junctions.endpoint-shape") && oc.isDefault("junctions.endpoint-shape") && oc.getBool("junctions.endpoint-shape") != myJunctionsEndpointShape) {
    //         oc.set("junctions.endpoint-shape", toString(myJunctionsEndpointShape));
    //     }
    //     if (!deprecatedVehicleClassesSeen.empty()) {
    //         WRITE_WARNINGF(TL("Deprecated vehicle class(es) '%' in input network."), toString(deprecatedVehicleClassesSeen));
    //         deprecatedVehicleClassesSeen.clear();
    //     }
    //     if (!oc.getBool("no-internal-links")) {
    //         // add loaded crossings
    //         for (const auto& crossIt : myPedestrianCrossings) {
    //             NBNode* const node = myNodeCont.retrieve(crossIt.first);
    //             for (const Crossing& crossing : crossIt.second) {
    //                 EdgeVector edges;
    //                 for (const std::string& edgeID : crossing.crossingEdges) {
    //                     NBEdge* edge = myNetBuilder.getEdgeCont().retrieve(edgeID);
    //                     // edge might have been removed due to options
    //                     if (edge != nullptr) {
    //                         edges.push_back(edge);
    //                     }
    //                 }
    //                 if (!edges.empty()) {
    //                     node->addCrossing(edges, crossing.width, crossing.priority,
    //                                       crossing.customTLIndex, crossing.customTLIndex2, crossing.customShape, true, &crossing);
    //                 }
    //             }
    //         }
    //         myNetBuilder.setHaveNetworkCrossings(myPedestrianCrossings.size() > 0);
    //         // add walking area custom shapes
    //         for (const auto& item : myWACustomShapes) {
    //             std::string nodeID = SUMOXMLDefinitions::getJunctionIDFromInternalEdge(item.first);
    //             NBNode* node = myNodeCont.retrieve(nodeID);
    //             std::vector<std::string> edgeIDs;
    //             if (item.second.fromEdges.size() + item.second.toEdges.size() == 0) {
    //                 // must be a split crossing
    //                 assert(item.second.fromCrossed.size() > 0);
    //                 assert(item.second.toCrossed.size() > 0);
    //                 edgeIDs = item.second.fromCrossed;
    //                 edgeIDs.insert(edgeIDs.end(), item.second.toCrossed.begin(), item.second.toCrossed.end());
    //             } else if (item.second.fromEdges.size() > 0) {
    //                 edgeIDs = item.second.fromEdges;
    //             } else {
    //                 edgeIDs = item.second.toEdges;
    //             }
    //             EdgeVector edges;
    //             for (const std::string& edgeID : edgeIDs) {
    //                 NBEdge* edge = myNetBuilder.getEdgeCont().retrieve(edgeID);
    //                 // edge might have been removed due to options
    //                 if (edge != nullptr) {
    //                     edges.push_back(edge);
    //                 }
    //             }
    //             if (edges.size() > 0) {
    //                 node->addWalkingAreaShape(edges, item.second.shape, item.second.width);
    //             }
    //         }
    //     }
    //     // add roundabouts
    //     for (const std::vector<std::string>& ra : myRoundabouts) {
    //         EdgeSet roundabout;
    //         for (const std::string& edgeID : ra) {
    //             NBEdge* edge = myNetBuilder.getEdgeCont().retrieve(edgeID);
    //             if (edge == nullptr) {
    //                 if (!myNetBuilder.getEdgeCont().wasIgnored(edgeID)) {
    //                     WRITE_ERRORF(TL("Unknown edge '%' in roundabout"), (edgeID));
    //                 }
    //             } else {
    //                 roundabout.insert(edge);
    //             }
    //         }
    //         myNetBuilder.getEdgeCont().addRoundabout(roundabout);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_network_2(&mut self, oc: &mut OptionsCont) {
        // check whether the option is set (properly)
        if !oc.is_usable_file_list("sumo-net-file").unwrap() {
            return;
        }
        let discardableParams: StringVector = oc.get_string_vector("discard-params").unwrap().clone();
        self.myDiscardableParams.extend(discardableParams.iter().cloned());
        // parse file(s)
        let files: StringVector = oc.get_string_vector("sumo-net-file").unwrap().clone();
        for file in files.iter() {
            if !utils::common::file_helpers::is_readable(file) {
                write_error(&format!("Could not open sumo-net-file '{}'.", file));
                return;
            }
            self.base.set_file_name(file);
            let before = std::time::Instant::now();
            progress_begin_message(&format!("Parsing sumo-net from '{}'", file));
            let _ = utils::xml::xml_sub_sys::run_parser(self, file, true, false, false, true);
            progress_time_message(before.elapsed().as_millis() as i64);
        }
        // build edges
        let maxSegmentLength: f64 = oc.get_float("geometry.max-segment-length").unwrap();
        let edgeAttrs: Vec<Ptr<NIImporter_SUMO_EdgeAttrs>> = self.myEdges.values().copied().collect();
        for ed in edgeAttrs.iter().copied() {
            let edm = ed.m();
            // skip internal edges
            if matches!(edm.func, SumoXMLEdgeFunc::INTERNAL | SumoXMLEdgeFunc::CROSSING | SumoXMLEdgeFunc::WALKINGAREA) {
                continue;
            }
            // get and check the nodes
            let from = self.myNodeCont.m().retrieve(&edm.fromNode);
            let to = self.myNodeCont.m().retrieve(&edm.toNode);
            if from.is_null() {
                write_error(&format!("Edge's '{}' from-node '{}' is not known.", edm.id, edm.fromNode));
                continue;
            }
            if to.is_null() {
                write_error(&format!("Edge's '{}' to-node '{}' is not known.", edm.id, edm.toNode));
                continue;
            }
            if from == to {
                write_error(&format!("Edge's '{}' from-node and to-node '{}' are identical.", edm.id, edm.toNode));
                continue;
            }
            if edm.shape.len() == 0 && maxSegmentLength > 0.0 {
                edm.shape.push(from.m().get_position().clone());
                edm.shape.push(to.m().get_position().clone());
                // shape is already cartesian but we must use a copy because the original will be modified
                let cartesian = edm.shape.clone();
                NBNetBuilder::add_geometry_segments(&mut edm.shape, &cartesian, maxSegmentLength);
            }
            // build and insert the edge
            // always use tryIgnoreNodePositions to keep original shape
            let e = Ptr::from_raw(Box::into_raw(Box::new(NBEdge::new_2(
                &edm.id,
                from,
                to,
                edm.type_.clone(),
                edm.maxSpeed,
                NBEDGE_UNSPECIFIED_FRICTION,
                edm.lanes.len() as i32,
                edm.priority,
                NBEDGE_UNSPECIFIED_WIDTH,
                NBEDGE_UNSPECIFIED_OFFSET,
                edm.shape.clone(),
                edm.lsf,
                &edm.streetName,
                "",
                true,
            ))));
            e.m().set_loaded_length(edm.length);
            e.m().base2.update_parameters(edm.get_parameters_map());
            e.m().set_distance(edm.distance);
            let ec = self.myNetBuilder.m().get_edge_cont();
            if !ec.insert(e, false) {
                write_error(&format!("Could not insert edge '{}'.", edm.id));
                unsafe { drop(Box::from_raw(e.as_raw())) };
                continue;
            }
            edm.builtEdge = ec.retrieve(&edm.id, false);
            if !edm.builtEdge.is_null() {
                let built = edm.builtEdge;
                built.m().set_edge_stop_offset(-1, &edm.edgeStopOffset, false);
                built.m().set_bidi(!edm.bidi.is_empty());
                built.m().set_routing_type(&edm.routingType);
            }
        }
        // assign further lane attributes (edges are built)
        let mut toRemove: EdgeVector = Vec::new();
        let dismissVclasses = oc.get_bool("dismiss-vclasses").unwrap();
        for ed in edgeAttrs.iter().copied() {
            let edm = ed.m();
            let nbe = edm.builtEdge;
            if nbe.is_null() {
                // inner edge or removed by explicit list, vclass, ...
                continue;
            }
            let toType = nbe.m().get_to_node().m().get_type();
            for fromLaneIndex in 0..(edm.lanes.len() as i32) {
                let lane = edm.lanes[fromLaneIndex as usize];
                // connections
                for c in lane.m().connections.iter() {
                    if !self.myEdges.contains_key(&c.toEdgeID) {
                        write_error(&format!("Unknown edge '{}' given in connection.", c.toEdgeID));
                        continue;
                    }
                    let toEdge = self.myEdges[&c.toEdgeID].m().builtEdge;
                    if toEdge.is_null() {
                        // removed by explicit list, vclass, ...
                        continue;
                    }
                    if toEdge.m().get_from_node() != nbe.m().get_to_node() {
                        // inconsistency may occur when merging networks
                        write_warning(&format!("Removing invalid connection from edge '{}' to edge '{}'", nbe.m().get_id(), toEdge.m().get_id()));
                        continue;
                    }
                    // patch attribute uncontrolled for legacy networks where it is not set explicitly
                    let mut uncontrolled = c.uncontrolled;
                    if (NBNode::is_traffic_light(toType) || matches!(toType, SumoXMLNodeType::RAIL_SIGNAL)) && c.tlLinkIndex == NBCONNECTION_INVALID_TL_INDEX {
                        uncontrolled = true;
                    }
                    let keepClear = if c.keepClear { KeepClear::KEEPCLEAR_TRUE } else { KeepClear::KEEPCLEAR_FALSE };
                    nbe.m().add_lane2_lane_connection(
                        fromLaneIndex,
                        toEdge,
                        c.toLaneIdx,
                        NBEdge_Lane2LaneInfoType::VALIDATED,
                        true,
                        c.mayDefinitelyPass,
                        keepClear,
                        c.contPos,
                        c.visibility,
                        c.speed,
                        c.friction,
                        c.customLength,
                        &c.customShape,
                        uncontrolled,
                        c.permissions,
                        c.indirectLeft,
                        &c.edgeType,
                        c.changeLeft,
                        c.changeRight,
                        false,
                    );
                    // shortcut: CHBuilder_Connection has no Parameterised base, so connection parameters are not copied yet; upgrade when NBEdge::Connection is mapped
                    // maybe we have a tls-controlled connection
                    if !c.tlID.is_empty() && !self.myRailSignals.contains(&c.tlID) {
                        let programs: BTreeMap<String, Ptr<NBTrafficLightDefinition>> = self.myTLLCont.m().get_programs(&c.tlID).clone();
                        if programs.len() > 0 {
                            for (progID, progDef) in programs.iter() {
                                // C++ dynamic_cast<NBLoadedSUMOTLDef*>
                                let tlDef = loaded_tl_def(*progDef);
                                if !tlDef.is_null() {
                                    tlDef.m().add_connection(nbe, toEdge, fromLaneIndex, c.toLaneIdx, c.tlLinkIndex, c.tlLinkIndex2, false);
                                } else {
                                    // shortcut: C++ throws ProcessError out of this void function; panic until load_network returns a Result
                                    panic!("Corrupt traffic light definition '{}' (program '{}')", c.tlID, progID);
                                }
                            }
                        } else {
                            write_error(&format!("The traffic light '{}' is not known.", c.tlID));
                        }
                    }
                }
                // allow/disallow XXX preferred
                if !dismissVclasses {
                    nbe.m().set_permissions(parse_vehicle_classes_allow_disallow(&lane.m().allow, &lane.m().disallow, self.myNetworkVersion), fromLaneIndex);
                }
                nbe.m().set_permitted_changing(
                    fromLaneIndex,
                    parse_vehicle_classes_allow_disallow(&lane.m().changeLeft, "", NETWORK_VERSION),
                    parse_vehicle_classes_allow_disallow(&lane.m().changeRight, "", NETWORK_VERSION),
                );
                let lm = lane.m();
                // width, offset
                nbe.m().set_lane_width(fromLaneIndex, lm.width);
                nbe.m().set_end_offset(fromLaneIndex, lm.endOffset);
                nbe.m().set_speed(fromLaneIndex, lm.maxSpeed);
                nbe.m().set_friction(fromLaneIndex, lm.friction);
                nbe.m().set_acceleration(fromLaneIndex, lm.accelRamp);
                nbe.m().get_lane_struct(fromLaneIndex).oppositeID = lm.oppositeID.clone();
                nbe.m().get_lane_struct(fromLaneIndex).type_ = lm.type_.clone();
                nbe.m().get_lane_struct(fromLaneIndex).base.update_parameters(lm.get_parameters_map());
                if lm.customShape {
                    nbe.m().set_lane_shape(fromLaneIndex, &lm.shape);
                }
                // stop offset for lane
                let mut stopOffsetSet = false;
                if lm.laneStopOffset.is_defined() || nbe.m().get_edge_stop_offset().is_defined() {
                    // apply lane-specific stopOffset (might be none as well)
                    stopOffsetSet = nbe.m().set_edge_stop_offset(fromLaneIndex, &lm.laneStopOffset, false);
                }
                if !stopOffsetSet {
                    // apply default stop offset to lane
                    let defaultOffset = nbe.m().get_edge_stop_offset().clone();
                    nbe.m().set_edge_stop_offset(fromLaneIndex, &defaultOffset, false);
                }
            }
            nbe.m().declare_connections_as_loaded(NBEdge_EdgeBuildingStep::LANES2LANES_USER);
            if !nbe.m().has_lane_specific_width() && nbe.m().get_lanes()[0].width != NBEDGE_UNSPECIFIED_WIDTH {
                let w = nbe.m().get_lane_width_2(0);
                nbe.m().set_lane_width(-1, w);
            }
            if !nbe.m().has_lane_specific_end_offset() && nbe.m().get_end_offset_2(0) != NBEDGE_UNSPECIFIED_OFFSET {
                let o = nbe.m().get_end_offset_2(0);
                nbe.m().set_end_offset(-1, o);
            }
            if !nbe.m().has_lane_specific_stop_offsets() && nbe.m().get_edge_stop_offset().is_defined() {
                let defaultOffset = nbe.m().get_edge_stop_offset().clone();
                nbe.m().set_edge_stop_offset(-1, &defaultOffset, false);
            }
            // check again after permissions are set
            if self.myNetBuilder.m().get_edge_cont().ignore_filter_match(nbe) {
                self.myNetBuilder.m().get_edge_cont().ignore(nbe.m().get_id().to_string());
                toRemove.push(nbe);
            }
        }
        for e in toRemove.iter().copied() {
            let ec = Ptr::from_mut(self.myNetBuilder.m().get_edge_cont());
            let dc = Ptr::from_mut(self.myNetBuilder.m().get_district_cont());
            ec.m().erase(dc.m(), e);
        }
        // insert loaded prohibitions
        for it in self.myProhibitions.iter() {
            let prohibitedFrom = self.myEdges[&it.prohibitedFrom].m().builtEdge;
            let prohibitedTo = self.myEdges[&it.prohibitedTo].m().builtEdge;
            let prohibitorFrom = self.myEdges[&it.prohibitorFrom].m().builtEdge;
            let prohibitorTo = self.myEdges[&it.prohibitorTo].m().builtEdge;
            if prohibitedFrom.is_null() {
                write_warning(&format!("Edge '{}' in prohibition was not built.", it.prohibitedFrom));
            } else if prohibitedTo.is_null() {
                write_warning(&format!("Edge '{}' in prohibition was not built.", it.prohibitedTo));
            } else if prohibitorFrom.is_null() {
                write_warning(&format!("Edge '{}' in prohibition was not built.", it.prohibitorFrom));
            } else if prohibitorTo.is_null() {
                write_warning(&format!("Edge '{}' in prohibition was not built.", it.prohibitorTo));
            } else {
                let n = prohibitedFrom.m().get_to_node();
                n.m().add_sorted_link_foes(
                    &NBConnection::new(prohibitorFrom, prohibitorTo),
                    &NBConnection::new(prohibitedFrom, prohibitedTo),
                );
            }
        }
        if oc.is_writeable("no-internal-links").unwrap() && oc.is_default("no-internal-links") && !self.myHaveSeenInternalEdge {
            oc.set("no-internal-links", "true", false).unwrap();
        }
        if oc.is_writeable("lefthand").unwrap() && oc.is_default("lefthand") && self.myAmLefthand {
            oc.set("lefthand", &format!("{}", self.myAmLefthand), false).unwrap();
        }
        if oc.is_writeable("junctions.corner-detail").unwrap() && oc.is_default("junctions.corner-detail") && oc.get_int("junctions.corner-detail").unwrap() != self.myCornerDetail {
            oc.set("junctions.corner-detail", &format!("{}", self.myCornerDetail), false).unwrap();
        }
        if oc.is_writeable("junctions.internal-link-detail").unwrap() && oc.is_default("junctions.internal-link-detail") && oc.get_int("junctions.internal-link-detail").unwrap() != self.myLinkDetail && self.myLinkDetail > 0 {
            oc.set("junctions.internal-link-detail", &format!("{}", self.myLinkDetail), false).unwrap();
        }
        if oc.is_writeable("rectangular-lane-cut").unwrap() && oc.is_default("rectangular-lane-cut") && oc.get_bool("rectangular-lane-cut").unwrap() != self.myRectLaneCut {
            oc.set("rectangular-lane-cut", &format!("{}", self.myRectLaneCut), false).unwrap();
        }
        if oc.is_writeable("walkingareas").unwrap() && oc.is_default("walkingareas") && oc.get_bool("walkingareas").unwrap() != self.myWalkingAreas {
            oc.set("walkingareas", &format!("{}", self.myWalkingAreas), false).unwrap();
        }
        if oc.is_writeable("junctions.limit-turn-speed").unwrap() && oc.is_default("junctions.limit-turn-speed") && oc.get_float("junctions.limit-turn-speed").unwrap() != self.myLimitTurnSpeed {
            oc.set("junctions.limit-turn-speed", &utils::common::to_string::to_string(&self.myLimitTurnSpeed), false).unwrap();
        }
        if oc.is_writeable("check-lane-foes.all").unwrap() && oc.is_default("check-lane-foes.all") && oc.get_bool("check-lane-foes.all").unwrap() != self.myCheckLaneFoesAll {
            oc.set("check-lane-foes.all", &format!("{}", self.myCheckLaneFoesAll), false).unwrap();
        }
        if oc.is_writeable("check-lane-foes.roundabout").unwrap() && oc.is_default("check-lane-foes.roundabout") && oc.get_bool("check-lane-foes.roundabout").unwrap() != self.myCheckLaneFoesRoundabout {
            oc.set("check-lane-foes.roundabout", &format!("{}", self.myCheckLaneFoesRoundabout), false).unwrap();
        }
        if oc.is_writeable("tls.ignore-internal-junction-jam").unwrap() && oc.is_default("tls.ignore-internal-junction-jam") && oc.get_bool("tls.ignore-internal-junction-jam").unwrap() != self.myTlsIgnoreInternalJunctionJam {
            oc.set("tls.ignore-internal-junction-jam", &format!("{}", self.myTlsIgnoreInternalJunctionJam), false).unwrap();
        }
        if oc.is_writeable("default.spreadtype").unwrap() && oc.is_default("default.spreadtype") && oc.get_string("default.spreadtype").unwrap() != self.myDefaultSpreadType {
            oc.set("default.spreadtype", &self.myDefaultSpreadType, false).unwrap();
        }
        if oc.is_writeable("geometry.avoid-overlap").unwrap() && oc.is_default("geometry.avoid-overlap") && oc.get_bool("geometry.avoid-overlap").unwrap() != self.myGeomAvoidOverlap {
            oc.set("geometry.avoid-overlap", &format!("{}", self.myGeomAvoidOverlap), false).unwrap();
        }
        if oc.is_writeable("junctions.higher-speed").unwrap() && oc.is_default("junctions.higher-speed") && oc.get_bool("junctions.higher-speed").unwrap() != self.myJunctionsHigherSpeed {
            oc.set("junctions.higher-speed", &format!("{}", self.myJunctionsHigherSpeed), false).unwrap();
        }
        if oc.is_writeable("internal-junctions.vehicle-width").unwrap() && oc.is_default("internal-junctions.vehicle-width") && oc.get_float("internal-junctions.vehicle-width").unwrap() != self.myInternalJunctionsVehicleWidth {
            oc.set("internal-junctions.vehicle-width", &utils::common::to_string::to_string(&self.myInternalJunctionsVehicleWidth), false).unwrap();
        }
        if oc.is_writeable("junctions.minimal-shape").unwrap() && oc.is_default("junctions.minimal-shape") && oc.get_bool("junctions.minimal-shape").unwrap() != self.myJunctionsMinimalShape {
            oc.set("junctions.minimal-shape", &format!("{}", self.myJunctionsMinimalShape), false).unwrap();
        }
        if oc.is_writeable("junctions.endpoint-shape").unwrap() && oc.is_default("junctions.endpoint-shape") && oc.get_bool("junctions.endpoint-shape").unwrap() != self.myJunctionsEndpointShape {
            oc.set("junctions.endpoint-shape", &format!("{}", self.myJunctionsEndpointShape), false).unwrap();
        }
        {
            let mut deprecated = DEPRECATED_VEHICLE_CLASSES_SEEN.lock().unwrap();
            if !deprecated.is_empty() {
                write_warning(&format!("Deprecated vehicle class(es) '{}' in input network.", utils::common::to_string::to_string(&*deprecated)));
                deprecated.clear();
            }
        }
        if !oc.get_bool("no-internal-links").unwrap() {
            // add loaded crossings
            for (crossKey, crossings) in self.myPedestrianCrossings.iter() {
                let node = self.myNodeCont.m().retrieve(crossKey);
                for crossing in crossings.iter() {
                    let mut edges: EdgeVector = Vec::new();
                    for edgeID in crossing.crossingEdges.iter() {
                        let edge = self.myNetBuilder.m().get_edge_cont().retrieve(edgeID, false);
                        // edge might have been removed due to options
                        if !edge.is_null() {
                            edges.push(edge);
                        }
                    }
                    if !edges.is_empty() {
                        node.m().add_crossing(
                            edges,
                            crossing.width,
                            crossing.priority,
                            crossing.customTLIndex,
                            crossing.customTLIndex2,
                            &crossing.customShape,
                            true,
                            Ptr::new(&crossing.base),
                        );
                    }
                }
            }
            self.myNetBuilder.m().set_have_network_crossings(self.myPedestrianCrossings.len() > 0);
            // add walking area custom shapes
            for (itemKey, item) in self.myWACustomShapes.iter() {
                let nodeID = get_junction_id_from_internal_edge(itemKey);
                let node = self.myNodeCont.m().retrieve(&nodeID);
                let edgeIDs: Vec<String> = if item.fromEdges.len() + item.toEdges.len() == 0 {
                    // must be a split crossing
                    assert!(item.fromCrossed.len() > 0);
                    assert!(item.toCrossed.len() > 0);
                    let mut ids = item.fromCrossed.clone();
                    ids.extend(item.toCrossed.iter().cloned());
                    ids
                } else if item.fromEdges.len() > 0 {
                    item.fromEdges.clone()
                } else {
                    item.toEdges.clone()
                };
                let mut edges: EdgeVector = Vec::new();
                for edgeID in edgeIDs.iter() {
                    let edge = self.myNetBuilder.m().get_edge_cont().retrieve(edgeID, false);
                    // edge might have been removed due to options
                    if !edge.is_null() {
                        edges.push(edge);
                    }
                }
                if edges.len() > 0 {
                    node.m().add_walking_area_shape(edges, &item.shape, item.width);
                }
            }
        }
        // add roundabouts
        for ra in self.myRoundabouts.iter() {
            let mut roundabout: EdgeSet = BTreeSet::new();
            for edgeID in ra.iter() {
                let edge = self.myNetBuilder.m().get_edge_cont().retrieve(edgeID, false);
                if edge.is_null() {
                    if !self.myNetBuilder.m().get_edge_cont().was_ignored(edgeID.clone()) {
                        write_error(&format!("Unknown edge '{}' in roundabout", edgeID));
                    }
                } else {
                    roundabout.insert(edge);
                }
            }
            self.myNetBuilder.m().get_edge_cont().add_roundabout(&roundabout);
        }
    }

    /// C++: `void NIImporter_SUMO::addEdge(const SUMOSAXAttributes &)` [private]
    /// Parses an edge and stores the values in "myCurrentEdge"
    // void
    // NIImporter_SUMO::addEdge(const SUMOSAXAttributes& attrs) {
    //     // get the id, report an error if not given or empty...
    //     bool ok = true;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     myCurrentEdge = new EdgeAttrs();
    //     myLastParameterised.push_back(myCurrentEdge);
    //     myCurrentEdge->builtEdge = nullptr;
    //     myCurrentEdge->id = id;
    //     // get the function
    //     myCurrentEdge->func = attrs.getOpt<SumoXMLEdgeFunc>(SUMO_ATTR_FUNCTION, id.c_str(), ok, SumoXMLEdgeFunc::NORMAL);
    //     if (myCurrentEdge->func == SumoXMLEdgeFunc::CROSSING) {
    //         // add the crossing but don't do anything else
    //         Crossing c(id);
    //         c.crossingEdges = attrs.get<std::vector<std::string> >(SUMO_ATTR_CROSSING_EDGES, nullptr, ok);
    //         myPedestrianCrossings[SUMOXMLDefinitions::getJunctionIDFromInternalEdge(id)].push_back(c);
    //         return;
    //     } else if (myCurrentEdge->func == SumoXMLEdgeFunc::INTERNAL || myCurrentEdge->func == SumoXMLEdgeFunc::WALKINGAREA) {
    //         myHaveSeenInternalEdge = true;
    //         return; // skip internal edges
    //     }
    //     // get the origin and the destination node
    //     myCurrentEdge->fromNode = attrs.getOpt<std::string>(SUMO_ATTR_FROM, id.c_str(), ok, "");
    //     myCurrentEdge->toNode = attrs.getOpt<std::string>(SUMO_ATTR_TO, id.c_str(), ok, "");
    //     myCurrentEdge->priority = attrs.getOpt<int>(SUMO_ATTR_PRIORITY, id.c_str(), ok, -1);
    //     myCurrentEdge->type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //     myCurrentEdge->routingType = attrs.getOpt<std::string>(SUMO_ATTR_ROUTINGTYPE, id.c_str(), ok, "");
    //     myCurrentEdge->shape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok, PositionVector());
    //     NBNetBuilder::transformCoordinates(myCurrentEdge->shape, true, myLocation);
    //     myCurrentEdge->length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, id.c_str(), ok, NBEdge::UNSPECIFIED_LOADED_LENGTH);
    //     myCurrentEdge->maxSpeed = 0;
    //     myCurrentEdge->streetName = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     myCurrentEdge->distance = attrs.getOpt<double>(SUMO_ATTR_DISTANCE, id.c_str(), ok, 0);
    //     myCurrentEdge->bidi = attrs.getOpt<std::string>(SUMO_ATTR_BIDI, id.c_str(), ok, "");
    //     if (myCurrentEdge->streetName != "" && OptionsCont::getOptions().isDefault("output.street-names")) {
    //         OptionsCont::getOptions().set("output.street-names", "true");
    //     }
    //
    //     std::string lsfS = attrs.getOpt<std::string>(SUMO_ATTR_SPREADTYPE, id.c_str(), ok, myDefaultSpreadType);
    //     if (SUMOXMLDefinitions::LaneSpreadFunctions.hasString(lsfS)) {
    //         myCurrentEdge->lsf = SUMOXMLDefinitions::LaneSpreadFunctions.get(lsfS);
    //     } else {
    //         WRITE_ERRORF(TL("Unknown spreadType '%' for edge '%'."), lsfS, id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, attrs: &SUMOSAXAttributes) {
        // get the id, report an error if not given or empty...
        let mut ok = true;
        let id: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        // C++ leaves the members of EdgeAttrs uninitialised; the values below are placeholders until they are read
        let cur = Ptr::from_raw(Box::into_raw(Box::new(NIImporter_SUMO_EdgeAttrs {
            base: Parameterised::new(),
            id: String::new(),
            streetName: String::new(),
            type_: String::new(),
            routingType: String::new(),
            func: SumoXMLEdgeFunc::NORMAL,
            fromNode: String::new(),
            toNode: String::new(),
            shape: PositionVector::default(),
            length: 0.0,
            priority: 0,
            maxSpeed: 0.0,
            lanes: Vec::new(),
            builtEdge: Ptr::null(),
            lsf: LaneSpreadFunction::RIGHT,
            edgeStopOffset: StopOffset::default(),
            distance: 0.0,
            bidi: String::new(),
        })));
        self.myCurrentEdge = cur;
        self.myLastParameterised.push(Ptr::from_mut(&mut cur.m().base));
        cur.m().builtEdge = Ptr::null();
        cur.m().id = id.clone();
        // get the function
        cur.m().func = attrs.get_opt(SUMO_ATTR_FUNCTION, &id, &mut ok, SumoXMLEdgeFunc::NORMAL, true);
        if matches!(cur.m().func, SumoXMLEdgeFunc::CROSSING) {
            // add the crossing but don't do anything else
            let mut c = NIImporter_SUMO_Crossing::new(&id);
            c.crossingEdges = attrs.get(SUMO_ATTR_CROSSING_EDGES, "", &mut ok, true);
            self.myPedestrianCrossings.entry(get_junction_id_from_internal_edge(&id)).or_insert_with(Vec::new).push(c);
            return;
        } else if matches!(cur.m().func, SumoXMLEdgeFunc::INTERNAL | SumoXMLEdgeFunc::WALKINGAREA) {
            self.myHaveSeenInternalEdge = true;
            return; // skip internal edges
        }
        // get the origin and the destination node
        cur.m().fromNode = attrs.get_opt(SUMO_ATTR_FROM, &id, &mut ok, String::new(), true);
        cur.m().toNode = attrs.get_opt(SUMO_ATTR_TO, &id, &mut ok, String::new(), true);
        cur.m().priority = attrs.get_opt(SUMO_ATTR_PRIORITY, &id, &mut ok, -1, true);
        cur.m().type_ = attrs.get_opt(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true);
        cur.m().routingType = attrs.get_opt(SUMO_ATTR_ROUTINGTYPE, &id, &mut ok, String::new(), true);
        cur.m().shape = attrs.get_opt(SUMO_ATTR_SHAPE, &id, &mut ok, PositionVector::default(), true);
        NBNetBuilder::transform_coordinates(&mut cur.m().shape, true, self.myLocation);
        // NBEdge::UNSPECIFIED_LOADED_LENGTH = -1
        cur.m().length = attrs.get_opt(SUMO_ATTR_LENGTH, &id, &mut ok, -1.0, true);
        cur.m().maxSpeed = 0.0;
        cur.m().streetName = attrs.get_opt(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        cur.m().distance = attrs.get_opt(SUMO_ATTR_DISTANCE, &id, &mut ok, 0.0, true);
        cur.m().bidi = attrs.get_opt(SUMO_ATTR_BIDI, &id, &mut ok, String::new(), true);
        if !cur.m().streetName.is_empty() && utils::options::options_cont::is_default("output.street-names") {
            utils::options::options_cont::with_options(|oc| {
                oc.set("output.street-names", "true", false).unwrap();
            });
        }

        let lsfS: String = attrs.get_opt(SUMO_ATTR_SPREADTYPE, &id, &mut ok, self.myDefaultSpreadType.clone(), true);
        if LaneSpreadFunctions.has_string(&lsfS) {
            cur.m().lsf = LaneSpreadFunctions.get(&lsfS).unwrap();
        } else {
            write_error(&format!("Unknown spreadType '{}' for edge '{}'.", lsfS, id));
        }
    }

    /// C++: `void NIImporter_SUMO::addLane(const SUMOSAXAttributes &)` [private]
    /// Parses a lane and stores the values in "myCurrentLane"
    // void
    // NIImporter_SUMO::addLane(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     if (myCurrentEdge == nullptr) {
    //         WRITE_ERRORF(TL("Found lane '%' not within edge element."), id);
    //         return;
    //     }
    //     const std::string expectedID = myCurrentEdge->id + "_" + toString(myCurrentEdge->lanes.size());
    //     if (id != expectedID) {
    //         WRITE_WARNINGF(TL("Renaming lane '%' to '%'."), id, expectedID);
    //     }
    //     myCurrentLane = new LaneAttrs();
    //     myLastParameterised.push_back(myCurrentLane);
    //     myCurrentLane->customShape = attrs.getOpt<bool>(SUMO_ATTR_CUSTOMSHAPE, nullptr, ok, false);
    //     myCurrentLane->shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok);
    //     myCurrentLane->width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id.c_str(), ok, (double) NBEdge::UNSPECIFIED_WIDTH);
    //     myCurrentLane->type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //     if (myCurrentEdge->func == SumoXMLEdgeFunc::CROSSING) {
    //         // save the width and the lane id of the crossing but don't do anything else
    //         std::vector<Crossing>& crossings = myPedestrianCrossings[SUMOXMLDefinitions::getJunctionIDFromInternalEdge(myCurrentEdge->id)];
    //         assert(crossings.size() > 0);
    //         crossings.back().width = attrs.get<double>(SUMO_ATTR_WIDTH, id.c_str(), ok);
    //         myLastParameterised.pop_back();
    //         myLastParameterised.push_back(&crossings.back());
    //         if (myCurrentLane->customShape) {
    //             crossings.back().customShape = myCurrentLane->shape;
    //             NBNetBuilder::transformCoordinates(crossings.back().customShape, true, myLocation);
    //         }
    //     } else if (myCurrentEdge->func == SumoXMLEdgeFunc::WALKINGAREA) {
    //         // save custom shape if needed but don't do anything else
    //         if (myCurrentLane->customShape) {
    //             WalkingAreaParsedCustomShape wacs;
    //             wacs.shape = myCurrentLane->shape;
    //             wacs.width = myCurrentLane->width;
    //             NBNetBuilder::transformCoordinates(wacs.shape, true, myLocation);
    //             myWACustomShapes[myCurrentEdge->id] = wacs;
    //         }
    //         return;
    //     } else if (myCurrentEdge->func == SumoXMLEdgeFunc::INTERNAL) {
    //         return; // skip internal edges
    //     }
    //     if (attrs.hasAttribute("maxspeed")) {
    //         // !!! deprecated
    //         myCurrentLane->maxSpeed = attrs.getFloat("maxspeed");
    //     } else {
    //         myCurrentLane->maxSpeed = attrs.get<double>(SUMO_ATTR_SPEED, id.c_str(), ok);
    //     }
    //     myCurrentLane->friction = attrs.getOpt<double>(SUMO_ATTR_FRICTION, id.c_str(), ok, NBEdge::UNSPECIFIED_FRICTION, false); //sets 1 on empty
    //     try {
    //         myCurrentLane->allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, id.c_str(), ok, "", false);
    //     } catch (EmptyData&) {
    //         // !!! deprecated
    //         myCurrentLane->allow = "";
    //     }
    //     myCurrentLane->disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, id.c_str(), ok, "");
    //     myCurrentLane->endOffset = attrs.getOpt<double>(SUMO_ATTR_ENDOFFSET, id.c_str(), ok, (double) NBEdge::UNSPECIFIED_OFFSET);
    //     myCurrentLane->accelRamp = attrs.getOpt<bool>(SUMO_ATTR_ACCELERATION, id.c_str(), ok, false);
    //     myCurrentLane->changeLeft = attrs.getOpt<std::string>(SUMO_ATTR_CHANGE_LEFT, id.c_str(), ok, "");
    //     myCurrentLane->changeRight = attrs.getOpt<std::string>(SUMO_ATTR_CHANGE_RIGHT, id.c_str(), ok, "");
    //     if (myChangeLefthand) {
    //         std::swap(myCurrentLane->changeLeft, myCurrentLane->changeRight);
    //     }
    //
    //     // lane coordinates are derived (via lane spread) do not include them in convex boundary
    //     NBNetBuilder::transformCoordinates(myCurrentLane->shape, false, myLocation);
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let id: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        if self.myCurrentEdge.is_null() {
            write_error(&format!("Found lane '{}' not within edge element.", id));
            return;
        }
        let curEdge = self.myCurrentEdge;
        let expectedID = format!("{}_{}", curEdge.m().id, curEdge.m().lanes.len());
        if id != expectedID {
            write_warning(&format!("Renaming lane '{}' to '{}'.", id, expectedID));
        }
        // C++ leaves the members of LaneAttrs uninitialised; the values below are placeholders until they are read
        let lane = Ptr::from_raw(Box::into_raw(Box::new(NIImporter_SUMO_LaneAttrs {
            base: Parameterised::new(),
            maxSpeed: 0.0,
            friction: 0.0,
            shape: PositionVector::default(),
            connections: Vec::new(),
            allow: String::new(),
            disallow: String::new(),
            changeLeft: String::new(),
            changeRight: String::new(),
            width: 0.0,
            endOffset: 0.0,
            laneStopOffset: StopOffset::default(),
            accelRamp: false,
            oppositeID: String::new(),
            customShape: false,
            type_: String::new(),
        })));
        self.myCurrentLane = lane;
        self.myLastParameterised.push(Ptr::from_mut(&mut lane.m().base));
        lane.m().customShape = attrs.get_opt(SUMO_ATTR_CUSTOMSHAPE, "", &mut ok, false, true);
        lane.m().shape = attrs.get(SUMO_ATTR_SHAPE, &id, &mut ok, true);
        lane.m().width = attrs.get_opt(SUMO_ATTR_WIDTH, &id, &mut ok, NBEDGE_UNSPECIFIED_WIDTH, true);
        lane.m().type_ = attrs.get_opt(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true);
        if matches!(curEdge.m().func, SumoXMLEdgeFunc::CROSSING) {
            // save the width and the lane id of the crossing but don't do anything else
            let crossings = self.myPedestrianCrossings.entry(get_junction_id_from_internal_edge(&curEdge.m().id)).or_insert_with(Vec::new);
            assert!(crossings.len() > 0);
            let last = crossings.last_mut().unwrap();
            last.width = attrs.get(SUMO_ATTR_WIDTH, &id, &mut ok, true);
            self.myLastParameterised.pop();
            self.myLastParameterised.push(Ptr::from_mut(&mut last.base));
            if lane.m().customShape {
                last.customShape = lane.m().shape.clone();
                NBNetBuilder::transform_coordinates(&mut last.customShape, true, self.myLocation);
            }
        } else if matches!(curEdge.m().func, SumoXMLEdgeFunc::WALKINGAREA) {
            // save custom shape if needed but don't do anything else
            if lane.m().customShape {
                let mut wacs = NIImporter_SUMO_WalkingAreaParsedCustomShape {
                    shape: lane.m().shape.clone(),
                    fromEdges: Vec::new(),
                    toEdges: Vec::new(),
                    fromCrossed: Vec::new(),
                    toCrossed: Vec::new(),
                    width: lane.m().width,
                };
                NBNetBuilder::transform_coordinates(&mut wacs.shape, true, self.myLocation);
                self.myWACustomShapes.insert(curEdge.m().id.clone(), wacs);
            }
            return;
        } else if matches!(curEdge.m().func, SumoXMLEdgeFunc::INTERNAL) {
            return; // skip internal edges
        }
        if attrs.has_attribute_name("maxspeed") {
            // !!! deprecated
            lane.m().maxSpeed = attrs.get_float_name("maxspeed").unwrap();
        } else {
            lane.m().maxSpeed = attrs.get(SUMO_ATTR_SPEED, &id, &mut ok, true);
        }
        lane.m().friction = attrs.get_opt(SUMO_ATTR_FRICTION, &id, &mut ok, NBEDGE_UNSPECIFIED_FRICTION, false); //sets 1 on empty
        // C++ catches EmptyData here and sets allow to ""; get_opt already yields "" for an empty value
        lane.m().allow = attrs.get_opt(SUMO_ATTR_ALLOW, &id, &mut ok, String::new(), false);
        lane.m().disallow = attrs.get_opt(SUMO_ATTR_DISALLOW, &id, &mut ok, String::new(), true);
        lane.m().endOffset = attrs.get_opt(SUMO_ATTR_ENDOFFSET, &id, &mut ok, NBEDGE_UNSPECIFIED_OFFSET, true);
        lane.m().accelRamp = attrs.get_opt(SUMO_ATTR_ACCELERATION, &id, &mut ok, false, true);
        lane.m().changeLeft = attrs.get_opt(SUMO_ATTR_CHANGE_LEFT, &id, &mut ok, String::new(), true);
        lane.m().changeRight = attrs.get_opt(SUMO_ATTR_CHANGE_RIGHT, &id, &mut ok, String::new(), true);
        if self.myChangeLefthand {
            std::mem::swap(&mut lane.m().changeLeft, &mut lane.m().changeRight);
        }
        // lane coordinates are derived (via lane spread) do not include them in convex boundary
        NBNetBuilder::transform_coordinates(&mut lane.m().shape, false, self.myLocation);
    }

    /// C++: `void NIImporter_SUMO::addStopOffsets(const SUMOSAXAttributes &, bool &)` [private]
    /// parses stop offsets for the current lane or edge
    // void
    // NIImporter_SUMO::addStopOffsets(const SUMOSAXAttributes& attrs, bool& ok) {
    //     const StopOffset offset(attrs, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     // Admissibility of value will be checked in _loadNetwork(), when lengths are known
    //     if (myCurrentLane == nullptr) {
    //         if (myCurrentEdge->edgeStopOffset.isDefined()) {
    //             WRITE_WARNINGF(TL("Duplicate definition of stopOffset for edge %.\nIgnoring duplicate specification."), myCurrentEdge->id);
    //         } else {
    //             myCurrentEdge->edgeStopOffset = offset;
    //         }
    //     } else {
    //         if (myCurrentLane->laneStopOffset.isDefined()) {
    //             WRITE_WARNINGF(TL("Duplicate definition of lane's stopOffset on edge %.\nIgnoring duplicate specifications."), myCurrentEdge->id);
    //         } else {
    //             myCurrentLane->laneStopOffset = offset;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_stop_offsets(&mut self, attrs: &SUMOSAXAttributes, ok: &mut bool) {
        // inlined C++ StopOffset(const SUMOSAXAttributes&, bool&) constructor
        let mut offset = StopOffset::default();
        offset.set_permissions(SVC_IGNORING);
        offset.set_offset(0.0);
        // first check conditions
        if attrs.has_attribute(SUMO_ATTR_VCLASSES) && attrs.has_attribute(SUMO_ATTR_EXCEPTIONS) {
            write_error("Simultaneous specification of vClasses and exceptions is not allowed");
            *ok = false;
        }
        if !attrs.has_attribute(SUMO_ATTR_VALUE) {
            write_error("StopOffset requires an offset value");
            *ok = false;
        }
        // parse elements
        let vClasses: String = attrs.get_opt(SUMO_ATTR_VCLASSES, "", ok, String::new(), true);
        let exceptions: String = attrs.get_opt(SUMO_ATTR_EXCEPTIONS, "", ok, String::new(), true);
        // parse permissions
        if attrs.has_attribute(SUMO_ATTR_VCLASSES) {
            offset.set_permissions(parse_vehicle_classes(&vClasses));
        } else if attrs.has_attribute(SUMO_ATTR_EXCEPTIONS) {
            offset.set_exceptions(&exceptions);
        } else {
            // no vClasses specified, thus apply to all
            offset.set_permissions(parse_vehicle_classes("all"));
        }
        // parse offset
        offset.set_offset(attrs.get_opt(SUMO_ATTR_VALUE, "", ok, 0.0, true));
        if !*ok {
            return;
        }
        // Admissibility of value will be checked in _loadNetwork(), when lengths are known
        if self.myCurrentLane.is_null() {
            let edge = self.myCurrentEdge.m();
            if edge.edgeStopOffset.is_defined() {
                write_warning(&format!("Duplicate definition of stopOffset for edge {}.\nIgnoring duplicate specification.", edge.id));
            } else {
                edge.edgeStopOffset = offset;
            }
        } else {
            let lane = self.myCurrentLane.m();
            if lane.laneStopOffset.is_defined() {
                write_warning(&format!("Duplicate definition of lane's stopOffset on edge {}.\nIgnoring duplicate specifications.", self.myCurrentEdge.m().id));
            } else {
                lane.laneStopOffset = offset;
            }
        }
    }

    /// C++: `void NIImporter_SUMO::addJunction(const SUMOSAXAttributes &)` [private]
    /// Parses a junction and saves it in the node control
    // void
    // NIImporter_SUMO::addJunction(const SUMOSAXAttributes& attrs) {
    //     // get the id, report an error if not given or empty...
    //     myCurrentJunction.node = nullptr;
    //     myCurrentJunction.response.clear();
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     if (id[0] == ':') { // internal node
    //         return;
    //     }
    //     SumoXMLNodeType type = attrs.getOpt<SumoXMLNodeType>(SUMO_ATTR_TYPE, id.c_str(), ok, SumoXMLNodeType::UNKNOWN);
    //     if (ok) {
    //         if (type == SumoXMLNodeType::DEAD_END_DEPRECATED || type == SumoXMLNodeType::DEAD_END) {
    //             // dead end is a computed status. Reset this to unknown so it will
    //             // be corrected if additional connections are loaded
    //             type = SumoXMLNodeType::UNKNOWN;
    //         } else if (type == SumoXMLNodeType::INTERNAL) {
    //             WRITE_WARNINGF("Invalid node type '%' for junction '%' in input network", toString(SumoXMLNodeType::INTERNAL), id);
    //             type = SumoXMLNodeType::UNKNOWN;
    //         }
    //     }
    //     Position pos = readPosition(attrs, id, ok);
    //     NBNetBuilder::transformCoordinate(pos, true, myLocation);
    //     NBNode* node = new NBNode(id, pos, type);
    //     if (!myNodeCont.insert(node)) {
    //         WRITE_WARNINGF(TL("Junction '%' occurred at least twice in the input."), id);
    //         delete node;
    //         myLastParameterised.push_back(myNodeCont.retrieve(id));
    //         return;
    //     } else {
    //         myLastParameterised.push_back(node);
    //     }
    //     myCurrentJunction.node = node;
    //     // set optional radius
    //     if (attrs.hasAttribute(SUMO_ATTR_RADIUS)) {
    //         node->setRadius(attrs.get<double>(SUMO_ATTR_RADIUS, id.c_str(), ok));
    //     }
    //     // handle custom shape
    //     if (attrs.getOpt<bool>(SUMO_ATTR_CUSTOMSHAPE, id.c_str(), ok, false)) {
    //         PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok);
    //         NBNetBuilder::transformCoordinates(shape, true, myLocation);
    //         node->setCustomShape(shape);
    //     }
    //     if (type == SumoXMLNodeType::RAIL_SIGNAL || type == SumoXMLNodeType::RAIL_CROSSING) {
    //         // both types of nodes come without a tlLogic
    //         myRailSignals.insert(id);
    //     }
    //     node->setRightOfWay(attrs.getOpt<RightOfWay>(SUMO_ATTR_RIGHT_OF_WAY, id.c_str(), ok, node->getRightOfWay()));
    //     node->setFringeType(attrs.getOpt<FringeType>(SUMO_ATTR_FRINGE, id.c_str(), ok, node->getFringeType()));
    //     node->setRoundaboutType(attrs.getOpt<RoundaboutType>(SUMO_ATTR_ROUNDABOUT, id.c_str(), ok, node->getRoundaboutType()));
    //     if (attrs.hasAttribute(SUMO_ATTR_NAME)) {
    //         node->setName(attrs.get<std::string>(SUMO_ATTR_NAME, id.c_str(), ok));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_junction(&mut self, attrs: &SUMOSAXAttributes) {
        // get the id, report an error if not given or empty...
        self.myCurrentJunction.node = Ptr::null();
        self.myCurrentJunction.response.clear();
        let mut ok = true;
        let id: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        if id.starts_with(':') {
            // internal node
            return;
        }
        let mut type_: SumoXMLNodeType = attrs.get_opt(SUMO_ATTR_TYPE, &id, &mut ok, SumoXMLNodeType::UNKNOWN, true);
        if ok {
            if matches!(type_, SumoXMLNodeType::DEAD_END_DEPRECATED | SumoXMLNodeType::DEAD_END) {
                // dead end is a computed status. Reset this to unknown so it will
                // be corrected if additional connections are loaded
                type_ = SumoXMLNodeType::UNKNOWN;
            } else if matches!(type_, SumoXMLNodeType::INTERNAL) {
                write_warning(&format!(
                    "Invalid node type '{}' for junction '{}' in input network",
                    NodeTypes.get_string(SumoXMLNodeType::INTERNAL),
                    id
                ));
                type_ = SumoXMLNodeType::UNKNOWN;
            }
        }
        let mut pos = Self::read_position(attrs, &id, &mut ok);
        NBNetBuilder::transform_coordinate(&mut pos, true, self.myLocation);
        let node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(&id, &pos, type_))));
        if !self.myNodeCont.m().insert_2(node) {
            write_warning(&format!("Junction '{}' occurred at least twice in the input.", id));
            unsafe { drop(Box::from_raw(node.as_raw())) };
            let existing = self.myNodeCont.m().retrieve(&id);
            self.myLastParameterised.push(Ptr::from_mut(&mut existing.m().base2));
            return;
        } else {
            self.myLastParameterised.push(Ptr::from_mut(&mut node.m().base2));
        }
        self.myCurrentJunction.node = node;
        // set optional radius
        if attrs.has_attribute(SUMO_ATTR_RADIUS) {
            let radius: f64 = attrs.get(SUMO_ATTR_RADIUS, &id, &mut ok, true);
            node.m().set_radius(radius);
        }
        // handle custom shape
        if attrs.get_opt(SUMO_ATTR_CUSTOMSHAPE, &id, &mut ok, false, true) {
            let mut shape: PositionVector = attrs.get(SUMO_ATTR_SHAPE, &id, &mut ok, true);
            NBNetBuilder::transform_coordinates(&mut shape, true, self.myLocation);
            node.m().set_custom_shape(&shape);
        }
        if matches!(type_, SumoXMLNodeType::RAIL_SIGNAL | SumoXMLNodeType::RAIL_CROSSING) {
            // both types of nodes come without a tlLogic
            self.myRailSignals.insert(id.clone());
        }
        let rightOfWay: RightOfWay = attrs.get_opt(SUMO_ATTR_RIGHT_OF_WAY, &id, &mut ok, node.m().get_right_of_way(), true);
        node.m().set_right_of_way(rightOfWay);
        let fringeType: FringeType = attrs.get_opt(SUMO_ATTR_FRINGE, &id, &mut ok, node.m().get_fringe_type(), true);
        node.m().set_fringe_type(fringeType);
        let roundaboutType: RoundaboutType = attrs.get_opt(SUMO_ATTR_ROUNDABOUT, &id, &mut ok, node.m().get_roundabout_type(), true);
        node.m().set_roundabout_type(roundaboutType);
        if attrs.has_attribute(SUMO_ATTR_NAME) {
            let name: String = attrs.get(SUMO_ATTR_NAME, &id, &mut ok, true);
            node.m().set_name(&name);
        }
    }

    /// C++: `void NIImporter_SUMO::addRequest(const SUMOSAXAttributes &)` [private]
    /// Parses a request and saves selected attributes in myCurrentJunction
    // void
    // NIImporter_SUMO::addRequest(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentJunction.node != nullptr) {
    //         bool ok = true;
    //         myCurrentJunction.response.push_back(attrs.get<std::string>(SUMO_ATTR_RESPONSE, nullptr, ok));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_request(&mut self, attrs: &SUMOSAXAttributes) {
        if !self.myCurrentJunction.node.is_null() {
            let mut ok = true;
            let response: String = attrs.get(SUMO_ATTR_RESPONSE, "", &mut ok, true);
            self.myCurrentJunction.response.push(response);
        }
    }

    /// C++: `void NIImporter_SUMO::addConnection(const SUMOSAXAttributes &)` [private]
    /// Parses a connection and saves it into the lane's definition stored in "myCurrentLane"
    // void
    // NIImporter_SUMO::addConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     if (myEdges.count(fromID) == 0) {
    //         WRITE_ERRORF(TL("Unknown edge '%' given in connection."), fromID);
    //         return;
    //     }
    //     EdgeAttrs* from = myEdges[fromID];
    //     if (from->func == SumoXMLEdgeFunc::INTERNAL) {
    //         // internal junction connection
    //         return;
    //     }
    //
    //     Connection conn;
    //     conn.toEdgeID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     int fromLaneIdx = attrs.get<int>(SUMO_ATTR_FROM_LANE, nullptr, ok);
    //     conn.toLaneIdx = attrs.get<int>(SUMO_ATTR_TO_LANE, nullptr, ok);
    //     conn.tlID = attrs.getOpt<std::string>(SUMO_ATTR_TLID, nullptr, ok, "");
    //     conn.mayDefinitelyPass = attrs.getOpt<bool>(SUMO_ATTR_PASS, nullptr, ok, false);
    //     conn.keepClear = attrs.getOpt<bool>(SUMO_ATTR_KEEP_CLEAR, nullptr, ok, true);
    //     conn.indirectLeft = attrs.getOpt<bool>(SUMO_ATTR_INDIRECT, nullptr, ok, false);
    //     conn.edgeType = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, nullptr, ok, "");
    //     double contPos = NBEdge::UNSPECIFIED_CONTPOS;
    //     if (OptionsCont::getOptions().isSet("default.connection.cont-pos")) {
    //         contPos = OptionsCont::getOptions().getFloat("default.connection.cont-pos");
    //     }
    //     conn.contPos = attrs.getOpt<double>(SUMO_ATTR_CONTPOS, nullptr, ok, contPos);
    //     conn.visibility = attrs.getOpt<double>(SUMO_ATTR_VISIBILITY_DISTANCE, nullptr, ok, NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE);
    //     std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, nullptr, ok, "", false);
    //     std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, nullptr, ok, "", false);
    //     if (allow == "" && disallow == "") {
    //         conn.permissions = SVC_UNSPECIFIED;
    //     } else {
    //         conn.permissions = parseVehicleClasses(allow, disallow, myNetworkVersion);
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_CHANGE_LEFT)) {
    //         conn.changeLeft = parseVehicleClasses(attrs.get<std::string>(SUMO_ATTR_CHANGE_LEFT, nullptr, ok), "");
    //     } else {
    //         conn.changeLeft = SVC_UNSPECIFIED;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_CHANGE_RIGHT)) {
    //         conn.changeRight = parseVehicleClasses(attrs.get<std::string>(SUMO_ATTR_CHANGE_RIGHT, nullptr, ok), "");
    //     } else {
    //         conn.changeRight = SVC_UNSPECIFIED;
    //     }
    //     if (myChangeLefthand) {
    //         std::swap(conn.changeLeft, conn.changeRight);
    //     }
    //     conn.speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, nullptr, ok, NBEdge::UNSPECIFIED_SPEED);
    //     conn.friction = attrs.getOpt<double>(SUMO_ATTR_FRICTION, nullptr, ok, NBEdge::UNSPECIFIED_FRICTION);
    //     conn.customLength = attrs.getOpt<double>(SUMO_ATTR_LENGTH, nullptr, ok, NBEdge::UNSPECIFIED_LOADED_LENGTH);
    //     conn.customShape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, nullptr, ok, PositionVector::EMPTY);
    //     NBNetBuilder::transformCoordinates(conn.customShape, false, myLocation);
    //     conn.uncontrolled = attrs.getOpt<bool>(SUMO_ATTR_UNCONTROLLED, nullptr, ok, NBEdge::UNSPECIFIED_CONNECTION_UNCONTROLLED, false);
    //     if (conn.tlID != "") {
    //         conn.tlLinkIndex = attrs.get<int>(SUMO_ATTR_TLLINKINDEX, nullptr, ok);
    //         conn.tlLinkIndex2 = attrs.getOpt<int>(SUMO_ATTR_TLLINKINDEX2, nullptr, ok, -1);
    //     } else {
    //         conn.tlLinkIndex = NBConnection::InvalidTlIndex;
    //     }
    //     if ((int)from->lanes.size() <= fromLaneIdx) {
    //         WRITE_ERRORF(TL("Invalid lane index '%' for connection from '%'."), toString(fromLaneIdx), fromID);
    //         return;
    //     }
    //     from->lanes[fromLaneIdx]->connections.push_back(conn);
    //     myLastParameterised.push_back(&from->lanes[fromLaneIdx]->connections.back());
    //
    //     // determine crossing priority and tlIndex
    //     if (myPedestrianCrossings.size() > 0) {
    //         if (from->func == SumoXMLEdgeFunc::WALKINGAREA && myEdges[conn.toEdgeID]->func == SumoXMLEdgeFunc::CROSSING) {
    //             // connection from walkingArea to crossing
    //             std::vector<Crossing>& crossings = myPedestrianCrossings[SUMOXMLDefinitions::getJunctionIDFromInternalEdge(fromID)];
    //             for (std::vector<Crossing>::iterator it = crossings.begin(); it != crossings.end(); ++it) {
    //                 if (conn.toEdgeID == (*it).edgeID) {
    //                     if (conn.tlID != "") {
    //                         (*it).priority = true;
    //                         (*it).customTLIndex = conn.tlLinkIndex;
    //                     } else {
    //                         LinkState state = SUMOXMLDefinitions::LinkStates.get(attrs.get<std::string>(SUMO_ATTR_STATE, nullptr, ok));
    //                         (*it).priority = state == LINKSTATE_MAJOR;
    //                     }
    //                 }
    //             }
    //         } else if (from->func == SumoXMLEdgeFunc::CROSSING && myEdges[conn.toEdgeID]->func == SumoXMLEdgeFunc::WALKINGAREA) {
    //             // connection from crossing to walkingArea (set optional linkIndex2)
    //             for (Crossing& c : myPedestrianCrossings[SUMOXMLDefinitions::getJunctionIDFromInternalEdge(fromID)]) {
    //                 if (fromID == c.edgeID) {
    //                     c.customTLIndex2 = attrs.getOpt<int>(SUMO_ATTR_TLLINKINDEX, nullptr, ok, -1);
    //                 }
    //             }
    //         }
    //     }
    //     // determine walking area reference edges
    //     if (myWACustomShapes.size() > 0) {
    //         EdgeAttrs* to = myEdges[conn.toEdgeID];
    //         if (from->func == SumoXMLEdgeFunc::WALKINGAREA) {
    //             std::map<std::string, WalkingAreaParsedCustomShape>::iterator it = myWACustomShapes.find(fromID);
    //             if (it != myWACustomShapes.end()) {
    //                 if (to->func == SumoXMLEdgeFunc::NORMAL) {
    //                     // add target sidewalk as reference
    //                     it->second.toEdges.push_back(conn.toEdgeID);
    //                 } else if (to->func == SumoXMLEdgeFunc::CROSSING) {
    //                     // add target crossing edges as reference
    //                     for (Crossing crossing : myPedestrianCrossings[SUMOXMLDefinitions::getJunctionIDFromInternalEdge(fromID)]) {
    //                         if (conn.toEdgeID == crossing.edgeID) {
    //                             it->second.toCrossed.insert(it->second.toCrossed.end(), crossing.crossingEdges.begin(), crossing.crossingEdges.end());
    //                         }
    //                     }
    //                 }
    //             }
    //         } else if (to->func == SumoXMLEdgeFunc::WALKINGAREA) {
    //             std::map<std::string, WalkingAreaParsedCustomShape>::iterator it = myWACustomShapes.find(conn.toEdgeID);
    //             if (it != myWACustomShapes.end()) {
    //                 if (from->func == SumoXMLEdgeFunc::NORMAL) {
    //                     // add origin sidewalk as reference
    //                     it->second.fromEdges.push_back(fromID);
    //                 } else if (from->func == SumoXMLEdgeFunc::CROSSING) {
    //                     // add origin crossing edges as reference
    //                     for (Crossing crossing : myPedestrianCrossings[SUMOXMLDefinitions::getJunctionIDFromInternalEdge(fromID)]) {
    //                         if (fromID == crossing.edgeID) {
    //                             it->second.fromCrossed.insert(it->second.fromCrossed.end(), crossing.crossingEdges.begin(), crossing.crossingEdges.end());
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_connection(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let fromID: String = attrs.get(SUMO_ATTR_FROM, "", &mut ok, true);
        if !self.myEdges.contains_key(&fromID) {
            write_error(&format!("Unknown edge '{}' given in connection.", fromID));
            return;
        }
        let from = self.myEdges[&fromID];
        if matches!(from.m().func, SumoXMLEdgeFunc::INTERNAL) {
            // internal junction connection
            return;
        }

        let mut conn = NIImporter_SUMO_Connection {
            base: Parameterised::new(),
            toEdgeID: String::new(),
            toLaneIdx: 0,
            tlID: String::new(),
            tlLinkIndex: 0,
            tlLinkIndex2: 0,
            mayDefinitelyPass: false,
            keepClear: false,
            contPos: 0.0,
            visibility: 0.0,
            permissions: 0,
            changeLeft: 0,
            changeRight: 0,
            speed: 0.0,
            friction: 0.0,
            customLength: 0.0,
            customShape: PositionVector::default(),
            uncontrolled: false,
            indirectLeft: false,
            edgeType: String::new(),
        };
        conn.toEdgeID = attrs.get(SUMO_ATTR_TO, "", &mut ok, true);
        let fromLaneIdx: i32 = attrs.get(SUMO_ATTR_FROM_LANE, "", &mut ok, true);
        conn.toLaneIdx = attrs.get(SUMO_ATTR_TO_LANE, "", &mut ok, true);
        conn.tlID = attrs.get_opt(SUMO_ATTR_TLID, "", &mut ok, String::new(), true);
        conn.mayDefinitelyPass = attrs.get_opt(SUMO_ATTR_PASS, "", &mut ok, false, true);
        conn.keepClear = attrs.get_opt(SUMO_ATTR_KEEP_CLEAR, "", &mut ok, true, true);
        conn.indirectLeft = attrs.get_opt(SUMO_ATTR_INDIRECT, "", &mut ok, false, true);
        conn.edgeType = attrs.get_opt(SUMO_ATTR_TYPE, "", &mut ok, String::new(), true);
        // NBEdge::UNSPECIFIED_CONTPOS = -1
        let mut contPos: f64 = -1.0;
        if utils::options::options_cont::is_set("default.connection.cont-pos") {
            contPos = utils::options::options_cont::get_float("default.connection.cont-pos");
        }
        conn.contPos = attrs.get_opt(SUMO_ATTR_CONTPOS, "", &mut ok, contPos, true);
        // NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE = -1
        conn.visibility = attrs.get_opt(SUMO_ATTR_VISIBILITY_DISTANCE, "", &mut ok, -1.0, true);
        let allow: String = attrs.get_opt(SUMO_ATTR_ALLOW, "", &mut ok, String::new(), false);
        let disallow: String = attrs.get_opt(SUMO_ATTR_DISALLOW, "", &mut ok, String::new(), false);
        if allow.is_empty() && disallow.is_empty() {
            conn.permissions = SVC_UNSPECIFIED;
        } else {
            conn.permissions = parse_vehicle_classes_allow_disallow(&allow, &disallow, self.myNetworkVersion);
        }
        if attrs.has_attribute(SUMO_ATTR_CHANGE_LEFT) {
            let changeLeft: String = attrs.get(SUMO_ATTR_CHANGE_LEFT, "", &mut ok, true);
            conn.changeLeft = parse_vehicle_classes_allow_disallow(&changeLeft, "", NETWORK_VERSION);
        } else {
            conn.changeLeft = SVC_UNSPECIFIED;
        }
        if attrs.has_attribute(SUMO_ATTR_CHANGE_RIGHT) {
            let changeRight: String = attrs.get(SUMO_ATTR_CHANGE_RIGHT, "", &mut ok, true);
            conn.changeRight = parse_vehicle_classes_allow_disallow(&changeRight, "", NETWORK_VERSION);
        } else {
            conn.changeRight = SVC_UNSPECIFIED;
        }
        if self.myChangeLefthand {
            std::mem::swap(&mut conn.changeLeft, &mut conn.changeRight);
        }
        // NBEdge::UNSPECIFIED_SPEED = -1, UNSPECIFIED_FRICTION = 1, UNSPECIFIED_LOADED_LENGTH = -1
        conn.speed = attrs.get_opt(SUMO_ATTR_SPEED, "", &mut ok, -1.0, true);
        conn.friction = attrs.get_opt(SUMO_ATTR_FRICTION, "", &mut ok, 1.0, true);
        conn.customLength = attrs.get_opt(SUMO_ATTR_LENGTH, "", &mut ok, -1.0, true);
        conn.customShape = attrs.get_opt(SUMO_ATTR_SHAPE, "", &mut ok, PositionVector::default(), true);
        NBNetBuilder::transform_coordinates(&mut conn.customShape, false, self.myLocation);
        // NBEdge::UNSPECIFIED_CONNECTION_UNCONTROLLED = false
        conn.uncontrolled = attrs.get_opt(SUMO_ATTR_UNCONTROLLED, "", &mut ok, false, false);
        if !conn.tlID.is_empty() {
            conn.tlLinkIndex = attrs.get(SUMO_ATTR_TLLINKINDEX, "", &mut ok, true);
            conn.tlLinkIndex2 = attrs.get_opt(SUMO_ATTR_TLLINKINDEX2, "", &mut ok, -1, true);
        } else {
            conn.tlLinkIndex = NBCONNECTION_INVALID_TL_INDEX;
            conn.tlLinkIndex2 = NBCONNECTION_INVALID_TL_INDEX;
        }
        if (from.m().lanes.len() as i32) <= fromLaneIdx {
            write_error(&format!("Invalid lane index '{}' for connection from '{}'.", fromLaneIdx, fromID));
            return;
        }
        let lane = from.m().lanes[fromLaneIdx as usize];
        lane.m().connections.push(conn);
        let connPtr: Ptr<NIImporter_SUMO_Connection> = Ptr::from_mut(lane.m().connections.last_mut().unwrap());
        self.myLastParameterised.push(Ptr::from_mut(&mut connPtr.m().base));
        let conn: &NIImporter_SUMO_Connection = connPtr.m();

        // determine crossing priority and tlIndex
        if !self.myPedestrianCrossings.is_empty() {
            if matches!(from.m().func, SumoXMLEdgeFunc::WALKINGAREA) && matches!(self.myEdges[&conn.toEdgeID].m().func, SumoXMLEdgeFunc::CROSSING) {
                // connection from walkingArea to crossing
                let crossings = self.myPedestrianCrossings.entry(get_junction_id_from_internal_edge(&fromID)).or_insert_with(Vec::new);
                for c in crossings.iter_mut() {
                    if conn.toEdgeID == c.edgeID {
                        if !conn.tlID.is_empty() {
                            c.priority = true;
                            c.customTLIndex = conn.tlLinkIndex;
                        } else {
                            let state = LinkStates.get(&attrs.get::<String>(SUMO_ATTR_STATE, "", &mut ok, true)).unwrap();
                            c.priority = state == LINKSTATE_MAJOR;
                        }
                    }
                }
            } else if matches!(from.m().func, SumoXMLEdgeFunc::CROSSING) && matches!(self.myEdges[&conn.toEdgeID].m().func, SumoXMLEdgeFunc::WALKINGAREA) {
                // connection from crossing to walkingArea (set optional linkIndex2)
                let crossings = self.myPedestrianCrossings.entry(get_junction_id_from_internal_edge(&fromID)).or_insert_with(Vec::new);
                for c in crossings.iter_mut() {
                    if fromID == c.edgeID {
                        c.customTLIndex2 = attrs.get_opt(SUMO_ATTR_TLLINKINDEX, "", &mut ok, -1, true);
                    }
                }
            }
        }
        // determine walking area reference edges
        if !self.myWACustomShapes.is_empty() {
            let to = self.myEdges[&conn.toEdgeID];
            if matches!(from.m().func, SumoXMLEdgeFunc::WALKINGAREA) {
                if let Some(it) = self.myWACustomShapes.get_mut(&fromID) {
                    if matches!(to.m().func, SumoXMLEdgeFunc::NORMAL) {
                        // add target sidewalk as reference
                        it.toEdges.push(conn.toEdgeID.clone());
                    } else if matches!(to.m().func, SumoXMLEdgeFunc::CROSSING) {
                        // add target crossing edges as reference
                        let crossings = self.myPedestrianCrossings.entry(get_junction_id_from_internal_edge(&fromID)).or_insert_with(Vec::new);
                        for crossing in crossings.iter() {
                            if conn.toEdgeID == crossing.edgeID {
                                it.toCrossed.extend(crossing.crossingEdges.iter().cloned());
                            }
                        }
                    }
                }
            } else if matches!(to.m().func, SumoXMLEdgeFunc::WALKINGAREA) {
                if let Some(it) = self.myWACustomShapes.get_mut(&conn.toEdgeID) {
                    if matches!(from.m().func, SumoXMLEdgeFunc::NORMAL) {
                        // add origin sidewalk as reference
                        it.fromEdges.push(fromID.clone());
                    } else if matches!(from.m().func, SumoXMLEdgeFunc::CROSSING) {
                        // add origin crossing edges as reference
                        let crossings = self.myPedestrianCrossings.entry(get_junction_id_from_internal_edge(&fromID)).or_insert_with(Vec::new);
                        for crossing in crossings.iter() {
                            if fromID == crossing.edgeID {
                                it.fromCrossed.extend(crossing.crossingEdges.iter().cloned());
                            }
                        }
                    }
                }
            }
        }
    }

    /// C++: `void NIImporter_SUMO::addProhibition(const SUMOSAXAttributes &)` [private]
    /// Parses a prohibition and saves it
    // void
    // NIImporter_SUMO::addProhibition(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string prohibitor = attrs.getOpt<std::string>(SUMO_ATTR_PROHIBITOR, nullptr, ok, "");
    //     std::string prohibited = attrs.getOpt<std::string>(SUMO_ATTR_PROHIBITED, nullptr, ok, "");
    //     if (!ok) {
    //         return;
    //     }
    //     Prohibition p;
    //     parseProhibitionConnection(prohibitor, p.prohibitorFrom, p.prohibitorTo, ok);
    //     parseProhibitionConnection(prohibited, p.prohibitedFrom, p.prohibitedTo, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     myProhibitions.push_back(p);
    // }
    #[allow(unused_variables)]
    pub fn add_prohibition(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let prohibitor: String = attrs.get_opt(SUMO_ATTR_PROHIBITOR, "", &mut ok, String::new(), true);
        let prohibited: String = attrs.get_opt(SUMO_ATTR_PROHIBITED, "", &mut ok, String::new(), true);
        if !ok {
            return;
        }
        let mut p = NIImporter_SUMO_Prohibition {
            prohibitorFrom: String::new(),
            prohibitorTo: String::new(),
            prohibitedFrom: String::new(),
            prohibitedTo: String::new(),
        };
        self.parse_prohibition_connection(&prohibitor, &mut p.prohibitorFrom, &mut p.prohibitorTo, &mut ok);
        self.parse_prohibition_connection(&prohibited, &mut p.prohibitedFrom, &mut p.prohibitedTo, &mut ok);
        if !ok {
            return;
        }
        self.myProhibitions.push(p);
    }

    /// C++: `void NIImporter_SUMO::addRoundabout(const SUMOSAXAttributes &)` [private]
    /// Parses a roundabout and stores it in myEdgeCont.
    // void
    // NIImporter_SUMO::addRoundabout(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::vector<std::string>& edgeIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, nullptr, ok);
    //     if (ok) {
    //         myRoundabouts.push_back(edgeIDs);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_roundabout(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let edgeIDs: Vec<String> = attrs.get(SUMO_ATTR_EDGES, "", &mut ok, true);
        if ok {
            self.myRoundabouts.push(edgeIDs);
        }
    }

    /// C++: `Position NIImporter_SUMO::readPosition(const SUMOSAXAttributes &, const std::string &, bool &)` [static, private]
    /// read position from the given attributes, attribute errors to id
    // Position
    // NIImporter_SUMO::readPosition(const SUMOSAXAttributes& attrs, const std::string& id, bool& ok) {
    //     const double x = attrs.get<double>(SUMO_ATTR_X, id.c_str(), ok);
    //     const double y = attrs.get<double>(SUMO_ATTR_Y, id.c_str(), ok);
    //     const double z = attrs.getOpt<double>(SUMO_ATTR_Z, id.c_str(), ok, 0.);
    //     return Position(x, y, z);
    // }
    #[allow(unused_variables)]
    pub fn read_position(attrs: &SUMOSAXAttributes, id: &str, ok: &mut bool) -> Position {
        let x: f64 = attrs.get(SUMO_ATTR_X, id, ok, true);
        let y: f64 = attrs.get(SUMO_ATTR_Y, id, ok, true);
        let z: f64 = attrs.get_opt(SUMO_ATTR_Z, id, ok, 0.0, true);
        Position::new3(x, y, z)
    }

    /// C++: `void NIImporter_SUMO::parseProhibitionConnection(const std::string &, std::string &, std::string &, bool &)` [private]
    /// parses connection string of a prohibition (very old school)
    // void
    // NIImporter_SUMO::parseProhibitionConnection(const std::string& attr, std::string& from, std::string& to, bool& ok) {
    //     // split from/to
    //     const std::string::size_type div = attr.find("->");
    //     if (div == std::string::npos) {
    //         WRITE_ERRORF(TL("Missing connection divider in prohibition attribute '%'"), attr);
    //         ok = false;
    //     }
    //     from = attr.substr(0, div);
    //     to = attr.substr(div + 2);
    //     // check whether the edges are known
    //     if (myEdges.count(from) == 0) {
    //         WRITE_ERRORF(TL("Unknown edge prohibition '%'"), from);
    //         ok = false;
    //     }
    //     if (myEdges.count(to) == 0) {
    //         WRITE_ERRORF(TL("Unknown edge prohibition '%'"), to);
    //         ok = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_prohibition_connection(&mut self, attr: &str, from: &mut String, to: &mut String, ok: &mut bool) {
        // split from/to
        let div = attr.find("->");
        match div {
            Some(d) => {
                *from = attr[..d].to_string();
                *to = attr[d + 2..].to_string();
            }
            None => {
                write_error(&format!("Missing connection divider in prohibition attribute '{}'", attr));
                *ok = false;
                // C++ substr(npos + 2) wraps around to position 1
                *from = attr.to_string();
                *to = attr.get(1..).unwrap_or("").to_string();
            }
        }
        // check whether the edges are known
        if !self.myEdges.contains_key(from.as_str()) {
            write_error(&format!("Unknown edge prohibition '{}'", from));
            *ok = false;
        }
        if !self.myEdges.contains_key(to.as_str()) {
            write_error(&format!("Unknown edge prohibition '{}'", to));
            *ok = false;
        }
    }

}

/// Describes the values found in a prohibition
/// C++ class `NIImporter_SUMO::Prohibition` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO_Prohibition {
    pub prohibitorFrom: String,
    pub prohibitorTo: String,
    pub prohibitedFrom: String,
    pub prohibitedTo: String,
}

impl NIImporter_SUMO_Prohibition {
}

/// Describes custom shape for a walking area during parsing
/// C++ class `NIImporter_SUMO::WalkingAreaParsedCustomShape` (netimport/NIImporter_SUMO.h)
pub struct NIImporter_SUMO_WalkingAreaParsedCustomShape {
    pub shape: PositionVector,
    pub fromEdges: Vec<String>,
    pub toEdges: Vec<String>,
    pub fromCrossed: Vec<String>,
    pub toCrossed: Vec<String>,
    pub width: f64,
}

impl NIImporter_SUMO_WalkingAreaParsedCustomShape {
}

