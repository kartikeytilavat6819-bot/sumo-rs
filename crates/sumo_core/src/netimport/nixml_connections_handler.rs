//! Port of netimport/NIXMLConnectionsHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{MsgType, with_instance, write_error, write_warning};
use utils::common::std_defs::NETWORK_VERSION;
use utils::common::string_utils::to_int_secure;

/// NBEdge::UNSPECIFIED_WIDTH (NBEdge.cpp:69); no Rust const exists in nb_edge.rs yet
const UNSPECIFIED_WIDTH: f64 = -1.0;

/// C++ `NBConnection::InvalidConnection`
fn invalid_connection() -> NBConnection {
    NBConnection::new_3("invalidFrom", Ptr::null(), "invalidTo", Ptr::null())
}

/// MsgHandler::getWarningInstance() / getErrorInstance(). The singletons live in a Mutex-guarded
/// static that is only reset in cleanup_on_end, so the pointer stays valid while parsing.
fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    with_instance(if warning { MsgType::Warning } else { MsgType::Error }, |h| Ptr::from_mut(h))
}

/// The name toString(SUMO_ATTR_X) gives for an attribute id
fn attr_name(id: i32) -> &'static str {
    ATTRS.iter().find(|(_, v)| *v == id).map(|(n, _)| *n).unwrap_or("")
}

/// delete on a GeoConvHelper* allocated with Box::into_raw; null is a no-op
fn delete_geo_conv_helper(p: Ptr<GeoConvHelper>) {
    if !p.is_null() {
        drop(unsafe { Box::from_raw(p.as_raw()) });
    }
}

/// toString(EdgeVector): the edge ids separated by spaces (ToString.h, vector<V*> overload)
fn edges_to_string(edges: &EdgeVector) -> String {
    edges.iter().map(|e| e.get_id().to_string()).collect::<Vec<_>>().join(" ")
}

/// NBNode::getCrossing(const EdgeVector&, bool hardFail = true) (NBNode.cpp:4129). The Rust NBNode
/// only has the id overload, so the edge-set lookup is done here over myCrossings.
fn get_crossing_by_edges(node: Ptr<NBNode>, edges: &EdgeVector) -> Ptr<NBNode_Crossing> {
    // determinism: EdgeSet is a std::set<NBEdge*>; only set equality is used, so BTreeSet order does not matter
    let edge_set: BTreeSet<Ptr<NBEdge>> = edges.iter().copied().collect();
    for crossing in node.myCrossings.iter() {
        let edge_set2: BTreeSet<Ptr<NBEdge>> = crossing.edges.iter().copied().collect();
        if edge_set == edge_set2 {
            return Ptr::new(&**crossing);
        }
    }
    panic!("{}", ProcessError::new("Request for unknown crossing for the given Edges"))
}

/// Importer for edge connections stored in XML
/// C++ class `NIXMLConnectionsHandler` (netimport/NIXMLConnectionsHandler.h)
pub struct NIXMLConnectionsHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The edge container to fill
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The edge container to fill
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The traffic lights container to add built tls to (when invalidating tls)
    pub myTLLogicCont: Ptr<NBTrafficLightLogicCont>,
    /// Information whether we have a deprecated attribute
    pub myHaveWarnedAboutDeprecatedLanes: bool,
    /// the handler for loading errors
    pub myErrorMsgHandler: Ptr<MsgHandler>,
    /// The coordinate transformation which was used compute the custom shape coordinates for connections and crossings
    pub myLocation: Ptr<GeoConvHelper>,
    /// last item the could receive parameters
    pub myLastParameterised: Ptr<Parameterised>,
}

impl std::ops::Deref for NIXMLConnectionsHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLConnectionsHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIXMLConnectionsHandler {
    /// C++: `void NIXMLConnectionsHandler::NIXMLConnectionsHandler(NBEdgeCont &, NBNodeCont &, NBTrafficLightLogicCont &)`
    /// Constructor
    // NIXMLConnectionsHandler::NIXMLConnectionsHandler(NBEdgeCont& ec, NBNodeCont& nc, NBTrafficLightLogicCont& tlc) :
    //     SUMOSAXHandler("xml-connection-description"),
    //     myEdgeCont(ec),
    //     myNodeCont(nc),
    //     myTLLogicCont(tlc),
    //     myHaveWarnedAboutDeprecatedLanes(false),
    //     myErrorMsgHandler(OptionsCont::getOptions().getBool("ignore-errors.connections") ?
    //                       MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //     myLocation(nullptr),
    //     myLastParameterised(nullptr) {
    // }
    #[allow(unused_variables)]
    pub fn new(ec: &mut NBEdgeCont, nc: &mut NBNodeCont, tlc: &mut NBTrafficLightLogicCont) -> Self {
        NIXMLConnectionsHandler {
            base: GenericSAXHandler::new(&TAGS, SUMO_TAG_NOTHING, &ATTRS, SUMO_ATTR_NOTHING, "xml-connection-description", ""),
            myEdgeCont: Ptr::from_mut(ec),
            myNodeCont: Ptr::from_mut(nc),
            myTLLogicCont: Ptr::from_mut(tlc),
            myHaveWarnedAboutDeprecatedLanes: false,
            myErrorMsgHandler: msg_instance(utils::options::options_cont::get_bool("ignore-errors.connections")),
            myLocation: Ptr::null(),
            myLastParameterised: Ptr::null(),
        }
    }

    /// C++: `void NIXMLConnectionsHandler::~NIXMLConnectionsHandler() noexcept` [virtual]
    /// Destructor
    // NIXMLConnectionsHandler::~NIXMLConnectionsHandler() {
    //     delete myLocation;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        delete_geo_conv_helper(self.myLocation);
    }

    /// C++: `void NIXMLConnectionsHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIXMLConnectionsHandler::myStartElement(int element,
    //                                         const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_CONNECTIONS:
    //             // infer location for legacy networks that don't have location information
    //             myLocation = GeoConvHelper::getLoadedPlain(getFileName(), ".con.xml");
    //             break;
    //         case SUMO_TAG_LOCATION:
    //             delete myLocation;
    //             myLocation = NIImporter_SUMO::loadLocation(attrs, false);
    //             break;
    //         case SUMO_TAG_DEL:
    //             delConnection(attrs);
    //             break;
    //         case SUMO_TAG_CONNECTION:
    //             parseConnection(attrs);
    //             break;
    //         case SUMO_TAG_PROHIBITION:
    //             addProhibition(attrs);
    //             break;
    //         case SUMO_TAG_CROSSING:
    //             addCrossing(attrs);
    //             break;
    //         case SUMO_TAG_WALKINGAREA:
    //             addWalkingArea(attrs);
    //             break;
    //         case SUMO_TAG_PARAM:
    //             if (myLastParameterised != nullptr) {
    //                 bool ok = true;
    //                 const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //                 // circumventing empty string test
    //                 const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //                 myLastParameterised->setParameter(key, val);
    //             }
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        match element {
            SUMO_TAG_CONNECTIONS => {
                // infer location for legacy networks that don't have location information
                let loaded = GeoConvHelper::get_loaded_plain(self.get_file_name(), ".con.xml");
                self.myLocation = match loaded {
                    Some(g) => Ptr::from_raw(Box::into_raw(Box::new(g))),
                    None => Ptr::null(),
                };
            }
            SUMO_TAG_LOCATION => {
                delete_geo_conv_helper(self.myLocation);
                self.myLocation = NIImporter_SUMO::load_location(attrs, false);
            }
            SUMO_TAG_DEL => self.del_connection(attrs),
            SUMO_TAG_CONNECTION => self.parse_connection(attrs),
            SUMO_TAG_PROHIBITION => self.add_prohibition(attrs),
            SUMO_TAG_CROSSING => self.add_crossing(attrs),
            SUMO_TAG_WALKINGAREA => self.add_walking_area(attrs),
            SUMO_TAG_PARAM => {
                if !self.myLastParameterised.is_null() {
                    let mut ok = true;
                    let key = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
                    // circumventing empty string test
                    let val = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
                    self.myLastParameterised.m().set_parameter(&key, &val);
                }
            }
            _ => {}
        }
    }

    /// C++: `void NIXMLConnectionsHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIXMLConnectionsHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_CONNECTION:
    //         case SUMO_TAG_CROSSING:
    //             myLastParameterised = nullptr;
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            SUMO_TAG_CONNECTION | SUMO_TAG_CROSSING => {
                self.myLastParameterised = Ptr::null();
            }
            _ => {}
        }
    }

    /// C++: `NBConnection NIXMLConnectionsHandler::parseConnectionDef(const std::string &, const std::string &)` [private]
    /// Returns the connection described by def
    // NBConnection
    // NIXMLConnectionsHandler::parseConnectionDef(const std::string& defRole, const std::string& def) {
    //     // split from/to
    //     const std::string::size_type div = def.find("->");
    //     if (div == std::string::npos) {
    //         myErrorMsgHandler->inform("Missing connection divider in " + defRole + " '" + def + "'");
    //         return NBConnection::InvalidConnection;
    //     }
    //     std::string fromDef = def.substr(0, div);
    //     std::string toDef = def.substr(div + 2);
    //
    //     // retrieve them now
    //     NBEdge* fromE = myEdgeCont.retrieve(fromDef);
    //     NBEdge* toE = myEdgeCont.retrieve(toDef);
    //     // check
    //     if (fromE == nullptr) {
    //         myErrorMsgHandler->inform("Could not find edge '" + fromDef + "' in " + defRole + " '" + def + "'");
    //         return NBConnection::InvalidConnection;
    //     }
    //     if (toE == nullptr) {
    //         myErrorMsgHandler->inform("Could not find edge '" + toDef + "' in " + defRole + " '" + def + "'");
    //         return NBConnection::InvalidConnection;
    //     }
    //     return NBConnection(fromE, toE);
    // }
    #[allow(unused_variables)]
    pub fn parse_connection_def(&mut self, defRole: &str, def: &str) -> NBConnection {
        // split from/to
        let Some(div) = def.find("->") else {
            self.myErrorMsgHandler.m().inform(&format!("Missing connection divider in {defRole} '{def}'"), true);
            return invalid_connection();
        };
        let fromDef = &def[..div];
        let toDef = &def[div + 2..];

        // retrieve them now
        let fromE = self.myEdgeCont.retrieve(fromDef, false);
        let toE = self.myEdgeCont.retrieve(toDef, false);
        // check
        if fromE.is_null() {
            self.myErrorMsgHandler.m().inform(&format!("Could not find edge '{fromDef}' in {defRole} '{def}'"), true);
            return invalid_connection();
        }
        if toE.is_null() {
            self.myErrorMsgHandler.m().inform(&format!("Could not find edge '{toDef}' in {defRole} '{def}'"), true);
            return invalid_connection();
        }
        NBConnection::new(fromE, toE)
    }

    /// C++: `void NIXMLConnectionsHandler::parseLaneBound(const SUMOSAXAttributes &, NBEdge *, NBEdge *)` [private]
    /// Parses a connection when it describes a lane-2-lane relationship
    // void
    // NIXMLConnectionsHandler::parseLaneBound(const SUMOSAXAttributes& attrs, NBEdge* from, NBEdge* to) {
    //     if (to == nullptr) {
    //         // do nothing if it's a dead end
    //         return;
    //     }
    //     bool ok = true;
    //     // get the begin and the end lane
    //     int fromLane;
    //     int toLane;
    //     try {
    //         if (!parseLaneInfo(attrs, from, to, &fromLane, &toLane)) {
    //             return;
    //         }
    //         if (fromLane < 0) {
    //             myErrorMsgHandler->informf("Invalid value '%' for " + toString(SUMO_ATTR_FROM_LANE) +
    //                                        " in connection from '%' to '%'.", fromLane, from->getID(), to->getID());
    //             return;
    //         }
    //         if (toLane < 0) {
    //             myErrorMsgHandler->informf("Invalid value '%' for " + toString(SUMO_ATTR_TO_LANE) +
    //                                        " in connection from '%' to '%'.", toLane, from->getID(), to->getID());
    //             return;
    //         }
    //
    //         NBEdge::Connection defaultCon(fromLane, to, toLane);
    //         if (from->getStep() == NBEdge::EdgeBuildingStep::LANES2LANES_USER) {
    //             // maybe we are patching an existing connection
    //             std::vector<NBEdge::Connection> existing = from->getConnectionsFromLane(fromLane, to, toLane);
    //             if (existing.size() > 0) {
    //                 assert(existing.size() == 1);
    //                 defaultCon = existing.front();
    //                 // remove the original so we can insert the replacement
    //                 from->removeFromConnections(defaultCon);
    //             } else {
    //                 from->getToNode()->invalidateTLS(myTLLogicCont, true, false);
    //             }
    //         }
    //         if (OptionsCont::getOptions().isSet("default.connection.cont-pos")) {
    //             defaultCon.contPos = OptionsCont::getOptions().getFloat("default.connection.cont-pos");
    //         }
    //         const bool mayDefinitelyPass = attrs.getOpt<bool>(SUMO_ATTR_PASS, nullptr, ok, defaultCon.mayDefinitelyPass);
    //         KeepClear keepClear = defaultCon.keepClear;
    //         if (attrs.hasAttribute(SUMO_ATTR_KEEP_CLEAR)) {
    //             keepClear = attrs.get<bool>(SUMO_ATTR_KEEP_CLEAR, nullptr, ok) ? KEEPCLEAR_TRUE : KEEPCLEAR_FALSE;
    //         }
    //         const double contPos = attrs.getOpt<double>(SUMO_ATTR_CONTPOS, nullptr, ok, defaultCon.contPos);
    //         const double visibility = attrs.getOpt<double>(SUMO_ATTR_VISIBILITY_DISTANCE, nullptr, ok, defaultCon.visibility);
    //         const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, nullptr, ok, defaultCon.speed);
    //         const double friction = attrs.getOpt<double>(SUMO_ATTR_FRICTION, nullptr, ok, defaultCon.friction);
    //         const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, nullptr, ok, defaultCon.customLength);
    //         const bool uncontrolled = attrs.getOpt<bool>(SUMO_ATTR_UNCONTROLLED, nullptr, ok, defaultCon.uncontrolled);
    //         const bool indirectLeft = attrs.getOpt<bool>(SUMO_ATTR_INDIRECT, nullptr, ok, false);
    //         const std::string edgeType = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, nullptr, ok, "");
    //         PositionVector customShape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, nullptr, ok, defaultCon.customShape);
    //         std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, nullptr, ok, "");
    //         std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, nullptr, ok, "");
    //         SVCPermissions permissions;
    //         if (allow == "" && disallow == "") {
    //             permissions = SVC_UNSPECIFIED;
    //         } else {
    //             permissions = parseVehicleClasses(allow, disallow);
    //         }
    //         SVCPermissions changeLeft = SVC_UNSPECIFIED;
    //         SVCPermissions changeRight = SVC_UNSPECIFIED;
    //         if (attrs.hasAttribute(SUMO_ATTR_CHANGE_LEFT)) {
    //             changeLeft = parseVehicleClasses(attrs.get<std::string>(SUMO_ATTR_CHANGE_LEFT, nullptr, ok), "");
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_CHANGE_RIGHT)) {
    //             changeRight = parseVehicleClasses(attrs.get<std::string>(SUMO_ATTR_CHANGE_RIGHT, nullptr, ok), "");
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_SHAPE) && !NBNetBuilder::transformCoordinates(customShape, true, myLocation)) {
    //             WRITE_ERRORF(TL("Unable to project shape for connection from edge '%' to edge '%'."), from->getID(), to->getID());
    //         }
    //         if (!ok) {
    //             return;
    //         }
    //         if (!from->addLane2LaneConnection(fromLane, to, toLane, NBEdge::Lane2LaneInfoType::USER, true, mayDefinitelyPass,
    //                                           keepClear, contPos, visibility, speed, friction, length, customShape, uncontrolled, permissions, indirectLeft, edgeType, changeLeft, changeRight)) {
    //             if (OptionsCont::getOptions().getBool("show-errors.connections-first-try")) {
    //                 WRITE_WARNINGF(TL("Could not set loaded connection from lane '%' to lane '%'."), from->getLaneID(fromLane), to->getLaneID(toLane));
    //             }
    //             // set as to be re-applied after network processing
    //             myEdgeCont.addPostProcessConnection(from->getID(), fromLane, to->getID(), toLane, mayDefinitelyPass, keepClear, contPos, visibility,
    //                                                 speed, friction, length, customShape, uncontrolled, false, permissions, indirectLeft, edgeType, changeLeft, changeRight);
    //         }
    //     } catch (NumberFormatException&) {
    //         myErrorMsgHandler->inform("At least one of the defined lanes was not numeric");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_lane_bound(&mut self, attrs: &SUMOSAXAttributes, from: Ptr<NBEdge>, to: Ptr<NBEdge>) {
        if to.is_null() {
            // do nothing if it's a dead end
            return;
        }
        let mut ok = true;
        // get the begin and the end lane
        let mut fromLane: i32 = 0;
        let mut toLane: i32 = 0;
        // shortcut: the C++ try/catch(NumberFormatException) is not reproduced; get() reports bad numbers itself and clears ok, so the "not numeric" inform is not emitted
        if !self.parse_lane_info(attrs, from, to, Ptr::from_mut(&mut fromLane), Ptr::from_mut(&mut toLane)) {
            return;
        }
        if fromLane < 0 {
            self.myErrorMsgHandler.m().inform(
                &format!(
                    "Invalid value '{}' for {} in connection from '{}' to '{}'.",
                    fromLane,
                    attr_name(SUMO_ATTR_FROM_LANE),
                    from.get_id(),
                    to.get_id()
                ),
                true,
            );
            return;
        }
        if toLane < 0 {
            self.myErrorMsgHandler.m().inform(
                &format!(
                    "Invalid value '{}' for {} in connection from '{}' to '{}'.",
                    toLane,
                    attr_name(SUMO_ATTR_TO_LANE),
                    from.get_id(),
                    to.get_id()
                ),
                true,
            );
            return;
        }

        let mut defaultCon = NBEdge_Connection::new(fromLane, to, toLane, false);
        if from.get_step() == NBEdge_EdgeBuildingStep::LANES2LANES_USER {
            // maybe we are patching an existing connection
            // shortcut: NBEdge::getConnectionsFromLane is still todo!() and returns CHBuilder_Connection, not NBEdge_Connection; panics here until nb_edge.rs types myConnections as Vec<NBEdge_Connection>
            let existing: Vec<NBEdge_Connection> = panic!("shortcut: NBEdge::getConnectionsFromLane not ported");
            if existing.len() > 0 {
                assert!(existing.len() == 1);
                defaultCon = existing[0].clone();
                // remove the original so we can insert the replacement
                from.m().remove_from_connections_2(&defaultCon);
            } else {
                from.get_to_node().m().invalidate_tls(self.myTLLogicCont.m(), true, false);
            }
        }
        if utils::options::options_cont::is_set("default.connection.cont-pos") {
            defaultCon.contPos = utils::options::options_cont::get_float("default.connection.cont-pos");
        }
        let mayDefinitelyPass = attrs.get_opt::<bool>(SUMO_ATTR_PASS, "", &mut ok, defaultCon.mayDefinitelyPass, true);
        let mut keepClear = defaultCon.keepClear;
        if attrs.has_attribute(SUMO_ATTR_KEEP_CLEAR) {
            keepClear = if attrs.get::<bool>(SUMO_ATTR_KEEP_CLEAR, "", &mut ok, true) { KeepClear::KEEPCLEAR_TRUE } else { KeepClear::KEEPCLEAR_FALSE };
        }
        let contPos = attrs.get_opt::<f64>(SUMO_ATTR_CONTPOS, "", &mut ok, defaultCon.contPos, true);
        let visibility = attrs.get_opt::<f64>(SUMO_ATTR_VISIBILITY_DISTANCE, "", &mut ok, defaultCon.visibility, true);
        let speed = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, "", &mut ok, defaultCon.speed, true);
        let friction = attrs.get_opt::<f64>(SUMO_ATTR_FRICTION, "", &mut ok, defaultCon.friction, true);
        let length = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, "", &mut ok, defaultCon.customLength, true);
        let uncontrolled = attrs.get_opt::<bool>(SUMO_ATTR_UNCONTROLLED, "", &mut ok, defaultCon.uncontrolled, true);
        let indirectLeft = attrs.get_opt::<bool>(SUMO_ATTR_INDIRECT, "", &mut ok, false, true);
        let edgeType = attrs.get_opt::<String>(SUMO_ATTR_TYPE, "", &mut ok, String::new(), true);
        let mut customShape = attrs.get_opt::<PositionVector>(SUMO_ATTR_SHAPE, "", &mut ok, defaultCon.customShape.clone(), true);
        let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, "", &mut ok, String::new(), true);
        let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, "", &mut ok, String::new(), true);
        let permissions: SVCPermissions = if allow == "" && disallow == "" {
            SVC_UNSPECIFIED
        } else {
            parse_vehicle_classes_allow_disallow(&allow, &disallow, NETWORK_VERSION)
        };
        let mut changeLeft: SVCPermissions = SVC_UNSPECIFIED;
        let mut changeRight: SVCPermissions = SVC_UNSPECIFIED;
        if attrs.has_attribute(SUMO_ATTR_CHANGE_LEFT) {
            changeLeft = parse_vehicle_classes_allow_disallow(&attrs.get::<String>(SUMO_ATTR_CHANGE_LEFT, "", &mut ok, true), "", NETWORK_VERSION);
        }
        if attrs.has_attribute(SUMO_ATTR_CHANGE_RIGHT) {
            changeRight = parse_vehicle_classes_allow_disallow(&attrs.get::<String>(SUMO_ATTR_CHANGE_RIGHT, "", &mut ok, true), "", NETWORK_VERSION);
        }
        if attrs.has_attribute(SUMO_ATTR_SHAPE) && !NBNetBuilder::transform_coordinates(&mut customShape, true, self.myLocation) {
            write_error(&tlf!("Unable to project shape for connection from edge '%' to edge '%'.", from.get_id(), to.get_id()));
        }
        if !ok {
            return;
        }
        if !from.m().add_lane2_lane_connection(
            fromLane,
            to,
            toLane,
            NBEdge_Lane2LaneInfoType::USER,
            true,
            mayDefinitelyPass,
            keepClear,
            contPos,
            visibility,
            speed,
            friction,
            length,
            &customShape,
            uncontrolled,
            permissions,
            indirectLeft,
            &edgeType,
            changeLeft,
            changeRight,
            false,
        ) {
            if utils::options::options_cont::get_bool("show-errors.connections-first-try") {
                write_warning(&tlf!(
                    "Could not set loaded connection from lane '%' to lane '%'.",
                    from.get_lane_id(fromLane),
                    to.get_lane_id(toLane)
                ));
            }
            // set as to be re-applied after network processing
            self.myEdgeCont.m().add_post_process_connection(
                from.get_id(),
                fromLane,
                to.get_id(),
                toLane,
                mayDefinitelyPass,
                keepClear,
                contPos,
                visibility,
                speed,
                friction,
                length,
                &customShape,
                uncontrolled,
                false,
                permissions,
                indirectLeft,
                &edgeType,
                changeLeft,
                changeRight,
            );
        }
    }

    /// C++: `bool NIXMLConnectionsHandler::parseLaneInfo(const SUMOSAXAttributes &, NBEdge *, NBEdge *, int *, int *)` [private]
    /// Parses information about lane-2-lane connection when it describes a lane-2-lane relationship
    // bool
    // NIXMLConnectionsHandler::parseLaneInfo(const SUMOSAXAttributes& attributes, NBEdge* fromEdge, NBEdge* toEdge,
    //                                        int* fromLane, int* toLane) {
    //     if (attributes.hasAttribute(SUMO_ATTR_LANE)) {
    //         return parseDeprecatedLaneDefinition(attributes, fromEdge, toEdge, fromLane, toLane);
    //     } else {
    //         return parseLaneDefinition(attributes, fromLane, toLane);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_lane_info(&mut self, attributes: &SUMOSAXAttributes, fromEdge: Ptr<NBEdge>, toEdge: Ptr<NBEdge>, fromLane: Ptr<i32>, toLane: Ptr<i32>) -> bool {
        if attributes.has_attribute(SUMO_ATTR_LANE) {
            self.parse_deprecated_lane_definition(attributes, fromEdge, toEdge, fromLane, toLane)
        } else {
            self.parse_lane_definition(attributes, fromLane, toLane)
        }
    }

    /// C++: `bool NIXMLConnectionsHandler::parseDeprecatedLaneDefinition(const SUMOSAXAttributes &, NBEdge *, NBEdge *, int *, int *)` [private]
    /// Parses information about lane-2-lane connection in deprecated format.
    // inline bool
    // NIXMLConnectionsHandler::parseDeprecatedLaneDefinition(const SUMOSAXAttributes& attributes,
    //         NBEdge* from, NBEdge* to,
    //         int* fromLane, int* toLane) {
    //     bool ok = true;
    //     if (!myHaveWarnedAboutDeprecatedLanes) {
    //         myHaveWarnedAboutDeprecatedLanes = true;
    //         WRITE_WARNING("'" + toString(SUMO_ATTR_LANE) + "' is deprecated, please use '" +
    //                       toString(SUMO_ATTR_FROM_LANE) + "' and '" + toString(SUMO_ATTR_TO_LANE) +
    //                       "' instead.");
    //     }
    //
    //     std::string laneConn = attributes.get<std::string>(SUMO_ATTR_LANE, nullptr, ok);
    //     StringTokenizer st(laneConn, ':');
    //     if (!ok || st.size() != 2) {
    //         myErrorMsgHandler->inform("Invalid lane to lane connection from '" +
    //                                   from->getID() + "' to '" + to->getID() + "'.");
    //         return false; // There was an error.
    //     }
    //
    //     *fromLane = StringUtils::toIntSecure(st.next(), -1);
    //     *toLane = StringUtils::toIntSecure(st.next(), -1);
    //
    //     return true; // We succeeded.
    // }
    #[allow(unused_variables)]
    pub fn parse_deprecated_lane_definition(&mut self, attributes: &SUMOSAXAttributes, fromEdge: Ptr<NBEdge>, toEdge: Ptr<NBEdge>, fromLane: Ptr<i32>, toLane: Ptr<i32>) -> bool {
        let mut ok = true;
        if !self.myHaveWarnedAboutDeprecatedLanes {
            self.myHaveWarnedAboutDeprecatedLanes = true;
            write_warning(&format!(
                "'{}' is deprecated, please use '{}' and '{}' instead.",
                attr_name(SUMO_ATTR_LANE),
                attr_name(SUMO_ATTR_FROM_LANE),
                attr_name(SUMO_ATTR_TO_LANE)
            ));
        }

        let laneConn = attributes.get::<String>(SUMO_ATTR_LANE, "", &mut ok, true);
        let st = StringTokenizer::with_token(&laneConn, ":", false).get_vector();
        if !ok || st.len() != 2 {
            self.myErrorMsgHandler.m().inform(
                &format!("Invalid lane to lane connection from '{}' to '{}'.", fromEdge.get_id(), toEdge.get_id()),
                true,
            );
            return false; // There was an error.
        }

        // shortcut: NumberFormatException from toIntSecure is mapped to the catch-all failure of parseLaneBound (not reproduced); toIntSecure errors return false here
        let (Ok(fl), Ok(tl)) = (to_int_secure(&st[0], -1), to_int_secure(&st[1], -1)) else {
            self.myErrorMsgHandler.m().inform("At least one of the defined lanes was not numeric", true);
            return false;
        };
        fromLane.m().clone_from(&fl);
        toLane.m().clone_from(&tl);

        true // We succeeded.
    }

    /// C++: `bool NIXMLConnectionsHandler::parseLaneDefinition(const SUMOSAXAttributes &, int *, int *)` [private]
    /// Parses information about lane-2-lane connection.
    // inline bool
    // NIXMLConnectionsHandler::parseLaneDefinition(const SUMOSAXAttributes& attributes,
    //         int* fromLane,
    //         int* toLane) {
    //     bool ok = true;
    //     *fromLane = attributes.get<int>(SUMO_ATTR_FROM_LANE, nullptr, ok);
    //     *toLane = attributes.get<int>(SUMO_ATTR_TO_LANE, nullptr, ok);
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_lane_definition(&mut self, attributes: &SUMOSAXAttributes, fromLane: Ptr<i32>, toLane: Ptr<i32>) -> bool {
        let mut ok = true;
        fromLane.m().clone_from(&attributes.get::<i32>(SUMO_ATTR_FROM_LANE, "", &mut ok, true));
        toLane.m().clone_from(&attributes.get::<i32>(SUMO_ATTR_TO_LANE, "", &mut ok, true));
        ok
    }

    /// C++: `void NIXMLConnectionsHandler::delConnection(const SUMOSAXAttributes &)` [private]
    /// Parses a delete element that specifies a connection to delete
    // void
    // NIXMLConnectionsHandler::delConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string from = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     std::string to = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     // these connections were removed when the edge was deleted
    //     if (myEdgeCont.wasRemoved(from) || myEdgeCont.wasRemoved(to)) {
    //         return;
    //     }
    //     NBEdge* fromEdge = myEdgeCont.retrieve(from);
    //     NBEdge* toEdge = myEdgeCont.retrieve(to);
    //     if (fromEdge == nullptr) {
    //         myErrorMsgHandler->informf("The connection-source edge '%' to reset is not known.", from);
    //         return;
    //     }
    //     if (toEdge == nullptr) {
    //         myErrorMsgHandler->informf("The connection-destination edge '%' to reset is not known.", to);
    //         return;
    //     }
    //     if (!fromEdge->isConnectedTo(toEdge) && fromEdge->getStep() >= NBEdge::EdgeBuildingStep::EDGE2EDGES) {
    //         WRITE_WARNINGF(TL("Target edge '%' is not connected with '%'; the connection cannot be reset."), toEdge->getID(), fromEdge->getID());
    //         return;
    //     }
    //     int fromLane = -1; // Assume all lanes are to be reset.
    //     int toLane = -1;
    //     if (attrs.hasAttribute(SUMO_ATTR_LANE)
    //             || attrs.hasAttribute(SUMO_ATTR_FROM_LANE)
    //             || attrs.hasAttribute(SUMO_ATTR_TO_LANE)) {
    //         if (!parseLaneInfo(attrs, fromEdge, toEdge, &fromLane, &toLane)) {
    //             return;
    //         }
    //         // we could be trying to reset a connection loaded from a sumo net and which has become obsolete.
    //         // In this case it's ok to encounter invalid lance indices
    //         if (!fromEdge->hasConnectionTo(toEdge, toLane) && fromEdge->getStep() >= NBEdge::EdgeBuildingStep::LANES2EDGES) {
    //             WRITE_WARNINGF(TL("Edge '%' has no connection to lane '%'; the connection cannot be reset."), fromEdge->getID(), toEdge->getLaneID(toLane));
    //         }
    //     }
    //     fromEdge->removeFromConnections(toEdge, fromLane, toLane, true);
    // }
    #[allow(unused_variables)]
    pub fn del_connection(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let from = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
        let to = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
        if !ok {
            return;
        }
        // these connections were removed when the edge was deleted
        if self.myEdgeCont.m().was_removed(from.clone()) || self.myEdgeCont.m().was_removed(to.clone()) {
            return;
        }
        // extract edges
        let fromEdge = self.myEdgeCont.m().retrieve(&from, false);
        let toEdge = self.myEdgeCont.m().retrieve(&to, false);
        if fromEdge.is_null() {
            self.myErrorMsgHandler.m().inform(&format!("The connection-source edge '{from}' to reset is not known."), true);
            return;
        }
        if toEdge.is_null() {
            self.myErrorMsgHandler.m().inform(&format!("The connection-destination edge '{to}' to reset is not known."), true);
            return;
        }
        if !fromEdge.is_connected_to(toEdge, false) && fromEdge.get_step() >= NBEdge_EdgeBuildingStep::EDGE2EDGES {
            write_warning(&tlf!(
                "Target edge '%' is not connected with '%'; the connection cannot be reset.",
                toEdge.get_id(),
                fromEdge.get_id()
            ));
            return;
        }
        let mut fromLane: i32 = -1; // Assume all lanes are to be reset.
        let mut toLane: i32 = -1;
        if attrs.has_attribute(SUMO_ATTR_LANE) || attrs.has_attribute(SUMO_ATTR_FROM_LANE) || attrs.has_attribute(SUMO_ATTR_TO_LANE) {
            if !self.parse_lane_info(attrs, fromEdge, toEdge, Ptr::from_mut(&mut fromLane), Ptr::from_mut(&mut toLane)) {
                return;
            }
            // we could be trying to reset a connection loaded from a sumo net and which has become obsolete.
            // In this case it's ok to encounter invalid lance indices
            if !fromEdge.has_connection_to(toEdge, toLane, -1) && fromEdge.get_step() >= NBEdge_EdgeBuildingStep::LANES2EDGES {
                write_warning(&tlf!(
                    "Edge '%' has no connection to lane '%'; the connection cannot be reset.",
                    fromEdge.get_id(),
                    toEdge.get_lane_id(toLane)
                ));
            }
        }
        fromEdge.m().remove_from_connections(toEdge, fromLane, toLane, true, false, false);
    }

    /// C++: `void NIXMLConnectionsHandler::parseConnection(const SUMOSAXAttributes &)` [private]
    /// Parses a connection and adds it to the referenced edge
    // void
    // NIXMLConnectionsHandler::parseConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string from = attrs.get<std::string>(SUMO_ATTR_FROM, "connection", ok);
    //     std::string to = attrs.getOpt<std::string>(SUMO_ATTR_TO, "connection", ok, "");
    //     if (!ok || myEdgeCont.wasIgnored(from) || myEdgeCont.wasIgnored(to)) {
    //         return;
    //     }
    //     // extract edges
    //     NBEdge* fromEdge = myEdgeCont.retrieve(from);
    //     NBEdge* toEdge = to.length() != 0 ? myEdgeCont.retrieve(to) : nullptr;
    //     // check whether they are valid
    //     if (fromEdge == nullptr) {
    //         myErrorMsgHandler->inform("The connection-source edge '" + from + "' is not known.");
    //         return;
    //     }
    //     if (toEdge == nullptr && to.length() != 0) {
    //         myErrorMsgHandler->inform("The connection-destination edge '" + to + "' is not known.");
    //         return;
    //     }
    //     // parse optional lane information
    //     if (attrs.hasAttribute(SUMO_ATTR_LANE) || attrs.hasAttribute(SUMO_ATTR_FROM_LANE) || attrs.hasAttribute(SUMO_ATTR_TO_LANE)) {
    //         parseLaneBound(attrs, fromEdge, toEdge);
    //     } else {
    //         fromEdge->addEdge2EdgeConnection(toEdge);
    //         fromEdge->getToNode()->invalidateTLS(myTLLogicCont, true, false);
    //         const bool reset = attrs.getOpt<bool>(SUMO_ATTR_RESET, "connection", ok, false);
    //         if (reset) {
    //             fromEdge->declareConnectionsAsLoaded(NBEdge::EdgeBuildingStep::INIT);
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_PASS)
    //                 || attrs.hasAttribute(SUMO_ATTR_KEEP_CLEAR)
    //                 || attrs.hasAttribute(SUMO_ATTR_CONTPOS)
    //                 || attrs.hasAttribute(SUMO_ATTR_VISIBILITY_DISTANCE)
    //                 || attrs.hasAttribute(SUMO_ATTR_SPEED)
    //                 || attrs.hasAttribute(SUMO_ATTR_LENGTH)
    //                 || attrs.hasAttribute(SUMO_ATTR_UNCONTROLLED)
    //                 || attrs.hasAttribute(SUMO_ATTR_SHAPE)
    //                 || attrs.hasAttribute(SUMO_ATTR_ALLOW)
    //                 || attrs.hasAttribute(SUMO_ATTR_DISALLOW)) {
    //             WRITE_ERROR("No additional connection attributes are permitted in connection from edge '" + fromEdge->getID() + "' unless '"
    //                         + toString(SUMO_ATTR_FROM_LANE) + "' and '" + toString(SUMO_ATTR_TO_LANE) + "' are set.");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_connection(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let from = attrs.get::<String>(SUMO_ATTR_FROM, "connection", &mut ok, true);
        let to = attrs.get_opt::<String>(SUMO_ATTR_TO, "connection", &mut ok, String::new(), true);
        if !ok || self.myEdgeCont.m().was_ignored(from.clone()) || self.myEdgeCont.m().was_ignored(to.clone()) {
            return;
        }
        // extract edges
        let fromEdge = self.myEdgeCont.m().retrieve(&from, false);
        let toEdge = if to.len() != 0 { self.myEdgeCont.m().retrieve(&to, false) } else { Ptr::null() };
        // check whether they are valid
        if fromEdge.is_null() {
            self.myErrorMsgHandler.m().inform(&format!("The connection-source edge '{from}' is not known."), true);
            return;
        }
        if toEdge.is_null() && to.len() != 0 {
            self.myErrorMsgHandler.m().inform(&format!("The connection-destination edge '{to}' is not known."), true);
            return;
        }
        // parse optional lane information
        if attrs.has_attribute(SUMO_ATTR_LANE) || attrs.has_attribute(SUMO_ATTR_FROM_LANE) || attrs.has_attribute(SUMO_ATTR_TO_LANE) {
            self.parse_lane_bound(attrs, fromEdge, toEdge);
        } else {
            fromEdge.m().add_edge2_edge_connection(toEdge, false, SVC_UNSPECIFIED);
            fromEdge.get_to_node().m().invalidate_tls(self.myTLLogicCont.m(), true, false);
            let reset = attrs.get_opt::<bool>(SUMO_ATTR_RESET, "connection", &mut ok, false, true);
            if reset {
                fromEdge.m().declare_connections_as_loaded(NBEdge_EdgeBuildingStep::INIT);
            }
            if attrs.has_attribute(SUMO_ATTR_PASS)
                || attrs.has_attribute(SUMO_ATTR_KEEP_CLEAR)
                || attrs.has_attribute(SUMO_ATTR_CONTPOS)
                || attrs.has_attribute(SUMO_ATTR_VISIBILITY_DISTANCE)
                || attrs.has_attribute(SUMO_ATTR_SPEED)
                || attrs.has_attribute(SUMO_ATTR_LENGTH)
                || attrs.has_attribute(SUMO_ATTR_UNCONTROLLED)
                || attrs.has_attribute(SUMO_ATTR_SHAPE)
                || attrs.has_attribute(SUMO_ATTR_ALLOW)
                || attrs.has_attribute(SUMO_ATTR_DISALLOW)
            {
                write_error(&format!(
                    "No additional connection attributes are permitted in connection from edge '{}' unless '{}' and '{}' are set.",
                    fromEdge.get_id(),
                    attr_name(SUMO_ATTR_FROM_LANE),
                    attr_name(SUMO_ATTR_TO_LANE)
                ));
            }
        }
    }

    /// C++: `void NIXMLConnectionsHandler::addCrossing(const SUMOSAXAttributes &)` [private]
    /// Parses a crossing and updates the referenced node
    // void
    // NIXMLConnectionsHandler::addCrossing(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     EdgeVector edges;
    //     const std::string nodeID = attrs.get<std::string>(SUMO_ATTR_NODE, nullptr, ok);
    //     double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, nodeID.c_str(), ok, NBEdge::UNSPECIFIED_WIDTH, true);
    //     const bool discard = attrs.getOpt<bool>(SUMO_ATTR_DISCARD, nodeID.c_str(), ok, false, true);
    //     int tlIndex = attrs.getOpt<int>(SUMO_ATTR_TLLINKINDEX, nullptr, ok, -1);
    //     int tlIndex2 = attrs.getOpt<int>(SUMO_ATTR_TLLINKINDEX2, nullptr, ok, -1);
    //     NBNode* node = myNodeCont.retrieve(nodeID);
    //     if (node == nullptr) {
    //         if (!discard && myNodeCont.wasRemoved(nodeID)) {
    //             WRITE_ERRORF(TL("Node '%' in crossing is not known."), nodeID);
    //         }
    //         return;
    //     }
    //     if (!attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //         if (discard) {
    //             node->discardAllCrossings(true);
    //             return;
    //         } else {
    //             WRITE_ERRORF(TL("No edges specified for crossing at node '%'."), nodeID);
    //             return;
    //         }
    //     }
    //     for (const std::string& id : attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, nodeID.c_str(), ok)) {
    //         NBEdge* edge = myEdgeCont.retrieve(id);
    //         if (edge == nullptr) {
    //             if (!(discard && myEdgeCont.wasRemoved(id))) {
    //                 WRITE_ERRORF(TL("Edge '%' for crossing at node '%' is not known."), id, nodeID);
    //                 return;
    //             } else {
    //                 edge = myEdgeCont.retrieve(id, true);
    //             }
    //         } else {
    //             if (edge->getToNode() != node && edge->getFromNode() != node) {
    //                 if (!discard) {
    //                     WRITE_ERRORF(TL("Edge '%' does not touch node '%'."), id, nodeID);
    //                     return;
    //                 }
    //             }
    //         }
    //         edges.push_back(edge);
    //     }
    //     if (!ok) {
    //         return;
    //     }
    //     bool priority = attrs.getOpt<bool>(SUMO_ATTR_PRIORITY, nodeID.c_str(), ok, node->isTLControlled(), true);
    //     if (node->isTLControlled() && !priority) {
    //         // traffic_light nodes should always have priority crossings
    //         WRITE_WARNINGF(TL("Crossing at controlled node '%' must be prioritized"), nodeID);
    //         priority = true;
    //     }
    //     PositionVector customShape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, nullptr, ok, PositionVector::EMPTY);
    //     if (!NBNetBuilder::transformCoordinates(customShape, true, myLocation)) {
    //         WRITE_ERRORF(TL("Unable to project shape for crossing at node '%'."), node->getID());
    //     }
    //     if (discard) {
    //         node->removeCrossing(edges);
    //     } else {
    //         if (node->checkCrossingDuplicated(edges)) {
    //             // possibly a diff
    //             NBNode::Crossing* existing =  node->getCrossing(edges);
    //             if (!(
    //                         (attrs.hasAttribute(SUMO_ATTR_WIDTH) && width != existing->width)
    //                         || (attrs.hasAttribute(SUMO_ATTR_TLLINKINDEX) && tlIndex != existing->customTLIndex)
    //                         || (attrs.hasAttribute(SUMO_ATTR_TLLINKINDEX2) && tlIndex2 != existing->customTLIndex2)
    //                         || (attrs.hasAttribute(SUMO_ATTR_PRIORITY) && priority != existing->priority))) {
    //                 WRITE_ERRORF(TL("Crossing with edges '%' already exists at node '%'."), toString(edges), node->getID());
    //                 return;
    //             } else {
    //                 // replace existing, keep old attributes
    //                 if (!attrs.hasAttribute(SUMO_ATTR_WIDTH)) {
    //                     width = existing->width;
    //                 }
    //                 if (!attrs.hasAttribute(SUMO_ATTR_TLLINKINDEX)) {
    //                     tlIndex = existing->customTLIndex;
    //                 }
    //                 if (!attrs.hasAttribute(SUMO_ATTR_TLLINKINDEX2)) {
    //                     tlIndex2 = existing->customTLIndex2;
    //                 }
    //                 if (!attrs.hasAttribute(SUMO_ATTR_PRIORITY)) {
    //                     priority = existing->priority;
    //                 }
    //                 node->removeCrossing(edges);
    //             }
    //         }
    //         NBNode::Crossing* c = node->addCrossing(edges, width, priority, tlIndex, tlIndex2, customShape);
    //         myLastParameterised = c;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_crossing(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let mut edges: EdgeVector = Vec::new();
        let nodeID = attrs.get::<String>(SUMO_ATTR_NODE, "", &mut ok, true);
        let mut width = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, &nodeID, &mut ok, UNSPECIFIED_WIDTH, true);
        let discard = attrs.get_opt::<bool>(SUMO_ATTR_DISCARD, &nodeID, &mut ok, false, true);
        let mut tlIndex = attrs.get_opt::<i32>(SUMO_ATTR_TLLINKINDEX, "", &mut ok, -1, true);
        let mut tlIndex2 = attrs.get_opt::<i32>(SUMO_ATTR_TLLINKINDEX2, "", &mut ok, -1, true);
        let node = self.myNodeCont.m().retrieve(&nodeID);
        if node.is_null() {
            if !discard && self.myNodeCont.m().was_removed(nodeID.clone()) {
                write_error(&tlf!("Node '%' in crossing is not known.", nodeID));
            }
            return;
        }
        if !attrs.has_attribute(SUMO_ATTR_EDGES) {
            if discard {
                node.m().discard_all_crossings(true);
                return;
            } else {
                write_error(&tlf!("No edges specified for crossing at node '%'.", nodeID));
                return;
            }
        }
        for id in attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, &nodeID, &mut ok, true) {
            let mut edge = self.myEdgeCont.m().retrieve(&id, false);
            if edge.is_null() {
                if !(discard && self.myEdgeCont.m().was_removed(id.clone())) {
                    write_error(&tlf!("Edge '%' for crossing at node '%' is not known.", id, nodeID));
                    return;
                } else {
                    edge = self.myEdgeCont.m().retrieve(&id, true);
                }
            } else if edge.get_to_node() != node && edge.get_from_node() != node && !discard {
                write_error(&tlf!("Edge '%' does not touch node '%'.", id, nodeID));
                return;
            }
            edges.push(edge);
        }
        if !ok {
            return;
        }
        let mut priority = attrs.get_opt::<bool>(SUMO_ATTR_PRIORITY, &nodeID, &mut ok, node.is_tl_controlled(), true);
        if node.is_tl_controlled() && !priority {
            // traffic_light nodes should always have priority crossings
            write_warning(&tlf!("Crossing at controlled node '%' must be prioritized", nodeID));
            priority = true;
        }
        let mut customShape = attrs.get_opt::<PositionVector>(SUMO_ATTR_SHAPE, "", &mut ok, PositionVector::EMPTY, true);
        if !NBNetBuilder::transform_coordinates(&mut customShape, true, self.myLocation) {
            write_error(&tlf!("Unable to project shape for crossing at node '%'.", node.get_id()));
        }
        if discard {
            node.m().remove_crossing(&edges);
        } else {
            if node.m().check_crossing_duplicated(edges.clone()) {
                // possibly a diff
                let existing = get_crossing_by_edges(node, &edges);
                if !((attrs.has_attribute(SUMO_ATTR_WIDTH) && width != existing.width)
                    || (attrs.has_attribute(SUMO_ATTR_TLLINKINDEX) && tlIndex != existing.customTLIndex)
                    || (attrs.has_attribute(SUMO_ATTR_TLLINKINDEX2) && tlIndex2 != existing.customTLIndex2)
                    || (attrs.has_attribute(SUMO_ATTR_PRIORITY) && priority != existing.priority))
                {
                    write_error(&tlf!("Crossing with edges '%' already exists at node '%'.", edges_to_string(&edges), node.get_id()));
                    return;
                } else {
                    // replace existing, keep old attributes
                    if !attrs.has_attribute(SUMO_ATTR_WIDTH) {
                        width = existing.width;
                    }
                    if !attrs.has_attribute(SUMO_ATTR_TLLINKINDEX) {
                        tlIndex = existing.customTLIndex;
                    }
                    if !attrs.has_attribute(SUMO_ATTR_TLLINKINDEX2) {
                        tlIndex2 = existing.customTLIndex2;
                    }
                    if !attrs.has_attribute(SUMO_ATTR_PRIORITY) {
                        priority = existing.priority;
                    }
                    node.m().remove_crossing(&edges);
                }
            }
            let c = node.m().add_crossing(edges, width, priority, tlIndex, tlIndex2, &customShape, false, Ptr::null());
            self.myLastParameterised = Ptr::from_mut(&mut c.m().base);
        }
    }

    /// C++: `void NIXMLConnectionsHandler::addWalkingArea(const SUMOSAXAttributes &)` [private]
    /// Parses a walkingArea and updates the referenced node
    // void
    // NIXMLConnectionsHandler::addWalkingArea(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     NBNode* node = nullptr;
    //     EdgeVector edges;
    //     const std::string nodeID = attrs.get<std::string>(SUMO_ATTR_NODE, nullptr, ok);
    //     std::vector<std::string> edgeIDs;
    //     if (!attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //         WRITE_ERRORF(TL("No edges specified for walkingArea at node '%'."), nodeID);
    //         return;
    //     }
    //     for (const std::string& id : attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, nodeID.c_str(), ok)) {
    //         NBEdge* edge = myEdgeCont.retrieve(id);
    //         if (edge == nullptr) {
    //             WRITE_ERRORF(TL("Edge '%' for walkingArea at node '%' is not known."), id, nodeID);
    //             return;
    //         }
    //         if (node == nullptr) {
    //             if (edge->getToNode()->getID() == nodeID) {
    //                 node = edge->getToNode();
    //             } else if (edge->getFromNode()->getID() == nodeID) {
    //                 node = edge->getFromNode();
    //             } else {
    //                 WRITE_ERRORF(TL("Edge '%' does not touch node '%'."), id, nodeID);
    //                 return;
    //             }
    //         } else {
    //             if (edge->getToNode() != node && edge->getFromNode() != node) {
    //                 WRITE_ERRORF(TL("Edge '%' does not touch node '%'."), id, nodeID);
    //                 return;
    //             }
    //         }
    //         edges.push_back(edge);
    //     }
    //     if (!ok) {
    //         return;
    //     }
    //     PositionVector customShape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, nullptr, ok, PositionVector::EMPTY);
    //     double customWidth = attrs.getOpt<double>(SUMO_ATTR_WIDTH, nullptr, ok, NBEdge::UNSPECIFIED_WIDTH);
    //     if (!NBNetBuilder::transformCoordinates(customShape, true, myLocation)) {
    //         WRITE_ERRORF(TL("Unable to project shape for walkingArea at node '%'."), node->getID());
    //     }
    //     node->addWalkingAreaShape(edges, customShape, customWidth);
    // }
    #[allow(unused_variables)]
    pub fn add_walking_area(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let mut node: Ptr<NBNode> = Ptr::null();
        let mut edges: EdgeVector = Vec::new();
        let nodeID = attrs.get::<String>(SUMO_ATTR_NODE, "", &mut ok, true);
        if !attrs.has_attribute(SUMO_ATTR_EDGES) {
            write_error(&tlf!("No edges specified for walkingArea at node '%'.", nodeID));
            return;
        }
        for id in attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, &nodeID, &mut ok, true) {
            let edge = self.myEdgeCont.m().retrieve(&id, false);
            if edge.is_null() {
                write_error(&tlf!("Edge '%' for walkingArea at node '%' is not known.", id, nodeID));
                return;
            }
            if node.is_null() {
                if edge.get_to_node().get_id() == nodeID {
                    node = edge.get_to_node();
                } else if edge.get_from_node().get_id() == nodeID {
                    node = edge.get_from_node();
                } else {
                    write_error(&tlf!("Edge '%' does not touch node '%'.", id, nodeID));
                    return;
                }
            } else if edge.get_to_node() != node && edge.get_from_node() != node {
                write_error(&tlf!("Edge '%' does not touch node '%'.", id, nodeID));
                return;
            }
            edges.push(edge);
        }
        if !ok {
            return;
        }
        let mut customShape = attrs.get_opt::<PositionVector>(SUMO_ATTR_SHAPE, "", &mut ok, PositionVector::EMPTY, true);
        let customWidth = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, "", &mut ok, UNSPECIFIED_WIDTH, true);
        if !NBNetBuilder::transform_coordinates(&mut customShape, true, self.myLocation) {
            write_error(&tlf!("Unable to project shape for walkingArea at node '%'.", node.get_id()));
        }
        node.m().add_walking_area_shape(edges, &customShape, customWidth);
    }

    /// C++: `void NIXMLConnectionsHandler::addProhibition(const SUMOSAXAttributes &)` [private]
    /// Parses a prohibition and updates the referenced node
    // void
    // NIXMLConnectionsHandler::addProhibition(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string prohibitor = attrs.getOpt<std::string>(SUMO_ATTR_PROHIBITOR, nullptr, ok, "");
    //     std::string prohibited = attrs.getOpt<std::string>(SUMO_ATTR_PROHIBITED, nullptr, ok, "");
    //     if (!ok) {
    //         return;
    //     }
    //     NBConnection prohibitorC = parseConnectionDef("prohibitor", prohibitor);
    //     NBConnection prohibitedC = parseConnectionDef("prohibited", prohibited);
    //     if (prohibitorC == NBConnection::InvalidConnection || prohibitedC == NBConnection::InvalidConnection) {
    //         // something failed
    //         return;
    //     }
    //     NBNode* n = prohibitorC.getFrom()->getToNode();
    //     n->addSortedLinkFoes(prohibitorC, prohibitedC);
    // }
    #[allow(unused_variables)]
    pub fn add_prohibition(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let prohibitor = attrs.get_opt::<String>(SUMO_ATTR_PROHIBITOR, "", &mut ok, String::new(), true);
        let prohibited = attrs.get_opt::<String>(SUMO_ATTR_PROHIBITED, "", &mut ok, String::new(), true);
        if !ok {
            return;
        }
        let prohibitorC = self.parse_connection_def("prohibitor", &prohibitor);
        let prohibitedC = self.parse_connection_def("prohibited", &prohibited);
        if prohibitorC.eq_op(&invalid_connection()) || prohibitedC.eq_op(&invalid_connection()) {
            // something failed
            return;
        }
        let n = prohibitorC.get_from().get_to_node();
        n.m().add_sorted_link_foes(&prohibitorC, &prohibitedC);
    }

    /// C++: `void NIXMLConnectionsHandler::NIXMLConnectionsHandler(const NIXMLConnectionsHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLConnectionsHandler) -> Self {
        // private and never defined in C++ (invalidated), so it cannot be called there either
        panic!("NIXMLConnectionsHandler copy constructor is invalidated")
    }

    /// C++: `NIXMLConnectionsHandler & NIXMLConnectionsHandler::operator=(const NIXMLConnectionsHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLConnectionsHandler) -> &mut NIXMLConnectionsHandler {
        // private and never defined in C++ (invalidated), so it cannot be called there either
        panic!("NIXMLConnectionsHandler operator= is invalidated")
    }

}

impl SaxHandler for NIXMLConnectionsHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIXMLConnectionsHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIXMLConnectionsHandler::my_end_element(self, element);
        Ok(())
    }
}
