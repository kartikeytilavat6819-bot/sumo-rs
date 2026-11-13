//! Port of netimport/NIXMLNodesHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::tlf;
use utils::common::msg_handler::{write_error, write_warning};

/// Importer for network nodes stored in XML
/// C++ class `NIXMLNodesHandler` (netimport/NIXMLNodesHandler.h)
#[repr(C)]
pub struct NIXMLNodesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// A reference to the program's options
    pub myOptions: Ptr<OptionsCont>,
    /// The id of the currently parsed node
    pub myID: String,
    /// The position of the currently parsed node
    pub myPosition: Position,
    /// The node container to add built nodes to
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The node container to add built nodes to
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The traffic lights container to add built tls to
    pub myTLLogicCont: Ptr<NBTrafficLightLogicCont>,
    /// The coordinate transformation which was used compute the node coordinates
    pub myLocation: Ptr<GeoConvHelper>,
    /// the default node type
    pub myDefaultNodeType: SumoXMLNodeType,
    /// last item the could receive parameters
    pub myLastParameterised: Ptr<Parameterised>,
}

impl std::ops::Deref for NIXMLNodesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLNodesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIXMLNodesHandler {
    /// C++: `void NIXMLNodesHandler::NIXMLNodesHandler(NBNodeCont &, NBEdgeCont &, NBTrafficLightLogicCont &, OptionsCont &)`
    /// Constructor
    // NIXMLNodesHandler::NIXMLNodesHandler(NBNodeCont& nc, NBEdgeCont& ec,
    //                                      NBTrafficLightLogicCont& tlc,
    //                                      OptionsCont& options) :
    //     SUMOSAXHandler("xml-nodes - file"),
    //     myOptions(options),
    //     myNodeCont(nc),
    //     myEdgeCont(ec),
    //     myTLLogicCont(tlc),
    //     myLocation(nullptr),
    //     myDefaultNodeType(SUMOXMLDefinitions::NodeTypes.hasString(options.getString("default.junctions.type"))
    //             ? SUMOXMLDefinitions::NodeTypes.get(options.getString("default.junctions.type")) : SumoXMLNodeType::UNKNOWN),
    //     myLastParameterised(nullptr) {
    // }
    #[allow(unused_variables)]
    pub fn new(nc: &mut NBNodeCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, options: &mut OptionsCont) -> Self {
        let type_s = options.get_string("default.junctions.type").unwrap_or_else(|e| panic!("{e}"));
        let default_node_type = if NodeTypes.has_string(&type_s) {
            NodeTypes.get(&type_s).unwrap_or_else(|e| panic!("{e}"))
        } else {
            SumoXMLNodeType::UNKNOWN
        };
        NIXMLNodesHandler {
            base: GenericSAXHandler::new(&TAGS, SUMO_TAG_NOTHING, &ATTRS, SUMO_ATTR_NOTHING, "xml-nodes - file", ""),
            myOptions: Ptr::from_mut(options),
            myID: String::new(),
            myPosition: Position::default(),
            myNodeCont: Ptr::from_mut(nc),
            myEdgeCont: Ptr::from_mut(ec),
            myTLLogicCont: Ptr::from_mut(tlc),
            myLocation: Ptr::null(),
            myDefaultNodeType: default_node_type,
            myLastParameterised: Ptr::null(),
        }
    }

    /// C++: `void NIXMLNodesHandler::~NIXMLNodesHandler() noexcept` [virtual]
    /// Destructor
    // NIXMLNodesHandler::~NIXMLNodesHandler() {
    //     delete myLocation;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // delete myLocation
        if !self.myLocation.is_null() {
            drop(unsafe { Box::from_raw(self.myLocation.as_raw()) });
        }
    }

    /// C++: `NBNode * NIXMLNodesHandler::processNodeType(const SUMOSAXAttributes &, NBNode *, const std::string &, const Position &, bool, SumoXMLNodeType, NBNodeCont &, NBEdgeCont &, NBTrafficLightLogicCont &, GeoConvHelper *)` [static]
    /// parses node attributes (not related to positioning)
    // NBNode*
    // NIXMLNodesHandler::processNodeType(const SUMOSAXAttributes& attrs, NBNode* node, const std::string& nodeID, const Position& position,
    //                                    bool updateEdgeGeometries,
    //                                    SumoXMLNodeType type,
    //                                    NBNodeCont& nc, NBEdgeCont& ec, NBTrafficLightLogicCont& tlc,
    //                                    GeoConvHelper* from_srs) {
    //     bool ok = true;
    //     // get the type
    //     if (node != nullptr) {
    //         type = node->getType();
    //     }
    //     std::string typeS = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, nodeID.c_str(), ok, "");
    //     if (SUMOXMLDefinitions::NodeTypes.hasString(typeS)) {
    //         type = SUMOXMLDefinitions::NodeTypes.get(typeS);
    //         if (type == SumoXMLNodeType::DEAD_END_DEPRECATED || type == SumoXMLNodeType::DEAD_END) {
    //             // dead end is a computed status. Reset this to unknown so it will
    //             // be corrected if additional connections are loaded
    //             type = SumoXMLNodeType::UNKNOWN;
    //         }
    //     }
    //     std::set<NBTrafficLightDefinition*> oldTLS;
    //     // check whether a prior node shall be modified
    //     const bool isPatch = node != nullptr;
    //     if (node == nullptr) {
    //         node = new NBNode(nodeID, position, type);
    //         if (!nc.insert(node)) {
    //             throw ProcessError(TLF("Could not insert node though checked this before (id='%').", nodeID));
    //         }
    //     } else {
    //         // patch information
    //         oldTLS = node->getControllingTLS();
    //         if (node->getType() == SumoXMLNodeType::PRIORITY
    //                 && (type == SumoXMLNodeType::RIGHT_BEFORE_LEFT || type == SumoXMLNodeType::LEFT_BEFORE_RIGHT)) {
    //             ec.removeRoundabout(node);
    //         }
    //         node->reinit(position, type, updateEdgeGeometries);
    //     }
    //     // process traffic light definition
    //     if (NBNode::isTrafficLight(type)) {
    //         processTrafficLightDefinitions(attrs, node, tlc);
    //     } else if (isPatch && typeS != "") {
    //         nc.markAsNotTLS(node);
    //     }
    //     // remove previously set tls if this node is not controlled by them
    //     for (std::set<NBTrafficLightDefinition*>::iterator i = oldTLS.begin(); i != oldTLS.end(); ++i) {
    //         if ((*i)->getNodes().size() == 0) {
    //             tlc.removeFully((*i)->getID());
    //         }
    //     }
    //
    //     // set optional shape
    //     PositionVector shape;
    //     if (attrs.hasAttribute(SUMO_ATTR_SHAPE)) {
    //         shape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, nodeID.c_str(), ok, PositionVector());
    //         if (!NBNetBuilder::transformCoordinates(shape, true, from_srs)) {
    //             WRITE_ERRORF(TL("Unable to project node shape at node '%'."), node->getID());
    //         }
    //         if (shape.size() > 2) {
    //             shape.closePolygon();
    //         }
    //         node->setCustomShape(shape);
    //     }
    //     // set optional radius
    //     if (attrs.hasAttribute(SUMO_ATTR_RADIUS)) {
    //         node->setRadius(attrs.get<double>(SUMO_ATTR_RADIUS, nodeID.c_str(), ok));
    //     }
    //     // set optional keepClear flag
    //     if (attrs.hasAttribute(SUMO_ATTR_KEEP_CLEAR)) {
    //         node->setKeepClear(attrs.get<bool>(SUMO_ATTR_KEEP_CLEAR, nodeID.c_str(), ok));
    //     }
    //     node->setRightOfWay(attrs.getOpt<RightOfWay>(SUMO_ATTR_RIGHT_OF_WAY, nodeID.c_str(), ok, node->getRightOfWay()));
    //     node->setFringeType(attrs.getOpt<FringeType>(SUMO_ATTR_FRINGE, nodeID.c_str(), ok, node->getFringeType()));
    //     node->setRoundaboutType(attrs.getOpt<RoundaboutType>(SUMO_ATTR_ROUNDABOUT, nodeID.c_str(), ok, node->getRoundaboutType()));
    //     // set optional name
    //     if (attrs.hasAttribute(SUMO_ATTR_NAME)) {
    //         node->setName(attrs.get<std::string>(SUMO_ATTR_NAME, nodeID.c_str(), ok));
    //     }
    //     return node;
    // }
    #[allow(unused_variables)]
    pub fn process_node_type(attrs: &SUMOSAXAttributes, node: Ptr<NBNode>, nodeID: &str, position: &Position, updateEdgeGeometries: bool, type_: SumoXMLNodeType, nc: &mut NBNodeCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, from_srs: Ptr<GeoConvHelper>) -> Ptr<NBNode> {
        let mut ok = true;
        let mut node = node;
        let mut type_ = type_;
        // get the type
        if !node.is_null() {
            type_ = node.get_type();
        }
        let typeS = attrs.get_opt::<String>(SUMO_ATTR_TYPE, nodeID, &mut ok, String::new(), true);
        if NodeTypes.has_string(&typeS) {
            type_ = NodeTypes.get(&typeS).unwrap_or_else(|e| panic!("{e}"));
            if type_ == SumoXMLNodeType::DEAD_END_DEPRECATED || type_ == SumoXMLNodeType::DEAD_END {
                // dead end is a computed status. Reset this to unknown so it will
                // be corrected if additional connections are loaded
                type_ = SumoXMLNodeType::UNKNOWN;
            }
        }
        // determinism: C++ std::set of pointers is ordered by address; BTreeSet<Ptr> orders the same way
        let mut oldTLS: BTreeSet<Ptr<NBTrafficLightDefinition>> = BTreeSet::new();
        // check whether a prior node shall be modified
        let isPatch = !node.is_null();
        if node.is_null() {
            node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(nodeID, position, type_))));
            if !nc.insert_2(node) {
                panic!("{}", ProcessError::new(tlf!("Could not insert node though checked this before (id='%').", nodeID)));
            }
        } else {
            // patch information
            oldTLS = node.get_controlling_tls().clone();
            if node.get_type() == SumoXMLNodeType::PRIORITY && (type_ == SumoXMLNodeType::RIGHT_BEFORE_LEFT || type_ == SumoXMLNodeType::LEFT_BEFORE_RIGHT) {
                ec.remove_roundabout(node);
            }
            node.m().reinit(position, type_, updateEdgeGeometries);
        }
        // process traffic light definition
        if NBNode::is_traffic_light(type_) {
            Self::process_traffic_light_definitions(attrs, node, tlc);
        } else if isPatch && typeS != "" {
            nc.mark_as_not_tls(node);
        }
        // remove previously set tls if this node is not controlled by them
        for tl in oldTLS.iter() {
            if tl.get_nodes().len() == 0 {
                tlc.remove_fully(tl.base.get_id().to_string());
            }
        }

        // set optional shape
        let mut shape = PositionVector::default();
        if attrs.has_attribute(SUMO_ATTR_SHAPE) {
            shape = attrs.get_opt::<PositionVector>(SUMO_ATTR_SHAPE, nodeID, &mut ok, PositionVector::default(), true);
            if !NBNetBuilder::transform_coordinates(&mut shape, true, from_srs) {
                write_error(&tlf!("Unable to project node shape at node '%'.", node.base.get_id()));
            }
            if shape.len() > 2 {
                shape.close_polygon();
            }
            node.m().set_custom_shape(&shape);
        }
        // set optional radius
        if attrs.has_attribute(SUMO_ATTR_RADIUS) {
            let radius = attrs.get::<f64>(SUMO_ATTR_RADIUS, nodeID, &mut ok, true);
            node.m().set_radius(radius);
        }
        // set optional keepClear flag
        if attrs.has_attribute(SUMO_ATTR_KEEP_CLEAR) {
            let keepClear = attrs.get::<bool>(SUMO_ATTR_KEEP_CLEAR, nodeID, &mut ok, true);
            node.m().set_keep_clear(keepClear);
        }
        let rightOfWay = attrs.get_opt::<RightOfWay>(SUMO_ATTR_RIGHT_OF_WAY, nodeID, &mut ok, node.get_right_of_way(), true);
        node.m().set_right_of_way(rightOfWay);
        let fringeType = attrs.get_opt::<FringeType>(SUMO_ATTR_FRINGE, nodeID, &mut ok, node.get_fringe_type(), true);
        node.m().set_fringe_type(fringeType);
        let roundaboutType = attrs.get_opt::<RoundaboutType>(SUMO_ATTR_ROUNDABOUT, nodeID, &mut ok, node.get_roundabout_type(), true);
        node.m().set_roundabout_type(roundaboutType);
        // set optional name
        if attrs.has_attribute(SUMO_ATTR_NAME) {
            let name = attrs.get::<String>(SUMO_ATTR_NAME, nodeID, &mut ok, true);
            node.m().set_name(&name);
        }
        node
    }

    /// C++: `void NIXMLNodesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIXMLNodesHandler::myStartElement(int element,
    //                                   const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_LOCATION:
    //             delete myLocation;
    //             myLocation = NIImporter_SUMO::loadLocation(attrs);
    //             if (myLocation) {
    //                 GeoConvHelper::setLoadedPlain(getFileName(), *myLocation);
    //             }
    //             break;
    //         case SUMO_TAG_NODE:
    //             addNode(attrs);
    //             break;
    //         case SUMO_TAG_JOIN:
    //             addJoinCluster(attrs);
    //             break;
    //         case SUMO_TAG_JOINEXCLUDE:
    //             addJoinExclusion(attrs);
    //             break;
    //         case SUMO_TAG_DEL:
    //             deleteNode(attrs);
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
            SUMO_TAG_LOCATION => {
                // delete myLocation
                if !self.myLocation.is_null() {
                    drop(unsafe { Box::from_raw(self.myLocation.as_raw()) });
                }
                self.myLocation = NIImporter_SUMO::load_location(attrs, true);
                if !self.myLocation.is_null() {
                    GeoConvHelper::set_loaded_plain(self.get_file_name(), &*self.myLocation);
                }
            }
            SUMO_TAG_NODE => self.add_node(attrs),
            SUMO_TAG_JOIN => self.add_join_cluster(attrs),
            SUMO_TAG_JOINEXCLUDE => self.add_join_exclusion(attrs),
            SUMO_TAG_DEL => self.delete_node(attrs),
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

    /// C++: `void NIXMLNodesHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIXMLNodesHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_NODE:
    //             myLastParameterised = nullptr;
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            SUMO_TAG_NODE => {
                self.myLastParameterised = Ptr::null();
            }
            _ => {}
        }
    }

    /// C++: `void NIXMLNodesHandler::addNode(const SUMOSAXAttributes &)` [private]
    // void
    // NIXMLNodesHandler::addNode(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, report a warning if not given or empty...
    //     myID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     NBNode* node = myNodeCont.retrieve(myID);
    //     // retrieve the position of the node
    //     bool xOk = false;
    //     bool yOk = false;
    //     bool needConversion = true;
    //     if (node != nullptr) {
    //         myPosition = node->getPosition();
    //         xOk = yOk = true;
    //         needConversion = false;
    //     } else {
    //         myPosition.set(0, 0, 0); // better to reset than to reuse the previous (z)-value
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_X)) {
    //         myPosition.set(attrs.get<double>(SUMO_ATTR_X, myID.c_str(), ok), myPosition.y());
    //         xOk = true;
    //         needConversion = true;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_Y)) {
    //         myPosition.set(myPosition.x(), attrs.get<double>(SUMO_ATTR_Y, myID.c_str(), ok));
    //         yOk = true;
    //         needConversion = true;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_Z)) {
    //         myPosition.set(myPosition.x(), myPosition.y(), attrs.get<double>(SUMO_ATTR_Z, myID.c_str(), ok));
    //     }
    //     if (xOk && yOk) {
    //         if (needConversion && !NBNetBuilder::transformCoordinate(myPosition, true, myLocation)) {
    //             WRITE_ERRORF(TL("Unable to project coordinates for node '%'."), myID);
    //         }
    //     } else {
    //         WRITE_ERRORF(TL("Missing position (at node ID='%')."), myID);
    //     }
    //     bool updateEdgeGeometries = node != nullptr && myPosition != node->getPosition();
    //     node = processNodeType(attrs, node, myID, myPosition, updateEdgeGeometries, myDefaultNodeType, myNodeCont, myEdgeCont, myTLLogicCont, myLocation);
    //     myLastParameterised = node;
    // }
    #[allow(unused_variables)]
    pub fn add_node(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, report a warning if not given or empty...
        self.myID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        let mut node = self.myNodeCont.retrieve(&self.myID);
        // retrieve the position of the node
        let mut xOk = false;
        let mut yOk = false;
        let mut needConversion = true;
        if !node.is_null() {
            self.myPosition = node.get_position().clone();
            xOk = true;
            yOk = true;
            needConversion = false;
        } else {
            self.myPosition = Position::new3(0., 0., 0.); // better to reset than to reuse the previous (z)-value
        }
        if attrs.has_attribute(SUMO_ATTR_X) {
            let x = attrs.get::<f64>(SUMO_ATTR_X, &self.myID, &mut ok, true);
            let y = self.myPosition.y();
            self.myPosition.set(x, y);
            xOk = true;
            needConversion = true;
        }
        if attrs.has_attribute(SUMO_ATTR_Y) {
            let x = self.myPosition.x();
            let y = attrs.get::<f64>(SUMO_ATTR_Y, &self.myID, &mut ok, true);
            self.myPosition.set(x, y);
            yOk = true;
            needConversion = true;
        }
        if attrs.has_attribute(SUMO_ATTR_Z) {
            let z = attrs.get::<f64>(SUMO_ATTR_Z, &self.myID, &mut ok, true);
            self.myPosition.setz(z);
        }
        if xOk && yOk {
            if needConversion && !NBNetBuilder::transform_coordinate(&mut self.myPosition, true, self.myLocation) {
                write_error(&tlf!("Unable to project coordinates for node '%'.", self.myID));
            }
        } else {
            write_error(&tlf!("Missing position (at node ID='%').", self.myID));
        }
        let updateEdgeGeometries = !node.is_null() && self.myPosition != *node.get_position();
        node = Self::process_node_type(attrs, node, &self.myID, &self.myPosition, updateEdgeGeometries, self.myDefaultNodeType, self.myNodeCont.m(), self.myEdgeCont.m(), self.myTLLogicCont.m(), self.myLocation);
        self.myLastParameterised = if node.is_null() { Ptr::null() } else { Ptr::from_mut(&mut node.m().base2) };
    }

    /// C++: `void NIXMLNodesHandler::deleteNode(const SUMOSAXAttributes &)` [private]
    // void
    // NIXMLNodesHandler::deleteNode(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, report a warning if not given or empty...
    //     myID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     NBNode* node = myNodeCont.retrieve(myID);
    //     if (node == nullptr) {
    //         WRITE_WARNING("Ignoring tag '" + toString(SUMO_TAG_DEL) + "' for unknown node '" +
    //                       myID + "'");
    //         return;
    //     } else {
    //         myNodeCont.extract(node, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn delete_node(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, report a warning if not given or empty...
        self.myID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        let node = self.myNodeCont.retrieve(&self.myID);
        if node.is_null() {
            // toString(SUMO_TAG_DEL) is "del"
            write_warning(&("Ignoring tag 'del' for unknown node '".to_string() + &self.myID + "'"));
            return;
        } else {
            self.myNodeCont.m().extract(node, true);
        }
    }

    /// C++: `void NIXMLNodesHandler::addJoinCluster(const SUMOSAXAttributes &)` [private]
    // void
    // NIXMLNodesHandler::addJoinCluster(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string clusterString = attrs.get<std::string>(SUMO_ATTR_NODES, nullptr, ok);
    //     const std::set<std::string>& cluster = StringTokenizer(clusterString).getSet();
    //
    //     myID = attrs.getOpt<std::string>(SUMO_ATTR_ID, nullptr, ok, myNodeCont.createClusterId(cluster));
    //
    //     Position pos = Position::INVALID;
    //     if (attrs.hasAttribute(SUMO_ATTR_X)) {
    //         pos.setx(attrs.get<double>(SUMO_ATTR_X, myID.c_str(), ok));
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_Y)) {
    //         pos.sety(attrs.get<double>(SUMO_ATTR_Y, myID.c_str(), ok));
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_Z)) {
    //         pos.setz(attrs.get<double>(SUMO_ATTR_Z, myID.c_str(), ok));
    //     }
    //     const bool reset = attrs.getOpt<bool>(SUMO_ATTR_RESET, myID.c_str(), ok, OptionsCont::getOptions().getBool("junctions.join-reset"));
    //
    //     NBNode* node = processNodeType(attrs, nullptr, myID, pos, false, myDefaultNodeType, myNodeCont, myEdgeCont, myTLLogicCont, myLocation);
    //     if (ok) {
    //         myNodeCont.addCluster2Join(cluster, node, reset);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_join_cluster(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let clusterString = attrs.get::<String>(SUMO_ATTR_NODES, "", &mut ok, true);
        let cluster = StringTokenizer::new(&clusterString).get_set();
        let default_id = self.myNodeCont.m().create_cluster_id_2(&cluster, "cluster_");
        self.myID = attrs.get_opt::<String>(SUMO_ATTR_ID, "", &mut ok, default_id, true);
        let mut pos = Position::INVALID;
        if attrs.has_attribute(SUMO_ATTR_X) {
            pos.setx(attrs.get::<f64>(SUMO_ATTR_X, &self.myID, &mut ok, true));
        }
        if attrs.has_attribute(SUMO_ATTR_Y) {
            pos.sety(attrs.get::<f64>(SUMO_ATTR_Y, &self.myID, &mut ok, true));
        }
        if attrs.has_attribute(SUMO_ATTR_Z) {
            pos.setz(attrs.get::<f64>(SUMO_ATTR_Z, &self.myID, &mut ok, true));
        }
        let reset = attrs.get_opt::<bool>(SUMO_ATTR_RESET, &self.myID, &mut ok, utils::options::options_cont::get_bool("junctions.join-reset"), true);
        let node = Self::process_node_type(attrs, Ptr::null(), &self.myID, &pos, false, self.myDefaultNodeType, self.myNodeCont.m(), self.myEdgeCont.m(), self.myTLLogicCont.m(), self.myLocation);
        if ok {
            self.myNodeCont.m().add_cluster2_join(&cluster, node, reset);
        }
    }

    /// C++: `void NIXMLNodesHandler::addJoinExclusion(const SUMOSAXAttributes &)` [private]
    // void
    // NIXMLNodesHandler::addJoinExclusion(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::vector<std::string> ids = StringTokenizer(
    //             attrs.get<std::string>(SUMO_ATTR_NODES, nullptr, ok)).getVector();
    //     if (ok) {
    //         myNodeCont.addJoinExclusion(ids);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_join_exclusion(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let nodes = attrs.get::<String>(SUMO_ATTR_NODES, "", &mut ok, true);
        let ids: Vec<String> = StringTokenizer::new(&nodes).get_vector();
        if ok {
            self.myNodeCont.m().add_join_exclusion(&ids);
        }
    }

    /// C++: `void NIXMLNodesHandler::processTrafficLightDefinitions(const SUMOSAXAttributes &, NBNode *, NBTrafficLightLogicCont &)` [static, private]
    /// Builds the defined traffic light or adds a node to it
    // void
    // NIXMLNodesHandler::processTrafficLightDefinitions(const SUMOSAXAttributes& attrs,
    //         NBNode* currentNode, NBTrafficLightLogicCont& tlc) {
    //     // try to get the tl-id
    //     // if a tl-id is given, we will look whether this tl already exists
    //     //  if so, we will add the node to it (and to all programs with this id), otherwise allocate a new one with this id
    //     // if no tl-id exists, we will build a tl with the node's id
    //     std::set<NBTrafficLightDefinition*> tlDefs;
    //     bool ok = true;
    //
    //     std::string oldTlID = "";
    //     std::string oldTypeS = OptionsCont::getOptions().getString("tls.default-type");
    //
    //     if (currentNode->isTLControlled()) {
    //         NBTrafficLightDefinition* oldDef = *(currentNode->getControllingTLS().begin());
    //         oldTlID = oldDef->getID();
    //         oldTypeS = toString(oldDef->getType());
    //     }
    //     std::string tlID = attrs.getOpt<std::string>(SUMO_ATTR_TLID, nullptr, ok, oldTlID);
    //     std::string typeS = attrs.getOpt<std::string>(SUMO_ATTR_TLTYPE, nullptr, ok, oldTypeS);
    //     if (tlID != oldTlID || typeS != oldTypeS) {
    //         currentNode->removeTrafficLights();
    //     }
    //     TrafficLightType type;
    //     if (SUMOXMLDefinitions::TrafficLightTypes.hasString(typeS)) {
    //         type = SUMOXMLDefinitions::TrafficLightTypes.get(typeS);
    //     } else {
    //         WRITE_ERRORF(TL("Unknown traffic light type '%' for node '%'."), typeS, currentNode->getID());
    //         return;
    //     }
    //     TrafficLightLayout layout = TrafficLightLayout::DEFAULT;
    //     if (attrs.hasAttribute(SUMO_ATTR_TLLAYOUT)) {
    //         std::string layoutS = attrs.get<std::string>(SUMO_ATTR_TLLAYOUT, nullptr, ok);
    //         if (SUMOXMLDefinitions::TrafficLightLayouts.hasString(layoutS)) {
    //             layout = SUMOXMLDefinitions::TrafficLightLayouts.get(layoutS);
    //         } else {
    //             WRITE_ERRORF(TL("Unknown traffic light layout '%' for node '%'."), typeS, currentNode->getID());
    //             return;
    //         }
    //     }
    //     if (tlID != "" && tlc.getPrograms(tlID).size() > 0) {
    //         // we already have definitions for this tlID
    //         for (auto item : tlc.getPrograms(tlID)) {
    //             NBTrafficLightDefinition* def = item.second;
    //             tlDefs.insert(def);
    //             def->addNode(currentNode);
    //             if (def->getType() != type && attrs.hasAttribute(SUMO_ATTR_TLTYPE)) {
    //                 WRITE_WARNINGF(TL("Changing traffic light type '%' to '%' for tl '%'."), toString(def->getType()), typeS, tlID);
    //                 def->setType(type);
    //                 if (type != TrafficLightType::STATIC && dynamic_cast<NBLoadedSUMOTLDef*>(def) != nullptr) {
    //                     dynamic_cast<NBLoadedSUMOTLDef*>(def)->guessMinMaxDuration();
    //                 }
    //             }
    //             if (layout != TrafficLightLayout::DEFAULT && dynamic_cast<NBOwnTLDef*>(def) != nullptr) {
    //                 dynamic_cast<NBOwnTLDef*>(def)->setLayout(layout);
    //             }
    //         }
    //     } else {
    //         // we need to add a new defition
    //         tlID = (tlID == "" ? currentNode->getID() : tlID);
    //         NBOwnTLDef* tlDef = new NBOwnTLDef(tlID, currentNode, 0, type);
    //         if (!tlc.insert(tlDef)) {
    //             // actually, nothing should fail here
    //             delete tlDef;
    //             throw ProcessError(TLF("Could not allocate tls '%'.", currentNode->getID()));
    //         }
    //         tlDef->setLayout(layout);
    //         tlDefs.insert(tlDef);
    //     }
    //     // process inner edges which shall be controlled
    //     const std::vector<std::string>& controlledInner = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_CONTROLLED_INNER, nullptr, ok);
    //     if (controlledInner.size() != 0) {
    //         for (std::set<NBTrafficLightDefinition*>::iterator it = tlDefs.begin(); it != tlDefs.end(); it++) {
    //             (*it)->addControlledInnerEdges(controlledInner);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process_traffic_light_definitions(attrs: &SUMOSAXAttributes, currentNode: Ptr<NBNode>, tlc: &mut NBTrafficLightLogicCont) {
        // determinism: C++ std::set of pointers is ordered by address; BTreeSet<Ptr> orders the same way
        let mut tlDefs: BTreeSet<Ptr<NBTrafficLightDefinition>> = BTreeSet::new();
        let mut ok = true;
        let mut oldTlID = String::new();
        let mut oldTypeS = utils::options::options_cont::get_string("tls.default-type");
        if currentNode.is_tl_controlled() {
            let oldDef: Ptr<NBTrafficLightDefinition> = *currentNode.get_controlling_tls().iter().next().unwrap();
            oldTlID = oldDef.base.get_id().to_string();
            oldTypeS = TrafficLightTypes.get_string(oldDef.get_type()).to_string();
        }
        let mut tlID = attrs.get_opt::<String>(SUMO_ATTR_TLID, "", &mut ok, oldTlID.clone(), true);
        let typeS = attrs.get_opt::<String>(SUMO_ATTR_TLTYPE, "", &mut ok, oldTypeS.clone(), true);
        if tlID != oldTlID || typeS != oldTypeS {
            currentNode.m().remove_traffic_lights(false);
        }
        let type_ = if TrafficLightTypes.has_string(&typeS) {
            TrafficLightTypes.get(&typeS).unwrap_or_else(|e| panic!("{e}"))
        } else {
            write_error(&tlf!("Unknown traffic light type '%' for node '%'.", typeS, currentNode.base.get_id()));
            return;
        };
        let mut layout = TrafficLightLayout::DEFAULT;
        if attrs.has_attribute(SUMO_ATTR_TLLAYOUT) {
            let layoutS = attrs.get::<String>(SUMO_ATTR_TLLAYOUT, "", &mut ok, true);
            if TrafficLightLayouts.has_string(&layoutS) {
                layout = TrafficLightLayouts.get(&layoutS).unwrap_or_else(|e| panic!("{e}"));
            } else {
                write_error(&tlf!("Unknown traffic light layout '%' for node '%'.", typeS, currentNode.base.get_id()));
                return;
            }
        }
        if tlID != "" && tlc.get_programs(&tlID).len() > 0 {
            // we already have definitions for this tlID
            for item in tlc.get_programs(&tlID).values() {
                let def: Ptr<NBTrafficLightDefinition> = *item;
                tlDefs.insert(def);
                def.m().add_node(currentNode);
                if def.get_type() != type_ && attrs.has_attribute(SUMO_ATTR_TLTYPE) {
                    write_warning(&tlf!("Changing traffic light type '%' to '%' for tl '%'.", TrafficLightTypes.get_string(def.get_type()), typeS, tlID));
                    def.m().set_type(type_);
                    // shortcut: C++ dynamic_cast<NBLoadedSUMOTLDef*> (guessMinMaxDuration) and dynamic_cast<NBOwnTLDef*> (setLayout) need a runtime type tag NBTrafficLightDefinition lacks, so these two steps are skipped, upgrade when the TL definition carries its kind
                }
            }
        } else {
            // we need to add a new defition
            if tlID == "" {
                tlID = currentNode.base.get_id().to_string();
            }
            let tlDef: Ptr<NBOwnTLDef> = Ptr::from_raw(Box::into_raw(Box::new(NBOwnTLDef::new_2(&tlID, currentNode, 0, type_))));
            currentNode.m().add_traffic_light(tlDef.cast());
            if !tlc.insert(tlDef.cast(), false) {
                // actually, nothing should fail here
                drop(unsafe { Box::from_raw(tlDef.as_raw()) });
                panic!("{}", ProcessError::new(tlf!("Could not allocate tls '%'.", currentNode.base.get_id())));
            }
            tlDef.m().set_layout(layout);
            tlDefs.insert(tlDef.cast());
        }
        // process inner edges which shall be controlled
        let controlledInner = attrs.get_opt::<Vec<String>>(SUMO_ATTR_CONTROLLED_INNER, "", &mut ok, Vec::new(), true);
        if controlledInner.len() != 0 {
            for def in tlDefs.iter() {
                def.m().add_controlled_inner_edges(&controlledInner);
            }
        }
    }

    /// C++: `void NIXMLNodesHandler::NIXMLNodesHandler(const NIXMLNodesHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLNodesHandler) -> Self {
        panic!("NIXMLNodesHandler is not copyable: the copy constructor is private and undefined in C++")
    }

    /// C++: `NIXMLNodesHandler & NIXMLNodesHandler::operator=(const NIXMLNodesHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLNodesHandler) -> &mut NIXMLNodesHandler {
        panic!("NIXMLNodesHandler is not assignable: operator= is invalidated in C++")
    }

}

impl SaxHandler for NIXMLNodesHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIXMLNodesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIXMLNodesHandler::my_end_element(self, element);
        Ok(())
    }
}
