//! Port of router/RONetHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The handler that parses a SUMO-network for its usage in a router
/// C++ class `RONetHandler` (router/RONetHandler.h)
pub struct RONetHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// The net to store the information into
    pub myNet: Ptr<RONet>,
    /// the loaded network version
    pub myNetworkVersion: Unmapped /* MMVersion */,
    /// The object used to build of edges of the desired type
    pub myEdgeBuilder: Ptr<ROAbstractEdgeBuilder>,
    /// whether to ignore junction internal edges
    pub myIgnoreInternal: bool,
    /// The name of the edge/node that is currently processed
    pub myCurrentName: String,
    /// The id of the currently processed edge type
    pub myCurrentTypeID: String,
    /// The currently built edge
    pub myCurrentEdge: Ptr<ROEdge>,
    /// The currently built stopping place
    pub myCurrentStoppingPlace: Ptr<SUMOVehicleParameter_Stop>,
    /// temporary data for checking node initialisation after network parsing is finished
    pub myUnseenNodeIDs: i32,
    /// time penalty for passing a minor link
    pub myMinorPenalty: f64,
    pub myTLSPenalty: f64,
    pub myTurnaroundPenalty: f64,
    /// temporary storage for bidi attributes (to be resolved after loading all edges)
    pub myBidiEdges: i32,
    pub myRerouterID: String,
    pub myIntervalBegin: SUMOTime,
    pub myIntervalEnd: SUMOTime,
}

impl std::ops::Deref for RONetHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for RONetHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl RONetHandler {
    /// C++: `void RONetHandler::RONetHandler(RONet &, ROAbstractEdgeBuilder &, const bool, const double, double, double)`
    /// Constructor
    // RONetHandler::RONetHandler(RONet& net, ROAbstractEdgeBuilder& eb, const bool ignoreInternal, const double minorPenalty, double tlsPenalty, double turnaroundPenalty) :
    //     SUMOSAXHandler("sumo-network"),
    //     myNet(net),
    //     myNetworkVersion(0, 0),
    //     myEdgeBuilder(eb), myIgnoreInternal(ignoreInternal),
    //     myCurrentName(), myCurrentEdge(nullptr), myCurrentStoppingPlace(nullptr),
    //     myMinorPenalty(minorPenalty),
    //     myTLSPenalty(tlsPenalty),
    //     myTurnaroundPenalty(turnaroundPenalty)
    // {}
    #[allow(unused_variables)]
    pub fn new(net: &mut RONet, eb: &mut ROAbstractEdgeBuilder, ignoreInternal: bool, minorPenalty: f64, tlsPenalty: f64, turnaroundPenalty: f64) -> Self {
        todo!("port RONetHandler::RONetHandler")
    }

    /// C++: `void RONetHandler::~RONetHandler()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RONetHandler::~RONetHandler")
    }

    /// C++: `const int & RONetHandler::getBidiMap() const` [const]
    /// retrieve mapping of edges to bidi edges (must be resolved after loading network)
    // const std::map<ROEdge*, std::string>& getBidiMap() const {
    //         return myBidiEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_map(&self) -> &i32 {
        todo!("port RONetHandler::getBidiMap")
    }

    /// C++: `void RONetHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // RONetHandler::myStartElement(int element,
    //                              const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_LOCATION:
    //             setLocation(attrs);
    //             break;
    //         case SUMO_TAG_NET: {
    //             bool ok;
    //             myNetworkVersion = StringUtils::toVersion(attrs.get<std::string>(SUMO_ATTR_VERSION, nullptr, ok, false));
    //             break;
    //         }
    //         case SUMO_TAG_EDGE:
    //             // in the first step, we do need the name to allocate the edge
    //             // in the second, we need it to know to which edge we have to add
    //             //  the following edges to
    //             parseEdge(attrs);
    //             break;
    //         case SUMO_TAG_LANE:
    //             parseLane(attrs);
    //             break;
    //         case SUMO_TAG_JUNCTION:
    //             parseJunction(attrs);
    //             break;
    //         case SUMO_TAG_CONNECTION:
    //             parseConnection(attrs);
    //             break;
    //         case SUMO_TAG_BUS_STOP:
    //         case SUMO_TAG_TRAIN_STOP:
    //         case SUMO_TAG_CONTAINER_STOP:
    //         case SUMO_TAG_PARKING_AREA:
    //         case SUMO_TAG_CHARGING_STATION:
    //         case SUMO_TAG_OVERHEAD_WIRE_SEGMENT:
    //             parseStoppingPlace(attrs, (SumoXMLTag)element);
    //             break;
    //         case SUMO_TAG_ACCESS:
    //             parseAccess(attrs);
    //             break;
    //         case SUMO_TAG_TAZ:
    //             parseDistrict(attrs);
    //             break;
    //         case SUMO_TAG_TAZSOURCE:
    //             parseDistrictEdge(attrs, true);
    //             break;
    //         case SUMO_TAG_TAZSINK:
    //             parseDistrictEdge(attrs, false);
    //             break;
    //         case SUMO_TAG_TYPE: {
    //             bool ok = true;
    //             myCurrentTypeID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             break;
    //         }
    //         case SUMO_TAG_RESTRICTION: {
    //             bool ok = true;
    //             const SUMOVehicleClass svc = getVehicleClassID(attrs.get<std::string>(SUMO_ATTR_VCLASS, myCurrentTypeID.c_str(), ok));
    //             const double speed = attrs.get<double>(SUMO_ATTR_SPEED, myCurrentTypeID.c_str(), ok);
    //             if (ok) {
    //                 myNet.addSpeedRestriction(myCurrentTypeID, svc, speed);
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_PREFERENCE: {
    //             bool ok = true;
    //             const std::string routingType = attrs.get<std::string>(SUMO_ATTR_ROUTINGTYPE, nullptr, ok);
    //             const double prio = attrs.get<double>(SUMO_ATTR_PRIORITY, routingType.c_str(), ok);
    //             if (prio <= 0) {
    //                 throw InvalidArgument("In preference for routingType '" + routingType + "', priority must be positve");
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_VCLASSES)) {
    //                 StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_VCLASSES, routingType.c_str(), ok));
    //                 for (std::string className : st.getVector()) {
    //                     myNet.addPreference(routingType, getVehicleClassID(className), prio);
    //                 }
    //             } else if (!attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //                 // general preferenze applying to all types and vClasses
    //                 myNet.addPreference(routingType, "", prio);
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //                 StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_VTYPES, routingType.c_str(), ok));
    //                 for (std::string typeName : st.getVector()) {
    //                     myNet.addPreference(routingType, typeName, prio);
    //                 }
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_REROUTER: {
    //             bool ok = true;
    //             myRerouterID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             break;
    //         }
    //         case SUMO_TAG_INTERVAL:
    //             if (myRerouterID != "") {
    //                 bool ok = true;
    //                 myIntervalBegin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, myRerouterID.c_str(), ok, -1);
    //                 myIntervalEnd = attrs.getOptSUMOTimeReporting(SUMO_ATTR_END, myRerouterID.c_str(), ok, SUMOTime_MAX);
    //                 if (myIntervalEnd <= myIntervalBegin) {
    //                     throw ProcessError(TLF("Invalid rerouter interval from % to %", time2string(myIntervalBegin), time2string(myIntervalEnd)));
    //                 }
    //             }
    //             break;
    //         case SUMO_TAG_CLOSING_REROUTE: {
    //             const std::string& closed_id = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //             ROEdge* closedEdge = myNet.getEdge(closed_id);
    //             if (closedEdge == nullptr) {
    //                 throw ProcessError(TLF("rerouter '%': Edge '%' to close is not known.", myRerouterID, closed_id));
    //             }
    //             bool ok;
    //             const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, myRerouterID.c_str(), ok, "", false);
    //             const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, myRerouterID.c_str(), ok, "");
    //             RouterProhibition prohibition;
    //             prohibition.permissions = parseVehicleClasses(allow, disallow);
    //             prohibition.begin = STEPS2TIME(myIntervalBegin);
    //             prohibition.end = STEPS2TIME(attrs.getOptSUMOTimeReporting(SUMO_ATTR_UNTIL, nullptr, ok, myIntervalEnd));
    //             myNet.addProhibition(closedEdge, prohibition);
    //             break;
    //         }
    //         case SUMO_TAG_CLOSING_LANE_REROUTE: {
    //             const std::string& closed_id = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //             ROLane* closedLane = myNet.getLane(closed_id);
    //             if (closedLane == nullptr) {
    //                 throw ProcessError(TLF("rerouter '%': Lane '%' to close is not known.", myRerouterID, closed_id));
    //             }
    //             bool ok;
    //             const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, myRerouterID.c_str(), ok, "", false);
    //             const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, myRerouterID.c_str(), ok, "");
    //             RouterProhibition prohibition;
    //             prohibition.permissions = parseVehicleClasses(allow, disallow);
    //             prohibition.begin = STEPS2TIME(myIntervalBegin);
    //             prohibition.end = STEPS2TIME(attrs.getOptSUMOTimeReporting(SUMO_ATTR_UNTIL, nullptr, ok, myIntervalEnd));
    //             myNet.addLaneProhibition(closedLane, prohibition);
    //             break;
    //         }
    //         case SUMO_TAG_PARAM:
    //             addParam(attrs);
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::myStartElement")
    }

    /// C++: `void RONetHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // RONetHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_REROUTER:
    //             myRerouterID = "";
    //             break;
    //         case SUMO_TAG_NET:
    //             // build junction graph
    //             for (std::set<std::string>::const_iterator it = myUnseenNodeIDs.begin(); it != myUnseenNodeIDs.end(); ++it) {
    //                 WRITE_ERRORF(TL("Unknown node '%'."), *it);
    //             }
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port RONetHandler::myEndElement")
    }

    /// C++: `void RONetHandler::addParam(const SUMOSAXAttributes &)` [protected]
    /// assign arbitrary vehicle parameters
    // void
    // RONetHandler::addParam(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //     // circumventing empty string test
    //     const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //     // add parameter in current created element, or in myLoadedParameterised
    //     if (myCurrentEdge != nullptr) {
    //         myCurrentEdge->setParameter(key, val);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_param(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::addParam")
    }

    /// C++: `void RONetHandler::parseEdge(const SUMOSAXAttributes &)` [protected]
    /// Parses and builds an edge
    // void
    // RONetHandler::parseEdge(const SUMOSAXAttributes& attrs) {
    //     // get the id, report an error if not given or empty...
    //     bool ok = true;
    //     myCurrentName = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     const SumoXMLEdgeFunc func = attrs.getOpt<SumoXMLEdgeFunc>(SUMO_ATTR_FUNCTION, myCurrentName.c_str(), ok, SumoXMLEdgeFunc::NORMAL);
    //     if (!ok) {
    //         return;
    //     }
    //     // get the edge
    //     std::string from;
    //     std::string to;
    //     int priority;
    //     myCurrentEdge = nullptr;
    //     if (func == SumoXMLEdgeFunc::INTERNAL || func == SumoXMLEdgeFunc::CROSSING || func == SumoXMLEdgeFunc::WALKINGAREA) {
    //         assert(myCurrentName[0] == ':');
    //         const std::string junctionID = SUMOXMLDefinitions::getJunctionIDFromInternalEdge(myCurrentName);
    //         from = junctionID;
    //         to = junctionID;
    //         priority = -1;
    //     } else {
    //         from = attrs.get<std::string>(SUMO_ATTR_FROM, myCurrentName.c_str(), ok);
    //         to = attrs.get<std::string>(SUMO_ATTR_TO, myCurrentName.c_str(), ok);
    //         priority = attrs.get<int>(SUMO_ATTR_PRIORITY, myCurrentName.c_str(), ok);
    //         if (!ok) {
    //             return;
    //         }
    //     }
    //     RONode* fromNode = myNet.getNode(from);
    //     if (fromNode == nullptr) {
    //         myUnseenNodeIDs.insert(from);
    //         fromNode = new RONode(from);
    //         myNet.addNode(fromNode);
    //     }
    //     RONode* toNode = myNet.getNode(to);
    //     if (toNode == nullptr) {
    //         myUnseenNodeIDs.insert(to);
    //         toNode = new RONode(to);
    //         myNet.addNode(toNode);
    //     }
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, myCurrentName.c_str(), ok, "");
    //     const std::string routingType = attrs.getOpt<std::string>(SUMO_ATTR_ROUTINGTYPE, myCurrentName.c_str(), ok, "");
    //     // build the edge
    //     myCurrentEdge = myEdgeBuilder.buildEdge(myCurrentName, fromNode, toNode, priority, type, routingType);
    //     myCurrentEdge->setFunction(func);
    //
    //     if (myNet.addEdge(myCurrentEdge)) {
    //         fromNode->addOutgoing(myCurrentEdge);
    //         toNode->addIncoming(myCurrentEdge);
    //         const std::string bidi = attrs.getOpt<std::string>(SUMO_ATTR_BIDI, myCurrentName.c_str(), ok, "");
    //         if (bidi != "") {
    //             myBidiEdges[myCurrentEdge] = bidi;
    //         }
    //     } else {
    //         myCurrentEdge = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edge(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::parseEdge")
    }

    /// C++: `void RONetHandler::parseLane(const SUMOSAXAttributes &)` [virtual, protected]
    /// Parses and builds a lane
    // void
    // RONetHandler::parseLane(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentEdge == nullptr) {
    //         // was an internal edge to skip or an error occurred
    //         return;
    //     }
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     // get the speed
    //     double maxSpeed = attrs.get<double>(SUMO_ATTR_SPEED, id.c_str(), ok);
    //     double length = attrs.get<double>(SUMO_ATTR_LENGTH, id.c_str(), ok);
    //     std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, id.c_str(), ok, "");
    //     std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, id.c_str(), ok, "");
    //     const PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok);
    //     if (!ok) {
    //         return;
    //     }
    //     if (shape.size() < 2 && myCurrentEdge->getFunction() != SumoXMLEdgeFunc::WALKINGAREA) {
    //         WRITE_ERRORF(TL("Ignoring lane '%' with broken shape."), id);
    //         return;
    //     }
    //     // get the length
    //     // get the vehicle classes
    //     SVCPermissions permissions = parseVehicleClasses(allow, disallow, myNetworkVersion);
    //     if (permissions != SVCAll) {
    //         myNet.setPermissionsFound();
    //     }
    //     // add when both values are valid
    //     if (maxSpeed > 0 && length > 0 && id.length() > 0) {
    //         myCurrentEdge->addLane(new ROLane(id, myCurrentEdge, length, maxSpeed, permissions, shape));
    //     } else {
    //         WRITE_WARNING("Ignoring lane '" + id + "' with speed " + toString(maxSpeed) + " and length " + toString(length));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_lane(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::parseLane")
    }

    /// C++: `void RONetHandler::parseJunction(const SUMOSAXAttributes &)` [protected]
    /// Parses a junction's position
    // void
    // RONetHandler::parseJunction(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const SumoXMLNodeType type = attrs.get<SumoXMLNodeType>(SUMO_ATTR_TYPE, id.c_str(), ok);
    //     if (type == SumoXMLNodeType::INTERNAL) {
    //         return;
    //     }
    //     myUnseenNodeIDs.erase(id);
    //     // get the position of the node
    //     const double x = attrs.get<double>(SUMO_ATTR_X, id.c_str(), ok);
    //     const double y = attrs.get<double>(SUMO_ATTR_Y, id.c_str(), ok);
    //     const double z = attrs.getOpt<double>(SUMO_ATTR_Z, id.c_str(), ok, 0.);
    //     if (!ok) {
    //         return;
    //     }
    //     RONode* n = myNet.getNode(id);
    //     if (n == nullptr) {
    //         WRITE_WARNINGF(TL("Skipping isolated junction '%'."), id);
    //     } else {
    //         n->setPosition(Position(x, y, z));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_junction(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::parseJunction")
    }

    /// C++: `void RONetHandler::parseConnection(const SUMOSAXAttributes &)` [protected]
    /// Parses a connection Called on the occurrence of a "connection" element
    // void
    // RONetHandler::parseConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     const int fromLane = attrs.get<int>(SUMO_ATTR_FROM_LANE, nullptr, ok);
    //     const int toLane = attrs.get<int>(SUMO_ATTR_TO_LANE, nullptr, ok);
    //     std::string dir = attrs.get<std::string>(SUMO_ATTR_DIR, nullptr, ok);
    //     std::string viaID = attrs.getOpt<std::string>(SUMO_ATTR_VIA, nullptr, ok, "");
    //     std::string tlID = attrs.getOpt<std::string>(SUMO_ATTR_TLID, nullptr, ok, "");
    //     ROEdge* from = myNet.getEdge(fromID);
    //     ROEdge* to = myNet.getEdge(toID);
    //     if (from == nullptr) {
    //         throw ProcessError(TLF("unknown from-edge '%' in connection", fromID));
    //     }
    //     if (to == nullptr) {
    //         throw ProcessError(TLF("unknown to-edge '%' in connection", toID));
    //     }
    //     if ((int)from->getLanes().size() <= fromLane) {
    //         throw ProcessError("invalid fromLane '" + toString(fromLane) + "' in connection from '" + fromID + "'.");
    //     }
    //     if ((int)to->getLanes().size() <= toLane) {
    //         throw ProcessError("invalid toLane '" + toString(toLane) + "' in connection to '" + toID + "'.");
    //     }
    //     if (myIgnoreInternal || viaID == "") {
    //         std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, nullptr, ok, "");
    //         std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, nullptr, ok, "");
    //         ROEdge* dummyVia = nullptr;
    //         SVCPermissions permissions;
    //         if (allow == "" && disallow == "") {
    //             permissions = SVC_UNSPECIFIED;
    //         } else {
    //             myNet.setPermissionsFound();
    //             // dummyVia is only needed to hold permissions
    //             permissions = parseVehicleClasses(allow, disallow);
    //             dummyVia = new ROEdge("dummyVia_" + from->getLanes()[fromLane]->getID() + "->" + to->getLanes()[toLane]->getID(),
    //                                   from->getToJunction(), from->getToJunction(), permissions);
    //             myNet.addEdge(dummyVia);
    //         }
    //         from->getLanes()[fromLane]->addOutgoingLane(to->getLanes()[toLane], dummyVia);
    //         from->addSuccessor(to, nullptr, dir);
    //     } else {
    //         ROEdge* const via = myNet.getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(viaID));
    //         if (via == nullptr) {
    //             throw ProcessError(TLF("unknown via-edge '%' in connection", viaID));
    //         }
    //         from->getLanes()[fromLane]->addOutgoingLane(to->getLanes()[toLane], via);
    //         from->addSuccessor(to, via, dir);
    //         via->addSuccessor(to, nullptr, dir);
    //         LinkState state = SUMOXMLDefinitions::LinkStates.get(attrs.get<std::string>(SUMO_ATTR_STATE, nullptr, ok));
    //         if (state == LINKSTATE_MINOR || state == LINKSTATE_EQUAL || state == LINKSTATE_STOP || state == LINKSTATE_ALLWAY_STOP) {
    //             via->setTimePenalty(myMinorPenalty);
    //         }
    //         if (dir == toString(LinkDirection::TURN) || dir == toString(LinkDirection::TURN_LEFTHAND)) {
    //             via->setTimePenalty(myTurnaroundPenalty);
    //         }
    //         if (tlID != "") {
    //             via->setTimePenalty(myTLSPenalty);
    //         }
    //     }
    //     if (to->isCrossing()) {
    //         to->setTimePenalty(myTLSPenalty);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_connection(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::parseConnection")
    }

    /// C++: `void RONetHandler::parseStoppingPlace(const SUMOSAXAttributes &, const SumoXMLTag)` [protected]
    /// Parses a stopping place Called on the occurrence of a "busStop", "trainStop" or "containerStop" element
    // void
    // RONetHandler::parseStoppingPlace(const SUMOSAXAttributes& attrs, const SumoXMLTag element) {
    //     bool ok = true;
    //     myCurrentStoppingPlace = new SUMOVehicleParameter::Stop();
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, toString(element).c_str(), ok);
    //     // get the lane
    //     myCurrentStoppingPlace->lane = attrs.get<std::string>(SUMO_ATTR_LANE, toString(element).c_str(), ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     const ROEdge* edge = myNet.getEdgeForLaneID(myCurrentStoppingPlace->lane);
    //     if (edge == nullptr) {
    //         throw InvalidArgument("Unknown lane '" + myCurrentStoppingPlace->lane + "' for " + toString(element) + " '" + id + "'.");
    //     }
    //     // get the positions
    //     myCurrentStoppingPlace->startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), ok, 0.);
    //     myCurrentStoppingPlace->endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok, edge->getLength());
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     if (!ok || (SUMORouteHandler::checkStopPos(myCurrentStoppingPlace->startPos, myCurrentStoppingPlace->endPos, edge->getLength(), POSITION_EPS, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position for " + toString(element) + " '" + id + "'.");
    //     }
    //     // this is a hack: the busstop attribute is meant to hold the id within the simulation context but this is not used within the router context
    //     myCurrentStoppingPlace->busstop = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     // this is a hack: the actType is not used when using this to encode a stopping place
    //     myCurrentStoppingPlace->actType = toString(element);
    //     myNet.addStoppingPlace(id, element, myCurrentStoppingPlace);
    // }
    #[allow(unused_variables)]
    pub fn parse_stopping_place(&mut self, attrs: &SUMOSAXAttributes, element: SumoXMLTag) {
        todo!("port RONetHandler::parseStoppingPlace")
    }

    /// C++: `void RONetHandler::parseAccess(const SUMOSAXAttributes &)` [protected]
    /// Parses an access point to a train stop Called on the occurrence of an "access" element
    // void
    // RONetHandler::parseAccess(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string lane = attrs.get<std::string>(SUMO_ATTR_LANE, "access", ok);
    //     const ROEdge* edge = myNet.getEdgeForLaneID(lane);
    //     if (edge == nullptr) {
    //         throw InvalidArgument("Unknown lane '" + lane + "' for access.");
    //     }
    //     if ((edge->getPermissions() & SVC_PEDESTRIAN) == 0) {
    //         WRITE_WARNINGF(TL("Ignoring invalid access from non-pedestrian edge '%'."), edge->getID());
    //         return;
    //     }
    //     const bool random = attrs.getOpt<std::string>(SUMO_ATTR_POSITION, "access", ok) == "random";
    //     double pos = random ? edge->getLength() / 2. : attrs.getOpt<double>(SUMO_ATTR_POSITION, "access", ok, 0.);
    //     double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, "access", ok, -1);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, "access", ok, false);
    //     if (!ok || (SUMORouteHandler::checkStopPos(pos, pos, edge->getLength(), 0., friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position " + toString(pos) + " for access on lane '" + lane + "'.");
    //     }
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     if (length < 0) {
    //         const Position accPos = myNet.getLane(lane)->geometryPositionAtOffset(pos);
    //         const double stopCenter = (myCurrentStoppingPlace->startPos + myCurrentStoppingPlace->endPos) / 2;
    //         const Position stopPos = myNet.getLane(myCurrentStoppingPlace->lane)->geometryPositionAtOffset(stopCenter);
    //         length  = accPos.distanceTo(stopPos);
    //     }
    //     myCurrentStoppingPlace->accessPos.push_back(std::make_tuple(lane, pos, length));
    // }
    #[allow(unused_variables)]
    pub fn parse_access(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::parseAccess")
    }

    /// C++: `void RONetHandler::parseDistrict(const SUMOSAXAttributes &)` [protected]
    /// Parses a district and creates a pseudo edge for it
    // void
    // RONetHandler::parseDistrict(const SUMOSAXAttributes& attrs) {
    //     myCurrentEdge = nullptr;
    //     bool ok = true;
    //     myCurrentName = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     ROEdge* const sink = myEdgeBuilder.buildEdge(myCurrentName + "-sink", nullptr, nullptr, 0, "", "");
    //     ROEdge* const source = myEdgeBuilder.buildEdge(myCurrentName + "-source", nullptr, nullptr, 0, "", "");
    //     myNet.addDistrict(myCurrentName, source, sink);
    //     if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //         const std::vector<std::string>& desc = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, myCurrentName.c_str(), ok);
    //         for (const std::string& eID : desc) {
    //             myNet.addDistrictEdge(myCurrentName, eID, true);
    //             myNet.addDistrictEdge(myCurrentName, eID, false);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_district(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::parseDistrict")
    }

    /// C++: `void RONetHandler::parseDistrictEdge(const SUMOSAXAttributes &, bool)` [protected]
    /// Parses a district edge and connects it to the district
    // void
    // RONetHandler::parseDistrictEdge(const SUMOSAXAttributes& attrs, bool isSource) {
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, myCurrentName.c_str(), ok);
    //     myNet.addDistrictEdge(myCurrentName, id, isSource);
    // }
    #[allow(unused_variables)]
    pub fn parse_district_edge(&mut self, attrs: &SUMOSAXAttributes, isSource: bool) {
        todo!("port RONetHandler::parseDistrictEdge")
    }

    /// C++: `void RONetHandler::setLocation(const SUMOSAXAttributes &)` [protected]
    /// Parses network location description
    // void
    // RONetHandler::setLocation(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     PositionVector s = attrs.get<PositionVector>(SUMO_ATTR_NET_OFFSET, nullptr, ok);
    //     Boundary convBoundary = attrs.get<Boundary>(SUMO_ATTR_CONV_BOUNDARY, nullptr, ok);
    //     Boundary origBoundary = attrs.get<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, nullptr, ok);
    //     std::string proj = attrs.get<std::string>(SUMO_ATTR_ORIG_PROJ, nullptr, ok);
    //     if (ok) {
    //         Position networkOffset = s[0];
    //         GeoConvHelper::init(proj, networkOffset, origBoundary, convBoundary);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_location(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RONetHandler::setLocation")
    }

    /// C++: `void RONetHandler::RONetHandler(const RONetHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RONetHandler) -> Self {
        todo!("port RONetHandler::RONetHandler")
    }

    /// C++: `RONetHandler & RONetHandler::operator=(const RONetHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RONetHandler) -> &mut RONetHandler {
        todo!("port RONetHandler::operator=")
    }

}

