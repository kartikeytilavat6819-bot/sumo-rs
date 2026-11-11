//! Port of netimport/NIXMLPTHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Importer for network edges stored in XML
/// C++ class `NIXMLPTHandler` (netimport/NIXMLPTHandler.h)
pub struct NIXMLPTHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// The edges container (for retrieving referenced stop edge)
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The stop container (for loading of stops)
    pub myStopCont: Ptr<NBPTStopCont>,
    /// The line container (for loading of lines)
    pub myLineCont: Ptr<NBPTLineCont>,
    /// The currently processed stop
    pub myCurrentStop: Rc<NBPTStop>,
    /// The currently processed line
    pub myCurrentLine: Ptr<NBPTLine>,
    /// The currently processed stand-alone route
    pub myCurrentRouteID: String,
    /// the completion level of the current line
    pub myCurrentCompletion: f64,
    /// element to receive parameters
    pub myLastParameterised: Vec<Ptr<Parameterised>>,
    /// stand-alone route information
    pub myRouteStops: BTreeMap<String, Vec<Rc<NBPTStop>>>,
    pub myRouteEdges: BTreeMap<String, EdgeVector>,
    /// whether the current stop should be discarded
    pub myCurrentStopWasIgnored: bool,
    pub myWarnOnly: bool,
    pub myMissingBefore: i32,
    pub myMissingAfter: i32,
}

impl std::ops::Deref for NIXMLPTHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLPTHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl NIXMLPTHandler {
    /// C++: `void NIXMLPTHandler::NIXMLPTHandler(NBEdgeCont &, NBPTStopCont &, NBPTLineCont &)`
    /// Constructor
    // NIXMLPTHandler::NIXMLPTHandler(NBEdgeCont& ec, NBPTStopCont& sc, NBPTLineCont& lc) :
    //     SUMOSAXHandler("public transport - file"),
    //     myEdgeCont(ec),
    //     myStopCont(sc),
    //     myLineCont(lc),
    //     myCurrentStop(nullptr),
    //     myCurrentLine(nullptr),
    //     myCurrentCompletion(0),
    //     myCurrentStopWasIgnored(false),
    //     myWarnOnly(OptionsCont::getOptions().getBool("ignore-errors"))
    // { }
    #[allow(unused_variables)]
    pub fn new(ec: &mut NBEdgeCont, sc: &mut NBPTStopCont, lc: &mut NBPTLineCont) -> Self {
        todo!("port NIXMLPTHandler::NIXMLPTHandler")
    }

    /// C++: `void NIXMLPTHandler::~NIXMLPTHandler() noexcept` [virtual]
    /// Destructor
    // NIXMLPTHandler::~NIXMLPTHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIXMLPTHandler::~NIXMLPTHandler")
    }

    /// C++: `void NIXMLPTHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIXMLPTHandler::myStartElement(int element,
    //                                const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_BUS_STOP:
    //         case SUMO_TAG_TRAIN_STOP:
    //         case SUMO_TAG_STOP:
    //             if (myCurrentRouteID != "") {
    //                 addRouteStop(attrs);
    //             } else if (myCurrentLine == nullptr) {
    //                 addPTStop(element, attrs);
    //             } else {
    //                 addPTLineStop(attrs);
    //             }
    //             break;
    //         case SUMO_TAG_ACCESS:
    //             addAccess(attrs);
    //             break;
    //         case SUMO_TAG_PT_LINE:
    //             addPTLine(attrs);
    //             break;
    //         case SUMO_TAG_ROUTE:
    //             if (myCurrentLine == nullptr) {
    //                 addRoute(attrs);
    //             } else {
    //                 addPTLineRoute(attrs);
    //             }
    //             break;
    //         case SUMO_TAG_FLOW:
    //         case SUMO_TAG_TRIP:
    //             addPTLineFromFlow(attrs);
    //             break;
    //         case SUMO_TAG_PARAM: {
    //             bool ok = true;
    //             const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //             if (myCurrentLine != nullptr) {
    //                 if (key == "completeness") {
    //                     myCurrentCompletion = attrs.get<double>(SUMO_ATTR_VALUE, nullptr, ok);
    //                 } else if (key == "name") {
    //                     myCurrentLine->setName(attrs.get<std::string>(SUMO_ATTR_VALUE, nullptr, ok));
    //                 } else if (key == "missingBefore") {
    //                     myMissingBefore = attrs.get<int>(SUMO_ATTR_VALUE, nullptr, ok);
    //                 } else if (key == "missingAfter") {
    //                     myMissingAfter = attrs.get<int>(SUMO_ATTR_VALUE, nullptr, ok);
    //                 }
    //             } else if (myCurrentStop != nullptr) {
    //                 const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //                 myCurrentStop->setParameter(key, val);
    //             }
    //         }
    //         break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::myStartElement")
    }

    /// C++: `void NIXMLPTHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIXMLPTHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_BUS_STOP:
    //         case SUMO_TAG_TRAIN_STOP:
    //             myCurrentStop = nullptr;
    //             myCurrentStopWasIgnored = false;
    //             break;
    //         case SUMO_TAG_PT_LINE:
    //         case SUMO_TAG_FLOW:
    //         case SUMO_TAG_TRIP:
    //             if (myCurrentLine != nullptr) {
    //                 myCurrentLine->setNumOfStops((int)((double)myCurrentLine->getStops().size() / myCurrentCompletion), myMissingBefore, myMissingAfter);
    //             }
    //             myCurrentLine = nullptr;
    //             break;
    //         case SUMO_TAG_ROUTE:
    //             myCurrentRouteID = "";
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port NIXMLPTHandler::myEndElement")
    }

    /// C++: `PositionVector NIXMLPTHandler::tryGetShape(const SUMOSAXAttributes &)` [private]
    /// Tries to parse the shape definition
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn try_get_shape(&mut self, attrs: &SUMOSAXAttributes) -> PositionVector {
        todo!("port NIXMLPTHandler::tryGetShape")
    }

    /// C++: `LaneSpreadFunction NIXMLPTHandler::tryGetLaneSpread(const SUMOSAXAttributes &)` [private]
    /// Tries to parse the spread type
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn try_get_lane_spread(&mut self, attrs: &SUMOSAXAttributes) -> LaneSpreadFunction {
        todo!("port NIXMLPTHandler::tryGetLaneSpread")
    }

    /// C++: `bool NIXMLPTHandler::setNodes(const SUMOSAXAttributes &)` [private]
    /// Sets from/to node information of the currently parsed edge
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_nodes(&mut self, attrs: &SUMOSAXAttributes) -> bool {
        todo!("port NIXMLPTHandler::setNodes")
    }

    /// C++: `void NIXMLPTHandler::addPTStop(int, const SUMOSAXAttributes &)` [private]
    /// Parses an public transport stop
    // void
    // NIXMLPTHandler::addPTStop(int element, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "busStop", ok);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     const std::string laneID = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), ok);
    //     double startPos = attrs.get<double>(SUMO_ATTR_STARTPOS, id.c_str(), ok);
    //     double endPos = attrs.get<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok);
    //     const double parkingLength = attrs.getOpt<double>(SUMO_ATTR_PARKING_LENGTH, id.c_str(), ok, 0);
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), ok, RGBColor(false));
    //     const std::string lines = attrs.getOpt<std::string>(SUMO_ATTR_LINES, id.c_str(), ok, "");
    //     int laneIndex = NBEdge::getLaneIndexFromLaneID(laneID);
    //     std::string edgeID = SUMOXMLDefinitions::getEdgeIDFromLane(laneID);
    //     NBEdge* edge = myEdgeCont.retrieve(edgeID);
    //     if (edge == nullptr) {
    //         edge = myEdgeCont.retrieve(edgeID, true);
    //         if (edge != nullptr && myEdgeCont.getSplit(edge) == nullptr) {
    //             // splits are treated later
    //             edge = nullptr;
    //         }
    //     }
    //     if (edge == nullptr) {
    //         if (!reportError(TL("Edge '%' for stop '%' not found"), edgeID, id)) {
    //             myCurrentStopWasIgnored = true;
    //             NBPTStopCont::addIgnored(id);
    //         }
    //         return;
    //     }
    //     if (edge->getNumLanes() <= laneIndex) {
    //         if (myWarnOnly) {
    //             WRITE_WARNINGF(TL("Lane '%' for stop '%' not found. Using leftmost lane"), laneID, id);
    //             laneIndex = edge->getNumLanes() - 1;
    //         } else {
    //             WRITE_ERRORF(TL("Lane '%' for stop '%' not found"), laneID, id);
    //             return;
    //         }
    //     }
    //     SVCPermissions permissions = edge->getPermissions(laneIndex);
    //     // possibly the stops were written for a different network. If the lane is not a typical public transport stop lane, assume bus as the default
    //     if (!isRailway(permissions) && permissions != SVC_SHIP && permissions != SVC_TAXI) {
    //         permissions = SVC_BUS;
    //     }
    //     if (ok) {
    //         if (startPos < 0) {
    //             startPos += edge->getLoadedLength();
    //         }
    //         if (endPos < 0) {
    //             endPos += edge->getLoadedLength();
    //         }
    //         if (myEdgeCont.wasRemoved(edgeID) && (
    //                     startPos >= endPos || startPos < 0 || endPos < 0
    //                     || startPos >= edge->getLoadedLength()
    //                     || endPos >= edge->getLoadedLength())) {
    //             NBEdge* longest = myEdgeCont.getSplitBase(edgeID);
    //             if (longest != nullptr) {
    //                 edge = longest;
    //             }
    //         }
    //         Position pos = edge->geometryPositionAtOffset((startPos + endPos) / 2);
    //         myCurrentStop = std::make_shared<NBPTStop>((SumoXMLTag)element, id, pos, edgeID, edgeID, endPos - startPos, name, permissions, parkingLength, color, startPos);
    //         while (myEdgeCont.getSplit(edge) != nullptr) {
    //             myCurrentStop->resetLoaded();
    //             const std::pair<NBEdge*, NBEdge*> split = *myEdgeCont.getSplit(edge);
    //             if (myCurrentStop->replaceEdge(edgeID, {split.first, split.second})) {
    //                 edge = split.first->getID() == myCurrentStop->getEdgeId() ? split.first : split.second;
    //                 edgeID = edge->getID();
    //             }
    //         }
    //         for (const std::string& line : StringTokenizer(lines).getVector()) {
    //             myCurrentStop->addLine(line);
    //         }
    //         if (!myStopCont.insert(myCurrentStop)) {
    //             WRITE_ERRORF(TL("Could not add public transport stop '%' (already exists)"), id);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_pt_stop(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addPTStop")
    }

    /// C++: `void NIXMLPTHandler::addPTLineRoute(const SUMOSAXAttributes &)` [private]
    /// Parses a route as port of a public transport line
    // void
    // NIXMLPTHandler::addPTLineRoute(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentLine == nullptr) {
    //         WRITE_ERROR(TL("Found route outside line definition"));
    //         return;
    //     }
    //     bool ok = true;
    //     const std::vector<std::string>& edgeIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, nullptr, ok);
    //     EdgeVector edges;
    //     for (const std::string& edgeID : edgeIDs) {
    //         NBEdge* edge = myEdgeCont.retrieve(edgeID);
    //         if (edge == nullptr) {
    //             reportError(TL("Edge '%' in route of line '%' not found"), edgeID, myCurrentLine->getLineID());
    //         } else {
    //             edges.push_back(edge);
    //         }
    //     }
    //     myCurrentLine->setEdges(edges);
    // }
    #[allow(unused_variables)]
    pub fn add_pt_line_route(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addPTLineRoute")
    }

    /// C++: `void NIXMLPTHandler::addRoute(const SUMOSAXAttributes &)` [private]
    /// Parses a stand-alone route when parsing implicit ptlines from routes and flows
    // void
    // NIXMLPTHandler::addRoute(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myCurrentRouteID = attrs.get<std::string>(SUMO_ATTR_ID, "route", ok);
    //     const std::vector<std::string>& edgeIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, myCurrentRouteID.c_str(), ok);
    //     EdgeVector edges;
    //     for (const std::string& edgeID : edgeIDs) {
    //         NBEdge* edge = myEdgeCont.retrieve(edgeID);
    //         if (edge == nullptr) {
    //             reportError(TL("Edge '%' in route of line '%' not found"), edgeID, myCurrentLine->getLineID());
    //         } else {
    //             edges.push_back(edge);
    //         }
    //     }
    //     myRouteEdges[myCurrentRouteID] = edges;
    // }
    #[allow(unused_variables)]
    pub fn add_route(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addRoute")
    }

    /// C++: `void NIXMLPTHandler::addPTLineStop(const SUMOSAXAttributes &)` [private]
    /// Parses an public transport stop reference within a line element
    // void
    // NIXMLPTHandler::addPTLineStop(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string id = attrs.hasAttribute(SUMO_ATTR_ID)
    //                            ? attrs.get<std::string>(SUMO_ATTR_ID, "ptLine", ok)
    //                            : attrs.get<std::string>(SUMO_ATTR_BUS_STOP, "ptline", ok);
    //     std::shared_ptr<NBPTStop> stop = myStopCont.get(id);
    //     if (stop == nullptr) {
    //         if (!NBPTStopCont::wasIgnored(id)) {
    //             WRITE_ERRORF(TL("Stop '%' within line '%' not found"), id, toString(myCurrentLine->getLineID()));
    //         }
    //         return;
    //     }
    //     myCurrentLine->addPTStop(stop);
    // }
    #[allow(unused_variables)]
    pub fn add_pt_line_stop(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addPTLineStop")
    }

    /// C++: `void NIXMLPTHandler::addRouteStop(const SUMOSAXAttributes &)` [private]
    /// Parses an public transport stop reference within a route element
    // void
    // NIXMLPTHandler::addRouteStop(const SUMOSAXAttributes& attrs) {
    //     assert(myCurrentRouteID != "");
    //     bool ok = true;
    //     const std::string id = attrs.hasAttribute(SUMO_ATTR_ID)
    //                            ? attrs.get<std::string>(SUMO_ATTR_ID, "ptLine", ok)
    //                            : attrs.get<std::string>(SUMO_ATTR_BUS_STOP, "ptline", ok);
    //     std::shared_ptr<NBPTStop> stop = myStopCont.get(id);
    //     if (stop == nullptr) {
    //         WRITE_ERRORF(TL("Stop '%' within route '%' not found"), id, toString(myCurrentRouteID));
    //         return;
    //     }
    //     myRouteStops[myCurrentRouteID].push_back(stop);
    // }
    #[allow(unused_variables)]
    pub fn add_route_stop(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addRouteStop")
    }

    /// C++: `void NIXMLPTHandler::addAccess(const SUMOSAXAttributes &)` [private]
    /// Parses an stop access definition
    // void
    // NIXMLPTHandler::addAccess(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentStop == nullptr) {
    //         if (myCurrentStopWasIgnored) {
    //             return;
    //         } else {
    //             throw InvalidArgument("Could not add access outside a stopping place.");
    //         }
    //     }
    //     bool ok = true;
    //     const std::string laneID = attrs.get<std::string>(SUMO_ATTR_LANE, "access", ok);
    //     const std::string edgeID = SUMOXMLDefinitions::getEdgeIDFromLane(laneID);
    //     if (myEdgeCont.retrieve(edgeID) == nullptr) {
    //         reportError(TL("Edge '%' for access to stop '%' not found"), edgeID, myCurrentStop->getID());
    //         return;
    //     }
    //     const double pos = attrs.get<double>(SUMO_ATTR_POSITION, "access", ok);
    //     const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, "access", ok, -1);
    //     if (ok) {
    //         myCurrentStop->addAccess(laneID, pos, length);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_access(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addAccess")
    }

    /// C++: `void NIXMLPTHandler::addPTLine(const SUMOSAXAttributes &)` [private]
    /// Parses a public transport line
    // void
    // NIXMLPTHandler::addPTLine(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "ptLine", ok);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     const std::string line = attrs.getOpt<std::string>(SUMO_ATTR_LINE, id.c_str(), ok, "");
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //     SUMOVehicleClass vClass = NIImporter_OpenStreetMap::interpretTransportType(type);
    //     if (attrs.hasAttribute(SUMO_ATTR_VCLASS)) {
    //         vClass = getVehicleClassID(attrs.get<std::string>(SUMO_ATTR_VCLASS, id.c_str(), ok));
    //     }
    //     RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), ok, RGBColor(false));
    //     const int intervalS = attrs.getOpt<int>(SUMO_ATTR_PERIOD, id.c_str(), ok, -1);
    //     const std::string nightService = attrs.getStringSecure("nightService", "");
    //     myCurrentCompletion = StringUtils::toDouble(attrs.getStringSecure("completeness", "1"));
    //     myMissingBefore = StringUtils::toInt(attrs.getStringSecure("missingBefore", "0"));
    //     myMissingAfter = StringUtils::toInt(attrs.getStringSecure("missingAfter", "0"));
    //     if (ok) {
    //         // patching existing line?
    //         myCurrentLine = myLineCont.retrieve(id);
    //         if (myCurrentLine == nullptr) {
    //             myCurrentLine = new NBPTLine(id, name, type, line, intervalS / 60, nightService, vClass, color);
    //             myLineCont.insert(myCurrentLine);
    //         } else {
    //             WRITE_MESSAGEF(TL("Duplicate ptLine id occurred ('%'); assuming overwriting is wished."), id);
    //             if (name != "") {
    //                 myCurrentLine->setName(name);
    //             }
    //             if (line != "") {
    //                 myCurrentLine->setRef(line);
    //             }
    //             if (intervalS != -1) {
    //                 myCurrentLine->setPeriod(intervalS);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_pt_line(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addPTLine")
    }

    /// C++: `void NIXMLPTHandler::addPTLineFromFlow(const SUMOSAXAttributes &)` [private]
    /// Parses a public transport line
    // void
    // NIXMLPTHandler::addPTLineFromFlow(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myMissingBefore = 0;
    //     myMissingAfter = 0;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "flow", ok);
    //     const std::string line = attrs.get<std::string>(SUMO_ATTR_LINE, id.c_str(), ok);
    //     const std::string type = attrs.get<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok);
    //     const std::string route = attrs.get<std::string>(SUMO_ATTR_ROUTE, id.c_str(), ok);
    //     SUMOVehicleClass vClass = NIImporter_OpenStreetMap::interpretTransportType(type);
    //     const int intervalS = attrs.getOpt<int>(SUMO_ATTR_PERIOD, id.c_str(), ok, -1);
    //     RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), ok, RGBColor(false));
    //     if (ok) {
    //         myCurrentLine = new NBPTLine(id, "", type, line, intervalS / 60, "", vClass, color);
    //         myCurrentLine->setEdges(myRouteEdges[route]);
    //         for (std::shared_ptr<NBPTStop> stop : myRouteStops[route]) {
    //             myCurrentLine->addPTStop(stop);
    //         }
    //         myLineCont.insert(myCurrentLine);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_pt_line_from_flow(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLPTHandler::addPTLineFromFlow")
    }

    /// C++: `bool NIXMLPTHandler::reportError(const std::string &, const std::string &, const std::string &)` [private]
    /// reprot error or warning depending on options and explicit edge removal
    // bool
    // NIXMLPTHandler::reportError(const std::string& msg, const std::string& edgeID, const std::string& id) {
    //     const bool ignored = myEdgeCont.wasIgnored(edgeID);
    //     if (!ignored && !myWarnOnly) {
    //         WRITE_ERRORF(msg, edgeID, id);
    //         return true;
    //     } else if (!ignored) {
    //         WRITE_WARNINGF(msg, edgeID, id);
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn report_error(&mut self, msg: &str, edgeID: &str, id: &str) -> bool {
        todo!("port NIXMLPTHandler::reportError")
    }

    /// C++: `void NIXMLPTHandler::NIXMLPTHandler(const NIXMLPTHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLPTHandler) -> Self {
        todo!("port NIXMLPTHandler::NIXMLPTHandler")
    }

    /// C++: `NIXMLPTHandler & NIXMLPTHandler::operator=(const NIXMLPTHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLPTHandler) -> &mut NIXMLPTHandler {
        todo!("port NIXMLPTHandler::operator=")
    }

}

