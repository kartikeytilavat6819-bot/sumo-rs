//! Port of netimport/NIXMLEdgesHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::tlf;
use utils::common::msg_handler::{write_error, write_message, write_warning};
use utils::common::std_defs::{INVALID_DOUBLE, NETWORK_VERSION};
use utils::common::string_utils::to_int;
use utils::common::to_string::f64_to_string;
use utils::geom::POSITION_EPS;
use utils::options::options_cont::with_options;

// C++ NBEdge::UNSPECIFIED_* (netbuild/NBEdge.cpp); NBEdge in Rust does not expose them yet
const UNSPECIFIED_WIDTH: f64 = -1.;
const UNSPECIFIED_OFFSET: f64 = 0.;
const UNSPECIFIED_SPEED: f64 = -1.;
const UNSPECIFIED_FRICTION: f64 = 1.;
const UNSPECIFIED_LOADED_LENGTH: f64 = -1.;

/// Importer for network edges stored in XML
/// C++ class `NIXMLEdgesHandler` (netimport/NIXMLEdgesHandler.h)
#[repr(C)]
pub struct NIXMLEdgesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// A reference to the program's options
    pub myOptions: Ptr<OptionsCont>,
    /// The current edge's id
    pub myCurrentID: String,
    /// The current edge's maximum speed
    pub myCurrentSpeed: f64,
    /// The current edge's friction
    pub myCurrentFriction: f64,
    /// The current edge's priority
    pub myCurrentPriority: i32,
    /// The current edge's number of lanes
    pub myCurrentLaneNo: i32,
    /// The current edge's lane width
    pub myCurrentWidth: f64,
    /// The current edge's offset till the destination node
    pub myCurrentEndOffset: f64,
    /// The current edge's street name
    pub myCurrentStreetName: String,
    /// The current edge's type
    pub myCurrentType: String,
    /// The nodes the edge starts and ends at
    pub myFromNode: Ptr<NBNode>,
    /// The nodes the edge starts and ends at
    pub myToNode: Ptr<NBNode>,
    /// The current edge's length
    pub myLength: f64,
    /// The shape of the edge
    pub myShape: PositionVector,
    /// Information about how to spread the lanes
    pub myLanesSpread: LaneSpreadFunction,
    /// Information about lane permissions
    pub myPermissions: SVCPermissions,
    /// Whether the edge shape shall be kept at reinitilization
    pub myReinitKeepEdgeShape: bool,
    /// The width of the sidewalk that shall be added to the current edge
    pub mySidewalkWidth: f64,
    /// The width of the bike lane that shall be added to the current edge
    pub myBikeLaneWidth: f64,
    /// Whether this edge definition is an update of a previously inserted edge
    pub myIsUpdate: bool,
    /// The nodes container (for retrieval of referenced nodes)
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The edges container (for insertion of build edges)
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The types container (for retrieval of type defaults)
    pub myTypeCont: Ptr<NBTypeCont>,
    /// The districts container (needed if an edge must be split)
    pub myDistrictCont: Ptr<NBDistrictCont>,
    /// The traffic lights container to add built tls to (when invalidating tls because of splits)
    pub myTLLogicCont: Ptr<NBTrafficLightLogicCont>,
    /// The currently processed edge
    pub myCurrentEdge: Ptr<NBEdge>,
    /// The currently processed lane index
    pub myCurrentLaneIndex: i32,
    /// The list of this edge's splits
    pub mySplits: Vec<NBEdgeCont_Split>,
    /// Information whether at least one edge's attributes were overwritten
    pub myHaveReportedAboutOverwriting: bool,
    /// Information whether at least one edge's type was changed
    pub myHaveReportedAboutTypeOverride: bool,
    pub myHaveWarnedAboutDeprecatedLaneId: bool,
    /// the default node type for splits
    pub myDefaultNodeType: SumoXMLNodeType,
    /// Whether the edge shape shall be kept generally
    pub myKeepEdgeShape: bool,
    /// element to receive parameters
    pub myLastParameterised: Vec<Ptr<Parameterised>>,
    /// The coordinate transformation which was used compute the node coordinates
    pub myLocation: Ptr<GeoConvHelper>,
}

impl std::ops::Deref for NIXMLEdgesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLEdgesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIXMLEdgesHandler {
    /// C++: `void NIXMLEdgesHandler::NIXMLEdgesHandler(NBNodeCont &, NBEdgeCont &, NBTypeCont &, NBDistrictCont &, NBTrafficLightLogicCont &, OptionsCont &)`
    /// Constructor
    // NIXMLEdgesHandler::NIXMLEdgesHandler(NBNodeCont& nc,
    //                                      NBEdgeCont& ec,
    //                                      NBTypeCont& tc,
    //                                      NBDistrictCont& dc,
    //                                      NBTrafficLightLogicCont& tlc,
    //                                      OptionsCont& options) :
    //     SUMOSAXHandler("xml-edges - file"),
    //     myOptions(options),
    //     myNodeCont(nc),
    //     myEdgeCont(ec),
    //     myTypeCont(tc),
    //     myDistrictCont(dc),
    //     myTLLogicCont(tlc),
    //     myCurrentEdge(nullptr),
    //     myCurrentLaneIndex(-1),
    //     myHaveReportedAboutOverwriting(false),
    //     myHaveReportedAboutTypeOverride(false),
    //     myHaveWarnedAboutDeprecatedLaneId(false),
    //     myDefaultNodeType(SUMOXMLDefinitions::NodeTypes.hasString(options.getString("default.junctions.type"))
    //             ? SUMOXMLDefinitions::NodeTypes.get(options.getString("default.junctions.type")) : SumoXMLNodeType::UNKNOWN),
    //     myKeepEdgeShape(!options.getBool("plain.extend-edge-shape")) {
    // }
    #[allow(unused_variables)]
    pub fn new(nc: &mut NBNodeCont, ec: &mut NBEdgeCont, tc: &mut NBTypeCont, dc: &mut NBDistrictCont, tlc: &mut NBTrafficLightLogicCont, options: &mut OptionsCont) -> Self {
        let type_s = options.get_string("default.junctions.type").unwrap_or_else(|e| panic!("{e}"));
        let default_node_type = if NodeTypes.has_string(&type_s) {
            NodeTypes.get(&type_s).unwrap_or_else(|e| panic!("{e}"))
        } else {
            SumoXMLNodeType::UNKNOWN
        };
        let keep_edge_shape = !options.get_bool("plain.extend-edge-shape").unwrap_or_else(|e| panic!("{e}"));
        NIXMLEdgesHandler {
            base: GenericSAXHandler::new(&TAGS, SUMO_TAG_NOTHING, &ATTRS, SUMO_ATTR_NOTHING, "xml-edges - file", ""),
            myOptions: Ptr::from_mut(options),
            myCurrentID: String::new(),
            myCurrentSpeed: 0.,
            myCurrentFriction: 0.,
            myCurrentPriority: 0,
            myCurrentLaneNo: 0,
            myCurrentWidth: 0.,
            myCurrentEndOffset: 0.,
            myCurrentStreetName: String::new(),
            myCurrentType: String::new(),
            myFromNode: Ptr::null(),
            myToNode: Ptr::null(),
            myLength: 0.,
            myShape: PositionVector::default(),
            // C++ leaves this uninitialized until tryGetLaneSpread sets it; the enum's zero value is used here
            myLanesSpread: LaneSpreadFunction::RIGHT,
            myPermissions: 0,
            myReinitKeepEdgeShape: false,
            mySidewalkWidth: 0.,
            myBikeLaneWidth: 0.,
            myIsUpdate: false,
            myNodeCont: Ptr::from_mut(nc),
            myEdgeCont: Ptr::from_mut(ec),
            myTypeCont: Ptr::from_mut(tc),
            myDistrictCont: Ptr::from_mut(dc),
            myTLLogicCont: Ptr::from_mut(tlc),
            myCurrentEdge: Ptr::null(),
            myCurrentLaneIndex: -1,
            mySplits: Vec::new(),
            myHaveReportedAboutOverwriting: false,
            myHaveReportedAboutTypeOverride: false,
            myHaveWarnedAboutDeprecatedLaneId: false,
            myDefaultNodeType: default_node_type,
            myKeepEdgeShape: keep_edge_shape,
            myLastParameterised: Vec::new(),
            myLocation: Ptr::null(),
        }
    }

    /// C++: `void NIXMLEdgesHandler::~NIXMLEdgesHandler() noexcept` [virtual]
    /// Destructor
    // NIXMLEdgesHandler::~NIXMLEdgesHandler() {
    //     delete myLocation;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // delete myLocation
        if !self.myLocation.is_null() {
            drop(unsafe { Box::from_raw(self.myLocation.as_raw()) });
        }
    }

    /// C++: `void NIXMLEdgesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIXMLEdgesHandler::myStartElement(int element,
    //                                   const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_VIEWSETTINGS_EDGES:
    //             // infer location for legacy networks that don't have location information
    //             myLocation = GeoConvHelper::getLoadedPlain(getFileName());
    //             break;
    //         case SUMO_TAG_LOCATION:
    //             delete myLocation;
    //             myLocation = NIImporter_SUMO::loadLocation(attrs, false);
    //             break;
    //         case SUMO_TAG_EDGE:
    //             addEdge(attrs);
    //             break;
    //         case SUMO_TAG_LANE:
    //             addLane(attrs);
    //             break;
    //         case SUMO_TAG_NEIGH:
    //             myCurrentEdge->getLaneStruct((int)myCurrentEdge->getNumLanes() - 1).oppositeID = attrs.getString(SUMO_ATTR_LANE);
    //             break;
    //         case SUMO_TAG_SPLIT:
    //             addSplit(attrs);
    //             break;
    //         case SUMO_TAG_DEL:
    //             deleteEdge(attrs);
    //             break;
    //         case SUMO_TAG_ROUNDABOUT:
    //             addRoundabout(attrs);
    //             break;
    //         case SUMO_TAG_PARAM:
    //             if (myLastParameterised.size() != 0 && myCurrentEdge != nullptr) {
    //                 bool ok = true;
    //                 const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //                 // circumventing empty string test
    //                 const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //                 myLastParameterised.back()->setParameter(key, val);
    //             }
    //             break;
    //         case SUMO_TAG_STOPOFFSET: {
    //             bool ok = true;
    //             const StopOffset stopOffset(attrs, ok);
    //             if (!ok) {
    //                 std::stringstream ss;
    //                 ss << "(Error encountered at lane " << myCurrentLaneIndex << " of edge '" << myCurrentID << "' while parsing stopOffsets.)";
    //                 WRITE_ERROR(ss.str());
    //             } else {
    //                 if (myCurrentEdge->getLaneStopOffset(myCurrentLaneIndex).isDefined()) {
    //                     std::stringstream ss;
    //                     ss << "Duplicate definition of stopOffset for ";
    //                     if (myCurrentLaneIndex != -1) {
    //                         ss << "lane " << myCurrentLaneIndex << " on ";
    //                     }
    //                     ss << "edge " << myCurrentEdge->getID() << ". Ignoring duplicate specification.";
    //                     WRITE_WARNING(ss.str());
    //                 } else if ((stopOffset.getOffset() > myCurrentEdge->getLength()) || (stopOffset.getOffset() < 0)) {
    //                     std::stringstream ss;
    //                     ss << "Ignoring invalid stopOffset for ";
    //                     if (myCurrentLaneIndex != -1) {
    //                         ss << "lane " << myCurrentLaneIndex << " on ";
    //                     }
    //                     ss << "edge " << myCurrentEdge->getID();
    //                     if (stopOffset.getOffset() > myCurrentEdge->getLength()) {
    //                         ss << " (offset larger than the edge length).";
    //                     } else {
    //                         ss << " (negative offset).";
    //                     }
    //                     WRITE_WARNING(ss.str());
    //                 } else {
    //                     myCurrentEdge->setEdgeStopOffset(myCurrentLaneIndex, stopOffset);
    //                 }
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
            SUMO_TAG_VIEWSETTINGS_EDGES => {
                // infer location for legacy networks that don't have location information
                self.myLocation = match GeoConvHelper::get_loaded_plain(self.get_file_name(), ".edg.xml") {
                    Some(loc) => Ptr::from_raw(Box::into_raw(Box::new(loc))),
                    None => Ptr::null(),
                };
            }
            SUMO_TAG_LOCATION => {
                // delete myLocation
                if !self.myLocation.is_null() {
                    drop(unsafe { Box::from_raw(self.myLocation.as_raw()) });
                }
                self.myLocation = NIImporter_SUMO::load_location(attrs, false);
            }
            SUMO_TAG_EDGE => self.add_edge(attrs),
            SUMO_TAG_LANE => self.add_lane(attrs),
            SUMO_TAG_NEIGH => {
                let n = self.myCurrentEdge.get_num_lanes() - 1;
                let oppositeID = attrs.get_string(SUMO_ATTR_LANE);
                self.myCurrentEdge.m().get_lane_struct(n).oppositeID = oppositeID;
            }
            SUMO_TAG_SPLIT => self.add_split(attrs),
            SUMO_TAG_DEL => self.delete_edge(attrs),
            SUMO_TAG_ROUNDABOUT => self.add_roundabout(attrs),
            SUMO_TAG_PARAM => {
                if self.myLastParameterised.len() != 0 && !self.myCurrentEdge.is_null() {
                    let mut ok = true;
                    let key = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
                    // circumventing empty string test
                    let val = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
                    self.myLastParameterised.last().unwrap().m().set_parameter(&key, &val);
                }
            }
            SUMO_TAG_STOPOFFSET => {
                // shortcut: C++ StopOffset(attrs, ok) constructor is not ported (utils StopOffset has no attribute ctor), add the STOPOFFSET branch body (parse, duplicate/invalid warnings, setEdgeStopOffset) when it is
            }
            _ => {}
        }
    }

    /// C++: `void NIXMLEdgesHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NIXMLEdgesHandler::myEndElement(int element) {
    //     if (element == SUMO_TAG_VIEWSETTINGS_EDGES) {
    //         delete myLocation;
    //         myLocation = nullptr;
    //         return;
    //     }
    //     if (myCurrentEdge == nullptr) {
    //         return;
    //     }
    //     if (element == SUMO_TAG_EDGE) {
    //         myLastParameterised.pop_back();
    //         // add bike lane, wait until lanes are loaded to avoid building if it already exists
    //         if (myBikeLaneWidth != NBEdge::UNSPECIFIED_WIDTH) {
    //             myCurrentEdge->addBikeLane(myBikeLaneWidth);
    //         }
    //         // add sidewalk, wait until lanes are loaded to avoid building if it already exists
    //         if (mySidewalkWidth != NBEdge::UNSPECIFIED_WIDTH) {
    //             myCurrentEdge->addSidewalk(mySidewalkWidth);
    //         }
    //         // apply default stopOffsets of edge to all lanes without specified stopOffset.
    //         const StopOffset stopOffsets = myCurrentEdge->getEdgeStopOffset();
    //         if (stopOffsets.isDefined()) {
    //             for (int i = 0; i < (int)myCurrentEdge->getLanes().size(); i++) {
    //                 myCurrentEdge->setEdgeStopOffset(i, stopOffsets, false);
    //             }
    //         }
    //         if (!myIsUpdate) {
    //             try {
    //                 if (!myEdgeCont.insert(myCurrentEdge)) {
    //                     WRITE_ERRORF(TL("Duplicate edge '%' occurred."), myCurrentID);
    //                     delete myCurrentEdge;
    //                     myCurrentEdge = nullptr;
    //                     return;
    //                 }
    //             } catch (InvalidArgument& e) {
    //                 WRITE_ERROR(e.what());
    //                 throw;
    //             } catch (...) {
    //                 WRITE_ERRORF(TL("An important information is missing in edge '%'."), myCurrentID);
    //             }
    //         }
    //         myEdgeCont.processSplits(myCurrentEdge, mySplits, myNodeCont, myDistrictCont, myTLLogicCont);
    //         myCurrentEdge = nullptr;
    //     } else if (element == SUMO_TAG_LANE && myCurrentLaneIndex != -1) {
    //         myLastParameterised.pop_back();
    //         myCurrentLaneIndex = -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_VIEWSETTINGS_EDGES {
            // delete myLocation
            if !self.myLocation.is_null() {
                drop(unsafe { Box::from_raw(self.myLocation.as_raw()) });
            }
            self.myLocation = Ptr::null();
            return;
        }
        if self.myCurrentEdge.is_null() {
            return;
        }
        if element == SUMO_TAG_EDGE {
            self.myLastParameterised.pop();
            // add bike lane, wait until lanes are loaded to avoid building if it already exists
            if self.myBikeLaneWidth != -1. {
                // -1 is NBEdge::UNSPECIFIED_WIDTH
                self.myCurrentEdge.m().add_bike_lane(self.myBikeLaneWidth);
            }
            // add sidewalk, wait until lanes are loaded to avoid building if it already exists
            if self.mySidewalkWidth != -1. {
                self.myCurrentEdge.m().add_sidewalk(self.mySidewalkWidth);
            }
            // apply default stopOffsets of edge to all lanes without specified stopOffset.
            let stopOffsets = self.myCurrentEdge.get_edge_stop_offset().clone();
            if stopOffsets.is_defined() {
                let n = self.myCurrentEdge.get_lanes().len() as i32;
                for i in 0..n {
                    self.myCurrentEdge.m().set_edge_stop_offset(i, &stopOffsets, false);
                }
            }
            if !self.myIsUpdate {
                // shortcut: C++ try/catch around myEdgeCont.insert (InvalidArgument rethrow and the "important information is missing" message) is not reproduced, Rust insert reports only a bool, upgrade when NBEdgeCont::insert can throw
                if !self.myEdgeCont.m().insert(self.myCurrentEdge, false) {
                    write_error(&tlf!("Duplicate edge '%' occurred.", self.myCurrentID));
                    drop(unsafe { Box::from_raw(self.myCurrentEdge.as_raw()) });
                    self.myCurrentEdge = Ptr::null();
                    return;
                }
            }
            // mySplits is cleared again in addEdge before the next edge reads it, so handing over the Vec is equivalent
            let splits = std::mem::take(&mut self.mySplits);
            self.myEdgeCont.m().process_splits(self.myCurrentEdge, splits, self.myNodeCont.m(), self.myDistrictCont.m(), self.myTLLogicCont.m());
            self.myCurrentEdge = Ptr::null();
        } else if element == SUMO_TAG_LANE && self.myCurrentLaneIndex != -1 {
            self.myLastParameterised.pop();
            self.myCurrentLaneIndex = -1;
        }
    }

    /// C++: `PositionVector NIXMLEdgesHandler::tryGetShape(const SUMOSAXAttributes &)` [private]
    /// Tries to parse the shape definition
    // PositionVector
    // NIXMLEdgesHandler::tryGetShape(const SUMOSAXAttributes& attrs) {
    //     if (!attrs.hasAttribute(SUMO_ATTR_SHAPE) && myShape.size() > 0) {
    //         return myShape;
    //     }
    //     // try to build shape
    //     bool ok = true;
    //     if (!attrs.hasAttribute(SUMO_ATTR_SHAPE)) {
    //         const double maxSegmentLength = OptionsCont::getOptions().getFloat("geometry.max-segment-length");
    //         if (maxSegmentLength > 0) {
    //             PositionVector shape;
    //             shape.push_back(myFromNode->getPosition());
    //             shape.push_back(myToNode->getPosition());
    //             // shape is already cartesian but we must use a copy because the original will be modified
    //             NBNetBuilder::addGeometrySegments(shape, PositionVector(shape), maxSegmentLength);
    //             return shape;
    //         } else {
    //             myReinitKeepEdgeShape = false;
    //             return PositionVector();
    //         }
    //     }
    //     PositionVector shape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, nullptr, ok, PositionVector());
    //     if (!NBNetBuilder::transformCoordinates(shape, true, myLocation)) {
    //         WRITE_ERRORF(TL("Unable to project coordinates for edge '%'."), myCurrentID);
    //     }
    //     myReinitKeepEdgeShape = myKeepEdgeShape;
    //     return shape;
    // }
    #[allow(unused_variables)]
    pub fn try_get_shape(&mut self, attrs: &SUMOSAXAttributes) -> PositionVector {
        if !attrs.has_attribute(SUMO_ATTR_SHAPE) && self.myShape.len() > 0 {
            return self.myShape.clone();
        }
        // try to build shape
        let mut ok = true;
        if !attrs.has_attribute(SUMO_ATTR_SHAPE) {
            let maxSegmentLength = utils::options::options_cont::get_float("geometry.max-segment-length");
            if maxSegmentLength > 0. {
                let mut shape = PositionVector::default();
                shape.push(self.myFromNode.get_position().clone());
                shape.push(self.myToNode.get_position().clone());
                // shape is already cartesian but we must use a copy because the original will be modified
                let cartesian = PositionVector(shape.0.clone());
                NBNetBuilder::add_geometry_segments(&mut shape, &cartesian, maxSegmentLength);
                return shape;
            } else {
                self.myReinitKeepEdgeShape = false;
                return PositionVector::default();
            }
        }
        let mut shape = attrs.get_opt::<PositionVector>(SUMO_ATTR_SHAPE, "", &mut ok, PositionVector::default(), true);
        if !NBNetBuilder::transform_coordinates(&mut shape, true, self.myLocation) {
            write_error(&tlf!("Unable to project coordinates for edge '%'.", self.myCurrentID));
        }
        self.myReinitKeepEdgeShape = self.myKeepEdgeShape;
        shape
    }

    /// C++: `LaneSpreadFunction NIXMLEdgesHandler::tryGetLaneSpread(const SUMOSAXAttributes &)` [private]
    /// Tries to parse the spread type
    // LaneSpreadFunction
    // NIXMLEdgesHandler::tryGetLaneSpread(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     LaneSpreadFunction result = myLanesSpread;
    //     std::string lsfS = toString(result);
    //     lsfS = attrs.getOpt<std::string>(SUMO_ATTR_SPREADTYPE, myCurrentID.c_str(), ok, lsfS);
    //     if (SUMOXMLDefinitions::LaneSpreadFunctions.hasString(lsfS)) {
    //         result = SUMOXMLDefinitions::LaneSpreadFunctions.get(lsfS);
    //     } else {
    //         WRITE_WARNINGF(TL("Ignoring unknown spreadType '%' for edge '%'."), lsfS, myCurrentID);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn try_get_lane_spread(&mut self, attrs: &SUMOSAXAttributes) -> LaneSpreadFunction {
        let mut ok = true;
        let mut result = self.myLanesSpread;
        let mut lsfS = LaneSpreadFunctions.get_string(result).to_string();
        lsfS = attrs.get_opt::<String>(SUMO_ATTR_SPREADTYPE, &self.myCurrentID, &mut ok, lsfS, true);
        if LaneSpreadFunctions.has_string(&lsfS) {
            result = LaneSpreadFunctions.get(&lsfS).unwrap_or_else(|e| panic!("{e}"));
        } else {
            write_warning(&tlf!("Ignoring unknown spreadType '%' for edge '%'.", lsfS, self.myCurrentID));
        }
        result
    }

    /// C++: `bool NIXMLEdgesHandler::setNodes(const SUMOSAXAttributes &)` [private]
    /// Sets from/to node information of the currently parsed edge
    // bool
    // NIXMLEdgesHandler::setNodes(const SUMOSAXAttributes& attrs) {
    //     // the names and the coordinates of the beginning and the end node
    //     // may be found, try
    //     bool ok = true;
    //     if (myIsUpdate) {
    //         myFromNode = myCurrentEdge->getFromNode();
    //         myToNode = myCurrentEdge->getToNode();
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_FROM)) {
    //         const std::string begNodeID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //         if (begNodeID != "") {
    //             myFromNode = myNodeCont.retrieve(begNodeID);
    //             if (myFromNode == nullptr) {
    //                 WRITE_ERRORF(TL("Edge's '%' from-node '%' is not known."), myCurrentID, begNodeID);
    //             }
    //         }
    //     } else if (!myIsUpdate) {
    //         WRITE_ERRORF(TL("The from-node is not given for edge '%'."), myCurrentID);
    //         ok = false;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_TO)) {
    //         const std::string endNodeID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //         if (endNodeID != "") {
    //             myToNode = myNodeCont.retrieve(endNodeID);
    //             if (myToNode == nullptr) {
    //                 WRITE_ERRORF(TL("Edge's '%' to-node '%' is not known."), myCurrentID, endNodeID);
    //             }
    //         }
    //     } else if (!myIsUpdate) {
    //         WRITE_ERRORF(TL("The to-node is not given for edge '%'."), myCurrentID);
    //         ok = false;
    //     }
    //     return ok && myFromNode != nullptr && myToNode != nullptr;
    // }
    #[allow(unused_variables)]
    pub fn set_nodes(&mut self, attrs: &SUMOSAXAttributes) -> bool {
        // the names and the coordinates of the beginning and the end node
        // may be found, try
        let mut ok = true;
        if self.myIsUpdate {
            self.myFromNode = self.myCurrentEdge.get_from_node();
            self.myToNode = self.myCurrentEdge.get_to_node();
        }
        if attrs.has_attribute(SUMO_ATTR_FROM) {
            let begNodeID = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
            if begNodeID != "" {
                self.myFromNode = self.myNodeCont.retrieve(&begNodeID);
                if self.myFromNode.is_null() {
                    write_error(&tlf!("Edge's '%' from-node '%' is not known.", self.myCurrentID, begNodeID));
                }
            }
        } else if !self.myIsUpdate {
            write_error(&tlf!("The from-node is not given for edge '%'.", self.myCurrentID));
            ok = false;
        }
        if attrs.has_attribute(SUMO_ATTR_TO) {
            let endNodeID = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
            if endNodeID != "" {
                self.myToNode = self.myNodeCont.retrieve(&endNodeID);
                if self.myToNode.is_null() {
                    write_error(&tlf!("Edge's '%' to-node '%' is not known.", self.myCurrentID, endNodeID));
                }
            }
        } else if !self.myIsUpdate {
            write_error(&tlf!("The to-node is not given for edge '%'.", self.myCurrentID));
            ok = false;
        }
        ok && !self.myFromNode.is_null() && !self.myToNode.is_null()
    }

    /// C++: `void NIXMLEdgesHandler::addEdge(const SUMOSAXAttributes &)` [private]
    /// Parses an edge and stores the values in "myCurrentEdge"
    // void
    // NIXMLEdgesHandler::addEdge(const SUMOSAXAttributes& attrs) {
    //     myIsUpdate = false;
    //     bool ok = true;
    //     // initialise the edge
    //     myCurrentEdge = nullptr;
    //     mySplits.clear();
    //     // get the id, report an error if not given or empty...
    //     myCurrentID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     myCurrentEdge = myEdgeCont.retrieve(myCurrentID);
    //     // check deprecated (unused) attributes
    //     // use default values, first
    //     myCurrentType = myCurrentEdge == nullptr ? myOptions.getString("default.type") : myCurrentEdge->getTypeID();
    //     // check whether a type's values shall be used
    //     if (attrs.hasAttribute(SUMO_ATTR_TYPE)) {
    //         myCurrentType = attrs.get<std::string>(SUMO_ATTR_TYPE, myCurrentID.c_str(), ok);
    //         if (!ok) {
    //             return;
    //         }
    //         if (!myTypeCont.knows(myCurrentType) && !myOptions.getBool("ignore-errors.edge-type")) {
    //             WRITE_ERRORF("Type '%' used by edge '%' was not defined (ignore with option --ignore-errors.edge-type).", myCurrentType, myCurrentID);
    //             return;
    //         }
    //     }
    //     myCurrentEndOffset = NBEdge::UNSPECIFIED_OFFSET;
    //     if (myCurrentEdge != nullptr) {
    //         // update existing edge. only update lane-specific settings when explicitly requested
    //         myIsUpdate = true;
    //         myCurrentSpeed = NBEdge::UNSPECIFIED_SPEED;
    //         myCurrentFriction = NBEdge::UNSPECIFIED_FRICTION;
    //         myCurrentPriority = myCurrentEdge->getPriority();
    //         myCurrentLaneNo = myCurrentEdge->getNumLanes();
    //         myPermissions = SVC_UNSPECIFIED;
    //         myCurrentWidth = NBEdge::UNSPECIFIED_WIDTH;
    //         myLanesSpread = myCurrentEdge->getLaneSpreadFunction();
    //         mySidewalkWidth = NBEdge::UNSPECIFIED_WIDTH;
    //         myBikeLaneWidth = NBEdge::UNSPECIFIED_WIDTH;
    //     } else {
    //         // this is a completely new edge. get the type specific defaults
    //         myCurrentSpeed = myTypeCont.getEdgeTypeSpeed(myCurrentType);
    //         myCurrentFriction = myTypeCont.getEdgeTypeFriction(myCurrentType);
    //         myCurrentPriority = myTypeCont.getEdgeTypePriority(myCurrentType);
    //         myCurrentLaneNo = myTypeCont.getEdgeTypeNumLanes(myCurrentType);
    //         myPermissions = myTypeCont.getEdgeTypePermissions(myCurrentType);
    //         myCurrentWidth = myTypeCont.getEdgeTypeWidth(myCurrentType);
    //         myLanesSpread = myTypeCont.getEdgeTypeSpreadType(myCurrentType);
    //         if (myLanesSpread == LaneSpreadFunction::SPREAD_UNKNOWN) {
    //             myLanesSpread = SUMOXMLDefinitions::LaneSpreadFunctions.get(myOptions.getString("default.spreadtype"));
    //         }
    //         mySidewalkWidth = myTypeCont.getEdgeTypeSidewalkWidth(myCurrentType);
    //         myBikeLaneWidth = myTypeCont.getEdgeTypeBikeLaneWidth(myCurrentType);
    //     }
    //     myShape = PositionVector();
    //     myLength = NBEdge::UNSPECIFIED_LOADED_LENGTH;
    //     myCurrentStreetName = "";
    //     myReinitKeepEdgeShape = false;
    //
    //     // use values from the edge to overwrite if existing, then
    //     if (myIsUpdate) {
    //         if (!myHaveReportedAboutOverwriting) {
    //             WRITE_MESSAGEF(TL("Duplicate edge id occurred ('%'); assuming overwriting is wished."), myCurrentID);
    //             myHaveReportedAboutOverwriting = true;
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_TYPE) && myCurrentType != myCurrentEdge->getTypeID()) {
    //             if (!myHaveReportedAboutTypeOverride) {
    //                 WRITE_MESSAGEF(TL("Edge '%' changed its type; assuming type override is wished."), myCurrentID);
    //                 myHaveReportedAboutTypeOverride = true;
    //             }
    //         }
    //         if (attrs.getOpt<bool>(SUMO_ATTR_REMOVE, myCurrentID.c_str(), ok, false)) {
    //             myEdgeCont.erase(myDistrictCont, myCurrentEdge);
    //             myCurrentEdge = nullptr;
    //             return;
    //         }
    //         if (!myCurrentEdge->hasDefaultGeometry()) {
    //             myShape = myCurrentEdge->getGeometry();
    //             myReinitKeepEdgeShape = true;
    //         }
    //         if (myCurrentEdge->hasLoadedLength()) {
    //             myLength = myCurrentEdge->getLoadedLength();
    //         }
    //         myCurrentStreetName = myCurrentEdge->getStreetName();
    //     }
    //     // speed, priority and the number of lanes have now default values;
    //     // try to read the real values from the file
    //     if (attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //         myCurrentSpeed = attrs.get<double>(SUMO_ATTR_SPEED, myCurrentID.c_str(), ok);
    //     }
    //     if (myOptions.getBool("speed-in-kmh") && myCurrentSpeed != NBEdge::UNSPECIFIED_SPEED) {
    //         myCurrentSpeed = myCurrentSpeed / (double) 3.6;
    //     }
    //     // try to read the friction value from file
    //     if (attrs.hasAttribute(SUMO_ATTR_FRICTION)) {
    //         myCurrentFriction = attrs.get<double>(SUMO_ATTR_FRICTION, myCurrentID.c_str(), ok);
    //     }
    //     // try to get the number of lanes
    //     if (attrs.hasAttribute(SUMO_ATTR_NUMLANES)) {
    //         myCurrentLaneNo = attrs.get<int>(SUMO_ATTR_NUMLANES, myCurrentID.c_str(), ok);
    //     }
    //     // try to get the priority
    //     if (attrs.hasAttribute(SUMO_ATTR_PRIORITY)) {
    //         myCurrentPriority = attrs.get<int>(SUMO_ATTR_PRIORITY, myCurrentID.c_str(), ok);
    //     }
    //     // try to get the width
    //     if (attrs.hasAttribute(SUMO_ATTR_WIDTH)) {
    //         myCurrentWidth = attrs.get<double>(SUMO_ATTR_WIDTH, myCurrentID.c_str(), ok);
    //     }
    //     // try to get the offset of the stop line from the intersection
    //     if (attrs.hasAttribute(SUMO_ATTR_ENDOFFSET)) {
    //         myCurrentEndOffset = attrs.get<double>(SUMO_ATTR_ENDOFFSET, myCurrentID.c_str(), ok);
    //     }
    //     // try to get the street name
    //     if (attrs.hasAttribute(SUMO_ATTR_NAME)) {
    //         myCurrentStreetName = attrs.get<std::string>(SUMO_ATTR_NAME, myCurrentID.c_str(), ok);
    //         if (myCurrentStreetName != "" && myOptions.isDefault("output.street-names")) {
    //             myOptions.set("output.street-names", "true");
    //         }
    //     }
    //
    //     // try to get the allowed/disallowed classes
    //     if (attrs.hasAttribute(SUMO_ATTR_ALLOW) || attrs.hasAttribute(SUMO_ATTR_DISALLOW)) {
    //         std::string allowS = attrs.hasAttribute(SUMO_ATTR_ALLOW) ? attrs.getStringSecure(SUMO_ATTR_ALLOW, "") : "";
    //         std::string disallowS = attrs.hasAttribute(SUMO_ATTR_DISALLOW) ? attrs.getStringSecure(SUMO_ATTR_DISALLOW, "") : "";
    //         // XXX matter of interpretation: should updated permissions replace or extend previously set permissions?
    //         myPermissions = parseVehicleClasses(allowS, disallowS);
    //     }
    //     // try to set the nodes
    //     if (!setNodes(attrs)) {
    //         // return if this failed
    //         myCurrentEdge = nullptr;
    //         return;
    //     }
    //     // try to get the shape
    //     myShape = tryGetShape(attrs);
    //     // try to get the spread type
    //     myLanesSpread = tryGetLaneSpread(attrs);
    //     // try to get the length
    //     myLength = attrs.getOpt<double>(SUMO_ATTR_LENGTH, myCurrentID.c_str(), ok, myLength);
    //     // try to get the sidewalkWidth
    //     mySidewalkWidth = attrs.getOpt<double>(SUMO_ATTR_SIDEWALKWIDTH, myCurrentID.c_str(), ok, mySidewalkWidth);
    //     // try to get the bikeLaneWidth
    //     myBikeLaneWidth = attrs.getOpt<double>(SUMO_ATTR_BIKELANEWIDTH, myCurrentID.c_str(), ok, myBikeLaneWidth);
    //     // insert the parsed edge into the edges map
    //     if (!ok) {
    //         myCurrentEdge = nullptr;
    //         return;
    //     }
    //     if (myFromNode == myToNode) {
    //         // this might as well be an error. We make this a warning mostly for
    //         // backward compatibility
    //         WRITE_WARNINGF(TL("Ignoring self-looped edge '%' at junction '%'"), myCurrentID, myFromNode->getID());
    //         myCurrentEdge = nullptr;
    //         return;
    //     }
    //     // check whether a previously defined edge shall be overwritten
    //     const bool applyLaneType = myCurrentEdge == nullptr;
    //     if (myCurrentEdge != nullptr) {
    //         myCurrentEdge->reinit(myFromNode, myToNode, myCurrentType, myCurrentSpeed, myCurrentFriction,
    //                               myCurrentLaneNo, myCurrentPriority, myShape,
    //                               myCurrentWidth, myCurrentEndOffset,
    //                               myCurrentStreetName, myLanesSpread,
    //                               myReinitKeepEdgeShape);
    //     } else {
    //         // the edge must be allocated in dependence to whether a shape is given
    //         if (myShape.size() == 0) {
    //             myCurrentEdge = new NBEdge(myCurrentID, myFromNode, myToNode, myCurrentType, myCurrentSpeed, myCurrentFriction,
    //                                        myCurrentLaneNo, myCurrentPriority, myCurrentWidth, myCurrentEndOffset,
    //                                        myLanesSpread, myCurrentStreetName);
    //         } else {
    //             myCurrentEdge = new NBEdge(myCurrentID, myFromNode, myToNode, myCurrentType, myCurrentSpeed, myCurrentFriction,
    //                                        myCurrentLaneNo, myCurrentPriority, myCurrentWidth, myCurrentEndOffset,
    //                                        myShape, myLanesSpread, myCurrentStreetName, "",
    //                                        myKeepEdgeShape);
    //         }
    //     }
    //     myCurrentEdge->setLoadedLength(myLength);
    //     if (myPermissions != SVC_UNSPECIFIED) {
    //         myCurrentEdge->setPermissions(myPermissions);
    //     }
    //     // apply laneType if given
    //     if (applyLaneType && myCurrentType != "" && myTypeCont.knows(myCurrentType)) {
    //         const NBTypeCont::EdgeTypeDefinition* eType = myTypeCont.getEdgeType(myCurrentType);
    //         if (eType->needsLaneType()) {
    //             int lane = 0;
    //             for (const NBTypeCont::LaneTypeDefinition& laneType : eType->laneTypeDefinitions) {
    //                 if (lane >= myCurrentLaneNo) {
    //                     break;
    //                 }
    //                 if (laneType.attrs.count(SUMO_ATTR_SPEED) > 0) {
    //                     myCurrentEdge->setSpeed(lane, laneType.speed);
    //                 }
    //                 if (laneType.attrs.count(SUMO_ATTR_FRICTION) > 0) {
    //                     myCurrentEdge->setFriction(lane, laneType.friction);
    //                 }
    //                 if (laneType.attrs.count(SUMO_ATTR_DISALLOW) > 0 || laneType.attrs.count(SUMO_ATTR_ALLOW) > 0) {
    //                     myCurrentEdge->setPermissions(laneType.permissions, lane);
    //                 }
    //                 if (laneType.attrs.count(SUMO_ATTR_WIDTH) > 0) {
    //                     myCurrentEdge->setLaneWidth(lane, laneType.width);
    //                 }
    //                 lane++;
    //             }
    //         }
    //     }
    //     // try to get the kilometrage/mileage
    //     myCurrentEdge->setDistance(attrs.getOpt<double>(SUMO_ATTR_DISTANCE, myCurrentID.c_str(), ok, myCurrentEdge->getDistance()));
    //     myCurrentEdge->setRoutingType(attrs.getOpt<std::string>(SUMO_ATTR_ROUTINGTYPE, myCurrentID.c_str(), ok, myCurrentEdge->getRoutingType()));
    //     // preserve bidi edge (only as bool, the actual edge will be recomputed)
    //     const std::string bidi = attrs.getOpt<std::string>(SUMO_ATTR_BIDI, myCurrentID.c_str(), ok, "");
    //     myCurrentEdge->setBidi(myCurrentEdge->getBidiEdge() != nullptr || myCurrentEdge->isBidi() || bidi != "");
    //
    //     myLastParameterised.push_back(myCurrentEdge);
    // }
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, attrs: &SUMOSAXAttributes) {
        self.myIsUpdate = false;
        let mut ok = true;
        // initialise the edge
        self.myCurrentEdge = Ptr::null();
        self.mySplits.clear();
        // get the id, report an error if not given or empty...
        self.myCurrentID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        self.myCurrentEdge = self.myEdgeCont.retrieve(&self.myCurrentID, false);
        // check deprecated (unused) attributes
        // use default values, first
        self.myCurrentType = if self.myCurrentEdge.is_null() {
            self.myOptions.get_string("default.type").unwrap_or_else(|err| panic!("{err}"))
        } else {
            self.myCurrentEdge.get_type_id().to_string()
        };
        // check whether a type's values shall be used
        if attrs.has_attribute(SUMO_ATTR_TYPE) {
            self.myCurrentType = attrs.get::<String>(SUMO_ATTR_TYPE, &self.myCurrentID, &mut ok, true);
            if !ok {
                return;
            }
            if !self.myTypeCont.knows(&self.myCurrentType) && !self.myOptions.get_bool("ignore-errors.edge-type").unwrap_or_else(|err| panic!("{err}")) {
                write_error(&tlf!("Type '%' used by edge '%' was not defined (ignore with option --ignore-errors.edge-type).", self.myCurrentType, self.myCurrentID));
                return;
            }
        }
        self.myCurrentEndOffset = UNSPECIFIED_OFFSET;
        if !self.myCurrentEdge.is_null() {
            // update existing edge. only update lane-specific settings when explicitly requested
            self.myIsUpdate = true;
            self.myCurrentSpeed = UNSPECIFIED_SPEED;
            self.myCurrentFriction = UNSPECIFIED_FRICTION;
            self.myCurrentPriority = self.myCurrentEdge.get_priority();
            self.myCurrentLaneNo = self.myCurrentEdge.get_num_lanes();
            self.myPermissions = SVC_UNSPECIFIED;
            self.myCurrentWidth = UNSPECIFIED_WIDTH;
            self.myLanesSpread = self.myCurrentEdge.get_lane_spread_function();
            self.mySidewalkWidth = UNSPECIFIED_WIDTH;
            self.myBikeLaneWidth = UNSPECIFIED_WIDTH;
        } else {
            // this is a completely new edge. get the type specific defaults
            self.myCurrentSpeed = self.myTypeCont.get_edge_type_speed(&self.myCurrentType);
            self.myCurrentFriction = self.myTypeCont.get_edge_type_friction(&self.myCurrentType);
            self.myCurrentPriority = self.myTypeCont.get_edge_type_priority(&self.myCurrentType);
            self.myCurrentLaneNo = self.myTypeCont.get_edge_type_num_lanes(&self.myCurrentType);
            self.myPermissions = self.myTypeCont.get_edge_type_permissions(&self.myCurrentType);
            self.myCurrentWidth = self.myTypeCont.get_edge_type_width(&self.myCurrentType);
            self.myLanesSpread = self.myTypeCont.get_edge_type_spread_type(&self.myCurrentType);
            if self.myLanesSpread == LaneSpreadFunction::SPREAD_UNKNOWN {
                let spread = self.myOptions.get_string("default.spreadtype").unwrap_or_else(|err| panic!("{err}"));
                self.myLanesSpread = LaneSpreadFunctions.get(&spread).unwrap_or_else(|err| panic!("{err}"));
            }
            self.mySidewalkWidth = self.myTypeCont.get_edge_type_sidewalk_width(&self.myCurrentType);
            self.myBikeLaneWidth = self.myTypeCont.get_edge_type_bike_lane_width(&self.myCurrentType);
        }
        self.myShape = PositionVector::default();
        self.myLength = UNSPECIFIED_LOADED_LENGTH;
        self.myCurrentStreetName = String::new();
        self.myReinitKeepEdgeShape = false;

        // use values from the edge to overwrite if existing, then
        if self.myIsUpdate {
            if !self.myHaveReportedAboutOverwriting {
                write_message(&tlf!("Duplicate edge id occurred ('%'); assuming overwriting is wished.", self.myCurrentID));
                self.myHaveReportedAboutOverwriting = true;
            }
            if attrs.has_attribute(SUMO_ATTR_TYPE) && self.myCurrentType != self.myCurrentEdge.get_type_id() {
                if !self.myHaveReportedAboutTypeOverride {
                    write_message(&tlf!("Edge '%' changed its type; assuming type override is wished.", self.myCurrentID));
                    self.myHaveReportedAboutTypeOverride = true;
                }
            }
            if attrs.get_opt::<bool>(SUMO_ATTR_REMOVE, &self.myCurrentID, &mut ok, false, true) {
                self.myEdgeCont.m().erase(self.myDistrictCont.m(), self.myCurrentEdge);
                self.myCurrentEdge = Ptr::null();
                return;
            }
            if !self.myCurrentEdge.has_default_geometry() {
                self.myShape = self.myCurrentEdge.get_geometry().clone();
                self.myReinitKeepEdgeShape = true;
            }
            if self.myCurrentEdge.has_loaded_length() {
                self.myLength = self.myCurrentEdge.get_loaded_length();
            }
            self.myCurrentStreetName = self.myCurrentEdge.get_street_name().to_string();
        }
        // speed, priority and the number of lanes have now default values;
        // try to read the real values from the file
        if attrs.has_attribute(SUMO_ATTR_SPEED) {
            self.myCurrentSpeed = attrs.get::<f64>(SUMO_ATTR_SPEED, &self.myCurrentID, &mut ok, true);
        }
        if self.myOptions.get_bool("speed-in-kmh").unwrap_or_else(|err| panic!("{err}")) && self.myCurrentSpeed != UNSPECIFIED_SPEED {
            self.myCurrentSpeed = self.myCurrentSpeed / 3.6;
        }
        // try to read the friction value from file
        if attrs.has_attribute(SUMO_ATTR_FRICTION) {
            self.myCurrentFriction = attrs.get::<f64>(SUMO_ATTR_FRICTION, &self.myCurrentID, &mut ok, true);
        }
        // try to get the number of lanes
        if attrs.has_attribute(SUMO_ATTR_NUMLANES) {
            self.myCurrentLaneNo = attrs.get::<i32>(SUMO_ATTR_NUMLANES, &self.myCurrentID, &mut ok, true);
        }
        // try to get the priority
        if attrs.has_attribute(SUMO_ATTR_PRIORITY) {
            self.myCurrentPriority = attrs.get::<i32>(SUMO_ATTR_PRIORITY, &self.myCurrentID, &mut ok, true);
        }
        // try to get the width
        if attrs.has_attribute(SUMO_ATTR_WIDTH) {
            self.myCurrentWidth = attrs.get::<f64>(SUMO_ATTR_WIDTH, &self.myCurrentID, &mut ok, true);
        }
        // try to get the offset of the stop line from the intersection
        if attrs.has_attribute(SUMO_ATTR_ENDOFFSET) {
            self.myCurrentEndOffset = attrs.get::<f64>(SUMO_ATTR_ENDOFFSET, &self.myCurrentID, &mut ok, true);
        }
        // try to get the street name
        if attrs.has_attribute(SUMO_ATTR_NAME) {
            self.myCurrentStreetName = attrs.get::<String>(SUMO_ATTR_NAME, &self.myCurrentID, &mut ok, true);
            if self.myCurrentStreetName != "" && self.myOptions.is_default("output.street-names") {
                self.myOptions.m().set("output.street-names", "true", false).unwrap_or_else(|err| panic!("{err}"));
            }
        }

        // try to get the allowed/disallowed classes
        if attrs.has_attribute(SUMO_ATTR_ALLOW) || attrs.has_attribute(SUMO_ATTR_DISALLOW) {
            let allowS = if attrs.has_attribute(SUMO_ATTR_ALLOW) { attrs.get_string_secure(SUMO_ATTR_ALLOW, "") } else { String::new() };
            let disallowS = if attrs.has_attribute(SUMO_ATTR_DISALLOW) { attrs.get_string_secure(SUMO_ATTR_DISALLOW, "") } else { String::new() };
            // XXX matter of interpretation: should updated permissions replace or extend previously set permissions?
            self.myPermissions = parse_vehicle_classes_allow_disallow(&allowS, &disallowS, NETWORK_VERSION);
        }

        // try to set the nodes
        if !self.set_nodes(attrs) {
            // return if this failed
            self.myCurrentEdge = Ptr::null();
            return;
        }
        // try to get the shape
        self.myShape = self.try_get_shape(attrs);
        // try to get the spread type
        self.myLanesSpread = self.try_get_lane_spread(attrs);
        // try to get the length
        self.myLength = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, &self.myCurrentID, &mut ok, self.myLength, true);
        // try to get the sidewalkWidth
        self.mySidewalkWidth = attrs.get_opt::<f64>(SUMO_ATTR_SIDEWALKWIDTH, &self.myCurrentID, &mut ok, self.mySidewalkWidth, true);
        // try to get the bikeLaneWidth
        self.myBikeLaneWidth = attrs.get_opt::<f64>(SUMO_ATTR_BIKELANEWIDTH, &self.myCurrentID, &mut ok, self.myBikeLaneWidth, true);
        // insert the parsed edge into the edges map
        if !ok {
            self.myCurrentEdge = Ptr::null();
            return;
        }
        if self.myFromNode == self.myToNode {
            // this might as well be an error. We make this a warning mostly for
            // backward compatibility
            write_warning(&tlf!("Ignoring self-looped edge '%' at junction '%'", self.myCurrentID, self.myFromNode.base.get_id()));
            self.myCurrentEdge = Ptr::null();
            return;
        }
        // check whether a previously defined edge shall be overwritten
        let applyLaneType = self.myCurrentEdge.is_null();
        if !self.myCurrentEdge.is_null() {
            self.myCurrentEdge.m().reinit(self.myFromNode, self.myToNode, &self.myCurrentType, self.myCurrentSpeed, self.myCurrentFriction,
                self.myCurrentLaneNo, self.myCurrentPriority, self.myShape.clone(),
                self.myCurrentWidth, self.myCurrentEndOffset,
                &self.myCurrentStreetName, self.myLanesSpread,
                self.myReinitKeepEdgeShape);
        } else {
            // the edge must be allocated in dependence to whether a shape is given
            if self.myShape.len() == 0 {
                self.myCurrentEdge = Ptr::from_raw(Box::into_raw(NBEdge::new(&self.myCurrentID, self.myFromNode, self.myToNode, self.myCurrentType.clone(), self.myCurrentSpeed, self.myCurrentFriction,
                    self.myCurrentLaneNo, self.myCurrentPriority, self.myCurrentWidth, self.myCurrentEndOffset,
                    self.myLanesSpread, &self.myCurrentStreetName)));
            } else {
                self.myCurrentEdge = Ptr::from_raw(Box::into_raw(NBEdge::new_2(&self.myCurrentID, self.myFromNode, self.myToNode, self.myCurrentType.clone(), self.myCurrentSpeed, self.myCurrentFriction,
                    self.myCurrentLaneNo, self.myCurrentPriority, self.myCurrentWidth, self.myCurrentEndOffset,
                    self.myShape.clone(), self.myLanesSpread, &self.myCurrentStreetName, "",
                    self.myKeepEdgeShape)));
            }
        }
        self.myCurrentEdge.m().set_loaded_length(self.myLength);
        if self.myPermissions != SVC_UNSPECIFIED {
            self.myCurrentEdge.m().set_permissions(self.myPermissions, -1);
        }
        // apply laneType if given
        if applyLaneType && self.myCurrentType != "" && self.myTypeCont.knows(&self.myCurrentType) {
            let eType = self.myTypeCont.get_edge_type(&self.myCurrentType);
            if eType.needs_lane_type() {
                let mut lane: i32 = 0;
                for laneType in &eType.laneTypeDefinitions {
                    if lane >= self.myCurrentLaneNo {
                        break;
                    }
                    if laneType.attrs.contains(&SUMO_ATTR_SPEED) {
                        self.myCurrentEdge.m().set_speed(lane, laneType.speed);
                    }
                    if laneType.attrs.contains(&SUMO_ATTR_FRICTION) {
                        self.myCurrentEdge.m().set_friction(lane, laneType.friction);
                    }
                    if laneType.attrs.contains(&SUMO_ATTR_DISALLOW) || laneType.attrs.contains(&SUMO_ATTR_ALLOW) {
                        self.myCurrentEdge.m().set_permissions(laneType.permissions, lane);
                    }
                    if laneType.attrs.contains(&SUMO_ATTR_WIDTH) {
                        self.myCurrentEdge.m().set_lane_width(lane, laneType.width);
                    }
                    lane += 1;
                }
            }
        }
        // try to get the kilometrage/mileage
        let distance = self.myCurrentEdge.get_distance();
        let distance = attrs.get_opt::<f64>(SUMO_ATTR_DISTANCE, &self.myCurrentID, &mut ok, distance, true);
        self.myCurrentEdge.m().set_distance(distance);
        let routingType = self.myCurrentEdge.get_routing_type().to_string();
        let routingType = attrs.get_opt::<String>(SUMO_ATTR_ROUTINGTYPE, &self.myCurrentID, &mut ok, routingType, true);
        self.myCurrentEdge.m().set_routing_type(&routingType);
        // preserve bidi edge (only as bool, the actual edge will be recomputed)
        let bidi = attrs.get_opt::<String>(SUMO_ATTR_BIDI, &self.myCurrentID, &mut ok, String::new(), true);
        let isBidi = !self.myCurrentEdge.get_bidi_edge().is_null() || self.myCurrentEdge.is_bidi() || bidi != "";
        self.myCurrentEdge.m().set_bidi(isBidi);

        // determinism: Parameterised pointers are pushed in the order the edges are parsed
        self.myLastParameterised.push(Ptr::from_mut(&mut self.myCurrentEdge.m().base2));
    }

    /// C++: `void NIXMLEdgesHandler::deleteEdge(const SUMOSAXAttributes &)` [private]
    /// parses delete tag and deletes the specified edge or lane
    // void
    // NIXMLEdgesHandler::deleteEdge(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myCurrentID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     NBEdge* edge = myEdgeCont.retrieve(myCurrentID);
    //     if (edge == nullptr) {
    //         WRITE_WARNING("Ignoring tag '" + toString(SUMO_TAG_DEL) + "' for unknown edge '" +
    //                       myCurrentID + "'");
    //         return;
    //     }
    //     const int lane = attrs.getOpt<int>(SUMO_ATTR_INDEX, myCurrentID.c_str(), ok, -1);
    //     if (lane < 0) {
    //         myEdgeCont.extract(myDistrictCont, edge, true);
    //     } else {
    //         edge->deleteLane(lane, false, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn delete_edge(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        self.myCurrentID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        let edge = self.myEdgeCont.retrieve(&self.myCurrentID, false);
        if edge.is_null() {
            // toString(SUMO_TAG_DEL) is "del"
            write_warning(&format!("Ignoring tag 'del' for unknown edge '{}'", self.myCurrentID));
            return;
        }
        let lane = attrs.get_opt::<i32>(SUMO_ATTR_INDEX, &self.myCurrentID, &mut ok, -1, true);
        if lane < 0 {
            self.myEdgeCont.m().extract(self.myDistrictCont.m(), edge, true);
        } else {
            edge.m().delete_lane(lane, false, true);
        }
    }

    /// C++: `void NIXMLEdgesHandler::addLane(const SUMOSAXAttributes &)` [private]
    /// Parses a lane and modifies myCurrentEdge according to the given attribures
    // void
    // NIXMLEdgesHandler::addLane(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentEdge == nullptr) {
    //         if (!OptionsCont::getOptions().isInStringVector("remove-edges.explicit", myCurrentID)) {
    //             WRITE_ERRORF("Additional lane information could not be set - the edge with id '%' is not known.", myCurrentID);
    //         }
    //         return;
    //     }
    //     bool ok = true;
    //     int lane;
    //     if (attrs.hasAttribute(SUMO_ATTR_ID)) {
    //         lane = attrs.get<int>(SUMO_ATTR_ID, myCurrentID.c_str(), ok);
    //         if (!myHaveWarnedAboutDeprecatedLaneId) {
    //             myHaveWarnedAboutDeprecatedLaneId = true;
    //             WRITE_WARNINGF(TL("'%' is deprecated, please use '%' instead."), toString(SUMO_ATTR_ID), toString(SUMO_ATTR_INDEX));
    //         }
    //     } else {
    //         lane = attrs.get<int>(SUMO_ATTR_INDEX, myCurrentID.c_str(), ok);
    //     }
    //     if (!ok) {
    //         return;
    //     }
    //     // check whether this lane exists
    //     if (lane >= myCurrentEdge->getNumLanes()) {
    //         WRITE_ERRORF(TL("Lane index is larger than number of lanes (edge '%')."), myCurrentID);
    //         return;
    //     }
    //     myCurrentLaneIndex = lane;
    //     // set information about allowed / disallowed vehicle classes (if specified)
    //     if (attrs.hasAttribute(SUMO_ATTR_ALLOW) || attrs.hasAttribute(SUMO_ATTR_DISALLOW)) {
    //         const std::string allowed = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, nullptr, ok, "");
    //         const std::string disallowed = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, nullptr, ok, "");
    //         myCurrentEdge->setPermissions(parseVehicleClasses(allowed, disallowed), lane);
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_PREFER)) {
    //         const std::string preferred  = attrs.get<std::string>(SUMO_ATTR_PREFER, nullptr, ok);
    //         myCurrentEdge->setPreferredVehicleClass(parseVehicleClasses(preferred), lane);
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_CHANGE_LEFT) || attrs.hasAttribute(SUMO_ATTR_CHANGE_RIGHT)) {
    //         const std::string changeLeft = attrs.getOpt<std::string>(SUMO_ATTR_CHANGE_LEFT, nullptr, ok, "");
    //         const std::string changeRight = attrs.getOpt<std::string>(SUMO_ATTR_CHANGE_RIGHT, nullptr, ok, "");
    //         myCurrentEdge->setPermittedChanging(lane, parseVehicleClasses(changeLeft, ""), parseVehicleClasses(changeRight, ""));
    //     }
    //     // try to get the width
    //     if (attrs.hasAttribute(SUMO_ATTR_WIDTH)) {
    //         myCurrentEdge->setLaneWidth(lane, attrs.get<double>(SUMO_ATTR_WIDTH, myCurrentID.c_str(), ok));
    //     }
    //     // try to get the end-offset (lane shortened due to pedestrian crossing etc..)
    //     if (attrs.hasAttribute(SUMO_ATTR_ENDOFFSET)) {
    //         myCurrentEdge->setEndOffset(lane, attrs.get<double>(SUMO_ATTR_ENDOFFSET, myCurrentID.c_str(), ok));
    //     }
    //     // try to get lane specific speed (should not occur for german networks)
    //     if (attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //         myCurrentEdge->setSpeed(lane, attrs.get<double>(SUMO_ATTR_SPEED, myCurrentID.c_str(), ok));
    //     }
    //     // try to get lane specific friction
    //     if (attrs.hasAttribute(SUMO_ATTR_FRICTION)) {
    //         myCurrentEdge->setFriction(lane, attrs.get<double>(SUMO_ATTR_FRICTION, myCurrentID.c_str(), ok));
    //     }
    //     // check whether this is an acceleration lane
    //     if (attrs.hasAttribute(SUMO_ATTR_ACCELERATION)) {
    //         myCurrentEdge->setAcceleration(lane, attrs.get<bool>(SUMO_ATTR_ACCELERATION, myCurrentID.c_str(), ok));
    //     }
    //     // check whether this lane has a custom shape
    //     if (attrs.hasAttribute(SUMO_ATTR_SHAPE)) {
    //         PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, myCurrentID.c_str(), ok);
    //         if (!NBNetBuilder::transformCoordinates(shape, true, myLocation)) {
    //             const std::string laneID = myCurrentID + "_" + toString(lane);
    //             WRITE_ERRORF(TL("Unable to project coordinates for lane '%'."), laneID);
    //         }
    //         if (shape.size() == 1) {
    //             // lane shape of length 1 is not permitted
    //             shape.push_front(myCurrentEdge->getFromNode()->getPosition());
    //             shape.push_back(myCurrentEdge->getToNode()->getPosition());
    //         }
    //         shape.removeDoublePoints();
    //         if (shape.size() < 2) {
    //             // ignore lane shape for very short lanes
    //             shape.clear();
    //         }
    //         myCurrentEdge->setLaneShape(lane, shape);
    //     }
    //     // set custom lane type
    //     if (attrs.hasAttribute(SUMO_ATTR_TYPE)) {
    //         myCurrentEdge->setLaneType(lane, attrs.get<std::string>(SUMO_ATTR_TYPE, myCurrentID.c_str(), ok));
    //     }
    //     myLastParameterised.push_back(&myCurrentEdge->getLaneStruct(lane));
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myCurrentEdge.is_null() {
            if !with_options(|oc| oc.is_in_string_vector("remove-edges.explicit", &self.myCurrentID)).unwrap_or_else(|err| panic!("{err}")) {
                write_error(&tlf!("Additional lane information could not be set - the edge with id '%' is not known.", self.myCurrentID));
            }
            return;
        }
        let mut ok = true;
        let lane: i32;
        if attrs.has_attribute(SUMO_ATTR_ID) {
            lane = attrs.get::<i32>(SUMO_ATTR_ID, &self.myCurrentID, &mut ok, true);
            if !self.myHaveWarnedAboutDeprecatedLaneId {
                self.myHaveWarnedAboutDeprecatedLaneId = true;
                // toString(SUMO_ATTR_ID) is "id", toString(SUMO_ATTR_INDEX) is "index"
                write_warning(&tlf!("'%' is deprecated, please use '%' instead.", "id", "index"));
            }
        } else {
            lane = attrs.get::<i32>(SUMO_ATTR_INDEX, &self.myCurrentID, &mut ok, true);
        }
        if !ok {
            return;
        }
        // check whether this lane exists
        if lane >= self.myCurrentEdge.get_num_lanes() {
            write_error(&tlf!("Lane index is larger than number of lanes (edge '%').", self.myCurrentID));
            return;
        }
        self.myCurrentLaneIndex = lane;
        // set information about allowed / disallowed vehicle classes (if specified)
        if attrs.has_attribute(SUMO_ATTR_ALLOW) || attrs.has_attribute(SUMO_ATTR_DISALLOW) {
            let allowed = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, "", &mut ok, String::new(), true);
            let disallowed = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, "", &mut ok, String::new(), true);
            self.myCurrentEdge.m().set_permissions(parse_vehicle_classes_allow_disallow(&allowed, &disallowed, NETWORK_VERSION), lane);
        }
        if attrs.has_attribute(SUMO_ATTR_PREFER) {
            let preferred = attrs.get::<String>(SUMO_ATTR_PREFER, "", &mut ok, true);
            self.myCurrentEdge.m().set_preferred_vehicle_class(parse_vehicle_classes(&preferred), lane);
        }
        if attrs.has_attribute(SUMO_ATTR_CHANGE_LEFT) || attrs.has_attribute(SUMO_ATTR_CHANGE_RIGHT) {
            let changeLeft = attrs.get_opt::<String>(SUMO_ATTR_CHANGE_LEFT, "", &mut ok, String::new(), true);
            let changeRight = attrs.get_opt::<String>(SUMO_ATTR_CHANGE_RIGHT, "", &mut ok, String::new(), true);
            self.myCurrentEdge.m().set_permitted_changing(lane, parse_vehicle_classes_allow_disallow(&changeLeft, "", NETWORK_VERSION), parse_vehicle_classes_allow_disallow(&changeRight, "", NETWORK_VERSION));
        }
        // try to get the width
        if attrs.has_attribute(SUMO_ATTR_WIDTH) {
            self.myCurrentEdge.m().set_lane_width(lane, attrs.get::<f64>(SUMO_ATTR_WIDTH, &self.myCurrentID, &mut ok, true));
        }
        // try to get the end-offset (lane shortened due to pedestrian crossing etc..)
        if attrs.has_attribute(SUMO_ATTR_ENDOFFSET) {
            self.myCurrentEdge.m().set_end_offset(lane, attrs.get::<f64>(SUMO_ATTR_ENDOFFSET, &self.myCurrentID, &mut ok, true));
        }
        // try to get lane specific speed (should not occur for german networks)
        if attrs.has_attribute(SUMO_ATTR_SPEED) {
            self.myCurrentEdge.m().set_speed(lane, attrs.get::<f64>(SUMO_ATTR_SPEED, &self.myCurrentID, &mut ok, true));
        }
        // try to get lane specific friction
        if attrs.has_attribute(SUMO_ATTR_FRICTION) {
            self.myCurrentEdge.m().set_friction(lane, attrs.get::<f64>(SUMO_ATTR_FRICTION, &self.myCurrentID, &mut ok, true));
        }
        // check whether this is an acceleration lane
        if attrs.has_attribute(SUMO_ATTR_ACCELERATION) {
            self.myCurrentEdge.m().set_acceleration(lane, attrs.get::<bool>(SUMO_ATTR_ACCELERATION, &self.myCurrentID, &mut ok, true));
        }
        // check whether this lane has a custom shape
        if attrs.has_attribute(SUMO_ATTR_SHAPE) {
            let mut shape = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &self.myCurrentID, &mut ok, true);
            if !NBNetBuilder::transform_coordinates(&mut shape, true, self.myLocation) {
                let laneID = format!("{}_{}", self.myCurrentID, lane);
                write_error(&tlf!("Unable to project coordinates for lane '%'.", laneID));
            }
            if shape.len() == 1 {
                // lane shape of length 1 is not permitted
                shape.push_front(self.myCurrentEdge.get_from_node().get_position().clone());
                shape.push(self.myCurrentEdge.get_to_node().get_position().clone());
            }
            shape.remove_double_points(POSITION_EPS, false, 0, 0, false);
            if shape.len() < 2 {
                // ignore lane shape for very short lanes
                shape.clear();
            }
            self.myCurrentEdge.m().set_lane_shape(lane, &shape);
        }
        // set custom lane type
        if attrs.has_attribute(SUMO_ATTR_TYPE) {
            let laneType = attrs.get::<String>(SUMO_ATTR_TYPE, &self.myCurrentID, &mut ok, true);
            self.myCurrentEdge.m().set_lane_type(lane, &laneType);
        }
        // determinism: Parameterised pointers are pushed in the order the lanes are parsed
        self.myLastParameterised.push(Ptr::from_mut(&mut self.myCurrentEdge.m().get_lane_struct(lane).base));
    }

    /// C++: `void NIXMLEdgesHandler::addSplit(const SUMOSAXAttributes &)` [private]
    /// Parses a split and stores it in mySplits. Splits are executed Upon reading the end tag of an edge
    // void NIXMLEdgesHandler::addSplit(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentEdge == nullptr) {
    //         if (!OptionsCont::getOptions().isInStringVector("remove-edges.explicit", myCurrentID)) {
    //             WRITE_WARNING(TL("Ignoring 'split' because it cannot be assigned to an edge"));
    //         }
    //         return;
    //     }
    //     bool ok = true;
    //     NBEdgeCont::Split e;
    //     e.pos = attrs.get<double>(SUMO_ATTR_POSITION, nullptr, ok);
    //     if (ok) {
    //         if (fabs(e.pos) > myCurrentEdge->getLoadedLength()) {
    //             WRITE_ERRORF(TL("Edge '%' has a split at invalid position %."), myCurrentID, toString(e.pos));
    //             return;
    //         }
    //         std::vector<NBEdgeCont::Split>::iterator i = find_if(mySplits.begin(), mySplits.end(), split_by_pos_finder(e.pos));
    //         if (i != mySplits.end()) {
    //             WRITE_ERRORF(TL("Edge '%' has already a split at position %."), myCurrentID, toString(e.pos));
    //             return;
    //         }
    //         // XXX rounding to int may duplicate the id of another split
    //         e.nameID = myCurrentID + "." + toString((int)e.pos);
    //         if (e.pos < 0) {
    //             e.pos += myCurrentEdge->getLoadedLength();
    //         }
    //         for (const std::string& id : attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LANES, myCurrentID.c_str(), ok)) {
    //             try {
    //                 int lane = StringUtils::toInt(id);
    //                 e.lanes.push_back(lane);
    //             } catch (NumberFormatException&) {
    //                 WRITE_ERRORF(TL("Error on parsing a split (edge '%')."), myCurrentID);
    //             } catch (EmptyData&) {
    //                 WRITE_ERRORF(TL("Error on parsing a split (edge '%')."), myCurrentID);
    //             }
    //         }
    //         if (e.lanes.empty()) {
    //             for (int l = 0; l < myCurrentEdge->getNumLanes(); ++l) {
    //                 e.lanes.push_back(l);
    //             }
    //         }
    //         e.speed = attrs.getOpt(SUMO_ATTR_SPEED, nullptr, ok, myCurrentEdge->getSpeed());
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEED) && myOptions.getBool("speed-in-kmh")) {
    //             e.speed /= 3.6;
    //         }
    //         e.idBefore = attrs.getOpt(SUMO_ATTR_ID_BEFORE, nullptr, ok, std::string(""));
    //         e.idAfter = attrs.getOpt(SUMO_ATTR_ID_AFTER, nullptr, ok, std::string(""));
    //         if (!ok) {
    //             return;
    //         }
    //         const std::string nodeID = attrs.getOpt(SUMO_ATTR_ID, nullptr, ok, e.nameID);
    //         if (nodeID == myCurrentEdge->getFromNode()->getID() || nodeID == myCurrentEdge->getToNode()->getID()) {
    //             WRITE_ERRORF(TL("Invalid split node id for edge '%' (from- and to-node are forbidden)"), myCurrentEdge->getID());
    //             return;
    //         }
    //         e.node = e.pos != 0 ? myNodeCont.retrieve(nodeID) : myCurrentEdge->getFromNode();
    //         e.offset = attrs.getOpt(SUMO_ATTR_OFFSET, nullptr, ok, 0.0);
    //         e.offsetFactor = OptionsCont::getOptions().getBool("lefthand") ? -1 : 1;
    //         if (e.node == nullptr) {
    //             double geomPos = e.pos;
    //             if (myCurrentEdge->hasLoadedLength()) {
    //                 geomPos *= myCurrentEdge->getGeometry().length() / myCurrentEdge->getLoadedLength();
    //             }
    //             e.node = new NBNode(nodeID, myCurrentEdge->getGeometry().positionAtOffset(geomPos));
    //             myNodeCont.insert(e.node);
    //         }
    //         NIXMLNodesHandler::processNodeType(attrs, e.node, e.node->getID(), e.node->getPosition(), false,
    //                                            myDefaultNodeType, myNodeCont, myEdgeCont, myTLLogicCont, myLocation);
    //         mySplits.push_back(e);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_split(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myCurrentEdge.is_null() {
            if !with_options(|oc| oc.is_in_string_vector("remove-edges.explicit", &self.myCurrentID)).unwrap_or_else(|err| panic!("{err}")) {
                write_warning(&tlf!("Ignoring 'split' because it cannot be assigned to an edge"));
            }
            return;
        }
        let mut ok = true;
        let mut e = NBEdgeCont_Split {
            lanes: Vec::new(),
            pos: INVALID_DOUBLE,
            speed: INVALID_DOUBLE,
            friction: INVALID_DOUBLE,
            node: Ptr::null(),
            idBefore: String::new(),
            idAfter: String::new(),
            nameID: String::new(),
            offset: 0.,
            offsetFactor: 1,
        };
        e.pos = attrs.get::<f64>(SUMO_ATTR_POSITION, "", &mut ok, true);
        if ok {
            if e.pos.abs() > self.myCurrentEdge.get_loaded_length() {
                write_error(&tlf!("Edge '%' has a split at invalid position %.", self.myCurrentID, f64_to_string(e.pos)));
                return;
            }
            let mut finder = NIXMLEdgesHandler_split_by_pos_finder::new(e.pos);
            if self.mySplits.iter().any(|s| finder.call_op(s)) {
                write_error(&tlf!("Edge '%' has already a split at position %.", self.myCurrentID, f64_to_string(e.pos)));
                return;
            }
            // XXX rounding to int may duplicate the id of another split
            e.nameID = format!("{}.{}", self.myCurrentID, e.pos as i32);
            if e.pos < 0. {
                e.pos += self.myCurrentEdge.get_loaded_length();
            }
            for id in attrs.get_opt::<Vec<String>>(SUMO_ATTR_LANES, &self.myCurrentID, &mut ok, Vec::new(), true) {
                // NumberFormatException and EmptyData both report the same error
                match to_int(&id) {
                    Ok(lane) => e.lanes.push(lane),
                    Err(_) => write_error(&tlf!("Error on parsing a split (edge '%').", self.myCurrentID)),
                }
            }
            if e.lanes.is_empty() {
                for l in 0..self.myCurrentEdge.get_num_lanes() {
                    e.lanes.push(l);
                }
            }
            e.speed = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, "", &mut ok, self.myCurrentEdge.get_speed(), true);
            if attrs.has_attribute(SUMO_ATTR_SPEED) && self.myOptions.get_bool("speed-in-kmh").unwrap_or_else(|err| panic!("{err}")) {
                e.speed /= 3.6;
            }
            e.idBefore = attrs.get_opt::<String>(SUMO_ATTR_ID_BEFORE, "", &mut ok, String::new(), true);
            e.idAfter = attrs.get_opt::<String>(SUMO_ATTR_ID_AFTER, "", &mut ok, String::new(), true);
            if !ok {
                return;
            }
            let nodeID = attrs.get_opt::<String>(SUMO_ATTR_ID, "", &mut ok, e.nameID.clone(), true);
            if nodeID == self.myCurrentEdge.get_from_node().base.get_id() || nodeID == self.myCurrentEdge.get_to_node().base.get_id() {
                write_error(&tlf!("Invalid split node id for edge '%' (from- and to-node are forbidden)", self.myCurrentEdge.base.get_id()));
                return;
            }
            e.node = if e.pos != 0. { self.myNodeCont.retrieve(&nodeID) } else { self.myCurrentEdge.get_from_node() };
            e.offset = attrs.get_opt::<f64>(SUMO_ATTR_OFFSET, "", &mut ok, 0.0, true);
            e.offsetFactor = if utils::options::options_cont::get_bool("lefthand") { -1 } else { 1 };
            if e.node.is_null() {
                let mut geomPos = e.pos;
                if self.myCurrentEdge.has_loaded_length() {
                    geomPos *= self.myCurrentEdge.get_geometry().length() / self.myCurrentEdge.get_loaded_length();
                }
                // C++ `new NBNode(id, pos)` selects the district overload with a null district
                let pos = self.myCurrentEdge.get_geometry().position_at_offset(geomPos, 0.);
                e.node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(&nodeID, &pos, Ptr::null()))));
                self.myNodeCont.m().insert_2(e.node);
            }
            let nodePos = e.node.get_position().clone();
            NIXMLNodesHandler::process_node_type(attrs, e.node, e.node.base.get_id(), &nodePos, false,
                self.myDefaultNodeType, self.myNodeCont.m(), self.myEdgeCont.m(), self.myTLLogicCont.m(), self.myLocation);
            self.mySplits.push(e);
        }
    }

    /// C++: `void NIXMLEdgesHandler::addRoundabout(const SUMOSAXAttributes &)` [private]
    /// Parses a roundabout and stores it in myEdgeCont.
    // void
    // NIXMLEdgesHandler::addRoundabout(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::vector<std::string>& edgeIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, nullptr, ok);
    //     if (ok) {
    //         EdgeSet roundabout;
    //         for (const std::string& eID : edgeIDs) {
    //             NBEdge* edge = myEdgeCont.retrieve(eID);
    //             if (edge == nullptr) {
    //                 if (!myEdgeCont.wasIgnored(eID)) {
    //                     WRITE_ERRORF(TL("Unknown edge '%' in roundabout."), eID);
    //                 }
    //             } else {
    //                 roundabout.insert(edge);
    //             }
    //         }
    //         myEdgeCont.addRoundabout(roundabout);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_roundabout(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let edgeIDs = attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, "", &mut ok, true);
        if ok {
            // determinism: EdgeSet orders by pointer address, like std::set<NBEdge*>
            let mut roundabout = EdgeSet::new();
            for eID in &edgeIDs {
                let edge = self.myEdgeCont.retrieve(eID, false);
                if edge.is_null() {
                    if !self.myEdgeCont.was_ignored(eID.clone()) {
                        write_error(&tlf!("Unknown edge '%' in roundabout.", eID));
                    }
                } else {
                    roundabout.insert(edge);
                }
            }
            self.myEdgeCont.m().add_roundabout(&roundabout);
        }
    }

    /// C++: `void NIXMLEdgesHandler::NIXMLEdgesHandler(const NIXMLEdgesHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLEdgesHandler) -> Self {
        panic!("NIXMLEdgesHandler is not copyable: the copy constructor is invalidated in C++")
    }

    /// C++: `NIXMLEdgesHandler & NIXMLEdgesHandler::operator=(const NIXMLEdgesHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLEdgesHandler) -> &mut NIXMLEdgesHandler {
        panic!("NIXMLEdgesHandler is not assignable: operator= is invalidated in C++")
    }

}

/// Finds a split at the given position
/// C++ class `NIXMLEdgesHandler::split_by_pos_finder` (netimport/NIXMLEdgesHandler.h)
pub struct NIXMLEdgesHandler_split_by_pos_finder {
    /// The position to search for
    pub myPosition: f64,
}

impl NIXMLEdgesHandler_split_by_pos_finder {
    /// C++: `void split_by_pos_finder::split_by_pos_finder(double)`
    /// Constructor
    // explicit split_by_pos_finder(double pos)
    //             : myPosition(pos) { }
    #[allow(unused_variables)]
    pub fn new(pos: f64) -> Self {
        NIXMLEdgesHandler_split_by_pos_finder { myPosition: pos }
    }

    /// C++: `bool split_by_pos_finder::operator()(const NBEdgeCont::Split &)`
    /// Comparing operator
    // bool operator()(const NBEdgeCont::Split& e) {
    //             return e.pos == myPosition;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, e: &NBEdgeCont_Split) -> bool {
        e.pos == self.myPosition
    }

}

impl SaxHandler for NIXMLEdgesHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIXMLEdgesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIXMLEdgesHandler::my_end_element(self, element);
        Ok(())
    }
}
