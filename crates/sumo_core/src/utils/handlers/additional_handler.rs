//! Port of utils/handlers/AdditionalHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::common_handler::{attr_str, tag_str};

/// C++ class `AdditionalHandler` (utils/handlers/AdditionalHandler.h)
#[repr(C)]
pub struct AdditionalHandler {
    /// C++ base class `CommonHandler`
    pub base: CommonHandler,
}

impl std::ops::Deref for AdditionalHandler {
    type Target = CommonHandler;
    fn deref(&self) -> &CommonHandler {
        &self.base
    }
}
impl std::ops::DerefMut for AdditionalHandler {
    fn deref_mut(&mut self) -> &mut CommonHandler {
        &mut self.base
    }
}

impl AdditionalHandler {
    /// C++: `void AdditionalHandler::AdditionalHandler(FileBucket *)`
    /// Constructor
    // AdditionalHandler::AdditionalHandler(FileBucket* fileBucket) :
    //     CommonHandler(fileBucket) {
    // }
    #[allow(unused_variables)]
    pub fn new(fileBucket: Ptr<Unmapped /* FileBucket */>) -> Self {
        Self { base: CommonHandler::new(fileBucket) }
    }

    /// C++: `void AdditionalHandler::~AdditionalHandler() noexcept` [virtual]
    /// Destructor
    // AdditionalHandler::~AdditionalHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `bool AdditionalHandler::beginParseAttributes(SumoXMLTag, const SUMOSAXAttributes &)`
    /// begin parse attributes
    // bool
    // AdditionalHandler::beginParseAttributes(SumoXMLTag tag, const SUMOSAXAttributes& attrs) {
    //     // open SUMOBaseOBject
    //     myCommonXMLStructure.openSUMOBaseOBject();
    //     // check tag
    //     try {
    //         switch (tag) {
    //             // Stopping Places
    //             case SUMO_TAG_BUS_STOP:
    //                 parseBusStopAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_TRAIN_STOP:
    //                 parseTrainStopAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_ACCESS:
    //                 parseAccessAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_CONTAINER_STOP:
    //                 parseContainerStopAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_CHARGING_STATION:
    //                 parseChargingStationAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_PARKING_AREA:
    //                 parseParkingAreaAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_PARKING_SPACE:
    //                 parseParkingSpaceAttributes(attrs);
    //                 break;
    //             // Detectors
    //             case SUMO_TAG_E1DETECTOR:
    //             case SUMO_TAG_INDUCTION_LOOP:
    //                 parseE1Attributes(attrs);
    //                 break;
    //             case SUMO_TAG_E2DETECTOR:
    //             case SUMO_TAG_LANE_AREA_DETECTOR:
    //                 parseE2Attributes(attrs);
    //                 break;
    //             case SUMO_TAG_E3DETECTOR:
    //             case SUMO_TAG_ENTRY_EXIT_DETECTOR:
    //                 parseE3Attributes(attrs);
    //                 break;
    //             case SUMO_TAG_DET_ENTRY:
    //                 parseEntryAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_DET_EXIT:
    //                 parseExitAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_INSTANT_INDUCTION_LOOP:
    //                 parseE1InstantAttributes(attrs);
    //                 break;
    //             // TAZs
    //             case SUMO_TAG_TAZ:
    //                 parseTAZAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_TAZSOURCE:
    //                 parseTAZSourceAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_TAZSINK:
    //                 parseTAZSinkAttributes(attrs);
    //                 break;
    //             // Variable Speed Sign
    //             case SUMO_TAG_VSS:
    //                 parseVariableSpeedSignAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_STEP:
    //                 parseVariableSpeedSignStepAttributes(attrs);
    //                 break;
    //             // Calibrator
    //             case SUMO_TAG_CALIBRATOR:
    //             case GNE_TAG_CALIBRATOR_LANE:
    //                 parseCalibratorAttributes(attrs);
    //                 break;
    //             // flow (calibrator)
    //             case SUMO_TAG_FLOW:
    //                 parseCalibratorFlowAttributes(attrs);
    //                 break;
    //             // Rerouter
    //             case SUMO_TAG_REROUTER:
    //                 parseRerouterAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_INTERVAL:
    //                 parseRerouterIntervalAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_CLOSING_LANE_REROUTE:
    //                 parseClosingLaneRerouteAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_CLOSING_REROUTE:
    //                 parseClosingRerouteAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_DEST_PROB_REROUTE:
    //                 parseDestProbRerouteAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_PARKING_AREA_REROUTE:
    //                 parseParkingAreaRerouteAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_ROUTE_PROB_REROUTE:
    //                 parseRouteProbRerouteAttributes(attrs);
    //                 break;
    //             // Route probe
    //             case SUMO_TAG_ROUTEPROBE:
    //                 parseRouteProbeAttributes(attrs);
    //                 break;
    //             // Vaporizer (deprecated)
    //             case SUMO_TAG_VAPORIZER:
    //                 parseVaporizerAttributes(attrs);
    //                 break;
    //             // wires
    //             case SUMO_TAG_TRACTION_SUBSTATION:
    //                 parseTractionSubstation(attrs);
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_CLAMP:
    //                 parseOverheadWireClamp(attrs);
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_SECTION:
    //                 parseOverheadWire(attrs);
    //                 break;
    //             // Poly
    //             case SUMO_TAG_POLY:
    //                 parsePolyAttributes(attrs);
    //                 break;
    //             case SUMO_TAG_POI:
    //                 parsePOIAttributes(attrs);
    //                 break;
    //             // JuPedSim
    //             case GNE_TAG_JPS_WALKABLEAREA:
    //                 parseJpsWalkableAreaAttributes(attrs);
    //                 break;
    //             case GNE_TAG_JPS_OBSTACLE:
    //                 parseJpsObstacleAttributes(attrs);
    //                 break;
    //             // parameters
    //             case SUMO_TAG_PARAM:
    //                 parseParameters(attrs);
    //                 break;
    //             default:
    //                 // tag cannot be parsed in AdditionalHandler
    //                 myCommonXMLStructure.abortSUMOBaseOBject();
    //                 return false;
    //         }
    //     } catch (InvalidArgument& e) {
    //         writeError(e.what());
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn begin_parse_attributes(&mut self, tag: SumoXMLTag, attrs: &SUMOSAXAttributes) -> bool {
        todo!("port AdditionalHandler::beginParseAttributes")
    }

    /// C++: `void AdditionalHandler::endParseAttributes()`
    /// end parse attributes
    // void
    // AdditionalHandler::endParseAttributes() {
    //     // get last inserted object
    //     CommonXMLStructure::SumoBaseObject* obj = myCommonXMLStructure.getCurrentSumoBaseObject();
    //     if (obj) {
    //         // close SUMOBaseOBject
    //         myCommonXMLStructure.closeSUMOBaseOBject();
    //         // check tag
    //         switch (obj->getTag()) {
    //             // Stopping Places
    //             case SUMO_TAG_BUS_STOP:
    //             case SUMO_TAG_TRAIN_STOP:
    //             case SUMO_TAG_CONTAINER_STOP:
    //             case SUMO_TAG_CHARGING_STATION:
    //             case SUMO_TAG_PARKING_AREA:
    //             // detectors
    //             case SUMO_TAG_E1DETECTOR:
    //             case SUMO_TAG_INDUCTION_LOOP:
    //             case SUMO_TAG_E2DETECTOR:
    //             case SUMO_TAG_LANE_AREA_DETECTOR:
    //             case SUMO_TAG_E3DETECTOR:
    //             case SUMO_TAG_ENTRY_EXIT_DETECTOR:
    //             case SUMO_TAG_INSTANT_INDUCTION_LOOP:
    //             // TAZs
    //             case SUMO_TAG_TAZ:
    //             // Variable Speed Sign
    //             case SUMO_TAG_VSS:
    //             // Calibrator
    //             case SUMO_TAG_CALIBRATOR:
    //             case GNE_TAG_CALIBRATOR_LANE:
    //             // Rerouter
    //             case SUMO_TAG_REROUTER:
    //             // Route probe
    //             case SUMO_TAG_ROUTEPROBE:
    //             // Vaporizer (deprecated)
    //             case SUMO_TAG_VAPORIZER:
    //             // wires
    //             case SUMO_TAG_TRACTION_SUBSTATION:
    //             case SUMO_TAG_OVERHEAD_WIRE_SECTION:
    //             case SUMO_TAG_OVERHEAD_WIRE_CLAMP:
    //             // Shapes
    //             case SUMO_TAG_POLY:
    //             case SUMO_TAG_POI:
    //             // JuPedSim
    //             case GNE_TAG_JPS_WALKABLEAREA:
    //             case GNE_TAG_JPS_OBSTACLE:
    //                 // parse object and all their childrens
    //                 parseSumoBaseObject(obj);
    //                 // delete object (and all of their childrens)
    //                 delete obj;
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn end_parse_attributes(&mut self) {
        todo!("port AdditionalHandler::endParseAttributes")
    }

    /// C++: `void AdditionalHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject *)`
    /// parse SumoBaseObject (it's called recursivelly)
    // void
    // AdditionalHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject* obj) {
    //     // check if loading was aborted
    //     if (!myAbortLoading) {
    //         // switch tag
    //         switch (obj->getTag()) {
    //             // Stopping Places
    //             case SUMO_TAG_BUS_STOP:
    //                 if (buildBusStop(obj,
    //                                  obj->getStringAttribute(SUMO_ATTR_ID),
    //                                  obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_STARTPOS),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_ENDPOS),
    //                                  obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                  obj->getStringListAttribute(SUMO_ATTR_LINES),
    //                                  obj->getIntAttribute(SUMO_ATTR_PERSON_CAPACITY),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_PARKING_LENGTH),
    //                                  obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                  obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                  obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_TRAIN_STOP:
    //                 if (buildTrainStop(obj,
    //                                    obj->getStringAttribute(SUMO_ATTR_ID),
    //                                    obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                    obj->getDoubleAttribute(SUMO_ATTR_STARTPOS),
    //                                    obj->getDoubleAttribute(SUMO_ATTR_ENDPOS),
    //                                    obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                    obj->getStringListAttribute(SUMO_ATTR_LINES),
    //                                    obj->getIntAttribute(SUMO_ATTR_PERSON_CAPACITY),
    //                                    obj->getDoubleAttribute(SUMO_ATTR_PARKING_LENGTH),
    //                                    obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                    obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                    obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                    obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_ACCESS:
    //                 if (buildAccess(obj,
    //                                 obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                 obj->getStringAttribute(SUMO_ATTR_POSITION),
    //                                 obj->getDoubleAttribute(SUMO_ATTR_LENGTH),
    //                                 obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                 obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_CONTAINER_STOP:
    //                 if (buildContainerStop(obj,
    //                                        obj->getStringAttribute(SUMO_ATTR_ID),
    //                                        obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                        obj->getDoubleAttribute(SUMO_ATTR_STARTPOS),
    //                                        obj->getDoubleAttribute(SUMO_ATTR_ENDPOS),
    //                                        obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                        obj->getStringListAttribute(SUMO_ATTR_LINES),
    //                                        obj->getIntAttribute(SUMO_ATTR_CONTAINER_CAPACITY),
    //                                        obj->getDoubleAttribute(SUMO_ATTR_PARKING_LENGTH),
    //                                        obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                        obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                        obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                        obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_CHARGING_STATION:
    //                 if (buildChargingStation(obj,
    //                                          obj->getStringAttribute(SUMO_ATTR_ID),
    //                                          obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                          obj->getDoubleAttribute(SUMO_ATTR_STARTPOS),
    //                                          obj->getDoubleAttribute(SUMO_ATTR_ENDPOS),
    //                                          obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                          obj->getDoubleAttribute(SUMO_ATTR_CHARGINGPOWER),
    //                                          obj->getDoubleAttribute(SUMO_ATTR_TOTALPOWER),
    //                                          obj->getDoubleAttribute(SUMO_ATTR_EFFICIENCY),
    //                                          obj->getBoolAttribute(SUMO_ATTR_CHARGEINTRANSIT),
    //                                          obj->getTimeAttribute(SUMO_ATTR_CHARGEDELAY),
    //                                          obj->getStringAttribute(SUMO_ATTR_CHARGETYPE),
    //                                          obj->getTimeAttribute(SUMO_ATTR_WAITINGTIME),
    //                                          obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                          obj->getStringAttribute(SUMO_ATTR_PARKING_AREA),
    //                                          obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_PARKING_AREA:
    //                 if (buildParkingArea(obj,
    //                                      obj->getStringAttribute(SUMO_ATTR_ID),
    //                                      obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_STARTPOS),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_ENDPOS),
    //                                      obj->getStringAttribute(SUMO_ATTR_DEPARTPOS),
    //                                      obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                      obj->getStringListAttribute(SUMO_ATTR_ACCEPTED_BADGES),
    //                                      obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                      obj->getIntAttribute(SUMO_ATTR_ROADSIDE_CAPACITY),
    //                                      obj->getBoolAttribute(SUMO_ATTR_ONROAD),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_WIDTH),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_LENGTH),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                      obj->getBoolAttribute(SUMO_ATTR_LEFTHAND),
    //                                      obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_PARKING_SPACE:
    //                 if (buildParkingSpace(obj,
    //                                       obj->getDoubleAttribute(SUMO_ATTR_X),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_Y),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_Z),
    //                                       obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                       obj->getStringAttribute(SUMO_ATTR_WIDTH),
    //                                       obj->getStringAttribute(SUMO_ATTR_LENGTH),
    //                                       obj->getStringAttribute(SUMO_ATTR_ANGLE),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_SLOPE),
    //                                       obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // Detectors
    //             case SUMO_TAG_E1DETECTOR:
    //             case SUMO_TAG_INDUCTION_LOOP:
    //                 // build E1
    //                 if (buildE1Detector(obj,
    //                                     obj->getStringAttribute(SUMO_ATTR_ID),
    //                                     obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                     obj->getPeriodAttribute(),
    //                                     obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                     obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                     obj->getStringListAttribute(SUMO_ATTR_NEXT_EDGES),
    //                                     obj->getStringAttribute(SUMO_ATTR_DETECT_PERSONS),
    //                                     obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                     obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                     obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_E2DETECTOR:
    //             case SUMO_TAG_LANE_AREA_DETECTOR:
    //                 if (obj->hasStringAttribute(SUMO_ATTR_LANE)) {
    //                     if (buildSingleLaneDetectorE2(obj,
    //                                                   obj->getStringAttribute(SUMO_ATTR_ID),
    //                                                   obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                                   obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                                   obj->getDoubleAttribute(SUMO_ATTR_LENGTH),
    //                                                   obj->getPeriodAttribute(),
    //                                                   obj->getStringAttribute(SUMO_ATTR_TLID),
    //                                                   obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                                   obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                                   obj->getStringListAttribute(SUMO_ATTR_NEXT_EDGES),
    //                                                   obj->getStringAttribute(SUMO_ATTR_DETECT_PERSONS),
    //                                                   obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                                   obj->getTimeAttribute(SUMO_ATTR_HALTING_TIME_THRESHOLD),
    //                                                   obj->getDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD),
    //                                                   obj->getDoubleAttribute(SUMO_ATTR_JAM_DIST_THRESHOLD),
    //                                                   obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                                   obj->getBoolAttribute(SUMO_ATTR_SHOW_DETECTOR),
    //                                                   obj->getParameters())) {
    //                         obj->markAsCreated();
    //                     }
    //                 } else {
    //                     if (buildMultiLaneDetectorE2(obj,
    //                                                  obj->getStringAttribute(SUMO_ATTR_ID),
    //                                                  obj->getStringListAttribute(SUMO_ATTR_LANES),
    //                                                  obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                                  obj->getDoubleAttribute(SUMO_ATTR_ENDPOS),
    //                                                  obj->getPeriodAttribute(),
    //                                                  obj->getStringAttribute(SUMO_ATTR_TLID),
    //                                                  obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                                  obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                                  obj->getStringListAttribute(SUMO_ATTR_NEXT_EDGES),
    //                                                  obj->getStringAttribute(SUMO_ATTR_DETECT_PERSONS),
    //                                                  obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                                  obj->getTimeAttribute(SUMO_ATTR_HALTING_TIME_THRESHOLD),
    //                                                  obj->getDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD),
    //                                                  obj->getDoubleAttribute(SUMO_ATTR_JAM_DIST_THRESHOLD),
    //                                                  obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                                  obj->getBoolAttribute(SUMO_ATTR_SHOW_DETECTOR),
    //                                                  obj->getParameters())) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             case SUMO_TAG_E3DETECTOR:
    //             case SUMO_TAG_ENTRY_EXIT_DETECTOR:
    //                 if (buildDetectorE3(obj,
    //                                     obj->getStringAttribute(SUMO_ATTR_ID),
    //                                     obj->getPositionAttribute(SUMO_ATTR_POSITION),
    //                                     obj->getPeriodAttribute(),
    //                                     obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                     obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                     obj->getStringListAttribute(SUMO_ATTR_NEXT_EDGES),
    //                                     obj->getStringAttribute(SUMO_ATTR_DETECT_PERSONS),
    //                                     obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                     obj->getTimeAttribute(SUMO_ATTR_HALTING_TIME_THRESHOLD),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD),
    //                                     obj->getBoolAttribute(SUMO_ATTR_OPEN_ENTRY),
    //                                     obj->getBoolAttribute(SUMO_ATTR_EXPECT_ARRIVAL),
    //                                     obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_DET_ENTRY:
    //                 if (buildDetectorEntry(obj,
    //                                        obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                        obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                        obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                        obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_DET_EXIT:
    //                 if (buildDetectorExit(obj,
    //                                       obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                       obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                       obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_INSTANT_INDUCTION_LOOP:
    //                 if (buildDetectorE1Instant(obj,
    //                                            obj->getStringAttribute(SUMO_ATTR_ID),
    //                                            obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                            obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                            obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                            obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                            obj->getStringListAttribute(SUMO_ATTR_NEXT_EDGES),
    //                                            obj->getStringAttribute(SUMO_ATTR_DETECT_PERSONS),
    //                                            obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                            obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                            obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // TAZs
    //             case SUMO_TAG_TAZ:
    //                 if (buildTAZ(obj,
    //                              obj->getStringAttribute(SUMO_ATTR_ID),
    //                              obj->getPositionVectorAttribute(SUMO_ATTR_SHAPE),
    //                              obj->getPositionAttribute(SUMO_ATTR_CENTER),
    //                              obj->getBoolAttribute(SUMO_ATTR_FILL),
    //                              obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                              obj->getStringListAttribute(SUMO_ATTR_EDGES),
    //                              obj->getStringAttribute(SUMO_ATTR_NAME),
    //                              obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_TAZSOURCE:
    //                 if (buildTAZSource(obj,
    //                                    obj->getStringAttribute(SUMO_ATTR_ID),
    //                                    obj->getDoubleAttribute(SUMO_ATTR_WEIGHT))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_TAZSINK:
    //                 if (buildTAZSink(obj,
    //                                  obj->getStringAttribute(SUMO_ATTR_ID),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_WEIGHT))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // Variable Speed Sign
    //             case SUMO_TAG_VSS:
    //                 if (buildVariableSpeedSign(obj,
    //                                            obj->getStringAttribute(SUMO_ATTR_ID),
    //                                            obj->getPositionAttribute(SUMO_ATTR_POSITION),
    //                                            obj->getStringListAttribute(SUMO_ATTR_LANES),
    //                                            obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                            obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                            obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_STEP:
    //                 if (buildVariableSpeedSignStep(obj,
    //                                                obj->getTimeAttribute(SUMO_ATTR_TIME),
    //                                                obj->getDoubleAttribute(SUMO_ATTR_SPEED))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // Calibrator
    //             case SUMO_TAG_CALIBRATOR:
    //                 if (buildEdgeCalibrator(obj,
    //                                         obj->getStringAttribute(SUMO_ATTR_ID),
    //                                         obj->getStringAttribute(SUMO_ATTR_EDGE),
    //                                         obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                         obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                         obj->getStringAttribute(SUMO_ATTR_OUTPUT),
    //                                         obj->getPeriodAttribute(),
    //                                         obj->getStringAttribute(SUMO_ATTR_ROUTEPROBE),
    //                                         obj->getDoubleAttribute(SUMO_ATTR_JAM_DIST_THRESHOLD),
    //                                         obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                         obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case GNE_TAG_CALIBRATOR_LANE:
    //                 if (buildLaneCalibrator(obj,
    //                                         obj->getStringAttribute(SUMO_ATTR_ID),
    //                                         obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                         obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                         obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                         obj->getStringAttribute(SUMO_ATTR_OUTPUT),
    //                                         obj->getPeriodAttribute(),
    //                                         obj->getStringAttribute(SUMO_ATTR_ROUTEPROBE),
    //                                         obj->getDoubleAttribute(SUMO_ATTR_JAM_DIST_THRESHOLD),
    //                                         obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                         obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_FLOW:
    //                 if (buildCalibratorFlow(obj,
    //                                         obj->getVehicleParameter())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // Rerouter
    //             case SUMO_TAG_REROUTER:
    //                 if (buildRerouter(obj,
    //                                   obj->getStringAttribute(SUMO_ATTR_ID),
    //                                   obj->getPositionAttribute(SUMO_ATTR_POSITION),
    //                                   obj->getStringListAttribute(SUMO_ATTR_EDGES),
    //                                   obj->getDoubleAttribute(SUMO_ATTR_PROB),
    //                                   obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                   obj->getBoolAttribute(SUMO_ATTR_OFF),
    //                                   obj->getBoolAttribute(SUMO_ATTR_OPTIONAL),
    //                                   obj->getTimeAttribute(SUMO_ATTR_HALTING_TIME_THRESHOLD),
    //                                   obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                   obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_CLOSING_LANE_REROUTE:
    //                 if (buildClosingLaneReroute(obj,
    //                                             obj->getStringAttribute(SUMO_ATTR_ID),
    //                                             parseVehicleClasses(obj->getStringAttribute(SUMO_ATTR_ALLOW),
    //                                                     obj->getStringAttribute(SUMO_ATTR_DISALLOW)))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_CLOSING_REROUTE:
    //                 if (buildClosingReroute(obj,
    //                                         obj->getStringAttribute(SUMO_ATTR_ID),
    //                                         parseVehicleClasses(obj->getStringAttribute(SUMO_ATTR_ALLOW),
    //                                                 obj->getStringAttribute(SUMO_ATTR_DISALLOW)))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_DEST_PROB_REROUTE:
    //                 if (buildDestProbReroute(obj,
    //                                          obj->getStringAttribute(SUMO_ATTR_ID),
    //                                          obj->getDoubleAttribute(SUMO_ATTR_PROB))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_PARKING_AREA_REROUTE:
    //                 if (buildParkingAreaReroute(obj,
    //                                             obj->getStringAttribute(SUMO_ATTR_ID),
    //                                             obj->getDoubleAttribute(SUMO_ATTR_PROB),
    //                                             obj->getBoolAttribute(SUMO_ATTR_VISIBLE))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_ROUTE_PROB_REROUTE:
    //                 if (buildRouteProbReroute(obj,
    //                                           obj->getStringAttribute(SUMO_ATTR_ID),
    //                                           obj->getDoubleAttribute(SUMO_ATTR_PROB))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_INTERVAL:
    //                 // check if is aREROUTER interval
    //                 if (obj->getParentSumoBaseObject()->getTag() == SUMO_TAG_REROUTER) {
    //                     if (buildRerouterInterval(obj,
    //                                               obj->getTimeAttribute(SUMO_ATTR_BEGIN),
    //                                               obj->getTimeAttribute(SUMO_ATTR_END))) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             // Route probe
    //             case SUMO_TAG_ROUTEPROBE:
    //                 if (buildRouteProbe(obj,
    //                                     obj->getStringAttribute(SUMO_ATTR_ID),
    //                                     obj->getStringAttribute(SUMO_ATTR_EDGE),
    //                                     obj->getPeriodAttribute(),
    //                                     obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                     obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                     obj->getTimeAttribute(SUMO_ATTR_BEGIN),
    //                                     obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                     obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // Vaporizer (deprecated)
    //             case SUMO_TAG_VAPORIZER:
    //                 if (buildVaporizer(obj,
    //                                    obj->getStringAttribute(SUMO_ATTR_EDGE),
    //                                    obj->getTimeAttribute(SUMO_ATTR_BEGIN),
    //                                    obj->getTimeAttribute(SUMO_ATTR_END),
    //                                    obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                    obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // wire elements
    //             case SUMO_TAG_TRACTION_SUBSTATION:
    //                 if (buildTractionSubstation(obj,
    //                                             obj->getStringAttribute(SUMO_ATTR_ID),
    //                                             obj->getPositionAttribute(SUMO_ATTR_POSITION),
    //                                             obj->getDoubleAttribute(SUMO_ATTR_VOLTAGE),
    //                                             obj->getDoubleAttribute(SUMO_ATTR_CURRENTLIMIT),
    //                                             obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_SECTION:
    //                 if (buildOverheadWire(obj,
    //                                       obj->getStringAttribute(SUMO_ATTR_ID),
    //                                       obj->getStringAttribute(SUMO_ATTR_SUBSTATIONID),
    //                                       obj->getStringListAttribute(SUMO_ATTR_LANES),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_STARTPOS),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_ENDPOS),
    //                                       obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                       obj->getStringListAttribute(SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN),
    //                                       obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_CLAMP:
    //                 if (buildOverheadWireClamp(obj,
    //                                            obj->getStringAttribute(SUMO_ATTR_ID),
    //                                            obj->getStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_START),
    //                                            obj->getStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANESTART),
    //                                            obj->getStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_END),
    //                                            obj->getStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANEEND),
    //                                            obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // Polygon
    //             case SUMO_TAG_POLY:
    //                 if (buildPolygon(obj,
    //                                  obj->getStringAttribute(SUMO_ATTR_ID),
    //                                  obj->getStringAttribute(SUMO_ATTR_TYPE),
    //                                  obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_LAYER),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                  obj->getStringAttribute(SUMO_ATTR_IMGFILE),
    //                                  obj->getPositionVectorAttribute(SUMO_ATTR_SHAPE),
    //                                  obj->getBoolAttribute(SUMO_ATTR_GEO),
    //                                  obj->getBoolAttribute(SUMO_ATTR_FILL),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_LINEWIDTH),
    //                                  obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_HEIGHT),
    //                                  obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // POI
    //             case SUMO_TAG_POI:
    //                 // check if we want to create a POI, POILane or POIGEO
    //                 if (obj->hasDoubleAttribute(SUMO_ATTR_X)) {
    //                     // build POI over view
    //                     if (buildPOI(obj,
    //                                  obj->getStringAttribute(SUMO_ATTR_ID),
    //                                  obj->getStringAttribute(SUMO_ATTR_TYPE),
    //                                  obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_X),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_Y),
    //                                  obj->getStringAttribute(SUMO_ATTR_ICON),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_LAYER),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                  obj->getStringAttribute(SUMO_ATTR_IMGFILE),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_WIDTH),
    //                                  obj->getDoubleAttribute(SUMO_ATTR_HEIGHT),
    //                                  obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                  obj->getParameters())) {
    //                         obj->markAsCreated();
    //                     }
    //                 } else if (obj->hasStringAttribute(SUMO_ATTR_LANE)) {
    //                     // build POI over Lane
    //                     if (buildPOILane(obj,
    //                                      obj->getStringAttribute(SUMO_ATTR_ID),
    //                                      obj->getStringAttribute(SUMO_ATTR_TYPE),
    //                                      obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                      obj->getStringAttribute(SUMO_ATTR_LANE),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_POSITION),
    //                                      obj->getBoolAttribute(SUMO_ATTR_FRIENDLY_POS),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_POSITION_LAT),
    //                                      obj->getStringAttribute(SUMO_ATTR_ICON),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_LAYER),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                      obj->getStringAttribute(SUMO_ATTR_IMGFILE),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_WIDTH),
    //                                      obj->getDoubleAttribute(SUMO_ATTR_HEIGHT),
    //                                      obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                      obj->getParameters())) {
    //                         obj->markAsCreated();
    //                     }
    //                 } else {
    //                     // build POIGEO over view
    //                     if (buildPOIGeo(obj,
    //                                     obj->getStringAttribute(SUMO_ATTR_ID),
    //                                     obj->getStringAttribute(SUMO_ATTR_TYPE),
    //                                     obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_LON),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_LAT),
    //                                     obj->getStringAttribute(SUMO_ATTR_ICON),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_LAYER),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_ANGLE),
    //                                     obj->getStringAttribute(SUMO_ATTR_IMGFILE),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_WIDTH),
    //                                     obj->getDoubleAttribute(SUMO_ATTR_HEIGHT),
    //                                     obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                     obj->getParameters())) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             // Jps WalkableArea
    //             case GNE_TAG_JPS_WALKABLEAREA:
    //                 if (buildJpsWalkableArea(obj,
    //                                          obj->getStringAttribute(SUMO_ATTR_ID),
    //                                          obj->getPositionVectorAttribute(SUMO_ATTR_SHAPE),
    //                                          obj->getBoolAttribute(SUMO_ATTR_GEO),
    //                                          obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                          obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // Jps Obstacle
    //             case GNE_TAG_JPS_OBSTACLE:
    //                 if (buildJpsObstacle(obj,
    //                                      obj->getStringAttribute(SUMO_ATTR_ID),
    //                                      obj->getPositionVectorAttribute(SUMO_ATTR_SHAPE),
    //                                      obj->getBoolAttribute(SUMO_ATTR_GEO),
    //                                      obj->getStringAttribute(SUMO_ATTR_NAME),
    //                                      obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             default:
    //                 break;
    //         }
    //         // now iterate over childrens
    //         for (const auto& child : obj->getSumoBaseObjectChildren()) {
    //             // call this function recursively
    //             parseSumoBaseObject(child);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_sumo_base_object(&mut self, obj: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>) {
        todo!("port AdditionalHandler::parseSumoBaseObject")
    }

    /// C++: `bool AdditionalHandler::buildBusStop(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::vector<std::string> &, const int, const double, const RGBColor &, const bool, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a bus stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_bus_stop(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, lines: &Vec<String>, personCapacity: i32, parkingLength: f64, color: &RGBColor, friendlyPosition: bool, angle: f64, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildBusStop is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTrainStop(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::vector<std::string> &, const int, const double, const RGBColor &, const bool, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a train stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_train_stop(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, lines: &Vec<String>, personCapacity: i32, parkingLength: f64, color: &RGBColor, friendlyPosition: bool, angle: f64, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildTrainStop is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildAccess(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds an Access
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_access(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, laneID: &str, pos: &str, length: f64, friendlyPos: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildAccess is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildContainerStop(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::vector<std::string> &, const int, const double, const RGBColor &, const bool, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a container stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_container_stop(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, lines: &Vec<String>, containerCapacity: i32, parkingLength: f64, color: &RGBColor, friendlyPosition: bool, angle: f64, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildContainerStop is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildChargingStation(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const double, const double, const double, const bool, const SUMOTime, const std::string &, const SUMOTime, const bool, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a charging Station
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_charging_station(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, chargingPower: f64, totalPower: f64, efficiency: f64, chargeInTransit: bool, chargeDelay: SUMOTime, chargeType: &str, waitingTime: SUMOTime, friendlyPosition: bool, parkingAreaID: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildChargingStation is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildParkingArea(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::string &, const std::vector<std::string> &, const bool, const int, const bool, const double, const double, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a Parking Area
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_parking_area(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, startPos: f64, endPos: f64, departPos: &str, name: &str, lines: &Vec<String>, friendlyPosition: bool, roadSideCapacity: i32, onRoad: bool, width: f64, length: f64, angle: f64, lefthand: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildParkingArea is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildParkingSpace(const CommonXMLStructure::SumoBaseObject *, const double, const double, const double, const std::string &, const std::string &, const std::string &, const std::string &, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a Parking Space
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_parking_space(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, x: f64, y: f64, z: f64, name: &str, width: &str, length: &str, angle: &str, slope: f64, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildParkingSpace is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildE1Detector(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const SUMOTime, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a induction loop detector (E1)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_e1_detector(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, position: f64, period: SUMOTime, file: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, friendlyPos: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildE1Detector is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildSingleLaneDetectorE2(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const SUMOTime, const std::string &, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const SUMOTime, const double, const double, const bool, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a single-lane Area Detector (E2)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_single_lane_detector_e2(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, pos: f64, length: f64, period: SUMOTime, trafficLight: &str, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, timeThreshold: SUMOTime, speedThreshold: f64, jamThreshold: f64, friendlyPos: bool, show: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildSingleLaneDetectorE2 is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildMultiLaneDetectorE2(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::vector<std::string> &, const double, const double, const SUMOTime, const std::string &, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const SUMOTime, const double, const double, const bool, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a multi-lane Area Detector (E2)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_multi_lane_detector_e2(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneIDs: &Vec<String>, pos: f64, endPos: f64, period: SUMOTime, trafficLight: &str, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, timeThreshold: SUMOTime, speedThreshold: f64, jamThreshold: f64, friendlyPos: bool, show: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildMultiLaneDetectorE2 is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorE3(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const SUMOTime, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const SUMOTime, const double, const bool, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a multi entry exit detector (E3)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_e3(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, pos: &Position, period: SUMOTime, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, timeThreshold: SUMOTime, speedThreshold: f64, openEntry: bool, expectedArrival: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildDetectorE3 is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorEntry(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a entry detector (E3)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_entry(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, laneID: &str, pos: f64, friendlyPos: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildDetectorEntry is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorExit(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a exit detector (E3)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_exit(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, laneID: &str, pos: f64, friendlyPos: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildDetectorExit is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorE1Instant(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a Instant Induction Loop Detector (E1Instant)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_e1_instant(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, pos: f64, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, friendlyPos: bool, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildDetectorE1Instant is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildLaneCalibrator(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const std::string &, const std::string &, const SUMOTime, const std::string &, const double, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a microscopic calibrator over a lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_lane_calibrator(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, laneID: &str, pos: f64, name: &str, outfile: &str, period: SUMOTime, routeprobe: &str, jamThreshold: f64, vTypes: &Vec<String>, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildLaneCalibrator is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildEdgeCalibrator(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const std::string &, const std::string &, const SUMOTime, const std::string &, const double, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a microscopic calibrator over an edge
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge_calibrator(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, edgeID: &str, pos: f64, name: &str, outfile: &str, period: SUMOTime, routeprobe: &str, jamThreshold: f64, vTypes: &Vec<String>, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildEdgeCalibrator is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildCalibratorFlow(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// builds a calibrator flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_calibrator_flow(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, vehicleParameter: &SUMOVehicleParameter) -> bool {
        panic!("AdditionalHandler::buildCalibratorFlow is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRerouter(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const std::vector<std::string> &, const double, const std::string &, const bool, const bool, const SUMOTime, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a rerouter
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_rerouter(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, pos: &Position, edgeIDs: &Vec<String>, prob: f64, name: &str, off: bool, optional: bool, timeThreshold: SUMOTime, vTypes: &Vec<String>, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildRerouter is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRerouterInterval(const CommonXMLStructure::SumoBaseObject *, const SUMOTime, const SUMOTime)` [virtual, pure virtual]
    /// builds a rerouter interval
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_rerouter_interval(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, begin: SUMOTime, end: SUMOTime) -> bool {
        panic!("AdditionalHandler::buildRerouterInterval is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildClosingLaneReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, SVCPermissions)` [virtual, pure virtual]
    /// builds a closing lane reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_closing_lane_reroute(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, closedLane: &str, permissions: SVCPermissions) -> bool {
        panic!("AdditionalHandler::buildClosingLaneReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildClosingReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, SVCPermissions)` [virtual, pure virtual]
    /// builds a closing edge reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_closing_reroute(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, closedEdgeID: &str, permissions: SVCPermissions) -> bool {
        panic!("AdditionalHandler::buildClosingReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDestProbReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// builds a dest prob reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_dest_prob_reroute(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, newEdgeDestinationID: &str, probability: f64) -> bool {
        panic!("AdditionalHandler::buildDestProbReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildParkingAreaReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const bool)` [virtual, pure virtual]
    /// builds a parking area reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_parking_area_reroute(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, newParkignAreaID: &str, probability: f64, visible: bool) -> bool {
        panic!("AdditionalHandler::buildParkingAreaReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRouteProbReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// builds a route prob reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_route_prob_reroute(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, newRouteID: &str, probability: f64) -> bool {
        panic!("AdditionalHandler::buildRouteProbReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRouteProbe(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const SUMOTime, const std::string &, const std::string &, const SUMOTime, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a Route probe
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_route_probe(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, edgeID: &str, period: SUMOTime, name: &str, file: &str, begin: SUMOTime, vTypes: &Vec<String>, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildRouteProbe is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildVariableSpeedSign(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const std::vector<std::string> &, const std::string &, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a VariableSpeedSign (lane speed additional)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_variable_speed_sign(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, pos: &Position, laneIDs: &Vec<String>, name: &str, vTypes: &Vec<String>, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildVariableSpeedSign is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildVariableSpeedSignStep(const CommonXMLStructure::SumoBaseObject *, const SUMOTime, const double)` [virtual, pure virtual]
    /// Builds a VariableSpeedSign Step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_variable_speed_sign_step(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, time: SUMOTime, speed: f64) -> bool {
        panic!("AdditionalHandler::buildVariableSpeedSignStep is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildVaporizer(const CommonXMLStructure::SumoBaseObject *, const std::string &, const SUMOTime, const SUMOTime, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a vaporizer (lane speed additional)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_vaporizer(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, edgeID: &str, from: SUMOTime, endTime: SUMOTime, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildVaporizer is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTAZ(const CommonXMLStructure::SumoBaseObject *, const std::string &, const PositionVector &, const Position &, const bool, const RGBColor &, const std::vector<std::string> &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a TAZ (Traffic Assignment Zone)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, shape: &PositionVector, center: &Position, fill: bool, color: &RGBColor, edgeIDs: &Vec<String>, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildTAZ is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTAZSource(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// Builds a TAZSource (Traffic Assignment Zone)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz_source(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, edgeID: &str, departWeight: f64) -> bool {
        panic!("AdditionalHandler::buildTAZSource is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTAZSink(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// Builds a TAZSink (Traffic Assignment Zone)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz_sink(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, edgeID: &str, arrivalWeight: f64) -> bool {
        panic!("AdditionalHandler::buildTAZSink is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTractionSubstation(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const double, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// build traction substation
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_traction_substation(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, pos: &Position, voltage: f64, currentLimit: f64, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildTractionSubstation is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildOverheadWire(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const std::vector<std::string> &, const double, const double, const bool, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// build overhead wire
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_overhead_wire(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, substationId: &str, laneIDs: &Vec<String>, startPos: f64, endPos: f64, friendlyPos: bool, forbiddenInnerLanes: &Vec<String>, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildOverheadWire is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildOverheadWireClamp(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// build overhead wire clamp
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_overhead_wire_clamp(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, overheadWireIDStartClamp: &str, laneIDStartClamp: &str, overheadWireIDEndClamp: &str, laneIDEndClamp: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildOverheadWireClamp is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPolygon(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const double, const double, const std::string &, const PositionVector &, const bool, const bool, const double, const std::string &, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a polygon using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_polygon(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, type_: &str, color: &RGBColor, layer: f64, angle: f64, imgFile: &str, shape: &PositionVector, geo: bool, fill: bool, lineWidth: f64, name: &str, height: f64, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildPolygon is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPOI(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const double, const double, const std::string &, const double, const double, const std::string &, const double, const double, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a POI using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_poi(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, type_: &str, color: &RGBColor, x: f64, y: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildPOI is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPOILane(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const std::string &, const double, const bool, const double, const std::string &, const double, const double, const std::string &, const double, const double, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a POI over lane using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_poi_lane(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, type_: &str, color: &RGBColor, laneID: &str, posOverLane: f64, friendlyPosition: bool, posLat: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildPOILane is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPOIGeo(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const double, const double, const std::string &, const double, const double, const std::string &, const double, const double, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a POI in GEO coordinaten using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_poi_geo(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, type_: &str, color: &RGBColor, lon: f64, lat: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildPOIGeo is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildJpsWalkableArea(const CommonXMLStructure::SumoBaseObject *, const std::string &, const PositionVector &, bool, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a JuPedSim walkable area using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_jps_walkable_area(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, shape: &PositionVector, geo: bool, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildJpsWalkableArea is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildJpsObstacle(const CommonXMLStructure::SumoBaseObject *, const std::string &, const PositionVector &, bool, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a JuPedSim obstacle using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_jps_obstacle(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, id: &str, shape: &PositionVector, geo: bool, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        panic!("AdditionalHandler::buildJpsObstacle is pure virtual in C++")
    }

    /// C++: `void AdditionalHandler::parseBusStopAttributes(const SUMOSAXAttributes &)` [private]
    /// parse busStop attributes
    // void
    // AdditionalHandler::parseBusStopAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const double startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const double endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const int personCapacity = attrs.getOpt<int>(SUMO_ATTR_PERSON_CAPACITY, id.c_str(), parsedOk, 6);
    //     const double parkingLength = attrs.getOpt<double>(SUMO_ATTR_PARKING_LENGTH, id.c_str(), parsedOk, 0);
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), parsedOk, RGBColor::INVISIBLE);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), parsedOk, 0);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_BUS_STOP);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_STARTPOS, startPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ENDPOS, endPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LINES, lines);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addIntAttribute(SUMO_ATTR_PERSON_CAPACITY, personCapacity);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PARKING_LENGTH, parkingLength);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ANGLE, angle);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_bus_stop_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseBusStopAttributes")
    }

    /// C++: `void AdditionalHandler::parseTrainStopAttributes(const SUMOSAXAttributes &)` [private]
    /// parse trainStop attributes
    // void
    // AdditionalHandler::parseTrainStopAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const double startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const double endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const int personCapacity = attrs.getOpt<int>(SUMO_ATTR_PERSON_CAPACITY, id.c_str(), parsedOk, 6);
    //     const double parkingLength = attrs.getOpt<double>(SUMO_ATTR_PARKING_LENGTH, id.c_str(), parsedOk, 0);
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), parsedOk, RGBColor::INVISIBLE);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), parsedOk, 0);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TRAIN_STOP);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_STARTPOS, startPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ENDPOS, endPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LINES, lines);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addIntAttribute(SUMO_ATTR_PERSON_CAPACITY, personCapacity);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PARKING_LENGTH, parkingLength);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ANGLE, angle);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_train_stop_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseTrainStopAttributes")
    }

    /// C++: `void AdditionalHandler::parseAccessAttributes(const SUMOSAXAttributes &)` [private]
    /// parse access attributes
    // void
    // AdditionalHandler::parseAccessAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, "", parsedOk);
    //     const std::string position = attrs.get<std::string>(SUMO_ATTR_POSITION, "", parsedOk);
    //     // optional attributes
    //     const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, "", parsedOk, -1.00, false); /* in future updates, INVALID_DOUBLE */
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, "", parsedOk, false);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_ACCESS, {SUMO_TAG_BUS_STOP, SUMO_TAG_TRAIN_STOP, SUMO_TAG_CONTAINER_STOP}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ACCESS);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_POSITION, position);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LENGTH, length);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_access_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseAccessAttributes")
    }

    /// C++: `void AdditionalHandler::parseContainerStopAttributes(const SUMOSAXAttributes &)` [private]
    /// parse containerStop attributes
    // void
    // AdditionalHandler::parseContainerStopAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const double startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const double endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const int containerCapacity = attrs.getOpt<int>(SUMO_ATTR_CONTAINER_CAPACITY, id.c_str(), parsedOk, 6);
    //     const double parkingLength = attrs.getOpt<double>(SUMO_ATTR_PARKING_LENGTH, id.c_str(), parsedOk, 0);
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), parsedOk, RGBColor::INVISIBLE);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), parsedOk, 0);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_CONTAINER_STOP);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_STARTPOS, startPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ENDPOS, endPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LINES, lines);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addIntAttribute(SUMO_ATTR_CONTAINER_CAPACITY, containerCapacity);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PARKING_LENGTH, parkingLength);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ANGLE, angle);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_container_stop_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseContainerStopAttributes")
    }

    /// C++: `void AdditionalHandler::parseChargingStationAttributes(const SUMOSAXAttributes &)` [private]
    /// parse chargingStation attributes
    // void
    // AdditionalHandler::parseChargingStationAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const double startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const double endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const double chargingPower = attrs.getOpt<double>(SUMO_ATTR_CHARGINGPOWER, id.c_str(), parsedOk, 22000);
    //     const double totalPower = attrs.getOpt<double>(SUMO_ATTR_TOTALPOWER, id.c_str(), parsedOk, 0);
    //     const double efficiency = attrs.getOpt<double>(SUMO_ATTR_EFFICIENCY, id.c_str(), parsedOk, 0.95);
    //     const bool chargeInTransit = attrs.getOpt<bool>(SUMO_ATTR_CHARGEINTRANSIT, id.c_str(), parsedOk, 0);
    //     const SUMOTime chargeDelay = attrs.getOptSUMOTimeReporting(SUMO_ATTR_CHARGEDELAY, id.c_str(), parsedOk, 0);
    //     const std::string chargeType = attrs.getOpt<std::string>(SUMO_ATTR_CHARGETYPE, id.c_str(), parsedOk, SUMOXMLDefinitions::ChargeTypes.getString(ChargeType::NORMAL));
    //     const SUMOTime waitingTime = attrs.getOptSUMOTimeReporting(SUMO_ATTR_WAITINGTIME, id.c_str(), parsedOk, TIME2STEPS(900));
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     const std::string parkingAreaID = attrs.getOpt<std::string>(SUMO_ATTR_PARKING_AREA, id.c_str(), parsedOk, "");
    //     // check charge type
    //     if ((chargeType != "normal") && (chargeType != "battery-exchange") && (chargeType != "fuel")) {
    //         writeError(TLF("Invalid charge type '%' defined in chargingStation '%'.", chargeType, id));
    //         parsedOk = false;
    //     }
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_CHARGING_STATION);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_STARTPOS, startPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ENDPOS, endPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LINES, lines);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_CHARGINGPOWER, chargingPower);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_TOTALPOWER, totalPower);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_EFFICIENCY, efficiency);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_CHARGEINTRANSIT, chargeInTransit);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_CHARGEDELAY, chargeDelay);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_CHARGETYPE, chargeType);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_WAITINGTIME, waitingTime);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_PARKING_AREA, parkingAreaID);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_charging_station_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseChargingStationAttributes")
    }

    /// C++: `void AdditionalHandler::parseParkingAreaAttributes(const SUMOSAXAttributes &)` [private]
    /// parse parking area attributes
    // void
    // AdditionalHandler::parseParkingAreaAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const double startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const double endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const std::string departPos = attrs.getOpt<std::string>(SUMO_ATTR_DEPARTPOS, id.c_str(), parsedOk, "");
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> badges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_ACCEPTED_BADGES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     const int roadSideCapacity = attrs.getOpt<int>(SUMO_ATTR_ROADSIDE_CAPACITY, id.c_str(), parsedOk, 0);
    //     const bool onRoad = attrs.getOpt<bool>(SUMO_ATTR_ONROAD, id.c_str(), parsedOk, false);
    //     const double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id.c_str(), parsedOk, 0);
    //     const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, id.c_str(), parsedOk, 0);
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), parsedOk, 0);
    //     const bool lefthand = attrs.getOpt<bool>(SUMO_ATTR_LEFTHAND, id.c_str(), parsedOk, false);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_PARKING_AREA);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_STARTPOS, startPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ENDPOS, endPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_DEPARTPOS, departPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_ACCEPTED_BADGES, badges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addIntAttribute(SUMO_ATTR_ROADSIDE_CAPACITY, roadSideCapacity);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_ONROAD, onRoad);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_WIDTH, width);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LENGTH, length);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ANGLE, angle);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_LEFTHAND, lefthand);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parking_area_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseParkingAreaAttributes")
    }

    /// C++: `void AdditionalHandler::parseParkingSpaceAttributes(const SUMOSAXAttributes &)` [private]
    /// parse parking space attributes
    // void
    // AdditionalHandler::parseParkingSpaceAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const double x = attrs.get<double>(SUMO_ATTR_X, "", parsedOk);
    //     const double y = attrs.get<double>(SUMO_ATTR_Y, "", parsedOk);
    //     // optional attributes
    //     const double z = attrs.getOpt<double>(SUMO_ATTR_Z, "", parsedOk, 0);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, "", parsedOk, "");
    //     const std::string width = attrs.getOpt<std::string>(SUMO_ATTR_WIDTH, "", parsedOk, "");
    //     const std::string length = attrs.getOpt<std::string>(SUMO_ATTR_LENGTH, "", parsedOk, "");
    //     const std::string angle = attrs.getOpt<std::string>(SUMO_ATTR_ANGLE, "", parsedOk, "");
    //     const double slope = attrs.getOpt<double>(SUMO_ATTR_SLOPE, "", parsedOk, 0);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_PARKING_SPACE, {SUMO_TAG_PARKING_AREA}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_PARKING_SPACE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_X, x);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_Y, y);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_Z, z);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_WIDTH, width);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LENGTH, length);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ANGLE, angle);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_SLOPE, slope);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parking_space_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseParkingSpaceAttributes")
    }

    /// C++: `void AdditionalHandler::parseE1Attributes(const SUMOSAXAttributes &)` [private]
    /// parse E1 attributes
    // void
    // AdditionalHandler::parseE1Attributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk);
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, id.c_str(), parsedOk);
    //     const SUMOTime period = attrs.getOptPeriod(id.c_str(), parsedOk, SUMOTime_MAX_PERIOD);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const std::vector<std::string> nextEdges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_NEXT_EDGES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const std::string detectPersons = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, "", parsedOk);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     // continue if flag is ok
    //     if (parsedOk && checkDetectPersons(SUMO_TAG_E1DETECTOR, id, detectPersons)) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_E1DETECTOR);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, position);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_PERIOD, period);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FILE, file);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_NEXT_EDGES, nextEdges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_DETECT_PERSONS, detectPersons);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_e1_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseE1Attributes")
    }

    /// C++: `void AdditionalHandler::parseE2Attributes(const SUMOSAXAttributes &)` [private]
    /// parse E2 attributes
    // void
    // AdditionalHandler::parseE2Attributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     const int positionDef = attrs.hasAttribute(SUMO_ATTR_POSITION) ? 1 : 0;
    //     const int endPosDef = attrs.hasAttribute(SUMO_ATTR_ENDPOS) ? 1 : 0;
    //     const int lengthDef = attrs.hasAttribute(SUMO_ATTR_LENGTH) ? 1 : 0;
    //     // check attributes
    //     if (attrs.hasAttribute(SUMO_ATTR_LANE) && ((positionDef + endPosDef + lengthDef) > 2)) {
    //         writeError(TL("'pos', 'endPos' and 'length' cannot be defined together in a single lane area detector."));
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     } else if (attrs.hasAttribute(SUMO_ATTR_LANE) && ((positionDef + endPosDef + lengthDef) < 2)) {
    //         writeError(TL("A single lane area detector requires two parameters of those 'pos', 'endPos' and 'length'."));
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     } else {
    //         // needed attributes
    //         const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //         const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), parsedOk);
    //         // special attributes
    //         const std::string laneId = attrs.getOpt<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk, "");
    //         const std::vector<std::string> laneIds = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LANES, id.c_str(), parsedOk, std::vector<std::string>());
    //         const double position = attrs.getOpt<double>(SUMO_ATTR_POSITION, id.c_str(), parsedOk, 0);
    //         const double endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), parsedOk, 0);
    //         const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, id.c_str(), parsedOk, 0);
    //         // optional attributes
    //         const SUMOTime period = attrs.getOptPeriod(id.c_str(), parsedOk, SUMOTime_MAX_PERIOD);
    //         const std::string trafficLight = attrs.getOpt<std::string>(SUMO_ATTR_TLID, id.c_str(), parsedOk, "");
    //         const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //         const SUMOTime haltingTimeThreshold = attrs.getOptSUMOTimeReporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, id.c_str(), parsedOk, TIME2STEPS(1));
    //         const double haltingSpeedThreshold = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, id.c_str(), parsedOk, 1.39);
    //         const double jamDistThreshold = attrs.getOpt<double>(SUMO_ATTR_JAM_DIST_THRESHOLD, id.c_str(), parsedOk, 10);
    //         const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //         const std::vector<std::string> nextEdges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_NEXT_EDGES, id.c_str(), parsedOk, std::vector<std::string>());
    //         const std::string detectPersons = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, "", parsedOk);
    //         const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //         const bool show = attrs.getOpt<bool>(SUMO_ATTR_SHOW_DETECTOR, id.c_str(), parsedOk, true);
    //         // continue if flag is ok
    //         if (parsedOk && checkDetectPersons(SUMO_TAG_E2DETECTOR, id, detectPersons)) {
    //             // set tag
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_E2DETECTOR);
    //             // add attributes depending of Lane/Lanes
    //             if (attrs.hasAttribute(SUMO_ATTR_LANE)) {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //                 if (positionDef == 0) {
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, endPos - length);
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LENGTH, length);
    //                 } else if (endPosDef == 0) {
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, position);
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LENGTH, length);
    //                 } else if (lengthDef == 0) {
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, position);
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LENGTH, endPos - position);
    //                 }
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LANES, laneIds);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, position);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ENDPOS, endPos);
    //             }
    //             // add all attributes
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_PERIOD, period);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TLID, trafficLight);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FILE, file);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_NEXT_EDGES, nextEdges);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_DETECT_PERSONS, detectPersons);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_HALTING_TIME_THRESHOLD, haltingTimeThreshold);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, haltingSpeedThreshold);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_JAM_DIST_THRESHOLD, jamDistThreshold);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_SHOW_DETECTOR, show);
    //         } else {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_e2_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseE2Attributes")
    }

    /// C++: `void AdditionalHandler::parseE3Attributes(const SUMOSAXAttributes &)` [private]
    /// parse E3 attributes
    // void
    // AdditionalHandler::parseE3Attributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), parsedOk);
    //     const SUMOTime period = attrs.getOptPeriod(id.c_str(), parsedOk, SUMOTime_MAX_PERIOD);
    //     // optional attributes
    //     const Position pos = attrs.getOpt<Position>(SUMO_ATTR_POSITION, id.c_str(), parsedOk, Position());
    //     const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const std::vector<std::string> nextEdges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_NEXT_EDGES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const std::string detectPersons = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, "", parsedOk);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const SUMOTime haltingTimeThreshold = attrs.getOptSUMOTimeReporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, id.c_str(), parsedOk, TIME2STEPS(1));
    //     const double haltingSpeedThreshold = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, id.c_str(), parsedOk, 1.39);
    //     const bool openEntry = attrs.getOpt<bool>(SUMO_ATTR_OPEN_ENTRY, id.c_str(), parsedOk, false);
    //     const bool expectedArrival = attrs.getOpt<bool>(SUMO_ATTR_EXPECT_ARRIVAL, id.c_str(), parsedOk, false);
    //     // continue if flag is ok
    //     if (parsedOk && checkDetectPersons(SUMO_TAG_E3DETECTOR, id, detectPersons)) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_E3DETECTOR);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FILE, file);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_PERIOD, period);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionAttribute(SUMO_ATTR_POSITION, pos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_NEXT_EDGES, nextEdges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_DETECT_PERSONS, detectPersons);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_HALTING_TIME_THRESHOLD, haltingTimeThreshold);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, haltingSpeedThreshold);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_OPEN_ENTRY, openEntry);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_EXPECT_ARRIVAL, expectedArrival);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_e3_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseE3Attributes")
    }

    /// C++: `void AdditionalHandler::parseEntryAttributes(const SUMOSAXAttributes &)` [private]
    /// parse entry attributes
    // void
    // AdditionalHandler::parseEntryAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, "", parsedOk);
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, "", parsedOk);
    //     // optional attributes
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, "", parsedOk, false);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_DET_ENTRY, {SUMO_TAG_E3DETECTOR}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_DET_ENTRY);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, position);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_entry_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseEntryAttributes")
    }

    /// C++: `void AdditionalHandler::parseExitAttributes(const SUMOSAXAttributes &)` [private]
    /// parse exist attributes
    // void
    // AdditionalHandler::parseExitAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, "", parsedOk);
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, "", parsedOk);
    //     // optional attributes
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, "", parsedOk, false);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_DET_EXIT, {SUMO_TAG_E3DETECTOR}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_DET_EXIT);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, position);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_exit_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseExitAttributes")
    }

    /// C++: `void AdditionalHandler::parseE1InstantAttributes(const SUMOSAXAttributes &)` [private]
    /// parse E1 instant attributes
    // void
    // AdditionalHandler::parseE1InstantAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string laneId = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk);
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, id.c_str(), parsedOk);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const std::vector<std::string> nextEdges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_NEXT_EDGES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const std::string detectPersons = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, "", parsedOk);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     // continue if flag is ok
    //     if (parsedOk && checkDetectPersons(SUMO_TAG_INSTANT_INDUCTION_LOOP, id, detectPersons)) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_INSTANT_INDUCTION_LOOP);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, laneId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, position);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FILE, file);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_NEXT_EDGES, nextEdges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_DETECT_PERSONS, detectPersons);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_e1_instant_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseE1InstantAttributes")
    }

    /// C++: `void AdditionalHandler::parseTAZAttributes(const SUMOSAXAttributes &)` [private]
    /// parse TAZ attributes
    // void
    // AdditionalHandler::parseTAZAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // optional attributes
    //     const PositionVector shape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), parsedOk, PositionVector());
    //     const Position center = attrs.getOpt<Position>(SUMO_ATTR_CENTER, id.c_str(), parsedOk, shape.size() > 0 ? shape.getCentroid() : Position::INVALID);
    //     const bool fill = attrs.getOpt<bool>(SUMO_ATTR_FILL, id.c_str(), parsedOk, false);
    //     const std::vector<std::string> edges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), parsedOk, RGBColor::RED);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TAZ);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionVectorAttribute(SUMO_ATTR_SHAPE, shape);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionAttribute(SUMO_ATTR_CENTER, center);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FILL, fill);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_EDGES, edges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_taz_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseTAZAttributes")
    }

    /// C++: `void AdditionalHandler::parseTAZSourceAttributes(const SUMOSAXAttributes &)` [private]
    /// parse TAZ source attributes
    // void
    // AdditionalHandler::parseTAZSourceAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string edgeID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const double weight = attrs.get<double>(SUMO_ATTR_WEIGHT, edgeID.c_str(), parsedOk);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_TAZSOURCE, {SUMO_TAG_TAZ}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TAZSOURCE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, edgeID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_WEIGHT, weight);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_taz_source_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseTAZSourceAttributes")
    }

    /// C++: `void AdditionalHandler::parseTAZSinkAttributes(const SUMOSAXAttributes &)` [private]
    /// parse TAZ sink attributes
    // void
    // AdditionalHandler::parseTAZSinkAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string edgeID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const double weight = attrs.get<double>(SUMO_ATTR_WEIGHT, edgeID.c_str(), parsedOk);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_TAZSINK, {SUMO_TAG_TAZ}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TAZSINK);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, edgeID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_WEIGHT, weight);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_taz_sink_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseTAZSinkAttributes")
    }

    /// C++: `void AdditionalHandler::parseVariableSpeedSignAttributes(const SUMOSAXAttributes &)` [private]
    /// parse variable speed sign attributes
    // void
    // AdditionalHandler::parseVariableSpeedSignAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::vector<std::string> lanes = attrs.get<std::vector<std::string> >(SUMO_ATTR_LANES, id.c_str(), parsedOk);
    //     // optional attributes
    //     const Position pos = attrs.getOpt<Position>(SUMO_ATTR_POSITION, id.c_str(), parsedOk, Position());
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_VSS);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LANES, lanes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionAttribute(SUMO_ATTR_POSITION, pos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_variable_speed_sign_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseVariableSpeedSignAttributes")
    }

    /// C++: `void AdditionalHandler::parseVariableSpeedSignStepAttributes(const SUMOSAXAttributes &)` [private]
    /// parse variable speed sign step attributes
    // void
    // AdditionalHandler::parseVariableSpeedSignStepAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const SUMOTime time = attrs.getSUMOTimeReporting(SUMO_ATTR_TIME, "", parsedOk);
    //     // optional attributes
    //     const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, "", parsedOk, OptionsCont::getOptions().getFloat("default.speed"));
    //     // check parent
    //     checkParsedParent(SUMO_TAG_STEP, {SUMO_TAG_VSS}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_STEP);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_TIME, time);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_SPEED, speed);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_variable_speed_sign_step_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseVariableSpeedSignStepAttributes")
    }

    /// C++: `void AdditionalHandler::parseCalibratorAttributes(const SUMOSAXAttributes &)` [private]
    /// parse calibrator attributes
    // void
    // AdditionalHandler::parseCalibratorAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // check that frecuency and trafficLight aren't defined together
    //     if ((attrs.hasAttribute(SUMO_ATTR_EDGE) && attrs.hasAttribute(SUMO_ATTR_LANE)) ||
    //             (!attrs.hasAttribute(SUMO_ATTR_EDGE) && !attrs.hasAttribute(SUMO_ATTR_LANE))) {
    //         writeError(TL("Calibrators need either an edge or a lane"));
    //         parsedOk = false;
    //     }
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const double pos = attrs.get<double>(SUMO_ATTR_POSITION, id.c_str(), parsedOk);
    //     // special attributes
    //     const std::string edge = attrs.getOpt<std::string>(SUMO_ATTR_EDGE, id.c_str(), parsedOk, "");
    //     const std::string lane = attrs.getOpt<std::string>(SUMO_ATTR_LANE, id.c_str(), parsedOk, "");
    //     // optional attributes
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const SUMOTime period = attrs.getOptPeriod(id.c_str(), parsedOk, DELTA_T);
    //     const std::string routeProbe = attrs.getOpt<std::string>(SUMO_ATTR_ROUTEPROBE, id.c_str(), parsedOk, "");
    //     const double jamThreshold = attrs.getOpt<double>(SUMO_ATTR_JAM_DIST_THRESHOLD, id.c_str(), parsedOk, 0.5);
    //     const std::string output = attrs.getOpt<std::string>(SUMO_ATTR_OUTPUT, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag depending of edge/lane
    //         if (attrs.hasAttribute(SUMO_ATTR_EDGE)) {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_CALIBRATOR);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_EDGE, edge);
    //         } else {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(GNE_TAG_CALIBRATOR_LANE);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, lane);
    //         }
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, pos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_PERIOD, period);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ROUTEPROBE, routeProbe);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_JAM_DIST_THRESHOLD, jamThreshold);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_OUTPUT, output);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_calibrator_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseCalibratorAttributes")
    }

    /// C++: `void AdditionalHandler::parseCalibratorFlowAttributes(const SUMOSAXAttributes &)` [private]
    /// parse calibrator flow attributes
    // void
    // AdditionalHandler::parseCalibratorFlowAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // check parent
    //     if (checkCalibratorFlowParents()) {
    //         // check that frecuency and trafficLight aren't defined together
    //         if (!attrs.hasAttribute(SUMO_ATTR_TYPE) && !attrs.hasAttribute(SUMO_ATTR_VEHSPERHOUR) && !attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //             writeError(TL("CalibratorFlows need either the attribute vehsPerHour or speed or type (or any combination of these)"));
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         } else {
    //             SUMOVehicleParameter* flowParameter = SUMOVehicleParserHelper::parseVehicleAttributes(SUMO_TAG_FLOW, attrs, false, true, true);
    //             if (flowParameter) {
    //                 // set VPH and speed
    //                 if (attrs.hasAttribute(SUMO_ATTR_VEHSPERHOUR)) {
    //                     flowParameter->repetitionOffset = TIME2STEPS(3600. / attrs.get<double>(SUMO_ATTR_VEHSPERHOUR, "", parsedOk));
    //                     flowParameter->parametersSet |= VEHPARS_VPH_SET;
    //                 }
    //                 if (attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //                     flowParameter->calibratorSpeed = attrs.get<double>(SUMO_ATTR_SPEED, "", parsedOk);
    //                     flowParameter->parametersSet |= VEHPARS_CALIBRATORSPEED_SET;
    //                 }
    //                 // set begin and end
    //                 flowParameter->depart = attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, "", parsedOk);
    //                 flowParameter->repetitionEnd = attrs.getSUMOTimeReporting(SUMO_ATTR_END, "", parsedOk);
    //                 if (parsedOk) {
    //                     // set tag
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_FLOW);
    //                     // set vehicle parameters
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(flowParameter);
    //                     // delete flow parameter (because in XMLStructure we have a copy)
    //                     delete flowParameter;
    //                 } else {
    //                     myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //                 }
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         }
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_calibrator_flow_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseCalibratorFlowAttributes")
    }

    /// C++: `void AdditionalHandler::parseRerouterAttributes(const SUMOSAXAttributes &)` [private]
    /// parse rerouter attributes
    // void
    // AdditionalHandler::parseRerouterAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::vector<std::string> edges = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), parsedOk);
    //     // optional attributes
    //     const Position pos = attrs.getOpt<Position>(SUMO_ATTR_POSITION, id.c_str(), parsedOk, Position::INVALID);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const double probability = attrs.getOpt<double>(SUMO_ATTR_PROB, id.c_str(), parsedOk, 1);
    //     SUMOTime timeThreshold = attrs.getOptSUMOTimeReporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, id.c_str(), parsedOk, 0);
    //     const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //     const bool off = attrs.getOpt<bool>(SUMO_ATTR_OFF, id.c_str(), parsedOk, false);
    //     const bool optional = attrs.getOpt<bool>(SUMO_ATTR_OPTIONAL, id.c_str(), parsedOk, false);
    //     // check attributes
    //     if (!checkNegative(SUMO_TAG_REROUTER, id, SUMO_ATTR_PROB, probability, true)) {
    //         parsedOk = false;
    //     }
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_REROUTER);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_EDGES, edges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionAttribute(SUMO_ATTR_POSITION, pos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probability);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_HALTING_TIME_THRESHOLD, timeThreshold);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_OFF, off);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_OPTIONAL, optional);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_rerouter_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseRerouterAttributes")
    }

    /// C++: `void AdditionalHandler::parseRerouterIntervalAttributes(const SUMOSAXAttributes &)` [private]
    /// parse rerouter interval attributes
    // void
    // AdditionalHandler::parseRerouterIntervalAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const SUMOTime begin = attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, "", parsedOk);
    //     const SUMOTime end = attrs.getSUMOTimeReporting(SUMO_ATTR_END, "", parsedOk);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_INTERVAL, {SUMO_TAG_REROUTER}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_INTERVAL);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_BEGIN, begin);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_END, end);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_rerouter_interval_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseRerouterIntervalAttributes")
    }

    /// C++: `void AdditionalHandler::parseClosingLaneRerouteAttributes(const SUMOSAXAttributes &)` [private]
    /// parse closing lane reroute attributes
    // void
    // AdditionalHandler::parseClosingLaneRerouteAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string laneID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // optional attributes
    //     const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, "", parsedOk, "");
    //     const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, "", parsedOk, !disallow.size() ? "authority" : "");
    //     // check parent
    //     checkParsedParent(SUMO_TAG_CLOSING_LANE_REROUTE, {SUMO_TAG_INTERVAL}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_CLOSING_LANE_REROUTE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, laneID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ALLOW, allow);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_DISALLOW, disallow);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_closing_lane_reroute_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseClosingLaneRerouteAttributes")
    }

    /// C++: `void AdditionalHandler::parseClosingRerouteAttributes(const SUMOSAXAttributes &)` [private]
    /// parse closing reroute attributes
    // void
    // AdditionalHandler::parseClosingRerouteAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string edgeID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // optional attributes
    //     const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, "", parsedOk, "");
    //     const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, "", parsedOk, "");
    //     // check parent
    //     checkParsedParent(SUMO_TAG_CLOSING_REROUTE, {SUMO_TAG_INTERVAL}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_CLOSING_REROUTE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, edgeID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ALLOW, allow);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_DISALLOW, disallow);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_closing_reroute_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseClosingRerouteAttributes")
    }

    /// C++: `void AdditionalHandler::parseDestProbRerouteAttributes(const SUMOSAXAttributes &)` [private]
    /// parse dest prob reroute attributes
    // void
    // AdditionalHandler::parseDestProbRerouteAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string edgeID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const double probability = attrs.getOpt<double>(SUMO_ATTR_PROB, "", parsedOk, 1);
    //     // check attributes
    //     if (!checkNegative(SUMO_TAG_DEST_PROB_REROUTE, edgeID, SUMO_ATTR_PROB, probability, true)) {
    //         parsedOk = false;
    //     }
    //     // check parent
    //     checkParsedParent(SUMO_TAG_DEST_PROB_REROUTE, {SUMO_TAG_INTERVAL}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_DEST_PROB_REROUTE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, edgeID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probability);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_dest_prob_reroute_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseDestProbRerouteAttributes")
    }

    /// C++: `void AdditionalHandler::parseParkingAreaRerouteAttributes(const SUMOSAXAttributes &)` [private]
    /// parse parking area reroute attributes
    // void
    // AdditionalHandler::parseParkingAreaRerouteAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string parkingAreaID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const double probability = attrs.getOpt<double>(SUMO_ATTR_PROB, "", parsedOk, 1);
    //     // check attributes
    //     if (!checkNegative(SUMO_TAG_PARKING_AREA_REROUTE, parkingAreaID, SUMO_ATTR_PROB, probability, true)) {
    //         parsedOk = false;
    //     }
    //     // optional attributes
    //     const bool visible = attrs.getOpt<bool>(SUMO_ATTR_VISIBLE, "", parsedOk, false);
    //     // check parent
    //     checkParsedParent(SUMO_TAG_PARKING_AREA_REROUTE, {SUMO_TAG_INTERVAL}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_PARKING_AREA_REROUTE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, parkingAreaID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probability);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_VISIBLE, visible);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parking_area_reroute_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseParkingAreaRerouteAttributes")
    }

    /// C++: `void AdditionalHandler::parseRouteProbRerouteAttributes(const SUMOSAXAttributes &)` [private]
    /// parse route prob reroute attributes
    // void
    // AdditionalHandler::parseRouteProbRerouteAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string routeID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const double probability = attrs.getOpt<double>(SUMO_ATTR_PROB, "", parsedOk, 1);
    //     // check attributes
    //     if (!checkNegative(SUMO_TAG_ROUTE_PROB_REROUTE, routeID, SUMO_ATTR_PROB, probability, true)) {
    //         parsedOk = false;
    //     }
    //     // check parent
    //     checkParsedParent(SUMO_TAG_ROUTE_PROB_REROUTE, {SUMO_TAG_INTERVAL}, parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ROUTE_PROB_REROUTE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, routeID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probability);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_route_prob_reroute_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseRouteProbRerouteAttributes")
    }

    /// C++: `void AdditionalHandler::parseRouteProbeAttributes(const SUMOSAXAttributes &)` [private]
    /// parse route probe attributes
    // void
    // AdditionalHandler::parseRouteProbeAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string edge = attrs.get<std::string>(SUMO_ATTR_EDGE, id.c_str(), parsedOk);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), parsedOk);
    //     const SUMOTime period = attrs.getOptPeriod(id.c_str(), parsedOk, SUMOTime_MAX_PERIOD);
    //     const SUMOTime begin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, id.c_str(), parsedOk, -1);
    //     // optional attributes
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     const std::vector<std::string> vehicleTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, std::vector<std::string>());
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ROUTEPROBE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_EDGE, edge);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FILE, file);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_PERIOD, period);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_BEGIN, begin);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vehicleTypes);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_route_probe_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseRouteProbeAttributes")
    }

    /// C++: `void AdditionalHandler::parseVaporizerAttributes(const SUMOSAXAttributes &)` [private]
    /// parse vaporizer attributes
    // void
    // AdditionalHandler::parseVaporizerAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string edgeID = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     SUMOTime begin = attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, nullptr, parsedOk);
    //     SUMOTime end = attrs.getSUMOTimeReporting(SUMO_ATTR_END, nullptr, parsedOk);
    //     // optional attributes
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, edgeID.c_str(), parsedOk, "");
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_VAPORIZER);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_EDGE, edgeID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_BEGIN, begin);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_END, end);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_vaporizer_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseVaporizerAttributes")
    }

    /// C++: `void AdditionalHandler::parseTractionSubstation(const SUMOSAXAttributes &)` [private]
    /// parse traction substation
    // void
    // AdditionalHandler::parseTractionSubstation(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // optional attributes
    //     const Position pos = attrs.getOpt<Position>(SUMO_ATTR_POSITION, id.c_str(), parsedOk, Position::INVALID);
    //     const double voltage = attrs.getOpt<double>(SUMO_ATTR_VOLTAGE, id.c_str(), parsedOk, 600);
    //     const double currentLimit = attrs.getOpt<double>(SUMO_ATTR_CURRENTLIMIT, id.c_str(), parsedOk, 400);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TRACTION_SUBSTATION);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionAttribute(SUMO_ATTR_POSITION, pos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_VOLTAGE, voltage);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_CURRENTLIMIT, currentLimit);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_traction_substation(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseTractionSubstation")
    }

    /// C++: `void AdditionalHandler::parseOverheadWire(const SUMOSAXAttributes &)` [private]
    /// parse overhead wire segment
    // void
    // AdditionalHandler::parseOverheadWire(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string substationID = attrs.get<std::string>(SUMO_ATTR_SUBSTATIONID, id.c_str(), parsedOk);
    //     const std::vector<std::string> laneIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_LANES, id.c_str(), parsedOk);
    //     // optional attributes
    //     const double startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), parsedOk, 0);
    //     const double endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), parsedOk, INVALID_DOUBLE);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     const std::vector<std::string> forbiddenInnerLanes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN, "", parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_OVERHEAD_WIRE_SECTION);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_SUBSTATIONID, substationID);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LANES, laneIDs);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_STARTPOS, startPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ENDPOS, endPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN, forbiddenInnerLanes);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_overhead_wire(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseOverheadWire")
    }

    /// C++: `void AdditionalHandler::parseOverheadWireClamp(const SUMOSAXAttributes &)` [private]
    /// parse overhead wire clamp
    // void
    // AdditionalHandler::parseOverheadWireClamp(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string substationId = attrs.get<std::string>(SUMO_ATTR_SUBSTATIONID, id.c_str(), parsedOk);
    //     const std::string wireClampStart = attrs.get<std::string>(SUMO_ATTR_OVERHEAD_WIRECLAMP_START, id.c_str(), parsedOk);
    //     const std::string wireClampLaneStart = attrs.get<std::string>(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANESTART, id.c_str(), parsedOk);
    //     const std::string wireClampEnd = attrs.get<std::string>(SUMO_ATTR_OVERHEAD_WIRECLAMP_END, id.c_str(), parsedOk);
    //     const std::string wireClampLaneEnd = attrs.get<std::string>(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANEEND, id.c_str(), parsedOk);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_OVERHEAD_WIRE_CLAMP);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_SUBSTATIONID, substationId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_START, wireClampStart);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANESTART, wireClampLaneStart);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_END, wireClampEnd);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANEEND, wireClampLaneEnd);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_overhead_wire_clamp(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseOverheadWireClamp")
    }

    /// C++: `void AdditionalHandler::parsePolyAttributes(const SUMOSAXAttributes &)` [private]
    // void
    // AdditionalHandler::parsePolyAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const PositionVector shapeStr = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), parsedOk, RGBColor::RED);
    //     const bool geo = attrs.getOpt<bool>(SUMO_ATTR_GEO, id.c_str(), parsedOk, false);
    //     const bool fill = attrs.getOpt<bool>(SUMO_ATTR_FILL, id.c_str(), parsedOk, false);
    //     const double lineWidth = attrs.getOpt<double>(SUMO_ATTR_LINEWIDTH, id.c_str(), parsedOk, Shape::DEFAULT_LINEWIDTH);
    //     const double layer = attrs.getOpt<double>(SUMO_ATTR_LAYER, id.c_str(), parsedOk, Shape::DEFAULT_LAYER);
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), parsedOk, Shape::DEFAULT_TYPE);
    //     const std::string imgFile = attrs.getOpt<std::string>(SUMO_ATTR_IMGFILE, id.c_str(), parsedOk, Shape::DEFAULT_IMG_FILE);
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), parsedOk, Shape::DEFAULT_ANGLE);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, Shape::DEFAULT_NAME);
    //     const double height = attrs.getOpt<double>(SUMO_ATTR_HEIGHT, id.c_str(), parsedOk, Shape::DEFAULT_HEIGHT);
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_POLY);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionVectorAttribute(SUMO_ATTR_SHAPE, shapeStr);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_GEO, geo);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FILL, fill);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LINEWIDTH, lineWidth);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LAYER, layer);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TYPE, type);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_IMGFILE, imgFile);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ANGLE, angle);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_HEIGHT, height);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_poly_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parsePolyAttributes")
    }

    /// C++: `void AdditionalHandler::parsePOIAttributes(const SUMOSAXAttributes &)` [private]
    /// parse POI attributes
    // void
    // AdditionalHandler::parsePOIAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // check that x and y are defined together
    //     if ((attrs.hasAttribute(SUMO_ATTR_X) && !attrs.hasAttribute(SUMO_ATTR_Y)) ||
    //             (!attrs.hasAttribute(SUMO_ATTR_X) && attrs.hasAttribute(SUMO_ATTR_Y))) {
    //         writeError(TL("X and Y must be be defined together in POIs"));
    //         parsedOk = false;
    //     }
    //     // check that lane and pos are defined together
    //     if ((attrs.hasAttribute(SUMO_ATTR_LANE) && !attrs.hasAttribute(SUMO_ATTR_POSITION)) ||
    //             (!attrs.hasAttribute(SUMO_ATTR_LANE) && attrs.hasAttribute(SUMO_ATTR_POSITION))) {
    //         writeError(TL("lane and position must be defined together in POIs"));
    //         parsedOk = false;
    //     }
    //     // check that lon and lat are defined together
    //     if ((attrs.hasAttribute(SUMO_ATTR_LON) && !attrs.hasAttribute(SUMO_ATTR_LAT)) ||
    //             (!attrs.hasAttribute(SUMO_ATTR_LON) && attrs.hasAttribute(SUMO_ATTR_LAT))) {
    //         writeError(TL("lon and lat must be be defined together in POIs"));
    //         parsedOk = false;
    //     }
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // special attributes
    //     const double x = attrs.getOpt<double>(SUMO_ATTR_X, id.c_str(), parsedOk, 0);
    //     const double y = attrs.getOpt<double>(SUMO_ATTR_Y, id.c_str(), parsedOk, 0);
    //     const std::string lane = attrs.getOpt<std::string>(SUMO_ATTR_LANE, "", parsedOk, "");
    //     const double pos = attrs.getOpt<double>(SUMO_ATTR_POSITION, id.c_str(), parsedOk, 0);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), parsedOk, false);
    //     const double posLat = attrs.getOpt<double>(SUMO_ATTR_POSITION_LAT, id.c_str(), parsedOk, 0);
    //     const double lon = attrs.getOpt<double>(SUMO_ATTR_LON, id.c_str(), parsedOk, 0);
    //     const double lat = attrs.getOpt<double>(SUMO_ATTR_LAT, id.c_str(), parsedOk, 0);
    //     // optional attributes
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), parsedOk, RGBColor::RED);
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, "", parsedOk, Shape::DEFAULT_TYPE);
    //     std::string icon = attrs.getOpt<std::string>(SUMO_ATTR_ICON, "", parsedOk, SUMOXMLDefinitions::POIIcons.getString(POIIcon::NONE));
    //     const double layer = attrs.getOpt<double>(SUMO_ATTR_LAYER, id.c_str(), parsedOk, Shape::DEFAULT_LAYER_POI);
    //     const std::string imgFile = attrs.getOpt<std::string>(SUMO_ATTR_IMGFILE, "", parsedOk, Shape::DEFAULT_IMG_FILE);
    //     const double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id.c_str(), parsedOk, Shape::DEFAULT_IMG_WIDTH);
    //     const double height = attrs.getOpt<double>(SUMO_ATTR_HEIGHT, id.c_str(), parsedOk, Shape::DEFAULT_IMG_HEIGHT);
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), parsedOk, Shape::DEFAULT_ANGLE);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, Shape::DEFAULT_NAME);
    //     // check icon
    //     if (!SUMOXMLDefinitions::POIIcons.hasString(icon)) {
    //         WRITE_WARNING(TLF("Invalid icon % for POI '%', using default", icon, id));
    //         icon = "none";
    //     }
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_POI);
    //         // add attributes depending of Lane/Lanes
    //         if (attrs.hasAttribute(SUMO_ATTR_X) && attrs.hasAttribute(SUMO_ATTR_Y)) {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_X, x);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_Y, y);
    //         } else if (attrs.hasAttribute(SUMO_ATTR_LANE) && attrs.hasAttribute(SUMO_ATTR_POSITION)) {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_LANE, lane);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION, pos);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_POSITION_LAT, posLat);
    //         } else {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LON, lon);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LAT, lat);
    //         }
    //         // add rest attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TYPE, type);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ICON, icon);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_LAYER, layer);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_IMGFILE, imgFile);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_WIDTH, width);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_HEIGHT, height);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ANGLE, angle);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_poi_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parsePOIAttributes")
    }

    /// C++: `void AdditionalHandler::parseJpsWalkableAreaAttributes(const SUMOSAXAttributes &)` [private]
    /// parse juPedSim walkable area attributes
    // void
    // AdditionalHandler::parseJpsWalkableAreaAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const PositionVector shapeStr = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(GNE_TAG_JPS_WALKABLEAREA);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionVectorAttribute(SUMO_ATTR_SHAPE, shapeStr);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_jps_walkable_area_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseJpsWalkableAreaAttributes")
    }

    /// C++: `void AdditionalHandler::parseJpsObstacleAttributes(const SUMOSAXAttributes &)` [private]
    /// parse juPedSim obstacle attributes
    // void
    // AdditionalHandler::parseJpsObstacleAttributes(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const PositionVector shapeStr = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, "");
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(GNE_TAG_JPS_OBSTACLE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addPositionVectorAttribute(SUMO_ATTR_SHAPE, shapeStr);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_NAME, name);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_jps_obstacle_attributes(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AdditionalHandler::parseJpsObstacleAttributes")
    }

    /// C++: `bool AdditionalHandler::checkCalibratorFlowParents()` [private]
    /// check calibrator flow parents
    // bool
    // AdditionalHandler::checkCalibratorFlowParents() {
    //     const auto parentCalibrator = myCommonXMLStructure.getCurrentSumoBaseObject()->getParentSumoBaseObject();
    //     if (parentCalibrator == nullptr) {
    //         return false;
    //     }
    //     const auto parentRootFile = parentCalibrator->getParentSumoBaseObject();
    //     if (parentRootFile != nullptr) {
    //         return false;
    //     }
    //     if ((parentCalibrator->getTag() == SUMO_TAG_CALIBRATOR) || (parentCalibrator->getTag() == GNE_TAG_CALIBRATOR_LANE)) {
    //         return true;
    //     } else {
    //         return writeError(TLF("Calibrator Flows have to be defined within a %.", toString(SUMO_TAG_CALIBRATOR)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_calibrator_flow_parents(&mut self) -> bool {
        todo!("port AdditionalHandler::checkCalibratorFlowParents")
    }

    /// C++: `bool AdditionalHandler::checkDetectPersons(const SumoXMLTag, const std::string &, const std::string &)` [private]
    /// check detect persons
    // bool
    // AdditionalHandler::checkDetectPersons(const SumoXMLTag currentTag, const std::string& id, const std::string& detectPersons) {
    //     if (detectPersons.empty() || SUMOXMLDefinitions::PersonModeValues.hasString(detectPersons)) {
    //         return true;
    //     } else {
    //         writeError(TLF("Attribute '%' defined in % with id '%' doesn't have a valid value (given '%').", toString(SUMO_ATTR_DETECT_PERSONS), toString(currentTag), id, detectPersons));
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_detect_persons(&mut self, currentTag: SumoXMLTag, id: &str, detectPersons: &str) -> bool {
        if detectPersons.is_empty() || utils::xml::sumo_xml_definitions::personModeValuesInitializer.iter().any(|(k, _)| *k == detectPersons) {
            true
        } else {
            self.write_error(&format!("Attribute '{}' defined in {} with id '{}' doesn't have a valid value (given '{}').", attr_str(utils::xml::sumo_xml_definitions::SUMO_ATTR_DETECT_PERSONS), tag_str(currentTag), id, detectPersons));
            false
        }
    }

    /// C++: `void AdditionalHandler::AdditionalHandler()` [private]
    /// invalidate default onstructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        panic!("AdditionalHandler() is private and never defined in C++")
    }

    /// C++: `void AdditionalHandler::AdditionalHandler(const AdditionalHandler &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &AdditionalHandler) -> Self {
        panic!("AdditionalHandler(const AdditionalHandler&) is private and never defined in C++")
    }

    /// C++: `AdditionalHandler & AdditionalHandler::operator=(const AdditionalHandler &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &AdditionalHandler) -> &mut AdditionalHandler {
        panic!("AdditionalHandler::operator= is private and never defined in C++")
    }

}

