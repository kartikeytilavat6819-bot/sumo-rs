//! Port of netimport/NIXMLTypesHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::NETWORK_VERSION;
use utils::common::msg_handler::write_error;

/// C++ `throw`: raised as a panic carrying the ProcessError until a top-level catch exists
fn throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| std::panic::panic_any(e))
}

/// Importer for edge type information stored in XML
/// C++ class `NIXMLTypesHandler` (netimport/NIXMLTypesHandler.h)
#[repr(C)]
pub struct NIXMLTypesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The type container to fill
    pub myTypeCont: Ptr<NBTypeCont>,
    /// The currently parsed type
    pub myCurrentTypeID: String,
}

impl std::ops::Deref for NIXMLTypesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLTypesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIXMLTypesHandler {
    /// C++: `void NIXMLTypesHandler::NIXMLTypesHandler(NBTypeCont &)`
    /// Constructor
    // NIXMLTypesHandler::NIXMLTypesHandler(NBTypeCont& tc)
    //     : SUMOSAXHandler("xml-types - file"),
    //       myTypeCont(tc) {}
    #[allow(unused_variables)]
    pub fn new(tc: &mut NBTypeCont) -> Self {
        Self {
            base: utils::xml::sumo_sax_handler::new("xml-types - file", ""),
            myTypeCont: Ptr::from_mut(tc),
            myCurrentTypeID: String::new(),
        }
    }

    /// C++: `void NIXMLTypesHandler::~NIXMLTypesHandler()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has no body
    }

    /// C++: `void NIXMLTypesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual]
    /// Called on the opening of a tag; Parses edge type information
    // void
    // NIXMLTypesHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_TYPE: {
    //             bool ok = true;
    //             // get the id, report a warning if not given or empty...
    //             myCurrentTypeID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             const char* const id = myCurrentTypeID.c_str();
    //             const std::string defType = myTypeCont.knows(myCurrentTypeID) ? myCurrentTypeID : "";
    //             const int priority = attrs.getOpt<int>(SUMO_ATTR_PRIORITY, id, ok, myTypeCont.getEdgeTypePriority(defType));
    //             const int numLanes = attrs.getOpt<int>(SUMO_ATTR_NUMLANES, id, ok, myTypeCont.getEdgeTypeNumLanes(defType));
    //             const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, id, ok, myTypeCont.getEdgeTypeSpeed(defType));
    //             const std::string allowS = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, id, ok, "");
    //             const std::string disallowS = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, id, ok, "");
    //             const std::string spreadTypeS = attrs.getOpt<std::string>(SUMO_ATTR_SPREADTYPE, id, ok, toString(LaneSpreadFunction::SPREAD_UNKNOWN));
    //             const bool oneway = attrs.getOpt<bool>(SUMO_ATTR_ONEWAY, id, ok, myTypeCont.getEdgeTypeIsOneWay(defType));
    //             const bool discard = attrs.getOpt<bool>(SUMO_ATTR_DISCARD, id, ok, false);
    //             const double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id, ok, myTypeCont.getEdgeTypeWidth(defType));
    //             const double maxWidth = attrs.getOpt<double>(SUMO_ATTR_MAXWIDTH, id, ok, myTypeCont.getEdgeTypeMaxWidth(defType));
    //             const double minWidth = attrs.getOpt<double>(SUMO_ATTR_MINWIDTH, id, ok, myTypeCont.getEdgeTypeMinWidth(defType));
    //             const double widthResolution = attrs.getOpt<double>(SUMO_ATTR_WIDTHRESOLUTION, id, ok, myTypeCont.getEdgeTypeWidthResolution(defType));
    //             const double sidewalkWidth = attrs.getOpt<double>(SUMO_ATTR_SIDEWALKWIDTH, id, ok, myTypeCont.getEdgeTypeSidewalkWidth(defType));
    //             const double bikeLaneWidth = attrs.getOpt<double>(SUMO_ATTR_BIKELANEWIDTH, id, ok, myTypeCont.getEdgeTypeBikeLaneWidth(defType));
    //             // continue if parsing parameter was ok
    //             if (ok) {
    //                 // build the type
    //                 SVCPermissions permissions = myTypeCont.getEdgeTypePermissions(defType);
    //                 if (allowS != "" || disallowS != "") {
    //                     permissions = parseVehicleClasses(allowS, disallowS);
    //                 }
    //                 // get spreadType
    //                 LaneSpreadFunction spreadType = LaneSpreadFunction::RIGHT;
    //                 // check if spreadType is valid
    //                 if (SUMOXMLDefinitions::LaneSpreadFunctions.hasString(spreadTypeS)) {
    //                     spreadType = SUMOXMLDefinitions::LaneSpreadFunctions.get(spreadTypeS);
    //                 } else {
    //                     WRITE_ERRORF(TL("Invalid lane spread type '%'. Using default 'right'"), spreadTypeS);
    //                 }
    //                 // insert edgeType in container
    //                 myTypeCont.insertEdgeType(myCurrentTypeID, numLanes, speed, priority, permissions, spreadType, width,
    //                                           oneway, sidewalkWidth, bikeLaneWidth, widthResolution, maxWidth, minWidth);
    //                 // check if mark edgeType as discard
    //                 if (discard) {
    //                     myTypeCont.markEdgeTypeAsToDiscard(myCurrentTypeID);
    //                 }
    //                 // mark attributes as set
    //                 SumoXMLAttr myAttrs[] = {SUMO_ATTR_PRIORITY, SUMO_ATTR_NUMLANES, SUMO_ATTR_SPEED, SUMO_ATTR_ALLOW,
    //                                          SUMO_ATTR_DISALLOW, SUMO_ATTR_SPREADTYPE, SUMO_ATTR_ONEWAY, SUMO_ATTR_DISCARD,
    //                                          SUMO_ATTR_WIDTH, SUMO_ATTR_SIDEWALKWIDTH, SUMO_ATTR_BIKELANEWIDTH
    //                                         };
    //                 for (const auto& attr : myAttrs) {
    //                     if (attrs.hasAttribute(attr)) {
    //                         myTypeCont.markEdgeTypeAsSet(myCurrentTypeID, attr);
    //                     }
    //                 }
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_LANETYPE: {
    //             bool ok = true;
    //             // use id of last inserted edge
    //             const char* const edgeTypeId = myCurrentTypeID.c_str();
    //             const int index = attrs.get<int>(SUMO_ATTR_INDEX, edgeTypeId, ok);
    //             const std::string defType = myTypeCont.knows(myCurrentTypeID) ? myCurrentTypeID : "";
    //             if (index >= myTypeCont.getEdgeTypeNumLanes(defType)) {
    //                 WRITE_ERRORF(TL("Invalid lane index % for edge type '%' with % lanes"), toString(index), defType, toString(myTypeCont.getEdgeTypeNumLanes(defType)));
    //                 ok = false;
    //             }
    //             const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, edgeTypeId, ok, myTypeCont.getEdgeTypeSpeed(edgeTypeId));
    //             const std::string allowS = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, edgeTypeId, ok, "");
    //             const std::string disallowS = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, edgeTypeId, ok, "");
    //             const double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, edgeTypeId, ok, myTypeCont.getEdgeTypeWidth(defType));
    //             // continue if parsing parameter was ok
    //             if (ok) {
    //                 // build the type
    //                 SVCPermissions permissions = myTypeCont.getEdgeTypePermissions(defType);
    //                 if (allowS != "" || disallowS != "") {
    //                     permissions = parseVehicleClasses(allowS, disallowS);
    //                 }
    //
    //                 // insert laneType in container
    //                 myTypeCont.insertLaneType(myCurrentTypeID, index, speed, permissions, width, {});
    //                 // mark attributes as set
    //                 SumoXMLAttr myAttrs[] = {SUMO_ATTR_SPEED, SUMO_ATTR_ALLOW, SUMO_ATTR_DISALLOW, SUMO_ATTR_WIDTH};
    //                 for (const auto& attr : myAttrs) {
    //                     if (attrs.hasAttribute(attr)) {
    //                         myTypeCont.markLaneTypeAsSet(myCurrentTypeID, index, attr);
    //                     }
    //                 }
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_RESTRICTION: {
    //             bool ok = true;
    //             const SUMOVehicleClass svc = getVehicleClassID(attrs.get<std::string>(SUMO_ATTR_VCLASS, myCurrentTypeID.c_str(), ok));
    //             const double speed = attrs.get<double>(SUMO_ATTR_SPEED, myCurrentTypeID.c_str(), ok);
    //             if (ok) {
    //                 myTypeCont.addEdgeTypeRestriction(myCurrentTypeID, svc, speed);
    //             }
    //             break;
    //         }
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        match element {
            SUMO_TAG_TYPE => {
                let mut ok = true;
                // get the id, report a warning if not given or empty...
                self.myCurrentTypeID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
                let id = self.myCurrentTypeID.clone();
                let defType = if self.myTypeCont.m().knows(&self.myCurrentTypeID) { self.myCurrentTypeID.clone() } else { String::new() };
                let priority = attrs.get_opt::<i32>(SUMO_ATTR_PRIORITY, &id, &mut ok, self.myTypeCont.m().get_edge_type_priority(&defType), true);
                let numLanes = attrs.get_opt::<i32>(SUMO_ATTR_NUMLANES, &id, &mut ok, self.myTypeCont.m().get_edge_type_num_lanes(&defType), true);
                let speed = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, &id, &mut ok, self.myTypeCont.m().get_edge_type_speed(&defType), true);
                let allowS = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &id, &mut ok, String::new(), true);
                let disallowS = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &id, &mut ok, String::new(), true);
                let spreadTypeS = attrs.get_opt::<String>(SUMO_ATTR_SPREADTYPE, &id, &mut ok, LaneSpreadFunctions.get_string(LaneSpreadFunction::SPREAD_UNKNOWN).to_string(), true);
                let oneway = attrs.get_opt::<bool>(SUMO_ATTR_ONEWAY, &id, &mut ok, self.myTypeCont.m().get_edge_type_is_one_way(&defType), true);
                let discard = attrs.get_opt::<bool>(SUMO_ATTR_DISCARD, &id, &mut ok, false, true);
                let width = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, &id, &mut ok, self.myTypeCont.m().get_edge_type_width(&defType), true);
                let maxWidth = attrs.get_opt::<f64>(SUMO_ATTR_MAXWIDTH, &id, &mut ok, self.myTypeCont.m().get_edge_type_max_width(&defType), true);
                let minWidth = attrs.get_opt::<f64>(SUMO_ATTR_MINWIDTH, &id, &mut ok, self.myTypeCont.m().get_edge_type_min_width(&defType), true);
                let widthResolution = attrs.get_opt::<f64>(SUMO_ATTR_WIDTHRESOLUTION, &id, &mut ok, self.myTypeCont.m().get_edge_type_width_resolution(&defType), true);
                let sidewalkWidth = attrs.get_opt::<f64>(SUMO_ATTR_SIDEWALKWIDTH, &id, &mut ok, self.myTypeCont.m().get_edge_type_sidewalk_width(&defType), true);
                let bikeLaneWidth = attrs.get_opt::<f64>(SUMO_ATTR_BIKELANEWIDTH, &id, &mut ok, self.myTypeCont.m().get_edge_type_bike_lane_width(&defType), true);
                // continue if parsing parameter was ok
                if ok {
                    // build the type
                    let mut permissions = self.myTypeCont.m().get_edge_type_permissions(&defType);
                    if !allowS.is_empty() || !disallowS.is_empty() {
                        permissions = parse_vehicle_classes_allow_disallow(&allowS, &disallowS, NETWORK_VERSION);
                    }
                    // get spreadType
                    let mut spreadType = LaneSpreadFunction::RIGHT;
                    // check if spreadType is valid
                    if LaneSpreadFunctions.has_string(&spreadTypeS) {
                        spreadType = throw(LaneSpreadFunctions.get(&spreadTypeS));
                    } else {
                        write_error(&format!("Invalid lane spread type '{spreadTypeS}'. Using default 'right'"));
                    }
                    // insert edgeType in container
                    self.myTypeCont.m().insert_edge_type(&self.myCurrentTypeID, numLanes, speed, priority, permissions, spreadType, width,
                        oneway, sidewalkWidth, bikeLaneWidth, widthResolution, maxWidth, minWidth);
                    // check if mark edgeType as discard
                    if discard {
                        self.myTypeCont.m().mark_edge_type_as_to_discard(&self.myCurrentTypeID);
                    }
                    // mark attributes as set
                    let myAttrs = [SUMO_ATTR_PRIORITY, SUMO_ATTR_NUMLANES, SUMO_ATTR_SPEED, SUMO_ATTR_ALLOW,
                        SUMO_ATTR_DISALLOW, SUMO_ATTR_SPREADTYPE, SUMO_ATTR_ONEWAY, SUMO_ATTR_DISCARD,
                        SUMO_ATTR_WIDTH, SUMO_ATTR_SIDEWALKWIDTH, SUMO_ATTR_BIKELANEWIDTH];
                    for attr in myAttrs {
                        if attrs.has_attribute(attr) {
                            self.myTypeCont.m().mark_edge_type_as_set(&self.myCurrentTypeID, attr);
                        }
                    }
                }
            }
            SUMO_TAG_LANETYPE => {
                let mut ok = true;
                // use id of last inserted edge
                let edgeTypeId = self.myCurrentTypeID.clone();
                let index = attrs.get::<i32>(SUMO_ATTR_INDEX, &edgeTypeId, &mut ok, true);
                let defType = if self.myTypeCont.m().knows(&self.myCurrentTypeID) { self.myCurrentTypeID.clone() } else { String::new() };
                if index >= self.myTypeCont.m().get_edge_type_num_lanes(&defType) {
                    write_error(&format!("Invalid lane index {index} for edge type '{defType}' with {} lanes", self.myTypeCont.m().get_edge_type_num_lanes(&defType)));
                    ok = false;
                }
                let speed = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, &edgeTypeId, &mut ok, self.myTypeCont.m().get_edge_type_speed(&edgeTypeId), true);
                let allowS = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &edgeTypeId, &mut ok, String::new(), true);
                let disallowS = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &edgeTypeId, &mut ok, String::new(), true);
                let width = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, &edgeTypeId, &mut ok, self.myTypeCont.m().get_edge_type_width(&defType), true);
                // continue if parsing parameter was ok
                if ok {
                    // build the type
                    let mut permissions = self.myTypeCont.m().get_edge_type_permissions(&defType);
                    if !allowS.is_empty() || !disallowS.is_empty() {
                        permissions = parse_vehicle_classes_allow_disallow(&allowS, &disallowS, NETWORK_VERSION);
                    }
                    // insert laneType in container
                    self.myTypeCont.m().insert_lane_type(&self.myCurrentTypeID, index, speed, permissions, width, &BTreeSet::new());
                    // mark attributes as set
                    let myAttrs = [SUMO_ATTR_SPEED, SUMO_ATTR_ALLOW, SUMO_ATTR_DISALLOW, SUMO_ATTR_WIDTH];
                    for attr in myAttrs {
                        if attrs.has_attribute(attr) {
                            self.myTypeCont.m().mark_lane_type_as_set(&self.myCurrentTypeID, index, attr);
                        }
                    }
                }
            }
            SUMO_TAG_RESTRICTION => {
                let mut ok = true;
                let currentID = self.myCurrentTypeID.clone();
                let svc = throw(get_vehicle_class_id(&attrs.get::<String>(SUMO_ATTR_VCLASS, &currentID, &mut ok, true)));
                let speed = attrs.get::<f64>(SUMO_ATTR_SPEED, &currentID, &mut ok, true);
                if ok {
                    self.myTypeCont.m().add_edge_type_restriction(&currentID, svc, speed);
                }
            }
            _ => {}
        }
    }

    /// C++: `void NIXMLTypesHandler::NIXMLTypesHandler(const NIXMLTypesHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLTypesHandler) -> Self {
        // the copy constructor is declared private and never defined in C++
        panic!("NIXMLTypesHandler is not copyable")
    }

    /// C++: `NIXMLTypesHandler & NIXMLTypesHandler::operator=(const NIXMLTypesHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLTypesHandler) -> &mut NIXMLTypesHandler {
        // the assignment operator is declared private and never defined in C++
        panic!("NIXMLTypesHandler is not assignable")
    }

}

impl SaxHandler for NIXMLTypesHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIXMLTypesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
}
