//! Port of utils/vehicle/SUMOVehicleParameter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `ArrivalLaneDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum ArrivalLaneDefinition {
    DEFAULT = 0,
    CURRENT = 1,
    GIVEN = 2,
    RANDOM = 3,
    FIRST_ALLOWED = 4,
}

/// C++ enum `ArrivalPosDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum ArrivalPosDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    RANDOM = 2,
    CENTER = 3,
    MAX = 4,
}

/// C++ enum `ArrivalPosLatDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum ArrivalPosLatDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    RIGHT = 2,
    CENTER = 3,
    LEFT = 4,
}

/// C++ enum `ArrivalSpeedDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum ArrivalSpeedDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    CURRENT = 2,
}

/// C++ enum `DepartDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum DepartDefinition {
    GIVEN = 0,
    TRIGGERED = 1,
    CONTAINER_TRIGGERED = 2,
    NOW = 3,
    SPLIT = 4,
    BEGIN = 5,
    DEF_MAX = 6,
}

/// C++ enum `DepartLaneDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum DepartLaneDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    RANDOM = 2,
    FREE = 3,
    ALLOWED_FREE = 4,
    BEST_FREE = 5,
    BEST_PROB = 6,
    FIRST_ALLOWED = 7,
}

/// C++ enum `DepartPosDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum DepartPosDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    GIVEN_VEHROUTE = 2,
    RANDOM = 3,
    FREE = 4,
    BASE = 5,
    LAST = 6,
    RANDOM_FREE = 7,
    RANDOM_LOCATION = 8,
    SPLIT_FRONT = 9,
    STOP = 10,
}

/// C++ enum `DepartPosLatDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum DepartPosLatDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    GIVEN_VEHROUTE = 2,
    RIGHT = 3,
    CENTER = 4,
    LEFT = 5,
    RANDOM = 6,
    FREE = 7,
    RANDOM_FREE = 8,
}

/// C++ enum `DepartSpeedDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum DepartSpeedDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    GIVEN_VEHROUTE = 2,
    RANDOM = 3,
    MAX = 4,
    DESIRED = 5,
    LIMIT = 6,
    LAST = 7,
    AVG = 8,
}

/// C++ enum `RouteIndexDefinition`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum RouteIndexDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    RANDOM = 2,
}

/// C++ typedef `std::vector<SUMOVehicleParameter::Stop> StopParVector`
pub type StopParVector = Vec<SUMOVehicleParameter_Stop>;

/// Structure representing possible vehicle parameter
/// C++ class `SUMOVehicleParameter` (utils/vehicle/SUMOVehicleParameter.h)
pub struct SUMOVehicleParameter {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The vehicle tag
    pub tag: SumoXMLTag,
    /// The vehicle's id
    pub id: String,
    /// The vehicle's route id
    pub routeid: String,
    /// The vehicle's type id
    pub vtypeid: String,
    /// The vehicle's color, TraCI may change this
    pub color: RGBColor,
    /// The vehicle's departure time
    pub depart: SUMOTime,
    /// Information how the vehicle shall choose the depart time
    pub departProcedure: DepartDefinition,
    /// (optional) The lane the vehicle shall depart from (index in edge)
    pub departLane: i32,
    /// Information how the vehicle shall choose the lane to depart from
    pub departLaneProcedure: DepartLaneDefinition,
    /// (optional) The position the vehicle shall depart from
    pub departPos: f64,
    /// Information how the vehicle shall choose the departure position
    pub departPosProcedure: DepartPosDefinition,
    /// (optional) The lateral position the vehicle shall depart from
    pub departPosLat: f64,
    /// Information how the vehicle shall choose the lateral departure position
    pub departPosLatProcedure: DepartPosLatDefinition,
    /// (optional) The initial speed of the vehicle
    pub departSpeed: f64,
    /// Information how the vehicle's initial speed shall be chosen
    pub departSpeedProcedure: DepartSpeedDefinition,
    /// (optional) The initial edge within the route of the vehicle
    pub departEdge: i32,
    /// Information how the vehicle's initial edge shall be chosen
    pub departEdgeProcedure: RouteIndexDefinition,
    /// (optional) The lane the vehicle shall arrive on (not used yet)
    pub arrivalLane: i32,
    /// Information how the vehicle shall choose the lane to arrive on
    pub arrivalLaneProcedure: ArrivalLaneDefinition,
    /// (optional) The position the vehicle shall arrive on
    pub arrivalPos: f64,
    /// Information how the vehicle shall choose the arrival position
    pub arrivalPosProcedure: ArrivalPosDefinition,
    /// (optional) The lateral position the vehicle shall arrive on
    pub arrivalPosLat: f64,
    /// Information how the vehicle shall choose the lateral arrival position
    pub arrivalPosLatProcedure: ArrivalPosLatDefinition,
    /// (optional) The final speed of the vehicle (not used yet)
    pub arrivalSpeed: f64,
    /// Information how the vehicle's end speed shall be chosen
    pub arrivalSpeedProcedure: ArrivalSpeedDefinition,
    /// (optional) The final edge within the route of the vehicle
    pub arrivalEdge: i32,
    /// Information how the vehicle's final edge shall be chosen
    pub arrivalEdgeProcedure: RouteIndexDefinition,
    /// The number of times the vehicle shall be repeatedly inserted
    pub repetitionNumber: i64,
    /// The number of times the vehicle was already inserted
    pub repetitionsDone: i64,
    /// The time offset between vehicle reinsertions
    pub repetitionOffset: SUMOTime,
    /// The offset between depart and the time for the next vehicle insertions
    pub repetitionTotalOffset: SUMOTime,
    /// The probability for emitting a vehicle per second
    pub repetitionProbability: f64,
    /// The rate for emitting vehicles with a poisson distribution
    pub poissonRate: f64,
    /// The time at which the flow ends (only needed when using repetitionProbability)
    pub repetitionEnd: SUMOTime,
    /// The vehicle's line (mainly for public transport)
    pub line: String,
    /// The vehicle's origin zone (district)
    pub fromTaz: String,
    /// The vehicle's destination zone (district)
    pub toTaz: String,
    /// List of the stops the vehicle will make, TraCI may add entries here
    pub stops: Vec<SUMOVehicleParameter_Stop>,
    /// List of the via-edges the vehicle must visit
    pub via: Vec<String>,
    /// The parking access rights
    pub parkingBadges: Vec<String>,
    /// The modes a person or container can use
    pub modes: SVCPermissions,
    /// The types of usable (auto-generated) vehicles for a person / container
    pub vTypes: String,
    /// The static number of persons in the vehicle when it departs (not including boarding persons)
    pub personNumber: i32,
    /// The static number of containers in the vehicle when it departs
    pub containerNumber: i32,
    /// individual speedFactor (overriding distribution from vType)
    pub speedFactor: f64,
    /// speed (used by calibrator flows
    pub calibratorSpeed: f64,
    /// bitset of InsertionCheck
    pub insertionChecks: i32,
    /// Information for the router which parameter were set, TraCI may modify this (when changing color)
    pub parametersSet: i64,
}

impl std::ops::Deref for SUMOVehicleParameter {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for SUMOVehicleParameter {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl SUMOVehicleParameter {
    /// C++: `void SUMOVehicleParameter::SUMOVehicleParameter()`
    /// Constructor
    // SUMOVehicleParameter::SUMOVehicleParameter()
    //     : tag(SUMO_TAG_NOTHING), vtypeid(DEFAULT_VTYPE_ID), color(RGBColor::DEFAULT_COLOR),
    //       depart(-1), departProcedure(DepartDefinition::GIVEN),
    //       departLane(0), departLaneProcedure(DepartLaneDefinition::DEFAULT),
    //       departPos(0), departPosProcedure(DepartPosDefinition::DEFAULT),
    //       departPosLat(0), departPosLatProcedure(DepartPosLatDefinition::DEFAULT),
    //       departSpeed(-1), departSpeedProcedure(DepartSpeedDefinition::DEFAULT),
    //       departEdge(0), departEdgeProcedure(RouteIndexDefinition::DEFAULT),
    //       arrivalLane(0), arrivalLaneProcedure(ArrivalLaneDefinition::DEFAULT),
    //       arrivalPos(0), arrivalPosProcedure(ArrivalPosDefinition::DEFAULT),
    //       arrivalPosLat(0), arrivalPosLatProcedure(ArrivalPosLatDefinition::DEFAULT),
    //       arrivalSpeed(-1), arrivalSpeedProcedure(ArrivalSpeedDefinition::DEFAULT),
    //       arrivalEdge(-1), arrivalEdgeProcedure(RouteIndexDefinition::DEFAULT),
    //       repetitionNumber(-1),
    //       repetitionsDone(-1),
    //       repetitionOffset(-1),
    //       repetitionTotalOffset(0),
    //       repetitionProbability(-1),
    //       poissonRate(0),
    //       repetitionEnd(-1),
    //       line(), fromTaz(), toTaz(), personNumber(0), containerNumber(0),
    //       speedFactor(-1),
    //       calibratorSpeed(-1),
    //       insertionChecks((int)InsertionCheck::ALL),
    //       parametersSet(0)
    // { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port SUMOVehicleParameter::SUMOVehicleParameter")
    }

    /// C++: `void SUMOVehicleParameter::~SUMOVehicleParameter() noexcept` [virtual]
    /// Destructor
    // SUMOVehicleParameter::~SUMOVehicleParameter() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SUMOVehicleParameter::~SUMOVehicleParameter")
    }

    /// C++: `bool SUMOVehicleParameter::wasSet(long long) const` [const]
    /// Returns whether the given parameter was set
    // bool wasSet(long long int what) const {
    //         return (parametersSet & what) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_set(&self, what: i64) -> bool {
        todo!("port SUMOVehicleParameter::wasSet")
    }

    /// C++: `void SUMOVehicleParameter::write(OutputDevice &, const OptionsCont &, const SumoXMLTag, const std::string &) const` [const]
    /// Writes the parameters as a beginning element
    // void
    // SUMOVehicleParameter::write(OutputDevice& dev, const OptionsCont& oc, const SumoXMLTag altTag, const std::string& typeID) const {
    //     if (!id.empty()) {
    //         // only used by calibrator flows
    //         dev.openTag(altTag).writeAttr(SUMO_ATTR_ID, id);
    //     }
    //     if (typeID == "") {
    //         if (wasSet(VEHPARS_VTYPE_SET)) {
    //             dev.writeAttr(SUMO_ATTR_TYPE, vtypeid);
    //         }
    //     } else {
    //         dev.writeAttr(SUMO_ATTR_TYPE, typeID);
    //     }
    //     // write depart depending of tag
    //     if (altTag == SUMO_TAG_FLOW
    //             || altTag == SUMO_TAG_PERSONFLOW
    //             || altTag == SUMO_TAG_CONTAINERFLOW
    //             || altTag == GNE_TAG_FLOW_ROUTE
    //             || altTag == GNE_TAG_FLOW_WITHROUTE
    //             || altTag == SUMO_TAG_FLOWSTATE) {
    //         dev.writeAttr(SUMO_ATTR_BEGIN, getDepart());
    //
    //         if (wasSet(VEHPARS_END_SET)) {
    //             dev.writeAttr(SUMO_ATTR_END, time2string(repetitionEnd));
    //         }
    //         if (wasSet(VEHPARS_NUMBER_SET)) {
    //             dev.writeAttr(SUMO_ATTR_NUMBER, repetitionNumber);
    //         }
    //         if (wasSet(VEHPARS_VPH_SET)) {
    //             dev.writeAttr(SUMO_ATTR_PERHOUR, 3600 / STEPS2TIME(repetitionOffset));
    //         }
    //         // netedit uses VEHPARS_POISSON_SET
    //         if (wasSet(VEHPARS_PERIOD_SET) || wasSet(VEHPARS_POISSON_SET)) {
    //             if (repetitionOffset >= 0 && !wasSet(VEHPARS_POISSON_SET)) {
    //                 dev.writeAttr(SUMO_ATTR_PERIOD, StringUtils::adjustDecimalValue(STEPS2TIME(repetitionOffset), 20));
    //             } else {
    //                 dev.writeAttr(SUMO_ATTR_PERIOD, "exp(" + StringUtils::adjustDecimalValue(poissonRate, 20) + ")");
    //             }
    //         }
    //         if (wasSet(VEHPARS_PROB_SET) && repetitionProbability > 0) {
    //             dev.writeAttr(SUMO_ATTR_PROB, StringUtils::adjustDecimalValue(repetitionProbability, 20));
    //         }
    //     } else {
    //         dev.writeAttr(SUMO_ATTR_DEPART, getDepart());
    //     }
    //     // optional parameter
    //     //  departlane
    //     if (wasSet(VEHPARS_DEPARTLANE_SET) && !defaultOptionOverrides(oc, "departlane")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTLANE, getDepartLane());
    //     } else if (oc.exists("departlane") && oc.isSet("departlane")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTLANE, oc.getString("departlane"));
    //     }
    //     //  departpos
    //     if (wasSet(VEHPARS_DEPARTPOS_SET) && !defaultOptionOverrides(oc, "departpos")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTPOS, getDepartPos());
    //     } else if (oc.exists("departpos") && oc.isSet("departpos")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTPOS, oc.getString("departpos"));
    //     }
    //     //  departPosLat
    //     if (wasSet(VEHPARS_DEPARTPOSLAT_SET)) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTPOS_LAT, getDepartPosLat());
    //     }
    //     //  departspeed
    //     if (wasSet(VEHPARS_DEPARTSPEED_SET) && !defaultOptionOverrides(oc, "departspeed")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTSPEED, getDepartSpeed());
    //     } else if (oc.exists("departspeed") && oc.isSet("departspeed")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTSPEED, oc.getString("departspeed"));
    //     }
    //     //  departedge
    //     if (wasSet(VEHPARS_DEPARTEDGE_SET) && !defaultOptionOverrides(oc, "departedge")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTEDGE, getDepartEdge());
    //     } else if (oc.exists("departedge") && oc.isSet("departedge")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_DEPARTEDGE, oc.getString("departedge"));
    //     }
    //     //  arrivallane
    //     if (wasSet(VEHPARS_ARRIVALLANE_SET) && !defaultOptionOverrides(oc, "arrivallane")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALLANE, getArrivalLane());
    //     } else if (oc.exists("arrivallane") && oc.isSet("arrivallane")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALLANE, oc.getString("arrivallane"));
    //     }
    //     //  arrivalpos
    //     if (wasSet(VEHPARS_ARRIVALPOS_SET) && !defaultOptionOverrides(oc, "arrivalpos")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALPOS, getArrivalPos());
    //     } else if (oc.exists("arrivalpos") && oc.isSet("arrivalpos")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALPOS, oc.getString("arrivalpos"));
    //     }
    //     //  arrivalPosLat
    //     if (wasSet(VEHPARS_ARRIVALPOSLAT_SET)) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALPOS_LAT, getArrivalPosLat());
    //     }
    //     //  arrivalspeed
    //     if (wasSet(VEHPARS_ARRIVALSPEED_SET) && !defaultOptionOverrides(oc, "arrivalspeed")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALSPEED, getArrivalSpeed());
    //     } else if (oc.exists("arrivalspeed") && oc.isSet("arrivalspeed")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALSPEED, oc.getString("arrivalspeed"));
    //     }
    //     //  arrivalEdge
    //     if (wasSet(VEHPARS_ARRIVALEDGE_SET) && !defaultOptionOverrides(oc, "arrivaledge") && arrivalEdge >= 0) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALEDGE, getArrivalEdge());
    //     } else if (oc.exists("arrivaledge") && oc.isSet("arrivaledge")) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_ARRIVALEDGE, oc.getString("arrivaledge"));
    //     }
    //     // color
    //     if (wasSet(VEHPARS_COLOR_SET)) {
    //         if (color.isRandom()) {
    //             dev.writeAttr(SUMO_ATTR_COLOR, "random");
    //         } else {
    //             dev.writeAttr(SUMO_ATTR_COLOR, color);
    //         }
    //     }
    //     // line
    //     if (wasSet(VEHPARS_LINE_SET)) {
    //         dev.writeAttr(SUMO_ATTR_LINE, line);
    //     }
    //     // from TAZ
    //     if (wasSet(VEHPARS_FROM_TAZ_SET)) {
    //         dev.writeAttr(SUMO_ATTR_FROM_TAZ, fromTaz);
    //     }
    //     // to TAZ
    //     if (wasSet(VEHPARS_TO_TAZ_SET)) {
    //         dev.writeAttr(SUMO_ATTR_TO_TAZ, toTaz);
    //     }
    //     // person number
    //     if (wasSet(VEHPARS_PERSON_NUMBER_SET)) {
    //         dev.writeAttr(SUMO_ATTR_PERSON_NUMBER, personNumber);
    //     }
    //     // container number
    //     if (wasSet(VEHPARS_CONTAINER_NUMBER_SET)) {
    //         dev.writeAttr(SUMO_ATTR_CONTAINER_NUMBER, containerNumber);
    //     }
    //     // individual speedFactor
    //     if (wasSet(VEHPARS_SPEEDFACTOR_SET)) {
    //         // might be saving state with custom precision
    //         const int precision = dev.getPrecision();
    //         dev.setPrecision(MAX2(gPrecisionRandom, precision));
    //         dev.writeAttr(SUMO_ATTR_SPEEDFACTOR, speedFactor);
    //         dev.setPrecision(precision);
    //     }
    //     // speed (only used by calibrators)
    //     if (wasSet(VEHPARS_CALIBRATORSPEED_SET)) {
    //         dev.writeAttr(SUMO_ATTR_SPEED, calibratorSpeed);
    //     }
    //     // insertionChecks
    //     if (wasSet(VEHPARS_INSERTION_CHECKS_SET) && insertionChecks != (int)InsertionCheck::ALL) {
    //         std::vector<std::string> checks;
    //         if (insertionChecks == (int)InsertionCheck::NONE) {
    //             checks.push_back(toString(InsertionCheck::NONE));
    //         } else {
    //             for (auto it : SUMOXMLDefinitions::InsertionChecks.getValues()) {
    //                 if (((int)it & insertionChecks) != 0) {
    //                     checks.push_back(toString(it));
    //                 }
    //             }
    //         }
    //         dev.writeAttr(SUMO_ATTR_INSERTIONCHECKS, checks);
    //     }
    //     // parking access rights
    //     if (wasSet(VEHPARS_PARKING_BADGES_SET)) {
    //         dev.writeNonEmptyAttr(SUMO_ATTR_PARKING_BADGES, joinToString(parkingBadges, " "));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, oc: &OptionsCont, altTag: SumoXMLTag, typeID: &str) {
        todo!("port SUMOVehicleParameter::write")
    }

    /// C++: `bool SUMOVehicleParameter::defaultOptionOverrides(const OptionsCont &, const std::string &) const` [const]
    /// Returns whether the defaults shall be used
    // bool
    // SUMOVehicleParameter::defaultOptionOverrides(const OptionsCont& oc, const std::string& optionName) const {
    //     return oc.exists(optionName) && oc.isSet(optionName) && oc.getBool("defaults-override");
    // }
    #[allow(unused_variables)]
    pub fn default_option_overrides(&self, oc: &OptionsCont, optionName: &str) -> bool {
        todo!("port SUMOVehicleParameter::defaultOptionOverrides")
    }

    /// C++: `bool SUMOVehicleParameter::parseDepart(const std::string &, const std::string &, const std::string &, SUMOTime &, DepartDefinition &, std::string &, const std::string &)` [static]
    /// Validates a given depart value
    // bool
    // SUMOVehicleParameter::parseDepart(const std::string& val, const std::string& element, const std::string& id,
    //                                   SUMOTime& depart, DepartDefinition& dd, std::string& error, const std::string& attr) {
    //     if (val == "triggered") {
    //         dd = DepartDefinition::TRIGGERED;
    //     } else if (val == "containerTriggered") {
    //         dd = DepartDefinition::CONTAINER_TRIGGERED;
    //     } else if (val == "now") {
    //         // only used via TraCI. depart must be set by the calling code
    //         dd = DepartDefinition::NOW;
    //     } else if (val == "split") {
    //         dd = DepartDefinition::SPLIT;
    //     } else if (val == "begin") {
    //         dd = DepartDefinition::BEGIN;
    //     } else {
    //         try {
    //             depart = string2time(val);
    //             dd = DepartDefinition::GIVEN;
    //             if (depart < 0) {
    //                 error = "Negative " + attr + " time in the definition of " + element + " '" + id + "'.";
    //                 return false;
    //             }
    //         } catch (...) {
    //             if (id.empty()) {
    //                 error = "Invalid " + attr + " time for " + element + ". Must be one of (\"triggered\", \"containerTriggered\", \"now\", or a float >= 0)";
    //             } else {
    //                 error = "Invalid " + attr + " time for " + element + " '" + id + "';\n must be one of (\"triggered\", \"containerTriggered\", \"now\", or a float >= 0)";
    //             }
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_depart(val: &str, element: &str, id: &str, depart: &mut SUMOTime, dd: &mut DepartDefinition, error: &mut String, attr: &str) -> bool {
        todo!("port SUMOVehicleParameter::parseDepart")
    }

    /// C++: `bool SUMOVehicleParameter::parseDepartLane(const std::string &, const std::string &, const std::string &, int &, DepartLaneDefinition &, std::string &)` [static]
    /// Validates a given departLane value
    // bool
    // SUMOVehicleParameter::parseDepartLane(const std::string& val, const std::string& element, const std::string& id,
    //                                       int& lane, DepartLaneDefinition& dld, std::string& error) {
    //     bool ok = true;
    //     lane = 0;
    //     dld = DepartLaneDefinition::GIVEN;
    //     if (val == "random") {
    //         dld = DepartLaneDefinition::RANDOM;
    //     } else if (val == "free") {
    //         dld = DepartLaneDefinition::FREE;
    //     } else if (val == "allowed") {
    //         dld = DepartLaneDefinition::ALLOWED_FREE;
    //     } else if (val == "best") {
    //         dld = DepartLaneDefinition::BEST_FREE;
    //     } else if (val == "best_prob") {
    //         dld = DepartLaneDefinition::BEST_PROB;
    //     } else if (val == "first") {
    //         dld = DepartLaneDefinition::FIRST_ALLOWED;
    //     } else {
    //         try {
    //             lane = StringUtils::toInt(val);
    //             if (lane < 0) {
    //                 ok = false;
    //             }
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid departLane definition for " + element + ". Must be one of (\"random\", \"free\", \"allowed\", \"best\", \"best_prob\", \"first\", or an int>=0)";
    //         } else {
    //             error = "Invalid departLane definition for " + element + " '" + id + "';\n must be one of (\"random\", \"free\", \"allowed\", \"best\", \"best_prob\", \"first\", or an int>=0)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_depart_lane(val: &str, element: &str, id: &str, lane: &mut i32, dld: &mut DepartLaneDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseDepartLane")
    }

    /// C++: `bool SUMOVehicleParameter::parseDepartPos(const std::string &, const std::string &, const std::string &, double &, DepartPosDefinition &, std::string &)` [static]
    /// Validates a given departPos value
    // bool
    // SUMOVehicleParameter::parseDepartPos(const std::string& val, const std::string& element, const std::string& id,
    //                                      double& pos, DepartPosDefinition& dpd, std::string& error) {
    //     bool ok = true;
    //     pos = 0.;
    //     dpd = DepartPosDefinition::GIVEN;
    //     if (val == "random") {
    //         dpd = DepartPosDefinition::RANDOM;
    //     } else if (val == "random_free") {
    //         dpd = DepartPosDefinition::RANDOM_FREE;
    //     } else if (val == "random_location") {
    //         dpd = DepartPosDefinition::RANDOM_LOCATION;
    //     } else if (val == "free") {
    //         dpd = DepartPosDefinition::FREE;
    //     } else if (val == "base") {
    //         dpd = DepartPosDefinition::BASE;
    //     } else if (val == "last") {
    //         dpd = DepartPosDefinition::LAST;
    //     } else if (val == "splitFront") {
    //         dpd = DepartPosDefinition::SPLIT_FRONT;
    //     } else if (val == "stop") {
    //         dpd = DepartPosDefinition::STOP;
    //     } else {
    //         try {
    //             pos = StringUtils::toDouble(val);
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid departPos definition for " + element + ". Must be one of (\"random\", \"random_free\", \"free\", \"base\", \"last\" or a float)";
    //         } else {
    //             error = "Invalid departPos definition for " + element + " '" + id + "';\n must be one of (\"random\", \"random_free\", \"free\", \"base\", \"last\" or a float)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_depart_pos(val: &str, element: &str, id: &str, pos: &mut f64, dpd: &mut DepartPosDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseDepartPos")
    }

    /// C++: `bool SUMOVehicleParameter::parseDepartPosLat(const std::string &, const std::string &, const std::string &, double &, DepartPosLatDefinition &, std::string &)` [static]
    /// Validates a given departPosLat value
    // bool
    // SUMOVehicleParameter::parseDepartPosLat(const std::string& val, const std::string& element, const std::string& id,
    //                                         double& pos, DepartPosLatDefinition& dpd, std::string& error) {
    //     bool ok = true;
    //     pos = 0.;
    //     dpd = DepartPosLatDefinition::GIVEN;
    //     if (val == "random") {
    //         dpd = DepartPosLatDefinition::RANDOM;
    //     } else if (val == "random_free") {
    //         dpd = DepartPosLatDefinition::RANDOM_FREE;
    //     } else if (val == "free") {
    //         dpd = DepartPosLatDefinition::FREE;
    //     } else if (val == "right") {
    //         dpd = DepartPosLatDefinition::RIGHT;
    //     } else if (val == "center") {
    //         dpd = DepartPosLatDefinition::CENTER;
    //     } else if (val == "left") {
    //         dpd = DepartPosLatDefinition::LEFT;
    //     } else {
    //         try {
    //             pos = StringUtils::toDouble(val);
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid departPosLat definition for " + element + ". Must be one of (\"random\", \"random_free\", \"free\", \"right\", \"center\", \"left\", or a float)";
    //         } else {
    //             error = "Invalid departPosLat definition for " + element + " '" + id + "';\n must be one of (\"random\", \"random_free\", \"free\", \"right\", \"center\", \"left\", or a float)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_depart_pos_lat(val: &str, element: &str, id: &str, pos: &mut f64, dpd: &mut DepartPosLatDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseDepartPosLat")
    }

    /// C++: `bool SUMOVehicleParameter::parseDepartSpeed(const std::string &, const std::string &, const std::string &, double &, DepartSpeedDefinition &, std::string &)` [static]
    /// Validates a given departSpeed value
    // bool
    // SUMOVehicleParameter::parseDepartSpeed(const std::string& val, const std::string& element, const std::string& id,
    //                                        double& speed, DepartSpeedDefinition& dsd, std::string& error) {
    //     bool ok = true;
    //     speed = -1.;
    //     dsd = DepartSpeedDefinition::GIVEN;
    //     if (val == "random") {
    //         dsd = DepartSpeedDefinition::RANDOM;
    //     } else if (val == "max") {
    //         dsd = DepartSpeedDefinition::MAX;
    //     } else if (val == "desired") {
    //         dsd = DepartSpeedDefinition::DESIRED;
    //     } else if (val == "speedLimit") {
    //         dsd = DepartSpeedDefinition::LIMIT;
    //     } else if (val == "last") {
    //         dsd = DepartSpeedDefinition::LAST;
    //     } else if (val == "avg") {
    //         dsd = DepartSpeedDefinition::AVG;
    //     } else {
    //         try {
    //             speed = StringUtils::toDouble(val);
    //             if (speed < 0) {
    //                 ok = false;
    //             }
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid departSpeed '" + val + "' definition for " + element + ". Must be one of (\"random\", \"max\", or a float>=0)";
    //         } else {
    //             error = "Invalid departSpeed definition '" + val + "' for " + element + " '" + id + "';\n must be one of (\"random\", \"max\", or a float>=0)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_depart_speed(val: &str, element: &str, id: &str, speed: &mut f64, dsd: &mut DepartSpeedDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseDepartSpeed")
    }

    /// C++: `bool SUMOVehicleParameter::parseRouteIndex(const std::string &, const std::string &, const std::string &, const SumoXMLAttr, int &, RouteIndexDefinition &, std::string &)` [static]
    /// Validates a given departEdge or arrivalEdge value
    // bool
    // SUMOVehicleParameter::parseRouteIndex(const std::string& val, const std::string& element, const std::string& id,
    //                                       const SumoXMLAttr attr, int& edgeIndex, RouteIndexDefinition& rid, std::string& error) {
    //     bool ok = true;
    //     edgeIndex = -1;
    //     rid = RouteIndexDefinition::GIVEN;
    //     if (val == "random") {
    //         rid = RouteIndexDefinition::RANDOM;
    //     } else {
    //         try {
    //             edgeIndex = StringUtils::toInt(val);
    //             if (edgeIndex < 0) {
    //                 ok = false;
    //             }
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid " + toString(attr) + " definition for " + element + ". Must be one of (\"random\", \"free\", or an int>=0)";
    //         } else {
    //             error = "Invalid " + toString(attr) + " definition for " + element + " '" + id + "';\n must be one of (\"random\", \"free\", or an int>=0)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_route_index(val: &str, element: &str, id: &str, attr: SumoXMLAttr, edgeIndex: &mut i32, rid: &mut RouteIndexDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseRouteIndex")
    }

    /// C++: `bool SUMOVehicleParameter::parseArrivalLane(const std::string &, const std::string &, const std::string &, int &, ArrivalLaneDefinition &, std::string &)` [static]
    /// Validates a given arrivalLane value
    // bool
    // SUMOVehicleParameter::parseArrivalLane(const std::string& val, const std::string& element, const std::string& id,
    //                                        int& lane, ArrivalLaneDefinition& ald, std::string& error) {
    //     bool ok = true;
    //     lane = 0;
    //     ald = ArrivalLaneDefinition::GIVEN;
    //     if (val == "current") {
    //         ald = ArrivalLaneDefinition::CURRENT;
    //     } else if (val == "random") {
    //         ald = ArrivalLaneDefinition::RANDOM;
    //     } else if (val == "first") {
    //         ald = ArrivalLaneDefinition::FIRST_ALLOWED;
    //     } else {
    //         try {
    //             lane = StringUtils::toInt(val);
    //             if (lane < 0) {
    //                 ok = false;
    //             }
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid arrivalLane definition for " + element + ". Must be one of (\"current\", or an int>=0)";
    //         } else {
    //             error = "Invalid arrivalLane definition for " + element + " '" + id + "';\n must be one of (\"current\", or an int>=0)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_arrival_lane(val: &str, element: &str, id: &str, lane: &mut i32, ald: &mut ArrivalLaneDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseArrivalLane")
    }

    /// C++: `bool SUMOVehicleParameter::parseArrivalPos(const std::string &, const std::string &, const std::string &, double &, ArrivalPosDefinition &, std::string &)` [static]
    /// Validates a given arrivalPos value
    // bool
    // SUMOVehicleParameter::parseArrivalPos(const std::string& val, const std::string& element, const std::string& id,
    //                                       double& pos, ArrivalPosDefinition& apd, std::string& error) {
    //     bool ok = true;
    //     pos = 0.;
    //     apd = ArrivalPosDefinition::GIVEN;
    //     if (val == "random") {
    //         apd = ArrivalPosDefinition::RANDOM;
    //     } else if (val == "center") {
    //         apd = ArrivalPosDefinition::CENTER;
    //     } else if (val == "max") {
    //         apd = ArrivalPosDefinition::MAX;
    //     } else {
    //         try {
    //             pos = StringUtils::toDouble(val);
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid arrivalPos definition for " + element + ". Must be one of (\"random\", \"max\", or a float)";
    //         } else {
    //             error = "Invalid arrivalPos definition for " + element + " '" + id + "';\n must be one of (\"random\", \"max\", or a float)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_arrival_pos(val: &str, element: &str, id: &str, pos: &mut f64, apd: &mut ArrivalPosDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseArrivalPos")
    }

    /// C++: `bool SUMOVehicleParameter::parseArrivalPosLat(const std::string &, const std::string &, const std::string &, double &, ArrivalPosLatDefinition &, std::string &)` [static]
    /// Validates a given arrivalPosLat value
    // bool
    // SUMOVehicleParameter::parseArrivalPosLat(const std::string& val, const std::string& element, const std::string& id,
    //         double& pos, ArrivalPosLatDefinition& apd, std::string& error) {
    //     bool ok = true;
    //     pos = 0.;
    //     apd = ArrivalPosLatDefinition::GIVEN;
    //     if (val == "right") {
    //         apd = ArrivalPosLatDefinition::RIGHT;
    //     } else if (val == "center") {
    //         apd = ArrivalPosLatDefinition::CENTER;
    //     } else if (val == "left") {
    //         apd = ArrivalPosLatDefinition::LEFT;
    //     } else {
    //         try {
    //             pos = StringUtils::toDouble(val);
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid arrivalPosLat definition for " + element + ". Must be one of (\"right\", \"center\", \"left\", or a float)";
    //         } else {
    //             error = "Invalid arrivalPosLat definition for " + element + " '" + id + "';\n must be one of (\"right\", \"center\", \"left\", or a float)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_arrival_pos_lat(val: &str, element: &str, id: &str, pos: &mut f64, apd: &mut ArrivalPosLatDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseArrivalPosLat")
    }

    /// C++: `bool SUMOVehicleParameter::parseArrivalSpeed(const std::string &, const std::string &, const std::string &, double &, ArrivalSpeedDefinition &, std::string &)` [static]
    /// Validates a given arrivalSpeed value
    // bool
    // SUMOVehicleParameter::parseArrivalSpeed(const std::string& val, const std::string& element, const std::string& id,
    //                                         double& speed, ArrivalSpeedDefinition& asd, std::string& error) {
    //     bool ok = true;
    //     speed = -1.;
    //     asd = ArrivalSpeedDefinition::GIVEN;
    //     if (val == "current") {
    //         asd = ArrivalSpeedDefinition::CURRENT;
    //     } else {
    //         try {
    //             speed = StringUtils::toDouble(val);
    //             if (speed < 0) {
    //                 ok = false;
    //             }
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     if (!ok) {
    //         if (id.empty()) {
    //             error = "Invalid arrivalSpeed definition for " + element + ". Must be one of (\"current\", or a float>=0)";
    //         } else {
    //             error = "Invalid arrivalSpeed definition for " + element + " '" + id + "';\n must be one of (\"current\", or a float>=0)";
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_arrival_speed(val: &str, element: &str, id: &str, speed: &mut f64, asd: &mut ArrivalSpeedDefinition, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parseArrivalSpeed")
    }

    /// C++: `double SUMOVehicleParameter::interpretEdgePos(double, double, SumoXMLAttr, const std::string &, bool)` [static]
    /// Interprets negative edge positions and fits them onto a given edge
    // double
    // SUMOVehicleParameter::interpretEdgePos(double pos, double maximumValue, SumoXMLAttr attr, const std::string& id, bool silent) {
    //     if (pos < 0) {
    //         pos = maximumValue + pos;
    //     }
    //     if (pos > maximumValue && pos != std::numeric_limits<double>::infinity()) {
    //         if (!silent) {
    //             WRITE_WARNINGF(TL("Invalid % % given for %. Using edge end instead."), toString(attr), toString(pos), id);
    //         }
    //         pos = maximumValue;
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn interpret_edge_pos(pos: f64, maximumValue: f64, attr: SumoXMLAttr, id: &str, silent: bool) -> f64 {
        todo!("port SUMOVehicleParameter::interpretEdgePos")
    }

    /// C++: `bool SUMOVehicleParameter::parsePersonModes(const std::string &, const std::string &, const std::string &, SVCPermissions &, std::string &)` [static]
    /// Validates a given person modes value
    // bool
    // SUMOVehicleParameter::parsePersonModes(const std::string& modes, const std::string& element, const std::string& id, SVCPermissions& modeSet, std::string& error) {
    //     // separte modes in different strings, and check if modes are valid
    //     for (StringTokenizer st(modes); st.hasNext();) {
    //         const std::string mode = st.next();
    //         if (mode == "car") {
    //             modeSet |= SVC_PASSENGER;
    //         } else if (mode == "taxi") {
    //             modeSet |= SVC_TAXI;
    //         } else if (mode == "bicycle") {
    //             modeSet |= SVC_BICYCLE;
    //         } else if (mode == "public") {
    //             modeSet |= SVC_BUS;
    //         } else {
    //             if (id.empty()) {
    //                 error = "Unknown person mode '" + mode + "'. Must be a combination of (\"car\", \"taxi\", \"bicycle\" or \"public\")";
    //             } else {
    //                 error = "Unknown person mode '" + mode + "' for " + element + " '" + id + "';\n must be a combination of (\"car\", \"taxi\", \"bicycle\" or \"public\")";
    //             }
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_person_modes(modes: &str, element: &str, id: &str, modeSet: &mut SVCPermissions, error: &mut String) -> bool {
        todo!("port SUMOVehicleParameter::parsePersonModes")
    }

    /// C++: `void SUMOVehicleParameter::parseStopTriggers(const std::vector<std::string> &, bool, Stop &)` [static]
    /// parses stop trigger values
    // void
    // SUMOVehicleParameter::parseStopTriggers(const std::vector<std::string>& triggers, bool expectTrigger, Stop& stop) {
    //     if (triggers.size() == 0 && expectTrigger) {
    //         stop.triggered = true;
    //     }
    //     for (std::string val : triggers) {
    //         if (val == toString(SUMO_TAG_PERSON)) {
    //             stop.triggered = true;
    //         } else if (val == toString(SUMO_TAG_CONTAINER)) {
    //             stop.containerTriggered = true;
    //         } else if (val == toString(SUMO_ATTR_JOIN)) {
    //             stop.joinTriggered = true;
    //         } else {
    //             try {
    //                 stop.triggered = StringUtils::toBool(val);
    //             } catch (BoolFormatException&) {
    //                 WRITE_ERROR(TL("Value of stop attribute 'trigger' must be 'person', 'container', 'join' or a boolean"));
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_stop_triggers(triggers: &Vec<String>, expectTrigger: bool, stop: &mut SUMOVehicleParameter_Stop) {
        todo!("port SUMOVehicleParameter::parseStopTriggers")
    }

    /// C++: `ParkingType SUMOVehicleParameter::parseParkingType(const std::string &)` [static]
    /// parses parking type value
    // ParkingType
    // SUMOVehicleParameter::parseParkingType(const std::string& value) {
    //     if (value == toString(ParkingType::OPPORTUNISTIC)) {
    //         return ParkingType::OPPORTUNISTIC;
    //     } else {
    //         return StringUtils::toBool(value) ? ParkingType::OFFROAD : ParkingType::ONROAD;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parking_type(value: &str) -> ParkingType {
        todo!("port SUMOVehicleParameter::parseParkingType")
    }

    /// C++: `int SUMOVehicleParameter::parseInsertionChecks(const std::string &)` [static]
    /// parses insertion checks
    // int
    // SUMOVehicleParameter::parseInsertionChecks(const std::string& value) {
    //     // first reset insertionChecks
    //     int result = 0;
    //     if (value.empty()) {
    //         return (int)InsertionCheck::ALL;
    //     } else {
    //         // split value in substrinsg
    //         StringTokenizer insertionCheckStrs(value, " ");
    //         while (insertionCheckStrs.hasNext()) {
    //             std::string val = insertionCheckStrs.next();
    //             if (SUMOXMLDefinitions::InsertionChecks.hasString(val)) {
    //                 result |= (int)SUMOXMLDefinitions::InsertionChecks.get(val);
    //             } else {
    //                 throw InvalidArgument("Unknown value '" + val + "' in " + toString(SUMO_ATTR_INSERTIONCHECKS) + ".");
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn parse_insertion_checks(value: &str) -> i32 {
        todo!("port SUMOVehicleParameter::parseInsertionChecks")
    }

    /// C++: `void SUMOVehicleParameter::incrementFlow(double, SumoRNG *)`
    /// increment flow
    // void
    // SUMOVehicleParameter::incrementFlow(double scale, SumoRNG* rng) {
    //     repetitionsDone++;
    //     // equidistant or exponential offset (for poisson distributed arrivals)
    //     if (repetitionProbability < 0) {
    //         if (repetitionOffset >= 0) {
    //             repetitionTotalOffset += (SUMOTime)((double)repetitionOffset / scale);
    //         } else {
    //             assert(poissonRate > 0);
    //             // we need to cache this do avoid double generation of the rng in the TIME2STEPS macro
    //             const double r = RandHelper::randExp(poissonRate, rng);
    //             repetitionTotalOffset += TIME2STEPS(r / scale);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn increment_flow(&mut self, scale: f64, rng: Ptr<SumoRNG>) {
        todo!("port SUMOVehicleParameter::incrementFlow")
    }

    /// C++: `std::string SUMOVehicleParameter::getDepart() const` [const, protected]
    /// obtain depart parameter in string format
    // std::string
    // SUMOVehicleParameter::getDepart() const {
    //     if (departProcedure == DepartDefinition::TRIGGERED) {
    //         return "triggered";
    //     } else if (departProcedure == DepartDefinition::CONTAINER_TRIGGERED) {
    //         return "containerTriggered";
    // //    } else if (departProcedure == DepartDefinition::NOW) {  // TODO check whether this is useful in XML input (currently TraCI only)
    // //        return "now";
    //     } else if (departProcedure == DepartDefinition::SPLIT) {
    //         return "split";
    //     } else if (departProcedure == DepartDefinition::BEGIN) {
    //         return "begin";
    //     } else {
    //         return time2string(depart);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_depart(&self) -> String {
        todo!("port SUMOVehicleParameter::getDepart")
    }

    /// C++: `std::string SUMOVehicleParameter::getDepartLane() const` [const, protected]
    /// obtain depart lane parameter in string format
    // std::string
    // SUMOVehicleParameter::getDepartLane() const {
    //     std::string val;
    //     switch (departLaneProcedure) {
    //         case DepartLaneDefinition::GIVEN:
    //             val = toString(departLane);
    //             break;
    //         case DepartLaneDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case DepartLaneDefinition::FREE:
    //             val = "free";
    //             break;
    //         case DepartLaneDefinition::ALLOWED_FREE:
    //             val = "allowed";
    //             break;
    //         case DepartLaneDefinition::BEST_FREE:
    //             val = "best";
    //             break;
    //         case DepartLaneDefinition::BEST_PROB:
    //             val = "best_prob";
    //             break;
    //         case DepartLaneDefinition::FIRST_ALLOWED:
    //             val = "first";
    //             break;
    //         case DepartLaneDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_lane(&self) -> String {
        todo!("port SUMOVehicleParameter::getDepartLane")
    }

    /// C++: `std::string SUMOVehicleParameter::getDepartPos() const` [const, protected]
    /// obtain depart pos parameter in string format
    // std::string
    // SUMOVehicleParameter::getDepartPos() const {
    //     std::string val;
    //     switch (departPosProcedure) {
    //         case DepartPosDefinition::GIVEN:
    //             val = toString(departPos);
    //             break;
    //         case DepartPosDefinition::GIVEN_VEHROUTE:
    //             val = StringUtils::pruneZeros(toString(departPos, MAX2(gPrecisionRandom, gPrecision)), 2);
    //             break;
    //         case DepartPosDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case DepartPosDefinition::RANDOM_FREE:
    //             val = "random_free";
    //             break;
    //         case DepartPosDefinition::RANDOM_LOCATION:
    //             val = "random_location";
    //             break;
    //         case DepartPosDefinition::FREE:
    //             val = "free";
    //             break;
    //         case DepartPosDefinition::LAST:
    //             val = "last";
    //             break;
    //         case DepartPosDefinition::BASE:
    //             val = "base";
    //             break;
    //         case DepartPosDefinition::SPLIT_FRONT:
    //             val = "splitFront";
    //             break;
    //         case DepartPosDefinition::STOP:
    //             val = "stop";
    //             break;
    //         case DepartPosDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_pos(&self) -> String {
        todo!("port SUMOVehicleParameter::getDepartPos")
    }

    /// C++: `std::string SUMOVehicleParameter::getDepartPosLat() const` [const, protected]
    /// obtain depart pos lat parameter in string format
    // std::string
    // SUMOVehicleParameter::getDepartPosLat() const {
    //     std::string val;
    //     switch (departPosLatProcedure) {
    //         case DepartPosLatDefinition::GIVEN:
    //             val = toString(departPosLat);
    //             break;
    //         case DepartPosLatDefinition::GIVEN_VEHROUTE:
    //             val = StringUtils::pruneZeros(toString(departPosLat, MAX2(gPrecisionRandom, gPrecision)), 2);
    //             break;
    //         case DepartPosLatDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case DepartPosLatDefinition::RANDOM_FREE:
    //             val = "random_free";
    //             break;
    //         case DepartPosLatDefinition::FREE:
    //             val = "free";
    //             break;
    //         case DepartPosLatDefinition::RIGHT:
    //             val = "right";
    //             break;
    //         case DepartPosLatDefinition::CENTER:
    //             val = "center";
    //             break;
    //         case DepartPosLatDefinition::LEFT:
    //             val = "left";
    //             break;
    //         case DepartPosLatDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_pos_lat(&self) -> String {
        todo!("port SUMOVehicleParameter::getDepartPosLat")
    }

    /// C++: `std::string SUMOVehicleParameter::getDepartSpeed() const` [const, protected]
    /// obtain depart speed parameter in string format
    // std::string
    // SUMOVehicleParameter::getDepartSpeed() const {
    //     std::string val;
    //     switch (departSpeedProcedure) {
    //         case DepartSpeedDefinition::GIVEN:
    //             val = toString(departSpeed);
    //             break;
    //         case DepartSpeedDefinition::GIVEN_VEHROUTE:
    //             val = StringUtils::pruneZeros(toString(departSpeed, MAX2(gPrecisionRandom, gPrecision)), 2);
    //             break;
    //         case DepartSpeedDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case DepartSpeedDefinition::MAX:
    //             val = "max";
    //             break;
    //         case DepartSpeedDefinition::DESIRED:
    //             val = "desired";
    //             break;
    //         case DepartSpeedDefinition::LIMIT:
    //             val = "speedLimit";
    //             break;
    //         case DepartSpeedDefinition::LAST:
    //             val = "last";
    //             break;
    //         case DepartSpeedDefinition::AVG:
    //             val = "avg";
    //             break;
    //         case DepartSpeedDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_speed(&self) -> String {
        todo!("port SUMOVehicleParameter::getDepartSpeed")
    }

    /// C++: `std::string SUMOVehicleParameter::getDepartEdge() const` [const, protected]
    /// obtain depart edge parameter in string format
    // std::string
    // SUMOVehicleParameter::getDepartEdge() const {
    //     std::string val;
    //     switch (departEdgeProcedure) {
    //         case RouteIndexDefinition::GIVEN:
    //             val = toString(departEdge);
    //             break;
    //         case RouteIndexDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case RouteIndexDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> String {
        todo!("port SUMOVehicleParameter::getDepartEdge")
    }

    /// C++: `std::string SUMOVehicleParameter::getArrivalLane() const` [const, protected]
    /// obtain arrival lane parameter in string format
    // std::string
    // SUMOVehicleParameter::getArrivalLane() const {
    //     std::string val;
    //     switch (arrivalLaneProcedure) {
    //         case ArrivalLaneDefinition::GIVEN:
    //             val = toString(arrivalLane);
    //             break;
    //         case ArrivalLaneDefinition::CURRENT:
    //             val = "current";
    //             break;
    //         case ArrivalLaneDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case ArrivalLaneDefinition::FIRST_ALLOWED:
    //             val = "first";
    //             break;
    //         case ArrivalLaneDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_lane(&self) -> String {
        todo!("port SUMOVehicleParameter::getArrivalLane")
    }

    /// C++: `std::string SUMOVehicleParameter::getArrivalPos() const` [const, protected]
    /// obtain arrival pos parameter in string format
    // std::string
    // SUMOVehicleParameter::getArrivalPos() const {
    //     std::string val;
    //     switch (arrivalPosProcedure) {
    //         case ArrivalPosDefinition::GIVEN:
    //             val = toString(arrivalPos);
    //             break;
    //         case ArrivalPosDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case ArrivalPosDefinition::CENTER:
    //             val = "center";
    //             break;
    //         case ArrivalPosDefinition::MAX:
    //             val = "max";
    //             break;
    //         case ArrivalPosDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> String {
        todo!("port SUMOVehicleParameter::getArrivalPos")
    }

    /// C++: `std::string SUMOVehicleParameter::getArrivalPosLat() const` [const, protected]
    /// obtain arrival pos lat parameter in string format
    // std::string
    // SUMOVehicleParameter::getArrivalPosLat() const {
    //     std::string val;
    //     switch (arrivalPosLatProcedure) {
    //         case ArrivalPosLatDefinition::GIVEN:
    //             val = toString(arrivalPosLat);
    //             break;
    //         case ArrivalPosLatDefinition::RIGHT:
    //             val = "right";
    //             break;
    //         case ArrivalPosLatDefinition::CENTER:
    //             val = "center";
    //             break;
    //         case ArrivalPosLatDefinition::LEFT:
    //             val = "left";
    //             break;
    //         case ArrivalPosLatDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_pos_lat(&self) -> String {
        todo!("port SUMOVehicleParameter::getArrivalPosLat")
    }

    /// C++: `std::string SUMOVehicleParameter::getArrivalSpeed() const` [const, protected]
    /// obtain arrival speed parameter in string format
    // std::string
    // SUMOVehicleParameter::getArrivalSpeed() const {
    //     std::string val;
    //     switch (arrivalSpeedProcedure) {
    //         case ArrivalSpeedDefinition::GIVEN:
    //             val = toString(arrivalSpeed);
    //             break;
    //         case ArrivalSpeedDefinition::CURRENT:
    //             val = "current";
    //             break;
    //         case ArrivalSpeedDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_speed(&self) -> String {
        todo!("port SUMOVehicleParameter::getArrivalSpeed")
    }

    /// C++: `std::string SUMOVehicleParameter::getArrivalEdge() const` [const, protected]
    /// obtain arrival edge parameter in string format
    // std::string
    // SUMOVehicleParameter::getArrivalEdge() const {
    //     std::string val;
    //     switch (arrivalEdgeProcedure) {
    //         case RouteIndexDefinition::GIVEN:
    //             val = toString(arrivalEdge);
    //             break;
    //         case RouteIndexDefinition::RANDOM:
    //             val = "random";
    //             break;
    //         case RouteIndexDefinition::DEFAULT:
    //         default:
    //             break;
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_edge(&self) -> String {
        todo!("port SUMOVehicleParameter::getArrivalEdge")
    }

    /// C++: `std::string SUMOVehicleParameter::getInsertionChecks() const` [const, protected]
    /// get insertion checks in string format
    // std::string
    // SUMOVehicleParameter::getInsertionChecks() const {
    //     if ((insertionChecks == 0) || (insertionChecks == (int)InsertionCheck::ALL)) {
    //         return SUMOXMLDefinitions::InsertionChecks.getString(InsertionCheck::ALL);
    //     } else {
    //         std::vector<std::string> insertionChecksStrs;
    //         const auto insertionCheckValues = SUMOXMLDefinitions::InsertionChecks.getValues();
    //         // iterate over values
    //         for (const auto insertionCheckValue : insertionCheckValues) {
    //             if ((insertionCheckValue != InsertionCheck::ALL) && (insertionChecks & (int)insertionCheckValue) != 0) {
    //                 insertionChecksStrs.push_back(SUMOXMLDefinitions::InsertionChecks.getString(insertionCheckValue));
    //             }
    //         }
    //         return toString(insertionChecksStrs);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_insertion_checks(&self) -> String {
        todo!("port SUMOVehicleParameter::getInsertionChecks")
    }

    /// C++: `bool SUMOVehicleParameter::areInsertionChecksValid(const std::string &) const` [const, protected]
    /// check if given insertion checks are valid
    // bool
    // SUMOVehicleParameter::areInsertionChecksValid(const std::string& value) const {
    //     if (value.empty()) {
    //         return true;
    //     } else {
    //         // split value in substrinsg
    //         StringTokenizer valueStrs(value, " ");
    //         // iterate over values
    //         while (valueStrs.hasNext()) {
    //             if (!SUMOXMLDefinitions::InsertionChecks.hasString(valueStrs.next())) {
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn are_insertion_checks_valid(&self, value: &str) -> bool {
        todo!("port SUMOVehicleParameter::areInsertionChecksValid")
    }

}

/// Definition of vehicle stop (position and duration)
/// C++ class `SUMOVehicleParameter::Stop` (utils/vehicle/SUMOVehicleParameter.h)
pub struct SUMOVehicleParameter_Stop {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The edge to stop at
    pub edge: String,
    /// The lane to stop at
    pub lane: String,
    /// (Optional) bus stop if one is assigned to the stop
    pub busstop: String,
    /// (Optional) container stop if one is assigned to the stop
    pub containerstop: String,
    /// (Optional) parking area if one is assigned to the stop
    pub parkingarea: String,
    /// (Optional) charging station if one is assigned to the stop
    pub chargingStation: String,
    /// (Optional) overhead line segment if one is assigned to the stop
    pub overheadWireSegment: String,
    /// The stopping position start
    pub startPos: f64,
    /// The stopping position end
    pub endPos: f64,
    /// The (expected) time at which the vehicle reaches the stop
    pub arrival: SUMOTime,
    /// The stopping duration
    pub duration: SUMOTime,
    pub durationDist: Unmapped /* Distribution_Parameterized */,
    /// The time at which the vehicle may continue its journey
    pub until: SUMOTime,
    /// The maximum time extension for boarding / loading
    pub extension: SUMOTime,
    /// The earliest pickup time for a taxi stop
    pub waitUntil: SUMOTime,
    /// whether an arriving person lets the vehicle continue
    pub triggered: bool,
    /// whether an arriving container lets the vehicle continue
    pub containerTriggered: bool,
    /// whether an joined vehicle lets this vehicle continue
    pub joinTriggered: bool,
    /// whether the vehicle is removed from the net while stopping
    pub parking: ParkingType,
    /// IDs of persons the vehicle has to wait for until departing
    pub awaitedPersons: BTreeSet<String>,
    /// IDs of persons or containers that may board/load at this stop
    pub permitted: BTreeSet<String>,
    /// IDs of containers the vehicle has to wait for until departing
    pub awaitedContainers: BTreeSet<String>,
    /// enable or disable friendly position (used by netedit)
    pub friendlyPos: bool,
    /// act Type (only used by Persons) (used by netedit)
    pub actType: String,
    /// id of the trip within a cyclical public transport route
    pub tripId: String,
    /// the new line id of the trip within a cyclical public transport route
    pub line: String,
    /// the id of the vehicle (train portion) that splits of upon reaching this stop
    pub split: String,
    /// the id of the vehicle (train portion) to which this vehicle shall be joined
    pub join: String,
    /// the speed at which this stop counts as reached (waypoint mode)
    pub speed: f64,
    /// the lateral offset when stopping
    pub posLat: f64,
    /// whether the stop may be skipped
    pub onDemand: bool,
    /// priority for weighting/skipping stops
    pub priority: f64,
    /// transfer time if there shall be a jump from this stop to the next route edge
    pub jump: SUMOTime,
    /// earlierst jump end if there shall be a jump from this stop to the next route edge
    pub jumpUntil: SUMOTime,
    /// the time at which this stop was reached
    pub started: SUMOTime,
    /// the time at which this stop was ended
    pub ended: SUMOTime,
    /// lanes and positions connected to this stop (only used by duarouter where Stop is used to store stopping places)
    pub accessPos: Vec<Unmapped /* std::tuple<std::string, double, double> */>,
    /// at which position in the stops list
    pub index: i32,
    /// at which position within the route (only used for state saving)
    pub routeIndex: i32,
    /// Information for the output which parameter were set
    pub parametersSet: i32,
    /// Whether this stop was triggered by a collision
    pub collision: bool,
    /// Whether this stop was triggered by a car failure / mechanical problem / lack of energy
    pub breakDown: bool,
}

impl std::ops::Deref for SUMOVehicleParameter_Stop {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for SUMOVehicleParameter_Stop {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl SUMOVehicleParameter_Stop {
    /// C++: `void Stop::~Stop() noexcept` [virtual]
    /// Destructor
    // virtual ~Stop() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Stop::~Stop")
    }

    /// C++: `void Stop::write(OutputDevice &, const bool, const bool) const` [const]
    /// Writes the stop as XML
    // void
    // SUMOVehicleParameter::Stop::write(OutputDevice& dev, const bool close, const bool writeTagAndParents) const {
    //     if (writeTagAndParents) {
    //         dev.openTag(SUMO_TAG_STOP);
    //         if (busstop != "") {
    //             dev.writeAttr(SUMO_ATTR_BUS_STOP, busstop);
    //         }
    //         if (containerstop != "") {
    //             dev.writeAttr(SUMO_ATTR_CONTAINER_STOP, containerstop);
    //         }
    //         if (chargingStation != "") {
    //             dev.writeAttr(SUMO_ATTR_CHARGING_STATION, chargingStation);
    //         }
    //         if (parkingarea != "") {
    //             dev.writeAttr(SUMO_ATTR_PARKING_AREA, parkingarea);
    //         }
    //         if ((busstop == "") && (containerstop == "") && (parkingarea == "") && (chargingStation == "")) {
    //             if (lane != "") {
    //                 dev.writeAttr(SUMO_ATTR_LANE, lane);
    //             } else {
    //                 dev.writeAttr(SUMO_ATTR_EDGE, edge);
    //             }
    //             if ((parametersSet & STOP_START_SET) != 0) {
    //                 dev.writeAttr(SUMO_ATTR_STARTPOS, startPos);
    //             }
    //             if ((parametersSet & STOP_END_SET) != 0) {
    //                 dev.writeAttr(SUMO_ATTR_ENDPOS, endPos);
    //             }
    //         }
    //     }
    //     if (index > 0) {
    //         dev.writeAttr(SUMO_ATTR_INDEX, index);
    //     }
    //     if ((parametersSet & STOP_POSLAT_SET) != 0 && posLat != INVALID_DOUBLE) {
    //         dev.writeAttr(SUMO_ATTR_POSITION_LAT, posLat);
    //     }
    //     if ((parametersSet & STOP_ARRIVAL_SET) && (arrival >= 0)) {
    //         dev.writeAttr(SUMO_ATTR_ARRIVAL, time2string(arrival));
    //     }
    //     if ((parametersSet & STOP_DURATION_SET) && (duration >= 0)) {
    //         dev.writeAttr(SUMO_ATTR_DURATION, time2string(duration));
    //     }
    //     if ((parametersSet & STOP_UNTIL_SET) && (until >= 0)) {
    //         dev.writeAttr(SUMO_ATTR_UNTIL, time2string(until));
    //     }
    //     if ((parametersSet & STOP_STARTED_SET) && (started >= 0)) {
    //         dev.writeAttr(SUMO_ATTR_STARTED, time2string(started));
    //     }
    //     if ((parametersSet & STOP_ENDED_SET) && (ended >= 0)) {
    //         dev.writeAttr(SUMO_ATTR_ENDED, time2string(ended));
    //     }
    //     if ((parametersSet & STOP_EXTENSION_SET) && (extension >= 0)) {
    //         dev.writeAttr(SUMO_ATTR_EXTENSION, time2string(extension));
    //     }
    //     if ((parametersSet & STOP_TRIGGER_SET) != 0) {
    //         const std::vector<std::string> triggers = getTriggers();
    //         if (triggers.size() > 0) {
    //             dev.writeAttr(SUMO_ATTR_TRIGGERED, triggers);
    //         }
    //     }
    //     if ((parametersSet & STOP_PARKING_SET) != 0) {
    //         dev.writeAttr(SUMO_ATTR_PARKING, parking);
    //     }
    //     if ((parametersSet & STOP_EXPECTED_SET) != 0 && awaitedPersons.size() > 0) {
    //         dev.writeAttr(SUMO_ATTR_EXPECTED, awaitedPersons);
    //     }
    //     if ((parametersSet & STOP_PERMITTED_SET) != 0 && permitted.size() > 0) {
    //         dev.writeAttr(SUMO_ATTR_PERMITTED, permitted);
    //     }
    //     if ((parametersSet & STOP_EXPECTED_CONTAINERS_SET) != 0 && awaitedContainers.size() > 0) {
    //         dev.writeAttr(SUMO_ATTR_EXPECTED_CONTAINERS, awaitedContainers);
    //     }
    //     if ((parametersSet & STOP_TRIP_ID_SET) != 0) {
    //         dev.writeAttr(SUMO_ATTR_TRIP_ID, tripId);
    //     }
    //     if ((parametersSet & STOP_LINE_SET) != 0) {
    //         dev.writeAttr(SUMO_ATTR_LINE, line);
    //     }
    //     if ((parametersSet & STOP_SPLIT_SET) != 0) {
    //         dev.writeAttr(SUMO_ATTR_SPLIT, split);
    //     }
    //     if ((parametersSet & STOP_JOIN_SET) != 0) {
    //         dev.writeAttr(SUMO_ATTR_JOIN, join);
    //     }
    //     if ((parametersSet & STOP_SPEED_SET) != 0) {
    //         dev.writeAttr(SUMO_ATTR_SPEED, speed);
    //     }
    //     if ((parametersSet & STOP_ONDEMAND_SET) != 0) {
    //         dev.writeAttr(SUMO_ATTR_ONDEMAND, onDemand);
    //     }
    //     if ((parametersSet & STOP_JUMP_SET) != 0 && jump >= 0) {
    //         dev.writeAttr(SUMO_ATTR_JUMP, time2string(jump));
    //     }
    //     if ((parametersSet & STOP_JUMP_UNTIL_SET) != 0 && jumpUntil >= 0) {
    //         dev.writeAttr(SUMO_ATTR_JUMP_UNTIL, time2string(jumpUntil));
    //     }
    //     if ((parametersSet & STOP_PRIORITY_SET) != 0 && priority >= 0) {
    //         dev.writeAttr(SUMO_ATTR_PRIORITY, priority);
    //     }
    //     if (collision) {
    //         dev.writeAttr(SUMO_ATTR_COLLISION, collision);
    //     }
    //     // only write friendly position if is true
    //     if (friendlyPos) {
    //         dev.writeAttr(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //     }
    //     // only write act type if isn't empty
    //     if (!actType.empty()) {
    //         dev.writeAttr(SUMO_ATTR_ACTTYPE, actType);
    //     }
    //     if (close) {
    //         // the user is closing the stop it is responsible for writing params
    //         writeParams(dev);
    //         dev.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, close: bool, writeTagAndParents: bool) {
        todo!("port Stop::write")
    }

    /// C++: `std::vector<std::string> Stop::getStoppingPlaceIDs() const` [const]
    /// return list of stopping place ids
    // std::vector<std::string>
    // SUMOVehicleParameter::Stop::getStoppingPlaceIDs() const {
    //     std::vector<std::string> result;
    //     if (busstop != "") {
    //         result.push_back(busstop);
    //     }
    //     if (containerstop != "") {
    //         result.push_back(containerstop);
    //     }
    //     if (chargingStation != "") {
    //         result.push_back(chargingStation);
    //     }
    //     if (parkingarea != "") {
    //         result.push_back(parkingarea);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_i_ds(&self) -> Vec<String> {
        todo!("port Stop::getStoppingPlaceIDs")
    }

    /// C++: `std::vector<std::string> Stop::getTriggers() const` [const]
    /// write trigger attribute
    // std::vector<std::string>
    // SUMOVehicleParameter::Stop::getTriggers() const {
    //     std::vector<std::string> triggers;
    //     if (triggered) {
    //         triggers.push_back(toString(SUMO_TAG_PERSON));
    //     }
    //     if (containerTriggered) {
    //         triggers.push_back(toString(SUMO_TAG_CONTAINER));
    //     }
    //     if (joinTriggered) {
    //         triggers.push_back(toString(SUMO_ATTR_JOIN));
    //     }
    //     return triggers;
    // }
    #[allow(unused_variables)]
    pub fn get_triggers(&self) -> Vec<String> {
        todo!("port Stop::getTriggers")
    }

    /// C++: `int Stop::getFlags() const` [const]
    /// return flags as per Vehicle::getStops
    // int
    // SUMOVehicleParameter::Stop::getFlags() const {
    //     return (((parking == ParkingType::OFFROAD) ? 1 : 0) +
    //             (triggered ? 2 : 0) +
    //             (containerTriggered ? 4 : 0) +
    //             (busstop != "" ? 8 : 0) +
    //             (containerstop != "" ? 16 : 0) +
    //             (chargingStation != "" ? 32 : 0) +
    //             (parkingarea != "" ? 64 : 0) +
    //             (overheadWireSegment != "" ? 128 : 0));
    // }
    #[allow(unused_variables)]
    pub fn get_flags(&self) -> i32 {
        todo!("port Stop::getFlags")
    }

}

