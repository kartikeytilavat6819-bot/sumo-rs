//! Port of netimport/NIXMLTrafficLightsHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Importer for edge connections stored in XML
/// C++ class `NIXMLTrafficLightsHandler` (netimport/NIXMLTrafficLightsHandler.h)
pub struct NIXMLTrafficLightsHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// The traffic light container to fill
    pub myTLLCont: Ptr<NBTrafficLightLogicCont>,
    /// The edge container for retrieving edges
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The currently parsed traffic light
    pub myCurrentTL: Ptr<NBLoadedSUMOTLDef>,
    /// whether phases of a previously loaded traffic light must be reset
    pub myResetPhases: bool,
    /// whether definitions for unknown traffic lights shall be silently ignored
    pub myIgnoreUnknown: bool,
    /// list of traffic light logics that were found
    pub mySeenIDs: BTreeSet<String>,
}

impl std::ops::Deref for NIXMLTrafficLightsHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLTrafficLightsHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl NIXMLTrafficLightsHandler {
    /// C++: `void NIXMLTrafficLightsHandler::NIXMLTrafficLightsHandler(NBTrafficLightLogicCont &, NBEdgeCont &, bool)`
    /// Constructor
    // NIXMLTrafficLightsHandler::NIXMLTrafficLightsHandler(
    //     NBTrafficLightLogicCont& tlCont, NBEdgeCont& ec, bool ignoreUnknown) :
    //     SUMOSAXHandler("xml-tllogics"),
    //     myTLLCont(tlCont),
    //     myEdgeCont(ec),
    //     myCurrentTL(nullptr),
    //     myResetPhases(false),
    //     myIgnoreUnknown(ignoreUnknown)
    // { }
    #[allow(unused_variables)]
    pub fn new(tlCont: &mut NBTrafficLightLogicCont, ec: &mut NBEdgeCont, ignoreUnknown: bool) -> Self {
        todo!("port NIXMLTrafficLightsHandler::NIXMLTrafficLightsHandler")
    }

    /// C++: `void NIXMLTrafficLightsHandler::~NIXMLTrafficLightsHandler() noexcept` [virtual]
    /// Destructor
    // NIXMLTrafficLightsHandler::~NIXMLTrafficLightsHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIXMLTrafficLightsHandler::~NIXMLTrafficLightsHandler")
    }

    /// C++: `const std::set<std::string> & NIXMLTrafficLightsHandler::getSeenIDs()`
    // const std::set<std::string>& getSeenIDs() {
    //         return mySeenIDs;
    //     }
    #[allow(unused_variables)]
    pub fn get_seen_i_ds(&mut self) -> &BTreeSet<String> {
        todo!("port NIXMLTrafficLightsHandler::getSeenIDs")
    }

    /// C++: `void NIXMLTrafficLightsHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIXMLTrafficLightsHandler::myStartElement(
    //     int element, const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_TLLOGIC:
    //             myCurrentTL = initTrafficLightLogic(attrs, myCurrentTL);
    //             break;
    //         case SUMO_TAG_PHASE:
    //             if (myCurrentTL != nullptr) {
    //                 if (myResetPhases) {
    //                     myCurrentTL->getLogic()->resetPhases();
    //                     myResetPhases = false;
    //                 }
    //                 NIImporter_SUMO::addPhase(attrs, myCurrentTL);
    //                 myCurrentTL->phasesLoaded();
    //             }
    //             break;
    //         case SUMO_TAG_CONNECTION:
    //             addTlConnection(attrs);
    //             break;
    //         case SUMO_TAG_DEL:
    //             removeTlConnection(attrs);
    //             break;
    //         case SUMO_TAG_PARAM:
    //             if (myCurrentTL != nullptr) {
    //                 bool ok = true;
    //                 const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //                 // circumventing empty string test
    //                 const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //                 myCurrentTL->setParameter(key, val);
    //             }
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLTrafficLightsHandler::myStartElement")
    }

    /// C++: `void NIXMLTrafficLightsHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIXMLTrafficLightsHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_TLLOGIC:
    //             myCurrentTL = nullptr;
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port NIXMLTrafficLightsHandler::myEndElement")
    }

    /// C++: `NBLoadedSUMOTLDef * NIXMLTrafficLightsHandler::initTrafficLightLogic(const SUMOSAXAttributes &, NBLoadedSUMOTLDef *)` [private]
    /// begins the reading of a traffic lights logic This differs from NIImporter_SUMO::initTrafficLightLogic insofar as partial definitions (diffs) are allowed in an xml file
    // NBLoadedSUMOTLDef*
    // NIXMLTrafficLightsHandler::initTrafficLightLogic(const SUMOSAXAttributes& attrs, NBLoadedSUMOTLDef* currentTL) {
    //     if (currentTL) {
    //         WRITE_ERRORF(TL("Definition of tlLogic '%' was not finished."), currentTL->getID());
    //         return nullptr;
    //     }
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     std::string programID = attrs.getOpt<std::string>(SUMO_ATTR_PROGRAMID, id.c_str(), ok, "UNKNOWN_PROGRAM");
    //     const SUMOTime offset = attrs.getOptOffsetReporting(SUMO_ATTR_OFFSET, id.c_str(), ok, 0);
    //     std::string typeS = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, nullptr, ok,
    //                         OptionsCont::getOptions().getString("tls.default-type"));
    //     TrafficLightType type;
    //     if (SUMOXMLDefinitions::TrafficLightTypes.hasString(typeS)) {
    //         type = SUMOXMLDefinitions::TrafficLightTypes.get(typeS);
    //     } else {
    //         WRITE_ERRORF(TL("Unknown traffic light type '%' for tlLogic '%'."), typeS, id);
    //         return nullptr;
    //     }
    //     // there are three scenarios to consider
    //     // 1) the tll.xml is loaded to update traffic lights defined in a net.xml:
    //     //   simply retrieve the loaded definitions and update them
    //     // 2) the tll.xml is loaded to define new traffic lights
    //     //   nod.xml will have triggered building of NBOwnTLDef. Replace it with NBLoadedSUMOTLDef
    //     // 3) the tll.xml is loaded to define new programs for a defined traffic light
    //     //   there should be a definition with the same id but different programID
    //     const std::map<std::string, NBTrafficLightDefinition*>& programs = myTLLCont.getPrograms(id);
    //     if (programs.size() == 0) {
    //         if (!myIgnoreUnknown) {
    //             WRITE_ERRORF(TL("Cannot load traffic light program for unknown id '%', programID '%'."), id, programID);
    //         }
    //         return nullptr;
    //     }
    //     const std::string existingProgram = programs.begin()->first; // arbitrary for our purpose
    //     NBLoadedSUMOTLDef* loadedDef = dynamic_cast<NBLoadedSUMOTLDef*>(myTLLCont.getDefinition(id, programID));
    //     if (loadedDef == nullptr) {
    //         NBLoadedSUMOTLDef* oldDef = dynamic_cast<NBLoadedSUMOTLDef*>(myTLLCont.getDefinition(id, existingProgram));
    //         if (oldDef == nullptr) {
    //             // case 2
    //             NBTrafficLightDefinition* newDef = dynamic_cast<NBOwnTLDef*>(myTLLCont.getDefinition(
    //                                                    id, NBTrafficLightDefinition::DefaultProgramID));
    //             bool deleteDefault = false;
    //             if (newDef == nullptr) {
    //                 // the default program may have already been replaced with a loaded program
    //                 newDef = dynamic_cast<NBLoadedSUMOTLDef*>(myTLLCont.getDefinition(
    //                              id, NBTrafficLightDefinition::DefaultProgramID));
    //                 if (newDef == nullptr) {
    //                     WRITE_ERRORF(TL("Cannot load traffic light program for unknown id '%', programID '%'."), id, programID);
    //                     return nullptr;
    //                 }
    //             } else {
    //                 deleteDefault = true;
    //             }
    //             assert(newDef != nullptr);
    //             loadedDef = new NBLoadedSUMOTLDef(id, programID, offset, type);
    //             // copy nodes and controlled inner edges
    //             for (NBNode* const n : newDef->getNodes()) {
    //                 loadedDef->addNode(n);
    //             }
    //             loadedDef->addControlledInnerEdges(newDef->getControlledInnerEdges());
    //             if (deleteDefault) {
    //                 // make a copy because the vector is modified in the loop
    //                 const std::vector<NBNode*> nodes = newDef->getNodes();
    //                 // replace default Program
    //                 for (NBNode* const n : nodes) {
    //                     n->removeTrafficLight(newDef);
    //                 }
    //                 myTLLCont.removeProgram(id, NBTrafficLightDefinition::DefaultProgramID);
    //             }
    //             myTLLCont.insert(loadedDef);
    //         } else {
    //             // case 3
    //             NBTrafficLightLogic* oldLogic = oldDef->getLogic();
    //             NBTrafficLightLogic newLogic(id, programID, oldLogic->getNumLinks(), offset, type);
    //             loadedDef = new NBLoadedSUMOTLDef(*oldDef, newLogic);
    //             // copy nodes
    //             std::vector<NBNode*> nodes = oldDef->getNodes();
    //             for (std::vector<NBNode*>::iterator it = nodes.begin(); it != nodes.end(); it++) {
    //                 loadedDef->addNode(*it);
    //             }
    //             //std::cout << " case3 oldDef=" << oldDef->getDescription() << " loadedDef=" << loadedDef->getDescription() << "\n";
    //             myTLLCont.insert(loadedDef);
    //         }
    //     } else {
    //         // case 1
    //         if (attrs.hasAttribute(SUMO_ATTR_OFFSET)) {
    //             loadedDef->setOffset(offset);
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_TYPE)) {
    //             loadedDef->setType(type);
    //         }
    //     }
    //     if (ok) {
    //         myResetPhases = true;
    //         mySeenIDs.insert(id);
    //         return loadedDef;
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_traffic_light_logic(&mut self, attrs: &SUMOSAXAttributes, currentTL: Ptr<NBLoadedSUMOTLDef>) -> Ptr<NBLoadedSUMOTLDef> {
        todo!("port NIXMLTrafficLightsHandler::initTrafficLightLogic")
    }

    /// C++: `void NIXMLTrafficLightsHandler::addTlConnection(const SUMOSAXAttributes &)` [private]
    /// reads and adds tl-controlled connection
    // void
    // NIXMLTrafficLightsHandler::addTlConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // parse identifying attributes
    //     NBEdge* from = retrieveEdge(attrs, SUMO_ATTR_FROM, ok);
    //     NBEdge* to = retrieveEdge(attrs, SUMO_ATTR_TO, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     int fromLane = retrieveLaneIndex(attrs, SUMO_ATTR_FROM_LANE, from, ok);
    //     int toLane = retrieveLaneIndex(attrs, SUMO_ATTR_TO_LANE, to, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     // retrieve connection
    //     const std::vector<NBEdge::Connection>& connections = from->getConnections();
    //     std::vector<NBEdge::Connection>::const_iterator con_it;
    //     con_it = find_if(connections.begin(), connections.end(),
    //                      NBEdge::connections_finder(fromLane, to, toLane));
    //     if (con_it == connections.end()) {
    //         WRITE_ERROR("Connection from=" + from->getID() + " to=" + to->getID() +
    //                     " fromLane=" + toString(fromLane) + " toLane=" + toString(toLane) + " not found");
    //         return;
    //     }
    //     NBEdge::Connection c = *con_it;
    //     SumoXMLNodeType type = from->getToNode()->getType();
    //     if (type == SumoXMLNodeType::RAIL_CROSSING || type == SumoXMLNodeType::RAIL_SIGNAL) {
    //         // no traffic light programs are built for these, indices do not need to be defined
    //         return;
    //     }
    //     // read other  attributes
    //     std::string tlID = attrs.getOpt<std::string>(SUMO_ATTR_TLID, nullptr, ok, "");
    //     if (tlID == "") {
    //         // we are updating an existing tl-controlled connection
    //         assert(from->getToNode()->getControllingTLS().size() > 0);
    //         tlID = (*(from->getToNode()->getControllingTLS().begin()))->getID();
    //     }
    //     int tlIndex = attrs.getOpt<int>(SUMO_ATTR_TLLINKINDEX, nullptr, ok, -1);
    //     if (tlIndex == -1) {
    //         // we are updating an existing tl-controlled connection
    //         tlIndex = c.tlLinkIndex;
    //     }
    //     int tlIndex2 = attrs.getOpt<int>(SUMO_ATTR_TLLINKINDEX2, nullptr, ok, -1);
    //     if (tlIndex2 == -1) {
    //         // we are updating an existing tl-controlled connection or index2 is not used
    //         tlIndex2 = c.tlLinkIndex2;
    //     }
    //
    //     // register the connection with all definitions
    //     const std::map<std::string, NBTrafficLightDefinition*>& programs = myTLLCont.getPrograms(tlID);
    //     if (programs.size() > 0) {
    //         std::map<std::string, NBTrafficLightDefinition*>::const_iterator it;
    //         for (it = programs.begin(); it != programs.end(); it++) {
    //             NBLoadedSUMOTLDef* tlDef = dynamic_cast<NBLoadedSUMOTLDef*>(it->second);
    //             if (tlDef) {
    //                 tlDef->addConnection(from, c.toEdge, c.fromLane, c.toLane, tlIndex, tlIndex2,  false);
    //             } else {
    //                 throw ProcessError("Corrupt traffic light definition '"
    //                                    + tlID + "' (program '" + it->first + "')");
    //             }
    //         }
    //     } else {
    //         WRITE_ERRORF(TL("The traffic light '%' is not known."), tlID);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_tl_connection(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLTrafficLightsHandler::addTlConnection")
    }

    /// C++: `void NIXMLTrafficLightsHandler::removeTlConnection(const SUMOSAXAttributes &)` [private]
    /// reads and removes tl-controlled connection
    // void
    // NIXMLTrafficLightsHandler::removeTlConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string tlID = attrs.get<std::string>(SUMO_ATTR_TLID, nullptr, ok);
    //     // does the traffic light still exist?
    //     const std::map<std::string, NBTrafficLightDefinition*>& programs = myTLLCont.getPrograms(tlID);
    //     if (programs.size() > 0) {
    //         // parse identifying attributes
    //         NBEdge* from = retrieveEdge(attrs, SUMO_ATTR_FROM, ok);
    //         NBEdge* to = retrieveEdge(attrs, SUMO_ATTR_TO, ok);
    //         int fromLane = -1;
    //         int toLane = -1;
    //         if (ok) {
    //             fromLane = retrieveLaneIndex(attrs, SUMO_ATTR_FROM_LANE, from, ok, true);
    //             toLane = retrieveLaneIndex(attrs, SUMO_ATTR_TO_LANE, to, ok, true);
    //         }
    //         int tlIndex = attrs.get<int>(SUMO_ATTR_TLLINKINDEX, nullptr, ok);
    //
    //         NBConnection conn(from, fromLane, to, toLane, tlIndex);
    //         // remove the connection from all definitions
    //         std::map<std::string, NBTrafficLightDefinition*>::const_iterator it;
    //         for (it = programs.begin(); it != programs.end(); it++) {
    //             NBLoadedSUMOTLDef* tlDef = dynamic_cast<NBLoadedSUMOTLDef*>(it->second);
    //             if (tlDef) {
    //                 tlDef->removeConnection(conn, false);
    //             } else {
    //                 throw ProcessError("Corrupt traffic light definition '"
    //                                    + tlID + "' (program '" + it->first + "')");
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_tl_connection(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port NIXMLTrafficLightsHandler::removeTlConnection")
    }

    /// C++: `NBEdge * NIXMLTrafficLightsHandler::retrieveEdge(const SUMOSAXAttributes &, SumoXMLAttr, bool &)` [private]
    /// parses and edge id an returns an existing edge
    // NBEdge*
    // NIXMLTrafficLightsHandler::retrieveEdge(
    //     const SUMOSAXAttributes& attrs, SumoXMLAttr attr, bool& ok) {
    //     std::string edgeID = attrs.get<std::string>(attr, nullptr, ok);
    //     NBEdge* edge = myEdgeCont.retrieve(edgeID, true);
    //     if (edge == nullptr) {
    //         WRITE_ERRORF(TL("Unknown edge '%' given in connection."), edgeID);
    //         ok = false;
    //     }
    //     return edge;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_edge(&mut self, attrs: &SUMOSAXAttributes, attr: SumoXMLAttr, ok: &mut bool) -> Ptr<NBEdge> {
        todo!("port NIXMLTrafficLightsHandler::retrieveEdge")
    }

    /// C++: `int NIXMLTrafficLightsHandler::retrieveLaneIndex(const SUMOSAXAttributes &, SumoXMLAttr, NBEdge *, bool &, bool)` [private]
    /// parses a lane index and verifies its correctness
    // int
    // NIXMLTrafficLightsHandler::retrieveLaneIndex(
    //     const SUMOSAXAttributes& attrs, SumoXMLAttr attr, NBEdge* edge, bool& ok, bool isDelete) {
    //     int laneIndex = attrs.get<int>(attr, nullptr, ok);
    //     if (edge->getNumLanes() <= laneIndex) {
    //         if (!isDelete) {
    //             WRITE_ERRORF(TL("Invalid lane index '%' for edge '%'."), toString(laneIndex), edge->getID());
    //         }
    //         ok = false;
    //     }
    //     return laneIndex;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_lane_index(&mut self, attrs: &SUMOSAXAttributes, attr: SumoXMLAttr, edge: Ptr<NBEdge>, ok: &mut bool, isDelete: bool) -> i32 {
        todo!("port NIXMLTrafficLightsHandler::retrieveLaneIndex")
    }

    /// C++: `void NIXMLTrafficLightsHandler::NIXMLTrafficLightsHandler(const NIXMLTrafficLightsHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLTrafficLightsHandler) -> Self {
        todo!("port NIXMLTrafficLightsHandler::NIXMLTrafficLightsHandler")
    }

    /// C++: `NIXMLTrafficLightsHandler & NIXMLTrafficLightsHandler::operator=(const NIXMLTrafficLightsHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLTrafficLightsHandler) -> &mut NIXMLTrafficLightsHandler {
        todo!("port NIXMLTrafficLightsHandler::operator=")
    }

}

