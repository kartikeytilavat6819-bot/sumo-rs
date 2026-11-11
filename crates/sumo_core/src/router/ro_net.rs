//! Port of router/RONet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::multimap<const SUMOTime, const std::string> RONet::ContainerMap`
pub type RONet_ContainerMap = BTreeMap<SUMOTime, String>;

/// C++ typedef `std::map<const ROLane *, RouterProhibition> RONet::LaneProhibitions`
pub type RONet_LaneProhibitions = BTreeMap<Ptr<ROLane>, RouterProhibition>;

/// C++ typedef `std::map<const ROEdge *, RouterProhibition> RONet::Prohibitions`
pub type RONet_Prohibitions = BTreeMap<Ptr<ROEdge>, RouterProhibition>;

/// C++ typedef `MapMatcher<ROEdge, ROLane, RONode> ROMapMatcher`
pub type ROMapMatcher = Unmapped /* MapMatcher<ROEdge, ROLane, RONode> */;

/// C++ typedef `std::map<const SUMOTime, std::vector<RORoutable *>> RONet::RoutablesMap`
pub type RONet_RoutablesMap = BTreeMap<SUMOTime, Vec<Ptr<RORoutable>>>;

/// C++ typedef `std::map<std::string, RandomDistributor<SUMOVTypeParameter *> *> RONet::VTypeDistDictType`
pub type RONet_VTypeDistDictType = BTreeMap<String, Ptr<RandomDistributor<Ptr<SUMOVTypeParameter>>>>;

/// The router's network representation.
/// C++ class `RONet` (router/RONet.h)
pub struct RONet {
    /// Known vehicle ids and their departure
    pub myVehIDs: BTreeMap<String, SUMOTime>,
    /// Known person ids
    pub myPersonIDs: BTreeSet<String>,
    /// Known nodes
    pub myNodes: Unmapped /* NamedObjectCont<RONode *> */,
    /// Known edges
    pub myEdges: Unmapped /* NamedObjectCont<ROEdge *> */,
    /// Known bus / train / container stops and parking areas
    pub myStoppingPlaces: BTreeMap<SumoXMLTag, Unmapped /* NamedObjectCont<SUMOVehicleParameter::Stop *> */>,
    /// Known vehicle types
    pub myVehicleTypes: Unmapped /* NamedObjectCont<SUMOVTypeParameter *> */,
    /// A distribution of vehicle types (probability->vehicle type)
    pub myVTypeDistDict: RONet_VTypeDistDictType,
    /// Whether the default vehicle type was already used or can still be replaced
    pub myDefaultVTypeMayBeDeleted: bool,
    /// Whether the default pedestrian type was already used or can still be replaced
    pub myDefaultPedTypeMayBeDeleted: bool,
    /// Whether the default bicycle type was already used or can still be replaced
    pub myDefaultBikeTypeMayBeDeleted: bool,
    /// Whether the default taxi type was already used or can still be replaced
    pub myDefaultTaxiTypeMayBeDeleted: bool,
    /// Whether the default rail type was already used or can still be replaced
    pub myDefaultRailTypeMayBeDeleted: bool,
    /// Known routes
    pub myRoutes: Unmapped /* NamedObjectCont<RORouteDef *> */,
    /// Known routables
    pub myRoutables: RONet_RoutablesMap,
    /// Known flows
    pub myFlows: Unmapped /* NamedObjectCont<SUMOVehicleParameter *> */,
    /// whether any flows are still active
    pub myHaveActiveFlows: bool,
    pub myContainers: RONet_ContainerMap,
    /// vehicles to keep for public transport routing
    pub myPTVehicles: Vec<Ptr<RORoutable>>,
    /// Departure times for randomized flows
    pub myDepartures: BTreeMap<String, Vec<SUMOTime>>,
    /// traffic assignment zones with sources and sinks
    pub myDistricts: BTreeMap<String, (Vec<String>, Vec<String>)>, // C++: std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string>>>
    /// The file to write the computed routes into
    pub myRoutesOutput: Ptr<OutputDevice>,
    /// The file to write the computed route alternatives into
    pub myRouteAlternativesOutput: Ptr<OutputDevice>,
    /// The file to write the vehicle types into
    pub myTypesOutput: Ptr<OutputDevice>,
    /// The number of read routes
    pub myReadRouteNo: i32,
    /// The number of discarded routes
    pub myDiscardedRouteNo: i32,
    /// The number of written routes
    pub myWrittenRouteNo: i32,
    /// Whether traffic was excluded by the begin time or processed for output.
    pub myHaveSkippedRoutes: bool,
    pub myHaveProcessedRoutes: bool,
    /// Whether the network contains edges which not all vehicles may pass
    pub myHavePermissions: bool,
    /// The vehicle class specific speed restrictions
    pub mySpeedRestrictions: BTreeMap<String, BTreeMap<SUMOVehicleClass, f64>>, // C++: std::map<std::string, std::map<SUMOVehicleClass, double>>
    /// whether parameter-based access restrictions are configured
    pub myHaveParamRestrictions: bool,
    /// Preferences for routing
    pub myVClassPreferences: BTreeMap<SUMOVehicleClass, BTreeMap<String, f64>>, // C++: std::map<SUMOVehicleClass, std::map<std::string, double>>
    pub myVTypePreferences: BTreeMap<String, BTreeMap<String, f64>>, // C++: std::map<std::string, std::map<std::string, double>>
    /// The number of internal edges in the dictionary
    pub myNumInternalEdges: i32,
    /// handler for ignorable error messages
    pub myErrorHandler: Ptr<MsgHandler>,
    /// whether to keep the vtype distribution in output
    pub myKeepVTypeDist: bool,
    /// whether to calculate routes for public transport
    pub myDoPTRouting: bool,
    /// whether to preserve flows
    pub myKeepFlows: bool,
    /// whether the network contains bidirectional railway edges
    pub myHasBidiEdges: bool,
    /// the maximum traveltime beyond which routing is considered a failure
    pub myMaxTraveltime: f64,
    /// temporary edge closing (rerouters)
    pub myProhibitions: SUMOAbstractRouter_Prohibitions,
    /// temporary lane closing (rerouters)
    pub myLaneProhibitions: RONet_LaneProhibitions,
    pub myLaneProhibitionTimes: BTreeMap<f64, BTreeSet<Ptr<ROLane>>>, // C++: std::map<double, std::set<const ROLane *>>
}

// C++ static members of RONet (port as module-level statics, see AGENTS.md):
//   static RONet * RONet::myInstance;

impl RONet {
    /// C++: `void RONet::RONet()`
    /// Constructor
    // RONet::RONet() :
    //     myVehicleTypes(), myDefaultVTypeMayBeDeleted(true),
    //     myDefaultPedTypeMayBeDeleted(true),
    //     myDefaultBikeTypeMayBeDeleted(true),
    //     myDefaultTaxiTypeMayBeDeleted(true),
    //     myDefaultRailTypeMayBeDeleted(true),
    //     myHaveActiveFlows(true),
    //     myRoutesOutput(nullptr), myRouteAlternativesOutput(nullptr), myTypesOutput(nullptr),
    //     myReadRouteNo(0), myDiscardedRouteNo(0), myWrittenRouteNo(0),
    //     myHavePermissions(false),
    //     myHaveParamRestrictions(false),
    //     myNumInternalEdges(0),
    //     myErrorHandler(OptionsCont::getOptions().exists("ignore-errors")
    //                    && OptionsCont::getOptions().getBool("ignore-errors") ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //     myKeepVTypeDist(OptionsCont::getOptions().exists("keep-vtype-distributions")
    //                     && OptionsCont::getOptions().getBool("keep-vtype-distributions")),
    //     myDoPTRouting(!OptionsCont::getOptions().exists("ptline-routing")
    //                   || OptionsCont::getOptions().getBool("ptline-routing")),
    //     myKeepFlows(OptionsCont::getOptions().exists("keep-flows")
    //                 && OptionsCont::getOptions().getBool("keep-flows")),
    //     myHasBidiEdges(false),
    //     myMaxTraveltime(OptionsCont::getOptions().exists("max-traveltime") ? STEPS2TIME(string2time(OptionsCont::getOptions().getString("max-traveltime"))) : -1) {
    //     if (myInstance != nullptr) {
    //         throw ProcessError(TL("A network was already constructed."));
    //     }
    //     SUMOVTypeParameter* type = new SUMOVTypeParameter(DEFAULT_VTYPE_ID, SVC_PASSENGER);
    //     type->onlyReferenced = true;
    //     myVehicleTypes.add(type->id, type);
    //
    //     SUMOVTypeParameter* defPedType = new SUMOVTypeParameter(DEFAULT_PEDTYPE_ID, SVC_PEDESTRIAN);
    //     defPedType->onlyReferenced = true;
    //     defPedType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defPedType->id, defPedType);
    //
    //     SUMOVTypeParameter* defBikeType = new SUMOVTypeParameter(DEFAULT_BIKETYPE_ID, SVC_BICYCLE);
    //     defBikeType->onlyReferenced = true;
    //     defBikeType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defBikeType->id, defBikeType);
    //
    //     SUMOVTypeParameter* defTaxiType = new SUMOVTypeParameter(DEFAULT_TAXITYPE_ID, SVC_TAXI);
    //     defTaxiType->onlyReferenced = true;
    //     defTaxiType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defTaxiType->id, defTaxiType);
    //
    //     SUMOVTypeParameter* defRailType = new SUMOVTypeParameter(DEFAULT_RAILTYPE_ID, SVC_RAIL);
    //     defRailType->onlyReferenced = true;
    //     defRailType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defRailType->id, defRailType);
    //
    //     myInstance = this;
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port RONet::RONet")
    }

    /// C++: `RONet * RONet::getInstance()` [static]
    /// Returns the pointer to the unique instance of RONet (singleton).
    // RONet*
    // RONet::getInstance(void) {
    //     if (myInstance != nullptr) {
    //         return myInstance;
    //     }
    //     throw ProcessError(TL("A network was not yet constructed."));
    // }
    #[allow(unused_variables)]
    pub fn get_instance() -> Ptr<RONet> {
        todo!("port RONet::getInstance")
    }

    /// C++: `void RONet::~RONet()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RONet::~RONet")
    }

    /// C++: `void RONet::addSpeedRestriction(const std::string &, const SUMOVehicleClass, const double)`
    /// Adds a restriction for an edge type
    // void
    // RONet::addSpeedRestriction(const std::string& id, const SUMOVehicleClass svc, const double speed) {
    //     mySpeedRestrictions[id][svc] = speed;
    // }
    #[allow(unused_variables)]
    pub fn add_speed_restriction(&mut self, id: &str, svc: SUMOVehicleClass, speed: f64) {
        todo!("port RONet::addSpeedRestriction")
    }

    /// C++: `const std::map<SUMOVehicleClass, double> * RONet::getRestrictions(const std::string &) const` [const]
    /// Returns the restrictions for an edge type If no restrictions are present, 0 is returned.
    // const std::map<SUMOVehicleClass, double>*
    // RONet::getRestrictions(const std::string& id) const {
    //     std::map<std::string, std::map<SUMOVehicleClass, double> >::const_iterator i = mySpeedRestrictions.find(id);
    //     if (i == mySpeedRestrictions.end()) {
    //         return nullptr;
    //     }
    //     return &i->second;
    // }
    #[allow(unused_variables)]
    pub fn get_restrictions(&self, id: &str) -> Ptr<BTreeMap<SUMOVehicleClass, f64>> {
        todo!("port RONet::getRestrictions")
    }

    /// C++: `bool RONet::hasSpeedRestrictions() const` [const]
    // bool hasSpeedRestrictions() const {
    //         return !mySpeedRestrictions.empty();
    //     }
    #[allow(unused_variables)]
    pub fn has_speed_restrictions(&self) -> bool {
        todo!("port RONet::hasSpeedRestrictions")
    }

    /// C++: `bool RONet::hasParamRestrictions() const` [const]
    // bool hasParamRestrictions() const {
    //         return myHaveParamRestrictions;
    //     }
    #[allow(unused_variables)]
    pub fn has_param_restrictions(&self) -> bool {
        todo!("port RONet::hasParamRestrictions")
    }

    /// C++: `void RONet::setParamRestrictions()`
    // void setParamRestrictions() {
    //         myHaveParamRestrictions = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_param_restrictions(&mut self) {
        todo!("port RONet::setParamRestrictions")
    }

    /// C++: `double RONet::getPreference(const std::string &, const SUMOVTypeParameter &) const` [const]
    /// retriefe edge type specific routing preference
    // double
    // RONet::getPreference(const std::string& routingType, const SUMOVTypeParameter& pars) const {
    //     if (gRoutingPreferences) {
    //         auto it = myVTypePreferences.find(pars.id);
    //         if (it != myVTypePreferences.end()) {
    //             auto it2 = it->second.find(routingType);
    //             if (it2 != it->second.end()) {
    //                 return it2->second;
    //             }
    //         }
    //         auto it3 = myVClassPreferences.find(pars.vehicleClass);
    //         if (it3 != myVClassPreferences.end()) {
    //             auto it4 = it3->second.find(routingType);
    //             if (it4 != it3->second.end()) {
    //                 return it4->second;
    //             }
    //         }
    //         // fallback to generel preferences
    //         it = myVTypePreferences.find("");
    //         if (it != myVTypePreferences.end()) {
    //             auto it2 = it->second.find(routingType);
    //             if (it2 != it->second.end()) {
    //                 return it2->second;
    //             }
    //         }
    //     }
    //     return 1;
    // }
    #[allow(unused_variables)]
    pub fn get_preference(&self, routingType: &str, pars: &SUMOVTypeParameter) -> f64 {
        todo!("port RONet::getPreference")
    }

    /// C++: `void RONet::addPreference(const std::string &, SUMOVehicleClass, double)`
    /// add edge type specific routing preference
    // void
    // RONet::addPreference(const std::string& routingType, SUMOVehicleClass svc, double prio) {
    //     myVClassPreferences[svc][routingType] = prio;
    //     gRoutingPreferences = true;
    // }
    #[allow(unused_variables)]
    pub fn add_preference(&mut self, routingType: &str, svc: SUMOVehicleClass, prio: f64) {
        todo!("port RONet::addPreference")
    }

    /// C++: `void RONet::addPreference(const std::string &, std::string, double)`
    /// add edge type specific routing preference
    // void
    // RONet::addPreference(const std::string& routingType, std::string vType, double prio) {
    //     myVTypePreferences[vType][routingType] = prio;
    //     gRoutingPreferences = true;
    // }
    #[allow(unused_variables)]
    pub fn add_preference_2(&mut self, routingType: &str, vType: String, prio: f64) {
        todo!("port RONet::addPreference")
    }

    /// C++: `bool RONet::addEdge(ROEdge *)` [virtual]
    // bool
    // RONet::addEdge(ROEdge* edge) {
    //     if (!myEdges.add(edge->getID(), edge)) {
    //         WRITE_ERRORF(TL("The edge '%' occurs at least twice."), edge->getID());
    //         delete edge;
    //         return false;
    //     }
    //     if (edge->isInternal()) {
    //         myNumInternalEdges += 1;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, edge: Ptr<ROEdge>) -> bool {
        todo!("port RONet::addEdge")
    }

    /// C++: `bool RONet::addDistrict(const std::string, ROEdge *, ROEdge *)`
    // bool
    // RONet::addDistrict(const std::string id, ROEdge* source, ROEdge* sink) {
    //     if (myDistricts.count(id) > 0) {
    //         WRITE_ERRORF(TL("The TAZ '%' occurs at least twice."), id);
    //         delete source;
    //         delete sink;
    //         return false;
    //     }
    //     sink->setFunction(SumoXMLEdgeFunc::CONNECTOR);
    //     if (!addEdge(sink)) {
    //         return false;
    //     }
    //     source->setFunction(SumoXMLEdgeFunc::CONNECTOR);
    //     if (!addEdge(source)) {
    //         return false;
    //     }
    //     sink->setOtherTazConnector(source);
    //     source->setOtherTazConnector(sink);
    //     myDistricts[id] = std::make_pair(std::vector<std::string>(), std::vector<std::string>());
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_district(&mut self, id: String, source: Ptr<ROEdge>, sink: Ptr<ROEdge>) -> bool {
        todo!("port RONet::addDistrict")
    }

    /// C++: `bool RONet::addDistrictEdge(const std::string, const std::string, const bool)`
    // bool
    // RONet::addDistrictEdge(const std::string tazID, const std::string edgeID, const bool isSource) {
    //     if (myDistricts.count(tazID) == 0) {
    //         WRITE_ERRORF(TL("The TAZ '%' is unknown."), tazID);
    //         return false;
    //     }
    //     ROEdge* edge = getEdge(edgeID);
    //     if (edge == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' for TAZ '%' is unknown."), edgeID, tazID);
    //         return false;
    //     }
    //     if (isSource) {
    //         getEdge(tazID + "-source")->addSuccessor(edge);
    //         myDistricts[tazID].first.push_back(edgeID);
    //     } else {
    //         edge->addSuccessor(getEdge(tazID + "-sink"));
    //         myDistricts[tazID].second.push_back(edgeID);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_district_edge(&mut self, tazID: String, edgeID: String, isSource: bool) -> bool {
        todo!("port RONet::addDistrictEdge")
    }

    /// C++: `void RONet::addJunctionTaz(ROAbstractEdgeBuilder &)`
    /// add a taz for every junction unless a taz with the same id already exists
    // void
    // RONet::addJunctionTaz(ROAbstractEdgeBuilder& eb) {
    //     for (auto item : myNodes) {
    //         const std::string tazID = item.first;
    //         if (myDistricts.count(tazID) != 0) {
    //             WRITE_WARNINGF(TL("A TAZ with id '%' already exists. Not building junction TAZ."), tazID);
    //             continue;
    //         }
    //         const std::string sourceID = tazID + "-source";
    //         const std::string sinkID = tazID + "-sink";
    //         // sink must be added before source
    //         ROEdge* sink = eb.buildEdge(sinkID, nullptr, nullptr, 0, "", "");
    //         ROEdge* source = eb.buildEdge(sourceID, nullptr, nullptr, 0, "", "");
    //         sink->setOtherTazConnector(source);
    //         source->setOtherTazConnector(sink);
    //         if (!addDistrict(tazID, source, sink)) {
    //             continue;
    //         }
    //         auto& district = myDistricts[tazID];
    //         const RONode* junction = item.second;
    //         for (const ROEdge* edge : junction->getIncoming()) {
    //             if (!edge->isInternal()) {
    //                 const_cast<ROEdge*>(edge)->addSuccessor(sink);
    //                 district.second.push_back(edge->getID());
    //             }
    //         }
    //         for (const ROEdge* edge : junction->getOutgoing()) {
    //             if (!edge->isInternal()) {
    //                 source->addSuccessor(const_cast<ROEdge*>(edge));
    //                 district.first.push_back(edge->getID());
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_junction_taz(&mut self, eb: &mut ROAbstractEdgeBuilder) {
        todo!("port RONet::addJunctionTaz")
    }

    /// C++: `void RONet::setBidiEdges(const std::map<ROEdge *, std::string> &)`
    /// add a taz for every junction unless a taz with the same id already exists
    // void
    // RONet::setBidiEdges(const std::map<ROEdge*, std::string>& bidiMap) {
    //     for (const auto& item : bidiMap) {
    //         ROEdge* bidi = myEdges.get(item.second);
    //         if (bidi == nullptr) {
    //             WRITE_ERRORF(TL("The bidi edge '%' is not known."), item.second);
    //         }
    //         item.first->setBidiEdge(bidi);
    //         myHasBidiEdges = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_bidi_edges(&mut self, bidiMap: &BTreeMap<Ptr<ROEdge>, String>) {
        todo!("port RONet::setBidiEdges")
    }

    /// C++: `const std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string>>> & RONet::getDistricts() const` [const]
    /// Retrieves all TAZ (districts) from the network
    // const std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string> > >& getDistricts() const {
    //         return myDistricts;
    //     }
    #[allow(unused_variables)]
    pub fn get_districts(&self) -> &BTreeMap<String, (Vec<String>, Vec<String>)> {
        todo!("port RONet::getDistricts")
    }

    /// C++: `ROEdge * RONet::getEdge(const std::string &) const` [const]
    /// Retrieves an edge from the network
    // ROEdge* getEdge(const std::string& name) const {
    //         return myEdges.get(name);
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self, name: &str) -> Ptr<ROEdge> {
        todo!("port RONet::getEdge")
    }

    /// C++: `ROEdge * RONet::getEdgeForLaneID(const std::string &) const` [const]
    /// Retrieves an edge from the network when the lane id is given
    // ROEdge*
    // RONet::getEdgeForLaneID(const std::string& laneID) const {
    //     return getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(laneID));
    // }
    #[allow(unused_variables)]
    pub fn get_edge_for_lane_id(&self, laneID: &str) -> Ptr<ROEdge> {
        todo!("port RONet::getEdgeForLaneID")
    }

    /// C++: `ROLane * RONet::getLane(const std::string &) const` [const]
    /// Retrieves a lane rom the network given its id
    // ROLane*
    // RONet::getLane(const std::string& laneID) const {
    //     int laneIndex = SUMOXMLDefinitions::getIndexFromLane(laneID);
    //     return getEdgeForLaneID(laneID)->getLanes()[laneIndex];
    // }
    #[allow(unused_variables)]
    pub fn get_lane(&self, laneID: &str) -> Ptr<ROLane> {
        todo!("port RONet::getLane")
    }

    /// C++: `void RONet::addNode(RONode *)`
    // void
    // RONet::addNode(RONode* node) {
    //     if (!myNodes.add(node->getID(), node)) {
    //         WRITE_ERRORF(TL("The node '%' occurs at least twice."), node->getID());
    //         delete node;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_node(&mut self, node: Ptr<RONode>) {
        todo!("port RONet::addNode")
    }

    /// C++: `RONode * RONet::getNode(const std::string &) const` [const]
    /// Retrieves an node from the network
    // RONode* getNode(const std::string& id) const {
    //         return myNodes.get(id);
    //     }
    #[allow(unused_variables)]
    pub fn get_node(&self, id: &str) -> Ptr<RONode> {
        todo!("port RONet::getNode")
    }

    /// C++: `void RONet::addStoppingPlace(const std::string &, const SumoXMLTag, SUMOVehicleParameter::Stop *)`
    // void
    // RONet::addStoppingPlace(const std::string& id, const SumoXMLTag category, SUMOVehicleParameter::Stop* stop) {
    //     if (!myStoppingPlaces[category == SUMO_TAG_TRAIN_STOP ? SUMO_TAG_BUS_STOP : category].add(id, stop)) {
    //         WRITE_ERRORF(TL("The % '%' occurs at least twice."), toString(category), id);
    //         delete stop;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_stopping_place(&mut self, id: &str, category: SumoXMLTag, stop: Ptr<SUMOVehicleParameter_Stop>) {
        todo!("port RONet::addStoppingPlace")
    }

    /// C++: `const SUMOVehicleParameter::Stop * RONet::getStoppingPlace(const std::string &, const SumoXMLTag) const` [const]
    /// Retrieves a stopping place from the network
    // const SUMOVehicleParameter::Stop* getStoppingPlace(const std::string& id, const SumoXMLTag category) const {
    //         if (myStoppingPlaces.count(category) > 0) {
    //             return myStoppingPlaces.find(category)->second.get(id);
    //         }
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_stopping_place(&self, id: &str, category: SumoXMLTag) -> Ptr<SUMOVehicleParameter_Stop> {
        todo!("port RONet::getStoppingPlace")
    }

    /// C++: `const std::string RONet::getStoppingPlaceName(const std::string &) const` [const]
    /// return the name for the given stopping place id
    // const std::string
    // RONet::getStoppingPlaceName(const std::string& id) const {
    //     for (const auto& mapItem : myStoppingPlaces) {
    //         SUMOVehicleParameter::Stop* stop = mapItem.second.get(id);
    //         if (stop != nullptr) {
    //             // see RONetHandler::parseStoppingPlace
    //             return stop->busstop;
    //         }
    //     }
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_name(&self, id: &str) -> String {
        todo!("port RONet::getStoppingPlaceName")
    }

    /// C++: `const std::string RONet::getStoppingPlaceElement(const std::string &) const` [const]
    /// return the element name for the given stopping place id
    // const std::string
    // RONet::getStoppingPlaceElement(const std::string& id) const {
    //     for (const auto& mapItem : myStoppingPlaces) {
    //         SUMOVehicleParameter::Stop* stop = mapItem.second.get(id);
    //         if (stop != nullptr) {
    //             // see RONetHandler::parseStoppingPlace
    //             return stop->actType;
    //         }
    //     }
    //     return toString(SUMO_TAG_BUS_STOP);
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_element(&self, id: &str) -> String {
        todo!("port RONet::getStoppingPlaceElement")
    }

    /// C++: `bool RONet::checkVType(const std::string &)`
    /// Checks whether the vehicle type (distribution) may be added
    // bool
    // RONet::checkVType(const std::string& id) {
    //     if (id == DEFAULT_VTYPE_ID) {
    //         if (myDefaultVTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultVTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_PEDTYPE_ID) {
    //         if (myDefaultPedTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultPedTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_BIKETYPE_ID) {
    //         if (myDefaultBikeTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultBikeTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_TAXITYPE_ID) {
    //         if (myDefaultTaxiTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultTaxiTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_RAILTYPE_ID) {
    //         if (myDefaultRailTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultRailTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else {
    //         if (myVehicleTypes.get(id) != 0 || myVTypeDistDict.find(id) != myVTypeDistDict.end()) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_v_type(&mut self, id: &str) -> bool {
        todo!("port RONet::checkVType")
    }

    /// C++: `bool RONet::addVehicleType(SUMOVTypeParameter *)` [virtual]
    /// Adds a read vehicle type definition to the network
    // bool
    // RONet::addVehicleType(SUMOVTypeParameter* type) {
    //     if (checkVType(type->id)) {
    //         myVehicleTypes.add(type->id, type);
    //     } else {
    //         WRITE_ERRORF(TL("The vehicle type '%' occurs at least twice."), type->id);
    //         delete type;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle_type(&mut self, type_: Ptr<SUMOVTypeParameter>) -> bool {
        todo!("port RONet::addVehicleType")
    }

    /// C++: `bool RONet::addVTypeDistribution(const std::string &, RandomDistributor<SUMOVTypeParameter *> *)`
    /// Adds a vehicle type distribution
    // bool
    // RONet::addVTypeDistribution(const std::string& id, RandomDistributor<SUMOVTypeParameter*>* vehTypeDistribution) {
    //     if (checkVType(id)) {
    //         myVTypeDistDict[id] = vehTypeDistribution;
    //         return true;
    //     }
    //     delete vehTypeDistribution;
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_v_type_distribution(&mut self, id: &str, vehTypeDistribution: Ptr<RandomDistributor<Ptr<SUMOVTypeParameter>>>) -> bool {
        todo!("port RONet::addVTypeDistribution")
    }

    /// C++: `const RandomDistributor<SUMOVTypeParameter *> * RONet::getVTypeDistribution(const std::string &)`
    /// Retrieves the named vehicle type distribution
    // const RandomDistributor<SUMOVTypeParameter*>* getVTypeDistribution(const std::string& id) {
    //         const auto it = myVTypeDistDict.find(id);
    //         return it != myVTypeDistDict.end() ? it ->second : nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_distribution(&mut self, id: &str) -> Ptr<RandomDistributor<Ptr<SUMOVTypeParameter>>> {
        todo!("port RONet::getVTypeDistribution")
    }

    /// C++: `SUMOVTypeParameter * RONet::getVehicleTypeSecure(const std::string &)`
    /// Retrieves the named vehicle type
    // SUMOVTypeParameter*
    // RONet::getVehicleTypeSecure(const std::string& id) {
    //     // check whether the type was already known
    //     SUMOVTypeParameter* type = myVehicleTypes.get(id);
    //     if (id == DEFAULT_VTYPE_ID) {
    //         myDefaultVTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_PEDTYPE_ID) {
    //         myDefaultPedTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_BIKETYPE_ID) {
    //         myDefaultBikeTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_TAXITYPE_ID) {
    //         myDefaultTaxiTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_RAILTYPE_ID) {
    //         myDefaultRailTypeMayBeDeleted = false;
    //     }
    //     if (type != nullptr) {
    //         return type;
    //     }
    //     VTypeDistDictType::iterator it2 = myVTypeDistDict.find(id);
    //     if (it2 != myVTypeDistDict.end()) {
    //         return it2->second->get();
    //     }
    //     if (id == "") {
    //         // ok, no vehicle type or an unknown type was given within the user input
    //         //  return the default type
    //         myDefaultVTypeMayBeDeleted = false;
    //         return myVehicleTypes.get(DEFAULT_VTYPE_ID);
    //     }
    //     return type;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_type_secure(&mut self, id: &str) -> Ptr<SUMOVTypeParameter> {
        todo!("port RONet::getVehicleTypeSecure")
    }

    /// C++: `bool RONet::addRouteDef(RORouteDef *)`
    // bool
    // RONet::addRouteDef(RORouteDef* def) {
    //     return myRoutes.add(def->getID(), def);
    // }
    #[allow(unused_variables)]
    pub fn add_route_def(&mut self, def: Ptr<RORouteDef>) -> bool {
        todo!("port RONet::addRouteDef")
    }

    /// C++: `RORouteDef * RONet::getRouteDef(const std::string &) const` [const]
    /// Returns the named route definition
    // RORouteDef* getRouteDef(const std::string& name) const {
    //         return myRoutes.get(name);
    //     }
    #[allow(unused_variables)]
    pub fn get_route_def(&self, name: &str) -> Ptr<RORouteDef> {
        todo!("port RONet::getRouteDef")
    }

    /// C++: `bool RONet::addVehicle(const std::string &, ROVehicle *)` [virtual]
    // bool
    // RONet::addVehicle(const std::string& id, ROVehicle* veh) {
    //     if (myVehIDs.find(id) == myVehIDs.end()) {
    //         myVehIDs[id] = veh->getParameter().departProcedure == DepartDefinition::TRIGGERED ? -1 : veh->getDepartureTime();
    //
    //         if (veh->isPublicTransport()) {
    //             if (!veh->isPartOfFlow()) {
    //                 myPTVehicles.push_back(veh);
    //             }
    //             if (!myDoPTRouting) {
    //                 return true;
    //             }
    //         }
    //         myRoutables[veh->getDepart()].push_back(veh);
    //         return true;
    //     }
    //     WRITE_ERRORF(TL("Another vehicle with the id '%' exists."), id);
    //     delete veh;
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle(&mut self, id: &str, veh: Ptr<ROVehicle>) -> bool {
        todo!("port RONet::addVehicle")
    }

    /// C++: `bool RONet::knowsVehicle(const std::string &) const` [const]
    /// returns whether a vehicle with the given id was already loaded
    // bool
    // RONet::knowsVehicle(const std::string& id) const {
    //     return myVehIDs.find(id) != myVehIDs.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_vehicle(&self, id: &str) -> bool {
        todo!("port RONet::knowsVehicle")
    }

    /// C++: `bool RONet::knowsVType(const std::string &) const` [const]
    /// returns whether a vehicle type or distribution with the given id was already loaded
    // bool
    // RONet::knowsVType(const std::string& id) const {
    //     return myVehicleTypes.get(id) != nullptr || myVTypeDistDict.find(id) != myVTypeDistDict.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_v_type(&self, id: &str) -> bool {
        todo!("port RONet::knowsVType")
    }

    /// C++: `SUMOTime RONet::getDeparture(const std::string &) const` [const]
    /// returns departure time for the given vehicle id
    // SUMOTime
    // RONet::getDeparture(const std::string& vehID) const {
    //     auto it = myVehIDs.find(vehID);
    //     if (it != myVehIDs.end()) {
    //         return it->second;
    //     } else {
    //         throw ProcessError(TLF("Requesting departure time for unknown vehicle '%'", vehID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_departure(&self, vehID: &str) -> SUMOTime {
        todo!("port RONet::getDeparture")
    }

    /// C++: `bool RONet::addFlow(SUMOVehicleParameter *, const bool)`
    // bool
    // RONet::addFlow(SUMOVehicleParameter* flow, const bool randomize) {
    //     if (randomize && flow->repetitionOffset >= 0) {
    //         myDepartures[flow->id].reserve(flow->repetitionNumber);
    //         for (int i = 0; i < flow->repetitionNumber; ++i) {
    //             myDepartures[flow->id].push_back(flow->depart + RandHelper::rand(flow->repetitionNumber * flow->repetitionOffset));
    //         }
    //         std::sort(myDepartures[flow->id].begin(), myDepartures[flow->id].end());
    //         std::reverse(myDepartures[flow->id].begin(), myDepartures[flow->id].end());
    //     } else if (flow->repetitionOffset < 0) {
    //         // init poisson flow (but only the timing)
    //         flow->incrementFlow(1);
    //         flow->repetitionsDone--;
    //     }
    //     const bool added = myFlows.add(flow->id, flow);
    //     if (added) {
    //         myHaveActiveFlows = true;
    //     }
    //     return added;
    // }
    #[allow(unused_variables)]
    pub fn add_flow(&mut self, flow: Ptr<SUMOVehicleParameter>, randomize: bool) -> bool {
        todo!("port RONet::addFlow")
    }

    /// C++: `bool RONet::addPerson(ROPerson *)`
    // bool
    // RONet::addPerson(ROPerson* person) {
    //     if (myPersonIDs.count(person->getID()) == 0) {
    //         myPersonIDs.insert(person->getID());
    //         myRoutables[person->getDepart()].push_back(person);
    //         return true;
    //     }
    //     WRITE_ERRORF(TL("Another person with the id '%' exists."), person->getID());
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_person(&mut self, person: Ptr<ROPerson>) -> bool {
        todo!("port RONet::addPerson")
    }

    /// C++: `void RONet::addContainer(const SUMOTime, const std::string)`
    // void
    // RONet::addContainer(const SUMOTime depart, const std::string desc) {
    //     myContainers.insert(std::pair<const SUMOTime, const std::string>(depart, desc));
    // }
    #[allow(unused_variables)]
    pub fn add_container(&mut self, depart: SUMOTime, desc: String) {
        todo!("port RONet::addContainer")
    }

    /// C++: `SUMOTime RONet::saveAndRemoveRoutesUntil(OptionsCont &, const RORouterProvider &, SUMOTime)`
    /// Computes routes described by their definitions and saves them
    // SUMOTime
    // RONet::saveAndRemoveRoutesUntil(OptionsCont& options, const RORouterProvider& provider,
    //                                 SUMOTime time) {
    //     MsgHandler* mh = (options.getBool("ignore-errors") ?
    //                       MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance());
    //     if (myHaveActiveFlows) {
    //         checkFlows(time, mh);
    //     }
    //     SUMOTime lastTime = -1;
    //     const bool removeLoops = options.getBool("remove-loops");
    // #ifdef HAVE_FOX
    //     const int maxNumThreads = options.getInt("routing-threads");
    // #endif
    //     if (myRoutables.size() != 0) {
    //         if (options.getBool("bulk-routing")) {
    // #ifdef HAVE_FOX
    //             while ((int)myThreadPool.size() < maxNumThreads) {
    //                 new WorkerThread(myThreadPool, provider);
    //             }
    // #endif
    //             createBulkRouteRequests(provider, time, removeLoops);
    //         } else {
    //             for (RoutablesMap::const_iterator i = myRoutables.begin(); i != myRoutables.end(); ++i) {
    //                 if (i->first >= time) {
    //                     break;
    //                 }
    //                 for (RORoutable* const routable : i->second) {
    // #ifdef HAVE_FOX
    //                     // add task
    //                     if (maxNumThreads > 0) {
    //                         const int numThreads = (int)myThreadPool.size();
    //                         if (numThreads == 0) {
    //                             // This is the very first routing. Since at least the CHRouter needs initialization
    //                             // before it gets cloned, we do not do this in parallel
    //                             routable->computeRoute(provider, removeLoops, myErrorHandler);
    //                             new WorkerThread(myThreadPool, provider);
    //                         } else {
    //                             // add thread if necessary
    //                             if (numThreads < maxNumThreads && myThreadPool.isFull()) {
    //                                 new WorkerThread(myThreadPool, provider);
    //                             }
    //                             myThreadPool.add(new RoutingTask(routable, removeLoops, myErrorHandler));
    //                         }
    //                         continue;
    //                     }
    // #endif
    //                     routable->computeRoute(provider, removeLoops, myErrorHandler);
    //                 }
    //             }
    //         }
    // #ifdef HAVE_FOX
    //         myThreadPool.waitAll();
    // #endif
    //     }
    //     const double scale = options.exists("scale-suffix") ? options.getFloat("scale") : 1;
    //     // write all vehicles (and additional structures)
    //     while (myRoutables.size() != 0 || myContainers.size() != 0) {
    //         // get the next vehicle, person or container
    //         RoutablesMap::iterator routables = myRoutables.begin();
    //         const SUMOTime routableTime = routables == myRoutables.end() ? SUMOTime_MAX : routables->first;
    //         ContainerMap::iterator container = myContainers.begin();
    //         const SUMOTime containerTime = container == myContainers.end() ? SUMOTime_MAX : container->first;
    //         // check whether it shall not yet be computed
    //         if (routableTime >= time && containerTime >= time) {
    //             lastTime = MIN2(routableTime, containerTime);
    //             break;
    //         }
    //         const SUMOTime minTime = MIN2(routableTime, containerTime);
    //         myHaveProcessedRoutes = true;
    //         if (routableTime == minTime) {
    //             // check whether to print the output
    //             if (lastTime != routableTime && lastTime != -1) {
    //                 // report writing progress
    //                 if (options.getInt("stats-period") >= 0 && ((int)routableTime % options.getInt("stats-period")) == 0) {
    //                     WRITE_MESSAGE("Read: " + toString(myVehIDs.size()) + ",  Discarded: " + toString(myDiscardedRouteNo) + ",  Written: " + toString(myWrittenRouteNo));
    //                 }
    //             }
    //             lastTime = routableTime;
    //             for (const RORoutable* const r : routables->second) {
    //                 // ok, check whether it has been routed
    //                 if (r->getRoutingSuccess()) {
    //                     // write the route
    //                     int quota = getScalingQuota(scale, myWrittenRouteNo);
    //                     r->write(myRoutesOutput, myRouteAlternativesOutput, myTypesOutput, options, quota);
    //                     myWrittenRouteNo++;
    //                 } else {
    //                     myDiscardedRouteNo++;
    //                 }
    //                 // we need to keep individual public transport vehicles but not the flows
    //                 if (!r->isPublicTransport() || r->isPartOfFlow()) {
    //                     // delete routes and the vehicle
    //                     const ROVehicle* const veh = dynamic_cast<const ROVehicle*>(r);
    //                     if (veh != nullptr && veh->getRouteDefinition()->getID()[0] == '!') {
    //                         if (r->isPartOfFlow() || !myRoutes.remove(veh->getRouteDefinition()->getID())) {
    //                             delete veh->getRouteDefinition();
    //                         }
    //                     }
    //                     delete r;
    //                 }
    //             }
    //             myRoutables.erase(routables);
    //         }
    //         if (containerTime == minTime) {
    //             myRoutesOutput->writePreformattedTag(container->second);
    //             if (myRouteAlternativesOutput != nullptr) {
    //                 myRouteAlternativesOutput->writePreformattedTag(container->second);
    //             }
    //             myContainers.erase(container);
    //         }
    //     }
    //     return lastTime;
    // }
    #[allow(unused_variables)]
    pub fn save_and_remove_routes_until(&mut self, options: &mut OptionsCont, provider: &RORouterProvider, time: SUMOTime) -> SUMOTime {
        todo!("port RONet::saveAndRemoveRoutesUntil")
    }

    /// C++: `bool RONet::furtherStored()`
    /// Returns the information whether further vehicles, persons or containers are stored
    // bool
    // RONet::furtherStored() {
    //     return myRoutables.size() > 0 || (myFlows.size() > 0 && myHaveActiveFlows) || myContainers.size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn further_stored(&mut self) -> bool {
        todo!("port RONet::furtherStored")
    }

    /// C++: `void RONet::routeSkipped()`
    /// Record routes excluded by the begin time.
    // void routeSkipped() {
    //         myHaveSkippedRoutes = true;
    //     }
    #[allow(unused_variables)]
    pub fn route_skipped(&mut self) {
        todo!("port RONet::routeSkipped")
    }

    /// C++: `bool RONet::haveSkippedRoutes() const` [const]
    // bool haveSkippedRoutes() const {
    //         return myHaveSkippedRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn have_skipped_routes(&self) -> bool {
        todo!("port RONet::haveSkippedRoutes")
    }

    /// C++: `bool RONet::haveProcessedRoutes() const` [const]
    // bool haveProcessedRoutes() const {
    //         return myHaveProcessedRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn have_processed_routes(&self) -> bool {
        todo!("port RONet::haveProcessedRoutes")
    }

    /// C++: `void RONet::openOutput(const OptionsCont &)`
    /// Opens the output for computed routes
    // void
    // RONet::openOutput(const OptionsCont& options) {
    //     if (options.isSet("output-file") && options.getString("output-file") != "") {
    //         myRoutesOutput = &OutputDevice::getDevice(options.getString("output-file"));
    //         if (myRoutesOutput->isNull()) {
    //             myRoutesOutput = nullptr;
    //         } else {
    //             myRoutesOutput->writeXMLHeader("routes", "routes_file.xsd");
    //         }
    //     }
    //     if (options.exists("alternatives-output") && options.isSet("alternatives-output")
    //             && !(options.exists("write-trips") && options.getBool("write-trips"))) {
    //         myRouteAlternativesOutput = &OutputDevice::getDevice(options.getString("alternatives-output"));
    //         if (myRouteAlternativesOutput->isNull()) {
    //             myRouteAlternativesOutput = nullptr;
    //         } else {
    //             myRouteAlternativesOutput->writeXMLHeader("routes", "routes_file.xsd");
    //         }
    //     }
    //     if (options.isSet("vtype-output")) {
    //         myTypesOutput = &OutputDevice::getDevice(options.getString("vtype-output"));
    //         myTypesOutput->writeXMLHeader("routes", "routes_file.xsd");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_output(&mut self, options: &OptionsCont) {
        todo!("port RONet::openOutput")
    }

    /// C++: `void RONet::writeIntermodal(const OptionsCont &, ROIntermodalRouter &) const` [const]
    /// Writes the intermodal network and weights if requested
    // void
    // RONet::writeIntermodal(const OptionsCont& options, ROIntermodalRouter& router) const {
    //     if (options.exists("intermodal-network-output") && options.isSet("intermodal-network-output")) {
    //         OutputDevice::createDeviceByOption("intermodal-network-output", "intermodal");
    //         router.writeNetwork(OutputDevice::getDevice(options.getString("intermodal-network-output")));
    //     }
    //     if (options.exists("intermodal-weight-output") && options.isSet("intermodal-weight-output")) {
    //         OutputDevice::createDeviceByOption("intermodal-weight-output", "weights", "meandata_file.xsd");
    //         OutputDevice& dev = OutputDevice::getDeviceByOption("intermodal-weight-output");
    //         dev.openTag(SUMO_TAG_INTERVAL);
    //         dev.writeAttr(SUMO_ATTR_ID, "intermodalweights");
    //         dev.writeAttr(SUMO_ATTR_BEGIN, 0);
    //         dev.writeAttr(SUMO_ATTR_END, SUMOTime_MAX);
    //         router.writeWeights(dev);
    //         dev.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_intermodal(&self, options: &OptionsCont, router: &mut ROIntermodalRouter) {
        todo!("port RONet::writeIntermodal")
    }

    /// C++: `void RONet::cleanup()`
    /// closes the file output for computed routes and deletes associated threads if necessary
    // void
    // RONet::cleanup() {
    //     // end writing
    //     if (myRoutesOutput != nullptr) {
    //         myRoutesOutput->close();
    //     }
    //     // only if opened
    //     if (myRouteAlternativesOutput != nullptr) {
    //         myRouteAlternativesOutput->close();
    //     }
    //     // only if opened
    //     if (myTypesOutput != nullptr) {
    //         myTypesOutput->close();
    //     }
    //     RouteCostCalculator<RORoute, ROEdge, ROVehicle>::cleanup();
    // #ifdef HAVE_FOX
    //     if (myThreadPool.size() > 0) {
    //         myThreadPool.clear();
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn cleanup(&mut self) {
        todo!("port RONet::cleanup")
    }

    /// C++: `int RONet::getEdgeNumber() const` [const]
    /// Returns the total number of edges the network contains including internal edges
    // int
    // RONet::getEdgeNumber() const {
    //     return myEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_number(&self) -> i32 {
        todo!("port RONet::getEdgeNumber")
    }

    /// C++: `int RONet::getInternalEdgeNumber() const` [const]
    /// Returns the number of internal edges the network contains
    // int
    // RONet::getInternalEdgeNumber() const {
    //     return myNumInternalEdges;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_edge_number(&self) -> i32 {
        todo!("port RONet::getInternalEdgeNumber")
    }

    /// C++: `const NamedObjectCont<ROEdge *> & RONet::getEdgeMap() const` [const]
    // const NamedObjectCont<ROEdge*>& getEdgeMap() const {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_map(&self) -> &Unmapped /* NamedObjectCont<ROEdge *> */ {
        todo!("port RONet::getEdgeMap")
    }

    /// C++: `void RONet::adaptIntermodalRouter(ROIntermodalRouter &)` [static]
    // void
    // RONet::adaptIntermodalRouter(ROIntermodalRouter& router) {
    //     double taxiWait = STEPS2TIME(string2time(OptionsCont::getOptions().getString("persontrip.taxi.waiting-time")));
    //     for (const auto& stopType : myInstance->myStoppingPlaces) {
    //         // add access to all stopping places
    //         const SumoXMLTag element = stopType.first;
    //         for (const auto& stop : stopType.second) {
    //             router.getNetwork()->addAccess(stop.first, myInstance->getEdgeForLaneID(stop.second->lane),
    //                                            stop.second->startPos, stop.second->endPos, 0., element, false, taxiWait);
    //             // add access to all public transport stops
    //             if (element == SUMO_TAG_BUS_STOP) {
    //                 for (const auto& a : stop.second->accessPos) {
    //                     router.getNetwork()->addAccess(stop.first, myInstance->getEdgeForLaneID(std::get<0>(a)),
    //                                                    std::get<1>(a), std::get<1>(a), std::get<2>(a), SUMO_TAG_BUS_STOP, true, taxiWait);
    //                 }
    //             }
    //         }
    //     }
    //     // fill the public transport router with pre-parsed public transport lines
    //     for (const auto& i : myInstance->myFlows) {
    //         if (i.second->line != "") {
    //             const RORouteDef* const route = myInstance->getRouteDef(i.second->routeid);
    //             const StopParVector* addStops = nullptr;
    //             if (route != nullptr && route->getFirstRoute() != nullptr) {
    //                 addStops = &route->getFirstRoute()->getStops();
    //             }
    //             router.getNetwork()->addSchedule(*i.second, addStops);
    //         }
    //     }
    //     for (const RORoutable* const veh : myInstance->myPTVehicles) {
    //         // add single vehicles with line attribute which are not part of a flow
    //         // no need to add route stops here, they have been added to the vehicle before
    //         router.getNetwork()->addSchedule(veh->getParameter());
    //     }
    //     // add access to transfer from walking to taxi-use
    //     if ((router.getCarWalkTransfer() & ModeChangeOptions::TAXI_PICKUP_ANYWHERE) != 0) {
    //         for (const ROEdge* edge : ROEdge::getAllEdges()) {
    //             if (!edge->isTazConnector() && (edge->getPermissions() & SVC_PEDESTRIAN) != 0 && (edge->getPermissions() & SVC_TAXI) != 0) {
    //                 router.getNetwork()->addCarAccess(edge, SVC_TAXI, taxiWait);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn adapt_intermodal_router(router: &mut ROIntermodalRouter) {
        todo!("port RONet::adaptIntermodalRouter")
    }

    /// C++: `bool RONet::hasPermissions() const` [const]
    // bool
    // RONet::hasPermissions() const {
    //     return myHavePermissions;
    // }
    #[allow(unused_variables)]
    pub fn has_permissions(&self) -> bool {
        todo!("port RONet::hasPermissions")
    }

    /// C++: `void RONet::setPermissionsFound()`
    // void
    // RONet::setPermissionsFound() {
    //     myHavePermissions = true;
    // }
    #[allow(unused_variables)]
    pub fn set_permissions_found(&mut self) {
        todo!("port RONet::setPermissionsFound")
    }

    /// C++: `bool RONet::hasBidiEdges() const` [const]
    /// return whether the network contains bidirectional rail edges
    // bool hasBidiEdges() const {
    //         return myHasBidiEdges;
    //     }
    #[allow(unused_variables)]
    pub fn has_bidi_edges(&self) -> bool {
        todo!("port RONet::hasBidiEdges")
    }

    /// C++: `bool RONet::hasLoadedEffort() const` [const]
    /// whether efforts were loaded from file
    // bool
    // RONet::hasLoadedEffort() const {
    //     for (const auto& item : myEdges) {
    //         if (item.second->hasStoredEffort()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_loaded_effort(&self) -> bool {
        todo!("port RONet::hasLoadedEffort")
    }

    /// C++: `double RONet::getMaxTraveltime() const` [const]
    // double getMaxTraveltime() const {
    //         return myMaxTraveltime;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_traveltime(&self) -> f64 {
        todo!("port RONet::getMaxTraveltime")
    }

    /// C++: `bool RONet::hasProhibitions() const` [const]
    // bool hasProhibitions() const {
    //         return !myProhibitions.empty() || !myLaneProhibitions.empty();
    //     }
    #[allow(unused_variables)]
    pub fn has_prohibitions(&self) -> bool {
        todo!("port RONet::hasProhibitions")
    }

    /// C++: `const Prohibitions & RONet::getProhibitions() const` [const]
    // const Prohibitions& getProhibitions() const {
    //         return myProhibitions;
    //     }
    #[allow(unused_variables)]
    pub fn get_prohibitions(&self) -> &SUMOAbstractRouter_Prohibitions {
        todo!("port RONet::getProhibitions")
    }

    /// C++: `void RONet::updateLaneProhibitions(SUMOTime)`
    // void
    // RONet::updateLaneProhibitions(SUMOTime begin) {
    //     const double beginS = STEPS2TIME(begin);
    //     while (myLaneProhibitionTimes.size() > 0 && myLaneProhibitionTimes.begin()->first <= beginS) {
    //         const double t = myLaneProhibitionTimes.begin()->first;
    //         for (const ROLane* const lane : myLaneProhibitionTimes.begin()->second) {
    //             const SVCPermissions orig = lane->getPermissions();
    //             assert(myLaneProhibitions.count(lane) != 0);
    //             RouterProhibition& rp = myLaneProhibitions[lane];
    //             const_cast<ROLane*>(lane)->setPermissions(rp.permissions);
    //             lane->getEdge().resetSuccessors();
    //             for (ROEdge* pred : lane->getEdge().getPredecessors()) {
    //                 pred->resetSuccessors();
    //             }
    //             if (t == rp.begin) {
    //                 // schedule restoration of original permissions. This works
    //                 // without a stack because there is at most one prohibition per lane
    //                 myLaneProhibitionTimes[rp.end].insert(lane);
    //                 rp.permissions = orig;
    //             }
    //         }
    //         myLaneProhibitionTimes.erase(myLaneProhibitionTimes.begin());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_lane_prohibitions(&mut self, begin: SUMOTime) {
        todo!("port RONet::updateLaneProhibitions")
    }

    /// C++: `void RONet::addProhibition(const ROEdge *, const RouterProhibition &)`
    // void
    // RONet::addProhibition(const ROEdge* edge, const RouterProhibition& prohibition) {
    //     if (myProhibitions.count(edge) != 0) {
    //         throw ProcessError(TLF("Already loaded prohibition for edge '%'. (Only one prohibition per edge is supported)", edge->getID()));
    //     }
    //     myProhibitions[edge] = prohibition;
    //     myHavePermissions = true;
    // }
    #[allow(unused_variables)]
    pub fn add_prohibition(&mut self, edge: Ptr<ROEdge>, prohibition: &RouterProhibition) {
        todo!("port RONet::addProhibition")
    }

    /// C++: `void RONet::addLaneProhibition(const ROLane *, const RouterProhibition &)`
    // void
    // RONet::addLaneProhibition(const ROLane* lane, const RouterProhibition& prohibition) {
    //     if (myLaneProhibitions.count(lane) != 0) {
    //         throw ProcessError(TLF("Already loaded prohibition for lane '%'. (Only one prohibition per lane is supported)", lane->getID()));
    //     }
    //     assert(prohibition.end > prohibition.begin);
    //     myLaneProhibitions[lane] = prohibition;
    //     myLaneProhibitionTimes[prohibition.begin].insert(lane);
    //     myHavePermissions = true;
    // }
    #[allow(unused_variables)]
    pub fn add_lane_prohibition(&mut self, lane: Ptr<ROLane>, prohibition: &RouterProhibition) {
        todo!("port RONet::addLaneProhibition")
    }

    /// C++: `OutputDevice * RONet::getRouteOutput(const bool)`
    // OutputDevice* getRouteOutput(const bool alternative = false) {
    //         if (alternative) {
    //             return myRouteAlternativesOutput;
    //         }
    //         return myRoutesOutput;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_output(&mut self, alternative: bool) -> Ptr<OutputDevice> {
        todo!("port RONet::getRouteOutput")
    }

    /// C++: `void RONet::checkFlows(SUMOTime, MsgHandler *)` [private]
    // void
    // RONet::checkFlows(SUMOTime time, MsgHandler* errorHandler) {
    //     myHaveActiveFlows = false;
    //     for (const auto& i : myFlows) {
    //         SUMOVehicleParameter* const pars = i.second;
    //         if (pars->line != "" && !myDoPTRouting) {
    //             continue;
    //         }
    //         if (myKeepFlows) {
    //             if (pars->repetitionsDone < pars->repetitionNumber) {
    //                 // each each flow only once
    //                 pars->repetitionsDone = pars->repetitionNumber;
    //                 const SUMOVTypeParameter* type = getVehicleTypeSecure(pars->vtypeid);
    //                 if (type == nullptr) {
    //                     type = getVehicleTypeSecure(DEFAULT_VTYPE_ID);
    //                 } else {
    //                     auto dist = getVTypeDistribution(pars->vtypeid);
    //                     if (dist != nullptr) {
    //                         WRITE_WARNINGF("Keeping flow '%' with a vTypeDistribution can lead to invalid routes if the distribution contains different vClasses", pars->id);
    //                     }
    //                 }
    //                 RORouteDef* route = getRouteDef(pars->routeid)->copy(pars->routeid, pars->depart);
    //                 ROVehicle* veh = new ROVehicle(*pars, route, type, this, errorHandler);
    //                 addVehicle(pars->id, veh);
    //             }
    //         } else if (pars->repetitionProbability > 0) {
    //             if (pars->repetitionEnd > pars->depart && pars->repetitionsDone < pars->repetitionNumber) {
    //                 myHaveActiveFlows = true;
    //             }
    //             const SUMOTime origDepart = pars->depart;
    //             while (pars->depart < time && pars->repetitionsDone < pars->repetitionNumber) {
    //                 if (pars->repetitionEnd <= pars->depart) {
    //                     break;
    //                 }
    //                 // only call rand if all other conditions are met
    //                 if (RandHelper::rand() < (pars->repetitionProbability * TS)) {
    //                     SUMOVehicleParameter* newPars = new SUMOVehicleParameter(*pars);
    //                     newPars->id = pars->id + "." + toString(pars->repetitionsDone);
    //                     newPars->depart = pars->depart;
    //                     for (StopParVector::iterator stop = newPars->stops.begin(); stop != newPars->stops.end(); ++stop) {
    //                         if (stop->until >= 0) {
    //                             stop->until += pars->depart - origDepart;
    //                         }
    //                         if (stop->arrival >= 0) {
    //                             stop->arrival += pars->depart - origDepart;
    //                         }
    //                     }
    //                     pars->repetitionsDone++;
    //                     // try to build the vehicle
    //                     const SUMOVTypeParameter* type = getVehicleTypeSecure(pars->vtypeid);
    //                     if (type == nullptr) {
    //                         type = getVehicleTypeSecure(DEFAULT_VTYPE_ID);
    //                     } else if (!myKeepVTypeDist) {
    //                         // fix the type id in case we used a distribution
    //                         newPars->vtypeid = type->id;
    //                     }
    //                     const SUMOTime stopOffset = pars->routeid[0] == '!' ? pars->depart - origDepart : pars->depart;
    //                     RORouteDef* route = getRouteDef(pars->routeid)->copy("!" + newPars->id, stopOffset);
    //                     ROVehicle* veh = new ROVehicle(*newPars, route, type, this, errorHandler);
    //                     addVehicle(newPars->id, veh);
    //                     delete newPars;
    //                 }
    //                 pars->depart += DELTA_T;
    //             }
    //         } else {
    //             SUMOTime depart = static_cast<SUMOTime>(pars->depart + pars->repetitionTotalOffset);
    //             while (pars->repetitionsDone < pars->repetitionNumber && pars->repetitionEnd >= depart) {
    //                 myHaveActiveFlows = true;
    //                 depart = static_cast<SUMOTime>(pars->depart + pars->repetitionTotalOffset);
    //                 if (myDepartures.find(pars->id) != myDepartures.end()) {
    //                     depart = myDepartures[pars->id].back();
    //                 }
    //                 if (depart >= time + DELTA_T) {
    //                     break;
    //                 }
    //                 if (myDepartures.find(pars->id) != myDepartures.end()) {
    //                     myDepartures[pars->id].pop_back();
    //                 }
    //                 SUMOVehicleParameter* newPars = new SUMOVehicleParameter(*pars);
    //                 newPars->id = pars->id + "." + toString(pars->repetitionsDone);
    //                 newPars->depart = depart;
    //                 for (StopParVector::iterator stop = newPars->stops.begin(); stop != newPars->stops.end(); ++stop) {
    //                     if (stop->until >= 0) {
    //                         stop->until += depart - pars->depart;
    //                     }
    //                     if (stop->arrival >= 0) {
    //                         stop->arrival += depart - pars->depart;
    //                     }
    //                 }
    //                 pars->incrementFlow(1);
    //                 // try to build the vehicle
    //                 const SUMOVTypeParameter* type = getVehicleTypeSecure(pars->vtypeid);
    //                 if (type == nullptr) {
    //                     type = getVehicleTypeSecure(DEFAULT_VTYPE_ID);
    //                 } else if (!myKeepVTypeDist) {
    //                     // fix the type id in case we used a distribution
    //                     newPars->vtypeid = type->id;
    //                 }
    //                 const SUMOTime stopOffset = pars->routeid[0] == '!' ? depart - pars->depart : depart;
    //                 RORouteDef* route = getRouteDef(pars->routeid)->copy("!" + newPars->id, stopOffset);
    //                 ROVehicle* veh = new ROVehicle(*newPars, route, type, this, errorHandler);
    //                 addVehicle(newPars->id, veh);
    //                 delete newPars;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_flows(&mut self, time: SUMOTime, errorHandler: Ptr<MsgHandler>) {
        todo!("port RONet::checkFlows")
    }

    /// C++: `void RONet::createBulkRouteRequests(const RORouterProvider &, const SUMOTime, const bool)` [private]
    // void
    // RONet::createBulkRouteRequests(const RORouterProvider& provider, const SUMOTime time, const bool removeLoops) {
    //     std::map<const int, std::vector<RORoutable*> > bulkVehs;
    //     int numBulked = 0;
    //     for (RoutablesMap::const_iterator i = myRoutables.begin(); i != myRoutables.end(); ++i) {
    //         if (i->first >= time) {
    //             break;
    //         }
    //         for (RORoutable* const routable : i->second) {
    //             const ROEdge* const depEdge = routable->getDepartEdge();
    //             bulkVehs[depEdge->getNumericalID()].push_back(routable);
    //             RORoutable* const first = bulkVehs[depEdge->getNumericalID()].front();
    //             numBulked++;
    //             if (first->getMaxSpeed() != routable->getMaxSpeed()) {
    //                 WRITE_WARNINGF(TL("Bulking different maximum speeds ('%' and '%') may lead to suboptimal routes."), first->getID(), routable->getID());
    //             }
    //             if (first->getVClass() != routable->getVClass()) {
    //                 WRITE_WARNINGF(TL("Bulking different vehicle classes ('%' and '%') may lead to invalid routes."), first->getID(), routable->getID());
    //             }
    //         }
    //     }
    // #ifdef HAVE_FOX
    //     int workerIndex = 0;
    // #endif
    //     if ((int)bulkVehs.size() < numBulked) {
    //         WRITE_MESSAGE(TLF("Using bulk-mode for % entities from % origins", numBulked, bulkVehs.size()));
    //     }
    //     for (std::map<const int, std::vector<RORoutable*> >::const_iterator i = bulkVehs.begin(); i != bulkVehs.end(); ++i) {
    // #ifdef HAVE_FOX
    //         if (myThreadPool.size() > 0) {
    //             bool bulk = true;
    //             for (RORoutable* const r : i->second) {
    //                 myThreadPool.add(new RoutingTask(r, removeLoops, myErrorHandler), workerIndex);
    //                 if (bulk) {
    //                     myThreadPool.add(new BulkmodeTask(true), workerIndex);
    //                     bulk = false;
    //                 }
    //             }
    //             myThreadPool.add(new BulkmodeTask(false), workerIndex);
    //             workerIndex++;
    //             if (workerIndex == (int)myThreadPool.size()) {
    //                 workerIndex = 0;
    //             }
    //             continue;
    //         }
    // #endif
    //         for (RORoutable* const r : i->second) {
    //             r->computeRoute(provider, removeLoops, myErrorHandler);
    //             provider.setBulkMode(true);
    //         }
    //         provider.setBulkMode(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_bulk_route_requests(&mut self, provider: &RORouterProvider, time: SUMOTime, removeLoops: bool) {
        todo!("port RONet::createBulkRouteRequests")
    }

    /// C++: `void RONet::RONet(const RONet &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RONet) -> Self {
        todo!("port RONet::RONet")
    }

    /// C++: `RONet & RONet::operator=(const RONet &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RONet) -> &mut RONet {
        todo!("port RONet::operator=")
    }

}

