//! Port of netimport/NIXMLPTHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Every method is a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_message, write_warning};
use utils::common::rgb_color::RGBColor;
use utils::tlf;
use crate::netimport::ni_importer_open_street_map::NIImporter_OpenStreetMap;
use utils::common::string_tokenizer::StringTokenizer;
use utils::common::string_utils::{to_double, to_int};
use utils::common::sumo_vehicle_class::{SVC_BUS, SVC_SHIP, SVC_TAXI, get_vehicle_class_id, is_railway};
use utils::options::options_cont::get_bool;
use utils::xml::sumo_xml_definitions::get_edge_id_from_lane;

/// shortcut: NBPTStop is held as Rc but C++ mutates it in place; mutate through the Rc pointer until stop containers hold Ptr<NBPTStop>
#[allow(clippy::mut_from_ref)]
fn stop_mut(stop: &Rc<NBPTStop>) -> &mut NBPTStop {
    unsafe { &mut *(Rc::as_ptr(stop) as *mut NBPTStop) }
}

/// shortcut: SUMOSAXAttributes has no FromAttr impl for RGBColor, so getOpt<RGBColor> is emulated here, upgrade when utils implements FromAttr for RGBColor
fn get_opt_color(attrs: &SUMOSAXAttributes, attr: i32, objectid: &str, ok: &mut bool) -> RGBColor {
    if !attrs.has_attribute(attr) {
        return RGBColor::new_valid(false);
    }
    let value = attrs.get_string(attr);
    match RGBColor::parse_color(&value) {
        Ok(c) => c,
        Err(_) => RGBColor::parse_color_reporting(&value, attrs.get_object_type(), Some(objectid), true, ok).unwrap_or_default(),
    }
}

/// Importer for network edges stored in XML
/// C++ class `NIXMLPTHandler` (netimport/NIXMLPTHandler.h)
#[repr(C)]
pub struct NIXMLPTHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The edges container (for retrieving referenced stop edge)
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The stop container (for loading of stops)
    pub myStopCont: Ptr<NBPTStopCont>,
    /// The line container (for loading of lines)
    pub myLineCont: Ptr<NBPTLineCont>,
    /// The currently processed stop (None == C++ nullptr; Rc cannot be null)
    pub myCurrentStop: Option<Rc<NBPTStop>>,
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
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLPTHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
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
        NIXMLPTHandler {
            base: GenericSAXHandler::new(&TAGS, SUMO_TAG_NOTHING, &ATTRS, SUMO_ATTR_NOTHING, "public transport - file", ""),
            myEdgeCont: Ptr::from_mut(ec),
            myStopCont: Ptr::from_mut(sc),
            myLineCont: Ptr::from_mut(lc),
            myCurrentStop: None,
            myCurrentLine: Ptr::null(),
            myCurrentRouteID: String::new(),
            myCurrentCompletion: 0.,
            myLastParameterised: Vec::new(),
            myRouteStops: BTreeMap::new(),
            myRouteEdges: BTreeMap::new(),
            myCurrentStopWasIgnored: false,
            myWarnOnly: get_bool("ignore-errors"),
            myMissingBefore: 0,
            myMissingAfter: 0,
        }
    }

    /// C++: `void NIXMLPTHandler::~NIXMLPTHandler() noexcept` [virtual]
    /// Destructor
    // NIXMLPTHandler::~NIXMLPTHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty ({}); members are released by their own destructors
        // (myCurrentStop is an Rc and is dropped with the struct)
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
        match element {
            SUMO_TAG_BUS_STOP | SUMO_TAG_TRAIN_STOP | SUMO_TAG_STOP => {
                if self.myCurrentRouteID != "" {
                    self.add_route_stop(attrs);
                } else if self.myCurrentLine.is_null() {
                    self.add_pt_stop(element, attrs);
                } else {
                    self.add_pt_line_stop(attrs);
                }
            }
            SUMO_TAG_ACCESS => {
                self.add_access(attrs);
            }
            SUMO_TAG_PT_LINE => {
                self.add_pt_line(attrs);
            }
            SUMO_TAG_ROUTE => {
                if self.myCurrentLine.is_null() {
                    self.add_route(attrs);
                } else {
                    self.add_pt_line_route(attrs);
                }
            }
            SUMO_TAG_FLOW | SUMO_TAG_TRIP => {
                self.add_pt_line_from_flow(attrs);
            }
            SUMO_TAG_PARAM => {
                let mut ok = true;
                let key: String = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
                if !self.myCurrentLine.is_null() {
                    if key == "completeness" {
                        self.myCurrentCompletion = attrs.get::<f64>(SUMO_ATTR_VALUE, "", &mut ok, true);
                    } else if key == "name" {
                        let value: String = attrs.get::<String>(SUMO_ATTR_VALUE, "", &mut ok, true);
                        self.myCurrentLine.m().set_name(&value);
                    } else if key == "missingBefore" {
                        self.myMissingBefore = attrs.get::<i32>(SUMO_ATTR_VALUE, "", &mut ok, true);
                    } else if key == "missingAfter" {
                        self.myMissingAfter = attrs.get::<i32>(SUMO_ATTR_VALUE, "", &mut ok, true);
                    }
                } else if let Some(stop) = &self.myCurrentStop {
                    let val: String = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
                    stop_mut(stop).set_parameter(&key, &val);
                }
            }
            _ => {}
        }
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
        match element {
            SUMO_TAG_BUS_STOP | SUMO_TAG_TRAIN_STOP => {
                self.myCurrentStop = None;
                self.myCurrentStopWasIgnored = false;
            }
            SUMO_TAG_PT_LINE | SUMO_TAG_FLOW | SUMO_TAG_TRIP => {
                if !self.myCurrentLine.is_null() {
                    let size = self.myCurrentLine.m().get_stops().len();
                    let numOfStops = ((size as f64) / self.myCurrentCompletion) as i32;
                    self.myCurrentLine.m().set_num_of_stops(numOfStops, self.myMissingBefore, self.myMissingAfter);
                }
                self.myCurrentLine = Ptr::null();
            }
            SUMO_TAG_ROUTE => {
                self.myCurrentRouteID = String::new();
            }
            _ => {}
        }
    }

    /// C++: `PositionVector NIXMLPTHandler::tryGetShape(const SUMOSAXAttributes &)` [private]
    /// Tries to parse the shape definition
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn try_get_shape(&mut self, attrs: &SUMOSAXAttributes) -> PositionVector {
        // declared private in NIXMLPTHandler.h but never defined in NIXMLPTHandler.cpp (the body lives in NIXMLEdgesHandler.cpp);
        // C++ would fail at link time if called, so this panics instead of silently returning a value
        panic!("NIXMLPTHandler::tryGetShape is declared but not defined in NIXMLPTHandler.cpp")
    }

    /// C++: `LaneSpreadFunction NIXMLPTHandler::tryGetLaneSpread(const SUMOSAXAttributes &)` [private]
    /// Tries to parse the spread type
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn try_get_lane_spread(&mut self, attrs: &SUMOSAXAttributes) -> LaneSpreadFunction {
        // declared private in NIXMLPTHandler.h but never defined in NIXMLPTHandler.cpp (the body lives in NIXMLEdgesHandler.cpp);
        // C++ would fail at link time if called, so this panics instead of silently returning a value
        panic!("NIXMLPTHandler::tryGetLaneSpread is declared but not defined in NIXMLPTHandler.cpp")
    }

    /// C++: `bool NIXMLPTHandler::setNodes(const SUMOSAXAttributes &)` [private]
    /// Sets from/to node information of the currently parsed edge
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_nodes(&mut self, attrs: &SUMOSAXAttributes) -> bool {
        // declared private in NIXMLPTHandler.h but never defined in NIXMLPTHandler.cpp (the body lives in NIXMLEdgesHandler.cpp);
        // C++ would fail at link time if called, so this panics instead of silently returning a value
        panic!("NIXMLPTHandler::setNodes is declared but not defined in NIXMLPTHandler.cpp")
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
        let mut ok = true;
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "busStop", &mut ok, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let laneID: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut ok, true);
        let mut startPos: f64 = attrs.get::<f64>(SUMO_ATTR_STARTPOS, &id, &mut ok, true);
        let mut endPos: f64 = attrs.get::<f64>(SUMO_ATTR_ENDPOS, &id, &mut ok, true);
        let parkingLength: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PARKING_LENGTH, &id, &mut ok, 0., true);
        let color: RGBColor = get_opt_color(attrs, SUMO_ATTR_COLOR, &id, &mut ok);
        let lines: String = attrs.get_opt::<String>(SUMO_ATTR_LINES, &id, &mut ok, String::new(), true);
        let mut laneIndex: i32 = NBEdge::get_lane_index_from_lane_id(laneID.clone());
        let mut edgeID: String = get_edge_id_from_lane(&laneID);
        let mut edge: Ptr<NBEdge> = self.myEdgeCont.retrieve(&edgeID, false);
        if edge.is_null() {
            edge = self.myEdgeCont.retrieve(&edgeID, true);
            if !edge.is_null() && self.myEdgeCont.get_split(edge).is_null() {
                // splits are treated later
                edge = Ptr::null();
            }
        }
        if edge.is_null() {
            if !self.report_error("Edge '%' for stop '%' not found", &edgeID, &id) {
                self.myCurrentStopWasIgnored = true;
                NBPTStopCont::add_ignored(&id);
            }
            return;
        }
        if edge.get_num_lanes() <= laneIndex {
            if self.myWarnOnly {
                write_warning(&tlf!("Lane '%' for stop '%' not found. Using leftmost lane", laneID, id));
                laneIndex = edge.get_num_lanes() - 1;
            } else {
                write_error(&tlf!("Lane '%' for stop '%' not found", laneID, id));
                return;
            }
        }
        let mut permissions: SVCPermissions = edge.get_permissions(laneIndex);
        // possibly the stops were written for a different network. If the lane is not a typical public transport stop lane, assume bus as the default
        if !is_railway(permissions) && permissions != SVC_SHIP && permissions != SVC_TAXI {
            permissions = SVC_BUS;
        }
        if ok {
            if startPos < 0. {
                startPos += edge.get_loaded_length();
            }
            if endPos < 0. {
                endPos += edge.get_loaded_length();
            }
            if self.myEdgeCont.was_removed(edgeID.clone()) && (startPos >= endPos || startPos < 0. || endPos < 0. || startPos >= edge.get_loaded_length() || endPos >= edge.get_loaded_length()) {
                let longest: Ptr<NBEdge> = self.myEdgeCont.get_split_base(&edgeID);
                if !longest.is_null() {
                    edge = longest;
                }
            }
            let pos: Position = edge.geometry_position_at_offset((startPos + endPos) / 2.);
            let stopRc: Rc<NBPTStop> = Rc::new(NBPTStop::new(element, id.clone(), pos, edgeID.clone(), edgeID.clone(), endPos - startPos, name.clone(), permissions, parkingLength, color, startPos));
            self.myCurrentStop = Some(stopRc.clone());
            let stop: &mut NBPTStop = stop_mut(&stopRc);
            while !self.myEdgeCont.get_split(edge).is_null() {
                stop.reset_loaded();
                let split: (Ptr<NBEdge>, Ptr<NBEdge>) = *self.myEdgeCont.get_split(edge);
                if stop.replace_edge(&edgeID, &vec![split.0, split.1]) {
                    edge = if split.0.get_id() == stop.get_edge_id() { split.0 } else { split.1 };
                    edgeID = edge.get_id().to_string();
                }
            }
            for line in StringTokenizer::new(&lines).get_vector() {
                stop.add_line(&line);
            }
            if !self.myStopCont.m().insert(stopRc.clone(), false) {
                write_error(&tlf!("Could not add public transport stop '%' (already exists)", id));
            }
        }
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
        if self.myCurrentLine.is_null() {
            write_error(&tlf!("Found route outside line definition"));
            return;
        }
        let mut ok = true;
        let edgeIDs: Vec<String> = attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, "", &mut ok, true);
        let mut edges: EdgeVector = Vec::new();
        for edgeID in &edgeIDs {
            let edge: Ptr<NBEdge> = self.myEdgeCont.retrieve(edgeID, false);
            if edge.is_null() {
                let lineID: String = self.myCurrentLine.get_line_id().to_string();
                self.report_error("Edge '%' in route of line '%' not found", edgeID, &lineID);
            } else {
                edges.push(edge);
            }
        }
        self.myCurrentLine.m().set_edges(&edges);
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
        let mut ok = true;
        self.myCurrentRouteID = attrs.get::<String>(SUMO_ATTR_ID, "route", &mut ok, true);
        let edgeIDs: Vec<String> = attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, &self.myCurrentRouteID, &mut ok, true);
        let mut edges: EdgeVector = Vec::new();
        for edgeID in &edgeIDs {
            let edge: Ptr<NBEdge> = self.myEdgeCont.retrieve(edgeID, false);
            if edge.is_null() {
                // C++ dereferences myCurrentLine here although it is nullptr for stand-alone routes (undefined behaviour);
                // an empty line id is reported instead of crashing
                let lineID: String = if self.myCurrentLine.is_null() { String::new() } else { self.myCurrentLine.get_line_id().to_string() };
                self.report_error("Edge '%' in route of line '%' not found", edgeID, &lineID);
            } else {
                edges.push(edge);
            }
        }
        self.myRouteEdges.insert(self.myCurrentRouteID.clone(), edges);
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
        let mut ok = true;
        let id: String = if attrs.has_attribute(SUMO_ATTR_ID) {
            attrs.get::<String>(SUMO_ATTR_ID, "ptLine", &mut ok, true)
        } else {
            attrs.get::<String>(SUMO_ATTR_BUS_STOP, "ptline", &mut ok, true)
        };
        // C++ NBPTStopCont::get returns nullptr for unknown ids; an Rc cannot be null, so the lookup is the same map lookup on get_stops()
        let Some(stop) = self.myStopCont.get_stops().get(&id).cloned() else {
            if !NBPTStopCont::was_ignored(&id) {
                write_error(&tlf!("Stop '%' within line '%' not found", id, self.myCurrentLine.get_line_id()));
            }
            return;
        };
        self.myCurrentLine.m().add_pt_stop(stop);
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
        assert!(self.myCurrentRouteID != "");
        let mut ok = true;
        let id: String = if attrs.has_attribute(SUMO_ATTR_ID) {
            attrs.get::<String>(SUMO_ATTR_ID, "ptLine", &mut ok, true)
        } else {
            attrs.get::<String>(SUMO_ATTR_BUS_STOP, "ptline", &mut ok, true)
        };
        // C++ NBPTStopCont::get returns nullptr for unknown ids; an Rc cannot be null, so the lookup is the same map lookup on get_stops()
        let Some(stop) = self.myStopCont.get_stops().get(&id).cloned() else {
            write_error(&tlf!("Stop '%' within route '%' not found", id, self.myCurrentRouteID));
            return;
        };
        self.myRouteStops.entry(self.myCurrentRouteID.clone()).or_default().push(stop);
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
        let Some(stop) = self.myCurrentStop.clone() else {
            if self.myCurrentStopWasIgnored {
                return;
            }
            panic!("{}", ProcessError::new("Could not add access outside a stopping place."));
        };
        let mut ok = true;
        let laneID: String = attrs.get::<String>(SUMO_ATTR_LANE, "access", &mut ok, true);
        let edgeID: String = get_edge_id_from_lane(&laneID);
        if self.myEdgeCont.retrieve(&edgeID, false).is_null() {
            self.report_error("Edge '%' for access to stop '%' not found", &edgeID, &stop.get_id());
            return;
        }
        let pos: f64 = attrs.get::<f64>(SUMO_ATTR_POSITION, "access", &mut ok, true);
        let length: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, "access", &mut ok, -1., true);
        if ok {
            stop_mut(&stop).add_access(laneID, pos, length);
        }
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
        let mut ok = true;
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "ptLine", &mut ok, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let line: String = attrs.get_opt::<String>(SUMO_ATTR_LINE, &id, &mut ok, String::new(), true);
        let type_: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true);
        let mut vClass: SUMOVehicleClass = NIImporter_OpenStreetMap::interpret_transport_type(&type_, Ptr::null());
        if attrs.has_attribute(SUMO_ATTR_VCLASS) {
            vClass = get_vehicle_class_id(&attrs.get::<String>(SUMO_ATTR_VCLASS, &id, &mut ok, true)).unwrap_or_else(|e| panic!("{e}"));
        }
        let color: RGBColor = get_opt_color(attrs, SUMO_ATTR_COLOR, &id, &mut ok);
        let intervalS: i32 = attrs.get_opt::<i32>(SUMO_ATTR_PERIOD, &id, &mut ok, -1, true);
        let nightService: String = attrs.get_string_secure_name("nightService", "");
        self.myCurrentCompletion = to_double(&attrs.get_string_secure_name("completeness", "1")).unwrap_or_else(|e| panic!("{e}"));
        self.myMissingBefore = to_int(&attrs.get_string_secure_name("missingBefore", "0")).unwrap_or_else(|e| panic!("{e}"));
        self.myMissingAfter = to_int(&attrs.get_string_secure_name("missingAfter", "0")).unwrap_or_else(|e| panic!("{e}"));
        if ok {
            // patching existing line?
            self.myCurrentLine = self.myLineCont.m().retrieve(&id);
            if self.myCurrentLine.is_null() {
                let newLine: NBPTLine = NBPTLine::new(&id, &name, &type_, &line, intervalS / 60, &nightService, vClass, color);
                self.myCurrentLine = Ptr::from_raw(Box::into_raw(Box::new(newLine)));
                self.myLineCont.m().insert(self.myCurrentLine);
            } else {
                write_message(&tlf!("Duplicate ptLine id occurred ('%'); assuming overwriting is wished.", id));
                if name != "" {
                    self.myCurrentLine.m().set_name(&name);
                }
                if line != "" {
                    self.myCurrentLine.m().set_ref(&line);
                }
                if intervalS != -1 {
                    self.myCurrentLine.m().set_period(intervalS);
                }
            }
        }
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
        let mut ok = true;
        self.myMissingBefore = 0;
        self.myMissingAfter = 0;
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "flow", &mut ok, true);
        let line: String = attrs.get::<String>(SUMO_ATTR_LINE, &id, &mut ok, true);
        let type_: String = attrs.get::<String>(SUMO_ATTR_TYPE, &id, &mut ok, true);
        let route: String = attrs.get::<String>(SUMO_ATTR_ROUTE, &id, &mut ok, true);
        let vClass: SUMOVehicleClass = NIImporter_OpenStreetMap::interpret_transport_type(&type_, Ptr::null());
        let intervalS: i32 = attrs.get_opt::<i32>(SUMO_ATTR_PERIOD, &id, &mut ok, -1, true);
        let color: RGBColor = get_opt_color(attrs, SUMO_ATTR_COLOR, &id, &mut ok);
        if ok {
            let newLine: NBPTLine = NBPTLine::new(&id, "", &type_, &line, intervalS / 60, "", vClass, color);
            self.myCurrentLine = Ptr::from_raw(Box::into_raw(Box::new(newLine)));
            // operator[] default-inserts an empty entry for a missing route, as in C++
            let edges: EdgeVector = self.myRouteEdges.entry(route.clone()).or_default().clone();
            self.myCurrentLine.m().set_edges(&edges);
            let stops: Vec<Rc<NBPTStop>> = self.myRouteStops.entry(route.clone()).or_default().clone();
            for stop in stops {
                self.myCurrentLine.m().add_pt_stop(stop);
            }
            self.myLineCont.m().insert(self.myCurrentLine);
        }
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
        let ignored: bool = self.myEdgeCont.was_ignored(edgeID.to_string());
        if !ignored && !self.myWarnOnly {
            write_error(&tlf!(msg, edgeID, id));
            return true;
        } else if !ignored {
            write_warning(&tlf!(msg, edgeID, id));
        }
        false
    }

    /// C++: `void NIXMLPTHandler::NIXMLPTHandler(const NIXMLPTHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLPTHandler) -> Self {
        panic!("NIXMLPTHandler is not copyable: its copy constructor is declared private and never defined in C++")
    }

    /// C++: `NIXMLPTHandler & NIXMLPTHandler::operator=(const NIXMLPTHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLPTHandler) -> &mut NIXMLPTHandler {
        panic!("NIXMLPTHandler is not assignable: operator= is declared private and never defined in C++")
    }

}

impl SaxHandler for NIXMLPTHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIXMLPTHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIXMLPTHandler::my_end_element(self, element);
        Ok(())
    }
}
