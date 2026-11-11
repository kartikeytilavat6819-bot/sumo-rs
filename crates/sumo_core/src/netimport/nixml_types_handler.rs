//! Port of netimport/NIXMLTypesHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Importer for edge type information stored in XML
/// C++ class `NIXMLTypesHandler` (netimport/NIXMLTypesHandler.h)
pub struct NIXMLTypesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// The type container to fill
    pub myTypeCont: Ptr<NBTypeCont>,
    /// The currently parsed type
    pub myCurrentTypeID: String,
}

impl std::ops::Deref for NIXMLTypesHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLTypesHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
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
        todo!("port NIXMLTypesHandler::NIXMLTypesHandler")
    }

    /// C++: `void NIXMLTypesHandler::~NIXMLTypesHandler()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIXMLTypesHandler::~NIXMLTypesHandler")
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
        todo!("port NIXMLTypesHandler::myStartElement")
    }

    /// C++: `void NIXMLTypesHandler::NIXMLTypesHandler(const NIXMLTypesHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIXMLTypesHandler) -> Self {
        todo!("port NIXMLTypesHandler::NIXMLTypesHandler")
    }

    /// C++: `NIXMLTypesHandler & NIXMLTypesHandler::operator=(const NIXMLTypesHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIXMLTypesHandler) -> &mut NIXMLTypesHandler {
        todo!("port NIXMLTypesHandler::operator=")
    }

}

