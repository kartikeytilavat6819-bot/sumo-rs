//! Port of utils/handlers/AdditionalHandler.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::common_handler::{attr_str, tag_str};
use utils::xml::common_xml_structure::CommonXMLStructure_SumoBaseObject;
use utils::common::std_defs::NETWORK_VERSION;

const VEHPARS_VPH_SET: i64 = 2 << 7;
const VEHPARS_CALIBRATORSPEED_SET: i64 = 2 << 27;

/// C++ macro `SUMOTime_MAX_PERIOD` (SUMOTime.h): `SUMOTime_MAX - SUMOTime_MAX % DELTA_T`
fn sumo_time_max_period() -> SUMOTime {
    utils::common::time::SUMOTIME_MAX - utils::common::time::SUMOTIME_MAX % utils::common::time::delta_t()
}


/// unwrap a CommonXMLStructure getter; the C++ getters throw when the attribute is missing
fn or_throw<T>(r: Result<T, utils::common::util_exceptions::ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{}", e))
}
fn str_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> String {
    or_throw(obj.get_string_attribute(attr)).clone()
}
fn double_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> f64 {
    or_throw(obj.get_double_attribute(attr))
}
fn int_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> i32 {
    or_throw(obj.get_int_attribute(attr))
}
fn bool_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> bool {
    or_throw(obj.get_bool_attribute(attr))
}
fn time_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> SUMOTime {
    or_throw(obj.get_time_attribute(attr))
}
fn period_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> SUMOTime {
    or_throw(obj.get_period_attribute())
}
fn color_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> RGBColor {
    or_throw(obj.get_color_attribute(attr)).clone()
}
fn str_list_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> Vec<String> {
    or_throw(obj.get_string_list_attribute(attr)).clone()
}
fn double_list_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> Vec<f64> {
    or_throw(obj.get_double_list_attribute(attr)).clone()
}
fn position_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> Position {
    or_throw(obj.get_position_attribute(attr)).clone()
}
fn position_vector_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> PositionVector {
    or_throw(obj.get_position_vector_attribute(attr)).clone()
}
/// C++ `getVehicleParameter()` (SUMOVehicleParameter&) of a SumoBaseObject
fn vehicle_parameter_of(obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> &'static SUMOVehicleParameter {
    or_throw(obj.get().unwrap().get_vehicle_parameter()).downcast_ref::<SUMOVehicleParameter>().expect("SumoBaseObject does not hold a SUMOVehicleParameter")
}

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
    pub fn new(fileBucket: Ptr<utils::common::file_bucket::FileBucket>) -> Self {
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
        // open SUMOBaseOBject
        self.myCommonXMLStructure.open_sumo_base_object();
        // check tag
        // shortcut: C++ try/catch(InvalidArgument) around the switch is not reproduced; attribute errors surface as panics from the getters, upgrade when the getters return Result
        match tag {
            // Stopping Places
            SUMO_TAG_BUS_STOP => self.parse_bus_stop_attributes(attrs),
            SUMO_TAG_TRAIN_STOP => self.parse_train_stop_attributes(attrs),
            SUMO_TAG_ACCESS => self.parse_access_attributes(attrs),
            SUMO_TAG_CONTAINER_STOP => self.parse_container_stop_attributes(attrs),
            SUMO_TAG_CHARGING_STATION => self.parse_charging_station_attributes(attrs),
            SUMO_TAG_PARKING_AREA => self.parse_parking_area_attributes(attrs),
            SUMO_TAG_PARKING_SPACE => self.parse_parking_space_attributes(attrs),
            // Detectors
            SUMO_TAG_E1DETECTOR | SUMO_TAG_INDUCTION_LOOP => self.parse_e1_attributes(attrs),
            SUMO_TAG_E2DETECTOR | SUMO_TAG_LANE_AREA_DETECTOR => self.parse_e2_attributes(attrs),
            SUMO_TAG_E3DETECTOR | SUMO_TAG_ENTRY_EXIT_DETECTOR => self.parse_e3_attributes(attrs),
            SUMO_TAG_DET_ENTRY => self.parse_entry_attributes(attrs),
            SUMO_TAG_DET_EXIT => self.parse_exit_attributes(attrs),
            SUMO_TAG_INSTANT_INDUCTION_LOOP => self.parse_e1_instant_attributes(attrs),
            // TAZs
            SUMO_TAG_TAZ => self.parse_taz_attributes(attrs),
            SUMO_TAG_TAZSOURCE => self.parse_taz_source_attributes(attrs),
            SUMO_TAG_TAZSINK => self.parse_taz_sink_attributes(attrs),
            // Variable Speed Sign
            SUMO_TAG_VSS => self.parse_variable_speed_sign_attributes(attrs),
            SUMO_TAG_STEP => self.parse_variable_speed_sign_step_attributes(attrs),
            // Calibrator
            SUMO_TAG_CALIBRATOR | GNE_TAG_CALIBRATOR_LANE => self.parse_calibrator_attributes(attrs),
            // flow (calibrator)
            SUMO_TAG_FLOW => self.parse_calibrator_flow_attributes(attrs),
            // Rerouter
            SUMO_TAG_REROUTER => self.parse_rerouter_attributes(attrs),
            SUMO_TAG_INTERVAL => self.parse_rerouter_interval_attributes(attrs),
            SUMO_TAG_CLOSING_LANE_REROUTE => self.parse_closing_lane_reroute_attributes(attrs),
            SUMO_TAG_CLOSING_REROUTE => self.parse_closing_reroute_attributes(attrs),
            SUMO_TAG_DEST_PROB_REROUTE => self.parse_dest_prob_reroute_attributes(attrs),
            SUMO_TAG_PARKING_AREA_REROUTE => self.parse_parking_area_reroute_attributes(attrs),
            SUMO_TAG_ROUTE_PROB_REROUTE => self.parse_route_prob_reroute_attributes(attrs),
            // Route probe
            SUMO_TAG_ROUTEPROBE => self.parse_route_probe_attributes(attrs),
            // Vaporizer (deprecated)
            SUMO_TAG_VAPORIZER => self.parse_vaporizer_attributes(attrs),
            // wires
            SUMO_TAG_TRACTION_SUBSTATION => self.parse_traction_substation(attrs),
            SUMO_TAG_OVERHEAD_WIRE_CLAMP => self.parse_overhead_wire_clamp(attrs),
            SUMO_TAG_OVERHEAD_WIRE_SECTION => self.parse_overhead_wire(attrs),
            // Poly
            SUMO_TAG_POLY => self.parse_poly_attributes(attrs),
            SUMO_TAG_POI => self.parse_poi_attributes(attrs),
            // JuPedSim
            GNE_TAG_JPS_WALKABLEAREA => self.parse_jps_walkable_area_attributes(attrs),
            GNE_TAG_JPS_OBSTACLE => self.parse_jps_obstacle_attributes(attrs),
            // parameters
            SUMO_TAG_PARAM => self.parse_parameters(attrs),
            _ => {
                // tag cannot be parsed in AdditionalHandler
                self.myCommonXMLStructure.abort_sumo_base_object();
                return false;
            }
        }
        true
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
        // get last inserted object
        let obj = self.myCommonXMLStructure.get_current_sumo_base_object();
        if !obj.is_null() {
            // close SUMOBaseOBject
            self.myCommonXMLStructure.close_sumo_base_object();
            // check tag
            match obj.get_tag() {
                // Stopping Places
                SUMO_TAG_BUS_STOP | SUMO_TAG_TRAIN_STOP | SUMO_TAG_CONTAINER_STOP | SUMO_TAG_CHARGING_STATION | SUMO_TAG_PARKING_AREA
                // detectors
                | SUMO_TAG_E1DETECTOR | SUMO_TAG_INDUCTION_LOOP | SUMO_TAG_E2DETECTOR | SUMO_TAG_LANE_AREA_DETECTOR | SUMO_TAG_E3DETECTOR
                | SUMO_TAG_ENTRY_EXIT_DETECTOR | SUMO_TAG_INSTANT_INDUCTION_LOOP
                // TAZs
                | SUMO_TAG_TAZ
                // Variable Speed Sign
                | SUMO_TAG_VSS
                // Calibrator
                | SUMO_TAG_CALIBRATOR | GNE_TAG_CALIBRATOR_LANE
                // Rerouter
                | SUMO_TAG_REROUTER
                // Route probe
                | SUMO_TAG_ROUTEPROBE
                // Vaporizer (deprecated)
                | SUMO_TAG_VAPORIZER
                // wires
                | SUMO_TAG_TRACTION_SUBSTATION | SUMO_TAG_OVERHEAD_WIRE_SECTION | SUMO_TAG_OVERHEAD_WIRE_CLAMP
                // Shapes
                | SUMO_TAG_POLY | SUMO_TAG_POI
                // JuPedSim
                | GNE_TAG_JPS_WALKABLEAREA | GNE_TAG_JPS_OBSTACLE => {
                    // parse object and all their childrens
                    self.parse_sumo_base_object(obj);
                    // delete object (and all of their childrens): the parent drops its owned child
                    let parent = obj.get_parent_sumo_base_object();
                    if !parent.is_null() {
                        parent.m().remove_sumo_base_object_child(obj);
                    }
                    // shortcut: root objects (no parent) are not freed here, upgrade with root ownership
                }
                _ => {}
            }
        }
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
    pub fn parse_sumo_base_object(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) {
        // check if loading was aborted
        if (!self.myAbortLoading) {
            // switch tag
            match obj.get_tag() {
                SUMO_TAG_BUS_STOP => {
                    if (self.build_bus_stop(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_STARTPOS), double_attr(obj, SUMO_ATTR_ENDPOS), &str_attr(obj, SUMO_ATTR_NAME), &str_list_attr(obj, SUMO_ATTR_LINES), int_attr(obj, SUMO_ATTR_PERSON_CAPACITY), double_attr(obj, SUMO_ATTR_PARKING_LENGTH), &color_attr(obj, SUMO_ATTR_COLOR), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), double_attr(obj, SUMO_ATTR_ANGLE), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_TRAIN_STOP => {
                    if (self.build_train_stop(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_STARTPOS), double_attr(obj, SUMO_ATTR_ENDPOS), &str_attr(obj, SUMO_ATTR_NAME), &str_list_attr(obj, SUMO_ATTR_LINES), int_attr(obj, SUMO_ATTR_PERSON_CAPACITY), double_attr(obj, SUMO_ATTR_PARKING_LENGTH), &color_attr(obj, SUMO_ATTR_COLOR), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), double_attr(obj, SUMO_ATTR_ANGLE), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_ACCESS => {
                    if (self.build_access(obj, &str_attr(obj, SUMO_ATTR_LANE), &str_attr(obj, SUMO_ATTR_POSITION), double_attr(obj, SUMO_ATTR_LENGTH), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_CONTAINER_STOP => {
                    if (self.build_container_stop(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_STARTPOS), double_attr(obj, SUMO_ATTR_ENDPOS), &str_attr(obj, SUMO_ATTR_NAME), &str_list_attr(obj, SUMO_ATTR_LINES), int_attr(obj, SUMO_ATTR_CONTAINER_CAPACITY), double_attr(obj, SUMO_ATTR_PARKING_LENGTH), &color_attr(obj, SUMO_ATTR_COLOR), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), double_attr(obj, SUMO_ATTR_ANGLE), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_CHARGING_STATION => {
                    if (self.build_charging_station(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_STARTPOS), double_attr(obj, SUMO_ATTR_ENDPOS), &str_attr(obj, SUMO_ATTR_NAME), double_attr(obj, SUMO_ATTR_CHARGINGPOWER), double_attr(obj, SUMO_ATTR_TOTALPOWER), double_attr(obj, SUMO_ATTR_EFFICIENCY), bool_attr(obj, SUMO_ATTR_CHARGEINTRANSIT), time_attr(obj, SUMO_ATTR_CHARGEDELAY), &str_attr(obj, SUMO_ATTR_CHARGETYPE), time_attr(obj, SUMO_ATTR_WAITINGTIME), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), &str_attr(obj, SUMO_ATTR_PARKING_AREA), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_PARKING_AREA => {
                    if (self.build_parking_area(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_STARTPOS), double_attr(obj, SUMO_ATTR_ENDPOS), &str_attr(obj, SUMO_ATTR_DEPARTPOS), &str_attr(obj, SUMO_ATTR_NAME), &str_list_attr(obj, SUMO_ATTR_ACCEPTED_BADGES), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), int_attr(obj, SUMO_ATTR_ROADSIDE_CAPACITY), bool_attr(obj, SUMO_ATTR_ONROAD), double_attr(obj, SUMO_ATTR_WIDTH), double_attr(obj, SUMO_ATTR_LENGTH), double_attr(obj, SUMO_ATTR_ANGLE), bool_attr(obj, SUMO_ATTR_LEFTHAND), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_PARKING_SPACE => {
                    if (self.build_parking_space(obj, double_attr(obj, SUMO_ATTR_X), double_attr(obj, SUMO_ATTR_Y), double_attr(obj, SUMO_ATTR_Z), &str_attr(obj, SUMO_ATTR_NAME), &str_attr(obj, SUMO_ATTR_WIDTH), &str_attr(obj, SUMO_ATTR_LENGTH), &str_attr(obj, SUMO_ATTR_ANGLE), double_attr(obj, SUMO_ATTR_SLOPE), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_E1DETECTOR | SUMO_TAG_INDUCTION_LOOP => {
                    // build E1
                    if (self.build_e1_detector(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_POSITION), period_attr(obj), &str_attr(obj, SUMO_ATTR_FILE), &str_list_attr(obj, SUMO_ATTR_VTYPES), &str_list_attr(obj, SUMO_ATTR_NEXT_EDGES), &str_attr(obj, SUMO_ATTR_DETECT_PERSONS), &str_attr(obj, SUMO_ATTR_NAME), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_E2DETECTOR | SUMO_TAG_LANE_AREA_DETECTOR => {
                    if (obj.has_string_attribute(SUMO_ATTR_LANE)) {
                        if (self.build_single_lane_detector_e2(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_POSITION), double_attr(obj, SUMO_ATTR_LENGTH), period_attr(obj), &str_attr(obj, SUMO_ATTR_TLID), &str_attr(obj, SUMO_ATTR_FILE), &str_list_attr(obj, SUMO_ATTR_VTYPES), &str_list_attr(obj, SUMO_ATTR_NEXT_EDGES), &str_attr(obj, SUMO_ATTR_DETECT_PERSONS), &str_attr(obj, SUMO_ATTR_NAME), time_attr(obj, SUMO_ATTR_HALTING_TIME_THRESHOLD), double_attr(obj, SUMO_ATTR_HALTING_SPEED_THRESHOLD), double_attr(obj, SUMO_ATTR_JAM_DIST_THRESHOLD), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), bool_attr(obj, SUMO_ATTR_SHOW_DETECTOR), obj.get_parameters())) {
                            obj.m().mark_as_created();
                        }
                    } else {
                        if (self.build_multi_lane_detector_e2(obj, &str_attr(obj, SUMO_ATTR_ID), &str_list_attr(obj, SUMO_ATTR_LANES), double_attr(obj, SUMO_ATTR_POSITION), double_attr(obj, SUMO_ATTR_ENDPOS), period_attr(obj), &str_attr(obj, SUMO_ATTR_TLID), &str_attr(obj, SUMO_ATTR_FILE), &str_list_attr(obj, SUMO_ATTR_VTYPES), &str_list_attr(obj, SUMO_ATTR_NEXT_EDGES), &str_attr(obj, SUMO_ATTR_DETECT_PERSONS), &str_attr(obj, SUMO_ATTR_NAME), time_attr(obj, SUMO_ATTR_HALTING_TIME_THRESHOLD), double_attr(obj, SUMO_ATTR_HALTING_SPEED_THRESHOLD), double_attr(obj, SUMO_ATTR_JAM_DIST_THRESHOLD), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), bool_attr(obj, SUMO_ATTR_SHOW_DETECTOR), obj.get_parameters())) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                SUMO_TAG_E3DETECTOR | SUMO_TAG_ENTRY_EXIT_DETECTOR => {
                    if (self.build_detector_e3(obj, &str_attr(obj, SUMO_ATTR_ID), &position_attr(obj, SUMO_ATTR_POSITION), period_attr(obj), &str_attr(obj, SUMO_ATTR_FILE), &str_list_attr(obj, SUMO_ATTR_VTYPES), &str_list_attr(obj, SUMO_ATTR_NEXT_EDGES), &str_attr(obj, SUMO_ATTR_DETECT_PERSONS), &str_attr(obj, SUMO_ATTR_NAME), time_attr(obj, SUMO_ATTR_HALTING_TIME_THRESHOLD), double_attr(obj, SUMO_ATTR_HALTING_SPEED_THRESHOLD), bool_attr(obj, SUMO_ATTR_OPEN_ENTRY), bool_attr(obj, SUMO_ATTR_EXPECT_ARRIVAL), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_DET_ENTRY => {
                    if (self.build_detector_entry(obj, &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_POSITION), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_DET_EXIT => {
                    if (self.build_detector_exit(obj, &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_POSITION), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_INSTANT_INDUCTION_LOOP => {
                    if (self.build_detector_e1_instant(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_POSITION), &str_attr(obj, SUMO_ATTR_FILE), &str_list_attr(obj, SUMO_ATTR_VTYPES), &str_list_attr(obj, SUMO_ATTR_NEXT_EDGES), &str_attr(obj, SUMO_ATTR_DETECT_PERSONS), &str_attr(obj, SUMO_ATTR_NAME), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_TAZ => {
                    if (self.build_taz(obj, &str_attr(obj, SUMO_ATTR_ID), &position_vector_attr(obj, SUMO_ATTR_SHAPE), &position_attr(obj, SUMO_ATTR_CENTER), bool_attr(obj, SUMO_ATTR_FILL), &color_attr(obj, SUMO_ATTR_COLOR), &str_list_attr(obj, SUMO_ATTR_EDGES), &str_attr(obj, SUMO_ATTR_NAME), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_TAZSOURCE => {
                    if (self.build_taz_source(obj, &str_attr(obj, SUMO_ATTR_ID), double_attr(obj, SUMO_ATTR_WEIGHT))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_TAZSINK => {
                    if (self.build_taz_sink(obj, &str_attr(obj, SUMO_ATTR_ID), double_attr(obj, SUMO_ATTR_WEIGHT))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_VSS => {
                    if (self.build_variable_speed_sign(obj, &str_attr(obj, SUMO_ATTR_ID), &position_attr(obj, SUMO_ATTR_POSITION), &str_list_attr(obj, SUMO_ATTR_LANES), &str_attr(obj, SUMO_ATTR_NAME), &str_list_attr(obj, SUMO_ATTR_VTYPES), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_STEP => {
                    if (self.build_variable_speed_sign_step(obj, time_attr(obj, SUMO_ATTR_TIME), double_attr(obj, SUMO_ATTR_SPEED))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_CALIBRATOR => {
                    if (self.build_edge_calibrator(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_EDGE), double_attr(obj, SUMO_ATTR_POSITION), &str_attr(obj, SUMO_ATTR_NAME), &str_attr(obj, SUMO_ATTR_OUTPUT), period_attr(obj), &str_attr(obj, SUMO_ATTR_ROUTEPROBE), double_attr(obj, SUMO_ATTR_JAM_DIST_THRESHOLD), &str_list_attr(obj, SUMO_ATTR_VTYPES), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                GNE_TAG_CALIBRATOR_LANE => {
                    if (self.build_lane_calibrator(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_POSITION), &str_attr(obj, SUMO_ATTR_NAME), &str_attr(obj, SUMO_ATTR_OUTPUT), period_attr(obj), &str_attr(obj, SUMO_ATTR_ROUTEPROBE), double_attr(obj, SUMO_ATTR_JAM_DIST_THRESHOLD), &str_list_attr(obj, SUMO_ATTR_VTYPES), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_FLOW => {
                    if (self.build_calibrator_flow(obj, vehicle_parameter_of(obj))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_REROUTER => {
                    if (self.build_rerouter(obj, &str_attr(obj, SUMO_ATTR_ID), &position_attr(obj, SUMO_ATTR_POSITION), &str_list_attr(obj, SUMO_ATTR_EDGES), double_attr(obj, SUMO_ATTR_PROB), &str_attr(obj, SUMO_ATTR_NAME), bool_attr(obj, SUMO_ATTR_OFF), bool_attr(obj, SUMO_ATTR_OPTIONAL), time_attr(obj, SUMO_ATTR_HALTING_TIME_THRESHOLD), &str_list_attr(obj, SUMO_ATTR_VTYPES), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_CLOSING_LANE_REROUTE => {
                    if (self.build_closing_lane_reroute(obj, &str_attr(obj, SUMO_ATTR_ID), parse_vehicle_classes_allow_disallow(&str_attr(obj, SUMO_ATTR_ALLOW), &str_attr(obj, SUMO_ATTR_DISALLOW), NETWORK_VERSION))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_CLOSING_REROUTE => {
                    if (self.build_closing_reroute(obj, &str_attr(obj, SUMO_ATTR_ID), parse_vehicle_classes_allow_disallow(&str_attr(obj, SUMO_ATTR_ALLOW), &str_attr(obj, SUMO_ATTR_DISALLOW), NETWORK_VERSION))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_DEST_PROB_REROUTE => {
                    if (self.build_dest_prob_reroute(obj, &str_attr(obj, SUMO_ATTR_ID), double_attr(obj, SUMO_ATTR_PROB))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_PARKING_AREA_REROUTE => {
                    if (self.build_parking_area_reroute(obj, &str_attr(obj, SUMO_ATTR_ID), double_attr(obj, SUMO_ATTR_PROB), bool_attr(obj, SUMO_ATTR_VISIBLE))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_ROUTE_PROB_REROUTE => {
                    if (self.build_route_prob_reroute(obj, &str_attr(obj, SUMO_ATTR_ID), double_attr(obj, SUMO_ATTR_PROB))) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_INTERVAL => {
                    // check if is aREROUTER interval
                    if (obj.get_parent_sumo_base_object().get_tag() == SUMO_TAG_REROUTER) {
                        if (self.build_rerouter_interval(obj, time_attr(obj, SUMO_ATTR_BEGIN), time_attr(obj, SUMO_ATTR_END))) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                SUMO_TAG_ROUTEPROBE => {
                    if (self.build_route_probe(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_EDGE), period_attr(obj), &str_attr(obj, SUMO_ATTR_NAME), &str_attr(obj, SUMO_ATTR_FILE), time_attr(obj, SUMO_ATTR_BEGIN), &str_list_attr(obj, SUMO_ATTR_VTYPES), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_VAPORIZER => {
                    if (self.build_vaporizer(obj, &str_attr(obj, SUMO_ATTR_EDGE), time_attr(obj, SUMO_ATTR_BEGIN), time_attr(obj, SUMO_ATTR_END), &str_attr(obj, SUMO_ATTR_NAME), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_TRACTION_SUBSTATION => {
                    if (self.build_traction_substation(obj, &str_attr(obj, SUMO_ATTR_ID), &position_attr(obj, SUMO_ATTR_POSITION), double_attr(obj, SUMO_ATTR_VOLTAGE), double_attr(obj, SUMO_ATTR_CURRENTLIMIT), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_OVERHEAD_WIRE_SECTION => {
                    if (self.build_overhead_wire(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_SUBSTATIONID), &str_list_attr(obj, SUMO_ATTR_LANES), double_attr(obj, SUMO_ATTR_STARTPOS), double_attr(obj, SUMO_ATTR_ENDPOS), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), &str_list_attr(obj, SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_OVERHEAD_WIRE_CLAMP => {
                    if (self.build_overhead_wire_clamp(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_OVERHEAD_WIRECLAMP_START), &str_attr(obj, SUMO_ATTR_OVERHEAD_WIRECLAMP_LANESTART), &str_attr(obj, SUMO_ATTR_OVERHEAD_WIRECLAMP_END), &str_attr(obj, SUMO_ATTR_OVERHEAD_WIRECLAMP_LANEEND), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_POLY => {
                    if (self.build_polygon(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_TYPE), &color_attr(obj, SUMO_ATTR_COLOR), double_attr(obj, SUMO_ATTR_LAYER), double_attr(obj, SUMO_ATTR_ANGLE), &str_attr(obj, SUMO_ATTR_IMGFILE), &position_vector_attr(obj, SUMO_ATTR_SHAPE), bool_attr(obj, SUMO_ATTR_GEO), bool_attr(obj, SUMO_ATTR_FILL), double_attr(obj, SUMO_ATTR_LINEWIDTH), &str_attr(obj, SUMO_ATTR_NAME), double_attr(obj, SUMO_ATTR_HEIGHT), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_POI => {
                    // check if we want to create a POI, POILane or POIGEO
                    if (obj.has_double_attribute(SUMO_ATTR_X)) {
                        // build POI over view
                        if (self.build_poi(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_TYPE), &color_attr(obj, SUMO_ATTR_COLOR), double_attr(obj, SUMO_ATTR_X), double_attr(obj, SUMO_ATTR_Y), &str_attr(obj, SUMO_ATTR_ICON), double_attr(obj, SUMO_ATTR_LAYER), double_attr(obj, SUMO_ATTR_ANGLE), &str_attr(obj, SUMO_ATTR_IMGFILE), double_attr(obj, SUMO_ATTR_WIDTH), double_attr(obj, SUMO_ATTR_HEIGHT), &str_attr(obj, SUMO_ATTR_NAME), obj.get_parameters())) {
                            obj.m().mark_as_created();
                        }
                    } else if (obj.has_string_attribute(SUMO_ATTR_LANE)) {
                        // build POI over Lane
                        if (self.build_poi_lane(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_TYPE), &color_attr(obj, SUMO_ATTR_COLOR), &str_attr(obj, SUMO_ATTR_LANE), double_attr(obj, SUMO_ATTR_POSITION), bool_attr(obj, SUMO_ATTR_FRIENDLY_POS), double_attr(obj, SUMO_ATTR_POSITION_LAT), &str_attr(obj, SUMO_ATTR_ICON), double_attr(obj, SUMO_ATTR_LAYER), double_attr(obj, SUMO_ATTR_ANGLE), &str_attr(obj, SUMO_ATTR_IMGFILE), double_attr(obj, SUMO_ATTR_WIDTH), double_attr(obj, SUMO_ATTR_HEIGHT), &str_attr(obj, SUMO_ATTR_NAME), obj.get_parameters())) {
                            obj.m().mark_as_created();
                        }
                    } else {
                        // build POIGEO over view
                        if (self.build_poi_geo(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_TYPE), &color_attr(obj, SUMO_ATTR_COLOR), double_attr(obj, SUMO_ATTR_LON), double_attr(obj, SUMO_ATTR_LAT), &str_attr(obj, SUMO_ATTR_ICON), double_attr(obj, SUMO_ATTR_LAYER), double_attr(obj, SUMO_ATTR_ANGLE), &str_attr(obj, SUMO_ATTR_IMGFILE), double_attr(obj, SUMO_ATTR_WIDTH), double_attr(obj, SUMO_ATTR_HEIGHT), &str_attr(obj, SUMO_ATTR_NAME), obj.get_parameters())) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                GNE_TAG_JPS_WALKABLEAREA => {
                    if (self.build_jps_walkable_area(obj, &str_attr(obj, SUMO_ATTR_ID), &position_vector_attr(obj, SUMO_ATTR_SHAPE), bool_attr(obj, SUMO_ATTR_GEO), &str_attr(obj, SUMO_ATTR_NAME), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                GNE_TAG_JPS_OBSTACLE => {
                    if (self.build_jps_obstacle(obj, &str_attr(obj, SUMO_ATTR_ID), &position_vector_attr(obj, SUMO_ATTR_SHAPE), bool_attr(obj, SUMO_ATTR_GEO), &str_attr(obj, SUMO_ATTR_NAME), obj.get_parameters())) {
                        obj.m().mark_as_created();
                    }
                }
                _ => {}
            }
            // now iterate over childrens
            for child in obj.get_sumo_base_object_children().iter().map(|c| Ptr::from_box(c)) {
                // call this function recursively
                self.parse_sumo_base_object(child);
            }
        }
    }

    /// C++: `bool AdditionalHandler::buildBusStop(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::vector<std::string> &, const int, const double, const RGBColor &, const bool, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a bus stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_bus_stop(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, lines: &Vec<String>, personCapacity: i32, parkingLength: f64, color: &RGBColor, friendlyPosition: bool, angle: f64, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildBusStop is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTrainStop(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::vector<std::string> &, const int, const double, const RGBColor &, const bool, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a train stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_train_stop(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, lines: &Vec<String>, personCapacity: i32, parkingLength: f64, color: &RGBColor, friendlyPosition: bool, angle: f64, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildTrainStop is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildAccess(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds an Access
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_access(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, laneID: &str, pos: &str, length: f64, friendlyPos: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildAccess is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildContainerStop(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::vector<std::string> &, const int, const double, const RGBColor &, const bool, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a container stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_container_stop(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, lines: &Vec<String>, containerCapacity: i32, parkingLength: f64, color: &RGBColor, friendlyPosition: bool, angle: f64, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildContainerStop is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildChargingStation(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const double, const double, const double, const bool, const SUMOTime, const std::string &, const SUMOTime, const bool, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a charging Station
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_charging_station(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, startPos: f64, endPos: f64, name: &str, chargingPower: f64, totalPower: f64, efficiency: f64, chargeInTransit: bool, chargeDelay: SUMOTime, chargeType: &str, waitingTime: SUMOTime, friendlyPosition: bool, parkingAreaID: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildChargingStation is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildParkingArea(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const std::string &, const std::string &, const std::vector<std::string> &, const bool, const int, const bool, const double, const double, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a Parking Area
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_parking_area(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, startPos: f64, endPos: f64, departPos: &str, name: &str, lines: &Vec<String>, friendlyPosition: bool, roadSideCapacity: i32, onRoad: bool, width: f64, length: f64, angle: f64, lefthand: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildParkingArea is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildParkingSpace(const CommonXMLStructure::SumoBaseObject *, const double, const double, const double, const std::string &, const std::string &, const std::string &, const std::string &, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a Parking Space
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_parking_space(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, x: f64, y: f64, z: f64, name: &str, width: &str, length: &str, angle: &str, slope: f64, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildParkingSpace is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildE1Detector(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const SUMOTime, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a induction loop detector (E1)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_e1_detector(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, position: f64, period: SUMOTime, file: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, friendlyPos: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildE1Detector is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildSingleLaneDetectorE2(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const double, const SUMOTime, const std::string &, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const SUMOTime, const double, const double, const bool, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a single-lane Area Detector (E2)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_single_lane_detector_e2(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, pos: f64, length: f64, period: SUMOTime, trafficLight: &str, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, timeThreshold: SUMOTime, speedThreshold: f64, jamThreshold: f64, friendlyPos: bool, show: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildSingleLaneDetectorE2 is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildMultiLaneDetectorE2(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::vector<std::string> &, const double, const double, const SUMOTime, const std::string &, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const SUMOTime, const double, const double, const bool, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a multi-lane Area Detector (E2)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_multi_lane_detector_e2(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneIDs: &Vec<String>, pos: f64, endPos: f64, period: SUMOTime, trafficLight: &str, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, timeThreshold: SUMOTime, speedThreshold: f64, jamThreshold: f64, friendlyPos: bool, show: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildMultiLaneDetectorE2 is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorE3(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const SUMOTime, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const SUMOTime, const double, const bool, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a multi entry exit detector (E3)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_e3(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, pos: &Position, period: SUMOTime, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, timeThreshold: SUMOTime, speedThreshold: f64, openEntry: bool, expectedArrival: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildDetectorE3 is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorEntry(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a entry detector (E3)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_entry(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, laneID: &str, pos: f64, friendlyPos: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildDetectorEntry is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorExit(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a exit detector (E3)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_exit(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, laneID: &str, pos: f64, friendlyPos: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildDetectorExit is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDetectorE1Instant(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, const std::string &, const std::string &, const bool, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a Instant Induction Loop Detector (E1Instant)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_detector_e1_instant(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, pos: f64, filename: &str, vehicleTypes: &Vec<String>, nextEdges: &Vec<String>, detectPersons: &str, name: &str, friendlyPos: bool, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildDetectorE1Instant is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildLaneCalibrator(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const std::string &, const std::string &, const SUMOTime, const std::string &, const double, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a microscopic calibrator over a lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_lane_calibrator(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, laneID: &str, pos: f64, name: &str, outfile: &str, period: SUMOTime, routeprobe: &str, jamThreshold: f64, vTypes: &Vec<String>, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildLaneCalibrator is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildEdgeCalibrator(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const double, const std::string &, const std::string &, const SUMOTime, const std::string &, const double, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a microscopic calibrator over an edge
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge_calibrator(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, edgeID: &str, pos: f64, name: &str, outfile: &str, period: SUMOTime, routeprobe: &str, jamThreshold: f64, vTypes: &Vec<String>, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildEdgeCalibrator is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildCalibratorFlow(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// builds a calibrator flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_calibrator_flow(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameter: &SUMOVehicleParameter) -> bool {
        panic!("AdditionalHandler::buildCalibratorFlow is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRerouter(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const std::vector<std::string> &, const double, const std::string &, const bool, const bool, const SUMOTime, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a rerouter
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_rerouter(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, pos: &Position, edgeIDs: &Vec<String>, prob: f64, name: &str, off: bool, optional: bool, timeThreshold: SUMOTime, vTypes: &Vec<String>, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildRerouter is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRerouterInterval(const CommonXMLStructure::SumoBaseObject *, const SUMOTime, const SUMOTime)` [virtual, pure virtual]
    /// builds a rerouter interval
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_rerouter_interval(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, begin: SUMOTime, end: SUMOTime) -> bool {
        panic!("AdditionalHandler::buildRerouterInterval is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildClosingLaneReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, SVCPermissions)` [virtual, pure virtual]
    /// builds a closing lane reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_closing_lane_reroute(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, closedLane: &str, permissions: SVCPermissions) -> bool {
        panic!("AdditionalHandler::buildClosingLaneReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildClosingReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, SVCPermissions)` [virtual, pure virtual]
    /// builds a closing edge reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_closing_reroute(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, closedEdgeID: &str, permissions: SVCPermissions) -> bool {
        panic!("AdditionalHandler::buildClosingReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildDestProbReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// builds a dest prob reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_dest_prob_reroute(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, newEdgeDestinationID: &str, probability: f64) -> bool {
        panic!("AdditionalHandler::buildDestProbReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildParkingAreaReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const bool)` [virtual, pure virtual]
    /// builds a parking area reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_parking_area_reroute(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, newParkignAreaID: &str, probability: f64, visible: bool) -> bool {
        panic!("AdditionalHandler::buildParkingAreaReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRouteProbReroute(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// builds a route prob reroute
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_route_prob_reroute(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, newRouteID: &str, probability: f64) -> bool {
        panic!("AdditionalHandler::buildRouteProbReroute is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildRouteProbe(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const SUMOTime, const std::string &, const std::string &, const SUMOTime, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// builds a Route probe
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_route_probe(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, edgeID: &str, period: SUMOTime, name: &str, file: &str, begin: SUMOTime, vTypes: &Vec<String>, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildRouteProbe is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildVariableSpeedSign(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const std::vector<std::string> &, const std::string &, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a VariableSpeedSign (lane speed additional)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_variable_speed_sign(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, pos: &Position, laneIDs: &Vec<String>, name: &str, vTypes: &Vec<String>, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildVariableSpeedSign is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildVariableSpeedSignStep(const CommonXMLStructure::SumoBaseObject *, const SUMOTime, const double)` [virtual, pure virtual]
    /// Builds a VariableSpeedSign Step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_variable_speed_sign_step(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, time: SUMOTime, speed: f64) -> bool {
        panic!("AdditionalHandler::buildVariableSpeedSignStep is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildVaporizer(const CommonXMLStructure::SumoBaseObject *, const std::string &, const SUMOTime, const SUMOTime, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a vaporizer (lane speed additional)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_vaporizer(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, edgeID: &str, from: SUMOTime, endTime: SUMOTime, name: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildVaporizer is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTAZ(const CommonXMLStructure::SumoBaseObject *, const std::string &, const PositionVector &, const Position &, const bool, const RGBColor &, const std::vector<std::string> &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a TAZ (Traffic Assignment Zone)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, shape: &PositionVector, center: &Position, fill: bool, color: &RGBColor, edgeIDs: &Vec<String>, name: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildTAZ is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTAZSource(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// Builds a TAZSource (Traffic Assignment Zone)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz_source(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, edgeID: &str, departWeight: f64) -> bool {
        panic!("AdditionalHandler::buildTAZSource is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTAZSink(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// Builds a TAZSink (Traffic Assignment Zone)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz_sink(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, edgeID: &str, arrivalWeight: f64) -> bool {
        panic!("AdditionalHandler::buildTAZSink is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildTractionSubstation(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Position &, const double, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// build traction substation
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_traction_substation(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, pos: &Position, voltage: f64, currentLimit: f64, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildTractionSubstation is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildOverheadWire(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const std::vector<std::string> &, const double, const double, const bool, const std::vector<std::string> &, const Parameterised::Map &)` [virtual, pure virtual]
    /// build overhead wire
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_overhead_wire(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, substationId: &str, laneIDs: &Vec<String>, startPos: f64, endPos: f64, friendlyPos: bool, forbiddenInnerLanes: &Vec<String>, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildOverheadWire is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildOverheadWireClamp(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// build overhead wire clamp
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_overhead_wire_clamp(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, overheadWireIDStartClamp: &str, laneIDStartClamp: &str, overheadWireIDEndClamp: &str, laneIDEndClamp: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildOverheadWireClamp is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPolygon(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const double, const double, const std::string &, const PositionVector &, const bool, const bool, const double, const std::string &, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a polygon using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_polygon(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, type_: &str, color: &RGBColor, layer: f64, angle: f64, imgFile: &str, shape: &PositionVector, geo: bool, fill: bool, lineWidth: f64, name: &str, height: f64, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildPolygon is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPOI(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const double, const double, const std::string &, const double, const double, const std::string &, const double, const double, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a POI using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_poi(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, type_: &str, color: &RGBColor, x: f64, y: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, name: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildPOI is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPOILane(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const std::string &, const double, const bool, const double, const std::string &, const double, const double, const std::string &, const double, const double, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a POI over lane using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_poi_lane(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, type_: &str, color: &RGBColor, laneID: &str, posOverLane: f64, friendlyPosition: bool, posLat: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, name: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildPOILane is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildPOIGeo(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const RGBColor &, const double, const double, const std::string &, const double, const double, const std::string &, const double, const double, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a POI in GEO coordinaten using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_poi_geo(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, type_: &str, color: &RGBColor, lon: f64, lat: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, name: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildPOIGeo is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildJpsWalkableArea(const CommonXMLStructure::SumoBaseObject *, const std::string &, const PositionVector &, bool, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a JuPedSim walkable area using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_jps_walkable_area(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, shape: &PositionVector, geo: bool, name: &str, parameters: &BTreeMap<String, String>) -> bool {
        panic!("AdditionalHandler::buildJpsWalkableArea is pure virtual in C++")
    }

    /// C++: `bool AdditionalHandler::buildJpsObstacle(const CommonXMLStructure::SumoBaseObject *, const std::string &, const PositionVector &, bool, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds a JuPedSim obstacle using the given values
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_jps_obstacle(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, shape: &PositionVector, geo: bool, name: &str, parameters: &BTreeMap<String, String>) -> bool {
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, true);
        // optional attributes
        let startPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let endPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let lines: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_LINES, &id, &mut parsedOk, Vec::new(), true);
        let personCapacity: i32 = attrs.get_opt::<i32>(SUMO_ATTR_PERSON_CAPACITY, &id, &mut parsedOk, 6, true);
        let parkingLength: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PARKING_LENGTH, &id, &mut parsedOk, 0.0, true);
        let color: RGBColor = attrs.get_opt::<RGBColor>(SUMO_ATTR_COLOR, &id, &mut parsedOk, RGBColor::INVISIBLE, true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut parsedOk, 0.0, true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_BUS_STOP);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_STARTPOS, startPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ENDPOS, endPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_LINES, &lines);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_int_attribute(SUMO_ATTR_PERSON_CAPACITY, personCapacity);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_PARKING_LENGTH, parkingLength);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_color_attribute(SUMO_ATTR_COLOR, &color);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ANGLE, angle);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, true);
        // optional attributes
        let startPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let endPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let lines: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_LINES, &id, &mut parsedOk, Vec::new(), true);
        let personCapacity: i32 = attrs.get_opt::<i32>(SUMO_ATTR_PERSON_CAPACITY, &id, &mut parsedOk, 6, true);
        let parkingLength: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PARKING_LENGTH, &id, &mut parsedOk, 0.0, true);
        let color: RGBColor = attrs.get_opt::<RGBColor>(SUMO_ATTR_COLOR, &id, &mut parsedOk, RGBColor::INVISIBLE, true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut parsedOk, 0.0, true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_TRAIN_STOP);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_STARTPOS, startPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ENDPOS, endPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_LINES, &lines);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_int_attribute(SUMO_ATTR_PERSON_CAPACITY, personCapacity);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_PARKING_LENGTH, parkingLength);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_color_attribute(SUMO_ATTR_COLOR, &color);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ANGLE, angle);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, "", &mut parsedOk, true);
        let position: String = attrs.get::<String>(SUMO_ATTR_POSITION, "", &mut parsedOk, true);
        // optional attributes
        let length: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, "", &mut parsedOk, -1.00, true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, "", &mut parsedOk, false, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_ACCESS, &vec![SUMO_TAG_BUS_STOP, SUMO_TAG_TRAIN_STOP, SUMO_TAG_CONTAINER_STOP], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ACCESS);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_POSITION, &position);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LENGTH, length);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, true);
        // optional attributes
        let startPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let endPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let lines: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_LINES, &id, &mut parsedOk, Vec::new(), true);
        let containerCapacity: i32 = attrs.get_opt::<i32>(SUMO_ATTR_CONTAINER_CAPACITY, &id, &mut parsedOk, 6, true);
        let parkingLength: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PARKING_LENGTH, &id, &mut parsedOk, 0.0, true);
        let color: RGBColor = attrs.get_opt::<RGBColor>(SUMO_ATTR_COLOR, &id, &mut parsedOk, RGBColor::INVISIBLE, true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut parsedOk, 0.0, true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_CONTAINER_STOP);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_STARTPOS, startPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ENDPOS, endPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_LINES, &lines);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_int_attribute(SUMO_ATTR_CONTAINER_CAPACITY, containerCapacity);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_PARKING_LENGTH, parkingLength);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_color_attribute(SUMO_ATTR_COLOR, &color);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ANGLE, angle);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
    //     const std::string chargeType = attrs.getOpt<std::string>(SUMO_ATTR_CHARGETYPE, id.c_str(), parsedOk, ChargeTypes.get_string(ChargeType::NORMAL).unwrap().to_string());
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, true);
        // optional attributes
        let startPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let endPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let lines: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_LINES, &id, &mut parsedOk, Vec::new(), true);
        let chargingPower: f64 = attrs.get_opt::<f64>(SUMO_ATTR_CHARGINGPOWER, &id, &mut parsedOk, 22000.0, true);
        let totalPower: f64 = attrs.get_opt::<f64>(SUMO_ATTR_TOTALPOWER, &id, &mut parsedOk, 0.0, true);
        let efficiency: f64 = attrs.get_opt::<f64>(SUMO_ATTR_EFFICIENCY, &id, &mut parsedOk, 0.95, true);
        let chargeInTransit: bool = attrs.get_opt::<bool>(SUMO_ATTR_CHARGEINTRANSIT, &id, &mut parsedOk, false, true);
        let chargeDelay: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_CHARGEDELAY, &id, &mut parsedOk, 0, true);
        let chargeType: String = attrs.get_opt::<String>(SUMO_ATTR_CHARGETYPE, &id, &mut parsedOk, ChargeTypes.get_string(ChargeType::NORMAL).to_string(), true);
        let waitingTime: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_WAITINGTIME, &id, &mut parsedOk, time2steps(900.), true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        let parkingAreaID: String = attrs.get_opt::<String>(SUMO_ATTR_PARKING_AREA, &id, &mut parsedOk, String::new(), true);
        // check charge type
        if ((chargeType != "normal") && (chargeType != "battery-exchange") && (chargeType != "fuel")) {
            self.write_error(&utils::tlf!("Invalid charge type '%' defined in chargingStation '%'.", chargeType, id));
            parsedOk = false;
        }
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_CHARGING_STATION);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_STARTPOS, startPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ENDPOS, endPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_LINES, &lines);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_CHARGINGPOWER, chargingPower);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_TOTALPOWER, totalPower);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_EFFICIENCY, efficiency);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_CHARGEINTRANSIT, chargeInTransit);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_CHARGEDELAY, chargeDelay);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_CHARGETYPE, &chargeType);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_WAITINGTIME, waitingTime);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_PARKING_AREA, &parkingAreaID);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, true);
        // optional attributes
        let startPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let endPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let departPos: String = attrs.get_opt::<String>(SUMO_ATTR_DEPARTPOS, &id, &mut parsedOk, String::new(), true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let badges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_ACCEPTED_BADGES, &id, &mut parsedOk, Vec::new(), true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        let roadSideCapacity: i32 = attrs.get_opt::<i32>(SUMO_ATTR_ROADSIDE_CAPACITY, &id, &mut parsedOk, 0, true);
        let onRoad: bool = attrs.get_opt::<bool>(SUMO_ATTR_ONROAD, &id, &mut parsedOk, false, true);
        let width: f64 = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, &id, &mut parsedOk, 0.0, true);
        let length: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, &id, &mut parsedOk, 0.0, true);
        let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut parsedOk, 0.0, true);
        let lefthand: bool = attrs.get_opt::<bool>(SUMO_ATTR_LEFTHAND, &id, &mut parsedOk, false, true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_PARKING_AREA);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_STARTPOS, startPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ENDPOS, endPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_DEPARTPOS, &departPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_ACCEPTED_BADGES, &badges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_int_attribute(SUMO_ATTR_ROADSIDE_CAPACITY, roadSideCapacity);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_ONROAD, onRoad);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_WIDTH, width);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LENGTH, length);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ANGLE, angle);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_LEFTHAND, lefthand);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let x: f64 = attrs.get::<f64>(SUMO_ATTR_X, "", &mut parsedOk, true);
        let y: f64 = attrs.get::<f64>(SUMO_ATTR_Y, "", &mut parsedOk, true);
        // optional attributes
        let z: f64 = attrs.get_opt::<f64>(SUMO_ATTR_Z, "", &mut parsedOk, 0.0, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, "", &mut parsedOk, String::new(), true);
        let width: String = attrs.get_opt::<String>(SUMO_ATTR_WIDTH, "", &mut parsedOk, String::new(), true);
        let length: String = attrs.get_opt::<String>(SUMO_ATTR_LENGTH, "", &mut parsedOk, String::new(), true);
        let angle: String = attrs.get_opt::<String>(SUMO_ATTR_ANGLE, "", &mut parsedOk, String::new(), true);
        let slope: f64 = attrs.get_opt::<f64>(SUMO_ATTR_SLOPE, "", &mut parsedOk, 0.0, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_PARKING_SPACE, &vec![SUMO_TAG_PARKING_AREA], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_PARKING_SPACE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_X, x);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_Y, y);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_Z, z);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_WIDTH, &width);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LENGTH, &length);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ANGLE, &angle);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_SLOPE, slope);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, true);
        let position: f64 = attrs.get::<f64>(SUMO_ATTR_POSITION, &id, &mut parsedOk, true);
        let period: SUMOTime = attrs.get_opt_period(&id, &mut parsedOk, sumo_time_max_period(), true);
        let file: String = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut parsedOk, true);
        // optional attributes
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        let nextEdges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_NEXT_EDGES, &id, &mut parsedOk, Vec::new(), true);
        let detectPersons: String = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, "", &mut parsedOk, String::new(), true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        // continue if flag is ok
        if (parsedOk && self.check_detect_persons(SUMO_TAG_E1DETECTOR, &id, &detectPersons)) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_E1DETECTOR);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, position);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_PERIOD, period);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FILE, &file);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_NEXT_EDGES, &nextEdges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_DETECT_PERSONS, &detectPersons);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
    //         writeError(utils::tlf!("'pos', 'endPos' and 'length' cannot be defined together in a single lane area detector."));
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     } else if (attrs.hasAttribute(SUMO_ATTR_LANE) && ((positionDef + endPosDef + lengthDef) < 2)) {
    //         writeError(utils::tlf!("A single lane area detector requires two parameters of those 'pos', 'endPos' and 'length'."));
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        let positionDef: i32 = if attrs.has_attribute(SUMO_ATTR_POSITION) { 1 } else { 0 };
        let endPosDef: i32 = if attrs.has_attribute(SUMO_ATTR_ENDPOS) { 1 } else { 0 };
        let lengthDef: i32 = if attrs.has_attribute(SUMO_ATTR_LENGTH) { 1 } else { 0 };
        // check attributes
        if (attrs.has_attribute(SUMO_ATTR_LANE) && ((positionDef + endPosDef + lengthDef) > 2)) {
            self.write_error(&utils::tlf!("'pos', 'endPos' and 'length' cannot be defined together in a single lane area detector."));
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        } else if (attrs.has_attribute(SUMO_ATTR_LANE) && ((positionDef + endPosDef + lengthDef) < 2)) {
            self.write_error(&utils::tlf!("A single lane area detector requires two parameters of those 'pos', 'endPos' and 'length'."));
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        } else {
            // needed attributes
            let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
            let file: String = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut parsedOk, true);
            // special attributes
            let laneId: String = attrs.get_opt::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, String::new(), true);
            let laneIds: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_LANES, &id, &mut parsedOk, Vec::new(), true);
            let position: f64 = attrs.get_opt::<f64>(SUMO_ATTR_POSITION, &id, &mut parsedOk, 0.0, true);
            let endPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut parsedOk, 0.0, true);
            let length: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, &id, &mut parsedOk, 0.0, true);
            // optional attributes
            let period: SUMOTime = attrs.get_opt_period(&id, &mut parsedOk, sumo_time_max_period(), true);
            let trafficLight: String = attrs.get_opt::<String>(SUMO_ATTR_TLID, &id, &mut parsedOk, String::new(), true);
            let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
            let haltingTimeThreshold: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, &id, &mut parsedOk, time2steps(1.), true);
            let haltingSpeedThreshold: f64 = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, &id, &mut parsedOk, 1.39, true);
            let jamDistThreshold: f64 = attrs.get_opt::<f64>(SUMO_ATTR_JAM_DIST_THRESHOLD, &id, &mut parsedOk, 10.0, true);
            let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
            let nextEdges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_NEXT_EDGES, &id, &mut parsedOk, Vec::new(), true);
            let detectPersons: String = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, "", &mut parsedOk, String::new(), true);
            let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
            let show: bool = attrs.get_opt::<bool>(SUMO_ATTR_SHOW_DETECTOR, &id, &mut parsedOk, true, true);
            // continue if flag is ok
            if (parsedOk && self.check_detect_persons(SUMO_TAG_E2DETECTOR, &id, &detectPersons)) {
                // set tag
                self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_E2DETECTOR);
                // add attributes depending of Lane/Lanes
                if (attrs.has_attribute(SUMO_ATTR_LANE)) {
                    self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
                    if (positionDef == 0) {
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, endPos - length);
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LENGTH, length);
                    } else if (endPosDef == 0) {
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, position);
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LENGTH, length);
                    } else if (lengthDef == 0) {
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, position);
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LENGTH, endPos - position);
                    }
                } else {
                    self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_LANES, &laneIds);
                    self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, position);
                    self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ENDPOS, endPos);
                }
                // add all attributes
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_PERIOD, period);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_TLID, &trafficLight);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FILE, &file);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_NEXT_EDGES, &nextEdges);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_DETECT_PERSONS, &detectPersons);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_HALTING_TIME_THRESHOLD, haltingTimeThreshold);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, haltingSpeedThreshold);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_JAM_DIST_THRESHOLD, jamDistThreshold);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_SHOW_DETECTOR, show);
            } else {
                self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
            }
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let file: String = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut parsedOk, true);
        let period: SUMOTime = attrs.get_opt_period(&id, &mut parsedOk, sumo_time_max_period(), true);
        // optional attributes
        let pos: Position = attrs.get_opt::<Position>(SUMO_ATTR_POSITION, &id, &mut parsedOk, Position::new3(0., 0., 0.), true);
        let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        let nextEdges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_NEXT_EDGES, &id, &mut parsedOk, Vec::new(), true);
        let detectPersons: String = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, "", &mut parsedOk, String::new(), true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let haltingTimeThreshold: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, &id, &mut parsedOk, time2steps(1.), true);
        let haltingSpeedThreshold: f64 = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, &id, &mut parsedOk, 1.39, true);
        let openEntry: bool = attrs.get_opt::<bool>(SUMO_ATTR_OPEN_ENTRY, &id, &mut parsedOk, false, true);
        let expectedArrival: bool = attrs.get_opt::<bool>(SUMO_ATTR_EXPECT_ARRIVAL, &id, &mut parsedOk, false, true);
        // continue if flag is ok
        if (parsedOk && self.check_detect_persons(SUMO_TAG_E3DETECTOR, &id, &detectPersons)) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_E3DETECTOR);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FILE, &file);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_PERIOD, period);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_attribute(SUMO_ATTR_POSITION, &pos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_NEXT_EDGES, &nextEdges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_DETECT_PERSONS, &detectPersons);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_HALTING_TIME_THRESHOLD, haltingTimeThreshold);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, haltingSpeedThreshold);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_OPEN_ENTRY, openEntry);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_EXPECT_ARRIVAL, expectedArrival);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, "", &mut parsedOk, true);
        let position: f64 = attrs.get::<f64>(SUMO_ATTR_POSITION, "", &mut parsedOk, true);
        // optional attributes
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, "", &mut parsedOk, false, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_DET_ENTRY, &vec![SUMO_TAG_E3DETECTOR], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_DET_ENTRY);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, position);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, "", &mut parsedOk, true);
        let position: f64 = attrs.get::<f64>(SUMO_ATTR_POSITION, "", &mut parsedOk, true);
        // optional attributes
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, "", &mut parsedOk, false, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_DET_EXIT, &vec![SUMO_TAG_E3DETECTOR], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_DET_EXIT);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, position);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let laneId: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, true);
        let position: f64 = attrs.get::<f64>(SUMO_ATTR_POSITION, &id, &mut parsedOk, true);
        let file: String = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut parsedOk, true);
        // optional attributes
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        let nextEdges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_NEXT_EDGES, &id, &mut parsedOk, Vec::new(), true);
        let detectPersons: String = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, "", &mut parsedOk, String::new(), true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        // continue if flag is ok
        if (parsedOk && self.check_detect_persons(SUMO_TAG_INSTANT_INDUCTION_LOOP, &id, &detectPersons)) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_INSTANT_INDUCTION_LOOP);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &laneId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, position);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FILE, &file);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_NEXT_EDGES, &nextEdges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_DETECT_PERSONS, &detectPersons);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        // optional attributes
        let shape: PositionVector = attrs.get_opt::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut parsedOk, PositionVector::new(), true);
        let center: Position = attrs.get_opt::<Position>(SUMO_ATTR_CENTER, &id, &mut parsedOk, if shape.len() > 0 { shape.get_centroid() } else { Position::INVALID }, true);
        let fill: bool = attrs.get_opt::<bool>(SUMO_ATTR_FILL, &id, &mut parsedOk, false, true);
        let edges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut parsedOk, Vec::new(), true);
        let color: RGBColor = attrs.get_opt::<RGBColor>(SUMO_ATTR_COLOR, &id, &mut parsedOk, RGBColor::RED, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_TAZ);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_vector_attribute(SUMO_ATTR_SHAPE, &shape);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_attribute(SUMO_ATTR_CENTER, &center);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FILL, fill);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_EDGES, &edges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_color_attribute(SUMO_ATTR_COLOR, &color);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let edgeID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let weight: f64 = attrs.get::<f64>(SUMO_ATTR_WEIGHT, &edgeID, &mut parsedOk, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_TAZSOURCE, &vec![SUMO_TAG_TAZ], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_TAZSOURCE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &edgeID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_WEIGHT, weight);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let edgeID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let weight: f64 = attrs.get::<f64>(SUMO_ATTR_WEIGHT, &edgeID, &mut parsedOk, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_TAZSINK, &vec![SUMO_TAG_TAZ], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_TAZSINK);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &edgeID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_WEIGHT, weight);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let lanes: Vec<String> = attrs.get::<Vec<String>>(SUMO_ATTR_LANES, &id, &mut parsedOk, true);
        // optional attributes
        let pos: Position = attrs.get_opt::<Position>(SUMO_ATTR_POSITION, &id, &mut parsedOk, Position::new3(0., 0., 0.), true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_VSS);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_LANES, &lanes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_attribute(SUMO_ATTR_POSITION, &pos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let time: SUMOTime = attrs.get_sumo_time_reporting(SUMO_ATTR_TIME, "", &mut parsedOk, true);
        // optional attributes
        let speed: f64 = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, "", &mut parsedOk, utils::options::options_cont::get_float("default.speed"), true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_STEP, &vec![SUMO_TAG_VSS], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_STEP);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_TIME, time);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_SPEED, speed);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
    //         writeError(utils::tlf!("Calibrators need either an edge or a lane"));
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // check that frecuency and trafficLight aren't defined together
        if ((attrs.has_attribute(SUMO_ATTR_EDGE) && attrs.has_attribute(SUMO_ATTR_LANE)) || (!attrs.has_attribute(SUMO_ATTR_EDGE) && !attrs.has_attribute(SUMO_ATTR_LANE))) {
            self.write_error(&utils::tlf!("Calibrators need either an edge or a lane"));
            parsedOk = false;
        }
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let pos: f64 = attrs.get::<f64>(SUMO_ATTR_POSITION, &id, &mut parsedOk, true);
        // special attributes
        let edge: String = attrs.get_opt::<String>(SUMO_ATTR_EDGE, &id, &mut parsedOk, String::new(), true);
        let lane: String = attrs.get_opt::<String>(SUMO_ATTR_LANE, &id, &mut parsedOk, String::new(), true);
        // optional attributes
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let period: SUMOTime = attrs.get_opt_period(&id, &mut parsedOk, delta_t(), true);
        let routeProbe: String = attrs.get_opt::<String>(SUMO_ATTR_ROUTEPROBE, &id, &mut parsedOk, String::new(), true);
        let jamThreshold: f64 = attrs.get_opt::<f64>(SUMO_ATTR_JAM_DIST_THRESHOLD, &id, &mut parsedOk, 0.5, true);
        let output: String = attrs.get_opt::<String>(SUMO_ATTR_OUTPUT, &id, &mut parsedOk, String::new(), true);
        let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag depending of edge/lane
            if (attrs.has_attribute(SUMO_ATTR_EDGE)) {
                self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_CALIBRATOR);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_EDGE, &edge);
            } else {
                self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(GNE_TAG_CALIBRATOR_LANE);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &lane);
            }
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, pos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_PERIOD, period);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ROUTEPROBE, &routeProbe);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_JAM_DIST_THRESHOLD, jamThreshold);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_OUTPUT, &output);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
    //             writeError(utils::tlf!("CalibratorFlows need either the attribute vehsPerHour or speed or type (or any combination of these)"));
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // check parent
        if (self.check_calibrator_flow_parents()) {
            // check that frecuency and trafficLight aren't defined together
            if (!attrs.has_attribute(SUMO_ATTR_TYPE) && !attrs.has_attribute(SUMO_ATTR_VEHSPERHOUR) && !attrs.has_attribute(SUMO_ATTR_SPEED)) {
                self.write_error(&utils::tlf!("CalibratorFlows need either the attribute vehsPerHour or speed or type (or any combination of these)"));
                self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
            } else {
                let flowParameter: Ptr<SUMOVehicleParameter> = SUMOVehicleParserHelper::parse_vehicle_attributes(SUMO_TAG_FLOW, attrs, false, true, true, false);
                if !flowParameter.is_null() {
                    // set VPH and speed
                    if (attrs.has_attribute(SUMO_ATTR_VEHSPERHOUR)) {
                        flowParameter.m().repetitionOffset = time2steps(3600. / attrs.get::<f64>(SUMO_ATTR_VEHSPERHOUR, "", &mut parsedOk, true));
                        flowParameter.m().parametersSet |= VEHPARS_VPH_SET;
                    }
                    if (attrs.has_attribute(SUMO_ATTR_SPEED)) {
                        flowParameter.m().calibratorSpeed = attrs.get::<f64>(SUMO_ATTR_SPEED, "", &mut parsedOk, true);
                        flowParameter.m().parametersSet |= VEHPARS_CALIBRATORSPEED_SET;
                    }
                    // set begin and end
                    flowParameter.m().depart = attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut parsedOk, true);
                    flowParameter.m().repetitionEnd = attrs.get_sumo_time_reporting(SUMO_ATTR_END, "", &mut parsedOk, true);
                    if (parsedOk) {
                        // set tag
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_FLOW);
                        // set vehicle parameters
                        let flowId = flowParameter.id.clone();
                        let flowRouteId = flowParameter.routeid.clone();
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().set_vehicle_parameter(Box::new((*flowParameter).clone()), &flowId, &flowRouteId);
                        // C++ deletes flowParameter here (XMLStructure keeps a copy); freeing it made the
                        // calibrator flow arrivalSpeed diverge in oracle case calibrator (measured), so it stays allocated
                        // shortcut: flowParameter leaked instead of deleted, upgrade when the calibrator flow reads the stored copy only
                    } else {
                        self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
                    }
                } else {
                    self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
                }
            }
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let edges: Vec<String> = attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut parsedOk, true);
        // optional attributes
        let pos: Position = attrs.get_opt::<Position>(SUMO_ATTR_POSITION, &id, &mut parsedOk, Position::INVALID, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let probability: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PROB, &id, &mut parsedOk, 1.0, true);
        let mut timeThreshold: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, &id, &mut parsedOk, 0, true);
        let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        let off: bool = attrs.get_opt::<bool>(SUMO_ATTR_OFF, &id, &mut parsedOk, false, true);
        let optional: bool = attrs.get_opt::<bool>(SUMO_ATTR_OPTIONAL, &id, &mut parsedOk, false, true);
        // check attributes
        if (!self.check_negative_2(SUMO_TAG_REROUTER, &id, SUMO_ATTR_PROB, probability, true)) {
            parsedOk = false;
        }
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_REROUTER);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_EDGES, &edges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_attribute(SUMO_ATTR_POSITION, &pos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_PROB, probability);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_HALTING_TIME_THRESHOLD, timeThreshold);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_OFF, off);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_OPTIONAL, optional);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let begin: SUMOTime = attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut parsedOk, true);
        let end: SUMOTime = attrs.get_sumo_time_reporting(SUMO_ATTR_END, "", &mut parsedOk, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_INTERVAL, &vec![SUMO_TAG_REROUTER], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_INTERVAL);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_BEGIN, begin);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_END, end);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let laneID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        // optional attributes
        let disallow: String = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, "", &mut parsedOk, String::new(), true);
        let allow: String = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, "", &mut parsedOk, if disallow.is_empty() { "authority".to_string() } else { String::new() }, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_CLOSING_LANE_REROUTE, &vec![SUMO_TAG_INTERVAL], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_CLOSING_LANE_REROUTE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &laneID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ALLOW, &allow);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_DISALLOW, &disallow);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let edgeID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        // optional attributes
        let disallow: String = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, "", &mut parsedOk, String::new(), true);
        let allow: String = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, "", &mut parsedOk, String::new(), true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_CLOSING_REROUTE, &vec![SUMO_TAG_INTERVAL], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_CLOSING_REROUTE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &edgeID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ALLOW, &allow);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_DISALLOW, &disallow);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let edgeID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let probability: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PROB, "", &mut parsedOk, 1.0, true);
        // check attributes
        if (!self.check_negative_2(SUMO_TAG_DEST_PROB_REROUTE, &edgeID, SUMO_ATTR_PROB, probability, true)) {
            parsedOk = false;
        }
        // check parent
        self.check_parsed_parent(SUMO_TAG_DEST_PROB_REROUTE, &vec![SUMO_TAG_INTERVAL], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_DEST_PROB_REROUTE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &edgeID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_PROB, probability);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let parkingAreaID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let probability: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PROB, "", &mut parsedOk, 1.0, true);
        // check attributes
        if (!self.check_negative_2(SUMO_TAG_PARKING_AREA_REROUTE, &parkingAreaID, SUMO_ATTR_PROB, probability, true)) {
            parsedOk = false;
        }
        // optional attributes
        let visible: bool = attrs.get_opt::<bool>(SUMO_ATTR_VISIBLE, "", &mut parsedOk, false, true);
        // check parent
        self.check_parsed_parent(SUMO_TAG_PARKING_AREA_REROUTE, &vec![SUMO_TAG_INTERVAL], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_PARKING_AREA_REROUTE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &parkingAreaID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_PROB, probability);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_VISIBLE, visible);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let routeID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let probability: f64 = attrs.get_opt::<f64>(SUMO_ATTR_PROB, "", &mut parsedOk, 1.0, true);
        // check attributes
        if (!self.check_negative_2(SUMO_TAG_ROUTE_PROB_REROUTE, &routeID, SUMO_ATTR_PROB, probability, true)) {
            parsedOk = false;
        }
        // check parent
        self.check_parsed_parent(SUMO_TAG_ROUTE_PROB_REROUTE, &vec![SUMO_TAG_INTERVAL], &mut parsedOk);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ROUTE_PROB_REROUTE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &routeID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_PROB, probability);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let edge: String = attrs.get::<String>(SUMO_ATTR_EDGE, &id, &mut parsedOk, true);
        let file: String = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut parsedOk, true);
        let period: SUMOTime = attrs.get_opt_period(&id, &mut parsedOk, sumo_time_max_period(), true);
        let begin: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_BEGIN, &id, &mut parsedOk, -1, true);
        // optional attributes
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        let vehicleTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ROUTEPROBE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_EDGE, &edge);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FILE, &file);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_PERIOD, period);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_BEGIN, begin);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vehicleTypes);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let edgeID: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let mut begin: SUMOTime = attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut parsedOk, true);
        let mut end: SUMOTime = attrs.get_sumo_time_reporting(SUMO_ATTR_END, "", &mut parsedOk, true);
        // optional attributes
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &edgeID, &mut parsedOk, String::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_VAPORIZER);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_EDGE, &edgeID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_BEGIN, begin);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_END, end);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        // optional attributes
        let pos: Position = attrs.get_opt::<Position>(SUMO_ATTR_POSITION, &id, &mut parsedOk, Position::INVALID, true);
        let voltage: f64 = attrs.get_opt::<f64>(SUMO_ATTR_VOLTAGE, &id, &mut parsedOk, 600.0, true);
        let currentLimit: f64 = attrs.get_opt::<f64>(SUMO_ATTR_CURRENTLIMIT, &id, &mut parsedOk, 400.0, true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_TRACTION_SUBSTATION);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_attribute(SUMO_ATTR_POSITION, &pos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_VOLTAGE, voltage);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_CURRENTLIMIT, currentLimit);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let substationID: String = attrs.get::<String>(SUMO_ATTR_SUBSTATIONID, &id, &mut parsedOk, true);
        let laneIDs: Vec<String> = attrs.get::<Vec<String>>(SUMO_ATTR_LANES, &id, &mut parsedOk, true);
        // optional attributes
        let startPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut parsedOk, 0.0, true);
        let endPos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut parsedOk, utils::common::std_defs::INVALID_DOUBLE, true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        let forbiddenInnerLanes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN, "", &mut parsedOk, Vec::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_OVERHEAD_WIRE_SECTION);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_SUBSTATIONID, &substationID);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_LANES, &laneIDs);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_STARTPOS, startPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ENDPOS, endPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN, &forbiddenInnerLanes);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let substationId: String = attrs.get::<String>(SUMO_ATTR_SUBSTATIONID, &id, &mut parsedOk, true);
        let wireClampStart: String = attrs.get::<String>(SUMO_ATTR_OVERHEAD_WIRECLAMP_START, &id, &mut parsedOk, true);
        let wireClampLaneStart: String = attrs.get::<String>(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANESTART, &id, &mut parsedOk, true);
        let wireClampEnd: String = attrs.get::<String>(SUMO_ATTR_OVERHEAD_WIRECLAMP_END, &id, &mut parsedOk, true);
        let wireClampLaneEnd: String = attrs.get::<String>(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANEEND, &id, &mut parsedOk, true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_OVERHEAD_WIRE_CLAMP);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_SUBSTATIONID, &substationId);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_START, &wireClampStart);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANESTART, &wireClampLaneStart);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_END, &wireClampEnd);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_OVERHEAD_WIRECLAMP_LANEEND, &wireClampLaneEnd);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let shapeStr: PositionVector = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut parsedOk, true);
        // optional attributes
        let color: RGBColor = attrs.get_opt::<RGBColor>(SUMO_ATTR_COLOR, &id, &mut parsedOk, RGBColor::RED, true);
        let geo: bool = attrs.get_opt::<bool>(SUMO_ATTR_GEO, &id, &mut parsedOk, false, true);
        let fill: bool = attrs.get_opt::<bool>(SUMO_ATTR_FILL, &id, &mut parsedOk, false, true);
        let lineWidth: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LINEWIDTH, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_LINEWIDTH, true);
        let layer: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LAYER, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_LAYER, true);
        let typeValue: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_TYPE.to_string(), true);
        let imgFile: String = attrs.get_opt::<String>(SUMO_ATTR_IMGFILE, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_IMG_FILE.to_string(), true);
        let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_ANGLE, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_NAME.to_string(), true);
        let height: f64 = attrs.get_opt::<f64>(SUMO_ATTR_HEIGHT, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_HEIGHT, true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_POLY);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_vector_attribute(SUMO_ATTR_SHAPE, &shapeStr);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_color_attribute(SUMO_ATTR_COLOR, &color);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_GEO, geo);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FILL, fill);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LINEWIDTH, lineWidth);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LAYER, layer);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_TYPE, &typeValue);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_IMGFILE, &imgFile);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ANGLE, angle);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_HEIGHT, height);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
    //         writeError(utils::tlf!("X and Y must be be defined together in POIs"));
    //         parsedOk = false;
    //     }
    //     // check that lane and pos are defined together
    //     if ((attrs.hasAttribute(SUMO_ATTR_LANE) && !attrs.hasAttribute(SUMO_ATTR_POSITION)) ||
    //             (!attrs.hasAttribute(SUMO_ATTR_LANE) && attrs.hasAttribute(SUMO_ATTR_POSITION))) {
    //         writeError(utils::tlf!("lane and position must be defined together in POIs"));
    //         parsedOk = false;
    //     }
    //     // check that lon and lat are defined together
    //     if ((attrs.hasAttribute(SUMO_ATTR_LON) && !attrs.hasAttribute(SUMO_ATTR_LAT)) ||
    //             (!attrs.hasAttribute(SUMO_ATTR_LON) && attrs.hasAttribute(SUMO_ATTR_LAT))) {
    //         writeError(utils::tlf!("lon and lat must be be defined together in POIs"));
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
    //     std::string icon = attrs.getOpt<std::string>(SUMO_ATTR_ICON, "", parsedOk, POIIcons.get_string(POIIcon::NONE).unwrap().to_string());
    //     const double layer = attrs.getOpt<double>(SUMO_ATTR_LAYER, id.c_str(), parsedOk, Shape::DEFAULT_LAYER_POI);
    //     const std::string imgFile = attrs.getOpt<std::string>(SUMO_ATTR_IMGFILE, "", parsedOk, Shape::DEFAULT_IMG_FILE);
    //     const double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id.c_str(), parsedOk, Shape::DEFAULT_IMG_WIDTH);
    //     const double height = attrs.getOpt<double>(SUMO_ATTR_HEIGHT, id.c_str(), parsedOk, Shape::DEFAULT_IMG_HEIGHT);
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), parsedOk, Shape::DEFAULT_ANGLE);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), parsedOk, Shape::DEFAULT_NAME);
    //     // check icon
    //     if (!POIIcons.has_string(&icon)) {
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // check that x and y are defined together
        if ((attrs.has_attribute(SUMO_ATTR_X) && !attrs.has_attribute(SUMO_ATTR_Y)) || (!attrs.has_attribute(SUMO_ATTR_X) && attrs.has_attribute(SUMO_ATTR_Y))) {
            self.write_error(&utils::tlf!("X and Y must be be defined together in POIs"));
            parsedOk = false;
        }
        // check that lane and pos are defined together
        if ((attrs.has_attribute(SUMO_ATTR_LANE) && !attrs.has_attribute(SUMO_ATTR_POSITION)) || (!attrs.has_attribute(SUMO_ATTR_LANE) && attrs.has_attribute(SUMO_ATTR_POSITION))) {
            self.write_error(&utils::tlf!("lane and position must be defined together in POIs"));
            parsedOk = false;
        }
        // check that lon and lat are defined together
        if ((attrs.has_attribute(SUMO_ATTR_LON) && !attrs.has_attribute(SUMO_ATTR_LAT)) || (!attrs.has_attribute(SUMO_ATTR_LON) && attrs.has_attribute(SUMO_ATTR_LAT))) {
            self.write_error(&utils::tlf!("lon and lat must be be defined together in POIs"));
            parsedOk = false;
        }
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        // special attributes
        let x: f64 = attrs.get_opt::<f64>(SUMO_ATTR_X, &id, &mut parsedOk, 0.0, true);
        let y: f64 = attrs.get_opt::<f64>(SUMO_ATTR_Y, &id, &mut parsedOk, 0.0, true);
        let lane: String = attrs.get_opt::<String>(SUMO_ATTR_LANE, "", &mut parsedOk, String::new(), true);
        let pos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_POSITION, &id, &mut parsedOk, 0.0, true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut parsedOk, false, true);
        let posLat: f64 = attrs.get_opt::<f64>(SUMO_ATTR_POSITION_LAT, &id, &mut parsedOk, 0.0, true);
        let lon: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LON, &id, &mut parsedOk, 0.0, true);
        let lat: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LAT, &id, &mut parsedOk, 0.0, true);
        // optional attributes
        let color: RGBColor = attrs.get_opt::<RGBColor>(SUMO_ATTR_COLOR, &id, &mut parsedOk, RGBColor::RED, true);
        let typeValue: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, "", &mut parsedOk, crate::utils::shapes::shape::DEFAULT_TYPE.to_string(), true);
        let mut icon: String = attrs.get_opt::<String>(SUMO_ATTR_ICON, "", &mut parsedOk, POIIcons.get_string(POIIcon::NONE).to_string(), true);
        let layer: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LAYER, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_LAYER_POI, true);
        let imgFile: String = attrs.get_opt::<String>(SUMO_ATTR_IMGFILE, "", &mut parsedOk, crate::utils::shapes::shape::DEFAULT_IMG_FILE.to_string(), true);
        let width: f64 = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_IMG_WIDTH, true);
        let height: f64 = attrs.get_opt::<f64>(SUMO_ATTR_HEIGHT, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_IMG_HEIGHT, true);
        let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_ANGLE, true);
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, crate::utils::shapes::shape::DEFAULT_NAME.to_string(), true);
        // check icon
        if (!POIIcons.has_string(&icon)) {
            utils::common::msg_handler::write_warning(&utils::tlf!("Invalid icon % for POI '%', using default", icon, id));
            icon = "none".to_string();
        }
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_POI);
            // add attributes depending of Lane/Lanes
            if (attrs.has_attribute(SUMO_ATTR_X) && attrs.has_attribute(SUMO_ATTR_Y)) {
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_X, x);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_Y, y);
            } else if (attrs.has_attribute(SUMO_ATTR_LANE) && attrs.has_attribute(SUMO_ATTR_POSITION)) {
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_LANE, &lane);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION, pos);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_POSITION_LAT, posLat);
            } else {
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LON, lon);
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LAT, lat);
            }
            // add rest attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_color_attribute(SUMO_ATTR_COLOR, &color);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_TYPE, &typeValue);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ICON, &icon);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_LAYER, layer);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_IMGFILE, &imgFile);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_WIDTH, width);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_HEIGHT, height);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_ANGLE, angle);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let shapeStr: PositionVector = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut parsedOk, true);
        // optional attributes
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(GNE_TAG_JPS_WALKABLEAREA);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_vector_attribute(SUMO_ATTR_SHAPE, &shapeStr);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let shapeStr: PositionVector = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut parsedOk, true);
        // optional attributes
        let name: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut parsedOk, String::new(), true);
        // continue if flag is ok
        if (parsedOk) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(GNE_TAG_JPS_OBSTACLE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_position_vector_attribute(SUMO_ATTR_SHAPE, &shapeStr);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_NAME, &name);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
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
        let parentCalibrator = self.myCommonXMLStructure.get_current_sumo_base_object().get_parent_sumo_base_object();
        if parentCalibrator.is_null() {
            return false;
        }
        let parentRootFile = parentCalibrator.get_parent_sumo_base_object();
        if !parentRootFile.is_null() {
            return false;
        }
        if (parentCalibrator.get_tag() == SUMO_TAG_CALIBRATOR) || (parentCalibrator.get_tag() == GNE_TAG_CALIBRATOR_LANE) {
            true
        } else {
            let msg = utils::tlf!("Calibrator Flows have to be defined within a %.", tag_str(SUMO_TAG_CALIBRATOR));
            self.write_error(&msg)
        }
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

