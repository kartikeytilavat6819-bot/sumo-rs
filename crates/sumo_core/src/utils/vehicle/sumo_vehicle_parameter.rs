//! Port of utils/vehicle/SUMOVehicleParameter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::common::sumo_vehicle_class::{DEFAULT_VTYPE_ID, SVC_BICYCLE, SVC_BUS, SVC_PASSENGER, SVC_TAXI};
use utils::common::std_defs::{g_precision, g_precision_random, INVALID_DOUBLE};
use utils::common::string_tokenizer::StringTokenizer;
use utils::common::string_utils::{adjust_decimal_value, prune_zeros, to_bool, to_double, to_int};
use utils::common::time::{string2time, steps2time, time2steps, time2string};
use utils::common::to_string::{f64_to_string_prec, to_string};
use utils::iodevices::output_device::AttrName;
use utils::xml::sumo_xml_definitions::{
    InsertionCheck, InsertionChecks, ParkingType, Tags, GNE_TAG_FLOW_ROUTE, GNE_TAG_FLOW_WITHROUTE,
    SUMO_ATTR_ACTTYPE, SUMO_ATTR_ARRIVAL, SUMO_ATTR_ARRIVALEDGE, SUMO_ATTR_ARRIVALLANE, SUMO_ATTR_ARRIVALPOS,
    SUMO_ATTR_ARRIVALPOS_LAT, SUMO_ATTR_ARRIVALSPEED, SUMO_ATTR_BEGIN, SUMO_ATTR_BUS_STOP, SUMO_ATTR_CHARGING_STATION,
    SUMO_ATTR_COLLISION, SUMO_ATTR_COLOR, SUMO_ATTR_CONTAINER_NUMBER, SUMO_ATTR_CONTAINER_STOP, SUMO_ATTR_DEPART,
    SUMO_ATTR_DEPARTEDGE, SUMO_ATTR_DEPARTLANE, SUMO_ATTR_DEPARTPOS, SUMO_ATTR_DEPARTPOS_LAT, SUMO_ATTR_DEPARTSPEED,
    SUMO_ATTR_DURATION, SUMO_ATTR_EDGE, SUMO_ATTR_END, SUMO_ATTR_ENDED, SUMO_ATTR_ENDPOS, SUMO_ATTR_EXPECTED,
    SUMO_ATTR_EXPECTED_CONTAINERS, SUMO_ATTR_EXTENSION, SUMO_ATTR_FRIENDLY_POS, SUMO_ATTR_FROM_TAZ, SUMO_ATTR_ID,
    SUMO_ATTR_INDEX, SUMO_ATTR_INSERTIONCHECKS, SUMO_ATTR_JOIN, SUMO_ATTR_JUMP, SUMO_ATTR_JUMP_UNTIL, SUMO_ATTR_LANE,
    SUMO_ATTR_LINE, SUMO_ATTR_NUMBER, SUMO_ATTR_ONDEMAND, SUMO_ATTR_PARKING, SUMO_ATTR_PARKING_AREA,
    SUMO_ATTR_PARKING_BADGES, SUMO_ATTR_PERIOD, SUMO_ATTR_PERHOUR, SUMO_ATTR_PERMITTED, SUMO_ATTR_PERSON_NUMBER,
    SUMO_ATTR_POSITION_LAT, SUMO_ATTR_PRIORITY, SUMO_ATTR_PROB, SUMO_ATTR_SPEED, SUMO_ATTR_SPEEDFACTOR,
    SUMO_ATTR_SPLIT, SUMO_ATTR_STARTED, SUMO_ATTR_STARTPOS, SUMO_ATTR_STOPTIME, SUMO_ATTR_TIMELOSS, SUMO_ATTR_TO_TAZ,
    SUMO_ATTR_TRIGGERED, SUMO_ATTR_TRIP_ID, SUMO_ATTR_TYPE, SUMO_ATTR_UNTIL, SUMO_TAG_CONTAINER, SUMO_TAG_CONTAINERFLOW,
    SUMO_TAG_FLOW, SUMO_TAG_FLOWSTATE, SUMO_TAG_NOTHING, SUMO_TAG_PERSON, SUMO_TAG_PERSONFLOW, SUMO_TAG_STOP,
};

// C++ #define values from utils/vehicle/SUMOVehicleParameter.h (the parametersSet bits)
const VEHPARS_COLOR_SET: i64 = 1;
const VEHPARS_VTYPE_SET: i64 = 2;
const VEHPARS_DEPARTLANE_SET: i64 = 2 << 1;
const VEHPARS_DEPARTPOS_SET: i64 = 2 << 2;
const VEHPARS_DEPARTSPEED_SET: i64 = 2 << 3;
const VEHPARS_END_SET: i64 = 2 << 4;
const VEHPARS_NUMBER_SET: i64 = 2 << 5;
const VEHPARS_PERIOD_SET: i64 = 2 << 6;
const VEHPARS_VPH_SET: i64 = 2 << 7;
const VEHPARS_PROB_SET: i64 = 2 << 8;
const VEHPARS_POISSON_SET: i64 = 2 << 9;
const VEHPARS_ARRIVALLANE_SET: i64 = 2 << 11;
const VEHPARS_ARRIVALPOS_SET: i64 = 2 << 12;
const VEHPARS_ARRIVALSPEED_SET: i64 = 2 << 13;
const VEHPARS_LINE_SET: i64 = 2 << 14;
const VEHPARS_FROM_TAZ_SET: i64 = 2 << 15;
const VEHPARS_TO_TAZ_SET: i64 = 2 << 16;
const VEHPARS_PERSON_NUMBER_SET: i64 = 2 << 19;
const VEHPARS_CONTAINER_NUMBER_SET: i64 = 2 << 20;
const VEHPARS_DEPARTPOSLAT_SET: i64 = 2 << 21;
const VEHPARS_ARRIVALPOSLAT_SET: i64 = 2 << 22;
const VEHPARS_SPEEDFACTOR_SET: i64 = 2 << 24;
const VEHPARS_DEPARTEDGE_SET: i64 = 2 << 25;
const VEHPARS_ARRIVALEDGE_SET: i64 = 2 << 26;
const VEHPARS_CALIBRATORSPEED_SET: i64 = 2 << 27;
const VEHPARS_PARKING_BADGES_SET: i64 = 2 << 30;
const VEHPARS_INSERTION_CHECKS_SET: i64 = 2 << 31;

// C++ #define values for the Stop parametersSet bits
pub const STOP_START_SET: i32 = 1;
pub const STOP_END_SET: i32 = 2;
pub const STOP_DURATION_SET: i32 = 2 << 1;
pub const STOP_UNTIL_SET: i32 = 2 << 2;
const STOP_EXTENSION_SET: i32 = 2 << 3;
const STOP_TRIGGER_SET: i32 = 2 << 4;
const STOP_PARKING_SET: i32 = 2 << 5;
const STOP_EXPECTED_SET: i32 = 2 << 6;
const STOP_EXPECTED_CONTAINERS_SET: i32 = 2 << 8;
const STOP_TRIP_ID_SET: i32 = 2 << 9;
const STOP_LINE_SET: i32 = 2 << 10;
pub const STOP_SPEED_SET: i32 = 2 << 11;
const STOP_SPLIT_SET: i32 = 2 << 12;
const STOP_JOIN_SET: i32 = 2 << 13;
const STOP_ARRIVAL_SET: i32 = 2 << 14;
const STOP_PERMITTED_SET: i32 = 2 << 15;
const STOP_ENDED_SET: i32 = 2 << 16;
pub const STOP_STARTED_SET: i32 = 2 << 17;
const STOP_POSLAT_SET: i32 = 2 << 18;
const STOP_ONDEMAND_SET: i32 = 2 << 19;
pub const STOP_JUMP_SET: i32 = 2 << 20;
const STOP_JUMP_UNTIL_SET: i32 = 2 << 21;
const STOP_PRIORITY_SET: i32 = 2 << 22;

/// C++ `OptionsCont::exists(name) && OptionsCont::isSet(name)` (the set check cannot fail once the option exists)
fn option_exists_and_set(oc: &OptionsCont, name: &str) -> bool {
    oc.exists(name) && oc.is_set(name, true).unwrap_or(false)
}

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
#[derive(Clone)]
#[repr(C)]
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
        SUMOVehicleParameter {
            base: Parameterised::new(),
            tag: SUMO_TAG_NOTHING,
            id: String::new(),
            routeid: String::new(),
            vtypeid: DEFAULT_VTYPE_ID.to_string(),
            color: RGBColor::DEFAULT_COLOR,
            depart: -1,
            departProcedure: DepartDefinition::GIVEN,
            departLane: 0,
            departLaneProcedure: DepartLaneDefinition::DEFAULT,
            departPos: 0.0,
            departPosProcedure: DepartPosDefinition::DEFAULT,
            departPosLat: 0.0,
            departPosLatProcedure: DepartPosLatDefinition::DEFAULT,
            departSpeed: -1.0,
            departSpeedProcedure: DepartSpeedDefinition::DEFAULT,
            departEdge: 0,
            departEdgeProcedure: RouteIndexDefinition::DEFAULT,
            arrivalLane: 0,
            arrivalLaneProcedure: ArrivalLaneDefinition::DEFAULT,
            arrivalPos: 0.0,
            arrivalPosProcedure: ArrivalPosDefinition::DEFAULT,
            arrivalPosLat: 0.0,
            arrivalPosLatProcedure: ArrivalPosLatDefinition::DEFAULT,
            arrivalSpeed: -1.0,
            arrivalSpeedProcedure: ArrivalSpeedDefinition::DEFAULT,
            arrivalEdge: -1,
            arrivalEdgeProcedure: RouteIndexDefinition::DEFAULT,
            repetitionNumber: -1,
            repetitionsDone: -1,
            repetitionOffset: -1,
            repetitionTotalOffset: 0,
            repetitionProbability: -1.0,
            poissonRate: 0.0,
            repetitionEnd: -1,
            line: String::new(),
            fromTaz: String::new(),
            toTaz: String::new(),
            stops: Vec::new(),
            via: Vec::new(),
            parkingBadges: Vec::new(),
            modes: 0,
            vTypes: String::new(),
            personNumber: 0,
            containerNumber: 0,
            speedFactor: -1.0,
            calibratorSpeed: -1.0,
            insertionChecks: InsertionCheck::ALL as i32,
            parametersSet: 0,
        }
    }

    /// C++: `void SUMOVehicleParameter::~SUMOVehicleParameter() noexcept` [virtual]
    /// Destructor
    // SUMOVehicleParameter::~SUMOVehicleParameter() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor body is empty; the members are released by Rust's drop glue
    }

    /// C++: `bool SUMOVehicleParameter::wasSet(long long) const` [const]
    /// Returns whether the given parameter was set
    // bool wasSet(long long int what) const {
    //         return (parametersSet & what) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_set(&self, what: i64) -> bool {
        (self.parametersSet & what) != 0
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
        if !self.id.is_empty() {
            // only used by calibrator flows
            dev.open_tag_id(altTag).write_attr(SUMO_ATTR_ID, &self.id);
        }
        if typeID == "" {
            if self.was_set(VEHPARS_VTYPE_SET) {
                dev.write_attr(SUMO_ATTR_TYPE, &self.vtypeid);
            }
        } else {
            dev.write_attr(SUMO_ATTR_TYPE, typeID);
        }
        // write depart depending of tag
        if altTag == SUMO_TAG_FLOW
            || altTag == SUMO_TAG_PERSONFLOW
            || altTag == SUMO_TAG_CONTAINERFLOW
            || altTag == GNE_TAG_FLOW_ROUTE
            || altTag == GNE_TAG_FLOW_WITHROUTE
            || altTag == SUMO_TAG_FLOWSTATE
        {
            dev.write_attr(SUMO_ATTR_BEGIN, &self.get_depart());

            if self.was_set(VEHPARS_END_SET) {
                dev.write_attr(SUMO_ATTR_END, &time2string(self.repetitionEnd));
            }
            if self.was_set(VEHPARS_NUMBER_SET) {
                dev.write_attr(SUMO_ATTR_NUMBER, &self.repetitionNumber);
            }
            if self.was_set(VEHPARS_VPH_SET) {
                dev.write_attr(SUMO_ATTR_PERHOUR, &(3600.0 / steps2time(self.repetitionOffset)));
            }
            // netedit uses VEHPARS_POISSON_SET
            if self.was_set(VEHPARS_PERIOD_SET) || self.was_set(VEHPARS_POISSON_SET) {
                if self.repetitionOffset >= 0 && !self.was_set(VEHPARS_POISSON_SET) {
                    dev.write_attr(SUMO_ATTR_PERIOD, &adjust_decimal_value(steps2time(self.repetitionOffset), 20));
                } else {
                    dev.write_attr(SUMO_ATTR_PERIOD, &format!("exp({})", adjust_decimal_value(self.poissonRate, 20)));
                }
            }
            if self.was_set(VEHPARS_PROB_SET) && self.repetitionProbability > 0.0 {
                dev.write_attr(SUMO_ATTR_PROB, &adjust_decimal_value(self.repetitionProbability, 20));
            }
        } else {
            dev.write_attr(SUMO_ATTR_DEPART, &self.get_depart());
        }
        // optional parameter
        //  departlane
        if self.was_set(VEHPARS_DEPARTLANE_SET) && !self.default_option_overrides(oc, "departlane") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTLANE, &self.get_depart_lane());
        } else if option_exists_and_set(oc, "departlane") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTLANE, &oc.get_string("departlane").unwrap_or_default());
        }
        //  departpos
        if self.was_set(VEHPARS_DEPARTPOS_SET) && !self.default_option_overrides(oc, "departpos") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTPOS, &self.get_depart_pos());
        } else if option_exists_and_set(oc, "departpos") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTPOS, &oc.get_string("departpos").unwrap_or_default());
        }
        //  departPosLat
        if self.was_set(VEHPARS_DEPARTPOSLAT_SET) {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTPOS_LAT, &self.get_depart_pos_lat());
        }
        //  departspeed
        if self.was_set(VEHPARS_DEPARTSPEED_SET) && !self.default_option_overrides(oc, "departspeed") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTSPEED, &self.get_depart_speed());
        } else if option_exists_and_set(oc, "departspeed") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTSPEED, &oc.get_string("departspeed").unwrap_or_default());
        }
        //  departedge
        if self.was_set(VEHPARS_DEPARTEDGE_SET) && !self.default_option_overrides(oc, "departedge") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTEDGE, &self.get_depart_edge());
        } else if option_exists_and_set(oc, "departedge") {
            dev.write_non_empty_attr(SUMO_ATTR_DEPARTEDGE, &oc.get_string("departedge").unwrap_or_default());
        }
        //  arrivallane
        if self.was_set(VEHPARS_ARRIVALLANE_SET) && !self.default_option_overrides(oc, "arrivallane") {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALLANE, &self.get_arrival_lane());
        } else if option_exists_and_set(oc, "arrivallane") {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALLANE, &oc.get_string("arrivallane").unwrap_or_default());
        }
        //  arrivalpos
        if self.was_set(VEHPARS_ARRIVALPOS_SET) && !self.default_option_overrides(oc, "arrivalpos") {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALPOS, &self.get_arrival_pos());
        } else if option_exists_and_set(oc, "arrivalpos") {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALPOS, &oc.get_string("arrivalpos").unwrap_or_default());
        }
        //  arrivalPosLat
        if self.was_set(VEHPARS_ARRIVALPOSLAT_SET) {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALPOS_LAT, &self.get_arrival_pos_lat());
        }
        //  arrivalspeed
        if self.was_set(VEHPARS_ARRIVALSPEED_SET) && !self.default_option_overrides(oc, "arrivalspeed") {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALSPEED, &self.get_arrival_speed());
        } else if option_exists_and_set(oc, "arrivalspeed") {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALSPEED, &oc.get_string("arrivalspeed").unwrap_or_default());
        }
        //  arrivalEdge
        if self.was_set(VEHPARS_ARRIVALEDGE_SET) && !self.default_option_overrides(oc, "arrivaledge") && self.arrivalEdge >= 0 {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALEDGE, &self.get_arrival_edge());
        } else if option_exists_and_set(oc, "arrivaledge") {
            dev.write_non_empty_attr(SUMO_ATTR_ARRIVALEDGE, &oc.get_string("arrivaledge").unwrap_or_default());
        }
        // color
        if self.was_set(VEHPARS_COLOR_SET) {
            if self.color.is_random() {
                dev.write_attr(SUMO_ATTR_COLOR, "random");
            } else {
                dev.write_attr(SUMO_ATTR_COLOR, &self.color);
            }
        }
        // line
        if self.was_set(VEHPARS_LINE_SET) {
            dev.write_attr(SUMO_ATTR_LINE, &self.line);
        }
        // from TAZ
        if self.was_set(VEHPARS_FROM_TAZ_SET) {
            dev.write_attr(SUMO_ATTR_FROM_TAZ, &self.fromTaz);
        }
        // to TAZ
        if self.was_set(VEHPARS_TO_TAZ_SET) {
            dev.write_attr(SUMO_ATTR_TO_TAZ, &self.toTaz);
        }
        // person number
        if self.was_set(VEHPARS_PERSON_NUMBER_SET) {
            dev.write_attr(SUMO_ATTR_PERSON_NUMBER, &self.personNumber);
        }
        // container number
        if self.was_set(VEHPARS_CONTAINER_NUMBER_SET) {
            dev.write_attr(SUMO_ATTR_CONTAINER_NUMBER, &self.containerNumber);
        }
        // individual speedFactor
        if self.was_set(VEHPARS_SPEEDFACTOR_SET) {
            // might be saving state with custom precision
            let precision = dev.get_precision();
            dev.set_precision(std::cmp::max(g_precision_random(), precision));
            dev.write_attr(SUMO_ATTR_SPEEDFACTOR, &self.speedFactor);
            dev.set_precision(precision);
        }
        // speed (only used by calibrators)
        if self.was_set(VEHPARS_CALIBRATORSPEED_SET) {
            dev.write_attr(SUMO_ATTR_SPEED, &self.calibratorSpeed);
        }
        // insertionChecks
        if self.was_set(VEHPARS_INSERTION_CHECKS_SET) && self.insertionChecks != InsertionCheck::ALL as i32 {
            let mut checks: Vec<String> = Vec::new();
            if self.insertionChecks == InsertionCheck::NONE as i32 {
                checks.push(InsertionChecks.get_string(InsertionCheck::NONE).to_string());
            } else {
                for it in InsertionChecks.get_values() {
                    if (it as i32 & self.insertionChecks) != 0 {
                        checks.push(InsertionChecks.get_string(it).to_string());
                    }
                }
            }
            dev.write_attr(SUMO_ATTR_INSERTIONCHECKS, &checks);
        }
        // parking access rights
        if self.was_set(VEHPARS_PARKING_BADGES_SET) {
            dev.write_non_empty_attr(SUMO_ATTR_PARKING_BADGES, &self.parkingBadges.join(" "));
        }
    }

    /// C++: `bool SUMOVehicleParameter::defaultOptionOverrides(const OptionsCont &, const std::string &) const` [const]
    /// Returns whether the defaults shall be used
    // bool
    // SUMOVehicleParameter::defaultOptionOverrides(const OptionsCont& oc, const std::string& optionName) const {
    //     return oc.exists(optionName) && oc.isSet(optionName) && oc.getBool("defaults-override");
    // }
    #[allow(unused_variables)]
    pub fn default_option_overrides(&self, oc: &OptionsCont, optionName: &str) -> bool {
        option_exists_and_set(oc, optionName) && oc.get_bool("defaults-override").unwrap_or(false)
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
        if val == "triggered" {
            *dd = DepartDefinition::TRIGGERED;
        } else if val == "containerTriggered" {
            *dd = DepartDefinition::CONTAINER_TRIGGERED;
        } else if val == "now" {
            // only used via TraCI. depart must be set by the calling code
            *dd = DepartDefinition::NOW;
        } else if val == "split" {
            *dd = DepartDefinition::SPLIT;
        } else if val == "begin" {
            *dd = DepartDefinition::BEGIN;
        } else {
            match string2time(val) {
                Ok(t) => {
                    *depart = t;
                    *dd = DepartDefinition::GIVEN;
                    if *depart < 0 {
                        *error = format!("Negative {} time in the definition of {} '{}'.", attr, element, id);
                        return false;
                    }
                }
                Err(_) => {
                    if id.is_empty() {
                        *error = format!("Invalid {} time for {}. Must be one of (\"triggered\", \"containerTriggered\", \"now\", or a float >= 0)", attr, element);
                    } else {
                        *error = format!("Invalid {} time for {} '{}';\n must be one of (\"triggered\", \"containerTriggered\", \"now\", or a float >= 0)", attr, element, id);
                    }
                    return false;
                }
            }
        }
        true
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
        let mut ok = true;
        *lane = 0;
        *dld = DepartLaneDefinition::GIVEN;
        if val == "random" {
            *dld = DepartLaneDefinition::RANDOM;
        } else if val == "free" {
            *dld = DepartLaneDefinition::FREE;
        } else if val == "allowed" {
            *dld = DepartLaneDefinition::ALLOWED_FREE;
        } else if val == "best" {
            *dld = DepartLaneDefinition::BEST_FREE;
        } else if val == "best_prob" {
            *dld = DepartLaneDefinition::BEST_PROB;
        } else if val == "first" {
            *dld = DepartLaneDefinition::FIRST_ALLOWED;
        } else {
            match to_int(val) {
                Ok(v) => {
                    *lane = v;
                    if *lane < 0 {
                        ok = false;
                    }
                }
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid departLane definition for {}. Must be one of (\"random\", \"free\", \"allowed\", \"best\", \"best_prob\", \"first\", or an int>=0)", element);
            } else {
                *error = format!("Invalid departLane definition for {} '{}';\n must be one of (\"random\", \"free\", \"allowed\", \"best\", \"best_prob\", \"first\", or an int>=0)", element, id);
            }
        }
        ok
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
        let mut ok = true;
        *pos = 0.0;
        *dpd = DepartPosDefinition::GIVEN;
        if val == "random" {
            *dpd = DepartPosDefinition::RANDOM;
        } else if val == "random_free" {
            *dpd = DepartPosDefinition::RANDOM_FREE;
        } else if val == "random_location" {
            *dpd = DepartPosDefinition::RANDOM_LOCATION;
        } else if val == "free" {
            *dpd = DepartPosDefinition::FREE;
        } else if val == "base" {
            *dpd = DepartPosDefinition::BASE;
        } else if val == "last" {
            *dpd = DepartPosDefinition::LAST;
        } else if val == "splitFront" {
            *dpd = DepartPosDefinition::SPLIT_FRONT;
        } else if val == "stop" {
            *dpd = DepartPosDefinition::STOP;
        } else {
            match to_double(val) {
                Ok(v) => *pos = v,
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid departPos definition for {}. Must be one of (\"random\", \"random_free\", \"free\", \"base\", \"last\" or a float)", element);
            } else {
                *error = format!("Invalid departPos definition for {} '{}';\n must be one of (\"random\", \"random_free\", \"free\", \"base\", \"last\" or a float)", element, id);
            }
        }
        ok
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
        let mut ok = true;
        *pos = 0.0;
        *dpd = DepartPosLatDefinition::GIVEN;
        if val == "random" {
            *dpd = DepartPosLatDefinition::RANDOM;
        } else if val == "random_free" {
            *dpd = DepartPosLatDefinition::RANDOM_FREE;
        } else if val == "free" {
            *dpd = DepartPosLatDefinition::FREE;
        } else if val == "right" {
            *dpd = DepartPosLatDefinition::RIGHT;
        } else if val == "center" {
            *dpd = DepartPosLatDefinition::CENTER;
        } else if val == "left" {
            *dpd = DepartPosLatDefinition::LEFT;
        } else {
            match to_double(val) {
                Ok(v) => *pos = v,
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid departPosLat definition for {}. Must be one of (\"random\", \"random_free\", \"free\", \"right\", \"center\", \"left\", or a float)", element);
            } else {
                *error = format!("Invalid departPosLat definition for {} '{}';\n must be one of (\"random\", \"random_free\", \"free\", \"right\", \"center\", \"left\", or a float)", element, id);
            }
        }
        ok
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
        let mut ok = true;
        *speed = -1.0;
        *dsd = DepartSpeedDefinition::GIVEN;
        if val == "random" {
            *dsd = DepartSpeedDefinition::RANDOM;
        } else if val == "max" {
            *dsd = DepartSpeedDefinition::MAX;
        } else if val == "desired" {
            *dsd = DepartSpeedDefinition::DESIRED;
        } else if val == "speedLimit" {
            *dsd = DepartSpeedDefinition::LIMIT;
        } else if val == "last" {
            *dsd = DepartSpeedDefinition::LAST;
        } else if val == "avg" {
            *dsd = DepartSpeedDefinition::AVG;
        } else {
            match to_double(val) {
                Ok(v) => {
                    *speed = v;
                    if *speed < 0.0 {
                        ok = false;
                    }
                }
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid departSpeed '{}' definition for {}. Must be one of (\"random\", \"max\", or a float>=0)", val, element);
            } else {
                *error = format!("Invalid departSpeed definition '{}' for {} '{}';\n must be one of (\"random\", \"max\", or a float>=0)", val, element, id);
            }
        }
        ok
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
        let mut ok = true;
        *edgeIndex = -1;
        *rid = RouteIndexDefinition::GIVEN;
        if val == "random" {
            *rid = RouteIndexDefinition::RANDOM;
        } else {
            match to_int(val) {
                Ok(v) => {
                    *edgeIndex = v;
                    if *edgeIndex < 0 {
                        ok = false;
                    }
                }
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid {} definition for {}. Must be one of (\"random\", \"free\", or an int>=0)", attr.attr_name(), element);
            } else {
                *error = format!("Invalid {} definition for {} '{}';\n must be one of (\"random\", \"free\", or an int>=0)", attr.attr_name(), element, id);
            }
        }
        ok
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
        let mut ok = true;
        *lane = 0;
        *ald = ArrivalLaneDefinition::GIVEN;
        if val == "current" {
            *ald = ArrivalLaneDefinition::CURRENT;
        } else if val == "random" {
            *ald = ArrivalLaneDefinition::RANDOM;
        } else if val == "first" {
            *ald = ArrivalLaneDefinition::FIRST_ALLOWED;
        } else {
            match to_int(val) {
                Ok(v) => {
                    *lane = v;
                    if *lane < 0 {
                        ok = false;
                    }
                }
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid arrivalLane definition for {}. Must be one of (\"current\", or an int>=0)", element);
            } else {
                *error = format!("Invalid arrivalLane definition for {} '{}';\n must be one of (\"current\", or an int>=0)", element, id);
            }
        }
        ok
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
        let mut ok = true;
        *pos = 0.0;
        *apd = ArrivalPosDefinition::GIVEN;
        if val == "random" {
            *apd = ArrivalPosDefinition::RANDOM;
        } else if val == "center" {
            *apd = ArrivalPosDefinition::CENTER;
        } else if val == "max" {
            *apd = ArrivalPosDefinition::MAX;
        } else {
            match to_double(val) {
                Ok(v) => *pos = v,
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid arrivalPos definition for {}. Must be one of (\"random\", \"max\", or a float)", element);
            } else {
                *error = format!("Invalid arrivalPos definition for {} '{}';\n must be one of (\"random\", \"max\", or a float)", element, id);
            }
        }
        ok
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
        let mut ok = true;
        *pos = 0.0;
        *apd = ArrivalPosLatDefinition::GIVEN;
        if val == "right" {
            *apd = ArrivalPosLatDefinition::RIGHT;
        } else if val == "center" {
            *apd = ArrivalPosLatDefinition::CENTER;
        } else if val == "left" {
            *apd = ArrivalPosLatDefinition::LEFT;
        } else {
            match to_double(val) {
                Ok(v) => *pos = v,
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid arrivalPosLat definition for {}. Must be one of (\"right\", \"center\", \"left\", or a float)", element);
            } else {
                *error = format!("Invalid arrivalPosLat definition for {} '{}';\n must be one of (\"right\", \"center\", \"left\", or a float)", element, id);
            }
        }
        ok
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
        let mut ok = true;
        *speed = -1.0;
        *asd = ArrivalSpeedDefinition::GIVEN;
        if val == "current" {
            *asd = ArrivalSpeedDefinition::CURRENT;
        } else {
            match to_double(val) {
                Ok(v) => {
                    *speed = v;
                    if *speed < 0.0 {
                        ok = false;
                    }
                }
                Err(_) => ok = false,
            }
        }
        if !ok {
            if id.is_empty() {
                *error = format!("Invalid arrivalSpeed definition for {}. Must be one of (\"current\", or a float>=0)", element);
            } else {
                *error = format!("Invalid arrivalSpeed definition for {} '{}';\n must be one of (\"current\", or a float>=0)", element, id);
            }
        }
        ok
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
        let mut pos = pos;
        if pos < 0.0 {
            pos = maximumValue + pos;
        }
        if pos > maximumValue && pos != f64::INFINITY {
            if !silent {
                write_warning(&format!("Invalid {} {} given for {}. Using edge end instead.", attr.attr_name(), to_string(&pos), id));
            }
            pos = maximumValue;
        }
        pos
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
        // separte modes in different strings, and check if modes are valid
        let mut st = StringTokenizer::new(modes);
        while st.has_next() {
            let mode = st.next().unwrap_or_default();
            if mode == "car" {
                *modeSet |= SVC_PASSENGER;
            } else if mode == "taxi" {
                *modeSet |= SVC_TAXI;
            } else if mode == "bicycle" {
                *modeSet |= SVC_BICYCLE;
            } else if mode == "public" {
                *modeSet |= SVC_BUS;
            } else {
                if id.is_empty() {
                    *error = format!("Unknown person mode '{}'. Must be a combination of (\"car\", \"taxi\", \"bicycle\" or \"public\")", mode);
                } else {
                    *error = format!("Unknown person mode '{}' for {} '{}';\n must be a combination of (\"car\", \"taxi\", \"bicycle\" or \"public\")", mode, element, id);
                }
                return false;
            }
        }
        true
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
        if triggers.is_empty() && expectTrigger {
            stop.triggered = true;
        }
        for val in triggers.iter() {
            if val == Tags.get_string(SUMO_TAG_PERSON) {
                stop.triggered = true;
            } else if val == Tags.get_string(SUMO_TAG_CONTAINER) {
                stop.containerTriggered = true;
            } else if val == SUMO_ATTR_JOIN.attr_name() {
                stop.joinTriggered = true;
            } else {
                match to_bool(val) {
                    Ok(b) => stop.triggered = b,
                    // BoolFormatException in C++
                    Err(_) => write_error("Value of stop attribute 'trigger' must be 'person', 'container', 'join' or a boolean"),
                }
            }
        }
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
        if value == "opportunistic" {
            ParkingType::OPPORTUNISTIC
        } else {
            // C++ throws if the value is not a boolean; the uncaught exception is a panic here
            match to_bool(value) {
                Ok(true) => ParkingType::OFFROAD,
                Ok(false) => ParkingType::ONROAD,
                Err(e) => panic!("{:?}", e),
            }
        }
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
        Self::try_parse_insertion_checks(value).unwrap_or_else(|e| panic!("{}", e))
    }

    /// C++ `parseInsertionChecks` with its `throw InvalidArgument` returned as `Err`
    #[allow(unused_variables)]
    pub fn try_parse_insertion_checks(value: &str) -> Result<i32, utils::common::util_exceptions::ProcessError> {
        // first reset insertionChecks
        let mut result: i32 = 0;
        if value.is_empty() {
            return Ok(InsertionCheck::ALL as i32);
        } else {
            // split value in substrings
            let mut insertionCheckStrs = StringTokenizer::with_token(value, " ", true);
            while insertionCheckStrs.has_next() {
                let val = insertionCheckStrs.next().unwrap_or_default();
                if InsertionChecks.has_string(&val) {
                    if let Ok(check) = InsertionChecks.get(&val) {
                        result |= check as i32;
                    }
                } else {
                    return Err(utils::common::util_exceptions::ProcessError::InvalidArgument(format!(
                        "Unknown value '{}' in {}.",
                        val,
                        SUMO_ATTR_INSERTIONCHECKS.attr_name()
                    )));
                }
            }
        }
        Ok(result)
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
        self.repetitionsDone += 1;
        // equidistant or exponential offset (for poisson distributed arrivals)
        if self.repetitionProbability < 0.0 {
            if self.repetitionOffset >= 0 {
                self.repetitionTotalOffset += (self.repetitionOffset as f64 / scale) as SUMOTime;
            } else {
                // C++ assert(poissonRate > 0) is compiled out of release builds
                // we need to cache this do avoid double generation of the rng in the TIME2STEPS macro
                let r = rng.m().rand_exp(self.poissonRate);
                self.repetitionTotalOffset += time2steps(r / scale);
            }
        }
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
        if self.departProcedure == DepartDefinition::TRIGGERED {
            "triggered".to_string()
        } else if self.departProcedure == DepartDefinition::CONTAINER_TRIGGERED {
            "containerTriggered".to_string()
        } else if self.departProcedure == DepartDefinition::SPLIT {
            "split".to_string()
        } else if self.departProcedure == DepartDefinition::BEGIN {
            "begin".to_string()
        } else {
            time2string(self.depart)
        }
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
        match self.departLaneProcedure {
            DepartLaneDefinition::GIVEN => to_string(&self.departLane),
            DepartLaneDefinition::RANDOM => "random".to_string(),
            DepartLaneDefinition::FREE => "free".to_string(),
            DepartLaneDefinition::ALLOWED_FREE => "allowed".to_string(),
            DepartLaneDefinition::BEST_FREE => "best".to_string(),
            DepartLaneDefinition::BEST_PROB => "best_prob".to_string(),
            DepartLaneDefinition::FIRST_ALLOWED => "first".to_string(),
            DepartLaneDefinition::DEFAULT => String::new(),
        }
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
        match self.departPosProcedure {
            DepartPosDefinition::GIVEN => to_string(&self.departPos),
            DepartPosDefinition::GIVEN_VEHROUTE => prune_zeros(&f64_to_string_prec(self.departPos, std::cmp::max(g_precision_random(), g_precision())), 2),
            DepartPosDefinition::RANDOM => "random".to_string(),
            DepartPosDefinition::RANDOM_FREE => "random_free".to_string(),
            DepartPosDefinition::RANDOM_LOCATION => "random_location".to_string(),
            DepartPosDefinition::FREE => "free".to_string(),
            DepartPosDefinition::LAST => "last".to_string(),
            DepartPosDefinition::BASE => "base".to_string(),
            DepartPosDefinition::SPLIT_FRONT => "splitFront".to_string(),
            DepartPosDefinition::STOP => "stop".to_string(),
            DepartPosDefinition::DEFAULT => String::new(),
        }
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
        match self.departPosLatProcedure {
            DepartPosLatDefinition::GIVEN => to_string(&self.departPosLat),
            DepartPosLatDefinition::GIVEN_VEHROUTE => prune_zeros(&f64_to_string_prec(self.departPosLat, std::cmp::max(g_precision_random(), g_precision())), 2),
            DepartPosLatDefinition::RANDOM => "random".to_string(),
            DepartPosLatDefinition::RANDOM_FREE => "random_free".to_string(),
            DepartPosLatDefinition::FREE => "free".to_string(),
            DepartPosLatDefinition::RIGHT => "right".to_string(),
            DepartPosLatDefinition::CENTER => "center".to_string(),
            DepartPosLatDefinition::LEFT => "left".to_string(),
            DepartPosLatDefinition::DEFAULT => String::new(),
        }
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
        match self.departSpeedProcedure {
            DepartSpeedDefinition::GIVEN => to_string(&self.departSpeed),
            DepartSpeedDefinition::GIVEN_VEHROUTE => prune_zeros(&f64_to_string_prec(self.departSpeed, std::cmp::max(g_precision_random(), g_precision())), 2),
            DepartSpeedDefinition::RANDOM => "random".to_string(),
            DepartSpeedDefinition::MAX => "max".to_string(),
            DepartSpeedDefinition::DESIRED => "desired".to_string(),
            DepartSpeedDefinition::LIMIT => "speedLimit".to_string(),
            DepartSpeedDefinition::LAST => "last".to_string(),
            DepartSpeedDefinition::AVG => "avg".to_string(),
            DepartSpeedDefinition::DEFAULT => String::new(),
        }
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
        match self.departEdgeProcedure {
            RouteIndexDefinition::GIVEN => to_string(&self.departEdge),
            RouteIndexDefinition::RANDOM => "random".to_string(),
            RouteIndexDefinition::DEFAULT => String::new(),
        }
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
        match self.arrivalLaneProcedure {
            ArrivalLaneDefinition::GIVEN => to_string(&self.arrivalLane),
            ArrivalLaneDefinition::CURRENT => "current".to_string(),
            ArrivalLaneDefinition::RANDOM => "random".to_string(),
            ArrivalLaneDefinition::FIRST_ALLOWED => "first".to_string(),
            ArrivalLaneDefinition::DEFAULT => String::new(),
        }
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
        match self.arrivalPosProcedure {
            ArrivalPosDefinition::GIVEN => to_string(&self.arrivalPos),
            ArrivalPosDefinition::RANDOM => "random".to_string(),
            ArrivalPosDefinition::CENTER => "center".to_string(),
            ArrivalPosDefinition::MAX => "max".to_string(),
            ArrivalPosDefinition::DEFAULT => String::new(),
        }
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
        match self.arrivalPosLatProcedure {
            ArrivalPosLatDefinition::GIVEN => to_string(&self.arrivalPosLat),
            ArrivalPosLatDefinition::RIGHT => "right".to_string(),
            ArrivalPosLatDefinition::CENTER => "center".to_string(),
            ArrivalPosLatDefinition::LEFT => "left".to_string(),
            ArrivalPosLatDefinition::DEFAULT => String::new(),
        }
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
        match self.arrivalSpeedProcedure {
            ArrivalSpeedDefinition::GIVEN => to_string(&self.arrivalSpeed),
            ArrivalSpeedDefinition::CURRENT => "current".to_string(),
            ArrivalSpeedDefinition::DEFAULT => String::new(),
        }
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
        match self.arrivalEdgeProcedure {
            RouteIndexDefinition::GIVEN => to_string(&self.arrivalEdge),
            RouteIndexDefinition::RANDOM => "random".to_string(),
            RouteIndexDefinition::DEFAULT => String::new(),
        }
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
        if self.insertionChecks == 0 || self.insertionChecks == InsertionCheck::ALL as i32 {
            InsertionChecks.get_string(InsertionCheck::ALL).to_string()
        } else {
            let mut insertionChecksStrs: Vec<String> = Vec::new();
            let insertionCheckValuesList = InsertionChecks.get_values();
            // iterate over values
            for insertionCheckValue in insertionCheckValuesList {
                if insertionCheckValue != InsertionCheck::ALL && (self.insertionChecks & insertionCheckValue as i32) != 0 {
                    insertionChecksStrs.push(InsertionChecks.get_string(insertionCheckValue).to_string());
                }
            }
            insertionChecksStrs.join(" ")
        }
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
        if value.is_empty() {
            return true;
        } else {
            // split value in substrings
            let mut valueStrs = StringTokenizer::with_token(value, " ", true);
            // iterate over values
            while valueStrs.has_next() {
                if !InsertionChecks.has_string(&valueStrs.next().unwrap_or_default()) {
                    return false;
                }
            }
            true
        }
    }

}

/// Definition of vehicle stop (position and duration)
/// C++ class `SUMOVehicleParameter::Stop` (utils/vehicle/SUMOVehicleParameter.h)
#[derive(Clone)]
#[repr(C)]
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
    pub durationDist: DistributionParameterized,
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
    pub accessPos: Vec<(String, f64, f64)>,
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

/// C++ `SUMOVehicleParameter::Stop stop;` (default member values of SUMOVehicleParameter.h)
impl Default for SUMOVehicleParameter_Stop {
    fn default() -> Self {
        SUMOVehicleParameter_Stop {
            base: Parameterised::new(),
            edge: String::new(),
            lane: String::new(),
            busstop: String::new(),
            containerstop: String::new(),
            parkingarea: String::new(),
            chargingStation: String::new(),
            overheadWireSegment: String::new(),
            startPos: 0.,
            endPos: 0.,
            arrival: -1,
            duration: -1,
            durationDist: DistributionParameterized::default(),
            until: -1,
            extension: -1,
            waitUntil: -1,
            triggered: false,
            containerTriggered: false,
            joinTriggered: false,
            parking: ParkingType::ONROAD,
            awaitedPersons: BTreeSet::new(),
            permitted: BTreeSet::new(),
            awaitedContainers: BTreeSet::new(),
            friendlyPos: false,
            actType: String::new(),
            tripId: String::new(),
            line: String::new(),
            split: String::new(),
            join: String::new(),
            speed: 0.,
            posLat: utils::common::std_defs::INVALID_DOUBLE,
            onDemand: false,
            priority: -1.,
            jump: -1,
            jumpUntil: -1,
            started: -1,
            ended: -1,
            accessPos: Vec::new(),
            index: 0,
            routeIndex: 0,
            parametersSet: 0,
            collision: false,
            breakDown: false,
        }
    }
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
        // the C++ destructor body is empty; the members are released by Rust's drop glue
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
        if writeTagAndParents {
            dev.open_tag_id(SUMO_TAG_STOP);
            if self.busstop != "" {
                dev.write_attr(SUMO_ATTR_BUS_STOP, &self.busstop);
            }
            if self.containerstop != "" {
                dev.write_attr(SUMO_ATTR_CONTAINER_STOP, &self.containerstop);
            }
            if self.chargingStation != "" {
                dev.write_attr(SUMO_ATTR_CHARGING_STATION, &self.chargingStation);
            }
            if self.parkingarea != "" {
                dev.write_attr(SUMO_ATTR_PARKING_AREA, &self.parkingarea);
            }
            if self.busstop == "" && self.containerstop == "" && self.parkingarea == "" && self.chargingStation == "" {
                if self.lane != "" {
                    dev.write_attr(SUMO_ATTR_LANE, &self.lane);
                } else {
                    dev.write_attr(SUMO_ATTR_EDGE, &self.edge);
                }
                if (self.parametersSet & STOP_START_SET) != 0 {
                    dev.write_attr(SUMO_ATTR_STARTPOS, &self.startPos);
                }
                if (self.parametersSet & STOP_END_SET) != 0 {
                    dev.write_attr(SUMO_ATTR_ENDPOS, &self.endPos);
                }
            }
        }
        if self.index > 0 {
            dev.write_attr(SUMO_ATTR_INDEX, &self.index);
        }
        if (self.parametersSet & STOP_POSLAT_SET) != 0 && self.posLat != INVALID_DOUBLE {
            dev.write_attr(SUMO_ATTR_POSITION_LAT, &self.posLat);
        }
        if (self.parametersSet & STOP_ARRIVAL_SET) != 0 && self.arrival >= 0 {
            dev.write_attr(SUMO_ATTR_ARRIVAL, &time2string(self.arrival));
        }
        if (self.parametersSet & STOP_DURATION_SET) != 0 && self.duration >= 0 {
            dev.write_attr(SUMO_ATTR_DURATION, &time2string(self.duration));
        }
        if (self.parametersSet & STOP_UNTIL_SET) != 0 && self.until >= 0 {
            dev.write_attr(SUMO_ATTR_UNTIL, &time2string(self.until));
        }
        if (self.parametersSet & STOP_STARTED_SET) != 0 && self.started >= 0 {
            dev.write_attr(SUMO_ATTR_STARTED, &time2string(self.started));
        }
        if (self.parametersSet & STOP_ENDED_SET) != 0 && self.ended >= 0 {
            dev.write_attr(SUMO_ATTR_ENDED, &time2string(self.ended));
        }
        if (self.parametersSet & STOP_EXTENSION_SET) != 0 && self.extension >= 0 {
            dev.write_attr(SUMO_ATTR_EXTENSION, &time2string(self.extension));
        }
        if (self.parametersSet & STOP_TRIGGER_SET) != 0 {
            let triggers = self.get_triggers();
            if triggers.len() > 0 {
                dev.write_attr(SUMO_ATTR_TRIGGERED, &triggers);
            }
        }
        if (self.parametersSet & STOP_PARKING_SET) != 0 {
            dev.write_attr(SUMO_ATTR_PARKING, &self.parking);
        }
        if (self.parametersSet & STOP_EXPECTED_SET) != 0 && self.awaitedPersons.len() > 0 {
            let v: Vec<String> = self.awaitedPersons.iter().cloned().collect();
            dev.write_attr(SUMO_ATTR_EXPECTED, &v);
        }
        if (self.parametersSet & STOP_PERMITTED_SET) != 0 && self.permitted.len() > 0 {
            let v: Vec<String> = self.permitted.iter().cloned().collect();
            dev.write_attr(SUMO_ATTR_PERMITTED, &v);
        }
        if (self.parametersSet & STOP_EXPECTED_CONTAINERS_SET) != 0 && self.awaitedContainers.len() > 0 {
            let v: Vec<String> = self.awaitedContainers.iter().cloned().collect();
            dev.write_attr(SUMO_ATTR_EXPECTED_CONTAINERS, &v);
        }
        if (self.parametersSet & STOP_TRIP_ID_SET) != 0 {
            dev.write_attr(SUMO_ATTR_TRIP_ID, &self.tripId);
        }
        if (self.parametersSet & STOP_LINE_SET) != 0 {
            dev.write_attr(SUMO_ATTR_LINE, &self.line);
        }
        if (self.parametersSet & STOP_SPLIT_SET) != 0 {
            dev.write_attr(SUMO_ATTR_SPLIT, &self.split);
        }
        if (self.parametersSet & STOP_JOIN_SET) != 0 {
            dev.write_attr(SUMO_ATTR_JOIN, &self.join);
        }
        if (self.parametersSet & STOP_SPEED_SET) != 0 {
            dev.write_attr(SUMO_ATTR_SPEED, &self.speed);
        }
        if (self.parametersSet & STOP_ONDEMAND_SET) != 0 {
            dev.write_attr(SUMO_ATTR_ONDEMAND, &self.onDemand);
        }
        if (self.parametersSet & STOP_JUMP_SET) != 0 && self.jump >= 0 {
            dev.write_attr(SUMO_ATTR_JUMP, &time2string(self.jump));
        }
        if (self.parametersSet & STOP_JUMP_UNTIL_SET) != 0 && self.jumpUntil >= 0 {
            dev.write_attr(SUMO_ATTR_JUMP_UNTIL, &time2string(self.jumpUntil));
        }
        if (self.parametersSet & STOP_PRIORITY_SET) != 0 && self.priority >= 0.0 {
            dev.write_attr(SUMO_ATTR_PRIORITY, &self.priority);
        }
        if self.collision {
            dev.write_attr(SUMO_ATTR_COLLISION, &self.collision);
        }
        // only write friendly position if is true
        if self.friendlyPos {
            dev.write_attr(SUMO_ATTR_FRIENDLY_POS, &self.friendlyPos);
        }
        // only write act type if isn't empty
        if !self.actType.is_empty() {
            dev.write_attr(SUMO_ATTR_ACTTYPE, &self.actType);
        }
        if close {
            // the user is closing the stop it is responsible for writing params
            self.write_params(dev);
            dev.close_tag("");
        }
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
        let mut result: Vec<String> = Vec::new();
        if self.busstop != "" {
            result.push(self.busstop.clone());
        }
        if self.containerstop != "" {
            result.push(self.containerstop.clone());
        }
        if self.chargingStation != "" {
            result.push(self.chargingStation.clone());
        }
        if self.parkingarea != "" {
            result.push(self.parkingarea.clone());
        }
        result
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
        let mut triggers: Vec<String> = Vec::new();
        if self.triggered {
            triggers.push(Tags.get_string(SUMO_TAG_PERSON).to_string());
        }
        if self.containerTriggered {
            triggers.push(Tags.get_string(SUMO_TAG_CONTAINER).to_string());
        }
        if self.joinTriggered {
            triggers.push(SUMO_ATTR_JOIN.attr_name().to_string());
        }
        triggers
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
        ((if self.parking == ParkingType::OFFROAD { 1 } else { 0 })
            + (if self.triggered { 2 } else { 0 })
            + (if self.containerTriggered { 4 } else { 0 })
            + (if self.busstop != "" { 8 } else { 0 })
            + (if self.containerstop != "" { 16 } else { 0 })
            + (if self.chargingStation != "" { 32 } else { 0 })
            + (if self.parkingarea != "" { 64 } else { 0 })
            + (if self.overheadWireSegment != "" { 128 } else { 0 }))
    }

}

