//! Port of utils/vehicle/SUMOVTypeParameter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::cfmodels::mscf_model::LinearApproxMap;
use crate::utils::emissions::pollutants_interface::PollutantsInterface;
use utils::common::sumo_vehicle_class::*;
use utils::common::time::{SUMOTIME_MIN, steps2time, time2string};

// C++ VTYPEPARS_* bit flags (SUMOVTypeParameter.h), recording which attributes were set
const VTYPEPARS_LENGTH_SET: i64 = 1;
const VTYPEPARS_MINGAP_SET: i64 = 1 << 1;
const VTYPEPARS_MAXSPEED_SET: i64 = 1 << 2;
const VTYPEPARS_PROBABILITY_SET: i64 = 1 << 3;
const VTYPEPARS_SPEEDFACTOR_SET: i64 = 1 << 4;
const VTYPEPARS_EMISSIONCLASS_SET: i64 = 1 << 5;
const VTYPEPARS_COLOR_SET: i64 = 1 << 6;
const VTYPEPARS_VEHICLECLASS_SET: i64 = 1 << 7;
const VTYPEPARS_WIDTH_SET: i64 = 1 << 8;
const VTYPEPARS_HEIGHT_SET: i64 = 1 << 9;
const VTYPEPARS_SHAPE_SET: i64 = 1 << 10;
const VTYPEPARS_OSGFILE_SET: i64 = 1 << 11;
const VTYPEPARS_IMGFILE_SET: i64 = 1 << 12;
const VTYPEPARS_IMPATIENCE_SET: i64 = 1 << 13;
const VTYPEPARS_LANE_CHANGE_MODEL_SET: i64 = 1 << 14;
const VTYPEPARS_PERSON_CAPACITY: i64 = 1 << 15;
const VTYPEPARS_BOARDING_DURATION: i64 = 1 << 16;
const VTYPEPARS_CONTAINER_CAPACITY: i64 = 1 << 17;
const VTYPEPARS_LOADING_DURATION: i64 = 1 << 18;
const VTYPEPARS_CAR_FOLLOW_MODEL: i64 = 1 << 19;
const VTYPEPARS_MAXSPEED_LAT_SET: i64 = 1 << 20;
const VTYPEPARS_LATALIGNMENT_SET: i64 = 1 << 21;
const VTYPEPARS_MINGAP_LAT_SET: i64 = 1 << 22;
const VTYPEPARS_ACTIONSTEPLENGTH_SET: i64 = 1 << 23;
const VTYPEPARS_DESIRED_MAXSPEED_SET: i64 = 1 << 24;
const VTYPEPARS_CARRIAGE_LENGTH_SET: i64 = 1 << 25;
const VTYPEPARS_LOCOMOTIVE_LENGTH_SET: i64 = 1 << 26;
const VTYPEPARS_CARRIAGE_GAP_SET: i64 = 1 << 27;
const VTYPEPARS_CARRIAGE_DOORS_SET: i64 = 1 << 28;
const VTYPEPARS_MANEUVER_ANGLE_TIMES_SET: i64 = 1 << 29;
const VTYPEPARS_FRONT_SEAT_POS_SET: i64 = 1 << 30;
const VTYPEPARS_SCALE_SET: i64 = 1 << 31;
const VTYPEPARS_MASS_SET: i64 = 1 << 32;
const VTYPEPARS_TTT_SET: i64 = 1 << 33;
const VTYPEPARS_TTT_BIDI_SET: i64 = 1 << 34;
const VTYPEPARS_SEATING_WIDTH_SET: i64 = 1 << 35;
const VTYPEPARS_SPEEDFACTOR_PREMATURE_SET: i64 = 1 << 36;
const VTYPEPARS_PARKING_BADGES_SET: i64 = 1 << 37;
const VTYPEPARS_BOARDING_FACTOR_SET: i64 = 1 << 38;
// C++ VTYPEPARS_DEFAULT_EMERGENCYDECEL_DEFAULT / _DECEL
const VTYPEPARS_DEFAULT_EMERGENCYDECEL_DEFAULT: f64 = -1.0;
const VTYPEPARS_DEFAULT_EMERGENCYDECEL_DECEL: f64 = -2.0;

/// C++ `#define EMPREFIX std::string("HBEFA4/")`
const EMPREFIX: &str = "HBEFA4/";

/// `std::string` + `ProcessError` throw: C++ `throw ProcessError(msg)` becomes a panic carrying the error
fn throw_process_error(msg: String) -> ! {
    std::panic::panic_any(ProcessError::new(msg))
}

/// C++ `StringUtils::toDouble` / `toInt` throw on bad input; the Rust helpers return Result
fn or_throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{}", e))
}

/// C++ `Distribution_Parameterized::setParameter(1, v)` (index 1 always exists on the 4-parameter speedFactor)
fn set_speed_factor_deviation(speedFactor: &mut DistributionParameterized, v: f64) {
    speedFactor.set_parameter(1, v).unwrap();
}

/// C++ `LinearApproxHelpers::LinearApproxMap::operator[]` on the sorted-vector representation
fn linear_approx_map_set(map: &mut LinearApproxMap, key: f64, value: f64) {
    let pos = map.partition_point(|(k, _)| *k < key);
    if pos < map.len() && map[pos].0 == key {
        map[pos].1 = value;
    } else {
        map.insert(pos, (key, value));
    }
}

/// C++ `LatAlignmentDefinition` (SUMOVTypeParameter.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum LatAlignmentDefinition {
    DEFAULT = 0,
    GIVEN = 1,
    RIGHT = 2,
    CENTER = 3,
    ARBITRARY = 4,
    NICE = 5,
    COMPACT = 6,
    LEFT = 7,
}

/// C++ typedef `std::map<SumoXMLAttr, std::string> SUMOVTypeParameter::SubParams`
pub type SUMOVTypeParameter_SubParams = BTreeMap<SumoXMLAttr, String>;

/// Structure representing possible vehicle parameter
/// C++ class `SUMOVTypeParameter` (utils/vehicle/SUMOVTypeParameter.h)
#[repr(C)]
pub struct SUMOVTypeParameter {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The vehicle type's id
    pub id: String,
    /// The physical vehicle length
    pub length: f64,
    /// This class' free space in front of the vehicle itself
    pub minGap: f64,
    /// The vehicle type's (technical) maximum speed [m/s]
    pub maxSpeed: f64,
    /// The vehicle type's desired maximum speed [m/s]
    pub desiredMaxSpeed: f64,
    /// The vehicle type's default actionStepLength [ms], i.e. the interval between two control actions. The default value of 0ms. induces the value to be traced from MSGlobals::gActionStepLength
    pub actionStepLength: SUMOTime,
    /// The probability when being added to a distribution without an explicit probability
    pub defaultProbability: f64,
    /// The factor by which the maximum speed may deviate from the allowed max speed on the street
    pub speedFactor: DistributionParameterized,
    /// The emission class of this vehicle
    pub emissionClass: SUMOEmissionClass,
    /// The mass
    pub mass: f64,
    /// The color
    pub color: RGBColor,
    /// The vehicle's class
    pub vehicleClass: SUMOVehicleClass,
    /// The vehicle's impatience (willingness to obstruct others)
    pub impatience: f64,
    /// The person capacity of the vehicle
    pub personCapacity: i32,
    /// The container capacity of the vehicle
    pub containerCapacity: i32,
    /// The time a person needs to board the vehicle
    pub boardingDuration: SUMOTime,
    /// The time a container needs to get loaded on the vehicle
    pub loadingDuration: SUMOTime,
    /// individual scaling factor (-1 for undefined)
    pub scale: f64,
    /// This class' width
    pub width: f64,
    /// This class' height
    pub height: f64,
    /// This class' shape
    pub shape: SUMOVehicleShape,
    /// 3D model file for this class
    pub osgFile: String,
    /// Image file for this class
    pub imgFile: String,
    /// Image files for additional carriages
    pub carriageImages: Vec<String>,
    /// The enum-representation of the car-following model to use
    pub cfModel: SumoXMLTag,
    /// Car-following parameter
    pub cfParameter: SUMOVTypeParameter_SubParams,
    /// Lane-changing parameter
    pub lcParameter: SUMOVTypeParameter_SubParams,
    /// Junction-model parameter
    pub jmParameter: SUMOVTypeParameter_SubParams,
    /// The lane-change model to use
    pub lcModel: LaneChangeModel,
    /// The vehicle type's maximum lateral speed [m/s]
    pub maxSpeedLat: f64,
    /// (optional) The vehicle's desired lateral alignment as offset in m from center line
    pub latAlignmentOffset: f64,
    /// Information on how the vehicle shall choose the lateral alignment
    pub latAlignmentProcedure: LatAlignmentDefinition,
    /// The vehicle type's minimum lateral gap [m]
    pub minGapLat: f64,
    /// the scaling factor when drawing the object
    pub scaleVisual: f64,
    /// the length of train carriages
    pub carriageLength: f64,
    /// the length of the locomotive
    pub locomotiveLength: f64,
    /// the length of the gap between carriages
    pub carriageGap: f64,
    /// the number of doors per carriage
    pub carriageDoors: i32,
    /// the width of the carriage doors
    pub carriageDoorWidth: f64,
    /// the maximum distance between platform and train
    pub maxPlatformDistance: f64,
    /// the custom time-to-teleport for this type
    pub timeToTeleport: SUMOTime,
    /// the custom time-to-teleport.bidi for this type
    pub timeToTeleportBidi: SUMOTime,
    /// the possible speed reduction when a train is ahead of schedule
    pub speedFactorPremature: f64,
    /// the offset of the first person seat from the front of the vehicle
    pub frontSeatPos: f64,
    /// width to be used when comping seats
    pub seatingWidth: f64,
    /// the parking access rights
    pub parkingBadges: Vec<String>,
    /// factor for boardingDuration / loadingDuration
    pub boardingFactor: f64,
    /// Information for the router which parameter were set
    pub parametersSet: i64,
    /// Information whether this type was already saved (needed by routers)
    pub saved: bool,
    /// Information whether this is a type-stub, being only referenced but not defined (needed by routers)
    pub onlyReferenced: bool,
    /// cached value of parameters which may restrict access to certain edges
    pub paramRestrictions: Vec<f64>,
    /// Map of manoeuver angles versus the times (entry, exit) to execute the manoeuver
    pub myManoeuverAngleTimes: BTreeMap<i32, (SUMOTime, SUMOTime)>, // C++: std::map<int, std::pair<SUMOTime, SUMOTime>>
}

impl std::ops::Deref for SUMOVTypeParameter {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for SUMOVTypeParameter {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

// C++ static members of SUMOVTypeParameter (port as module-level statics, see AGENTS.md):
//   static std::set<SumoXMLAttr> SUMOVTypeParameter::AllowedJMAttrs;

/// C++ static `SUMOVTypeParameter::AllowedJMAttrs` (SUMOVTypeParameter.cpp)
pub fn allowed_jm_attrs() -> BTreeSet<SumoXMLAttr> {
    [
        SUMO_ATTR_JM_CROSSING_GAP,
        SUMO_ATTR_JM_DRIVE_AFTER_YELLOW_TIME,
        SUMO_ATTR_JM_DRIVE_AFTER_RED_TIME,
        SUMO_ATTR_JM_DRIVE_RED_SPEED,
        SUMO_ATTR_JM_IGNORE_KEEPCLEAR_TIME,
        SUMO_ATTR_JM_IGNORE_FOE_SPEED,
        SUMO_ATTR_JM_IGNORE_FOE_PROB,
        SUMO_ATTR_JM_IGNORE_JUNCTION_FOE_PROB,
        SUMO_ATTR_JM_SIGMA_MINOR,
        SUMO_ATTR_JM_STOPLINE_GAP,
        SUMO_ATTR_JM_STOPLINE_GAP_MINOR,
        SUMO_ATTR_JM_TIMEGAP_MINOR,
        SUMO_ATTR_JM_EXTRA_GAP,
        SUMO_ATTR_JM_ADVANCE,
        SUMO_ATTR_JM_STOPSIGN_WAIT,
        SUMO_ATTR_JM_ALLWAYSTOP_WAIT,
    ]
    .into_iter()
    .collect()
}

impl SUMOVTypeParameter {
    /// C++: `void SUMOVTypeParameter::SUMOVTypeParameter(const std::string &, const SUMOVehicleClass)`
    /// Constructor
    // SUMOVTypeParameter::SUMOVTypeParameter(const std::string& vtid, const SUMOVehicleClass vclass)
    //     : id(vtid),
    //       actionStepLength(0),
    //       defaultProbability(DEFAULT_VEH_PROB),
    //       speedFactor("normc", 1.0, 0.0, 0.2, 2.0),
    //       emissionClass(PollutantsInterface::getClassByName(EMPREFIX + "default", vclass)),
    //       color(RGBColor::DEFAULT_COLOR),
    //       vehicleClass(vclass),
    //       impatience(0.0),
    //       personCapacity(4),
    //       containerCapacity(0),
    //       boardingDuration(500),
    //       loadingDuration(90000),
    //       scale(1),
    //       width(1.8),
    //       height(1.5),
    //       shape(SUMOVehicleShape::UNKNOWN),
    //       osgFile("car-normal-citrus.obj"),
    //       cfModel(SUMO_TAG_CF_KRAUSS),
    //       lcModel(LaneChangeModel::DEFAULT),
    //       maxSpeedLat(1.0),
    //       latAlignmentOffset(0.0),
    //       latAlignmentProcedure(LatAlignmentDefinition::CENTER),
    //       scaleVisual(1),
    //       carriageLength(-1),
    //       locomotiveLength(-1),
    //       carriageGap(1.),
    //       carriageDoors(2),
    //       carriageDoorWidth(1.5),
    //       maxPlatformDistance(3.),
    //       timeToTeleport(TTT_UNSET),
    //       timeToTeleportBidi(TTT_UNSET),
    //       speedFactorPremature(-1),
    //       frontSeatPos(1.7),
    //       seatingWidth(-1),
    //       boardingFactor(1),
    //       parametersSet(0),
    //       saved(false),
    //       onlyReferenced(false) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.exists("carfollow.model")) {
    //         // check for valid value has been performed in MSFrame
    //         cfModel = SUMOXMLDefinitions::CarFollowModels.get(oc.getString("carfollow.model"));
    //     }
    //     // obtain default values depending of vclass
    //     VClassDefaultValues defaultValues(vclass);
    //     // overwrite SUMOVTypeParameter with VClassDefaultValues
    //     length = defaultValues.length;
    //     minGap = defaultValues.minGap;
    //     minGapLat = defaultValues.minGapLat;
    //     maxSpeed = defaultValues.maxSpeed;
    //     desiredMaxSpeed = defaultValues.desiredMaxSpeed;
    //     width = defaultValues.width;
    //     height = defaultValues.height;
    //     shape = defaultValues.shape;
    //     emissionClass = defaultValues.emissionClass;
    //     mass = defaultValues.mass;
    //     speedFactor = defaultValues.speedFactor;
    //     personCapacity = defaultValues.personCapacity;
    //     containerCapacity = defaultValues.containerCapacity;
    //     osgFile = defaultValues.osgFile;
    //     carriageLength = defaultValues.carriageLength;
    //     locomotiveLength = defaultValues.locomotiveLength;
    //     carriageDoors = defaultValues.carriageDoors;
    //     latAlignmentProcedure = defaultValues.latAlignmentProcedure;
    //     // check if default speeddev was defined
    //     if (oc.exists("default.speeddev")) {
    //         const double defaultSpeedDev = oc.getFloat("default.speeddev");
    //         if (defaultSpeedDev >= 0) {
    //             speedFactor.setParameter(1, defaultSpeedDev);
    //         }
    //     } else {
    //         speedFactor.setParameter(1, -1.);
    //     }
    //     setManoeuverAngleTimes(vclass);
    // }
    #[allow(unused_variables)]
    pub fn new(vtid: &str, vc: SUMOVehicleClass) -> Self {
        // member initialiser emissionClass(getClassByName(EMPREFIX + "default", vclass)) runs before the body;
        // its value is overwritten by the VClassDefaultValues below, but the call itself is kept for ordering
        PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}default"), vc);
        let mut cfModel = SUMO_TAG_CF_KRAUSS;
        if utils::options::options_cont::exists("carfollow.model") {
            // check for valid value has been performed in MSFrame
            cfModel = or_throw(CarFollowModels.get(&utils::options::options_cont::get_string("carfollow.model")));
        }
        // obtain default values depending of vclass
        let defaultValues = SUMOVTypeParameter_VClassDefaultValues::new(vc);
        let mut speedFactor = defaultValues.speedFactor.clone();
        // check if default speeddev was defined
        if utils::options::options_cont::exists("default.speeddev") {
            let defaultSpeedDev = utils::options::options_cont::get_float("default.speeddev");
            if defaultSpeedDev >= 0. {
                set_speed_factor_deviation(&mut speedFactor, defaultSpeedDev);
            }
        } else {
            set_speed_factor_deviation(&mut speedFactor, -1.);
        }
        let mut result = SUMOVTypeParameter {
            base: Parameterised::new(),
            id: vtid.to_string(),
            length: defaultValues.length,
            minGap: defaultValues.minGap,
            maxSpeed: defaultValues.maxSpeed,
            desiredMaxSpeed: defaultValues.desiredMaxSpeed,
            actionStepLength: 0,
            defaultProbability: DEFAULT_VEH_PROB,
            speedFactor,
            emissionClass: defaultValues.emissionClass,
            mass: defaultValues.mass,
            color: RGBColor::DEFAULT_COLOR,
            vehicleClass: vc,
            impatience: 0.0,
            personCapacity: defaultValues.personCapacity,
            containerCapacity: defaultValues.containerCapacity,
            boardingDuration: 500,
            loadingDuration: 90000,
            scale: 1.,
            width: defaultValues.width,
            height: defaultValues.height,
            shape: defaultValues.shape,
            osgFile: defaultValues.osgFile,
            imgFile: String::new(),
            carriageImages: Vec::new(),
            cfModel,
            cfParameter: BTreeMap::new(),
            lcParameter: BTreeMap::new(),
            jmParameter: BTreeMap::new(),
            lcModel: LaneChangeModel::DEFAULT,
            maxSpeedLat: 1.0,
            latAlignmentOffset: 0.0,
            latAlignmentProcedure: defaultValues.latAlignmentProcedure,
            minGapLat: defaultValues.minGapLat,
            scaleVisual: 1.,
            carriageLength: defaultValues.carriageLength,
            locomotiveLength: defaultValues.locomotiveLength,
            carriageGap: 1.,
            carriageDoors: defaultValues.carriageDoors,
            carriageDoorWidth: defaultValues.carriageDoorWidth,
            maxPlatformDistance: defaultValues.maxPlatformDistance,
            timeToTeleport: SUMOTIME_MIN,
            timeToTeleportBidi: SUMOTIME_MIN,
            speedFactorPremature: -1.,
            frontSeatPos: 1.7,
            seatingWidth: -1.,
            parkingBadges: Vec::new(),
            boardingFactor: 1.,
            parametersSet: 0,
            saved: false,
            onlyReferenced: false,
            paramRestrictions: Vec::new(),
            myManoeuverAngleTimes: BTreeMap::new(),
        };
        result.set_manoeuver_angle_times(vc);
        result
    }

    /// C++: `void SUMOVTypeParameter::~SUMOVTypeParameter() noexcept` [virtual]
    /// virtual destructor
    // virtual ~SUMOVTypeParameter() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {}

    /// C++: `bool SUMOVTypeParameter::wasSet(long long) const` [const]
    /// Returns whether the given parameter was set
    // bool wasSet(long long int what) const {
    //         return (parametersSet & what) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_set(&self, what: i64) -> bool {
        (self.parametersSet & what) != 0
    }

    /// C++: `void SUMOVTypeParameter::write(OutputDevice &) const` [const]
    /// Writes the vtype
    // void
    // SUMOVTypeParameter::write(OutputDevice& dev) const {
    //     // first check if vehicle type can be written
    //     if (onlyReferenced) {
    //         return;
    //     }
    //     // open vehicle tag
    //     dev.openTag(SUMO_TAG_VTYPE);
    //     // write ID (always needed)
    //     dev.writeAttr(SUMO_ATTR_ID, id);
    //     // write parameters depending if is set
    //     if (wasSet(VTYPEPARS_LENGTH_SET)) {
    //         dev.writeAttr(SUMO_ATTR_LENGTH, length);
    //     }
    //     if (wasSet(VTYPEPARS_MINGAP_SET)) {
    //         dev.writeAttr(SUMO_ATTR_MINGAP, minGap);
    //     }
    //     if (wasSet(VTYPEPARS_MAXSPEED_SET)) {
    //         dev.writeAttr(SUMO_ATTR_MAXSPEED, maxSpeed);
    //     }
    //     if (wasSet(VTYPEPARS_DESIRED_MAXSPEED_SET)) {
    //         dev.writeAttr(SUMO_ATTR_DESIRED_MAXSPEED, desiredMaxSpeed);
    //     }
    //     if (wasSet(VTYPEPARS_PROBABILITY_SET)) {
    //         dev.writeAttr(SUMO_ATTR_PROB, defaultProbability);
    //     }
    //     if (wasSet(VTYPEPARS_SPEEDFACTOR_SET)) {
    //         dev.writeAttr(SUMO_ATTR_SPEEDFACTOR, speedFactor);
    //     }
    //     if (wasSet(VTYPEPARS_ACTIONSTEPLENGTH_SET)) {
    //         // Note: action step length is only exposed in seconds to the user
    //         dev.writeAttr(SUMO_ATTR_ACTIONSTEPLENGTH, STEPS2TIME(actionStepLength));
    //     }
    //     if (wasSet(VTYPEPARS_VEHICLECLASS_SET)) {
    //         dev.writeAttr(SUMO_ATTR_VCLASS, toString(vehicleClass));
    //     }
    //     if (wasSet(VTYPEPARS_EMISSIONCLASS_SET)) {
    //         dev.writeAttr(SUMO_ATTR_EMISSIONCLASS, PollutantsInterface::getName(emissionClass));
    //     }
    //     if (wasSet(VTYPEPARS_MASS_SET)) {
    //         dev.writeAttr(SUMO_ATTR_MASS, mass);
    //     }
    //     if (wasSet(VTYPEPARS_IMPATIENCE_SET)) {
    //         if (impatience == -std::numeric_limits<double>::max()) {
    //             dev.writeAttr(SUMO_ATTR_IMPATIENCE, "off");
    //         } else {
    //             dev.writeAttr(SUMO_ATTR_IMPATIENCE, impatience);
    //         }
    //     }
    //     if (wasSet(VTYPEPARS_SHAPE_SET)) {
    //         dev.writeAttr(SUMO_ATTR_GUISHAPE, getVehicleShapeName(shape));
    //     }
    //     if (wasSet(VTYPEPARS_WIDTH_SET)) {
    //         dev.writeAttr(SUMO_ATTR_WIDTH, width);
    //     }
    //     if (wasSet(VTYPEPARS_HEIGHT_SET)) {
    //         dev.writeAttr(SUMO_ATTR_HEIGHT, height);
    //     }
    //     if (wasSet(VTYPEPARS_COLOR_SET)) {
    //         dev.writeAttr(SUMO_ATTR_COLOR, color);
    //     }
    //     if (wasSet(VTYPEPARS_OSGFILE_SET)) {
    //         dev.writeAttr(SUMO_ATTR_OSGFILE, osgFile);
    //     }
    //     if (wasSet(VTYPEPARS_IMGFILE_SET)) {
    //         dev.writeAttr(SUMO_ATTR_IMGFILE, imgFile);
    //     }
    //     if (wasSet(VTYPEPARS_PERSON_CAPACITY)) {
    //         dev.writeAttr(SUMO_ATTR_PERSON_CAPACITY, personCapacity);
    //     }
    //     if (wasSet(VTYPEPARS_CONTAINER_CAPACITY)) {
    //         dev.writeAttr(SUMO_ATTR_CONTAINER_CAPACITY, containerCapacity);
    //     }
    //     if (wasSet(VTYPEPARS_BOARDING_DURATION)) {
    //         dev.writeAttr(SUMO_ATTR_BOARDING_DURATION, time2string(boardingDuration));
    //     }
    //     if (wasSet(VTYPEPARS_LOADING_DURATION)) {
    //         dev.writeAttr(SUMO_ATTR_LOADING_DURATION, time2string(loadingDuration));
    //     }
    //     if (wasSet(VTYPEPARS_MAXSPEED_LAT_SET)) {
    //         dev.writeAttr(SUMO_ATTR_MAXSPEED_LAT, maxSpeedLat);
    //     }
    //     if (wasSet(VTYPEPARS_LATALIGNMENT_SET)) {
    //         switch (latAlignmentProcedure) {
    //             case LatAlignmentDefinition::GIVEN:
    //                 dev.writeAttr(SUMO_ATTR_LATALIGNMENT, latAlignmentOffset);
    //                 break;
    //             case LatAlignmentDefinition::RIGHT:
    //                 dev.writeAttr(SUMO_ATTR_LATALIGNMENT, "right");
    //                 break;
    //             case LatAlignmentDefinition::CENTER:
    //                 dev.writeAttr(SUMO_ATTR_LATALIGNMENT, "center");
    //                 break;
    //             case LatAlignmentDefinition::ARBITRARY:
    //                 dev.writeAttr(SUMO_ATTR_LATALIGNMENT, "arbitrary");
    //                 break;
    //             case LatAlignmentDefinition::NICE:
    //                 dev.writeAttr(SUMO_ATTR_LATALIGNMENT, "nice");
    //                 break;
    //             case LatAlignmentDefinition::COMPACT:
    //                 dev.writeAttr(SUMO_ATTR_LATALIGNMENT, "compact");
    //                 break;
    //             case LatAlignmentDefinition::LEFT:
    //                 dev.writeAttr(SUMO_ATTR_LATALIGNMENT, "left");
    //                 break;
    //             case LatAlignmentDefinition::DEFAULT:
    //             default:
    //                 break;
    //         }
    //     }
    //     if (wasSet(VTYPEPARS_MINGAP_LAT_SET)) {
    //         dev.writeAttr(SUMO_ATTR_MINGAP_LAT, minGapLat);
    //     }
    //     if (wasSet(VTYPEPARS_MANEUVER_ANGLE_TIMES_SET)) {
    //         dev.writeAttr(SUMO_ATTR_MANEUVER_ANGLE_TIMES, getManoeuverAngleTimesS());
    //     }
    //     if (wasSet(VTYPEPARS_SCALE_SET)) {
    //         dev.writeAttr(SUMO_ATTR_SCALE, scale);
    //     }
    //     if (wasSet(VTYPEPARS_TTT_SET)) {
    //         dev.writeAttr(SUMO_ATTR_TIME_TO_TELEPORT, time2string(timeToTeleport));
    //     }
    //     if (wasSet(VTYPEPARS_TTT_BIDI_SET)) {
    //         dev.writeAttr(SUMO_ATTR_TIME_TO_TELEPORT_BIDI, time2string(timeToTeleportBidi));
    //     }
    //     if (wasSet(VTYPEPARS_SPEEDFACTOR_PREMATURE_SET)) {
    //         dev.writeAttr(SUMO_ATTR_SPEEDFACTOR_PREMATURE, speedFactorPremature);
    //     }
    //     if (wasSet(VTYPEPARS_BOARDING_FACTOR_SET)) {
    //         dev.writeAttr(SUMO_ATTR_BOARDING_FACTOR, boardingFactor);
    //     }
    //     if (wasSet(VTYPEPARS_LANE_CHANGE_MODEL_SET)) {
    //         dev.writeAttr(SUMO_ATTR_LANE_CHANGE_MODEL, lcModel);
    //     }
    //     // Write Lane Change Model parameters
    //     for (const auto& lcParam : lcParameter) {
    //         dev.writeAttr(lcParam.first, lcParam.second);
    //     }
    //     // Write Junction Model parameter
    //     for (const auto& jmParam : jmParameter) {
    //         dev.writeAttr(jmParam.first, jmParam.second);
    //     }
    //     if (wasSet(VTYPEPARS_CAR_FOLLOW_MODEL)) {
    //         dev.writeAttr(SUMO_ATTR_CAR_FOLLOW_MODEL, SUMOXMLDefinitions::CarFollowModels.getString(cfModel));
    //     }
    //     // Write Car Following Model parameters
    //     for (const auto& cfParam : cfParameter) {
    //         dev.writeAttr(cfParam.first, cfParam.second);
    //     }
    //     // Write carriage length
    //     if (wasSet(VTYPEPARS_CARRIAGE_LENGTH_SET)) {
    //         dev.openTag(SUMO_TAG_PARAM);
    //         dev.writeAttr(SUMO_ATTR_KEY, toString(SUMO_ATTR_CARRIAGE_LENGTH));
    //         dev.writeAttr(SUMO_ATTR_VALUE, toString(carriageLength));
    //         dev.closeTag();
    //     }
    //     // Write locomotive length
    //     if (wasSet(VTYPEPARS_LOCOMOTIVE_LENGTH_SET)) {
    //         dev.openTag(SUMO_TAG_PARAM);
    //         dev.writeAttr(SUMO_ATTR_KEY, toString(SUMO_ATTR_LOCOMOTIVE_LENGTH));
    //         dev.writeAttr(SUMO_ATTR_VALUE, toString(locomotiveLength));
    //         dev.closeTag();
    //     }
    //     // Write carriage gap
    //     if (wasSet(VTYPEPARS_CARRIAGE_GAP_SET)) {
    //         dev.openTag(SUMO_TAG_PARAM);
    //         dev.writeAttr(SUMO_ATTR_KEY, toString(SUMO_ATTR_CARRIAGE_GAP));
    //         dev.writeAttr(SUMO_ATTR_VALUE, toString(carriageGap));
    //         dev.closeTag();
    //     }
    //     // Write carriage doors
    //     if (wasSet(VTYPEPARS_CARRIAGE_DOORS_SET)) {
    //         dev.openTag(SUMO_TAG_PARAM);
    //         dev.writeAttr(SUMO_ATTR_KEY, toString(SUMO_ATTR_CARRIAGE_DOORS));
    //         dev.writeAttr(SUMO_ATTR_VALUE, toString(carriageDoors));
    //         dev.closeTag();
    //     }
    //     // Write rest of parameters
    //     writeParams(dev);
    //     // close tag
    //     dev.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice) {
        // first check if vehicle type can be written
        if self.onlyReferenced {
            return;
        }
        // open vehicle tag
        dev.open_tag_id(SUMO_TAG_VTYPE);
        // write ID (always needed)
        dev.write_attr(SUMO_ATTR_ID, &self.id);
        // write parameters depending if is set
        if self.was_set(VTYPEPARS_LENGTH_SET) {
            dev.write_attr(SUMO_ATTR_LENGTH, &self.length);
        }
        if self.was_set(VTYPEPARS_MINGAP_SET) {
            dev.write_attr(SUMO_ATTR_MINGAP, &self.minGap);
        }
        if self.was_set(VTYPEPARS_MAXSPEED_SET) {
            dev.write_attr(SUMO_ATTR_MAXSPEED, &self.maxSpeed);
        }
        if self.was_set(VTYPEPARS_DESIRED_MAXSPEED_SET) {
            dev.write_attr(SUMO_ATTR_DESIRED_MAXSPEED, &self.desiredMaxSpeed);
        }
        if self.was_set(VTYPEPARS_PROBABILITY_SET) {
            dev.write_attr(SUMO_ATTR_PROB, &self.defaultProbability);
        }
        if self.was_set(VTYPEPARS_SPEEDFACTOR_SET) {
            dev.write_attr(SUMO_ATTR_SPEEDFACTOR, &self.speedFactor.to_str(dev.get_precision()));
        }
        if self.was_set(VTYPEPARS_ACTIONSTEPLENGTH_SET) {
            // Note: action step length is only exposed in seconds to the user
            dev.write_attr(SUMO_ATTR_ACTIONSTEPLENGTH, &steps2time(self.actionStepLength));
        }
        if self.was_set(VTYPEPARS_VEHICLECLASS_SET) {
            dev.write_attr(SUMO_ATTR_VCLASS, SUMO_VEHICLE_CLASS_STRINGS.get_string(self.vehicleClass).unwrap());
        }
        if self.was_set(VTYPEPARS_EMISSIONCLASS_SET) {
            dev.write_attr(SUMO_ATTR_EMISSIONCLASS, &PollutantsInterface::get_name(self.emissionClass));
        }
        if self.was_set(VTYPEPARS_MASS_SET) {
            dev.write_attr(SUMO_ATTR_MASS, &self.mass);
        }
        if self.was_set(VTYPEPARS_IMPATIENCE_SET) {
            if self.impatience == -f64::MAX {
                dev.write_attr(SUMO_ATTR_IMPATIENCE, "off");
            } else {
                dev.write_attr(SUMO_ATTR_IMPATIENCE, &self.impatience);
            }
        }
        if self.was_set(VTYPEPARS_SHAPE_SET) {
            dev.write_attr(SUMO_ATTR_GUISHAPE, &get_vehicle_shape_name(self.shape));
        }
        if self.was_set(VTYPEPARS_WIDTH_SET) {
            dev.write_attr(SUMO_ATTR_WIDTH, &self.width);
        }
        if self.was_set(VTYPEPARS_HEIGHT_SET) {
            dev.write_attr(SUMO_ATTR_HEIGHT, &self.height);
        }
        if self.was_set(VTYPEPARS_COLOR_SET) {
            dev.write_attr(SUMO_ATTR_COLOR, &self.color);
        }
        if self.was_set(VTYPEPARS_OSGFILE_SET) {
            dev.write_attr(SUMO_ATTR_OSGFILE, &self.osgFile);
        }
        if self.was_set(VTYPEPARS_IMGFILE_SET) {
            dev.write_attr(SUMO_ATTR_IMGFILE, &self.imgFile);
        }
        if self.was_set(VTYPEPARS_PERSON_CAPACITY) {
            dev.write_attr(SUMO_ATTR_PERSON_CAPACITY, &self.personCapacity);
        }
        if self.was_set(VTYPEPARS_CONTAINER_CAPACITY) {
            dev.write_attr(SUMO_ATTR_CONTAINER_CAPACITY, &self.containerCapacity);
        }
        if self.was_set(VTYPEPARS_BOARDING_DURATION) {
            dev.write_attr(SUMO_ATTR_BOARDING_DURATION, &time2string(self.boardingDuration));
        }
        if self.was_set(VTYPEPARS_LOADING_DURATION) {
            dev.write_attr(SUMO_ATTR_LOADING_DURATION, &time2string(self.loadingDuration));
        }
        if self.was_set(VTYPEPARS_MAXSPEED_LAT_SET) {
            dev.write_attr(SUMO_ATTR_MAXSPEED_LAT, &self.maxSpeedLat);
        }
        if self.was_set(VTYPEPARS_LATALIGNMENT_SET) {
            match self.latAlignmentProcedure {
                LatAlignmentDefinition::GIVEN => {
                    dev.write_attr(SUMO_ATTR_LATALIGNMENT, &self.latAlignmentOffset);
                }
                LatAlignmentDefinition::RIGHT => {
                    dev.write_attr(SUMO_ATTR_LATALIGNMENT, "right");
                }
                LatAlignmentDefinition::CENTER => {
                    dev.write_attr(SUMO_ATTR_LATALIGNMENT, "center");
                }
                LatAlignmentDefinition::ARBITRARY => {
                    dev.write_attr(SUMO_ATTR_LATALIGNMENT, "arbitrary");
                }
                LatAlignmentDefinition::NICE => {
                    dev.write_attr(SUMO_ATTR_LATALIGNMENT, "nice");
                }
                LatAlignmentDefinition::COMPACT => {
                    dev.write_attr(SUMO_ATTR_LATALIGNMENT, "compact");
                }
                LatAlignmentDefinition::LEFT => {
                    dev.write_attr(SUMO_ATTR_LATALIGNMENT, "left");
                }
                LatAlignmentDefinition::DEFAULT => {}
            }
        }
        if self.was_set(VTYPEPARS_MINGAP_LAT_SET) {
            dev.write_attr(SUMO_ATTR_MINGAP_LAT, &self.minGapLat);
        }
        if self.was_set(VTYPEPARS_MANEUVER_ANGLE_TIMES_SET) {
            dev.write_attr(SUMO_ATTR_MANEUVER_ANGLE_TIMES, &self.get_manoeuver_angle_times_s());
        }
        if self.was_set(VTYPEPARS_SCALE_SET) {
            dev.write_attr(SUMO_ATTR_SCALE, &self.scale);
        }
        if self.was_set(VTYPEPARS_TTT_SET) {
            dev.write_attr(SUMO_ATTR_TIME_TO_TELEPORT, &time2string(self.timeToTeleport));
        }
        if self.was_set(VTYPEPARS_TTT_BIDI_SET) {
            dev.write_attr(SUMO_ATTR_TIME_TO_TELEPORT_BIDI, &time2string(self.timeToTeleportBidi));
        }
        if self.was_set(VTYPEPARS_SPEEDFACTOR_PREMATURE_SET) {
            dev.write_attr(SUMO_ATTR_SPEEDFACTOR_PREMATURE, &self.speedFactorPremature);
        }
        if self.was_set(VTYPEPARS_BOARDING_FACTOR_SET) {
            dev.write_attr(SUMO_ATTR_BOARDING_FACTOR, &self.boardingFactor);
        }
        if self.was_set(VTYPEPARS_LANE_CHANGE_MODEL_SET) {
            dev.write_attr(SUMO_ATTR_LANE_CHANGE_MODEL, &self.lcModel);
        }
        // Write Lane Change Model parameters
        for (lcParamKey, lcParamValue) in &self.lcParameter {
            dev.write_attr(*lcParamKey, lcParamValue);
        }
        // Write Junction Model parameter
        for (jmParamKey, jmParamValue) in &self.jmParameter {
            dev.write_attr(*jmParamKey, jmParamValue);
        }
        if self.was_set(VTYPEPARS_CAR_FOLLOW_MODEL) {
            dev.write_attr(SUMO_ATTR_CAR_FOLLOW_MODEL, CarFollowModels.get_string(self.cfModel));
        }
        // Write Car Following Model parameters
        for (cfParamKey, cfParamValue) in &self.cfParameter {
            dev.write_attr(*cfParamKey, cfParamValue);
        }
        // Write carriage length
        if self.was_set(VTYPEPARS_CARRIAGE_LENGTH_SET) {
            dev.open_tag_id(SUMO_TAG_PARAM);
            dev.write_attr(SUMO_ATTR_KEY, Attrs.get_string(SUMO_ATTR_CARRIAGE_LENGTH));
            dev.write_attr(SUMO_ATTR_VALUE, &utils::common::to_string::to_string(&self.carriageLength));
            dev.close_tag("");
        }
        // Write locomotive length
        if self.was_set(VTYPEPARS_LOCOMOTIVE_LENGTH_SET) {
            dev.open_tag_id(SUMO_TAG_PARAM);
            dev.write_attr(SUMO_ATTR_KEY, Attrs.get_string(SUMO_ATTR_LOCOMOTIVE_LENGTH));
            dev.write_attr(SUMO_ATTR_VALUE, &utils::common::to_string::to_string(&self.locomotiveLength));
            dev.close_tag("");
        }
        // Write carriage gap
        if self.was_set(VTYPEPARS_CARRIAGE_GAP_SET) {
            dev.open_tag_id(SUMO_TAG_PARAM);
            dev.write_attr(SUMO_ATTR_KEY, Attrs.get_string(SUMO_ATTR_CARRIAGE_GAP));
            dev.write_attr(SUMO_ATTR_VALUE, &utils::common::to_string::to_string(&self.carriageGap));
            dev.close_tag("");
        }
        // Write carriage doors
        if self.was_set(VTYPEPARS_CARRIAGE_DOORS_SET) {
            dev.open_tag_id(SUMO_TAG_PARAM);
            dev.write_attr(SUMO_ATTR_KEY, Attrs.get_string(SUMO_ATTR_CARRIAGE_DOORS));
            dev.write_attr(SUMO_ATTR_VALUE, &self.carriageDoors.to_string());
            dev.close_tag("");
        }
        // Write rest of parameters
        self.write_params(dev);
        // close tag
        dev.close_tag("");
    }

    /// C++: `double SUMOVTypeParameter::getCFParam(const SumoXMLAttr, const double) const` [const]
    /// Returns the named value from the map, or the default if it is not contained there
    // double
    // SUMOVTypeParameter::getCFParam(const SumoXMLAttr attr, const double defaultValue) const {
    //     if (cfParameter.count(attr)) {
    //         return StringUtils::toDouble(cfParameter.find(attr)->second);
    //     } else {
    //         return defaultValue;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_cf_param(&self, attr: SumoXMLAttr, defaultValue: f64) -> f64 {
        if let Some(v) = self.cfParameter.get(&attr) {
            or_throw(utils::common::string_utils::to_double(v))
        } else {
            defaultValue
        }
    }

    /// C++: `std::vector<double> SUMOVTypeParameter::getCFValueTable(SumoXMLAttr) const` [const]
    /// retrieve value table from cfParameter
    // std::vector<double>
    // SUMOVTypeParameter::getCFValueTable(SumoXMLAttr attr) const {
    //     std::vector<double> result;
    //     const std::string values = getCFParamString(attr, "");
    //     if (!values.empty()) {
    //         for (std::string value : StringTokenizer(values).getVector()) {
    //             result.push_back(StringUtils::toDouble(value));
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_cf_value_table(&self, attr: SumoXMLAttr) -> Vec<f64> {
        let mut result = Vec::new();
        let values = self.get_cf_param_string(attr, String::new());
        if !values.is_empty() {
            for value in utils::common::string_tokenizer::StringTokenizer::new(&values).get_vector() {
                result.push(or_throw(utils::common::string_utils::to_double(&value)));
            }
        }
        result
    }

    /// C++: `std::string SUMOVTypeParameter::getCFParamString(const SumoXMLAttr, const std::string) const` [const]
    /// Returns the named value from the map, or the default if it is not contained there
    // std::string
    // SUMOVTypeParameter::getCFParamString(const SumoXMLAttr attr, const std::string defaultValue) const {
    //     if (cfParameter.count(attr)) {
    //         return cfParameter.find(attr)->second;
    //     } else {
    //         return defaultValue;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_cf_param_string(&self, attr: SumoXMLAttr, defaultValue: String) -> String {
        if let Some(v) = self.cfParameter.get(&attr) {
            v.clone()
        } else {
            defaultValue
        }
    }

    /// C++: `double SUMOVTypeParameter::getLCParam(const SumoXMLAttr, const double) const` [const]
    /// Returns the named value from the map, or the default if it is not contained there
    // double
    // SUMOVTypeParameter::getLCParam(const SumoXMLAttr attr, const double defaultValue) const {
    //     if (lcParameter.count(attr)) {
    //         return StringUtils::toDouble(lcParameter.find(attr)->second);
    //     } else {
    //         return defaultValue;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lc_param(&self, attr: SumoXMLAttr, defaultValue: f64) -> f64 {
        if let Some(v) = self.lcParameter.get(&attr) {
            or_throw(utils::common::string_utils::to_double(v))
        } else {
            defaultValue
        }
    }

    /// C++: `std::string SUMOVTypeParameter::getLCParamString(const SumoXMLAttr, const std::string &) const` [const]
    /// Returns the named value from the map, or the default if it is not contained there
    // std::string
    // SUMOVTypeParameter::getLCParamString(const SumoXMLAttr attr, const std::string& defaultValue) const {
    //     if (lcParameter.count(attr)) {
    //         return lcParameter.find(attr)->second;
    //     } else {
    //         return defaultValue;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lc_param_string(&self, attr: SumoXMLAttr, defaultValue: &str) -> String {
        if let Some(v) = self.lcParameter.get(&attr) {
            v.clone()
        } else {
            defaultValue.to_string()
        }
    }

    /// C++: `const SubParams & SUMOVTypeParameter::getLCParams() const` [const]
    /// Returns the LC parameter
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_lc_params(&self) -> &SUMOVTypeParameter_SubParams {
        &self.lcParameter
    }

    /// C++: `double SUMOVTypeParameter::getJMParam(const SumoXMLAttr, const double) const` [const]
    /// Returns the named value from the map, or the default if it is not contained there
    // double
    // SUMOVTypeParameter::getJMParam(const SumoXMLAttr attr, const double defaultValue) const {
    //     if (jmParameter.count(attr)) {
    //         return StringUtils::toDouble(jmParameter.find(attr)->second);
    //     } else {
    //         return defaultValue;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_jm_param(&self, attr: SumoXMLAttr, defaultValue: f64) -> f64 {
        if let Some(v) = self.jmParameter.get(&attr) {
            or_throw(utils::common::string_utils::to_double(v))
        } else {
            defaultValue
        }
    }

    /// C++: `std::string SUMOVTypeParameter::getJMParamString(const SumoXMLAttr, const std::string) const` [const]
    /// Returns the named value from the map, or the default if it is not contained there
    // std::string
    // SUMOVTypeParameter::getJMParamString(const SumoXMLAttr attr, const std::string defaultValue) const {
    //     if (jmParameter.count(attr)) {
    //         return jmParameter.find(attr)->second;
    //     } else {
    //         return defaultValue;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_jm_param_string(&self, attr: SumoXMLAttr, defaultValue: String) -> String {
        if let Some(v) = self.jmParameter.get(&attr) {
            v.clone()
        } else {
            defaultValue
        }
    }

    /// C++: `void SUMOVTypeParameter::cacheParamRestrictions(const std::vector<std::string> &)`
    // void
    // SUMOVTypeParameter::cacheParamRestrictions(const std::vector<std::string>& restrictionKeys) {
    //     for (const std::string& key : restrictionKeys) {
    //         paramRestrictions.push_back(StringUtils::toDouble(getParameter(key, "0")));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cache_param_restrictions(&mut self, restrictionKeys: &Vec<String>) {
        for key in restrictionKeys {
            let value = self.get_parameter(key, "0");
            self.paramRestrictions.push(or_throw(utils::common::string_utils::to_double(&value)));
        }
    }

    /// C++: `void SUMOVTypeParameter::initRailVisualizationParameters(const std::string)`
    /// init Rail Visualization Parameters
    // void
    // SUMOVTypeParameter::initRailVisualizationParameters(const std::string fileName) {
    //     if (hasParameter("scaleVisual")) {
    //         scaleVisual = StringUtils::toDouble(getParameter("scaleVisual"));
    //     }
    //     if (hasParameter("carriageLength")) {
    //         carriageLength = StringUtils::toDouble(getParameter("carriageLength"));
    //         parametersSet |= VTYPEPARS_CARRIAGE_LENGTH_SET;
    //     } else {
    //         switch (shape) {
    //             case SUMOVehicleShape::BUS_FLEXIBLE:
    //                 carriageLength = 8.25; // 16.5 overall, 2 modules http://de.wikipedia.org/wiki/Ikarus_180
    //                 carriageGap = 0;
    //                 break;
    //             case SUMOVehicleShape::RAIL:
    //                 if (vehicleClass == SVC_RAIL_ELECTRIC) {
    //                     carriageLength = 24.5;
    //                     locomotiveLength = 19.100; // https://en.wikipedia.org/wiki/DB_Class_101
    //                 } else if (vehicleClass == SVC_RAIL_FAST) {
    //                     carriageLength = 24.775; // http://de.wikipedia.org/wiki/ICE_3
    //                     locomotiveLength = 25.835;
    //                 } else {
    //                     carriageLength = 24.5; // http://de.wikipedia.org/wiki/UIC-Y-Wagen_%28DR%29
    //                     locomotiveLength = 16.4; // https://en.wikipedia.org/wiki/DB_Class_218
    //                 }
    //                 break;
    //             case SUMOVehicleShape::RAIL_CAR:
    //                 if (vehicleClass == SVC_TRAM) {
    //                     carriageLength = 5.71; // http://de.wikipedia.org/wiki/Bombardier_Flexity_Berlin
    //                     locomotiveLength = 5.71;
    //                 } else if (vehicleClass == SVC_RAIL_URBAN) {
    //                     carriageLength = 18.4;  // https://en.wikipedia.org/wiki/DBAG_Class_481
    //                     locomotiveLength = 18.4;
    //                 } else {
    //                     carriageLength = 16.85;  // 67.4m overall, 4 carriages http://de.wikipedia.org/wiki/DB-Baureihe_423
    //                 }
    //                 break;
    //             case SUMOVehicleShape::RAIL_CARGO:
    //                 carriageLength = 13.86; // UIC 571-1 http://de.wikipedia.org/wiki/Flachwagen
    //                 break;
    //             case SUMOVehicleShape::TRUCK_SEMITRAILER:
    //                 carriageLength = 13.5;
    //                 locomotiveLength = 2.5;
    //                 carriageGap = 0.5;
    //                 break;
    //             case SUMOVehicleShape::TRUCK_1TRAILER:
    //                 carriageLength = 6.75;
    //                 locomotiveLength = 2.5 + 6.75;
    //                 carriageGap = 0.5;
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    //     if (hasParameter("locomotiveLength")) {
    //         locomotiveLength = StringUtils::toDouble(getParameter("locomotiveLength"));
    //         parametersSet |= VTYPEPARS_LOCOMOTIVE_LENGTH_SET;
    //     } else if (locomotiveLength < 0) {
    //         locomotiveLength = carriageLength;
    //     }
    //     if (hasParameter("carriageGap")) {
    //         carriageGap = StringUtils::toDouble(getParameter("carriageGap"));
    //         parametersSet |= VTYPEPARS_CARRIAGE_GAP_SET;
    //     }
    //     if (hasParameter("carriageDoors")) {
    //         carriageDoors = StringUtils::toInt(getParameter("carriageDoors"));
    //         parametersSet |= VTYPEPARS_CARRIAGE_DOORS_SET;
    //     }
    //     if (hasParameter("carriageDoorWidth")) {
    //         carriageDoorWidth = StringUtils::toDouble(getParameter("carriageDoorWidth"));
    //     }
    //     if (hasParameter("maxPlatformDistance")) {
    //         maxPlatformDistance = StringUtils::toDouble(getParameter("maxPlatformDistance"));
    //     }
    //     if (hasParameter("frontSeatPos")) {
    //         frontSeatPos = StringUtils::toDouble(getParameter("frontSeatPos"));
    //         parametersSet |= VTYPEPARS_FRONT_SEAT_POS_SET;
    //     } else {
    //         switch (shape) {
    //             case SUMOVehicleShape::SHIP:
    //                 frontSeatPos = 5;
    //                 break;
    //             case SUMOVehicleShape::DELIVERY:
    //                 frontSeatPos = 1.2;
    //                 break;
    //             case SUMOVehicleShape::BICYCLE:
    //                 frontSeatPos = 0.6;
    //                 break;
    //             case SUMOVehicleShape::MOPED:
    //             case SUMOVehicleShape::MOTORCYCLE:
    //                 frontSeatPos = 0.9;
    //                 break;
    //             case SUMOVehicleShape::BUS:
    //             case SUMOVehicleShape::BUS_COACH:
    //             case SUMOVehicleShape::BUS_FLEXIBLE:
    //             case SUMOVehicleShape::BUS_TROLLEY:
    //                 frontSeatPos = 0.5;
    //                 break;
    //             case SUMOVehicleShape::TRUCK:
    //             case SUMOVehicleShape::TRUCK_1TRAILER:
    //             case SUMOVehicleShape::TRUCK_SEMITRAILER:
    //                 frontSeatPos = 0.8;
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    //
    //     if (hasParameter("seatingWidth")) {
    //         seatingWidth = StringUtils::toDouble(getParameter("seatingWidth"));
    //         parametersSet |= VTYPEPARS_SEATING_WIDTH_SET;
    //     }
    //     if (hasParameter("carriageImages")) {
    //         std::vector<std::string> rawFiles = StringTokenizer(getParameter("carriageImages"), ",").getVector();
    //         for (const std::string& f : rawFiles) {
    //             carriageImages.push_back(FileHelpers::checkForRelativity(f, fileName));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_rail_visualization_parameters(&mut self, fileName: String) {
        if self.has_parameter("scaleVisual") {
            self.scaleVisual = or_throw(utils::common::string_utils::to_double(&self.get_parameter("scaleVisual", "")));
        }
        if self.has_parameter("carriageLength") {
            self.carriageLength = or_throw(utils::common::string_utils::to_double(&self.get_parameter("carriageLength", "")));
            self.parametersSet |= VTYPEPARS_CARRIAGE_LENGTH_SET;
        } else {
            match self.shape {
                SUMOVehicleShape::BusFlexible => {
                    self.carriageLength = 8.25; // 16.5 overall, 2 modules http://de.wikipedia.org/wiki/Ikarus_180
                    self.carriageGap = 0.;
                }
                SUMOVehicleShape::Rail => {
                    if self.vehicleClass == SVC_RAIL_ELECTRIC {
                        self.carriageLength = 24.5;
                        self.locomotiveLength = 19.100; // https://en.wikipedia.org/wiki/DB_Class_101
                    } else if self.vehicleClass == SVC_RAIL_FAST {
                        self.carriageLength = 24.775; // http://de.wikipedia.org/wiki/ICE_3
                        self.locomotiveLength = 25.835;
                    } else {
                        self.carriageLength = 24.5; // http://de.wikipedia.org/wiki/UIC-Y-Wagen_%28DR%29
                        self.locomotiveLength = 16.4; // https://en.wikipedia.org/wiki/DB_Class_218
                    }
                }
                SUMOVehicleShape::RailCar => {
                    if self.vehicleClass == SVC_TRAM {
                        self.carriageLength = 5.71; // http://de.wikipedia.org/wiki/Bombardier_Flexity_Berlin
                        self.locomotiveLength = 5.71;
                    } else if self.vehicleClass == SVC_RAIL_URBAN {
                        self.carriageLength = 18.4; // https://en.wikipedia.org/wiki/DBAG_Class_481
                        self.locomotiveLength = 18.4;
                    } else {
                        self.carriageLength = 16.85; // 67.4m overall, 4 carriages http://de.wikipedia.org/wiki/DB-Baureihe_423
                    }
                }
                SUMOVehicleShape::RailCargo => {
                    self.carriageLength = 13.86; // UIC 571-1 http://de.wikipedia.org/wiki/Flachwagen
                }
                SUMOVehicleShape::TruckSemitrailer => {
                    self.carriageLength = 13.5;
                    self.locomotiveLength = 2.5;
                    self.carriageGap = 0.5;
                }
                SUMOVehicleShape::Truck1Trailer => {
                    self.carriageLength = 6.75;
                    self.locomotiveLength = 2.5 + 6.75;
                    self.carriageGap = 0.5;
                }
                _ => {}
            }
        }
        if self.has_parameter("locomotiveLength") {
            self.locomotiveLength = or_throw(utils::common::string_utils::to_double(&self.get_parameter("locomotiveLength", "")));
            self.parametersSet |= VTYPEPARS_LOCOMOTIVE_LENGTH_SET;
        } else if self.locomotiveLength < 0. {
            self.locomotiveLength = self.carriageLength;
        }
        if self.has_parameter("carriageGap") {
            self.carriageGap = or_throw(utils::common::string_utils::to_double(&self.get_parameter("carriageGap", "")));
            self.parametersSet |= VTYPEPARS_CARRIAGE_GAP_SET;
        }
        if self.has_parameter("carriageDoors") {
            self.carriageDoors = or_throw(utils::common::string_utils::to_int(&self.get_parameter("carriageDoors", "")));
            self.parametersSet |= VTYPEPARS_CARRIAGE_DOORS_SET;
        }
        if self.has_parameter("carriageDoorWidth") {
            self.carriageDoorWidth = or_throw(utils::common::string_utils::to_double(&self.get_parameter("carriageDoorWidth", "")));
        }
        if self.has_parameter("maxPlatformDistance") {
            self.maxPlatformDistance = or_throw(utils::common::string_utils::to_double(&self.get_parameter("maxPlatformDistance", "")));
        }
        if self.has_parameter("frontSeatPos") {
            self.frontSeatPos = or_throw(utils::common::string_utils::to_double(&self.get_parameter("frontSeatPos", "")));
            self.parametersSet |= VTYPEPARS_FRONT_SEAT_POS_SET;
        } else {
            match self.shape {
                SUMOVehicleShape::Ship => {
                    self.frontSeatPos = 5.;
                }
                SUMOVehicleShape::Delivery => {
                    self.frontSeatPos = 1.2;
                }
                SUMOVehicleShape::Bicycle => {
                    self.frontSeatPos = 0.6;
                }
                SUMOVehicleShape::Moped | SUMOVehicleShape::Motorcycle => {
                    self.frontSeatPos = 0.9;
                }
                SUMOVehicleShape::Bus | SUMOVehicleShape::BusCoach | SUMOVehicleShape::BusFlexible | SUMOVehicleShape::BusTrolley => {
                    self.frontSeatPos = 0.5;
                }
                SUMOVehicleShape::Truck | SUMOVehicleShape::Truck1Trailer | SUMOVehicleShape::TruckSemitrailer => {
                    self.frontSeatPos = 0.8;
                }
                _ => {}
            }
        }

        if self.has_parameter("seatingWidth") {
            self.seatingWidth = or_throw(utils::common::string_utils::to_double(&self.get_parameter("seatingWidth", "")));
            self.parametersSet |= VTYPEPARS_SEATING_WIDTH_SET;
        }
        if self.has_parameter("carriageImages") {
            let rawFiles = utils::common::string_tokenizer::StringTokenizer::with_token(&self.get_parameter("carriageImages", ""), ",", false).get_vector();
            for f in rawFiles {
                self.carriageImages.push(utils::common::file_helpers::check_for_relativity(&f, &fileName));
            }
        }
    }

    /// C++: `double SUMOVTypeParameter::getLength() const` [const]
    /// satisfy vType / router template requirements
    // inline double getLength() const {
    //         return length;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.length
    }

    /// C++: `double SUMOVTypeParameter::getDefaultAccel(const SUMOVehicleClass)` [static]
    /// Returns the default acceleration for the given vehicle class This needs to be a function because the actual value is stored in the car following model
    // double
    // SUMOVTypeParameter::getDefaultAccel(const SUMOVehicleClass vc) {
    //     switch (vc) {
    //         case SVC_PEDESTRIAN:
    //         case SVC_WHEELCHAIR:
    //             return 1.5;
    //         case SVC_BICYCLE:
    //         case SVC_SCOOTER:
    //             return 1.2;
    //         case SVC_MOTORCYCLE:
    //             return 6.;
    //         case SVC_MOPED:
    //             return 1.1;
    //         case SVC_TRUCK:
    //             return 1.3;
    //         case SVC_TRAILER:
    //             return 1.1;
    //         case SVC_BUS:
    //             return 1.2;
    //         case SVC_COACH:
    //             return 2.;
    //         case SVC_TRAM:
    //             return 1.;
    //         case SVC_RAIL_URBAN:
    //             return 1.;
    //         case SVC_RAIL:
    //             return 0.25;
    //         case SVC_RAIL_ELECTRIC:
    //         case SVC_RAIL_FAST:
    //             return 0.5;
    //         case SVC_SHIP:
    //             return 0.1;
    //         default:
    //             return 2.6;//2.9;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_default_accel(vc: SUMOVehicleClass) -> f64 {
        match vc {
            SVC_PEDESTRIAN | SVC_WHEELCHAIR => 1.5,
            SVC_BICYCLE | SVC_SCOOTER => 1.2,
            SVC_MOTORCYCLE => 6.,
            SVC_MOPED => 1.1,
            SVC_TRUCK => 1.3,
            SVC_TRAILER => 1.1,
            SVC_BUS => 1.2,
            SVC_COACH => 2.,
            SVC_TRAM => 1.,
            SVC_RAIL_URBAN => 1.,
            SVC_RAIL => 0.25,
            SVC_RAIL_ELECTRIC | SVC_RAIL_FAST => 0.5,
            SVC_SHIP => 0.1,
            _ => 2.6, //2.9;
        }
    }

    /// C++: `double SUMOVTypeParameter::getDefaultDecel(const SUMOVehicleClass)` [static]
    /// Returns the default deceleration for the given vehicle class This needs to be a function because the actual value is stored in the car following model
    // double
    // SUMOVTypeParameter::getDefaultDecel(const SUMOVehicleClass vc) {
    //     switch (vc) {
    //         case SVC_PEDESTRIAN:
    //         case SVC_WHEELCHAIR:
    //             return 2.;
    //         case SVC_BICYCLE:
    //         case SVC_SCOOTER:
    //             return 3.;
    //         case SVC_MOPED:
    //             return 7.;
    //         case SVC_MOTORCYCLE:
    //             return 10.;
    //         case SVC_TRUCK:
    //         case SVC_TRAILER:
    //         case SVC_BUS:
    //         case SVC_COACH:
    //             return 4.;
    //         case SVC_TRAM:
    //         case SVC_RAIL_URBAN:
    //             return 3.;
    //         case SVC_RAIL:
    //         case SVC_RAIL_ELECTRIC:
    //         case SVC_RAIL_FAST:
    //             return 1.3;
    //         case SVC_SHIP:
    //             return 0.15;
    //         default:
    //             return 4.5;//7.5;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_default_decel(vc: SUMOVehicleClass) -> f64 {
        match vc {
            SVC_PEDESTRIAN | SVC_WHEELCHAIR => 2.,
            SVC_BICYCLE | SVC_SCOOTER => 3.,
            SVC_MOPED => 7.,
            SVC_MOTORCYCLE => 10.,
            SVC_TRUCK | SVC_TRAILER | SVC_BUS | SVC_COACH => 4.,
            SVC_TRAM | SVC_RAIL_URBAN => 3.,
            SVC_RAIL | SVC_RAIL_ELECTRIC | SVC_RAIL_FAST => 1.3,
            SVC_SHIP => 0.15,
            _ => 4.5, //7.5;
        }
    }

    /// C++: `double SUMOVTypeParameter::getDefaultEmergencyDecel(const SUMOVehicleClass, double, double)` [static]
    /// Returns the default emergency deceleration for the given vehicle class This needs to be a function because the actual value is stored in the car following model
    // double
    // SUMOVTypeParameter::getDefaultEmergencyDecel(const SUMOVehicleClass vc, double decel, double defaultOption) {
    //     if (defaultOption == VTYPEPARS_DEFAULT_EMERGENCYDECEL_DEFAULT) {
    //         double vcDecel;
    //         switch (vc) {
    //             case SVC_PEDESTRIAN:
    //             case SVC_WHEELCHAIR:
    //                 vcDecel = 5.;
    //                 break;
    //             case SVC_BICYCLE:
    //             case SVC_SCOOTER:
    //                 vcDecel = 7.;
    //                 break;
    //             case SVC_MOPED:
    //             case SVC_MOTORCYCLE:
    //                 vcDecel = 10.;
    //                 break;
    //             case SVC_TRUCK:
    //             case SVC_TRAILER:
    //             case SVC_BUS:
    //             case SVC_COACH:
    //             case SVC_TRAM:
    //             case SVC_RAIL_URBAN:
    //                 vcDecel = 7.;
    //                 break;
    //             case SVC_RAIL:
    //             case SVC_RAIL_ELECTRIC:
    //             case SVC_RAIL_FAST:
    //                 vcDecel = 5.;
    //                 break;
    //             case SVC_SHIP:
    //                 vcDecel = 1.;
    //                 break;
    //             default:
    //                 vcDecel = 9.;
    //         }
    //         return MAX2(decel, vcDecel);
    //     } else if (defaultOption == VTYPEPARS_DEFAULT_EMERGENCYDECEL_DECEL) {
    //         return decel;
    //     } else {
    //         // value already checked in MSFrame::checkOptions
    //         return MAX2(decel, defaultOption);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_default_emergency_decel(vc: SUMOVehicleClass, decel: f64, defaultOption: f64) -> f64 {
        if defaultOption == VTYPEPARS_DEFAULT_EMERGENCYDECEL_DEFAULT {
            let vcDecel = match vc {
                SVC_PEDESTRIAN | SVC_WHEELCHAIR => 5.,
                SVC_BICYCLE | SVC_SCOOTER => 7.,
                SVC_MOPED | SVC_MOTORCYCLE => 10.,
                SVC_TRUCK | SVC_TRAILER | SVC_BUS | SVC_COACH | SVC_TRAM | SVC_RAIL_URBAN => 7.,
                SVC_RAIL | SVC_RAIL_ELECTRIC | SVC_RAIL_FAST => 5.,
                SVC_SHIP => 1.,
                _ => 9.,
            };
            // C++ MAX2(a, b) = a > b ? a : b
            if decel > vcDecel { decel } else { vcDecel }
        } else if defaultOption == VTYPEPARS_DEFAULT_EMERGENCYDECEL_DECEL {
            decel
        } else {
            // value already checked in MSFrame::checkOptions
            if decel > defaultOption { decel } else { defaultOption }
        }
    }

    /// C++: `double SUMOVTypeParameter::getDefaultImperfection(const SUMOVehicleClass)` [static]
    /// Returns the default driver's imperfection (sigma or epsilon in Krauss' model) for the given vehicle class This needs to be a function because the actual value is stored in the car following model
    // double
    // SUMOVTypeParameter::getDefaultImperfection(const SUMOVehicleClass vc) {
    //     switch (vc) {
    //         case SVC_TRAM:
    //         case SVC_RAIL_URBAN:
    //         case SVC_RAIL:
    //         case SVC_RAIL_ELECTRIC:
    //         case SVC_RAIL_FAST:
    //         case SVC_SHIP:
    //             return 0.;
    //         default:
    //             return 0.5;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_default_imperfection(vc: SUMOVehicleClass) -> f64 {
        match vc {
            SVC_TRAM | SVC_RAIL_URBAN | SVC_RAIL | SVC_RAIL_ELECTRIC | SVC_RAIL_FAST | SVC_SHIP => 0.,
            _ => 0.5,
        }
    }

    /// C++: `const SUMOVTypeParameter & SUMOVTypeParameter::getDefault()` [static]
    /// return the default parameters, this is a function due to the http://www.parashift.com/c++-faq/static-init-order.html
    // const SUMOVTypeParameter&
    // SUMOVTypeParameter::getDefault() {
    //     static SUMOVTypeParameter defaultParams("");
    //     return defaultParams;
    // }
    #[allow(unused_variables)]
    pub fn get_default() -> &'static SUMOVTypeParameter {
        // static SUMOVTypeParameter defaultParams("");
        static DEFAULT_PARAMS: std::sync::OnceLock<SUMOVTypeParameter> = std::sync::OnceLock::new();
        DEFAULT_PARAMS.get_or_init(|| SUMOVTypeParameter::new("", SVC_IGNORING))
    }

    /// C++: `bool SUMOVTypeParameter::parseLatAlignment(const std::string &, double &, LatAlignmentDefinition &)` [static]
    /// Parses and validates a given latAlignment value
    // bool
    // SUMOVTypeParameter::parseLatAlignment(const std::string& val, double& lao, LatAlignmentDefinition& lad) {
    //     bool ok = true;
    //     lao = 0.0;
    //     lad = LatAlignmentDefinition::GIVEN;
    //     if (val == "right") {
    //         lad = LatAlignmentDefinition::RIGHT;
    //     } else if (val == "center") {
    //         lad = LatAlignmentDefinition::CENTER;
    //     } else if (val == "arbitrary") {
    //         lad = LatAlignmentDefinition::ARBITRARY;
    //     } else if (val == "nice") {
    //         lad = LatAlignmentDefinition::NICE;
    //     } else if (val == "compact") {
    //         lad = LatAlignmentDefinition::COMPACT;
    //     } else if (val == "left") {
    //         lad = LatAlignmentDefinition::LEFT;
    //     } else {
    //         try {
    //             lao = StringUtils::toDouble(val);
    //         } catch (...) {
    //             ok = false;
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_lat_alignment(val: &str, lao: &mut f64, lad: &mut LatAlignmentDefinition) -> bool {
        let mut ok = true;
        *lao = 0.0;
        *lad = LatAlignmentDefinition::GIVEN;
        if val == "right" {
            *lad = LatAlignmentDefinition::RIGHT;
        } else if val == "center" {
            *lad = LatAlignmentDefinition::CENTER;
        } else if val == "arbitrary" {
            *lad = LatAlignmentDefinition::ARBITRARY;
        } else if val == "nice" {
            *lad = LatAlignmentDefinition::NICE;
        } else if val == "compact" {
            *lad = LatAlignmentDefinition::COMPACT;
        } else if val == "left" {
            *lad = LatAlignmentDefinition::LEFT;
        } else {
            // try { lao = StringUtils::toDouble(val); } catch (...) { ok = false; }
            match utils::common::string_utils::to_double(val) {
                Ok(v) => *lao = v,
                Err(_) => ok = false,
            }
        }
        ok
    }

    /// C++: `bool SUMOVTypeParameter::isValidLatAlignment(const std::string &)` [static]
    // static inline bool isValidLatAlignment(const std::string& val) {
    //         double lao;
    //         LatAlignmentDefinition lad;
    //         return SUMOVTypeParameter::parseLatAlignment(val, lao, lad);
    //     }
    #[allow(unused_variables)]
    pub fn is_valid_lat_alignment(val: &str) -> bool {
        let mut lao: f64 = 0.0;
        let mut lad: LatAlignmentDefinition = LatAlignmentDefinition::GIVEN;
        SUMOVTypeParameter::parse_lat_alignment(val, &mut lao, &mut lad)
    }

    /// C++: `std::vector<std::string> SUMOVTypeParameter::getLatAlignmentStrings()` [static]
    /// return all valid strings for latAlignment
    // static inline std::vector<std::string> getLatAlignmentStrings() {
    //         std::vector<std::string> result;
    //         result.push_back("right");
    //         result.push_back("center");
    //         result.push_back("arbitrary");
    //         result.push_back("nice");
    //         result.push_back("compact");
    //         result.push_back("left");
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn get_lat_alignment_strings() -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push("right".to_string());
        result.push("center".to_string());
        result.push("arbitrary".to_string());
        result.push("nice".to_string());
        result.push("compact".to_string());
        result.push("left".to_string());
        result
    }

    /// C++: `void SUMOVTypeParameter::setManoeuverAngleTimes(const SUMOVehicleClass)`
    /// Initialise the default mapping between manoeuver angle and times dependant on vehicle class
    // void
    // SUMOVTypeParameter::setManoeuverAngleTimes(const SUMOVehicleClass vclass) {
    //
    //     myManoeuverAngleTimes.clear();
    //     /**
    //      * Defaults assume:   approaching at angles between 0-10 and 171-180 (will never be > 180) are approaching a space roughly parallel to the road
    //      *                    approaching at angles between 11-80 are approaching an acute angled space that is easiest to drive straight in
    //      *                    approaching at angles between 81-110 are approaching a space at approximately right angles to the road so the driver has a choice
    //      *                    approaching at angles between 111 and 170 are approaching an obtuse angled space that is easiest to drive past and reverse in
    //      *              More (or less) granular angle ranges can be used - configurable as a vType parameter
    //      */
    //     switch (vclass) {
    //         case SVC_PASSENGER:
    //         case SVC_HOV:
    //         case SVC_TAXI:
    //         case SVC_E_VEHICLE:
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(10, std::pair< SUMOTime, SUMOTime>(3000, 4000)));     // straight in but potentially needing parallel parking
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(80, std::pair< SUMOTime, SUMOTime>(1000, 11000)));    // straight in
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(110, std::pair< SUMOTime, SUMOTime>(11000, 2000)));   // optional forwards/backwards
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(170, std::pair< SUMOTime, SUMOTime>(8000, 3000)));    // backwards into obtuse space
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(181, std::pair< SUMOTime, SUMOTime>(3000, 4000)));    // straight in but potentially needing parallel parking
    //             break;
    //         case SVC_TRUCK:
    //         case SVC_TRAILER:
    //         case SVC_BUS:
    //         case SVC_COACH:
    //         case SVC_DELIVERY:
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(10, std::pair< SUMOTime, SUMOTime>(6000, 8000)));    // straight in but potentially needing parallel parking
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(80, std::pair< SUMOTime, SUMOTime>(2000, 21000)));   // straight in
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(110, std::pair< SUMOTime, SUMOTime>(21000, 2000)));  // optional forwards/backwards
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(170, std::pair< SUMOTime, SUMOTime>(14000, 5000)));  // backwards into obtuse space
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(181, std::pair< SUMOTime, SUMOTime>(6000, 8000)));   // straight in but potentially needing parallel parking
    //             break;
    //         case SVC_PEDESTRIAN:
    //         case SVC_MOPED:
    //         case SVC_BICYCLE:
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(181, std::pair< SUMOTime, SUMOTime>(1000, 1000)));  // no dependence on angle
    //             break;
    //         default:
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(10, std::pair< SUMOTime, SUMOTime>(3000, 4000)));    // straight in but potentially needing parallel parking
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(80, std::pair< SUMOTime, SUMOTime>(1000, 11000)));   // straight in
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(110, std::pair< SUMOTime, SUMOTime>(11000, 2000)));  // optional forwards/backwards
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(170, std::pair< SUMOTime, SUMOTime>(8000, 3000)));   // backwards into obtuse space
    //             myManoeuverAngleTimes.insert(std::pair<int, std::pair<SUMOTime, SUMOTime>>(181, std::pair< SUMOTime, SUMOTime>(3000, 4000)));   // straight in but potentially needing parallel parking
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_manoeuver_angle_times(&mut self, vclass: SUMOVehicleClass) {
        self.myManoeuverAngleTimes.clear();
        // Defaults assume:   approaching at angles between 0-10 and 171-180 (will never be > 180) are approaching a space roughly parallel to the road
        //                    approaching at angles between 11-80 are approaching an acute angled space that is easiest to drive straight in
        //                    approaching at angles between 81-110 are approaching a space at approximately right angles to the road so the driver has a choice
        //                    approaching at angles between 111 and 170 are approaching an obtuse angled space that is easiest to drive past and reverse in
        //              More (or less) granular angle ranges can be used - configurable as a vType parameter
        match vclass {
            SVC_PASSENGER | SVC_HOV | SVC_TAXI | SVC_E_VEHICLE => {
                self.myManoeuverAngleTimes.insert(10, (3000, 4000)); // straight in but potentially needing parallel parking
                self.myManoeuverAngleTimes.insert(80, (1000, 11000)); // straight in
                self.myManoeuverAngleTimes.insert(110, (11000, 2000)); // optional forwards/backwards
                self.myManoeuverAngleTimes.insert(170, (8000, 3000)); // backwards into obtuse space
                self.myManoeuverAngleTimes.insert(181, (3000, 4000)); // straight in but potentially needing parallel parking
            }
            SVC_TRUCK | SVC_TRAILER | SVC_BUS | SVC_COACH | SVC_DELIVERY => {
                self.myManoeuverAngleTimes.insert(10, (6000, 8000)); // straight in but potentially needing parallel parking
                self.myManoeuverAngleTimes.insert(80, (2000, 21000)); // straight in
                self.myManoeuverAngleTimes.insert(110, (21000, 2000)); // optional forwards/backwards
                self.myManoeuverAngleTimes.insert(170, (14000, 5000)); // backwards into obtuse space
                self.myManoeuverAngleTimes.insert(181, (6000, 8000)); // straight in but potentially needing parallel parking
            }
            SVC_PEDESTRIAN | SVC_MOPED | SVC_BICYCLE => {
                self.myManoeuverAngleTimes.insert(181, (1000, 1000)); // no dependence on angle
            }
            _ => {
                self.myManoeuverAngleTimes.insert(10, (3000, 4000)); // straight in but potentially needing parallel parking
                self.myManoeuverAngleTimes.insert(80, (1000, 11000)); // straight in
                self.myManoeuverAngleTimes.insert(110, (11000, 2000)); // optional forwards/backwards
                self.myManoeuverAngleTimes.insert(170, (8000, 3000)); // backwards into obtuse space
                self.myManoeuverAngleTimes.insert(181, (3000, 4000)); // straight in but potentially needing parallel parking
            }
        }
    }

    /// C++: `SUMOTime SUMOVTypeParameter::getEntryManoeuvreTime(const int) const` [const]
    /// Returns the time that will be needed for the vehicle type to execute the (entry) manoeuvre (and be blocking the lane)
    // SUMOTime
    // SUMOVTypeParameter::getEntryManoeuvreTime(const int angle) const {
    //     SUMOTime last = 0;
    //     for (std::pair<int, std::pair<SUMOTime, SUMOTime>> angleTime : myManoeuverAngleTimes) {
    //         if (angle <= angleTime.first) {
    //             return (angleTime.second.first);
    //         } else {
    //             last = angleTime.second.first;
    //         }
    //     }
    //     return (last);
    // }
    #[allow(unused_variables)]
    pub fn get_entry_manoeuvre_time(&self, angle: i32) -> SUMOTime {
        let mut last: SUMOTime = 0;
        for (key, angleTime) in &self.myManoeuverAngleTimes {
            if angle <= *key {
                return angleTime.0;
            } else {
                last = angleTime.0;
            }
        }
        last
    }

    /// C++: `SUMOTime SUMOVTypeParameter::getExitManoeuvreTime(const int) const` [const]
    /// Returns the time that will be needed for the vehicle type to execute the (exit) manoeuvre (and be blocking the lane)
    // SUMOTime
    // SUMOVTypeParameter::getExitManoeuvreTime(const int angle) const {
    //     SUMOTime last = 0;
    //     for (std::pair<int, std::pair<SUMOTime, SUMOTime>> angleTime : myManoeuverAngleTimes) {
    //         if (angle <= angleTime.first) {
    //             return (angleTime.second.second);
    //         } else {
    //             last = angleTime.second.second;
    //         }
    //     }
    //     return (last);
    // }
    #[allow(unused_variables)]
    pub fn get_exit_manoeuvre_time(&self, angle: i32) -> SUMOTime {
        let mut last: SUMOTime = 0;
        for (key, angleTime) in &self.myManoeuverAngleTimes {
            if angle <= *key {
                return angleTime.1;
            } else {
                last = angleTime.1;
            }
        }
        last
    }

    /// C++: `std::string SUMOVTypeParameter::getManoeuverAngleTimesS() const` [const]
    /// Returns myManoeuverAngleTimes as a string for xml output
    // std::string
    // SUMOVTypeParameter::getManoeuverAngleTimesS() const {
    //     std::stringstream stream;
    //
    //     stream << std::fixed << std::setprecision(1);
    //     int count = 0;
    //     for (std::pair<int, std::pair<SUMOTime, SUMOTime>> angleTime : myManoeuverAngleTimes) {
    //         if (count++ > 0) {
    //             stream << ",";
    //         }
    //         stream << toString(angleTime.first) + " " << STEPS2TIME(angleTime.second.first) << " " << STEPS2TIME(angleTime.second.second);
    //     }
    //     std::string triplets = stream.str();
    //     return triplets;
    // }
    #[allow(unused_variables)]
    pub fn get_manoeuver_angle_times_s(&self) -> String {
        // stream << std::fixed << std::setprecision(1); -> {:.1} for the STEPS2TIME values
        let mut triplets: Vec<String> = Vec::new();
        for (key, angleTime) in &self.myManoeuverAngleTimes {
            triplets.push(format!("{} {:.1} {:.1}", key, steps2time(angleTime.0), steps2time(angleTime.1)));
        }
        triplets.join(",")
    }

    /// C++: `SUMOTime SUMOVTypeParameter::getTimeToTeleport(SUMOTime) const` [const]
    /// return time-to-teleport (either custom or default)
    // SUMOTime
    // SUMOVTypeParameter::getTimeToTeleport(SUMOTime defaultValue) const {
    //     return timeToTeleport == TTT_UNSET ? defaultValue : timeToTeleport;
    // }
    #[allow(unused_variables)]
    pub fn get_time_to_teleport(&self, defaultValue: SUMOTime) -> SUMOTime {
        if self.timeToTeleport == SUMOTIME_MIN { defaultValue } else { self.timeToTeleport }
    }

    /// C++: `SUMOTime SUMOVTypeParameter::getTimeToTeleportBidi(SUMOTime) const` [const]
    /// return time-to-teleport.bidi (either custom or default)
    // SUMOTime
    // SUMOVTypeParameter::getTimeToTeleportBidi(SUMOTime defaultValue) const {
    //     return timeToTeleportBidi == TTT_UNSET ? defaultValue : timeToTeleportBidi;
    // }
    #[allow(unused_variables)]
    pub fn get_time_to_teleport_bidi(&self, defaultValue: SUMOTime) -> SUMOTime {
        if self.timeToTeleportBidi == SUMOTIME_MIN { defaultValue } else { self.timeToTeleportBidi }
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap SUMOVTypeParameter::getDefaultMaxAccelProfile(const SUMOVehicleClass, double)` [static]
    /// Returns the default maximum acceleration profile for the given vehicle class This needs to be a function because the actual profile is stored in the car following model
    // LinearApproxHelpers::LinearApproxMap
    // SUMOVTypeParameter::getDefaultMaxAccelProfile(const SUMOVehicleClass vc, double maxAccel) {
    //     UNUSED_PARAMETER(maxAccel);
    //     LinearApproxHelpers::LinearApproxMap result;
    //     std::vector<std::pair<double, double> > MaxAccelProfile;
    //     switch (vc) {
    //         case SVC_PEDESTRIAN:
    //         case SVC_BICYCLE:
    //         case SVC_MOTORCYCLE:
    //         case SVC_MOPED:
    //         case SVC_TRUCK:
    //         case SVC_TRAILER:
    //         case SVC_BUS:
    //         case SVC_COACH:
    //         case SVC_TRAM:
    //         case SVC_RAIL_URBAN:
    //         case SVC_RAIL:
    //         case SVC_RAIL_ELECTRIC:
    //         case SVC_RAIL_FAST:
    //         case SVC_SHIP:
    //         default:
    //             return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_default_max_accel_profile(vc: SUMOVehicleClass, maxAccel: f64) -> LinearApproxMap {
        // UNUSED_PARAMETER(maxAccel);
        let result: LinearApproxMap = LinearApproxMap::new();
        // every vehicle class leaves the (empty) result unchanged in C++
        match vc {
            SVC_PEDESTRIAN | SVC_BICYCLE | SVC_MOTORCYCLE | SVC_MOPED | SVC_TRUCK | SVC_TRAILER | SVC_BUS | SVC_COACH | SVC_TRAM | SVC_RAIL_URBAN | SVC_RAIL | SVC_RAIL_ELECTRIC | SVC_RAIL_FAST | SVC_SHIP => result,
            _ => result,
        }
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap SUMOVTypeParameter::getDefaultDesAccelProfile(const SUMOVehicleClass, double)` [static]
    /// Returns the default desired acceleration profile for the given vehicle class This needs to be a function because the actual profile is stored in the car following model
    // LinearApproxHelpers::LinearApproxMap
    // SUMOVTypeParameter::getDefaultDesAccelProfile(const SUMOVehicleClass vc, double desAccel) {
    //     UNUSED_PARAMETER(desAccel);
    //     LinearApproxHelpers::LinearApproxMap result;
    //     switch (vc) {
    //         case SVC_PEDESTRIAN:
    //         case SVC_BICYCLE:
    //         case SVC_MOTORCYCLE:
    //         case SVC_MOPED:
    //         case SVC_TRUCK:
    //         case SVC_TRAILER:
    //         case SVC_BUS:
    //         case SVC_COACH:
    //         case SVC_TRAM:
    //         case SVC_RAIL_URBAN:
    //         case SVC_RAIL:
    //         case SVC_RAIL_ELECTRIC:
    //         case SVC_RAIL_FAST:
    //         case SVC_SHIP:
    //         default:
    //             return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_default_des_accel_profile(vc: SUMOVehicleClass, desAccel: f64) -> LinearApproxMap {
        // UNUSED_PARAMETER(desAccel);
        let result: LinearApproxMap = LinearApproxMap::new();
        // every vehicle class leaves the (empty) result unchanged in C++
        match vc {
            SVC_PEDESTRIAN | SVC_BICYCLE | SVC_MOTORCYCLE | SVC_MOPED | SVC_TRUCK | SVC_TRAILER | SVC_BUS | SVC_COACH | SVC_TRAM | SVC_RAIL_URBAN | SVC_RAIL | SVC_RAIL_ELECTRIC | SVC_RAIL_FAST | SVC_SHIP => result,
            _ => result,
        }
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap SUMOVTypeParameter::getCFProfile(const SumoXMLAttr, const LinearApproxHelpers::LinearApproxMap &) const` [const]
    /// Returns the named value from the map, or the default if it is not contained there
    // LinearApproxHelpers::LinearApproxMap
    // SUMOVTypeParameter::getCFProfile(const SumoXMLAttr attr, const LinearApproxHelpers::LinearApproxMap& defaultProfile) const {
    //     if (cfParameter.count(attr)) {
    //         std::vector<double> speedTable = getCFValueTable(SUMO_ATTR_SPEED_TABLE);
    //         std::vector<double> valueTable = getCFValueTable(attr);
    //         if (valueTable.size() == 1) {
    //             throw ProcessError(TLF("Invalid size of % table for vType '%' (at least 2 values are required).", toString(attr), id));
    //         } else if (speedTable.size() != valueTable.size()) {
    //             throw ProcessError(TLF("Mismatching size of speedTable (%) and % table (%) for vType '%'.", speedTable.size(), toString(attr), valueTable.size(), id));
    //         }
    //         LinearApproxHelpers::LinearApproxMap result;
    //         for (int i = 0; i < (int)speedTable.size(); i++) {
    //             result[speedTable[i]] = valueTable[i];
    //         }
    //         return result;
    //     } else {
    //         return defaultProfile;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_cf_profile(&self, attr: SumoXMLAttr, defaultProfile: &LinearApproxMap) -> LinearApproxMap {
        if self.cfParameter.contains_key(&attr) {
            let speedTable = self.get_cf_value_table(SUMO_ATTR_SPEED_TABLE);
            let valueTable = self.get_cf_value_table(attr);
            if valueTable.len() == 1 {
                throw_process_error(utils::tlf!("Invalid size of % table for vType '%' (at least 2 values are required).", Attrs.get_string(attr), self.id));
            } else if speedTable.len() != valueTable.len() {
                throw_process_error(utils::tlf!(
                    "Mismatching size of speedTable (%) and % table (%) for vType '%'.",
                    speedTable.len(),
                    Attrs.get_string(attr),
                    valueTable.len(),
                    self.id
                ));
            }
            let mut result: LinearApproxMap = LinearApproxMap::new();
            for i in 0..speedTable.len() {
                linear_approx_map_set(&mut result, speedTable[i], valueTable[i]);
            }
            result
        } else {
            defaultProfile.clone()
        }
    }

    /// C++: `std::vector<std::pair<double, double>> SUMOVTypeParameter::createCFProfile(const SumoXMLAttr, const std::string) const` [const]
    /// Creates a vector of pairs from a string
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn create_cf_profile(&self, attr: SumoXMLAttr, profile: String) -> Vec<(f64, f64)> {
        // shortcut: C++ declares SUMOVTypeParameter::createCFProfile (SUMOVTypeParameter.h:566) but never defines it,
        // so any call is a link error in C++; upgrade only if a definition is added upstream
        panic!("SUMOVTypeParameter::createCFProfile is declared but not defined in C++ (attr {}, profile {})", Attrs.get_string(attr), profile)
    }

}

/// struct for default values that depend of VClass
/// C++ class `SUMOVTypeParameter::VClassDefaultValues` (utils/vehicle/SUMOVTypeParameter.h)
pub struct SUMOVTypeParameter_VClassDefaultValues {
    /// The physical vehicle length
    pub length: f64,
    /// This class' free space in front of the vehicle itself
    pub minGap: f64,
    /// The vehicle type's minimum lateral gap [m]
    pub minGapLat: f64,
    /// The vehicle type's maximum speed [m/s] (technical limit, not subject to speed deviation)
    pub maxSpeed: f64,
    /// The vehicle type's desired maximum speed [m/s]
    pub desiredMaxSpeed: f64,
    /// This class' width
    pub width: f64,
    /// This class' height
    pub height: f64,
    /// This class' shape
    pub shape: SUMOVehicleShape,
    /// The emission class of this vehicle
    pub emissionClass: SUMOEmissionClass,
    /// This class' mass
    pub mass: f64,
    /// The factor by which the maximum speed may deviate from the allowed max speed on the street
    pub speedFactor: DistributionParameterized,
    /// The person capacity of the vehicle
    pub personCapacity: i32,
    /// The container capacity of the vehicle
    pub containerCapacity: i32,
    /// 3D model file for this class
    pub osgFile: String,
    /// the length of train carriages
    pub carriageLength: f64,
    /// the length of train locomotive
    pub locomotiveLength: f64,
    /// the number of doors per carriage
    pub carriageDoors: i32,
    /// the width of the carriage doors
    pub carriageDoorWidth: f64,
    /// the maximum distance between platform and train
    pub maxPlatformDistance: f64,
    /// the lateral alignment procedure
    pub latAlignmentProcedure: LatAlignmentDefinition,
}

impl SUMOVTypeParameter_VClassDefaultValues {
    /// C++: `void VClassDefaultValues::VClassDefaultValues(SUMOVehicleClass)`
    /// parameter constructor
    // SUMOVTypeParameter::VClassDefaultValues::VClassDefaultValues(SUMOVehicleClass vclass) :
    //     length(getDefaultVehicleLength(vclass)),
    //     minGap(2.5),
    //     minGapLat(0.6),
    //     maxSpeed(200. / 3.6),
    //     desiredMaxSpeed(10000 / 3.6), // backward-compatibility: do not influence speeds by default
    //     width(DEFAULT_VEH_WIDTH),
    //     height(DEFAULT_VEH_HEIGHT),
    //     shape(SUMOVehicleShape::UNKNOWN),
    //     emissionClass(PollutantsInterface::getClassByName(EMPREFIX + "default", vclass)),
    //     mass(DEFAULT_VEH_MASS),
    //     speedFactor("normc", 1.0, 0.0, 0.2, 2.0),
    //     personCapacity(4),
    //     containerCapacity(0),
    //     osgFile("car-normal-citrus.obj"),
    //     carriageLength(-1),
    //     locomotiveLength(-1),
    //     carriageDoors(2),
    //     carriageDoorWidth(1.5),
    //     maxPlatformDistance(3.),
    //     latAlignmentProcedure(LatAlignmentDefinition::CENTER) {
    //     // sources for the default values should be in docs/Vehicle_Type_Parameter_Defaults.md
    //     switch (vclass) {
    //         case SVC_PEDESTRIAN:
    //             minGap = 0.25;
    //             maxSpeed = 37.58 / 3.6; // Usain Bolt
    //             desiredMaxSpeed = DEFAULT_PEDESTRIAN_SPEED;
    //             width = 0.478;
    //             height = 1.719;
    //             shape = SUMOVehicleShape::PEDESTRIAN;
    //             osgFile = "humanResting.obj";
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 70.; // https://en.wikipedia.org/wiki/Human_body_weight for Europe
    //             speedFactor.setParameter(1, 0.1);
    //             break;
    //         case SVC_WHEELCHAIR:
    //             minGap = 0.5;
    //             maxSpeed = 30.0 / 3.6; // https://en.wikipedia.org/wiki/Wheelchair_racing
    //             desiredMaxSpeed = DEFAULT_PEDESTRIAN_SPEED;
    //             width = 0.72;
    //             height = 1.2;
    //             shape = SUMOVehicleShape::PEDESTRIAN;
    //             osgFile = "humanResting.obj";
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 90.; // 20 (empty) + 70 (person)
    //             speedFactor.setParameter(1, 0.1);
    //             break;
    //         case SVC_BICYCLE:
    //             minGap = 0.5;
    //             minGapLat = 0.35;
    //             maxSpeed = 50. / 3.6;
    //             desiredMaxSpeed = DEFAULT_BICYCLE_SPEED;
    //             width = 0.65;
    //             height = 1.7;
    //             shape = SUMOVehicleShape::BICYCLE;
    //             personCapacity = 1;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 10.;
    //             speedFactor.setParameter(1, 0.1);
    //             latAlignmentProcedure = LatAlignmentDefinition::RIGHT;
    //             break;
    //         case SVC_SCOOTER:
    //             minGap = 0.5;
    //             minGapLat = 0.35;
    //             maxSpeed = 25 / 3.6;
    //             desiredMaxSpeed = DEFAULT_BICYCLE_SPEED;
    //             width = 0.5;
    //             height = 1.7;
    //             shape = SUMOVehicleShape::SCOOTER;
    //             personCapacity = 1;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 10.;
    //             speedFactor.setParameter(1, 0.1);
    //             latAlignmentProcedure = LatAlignmentDefinition::RIGHT;
    //             break;
    //         case SVC_MOPED:
    //             maxSpeed = 60. / 3.6;
    //             width = 0.78;
    //             height = 1.7;
    //             shape = SUMOVehicleShape::MOPED;
    //             personCapacity = 1;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "Moped_le50cc_Euro-2", vclass);
    //             mass = 80.;
    //             speedFactor.setParameter(1, 0.1);
    //             break;
    //         case SVC_MOTORCYCLE:
    //             width = 0.9;
    //             height = 1.5;
    //             shape = SUMOVehicleShape::MOTORCYCLE;
    //             personCapacity = 1;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "MC_4S_gt250cc_preEuro", vclass);
    //             mass = 200.;
    //             speedFactor.setParameter(1, 0.1);
    //             break;
    //         case SVC_TRUCK:
    //             maxSpeed = 130. / 3.6;
    //             width = 2.4;
    //             height = 2.4;
    //             shape = SUMOVehicleShape::TRUCK;
    //             osgFile = "car-microcargo-citrus.obj";
    //             personCapacity = 2;
    //             containerCapacity = 1;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "RT_le7.5t_Euro-VI_A-C", vclass);
    //             mass = 4500.;
    //             speedFactor.setParameter(1, 0.05);
    //             break;
    //         case SVC_TRAILER:
    //             maxSpeed = 130. / 3.6;
    //             width = 2.55;
    //             height = 4.;
    //             shape = SUMOVehicleShape::TRUCK_1TRAILER;
    //             osgFile = "car-microcargo-citrus.obj";
    //             personCapacity = 2;
    //             containerCapacity = 2;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "TT_AT_gt34-40t_Euro-VI_A-C", vclass);
    //             mass = 13000.;
    //             speedFactor.setParameter(1, 0.05);
    //             break;
    //         case SVC_BUS:
    //             maxSpeed = 100. / 3.6;
    //             width = 2.5;
    //             height = 3.4;
    //             shape = SUMOVehicleShape::BUS;
    //             osgFile = "car-minibus-citrus.obj";
    //             personCapacity = 85;
    //             // the following values keep better consistency with the default bus length,
    //             // although the most frequent city bus seems to have 3 axes and >18t according to the HBEFA4 data
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "UBus_Std_gt15-18t_Euro-VI_A-C", vclass);
    //             mass = 12000.;
    //             break;
    //         case SVC_COACH:
    //             maxSpeed = 100. / 3.6;
    //             width = 2.6;
    //             height = 4.;
    //             shape = SUMOVehicleShape::BUS_COACH;
    //             osgFile = "car-minibus-citrus.obj";
    //             personCapacity = 70;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "Coach_3-Axes_gt18t_Euro-VI_A-C", vclass);
    //             mass = 25000.;
    //             speedFactor.setParameter(1, 0.05);
    //             break;
    //         case SVC_TRAM:
    //             maxSpeed = 80. / 3.6;
    //             width = 2.4;
    //             height = 3.2;
    //             shape = SUMOVehicleShape::RAIL_CAR;
    //             osgFile = "tram.obj";
    //             personCapacity = 120;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 37900.;
    //             break;
    //         case SVC_RAIL_URBAN:
    //         case SVC_SUBWAY:
    //             maxSpeed = 100. / 3.6;
    //             minGap = 5;
    //             width = 3.0;
    //             height = 3.6;
    //             shape = SUMOVehicleShape::RAIL_CAR;
    //             personCapacity = 300;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 59000.;
    //             break;
    //         case SVC_RAIL:
    //             maxSpeed = 160. / 3.6;
    //             minGap = 5;
    //             width = 2.84;
    //             height = 3.75;
    //             shape = SUMOVehicleShape::RAIL;
    //             personCapacity = 434;
    //             // slight understatement (-:
    //             emissionClass = PollutantsInterface::getClassByName("HBEFA3/HDV_D_EU0", vclass);
    //             mass = 79500.; // only locomotive
    //             break;
    //         case SVC_RAIL_ELECTRIC:
    //             maxSpeed = 220. / 3.6;
    //             minGap = 5;
    //             width = 2.95;
    //             height = 3.89;
    //             shape = SUMOVehicleShape::RAIL;
    //             personCapacity = 425;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 83000.; // only locomotive
    //             break;
    //         case SVC_RAIL_FAST:
    //             maxSpeed = 330. / 3.6;
    //             minGap = 5;
    //             width = 2.95;
    //             height = 3.89;
    //             shape = SUMOVehicleShape::RAIL;
    //             personCapacity = 425;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             mass = 409000.;
    //             break;
    //         case SVC_DELIVERY:
    //             width = 2.16;
    //             height = 2.86;
    //             shape = SUMOVehicleShape::DELIVERY;
    //             personCapacity = 2;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "LCV_diesel_N1-III_Euro-6ab", vclass);
    //             mass = 5000.;
    //             speedFactor.setParameter(1, 0.05);
    //             break;
    //         case SVC_EMERGENCY:
    //             width = 2.16;
    //             height = 2.86;
    //             shape = SUMOVehicleShape::DELIVERY;
    //             personCapacity = 2;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "LCV_diesel_N1-III_Euro-6ab", vclass);
    //             mass = 5000.;
    //             break;
    //         case SVC_PRIVATE:
    //         case SVC_VIP:
    //         case SVC_PASSENGER:
    //         case SVC_HOV:
    //         case SVC_CUSTOM1:
    //         case SVC_CUSTOM2:
    //             shape = SUMOVehicleShape::PASSENGER;
    //             speedFactor.setParameter(1, 0.1);
    //             break;
    //         case SVC_TAXI:
    //             shape = SUMOVehicleShape::TAXI;
    //             speedFactor.setParameter(1, 0.05);
    //             break;
    //         case SVC_E_VEHICLE:
    //             shape = SUMOVehicleShape::E_VEHICLE;
    //             emissionClass = PollutantsInterface::getClassByName(EMPREFIX + "zero", vclass);
    //             speedFactor.setParameter(1, 0.1);
    //             break;
    //         case SVC_CONTAINER:
    //             // ISO Container TEU
    //             width = 2.438;
    //             height = 2.591;
    //             break;
    //         case SVC_DRONE:
    //             width = 0.5;
    //             break;
    //         case SVC_AIRCRAFT:
    //             // Airbus A380
    //             shape = SUMOVehicleShape::AIRCRAFT;
    //             width = 79.8;
    //             break;
    //         case SVC_SHIP:
    //             width = 4;
    //             maxSpeed = 8 / 1.94; // 8 knots
    //             height = 4;
    //             shape = SUMOVehicleShape::SHIP;
    //             // slight understatement (-:
    //             emissionClass = PollutantsInterface::getClassByName("HBEFA3/HDV_D_EU0", vclass);
    //             mass = 100000.;
    //             speedFactor.setParameter(1, 0.1);
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(vClass: SUMOVehicleClass) -> Self {
        let mut result = SUMOVTypeParameter_VClassDefaultValues {
            length: get_default_vehicle_length(vClass),
            minGap: 2.5,
            minGapLat: 0.6,
            maxSpeed: 200. / 3.6,
            desiredMaxSpeed: 10000. / 3.6, // backward-compatibility: do not influence speeds by default
            width: DEFAULT_VEH_WIDTH,
            height: DEFAULT_VEH_HEIGHT,
            shape: SUMOVehicleShape::Unknown,
            emissionClass: PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}default"), vClass),
            mass: DEFAULT_VEH_MASS,
            speedFactor: DistributionParameterized::with_bounds("normc", 1.0, 0.0, 0.2, 2.0),
            personCapacity: 4,
            containerCapacity: 0,
            osgFile: "car-normal-citrus.obj".to_string(),
            carriageLength: -1.,
            locomotiveLength: -1.,
            carriageDoors: 2,
            carriageDoorWidth: 1.5,
            maxPlatformDistance: 3.,
            latAlignmentProcedure: LatAlignmentDefinition::CENTER,
        };
        // sources for the default values should be in docs/Vehicle_Type_Parameter_Defaults.md
        match vClass {
            SVC_PEDESTRIAN => {
                result.minGap = 0.25;
                result.maxSpeed = 37.58 / 3.6; // Usain Bolt
                result.desiredMaxSpeed = DEFAULT_PEDESTRIAN_SPEED;
                result.width = 0.478;
                result.height = 1.719;
                result.shape = SUMOVehicleShape::Pedestrian;
                result.osgFile = "humanResting.obj".to_string();
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 70.; // https://en.wikipedia.org/wiki/Human_body_weight for Europe
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
            }
            SVC_WHEELCHAIR => {
                result.minGap = 0.5;
                result.maxSpeed = 30.0 / 3.6; // https://en.wikipedia.org/wiki/Wheelchair_racing
                result.desiredMaxSpeed = DEFAULT_PEDESTRIAN_SPEED;
                result.width = 0.72;
                result.height = 1.2;
                result.shape = SUMOVehicleShape::Pedestrian;
                result.osgFile = "humanResting.obj".to_string();
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 90.; // 20 (empty) + 70 (person)
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
            }
            SVC_BICYCLE => {
                result.minGap = 0.5;
                result.minGapLat = 0.35;
                result.maxSpeed = 50. / 3.6;
                result.desiredMaxSpeed = DEFAULT_BICYCLE_SPEED;
                result.width = 0.65;
                result.height = 1.7;
                result.shape = SUMOVehicleShape::Bicycle;
                result.personCapacity = 1;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 10.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
                result.latAlignmentProcedure = LatAlignmentDefinition::RIGHT;
            }
            SVC_SCOOTER => {
                result.minGap = 0.5;
                result.minGapLat = 0.35;
                result.maxSpeed = 25. / 3.6;
                result.desiredMaxSpeed = DEFAULT_BICYCLE_SPEED;
                result.width = 0.5;
                result.height = 1.7;
                result.shape = SUMOVehicleShape::Scooter;
                result.personCapacity = 1;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 10.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
                result.latAlignmentProcedure = LatAlignmentDefinition::RIGHT;
            }
            SVC_MOPED => {
                result.maxSpeed = 60. / 3.6;
                result.width = 0.78;
                result.height = 1.7;
                result.shape = SUMOVehicleShape::Moped;
                result.personCapacity = 1;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}Moped_le50cc_Euro-2"), vClass);
                result.mass = 80.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
            }
            SVC_MOTORCYCLE => {
                result.width = 0.9;
                result.height = 1.5;
                result.shape = SUMOVehicleShape::Motorcycle;
                result.personCapacity = 1;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}MC_4S_gt250cc_preEuro"), vClass);
                result.mass = 200.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
            }
            SVC_TRUCK => {
                result.maxSpeed = 130. / 3.6;
                result.width = 2.4;
                result.height = 2.4;
                result.shape = SUMOVehicleShape::Truck;
                result.osgFile = "car-microcargo-citrus.obj".to_string();
                result.personCapacity = 2;
                result.containerCapacity = 1;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}RT_le7.5t_Euro-VI_A-C"), vClass);
                result.mass = 4500.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.05);
            }
            SVC_TRAILER => {
                result.maxSpeed = 130. / 3.6;
                result.width = 2.55;
                result.height = 4.;
                result.shape = SUMOVehicleShape::Truck1Trailer;
                result.osgFile = "car-microcargo-citrus.obj".to_string();
                result.personCapacity = 2;
                result.containerCapacity = 2;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}TT_AT_gt34-40t_Euro-VI_A-C"), vClass);
                result.mass = 13000.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.05);
            }
            SVC_BUS => {
                result.maxSpeed = 100. / 3.6;
                result.width = 2.5;
                result.height = 3.4;
                result.shape = SUMOVehicleShape::Bus;
                result.osgFile = "car-minibus-citrus.obj".to_string();
                result.personCapacity = 85;
                // the following values keep better consistency with the default bus length,
                // although the most frequent city bus seems to have 3 axes and >18t according to the HBEFA4 data
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}UBus_Std_gt15-18t_Euro-VI_A-C"), vClass);
                result.mass = 12000.;
            }
            SVC_COACH => {
                result.maxSpeed = 100. / 3.6;
                result.width = 2.6;
                result.height = 4.;
                result.shape = SUMOVehicleShape::BusCoach;
                result.osgFile = "car-minibus-citrus.obj".to_string();
                result.personCapacity = 70;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}Coach_3-Axes_gt18t_Euro-VI_A-C"), vClass);
                result.mass = 25000.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.05);
            }
            SVC_TRAM => {
                result.maxSpeed = 80. / 3.6;
                result.width = 2.4;
                result.height = 3.2;
                result.shape = SUMOVehicleShape::RailCar;
                result.osgFile = "tram.obj".to_string();
                result.personCapacity = 120;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 37900.;
            }
            SVC_RAIL_URBAN | SVC_SUBWAY => {
                result.maxSpeed = 100. / 3.6;
                result.minGap = 5.;
                result.width = 3.0;
                result.height = 3.6;
                result.shape = SUMOVehicleShape::RailCar;
                result.personCapacity = 300;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 59000.;
            }
            SVC_RAIL => {
                result.maxSpeed = 160. / 3.6;
                result.minGap = 5.;
                result.width = 2.84;
                result.height = 3.75;
                result.shape = SUMOVehicleShape::Rail;
                result.personCapacity = 434;
                // slight understatement (-:
                result.emissionClass = PollutantsInterface::get_class_by_name("HBEFA3/HDV_D_EU0", vClass);
                result.mass = 79500.; // only locomotive
            }
            SVC_RAIL_ELECTRIC => {
                result.maxSpeed = 220. / 3.6;
                result.minGap = 5.;
                result.width = 2.95;
                result.height = 3.89;
                result.shape = SUMOVehicleShape::Rail;
                result.personCapacity = 425;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 83000.; // only locomotive
            }
            SVC_RAIL_FAST => {
                result.maxSpeed = 330. / 3.6;
                result.minGap = 5.;
                result.width = 2.95;
                result.height = 3.89;
                result.shape = SUMOVehicleShape::Rail;
                result.personCapacity = 425;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                result.mass = 409000.;
            }
            SVC_DELIVERY => {
                result.width = 2.16;
                result.height = 2.86;
                result.shape = SUMOVehicleShape::Delivery;
                result.personCapacity = 2;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}LCV_diesel_N1-III_Euro-6ab"), vClass);
                result.mass = 5000.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.05);
            }
            SVC_EMERGENCY => {
                result.width = 2.16;
                result.height = 2.86;
                result.shape = SUMOVehicleShape::Delivery;
                result.personCapacity = 2;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}LCV_diesel_N1-III_Euro-6ab"), vClass);
                result.mass = 5000.;
            }
            SVC_PRIVATE | SVC_VIP | SVC_PASSENGER | SVC_HOV | SVC_CUSTOM1 | SVC_CUSTOM2 => {
                result.shape = SUMOVehicleShape::Passenger;
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
            }
            SVC_TAXI => {
                result.shape = SUMOVehicleShape::Taxi;
                set_speed_factor_deviation(&mut result.speedFactor, 0.05);
            }
            SVC_E_VEHICLE => {
                result.shape = SUMOVehicleShape::EVehicle;
                result.emissionClass = PollutantsInterface::get_class_by_name(&format!("{EMPREFIX}zero"), vClass);
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
            }
            SVC_CONTAINER => {
                // ISO Container TEU
                result.width = 2.438;
                result.height = 2.591;
            }
            SVC_DRONE => {
                result.width = 0.5;
            }
            SVC_AIRCRAFT => {
                // Airbus A380
                result.shape = SUMOVehicleShape::Aircraft;
                result.width = 79.8;
            }
            SVC_SHIP => {
                result.width = 4.;
                result.maxSpeed = 8. / 1.94; // 8 knots
                result.height = 4.;
                result.shape = SUMOVehicleShape::Ship;
                // slight understatement (-:
                result.emissionClass = PollutantsInterface::get_class_by_name("HBEFA3/HDV_D_EU0", vClass);
                result.mass = 100000.;
                set_speed_factor_deviation(&mut result.speedFactor, 0.1);
            }
            _ => {}
        }
        result
    }

    /// C++: `void VClassDefaultValues::VClassDefaultValues()` [private]
    /// default constructor
    // SUMOVTypeParameter::VClassDefaultValues::VClassDefaultValues() :
    //     speedFactor("normc", 1.0, 0.0, 0.2, 2.0) {}
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        // the C++ default constructor leaves the other members uninitialised; zero them here
        SUMOVTypeParameter_VClassDefaultValues {
            length: 0.,
            minGap: 0.,
            minGapLat: 0.,
            maxSpeed: 0.,
            desiredMaxSpeed: 0.,
            width: 0.,
            height: 0.,
            shape: SUMOVehicleShape::Unknown,
            emissionClass: 0,
            mass: 0.,
            speedFactor: DistributionParameterized::with_bounds("normc", 1.0, 0.0, 0.2, 2.0),
            personCapacity: 0,
            containerCapacity: 0,
            osgFile: String::new(),
            carriageLength: 0.,
            locomotiveLength: 0.,
            carriageDoors: 0,
            carriageDoorWidth: 0.,
            maxPlatformDistance: 0.,
            latAlignmentProcedure: LatAlignmentDefinition::DEFAULT,
        }
    }

}

