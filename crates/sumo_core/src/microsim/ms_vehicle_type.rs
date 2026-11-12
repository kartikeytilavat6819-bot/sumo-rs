//! Port of microsim/MSVehicleType.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering as AtomicOrdering};
use utils::common::msg_handler::write_warning;
use utils::common::std_defs::{g_precision_random, round_decimal};
use utils::common::string_utils::to_bool;
use utils::common::sumo_vehicle_class::{get_vehicle_class_names, parse_vehicle_classes_vec, SVC_IGNORING, SVC_PEDESTRIAN};
use utils::common::time::{delta_t, steps2time, ts};
use utils::common::to_string::{f64_to_string, f64_to_string_default};
use utils::common::util_exceptions::ProcessError;
use utils::options::options_cont::{get_bool, get_float, get_string_vector, is_default};

// C++ static `int MSVehicleType::myNextIndex = 0;`
static MY_NEXT_INDEX: AtomicI32 = AtomicI32::new(0);

// VTYPEPARS_*_SET bits from utils/vehicle/SUMOVTypeParameter.h (the ported SUMOVTypeParameter module does not export them yet)
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
const VTYPEPARS_IMPATIENCE_SET: i64 = 1 << 13;
const VTYPEPARS_MAXSPEED_LAT_SET: i64 = 1 << 20;
const VTYPEPARS_LATALIGNMENT_SET: i64 = 1 << 21;
const VTYPEPARS_MINGAP_LAT_SET: i64 = 1 << 22;
const VTYPEPARS_ACTIONSTEPLENGTH_SET: i64 = 1 << 23;
const VTYPEPARS_BOARDING_DURATION: i64 = 1 << 16;
const VTYPEPARS_MASS_SET: i64 = 1i64 << 32;
const VTYPEPARS_PARKING_BADGES_SET: i64 = 1i64 << 37;

// shortcut: MSGlobals statics are not ported yet (ms_globals.rs is empty); these return the C++ defaults, upgrade once MSGlobals is filled
fn g_action_step_length() -> SUMOTime {
    0
}
fn g_use_meso_sim() -> bool {
    false
}

/// C++ implicit copy constructor of SUMOVTypeParameter (the ported struct has no Clone impl)
fn copy_sumo_vtype_parameter(p: &SUMOVTypeParameter) -> SUMOVTypeParameter {
    SUMOVTypeParameter {
        base: p.base.clone(),
        id: p.id.clone(),
        length: p.length.clone(),
        minGap: p.minGap.clone(),
        maxSpeed: p.maxSpeed.clone(),
        desiredMaxSpeed: p.desiredMaxSpeed.clone(),
        actionStepLength: p.actionStepLength.clone(),
        defaultProbability: p.defaultProbability.clone(),
        speedFactor: p.speedFactor.clone(),
        emissionClass: p.emissionClass.clone(),
        mass: p.mass.clone(),
        color: p.color.clone(),
        vehicleClass: p.vehicleClass.clone(),
        impatience: p.impatience.clone(),
        personCapacity: p.personCapacity.clone(),
        containerCapacity: p.containerCapacity.clone(),
        boardingDuration: p.boardingDuration.clone(),
        loadingDuration: p.loadingDuration.clone(),
        scale: p.scale.clone(),
        width: p.width.clone(),
        height: p.height.clone(),
        shape: p.shape.clone(),
        osgFile: p.osgFile.clone(),
        imgFile: p.imgFile.clone(),
        carriageImages: p.carriageImages.clone(),
        cfModel: p.cfModel.clone(),
        cfParameter: p.cfParameter.clone(),
        lcParameter: p.lcParameter.clone(),
        jmParameter: p.jmParameter.clone(),
        lcModel: p.lcModel.clone(),
        maxSpeedLat: p.maxSpeedLat.clone(),
        latAlignmentOffset: p.latAlignmentOffset.clone(),
        latAlignmentProcedure: p.latAlignmentProcedure.clone(),
        minGapLat: p.minGapLat.clone(),
        scaleVisual: p.scaleVisual.clone(),
        carriageLength: p.carriageLength.clone(),
        locomotiveLength: p.locomotiveLength.clone(),
        carriageGap: p.carriageGap.clone(),
        carriageDoors: p.carriageDoors.clone(),
        carriageDoorWidth: p.carriageDoorWidth.clone(),
        maxPlatformDistance: p.maxPlatformDistance.clone(),
        timeToTeleport: p.timeToTeleport.clone(),
        timeToTeleportBidi: p.timeToTeleportBidi.clone(),
        speedFactorPremature: p.speedFactorPremature.clone(),
        frontSeatPos: p.frontSeatPos.clone(),
        seatingWidth: p.seatingWidth.clone(),
        parkingBadges: p.parkingBadges.clone(),
        boardingFactor: p.boardingFactor.clone(),
        parametersSet: p.parametersSet.clone(),
        saved: p.saved.clone(),
        onlyReferenced: p.onlyReferenced.clone(),
        paramRestrictions: p.paramRestrictions.clone(),
        myManoeuverAngleTimes: p.myManoeuverAngleTimes.clone(),
    }
}

/// The car-following model and parameter
/// C++ class `MSVehicleType` (microsim/MSVehicleType.h)
pub struct MSVehicleType {
    /// the parameter container
    pub myParameter: SUMOVTypeParameter,
    pub myEnergyParams: EnergyParams,
    /// the vtypes actionsStepLength in seconds (cached because needed very often)
    pub myCachedActionStepLengthSecs: f64,
    /// Indicator whether the user was already warned once about an action step length larger than the desired time headway.
    pub myWarnedActionStepLengthTauOnce: bool,
    pub myWarnedActionStepLengthBallisticOnce: bool,
    pub myWarnedStepLengthTauOnce: bool,
    /// the running index
    pub myIndex: i32,
    /// instance of the car following model.
    pub myCarFollowModel: Ptr<MSCFModel>,
    /// The original type
    pub myOriginalType: Ptr<MSVehicleType>,
}

// C++ static members of MSVehicleType (port as module-level statics, see AGENTS.md):
//   static int MSVehicleType::myNextIndex;

impl MSVehicleType {
    /// C++: `void MSVehicleType::MSVehicleType(const SUMOVTypeParameter &)`
    /// Constructor.
    // MSVehicleType::MSVehicleType(const SUMOVTypeParameter& parameter) :
    //     myParameter(parameter),
    //     myEnergyParams(&parameter),
    //     myWarnedActionStepLengthTauOnce(false),
    //     myWarnedActionStepLengthBallisticOnce(false),
    //     myWarnedStepLengthTauOnce(false),
    //     myIndex(myNextIndex++),
    //     myCarFollowModel(nullptr),
    //     myOriginalType(nullptr) {
    //     assert(getLength() > 0);
    //     assert(getMaxSpeed() > 0);
    //
    //     // Check if actionStepLength was set by user, if not init to global default
    //     if (!myParameter.wasSet(VTYPEPARS_ACTIONSTEPLENGTH_SET)) {
    //         myParameter.actionStepLength = MSGlobals::gActionStepLength;
    //     }
    //     myCachedActionStepLengthSecs = STEPS2TIME(myParameter.actionStepLength);
    // }
    #[allow(unused_variables)]
    pub fn new(parameter: &SUMOVTypeParameter) -> Self {
        let mut v = MSVehicleType {
            myParameter: copy_sumo_vtype_parameter(parameter),
            myEnergyParams: EnergyParams::new(Ptr::new(parameter)),
            myCachedActionStepLengthSecs: 0.,
            myWarnedActionStepLengthTauOnce: false,
            myWarnedActionStepLengthBallisticOnce: false,
            myWarnedStepLengthTauOnce: false,
            myIndex: MY_NEXT_INDEX.fetch_add(1, AtomicOrdering::Relaxed),
            myCarFollowModel: Ptr::null(),
            myOriginalType: Ptr::null(),
        };
        debug_assert!(v.get_length() > 0.);
        debug_assert!(v.get_max_speed() > 0.);
        // Check if actionStepLength was set by user, if not init to global default
        if !v.was_set(VTYPEPARS_ACTIONSTEPLENGTH_SET) {
            v.myParameter.actionStepLength = g_action_step_length();
        }
        v.myCachedActionStepLengthSecs = steps2time(v.myParameter.actionStepLength);
        v
    }

    /// C++: `void MSVehicleType::~MSVehicleType()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: C++ `delete myCarFollowModel` is not done, the model is Box::leak'ed by build()/duplicate_type() and has no owner to free it, upgrade when vehicle types are owned by MSVehicleControl
    }

    /// C++: `bool MSVehicleType::wasSet(long long) const` [const]
    /// Returns whether the given parameter was set
    // bool wasSet(long long int what) const {
    //         return (myParameter.parametersSet & what) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_set(&self, what: i64) -> bool {
        (self.myParameter.parametersSet & what) != 0
    }

    /// C++: `const std::string & MSVehicleType::getID() const` [const]
    /// Returns the name of the vehicle type
    // const std::string& getID() const {
    //         return myParameter.id;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        &self.myParameter.id
    }

    /// C++: `int MSVehicleType::getNumericalID() const` [const]
    /// Returns the running index of the vehicle type
    // int getNumericalID() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myIndex
    }

    /// C++: `double MSVehicleType::getLength() const` [const]
    /// Get vehicle's length [m]
    // double getLength() const {
    //         return myParameter.length;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myParameter.length
    }

    /// C++: `double MSVehicleType::getLengthWithGap() const` [const]
    /// Get vehicle's length including the minimum gap [m]
    // double getLengthWithGap() const {
    //         return myParameter.length + myParameter.minGap;
    //     }
    #[allow(unused_variables)]
    pub fn get_length_with_gap(&self) -> f64 {
        self.myParameter.length + self.myParameter.minGap
    }

    /// C++: `double MSVehicleType::getMinGap() const` [const]
    /// Get the free space in front of vehicles of this class
    // double getMinGap() const {
    //         return myParameter.minGap;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_gap(&self) -> f64 {
        self.myParameter.minGap
    }

    /// C++: `double MSVehicleType::getMinGapLat() const` [const]
    /// Get the minimum lateral gap that vehicles of this type maintain
    // double getMinGapLat() const {
    //         return myParameter.minGapLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_gap_lat(&self) -> f64 {
        self.myParameter.minGapLat
    }

    /// C++: `const MSCFModel & MSVehicleType::getCarFollowModel() const` [const]
    /// Returns the vehicle type's car following model definition (const version)
    // inline const MSCFModel& getCarFollowModel() const {
    //         return *myCarFollowModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_follow_model(&self) -> &MSCFModel {
        &*self.myCarFollowModel
    }

    /// C++: `MSCFModel & MSVehicleType::getCarFollowModel()`
    /// Returns the vehicle type's car following model definition (non-const version)
    // inline MSCFModel& getCarFollowModel() {
    //         return *myCarFollowModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_follow_model_2(&mut self) -> &mut MSCFModel {
        self.myCarFollowModel.m()
    }

    /// C++: `LaneChangeModel MSVehicleType::getLaneChangeModel() const` [const]
    // inline LaneChangeModel getLaneChangeModel() const {
    //         return myParameter.lcModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_change_model(&self) -> LaneChangeModel {
        self.myParameter.lcModel.clone()
    }

    /// C++: `double MSVehicleType::getMaxSpeed() const` [const]
    /// Get vehicle's (technical) maximum speed [m/s].
    // double getMaxSpeed() const {
    //         return myParameter.maxSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        self.myParameter.maxSpeed
    }

    /// C++: `double MSVehicleType::getDesiredMaxSpeed() const` [const]
    /// Returns the vehicles's desired maximum speed
    // double getDesiredMaxSpeed() const {
    //         return myParameter.desiredMaxSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_desired_max_speed(&self) -> f64 {
        self.myParameter.desiredMaxSpeed
    }

    /// C++: `double MSVehicleType::computeChosenSpeedDeviation(double, SumoRNG *, const double) const` [const]
    /// Computes and returns the speed deviation
    // double
    // MSVehicleType::computeChosenSpeedDeviation(double speedFactorOverride, SumoRNG* rng, const double minDev) const {
    //     return speedFactorOverride < 0
    //         ? roundDecimal(MAX2(minDev, myParameter.speedFactor.sample(rng)), gPrecisionRandom)
    //         : speedFactorOverride;
    // }
    #[allow(unused_variables)]
    pub fn compute_chosen_speed_deviation(&self, speedFactorOverride: f64, rng: Ptr<SumoRNG>, minDev: f64) -> f64 {
        if speedFactorOverride < 0. {
            // shortcut: SUMOVTypeParameter::speedFactor is Unmapped, so Distribution_Parameterized::sample(rng) is unavailable; uses its mean 1.0 with no deviation until that field is mapped
            round_decimal(if minDev > 1.0 { minDev } else { 1.0 }, g_precision_random())
        } else {
            speedFactorOverride
        }
    }

    /// C++: `double MSVehicleType::getDefaultProbability() const` [const]
    /// Get the default probability of this vehicle type
    // double getDefaultProbability() const {
    //         return myParameter.defaultProbability;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_probability(&self) -> f64 {
        self.myParameter.defaultProbability
    }

    /// C++: `SUMOVehicleClass MSVehicleType::getVehicleClass() const` [const]
    /// Get this vehicle type's vehicle class
    // SUMOVehicleClass getVehicleClass() const {
    //         return myParameter.vehicleClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_class(&self) -> SUMOVehicleClass {
        self.myParameter.vehicleClass
    }

    /// C++: `SUMOEmissionClass MSVehicleType::getEmissionClass() const` [const]
    /// Get this vehicle type's emission class
    // SUMOEmissionClass getEmissionClass() const {
    //         return myParameter.emissionClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_emission_class(&self) -> SUMOEmissionClass {
        self.myParameter.emissionClass.clone()
    }

    /// C++: `double MSVehicleType::getMass() const` [const]
    /// Get this vehicle type's mass
    // inline double getMass() const {
    //         return myEnergyParams.getDouble(SUMO_ATTR_MASS);
    //     }
    #[allow(unused_variables)]
    pub fn get_mass(&self) -> f64 {
        self.myEnergyParams.get_double(SUMO_ATTR_MASS)
    }

    /// C++: `const RGBColor & MSVehicleType::getColor() const` [const]
    /// Returns this type's color
    // const RGBColor& getColor() const {
    //         return myParameter.color;
    //     }
    #[allow(unused_variables)]
    pub fn get_color(&self) -> &RGBColor {
        &self.myParameter.color
    }

    /// C++: `const std::vector<std::string> & MSVehicleType::getParkingBadges() const` [const]
    /// Returns the parking access rights of this type
    // const std::vector<std::string>& getParkingBadges() const {
    //         return myParameter.parkingBadges;
    //     }
    #[allow(unused_variables)]
    pub fn get_parking_badges(&self) -> &Vec<String> {
        &self.myParameter.parkingBadges
    }

    /// C++: `const Distribution_Parameterized & MSVehicleType::getSpeedFactor() const` [const]
    /// Returns this type's speed factor
    // const Distribution_Parameterized& getSpeedFactor() const {
    //         return myParameter.speedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_factor(&self) -> &Unmapped /* Distribution_Parameterized */ {
        &self.myParameter.speedFactor
    }

    /// C++: `SUMOTime MSVehicleType::getActionStepLength() const` [const]
    /// Returns this type's default action step length
    // SUMOTime getActionStepLength() const {
    //         return myParameter.actionStepLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_action_step_length(&self) -> SUMOTime {
        self.myParameter.actionStepLength
    }

    /// C++: `double MSVehicleType::getActionStepLengthSecs() const` [const]
    /// Returns this type's default action step length in seconds
    // double getActionStepLengthSecs() const {
    //         return myCachedActionStepLengthSecs;
    //     }
    #[allow(unused_variables)]
    pub fn get_action_step_length_secs(&self) -> f64 {
        self.myCachedActionStepLengthSecs
    }

    /// C++: `double MSVehicleType::getImpatience() const` [const]
    /// Returns this type's impatience
    // double getImpatience() const {
    //         return myParameter.impatience;
    //     }
    #[allow(unused_variables)]
    pub fn get_impatience(&self) -> f64 {
        self.myParameter.impatience
    }

    /// C++: `double MSVehicleType::getWidth() const` [const]
    /// Get the width which vehicles of this class shall have when being drawn
    // double getWidth() const {
    //         return myParameter.width;
    //     }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        self.myParameter.width
    }

    /// C++: `double MSVehicleType::getSeatingWidth() const` [const]
    /// Get the width of the passenger compartment when being drawn
    // double getSeatingWidth() const {
    //         return myParameter.seatingWidth >= 0 ? myParameter.seatingWidth : myParameter.width;
    //     }
    #[allow(unused_variables)]
    pub fn get_seating_width(&self) -> f64 {
        if self.myParameter.seatingWidth >= 0. {
            self.myParameter.seatingWidth
        } else {
            self.myParameter.width
        }
    }

    /// C++: `double MSVehicleType::getHeight() const` [const]
    /// Get the height which vehicles of this class shall have when being drawn
    // double getHeight() const {
    //         return myParameter.height;
    //     }
    #[allow(unused_variables)]
    pub fn get_height(&self) -> f64 {
        self.myParameter.height
    }

    /// C++: `SUMOVehicleShape MSVehicleType::getGuiShape() const` [const]
    /// Get this vehicle type's shape
    // SUMOVehicleShape getGuiShape() const {
    //         return myParameter.shape;
    //     }
    #[allow(unused_variables)]
    pub fn get_gui_shape(&self) -> SUMOVehicleShape {
        self.myParameter.shape.clone()
    }

    /// C++: `std::string MSVehicleType::getOSGFile() const` [const]
    /// Get this vehicle type's 3D model file name
    // std::string getOSGFile() const {
    //         return myParameter.osgFile;
    //     }
    #[allow(unused_variables)]
    pub fn get_osg_file(&self) -> String {
        self.myParameter.osgFile.clone()
    }

    /// C++: `std::string MSVehicleType::getImgFile() const` [const]
    /// Get this vehicle type's raster model file name
    // std::string getImgFile() const {
    //         return myParameter.imgFile;
    //     }
    #[allow(unused_variables)]
    pub fn get_img_file(&self) -> String {
        self.myParameter.imgFile.clone()
    }

    /// C++: `int MSVehicleType::getPersonCapacity() const` [const]
    /// Get this vehicle type's person capacity
    // int getPersonCapacity() const {
    //         return myParameter.personCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_person_capacity(&self) -> i32 {
        self.myParameter.personCapacity
    }

    /// C++: `int MSVehicleType::getContainerCapacity() const` [const]
    /// Get this vehicle type's container capacity
    // int getContainerCapacity() const {
    //         return myParameter.containerCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_container_capacity(&self) -> i32 {
        self.myParameter.containerCapacity
    }

    /// C++: `SUMOTime MSVehicleType::getLoadingDuration(const bool) const` [const]
    /// Get this vehicle type's loading duration
    // SUMOTime getLoadingDuration(const bool isPerson) const {
    //         return isPerson ? myParameter.boardingDuration : myParameter.loadingDuration;
    //     }
    #[allow(unused_variables)]
    pub fn get_loading_duration(&self, isPerson: bool) -> SUMOTime {
        if isPerson {
            self.myParameter.boardingDuration
        } else {
            self.myParameter.loadingDuration
        }
    }

    /// C++: `SUMOTime MSVehicleType::getBoardingDuration(const bool) const` [const]
    /// Get this vehicle type's boarding duration
    // SUMOTime getBoardingDuration(const bool isPerson) const {
    //         return isPerson ? myParameter.boardingDuration : myParameter.loadingDuration;
    //     }
    #[allow(unused_variables)]
    pub fn get_boarding_duration(&self, isPerson: bool) -> SUMOTime {
        if isPerson {
            self.myParameter.boardingDuration
        } else {
            self.myParameter.loadingDuration
        }
    }

    /// C++: `double MSVehicleType::getBoardingFactor() const` [const]
    /// Get this person type's factor for loading/boarding duration
    // double getBoardingFactor() const {
    //         return myParameter.boardingFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_boarding_factor(&self) -> f64 {
        self.myParameter.boardingFactor
    }

    /// C++: `double MSVehicleType::getMaxSpeedLat() const` [const]
    /// Get vehicle's maximum lateral speed [m/s].
    // double getMaxSpeedLat() const {
    //         return myParameter.maxSpeedLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed_lat(&self) -> f64 {
        self.myParameter.maxSpeedLat
    }

    /// C++: `const LatAlignmentDefinition & MSVehicleType::getPreferredLateralAlignment() const` [const]
    /// Get vehicle's preferred lateral alignment procedure
    // const LatAlignmentDefinition& getPreferredLateralAlignment() const {
    //         return myParameter.latAlignmentProcedure;
    //     }
    #[allow(unused_variables)]
    pub fn get_preferred_lateral_alignment(&self) -> &LatAlignmentDefinition {
        &self.myParameter.latAlignmentProcedure
    }

    /// C++: `double MSVehicleType::getPreferredLateralAlignmentOffset() const` [const]
    /// Get vehicle's preferred lateral alignment offset (in m from center line)
    // double getPreferredLateralAlignmentOffset() const {
    //         return myParameter.latAlignmentOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_preferred_lateral_alignment_offset(&self) -> f64 {
        self.myParameter.latAlignmentOffset
    }

    /// C++: `double MSVehicleType::getFrontSeatPos() const` [const]
    /// Get offset of first seat from vehicle front
    // double getFrontSeatPos() const {
    //         return myParameter.frontSeatPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_front_seat_pos(&self) -> f64 {
        self.myParameter.frontSeatPos
    }

    /// C++: `void MSVehicleType::setAccel(double)`
    /// Set a new value for this type's acceleration.
    // void
    // MSVehicleType::setAccel(double accel) {
    //     if (myOriginalType != nullptr && accel < 0) {
    //         accel = myOriginalType->getCarFollowModel().getMaxAccel();
    //     }
    //     myCarFollowModel->setMaxAccel(accel);
    //     myParameter.cfParameter[SUMO_ATTR_ACCEL] = toString(accel);
    // }
    #[allow(unused_variables)]
    pub fn set_accel(&mut self, accel: f64) {
        let mut accel = accel;
        if !self.myOriginalType.is_null() && accel < 0. {
            accel = self.myOriginalType.get_car_follow_model().get_max_accel();
        }
        self.myCarFollowModel.m().set_max_accel(accel);
        self.myParameter.cfParameter.insert(SUMO_ATTR_ACCEL, f64_to_string(accel));
    }

    /// C++: `void MSVehicleType::setDecel(double)`
    /// Set a new value for this type's deceleration.
    // void
    // MSVehicleType::setDecel(double decel) {
    //     if (myOriginalType != nullptr && decel < 0) {
    //         decel = myOriginalType->getCarFollowModel().getMaxDecel();
    //     }
    //     myCarFollowModel->setMaxDecel(decel);
    //     myParameter.cfParameter[SUMO_ATTR_DECEL] = toString(decel);
    // }
    #[allow(unused_variables)]
    pub fn set_decel(&mut self, decel: f64) {
        let mut decel = decel;
        if !self.myOriginalType.is_null() && decel < 0. {
            decel = self.myOriginalType.get_car_follow_model().get_max_decel();
        }
        self.myCarFollowModel.m().set_max_decel(decel);
        self.myParameter.cfParameter.insert(SUMO_ATTR_DECEL, f64_to_string(decel));
    }

    /// C++: `void MSVehicleType::setEmergencyDecel(double)`
    /// Set a new value for this type's emergency deceleration.
    // void
    // MSVehicleType::setEmergencyDecel(double emergencyDecel) {
    //     if (myOriginalType != nullptr && emergencyDecel < 0) {
    //         emergencyDecel = myOriginalType->getCarFollowModel().getEmergencyDecel();
    //     }
    //     myCarFollowModel->setEmergencyDecel(emergencyDecel);
    //     myParameter.cfParameter[SUMO_ATTR_EMERGENCYDECEL] = toString(emergencyDecel);
    // }
    #[allow(unused_variables)]
    pub fn set_emergency_decel(&mut self, emergencyDecel: f64) {
        let mut emergencyDecel = emergencyDecel;
        if !self.myOriginalType.is_null() && emergencyDecel < 0. {
            emergencyDecel = self.myOriginalType.get_car_follow_model().get_emergency_decel();
        }
        self.myCarFollowModel.m().set_emergency_decel(emergencyDecel);
        self.myParameter.cfParameter.insert(SUMO_ATTR_EMERGENCYDECEL, f64_to_string(emergencyDecel));
    }

    /// C++: `void MSVehicleType::setApparentDecel(double)`
    /// Set a new value for this type's apparent deceleration.
    // void
    // MSVehicleType::setApparentDecel(double apparentDecel) {
    //     if (myOriginalType != nullptr && apparentDecel < 0) {
    //         apparentDecel = myOriginalType->getCarFollowModel().getApparentDecel();
    //     }
    //     myCarFollowModel->setApparentDecel(apparentDecel);
    //     myParameter.cfParameter[SUMO_ATTR_APPARENTDECEL] = toString(apparentDecel);
    // }
    #[allow(unused_variables)]
    pub fn set_apparent_decel(&mut self, apparentDecel: f64) {
        let mut apparentDecel = apparentDecel;
        if !self.myOriginalType.is_null() && apparentDecel < 0. {
            apparentDecel = self.myOriginalType.get_car_follow_model().get_apparent_decel();
        }
        self.myCarFollowModel.m().set_apparent_decel(apparentDecel);
        self.myParameter.cfParameter.insert(SUMO_ATTR_APPARENTDECEL, f64_to_string(apparentDecel));
    }

    /// C++: `void MSVehicleType::setMaxAccelProfile(std::vector<std::pair<double, double>>)`
    /// Set a new value for this type's maximum acceleration profile.
    // void
    // MSVehicleType::setMaxAccelProfile(std::vector<std::pair<double, double> > /* accelProfile */) {
    //     /*
    //     if (myOriginalType != nullptr) {
    //         accelProfile = myOriginalType->getCarFollowModel().getMaxAccelProfile();
    //     } else {
    //         if (accelProfile[0].first > 0.) {
    //             accelProfile.insert(accelProfile.begin(), std::make_pair(0.0, accelProfile[0].second));
    //         }
    //         if (accelProfile.back().first < (10000 / 3.6)) {
    //             accelProfile.push_back(std::make_pair((10000 / 3.6), accelProfile.back().second));
    //         }
    //         double prevSpeed = 0.0;
    //         for (const auto& accelPair : accelProfile) {
    //             if (accelPair.first < 0.) {
    //                 accelProfile = myOriginalType->getCarFollowModel().getMaxAccelProfile();
    //                 break;
    //             } else if (accelPair.second < 0.) {
    //                 accelProfile = myOriginalType->getCarFollowModel().getMaxAccelProfile();
    //                 break;
    //             } else if (accelPair.first < prevSpeed) {
    //                 accelProfile = myOriginalType->getCarFollowModel().getMaxAccelProfile();
    //                 break;
    //             }
    //             prevSpeed = accelPair.first;
    //         }
    //     }
    //     myCarFollowModel->setMaxAccelProfile(accelProfile);
    //
    //     std::stringstream accelProfileString;
    //     accelProfileString << std::fixed << std::setprecision(2);
    //     int count = 0;
    //     for (const auto& accelPair : accelProfile) {
    //         if (count > 0) {
    //             accelProfileString << " ";
    //         }
    //         accelProfileString << toString(accelPair.first) + "," << accelPair.second;
    //         count++;
    //     }
    //     myParameter.cfParameter[SUMO_ATTR_MAXACCEL_PROFILE] = accelProfileString.str();
    //     */
    // }
    #[allow(unused_variables)]
    pub fn set_max_accel_profile(&mut self, accelProfile: Vec<(f64, f64)>) {
        // The C++ body of this method is entirely inside a comment block, so it is a no-op in SUMO
        let _ = accelProfile;
    }

    /// C++: `void MSVehicleType::setDesAccelProfile(std::vector<std::pair<double, double>>)`
    /// Set a new value for this type's desired acceleration profile.
    // void
    // MSVehicleType::setDesAccelProfile(std::vector<std::pair<double, double> > /* accelProfile */) {
    //     /*
    //     if (myOriginalType != nullptr) {
    //         accelProfile = myOriginalType->getCarFollowModel().getDesAccelProfile();
    //     } else {
    //         if (accelProfile[0].first > 0.) {
    //             accelProfile.insert(accelProfile.begin(), std::make_pair(0.0, accelProfile[0].second));
    //         }
    //         if (accelProfile.back().first < (10000 / 3.6)) {
    //             accelProfile.push_back(std::make_pair((10000 / 3.6), accelProfile.back().second));
    //         }
    //         double prevSpeed = 0.0;
    //         for (const auto& accelPair : accelProfile) {
    //             if (accelPair.first < 0.) {
    //                 accelProfile = myOriginalType->getCarFollowModel().getDesAccelProfile();
    //                 break;
    //             } else if (accelPair.second < 0.) {
    //                 accelProfile = myOriginalType->getCarFollowModel().getDesAccelProfile();
    //                 break;
    //             } else if (accelPair.first < prevSpeed) {
    //                 accelProfile = myOriginalType->getCarFollowModel().getDesAccelProfile();
    //                 break;
    //             }
    //             prevSpeed = accelPair.first;
    //         }
    //     }
    //     myCarFollowModel->setDesAccelProfile(accelProfile);
    //
    //     std::stringstream accelProfileString;
    //     accelProfileString << std::fixed << std::setprecision(2);
    //     int count = 0;
    //     for (const auto& accelPair : accelProfile) {
    //         if (count > 0) {
    //             accelProfileString << " ";
    //         }
    //         accelProfileString << toString(accelPair.first) + "," << accelPair.second;
    //         count++;
    //     }
    //     myParameter.cfParameter[SUMO_ATTR_DESACCEL_PROFILE] = accelProfileString.str();
    //     */
    // }
    #[allow(unused_variables)]
    pub fn set_des_accel_profile(&mut self, accelProfile: Vec<(f64, f64)>) {
        // The C++ body of this method is entirely inside a comment block, so it is a no-op in SUMO
        let _ = accelProfile;
    }

    /// C++: `void MSVehicleType::setImperfection(double)`
    /// Set a new value for this type's imperfection.
    // void
    // MSVehicleType::setImperfection(double imperfection) {
    //     if (myOriginalType != nullptr && imperfection < 0) {
    //         imperfection = myOriginalType->getCarFollowModel().getImperfection();
    //     }
    //     myCarFollowModel->setImperfection(imperfection);
    //     myParameter.cfParameter[SUMO_ATTR_SIGMA] = toString(imperfection);
    // }
    #[allow(unused_variables)]
    pub fn set_imperfection(&mut self, imperfection: f64) {
        let mut imperfection = imperfection;
        if !self.myOriginalType.is_null() && imperfection < 0. {
            imperfection = self.myOriginalType.get_car_follow_model().get_imperfection();
        }
        self.myCarFollowModel.m().set_imperfection(imperfection);
        self.myParameter.cfParameter.insert(SUMO_ATTR_SIGMA, f64_to_string(imperfection));
    }

    /// C++: `void MSVehicleType::setTau(double)`
    /// Set a new value for this type's headway.
    // void
    // MSVehicleType::setTau(double tau) {
    //     if (myOriginalType != nullptr && tau < 0) {
    //         tau = myOriginalType->getCarFollowModel().getHeadwayTime();
    //     }
    //     myCarFollowModel->setHeadwayTime(tau);
    //     myParameter.cfParameter[SUMO_ATTR_TAU] = toString(tau);
    // }
    #[allow(unused_variables)]
    pub fn set_tau(&mut self, tau: f64) {
        let mut tau = tau;
        if !self.myOriginalType.is_null() && tau < 0. {
            tau = self.myOriginalType.get_car_follow_model().get_headway_time();
        }
        self.myCarFollowModel.m().set_headway_time(tau);
        self.myParameter.cfParameter.insert(SUMO_ATTR_TAU, f64_to_string(tau));
    }

    /// C++: `void MSVehicleType::setLength(const double &)`
    /// Set a new value for this type's length
    // void
    // MSVehicleType::setLength(const double& length) {
    //     if (myOriginalType != nullptr && length < 0) {
    //         myParameter.length = myOriginalType->getLength();
    //     } else {
    //         myParameter.length = length;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_LENGTH_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_length(&mut self, length: &f64) {
        if !self.myOriginalType.is_null() && *length < 0. {
            self.myParameter.length = self.myOriginalType.get_length();
        } else {
            self.myParameter.length = *length;
        }
        self.myParameter.parametersSet |= VTYPEPARS_LENGTH_SET;
    }

    /// C++: `void MSVehicleType::setHeight(const double &)`
    /// Set a new value for this type's height
    // void
    // MSVehicleType::setHeight(const double& height) {
    //     if (myOriginalType != nullptr && height < 0) {
    //         myParameter.height = myOriginalType->getHeight();
    //     } else {
    //         myParameter.height = height;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_HEIGHT_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_height(&mut self, height: &f64) {
        if !self.myOriginalType.is_null() && *height < 0. {
            self.myParameter.height = self.myOriginalType.get_height();
        } else {
            self.myParameter.height = *height;
        }
        self.myParameter.parametersSet |= VTYPEPARS_HEIGHT_SET;
    }

    /// C++: `void MSVehicleType::setMinGap(const double &)`
    /// Set a new value for this type's minimum gap
    // void
    // MSVehicleType::setMinGap(const double& minGap) {
    //     if (myOriginalType != nullptr && minGap < 0) {
    //         myParameter.minGap = myOriginalType->getMinGap();
    //     } else {
    //         myParameter.minGap = minGap;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_MINGAP_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap(&mut self, minGap: &f64) {
        if !self.myOriginalType.is_null() && *minGap < 0. {
            self.myParameter.minGap = self.myOriginalType.get_min_gap();
        } else {
            self.myParameter.minGap = *minGap;
        }
        self.myParameter.parametersSet |= VTYPEPARS_MINGAP_SET;
    }

    /// C++: `void MSVehicleType::setMinGapLat(const double &)`
    /// Set a new value for this type's minimum lataral gap
    // void
    // MSVehicleType::setMinGapLat(const double& minGapLat) {
    //     if (myOriginalType != nullptr && minGapLat < 0) {
    //         myParameter.minGapLat = myOriginalType->getMinGapLat();
    //     } else {
    //         myParameter.minGapLat = minGapLat;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_MINGAP_LAT_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap_lat(&mut self, minGapLat: &f64) {
        if !self.myOriginalType.is_null() && *minGapLat < 0. {
            self.myParameter.minGapLat = self.myOriginalType.get_min_gap_lat();
        } else {
            self.myParameter.minGapLat = *minGapLat;
        }
        self.myParameter.parametersSet |= VTYPEPARS_MINGAP_LAT_SET;
    }

    /// C++: `void MSVehicleType::setMaxSpeed(const double &)`
    /// Set a new value for this type's maximum speed
    // void
    // MSVehicleType::setMaxSpeed(const double& maxSpeed) {
    //     if (myOriginalType != nullptr && maxSpeed < 0) {
    //         myParameter.maxSpeed = myOriginalType->getMaxSpeed();
    //     } else {
    //         myParameter.maxSpeed = maxSpeed;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_MAXSPEED_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(&mut self, maxSpeed: &f64) {
        if !self.myOriginalType.is_null() && *maxSpeed < 0. {
            self.myParameter.maxSpeed = self.myOriginalType.get_max_speed();
        } else {
            self.myParameter.maxSpeed = *maxSpeed;
        }
        self.myParameter.parametersSet |= VTYPEPARS_MAXSPEED_SET;
    }

    /// C++: `void MSVehicleType::setMaxSpeedLat(const double &)`
    /// Set a new value for this type's maximum lateral speed
    // void
    // MSVehicleType::setMaxSpeedLat(const double& maxSpeedLat) {
    //     if (myOriginalType != nullptr && maxSpeedLat < 0) {
    //         myParameter.maxSpeedLat = myOriginalType->getMaxSpeedLat();
    //     } else {
    //         myParameter.maxSpeedLat = maxSpeedLat;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_MAXSPEED_LAT_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed_lat(&mut self, maxSpeedLat: &f64) {
        if !self.myOriginalType.is_null() && *maxSpeedLat < 0. {
            self.myParameter.maxSpeedLat = self.myOriginalType.get_max_speed_lat();
        } else {
            self.myParameter.maxSpeedLat = *maxSpeedLat;
        }
        self.myParameter.parametersSet |= VTYPEPARS_MAXSPEED_LAT_SET;
    }

    /// C++: `void MSVehicleType::setVClass(SUMOVehicleClass)`
    /// Set a new value for this type's vehicle class
    // void
    // MSVehicleType::setVClass(SUMOVehicleClass vclass) {
    //     myParameter.vehicleClass = vclass;
    //     myParameter.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_v_class(&mut self, vclass: SUMOVehicleClass) {
        self.myParameter.vehicleClass = vclass;
        self.myParameter.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    }

    /// C++: `void MSVehicleType::setGUIShape(SUMOVehicleShape)`
    /// Set a new value for this type's gui shape
    // void
    // MSVehicleType::setGUIShape(SUMOVehicleShape shape) {
    //     myParameter.shape = shape;
    //     myParameter.parametersSet |= VTYPEPARS_SHAPE_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_gui_shape(&mut self, shape: SUMOVehicleShape) {
        self.myParameter.shape = shape.clone();
        self.myParameter.parametersSet |= VTYPEPARS_SHAPE_SET;
    }

    /// C++: `void MSVehicleType::setDefaultProbability(const double &)`
    /// Set a new value for this type's default probability
    // void
    // MSVehicleType::setDefaultProbability(const double& prob) {
    //     if (myOriginalType != nullptr && prob < 0) {
    //         myParameter.defaultProbability = myOriginalType->getDefaultProbability();
    //     } else {
    //         myParameter.defaultProbability = prob;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_PROBABILITY_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_default_probability(&mut self, prob: &f64) {
        if !self.myOriginalType.is_null() && *prob < 0. {
            self.myParameter.defaultProbability = self.myOriginalType.get_default_probability();
        } else {
            self.myParameter.defaultProbability = *prob;
        }
        self.myParameter.parametersSet |= VTYPEPARS_PROBABILITY_SET;
    }

    /// C++: `void MSVehicleType::setSpeedFactor(const double &)`
    /// Set a new value for this type's speed factor
    // void
    // MSVehicleType::setSpeedFactor(const double& factor) {
    //     if (myOriginalType != nullptr && factor < 0) {
    //         myParameter.speedFactor.setParameter(0, myOriginalType->myParameter.speedFactor.getParameter(0));
    //     } else {
    //         myParameter.speedFactor.setParameter(0, factor);
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_speed_factor(&mut self, factor: &f64) {
        // shortcut: SUMOVTypeParameter::speedFactor is Unmapped, so Distribution_Parameterized parameter 0 (and the original's value) cannot be set, upgrade when that field is mapped
        self.myParameter.parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    }

    /// C++: `void MSVehicleType::setSpeedDeviation(const double &)`
    /// Set a new value for this type's speed deviation
    // void
    // MSVehicleType::setSpeedDeviation(const double& dev) {
    //     if (myOriginalType != nullptr && dev < 0) {
    //         myParameter.speedFactor.setParameter(1, myOriginalType->myParameter.speedFactor.getParameter(1));
    //     } else {
    //         myParameter.speedFactor.setParameter(1, dev);
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_speed_deviation(&mut self, dev: &f64) {
        // shortcut: SUMOVTypeParameter::speedFactor is Unmapped, so Distribution_Parameterized parameter 1 (and the original's value) cannot be set, upgrade when that field is mapped
        self.myParameter.parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    }

    /// C++: `void MSVehicleType::setActionStepLength(const SUMOTime, bool)`
    /// Set a new value for this type's action step length
    // void
    // MSVehicleType::setActionStepLength(const SUMOTime actionStepLength, bool resetActionOffset) {
    //     assert(actionStepLength >= 0);
    //     myParameter.parametersSet |= VTYPEPARS_ACTIONSTEPLENGTH_SET;
    //
    //     if (myParameter.actionStepLength == actionStepLength) {
    //         return;
    //     }
    //
    //     SUMOTime previousActionStepLength = myParameter.actionStepLength;
    //     myParameter.actionStepLength = actionStepLength;
    //     myCachedActionStepLengthSecs = STEPS2TIME(myParameter.actionStepLength);
    //     check();
    //
    //     if (isVehicleSpecific()) {
    //         // don't perform vehicle lookup for singular vtype
    //         return;
    //     }
    //
    //     // For non-singular vType reset all vehicle's actionOffsets
    //     // Iterate through vehicles
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     for (auto vehIt = vc.loadedVehBegin(); vehIt != vc.loadedVehEnd(); ++vehIt) {
    //         MSVehicle* veh = static_cast<MSVehicle*>(vehIt->second);
    //         if (&veh->getVehicleType() == this) {
    //             // Found vehicle of this type. Perform requested actionOffsetReset
    //             if (resetActionOffset) {
    //                 veh->resetActionOffset();
    //             } else {
    //                 veh->updateActionOffset(previousActionStepLength, actionStepLength);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_action_step_length(&mut self, actionStepLength: SUMOTime, resetActionOffset: bool) {
        debug_assert!(actionStepLength >= 0);
        self.myParameter.parametersSet |= VTYPEPARS_ACTIONSTEPLENGTH_SET;

        if self.myParameter.actionStepLength == actionStepLength {
            return;
        }

        let previousActionStepLength = self.myParameter.actionStepLength;
        self.myParameter.actionStepLength = actionStepLength;
        self.myCachedActionStepLengthSecs = steps2time(self.myParameter.actionStepLength);
        self.check();

        if self.is_vehicle_specific() {
            // don't perform vehicle lookup for singular vtype
            return;
        }

        // For non-singular vType reset all vehicle's actionOffsets
        // Iterate through vehicles
        // determinism: C++ iterates the loaded-vehicle std::map in key order, as does this BTreeMap
        let self_ptr: *const MSVehicleType = self;
        let vc = MSNet::get_instance().m().get_vehicle_control();
        for vehPtr in vc.myVehicleDict.values() {
            let mut veh: Ptr<MSVehicle> = vehPtr.cast::<MSVehicle>();
            if std::ptr::eq(veh.get_vehicle_type(), self_ptr) {
                // Found vehicle of this type. Perform requested actionOffsetReset
                if resetActionOffset {
                    veh.m().reset_action_offset(0);
                } else {
                    veh.m().update_action_offset(previousActionStepLength, actionStepLength);
                }
            }
        }
    }

    /// C++: `void MSVehicleType::setEmissionClass(SUMOEmissionClass)`
    /// Set a new value for this type's emission class
    // void
    // MSVehicleType::setEmissionClass(SUMOEmissionClass eclass) {
    //     myParameter.emissionClass = eclass;
    //     myParameter.parametersSet |= VTYPEPARS_EMISSIONCLASS_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_emission_class(&mut self, eclass: SUMOEmissionClass) {
        self.myParameter.emissionClass = eclass;
        self.myParameter.parametersSet |= VTYPEPARS_EMISSIONCLASS_SET;
    }

    /// C++: `void MSVehicleType::setMass(double)`
    /// Set a new value for this type's mass
    // void
    // MSVehicleType::setMass(double mass) {
    //     myParameter.mass = mass;
    //     myParameter.parametersSet |= VTYPEPARS_MASS_SET;
    //     const_cast<EnergyParams&>(myEnergyParams).setMass(mass);
    // }
    #[allow(unused_variables)]
    pub fn set_mass(&mut self, mass: f64) {
        self.myParameter.mass = mass;
        self.myParameter.parametersSet |= VTYPEPARS_MASS_SET;
        self.myEnergyParams.set_mass(mass);
    }

    /// C++: `void MSVehicleType::setColor(const RGBColor &)`
    /// Set a new value for this type's color
    // void
    // MSVehicleType::setColor(const RGBColor& color) {
    //     myParameter.color = color;
    //     myParameter.parametersSet |= VTYPEPARS_COLOR_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_color(&mut self, color: &RGBColor) {
        self.myParameter.color = color.clone();
        self.myParameter.parametersSet |= VTYPEPARS_COLOR_SET;
    }

    /// C++: `void MSVehicleType::setParkingBadges(const std::vector<std::string> &)`
    /// Set a new value for parking access rights of this type
    // void
    // MSVehicleType::setParkingBadges(const std::vector<std::string>& badges) {
    //     myParameter.parkingBadges.assign(badges.begin(), badges.end());
    //     myParameter.parametersSet |= VTYPEPARS_PARKING_BADGES_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_parking_badges(&mut self, badges: &Vec<String>) {
        self.myParameter.parkingBadges = badges.clone();
        self.myParameter.parametersSet |= VTYPEPARS_PARKING_BADGES_SET;
    }

    /// C++: `void MSVehicleType::setWidth(const double &)`
    /// Set a new value for this type's width
    // void
    // MSVehicleType::setWidth(const double& width) {
    //     if (myOriginalType != nullptr && width < 0) {
    //         myParameter.width = myOriginalType->getWidth();
    //     } else {
    //         myParameter.width = width;
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_WIDTH_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_width(&mut self, width: &f64) {
        if !self.myOriginalType.is_null() && *width < 0. {
            self.myParameter.width = self.myOriginalType.get_width();
        } else {
            self.myParameter.width = *width;
        }
        self.myParameter.parametersSet |= VTYPEPARS_WIDTH_SET;
    }

    /// C++: `void MSVehicleType::setShape(SUMOVehicleShape)`
    /// Set a new value for this type's shape
    // void
    // MSVehicleType::setShape(SUMOVehicleShape shape) {
    //     myParameter.shape = shape;
    //     myParameter.parametersSet |= VTYPEPARS_SHAPE_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_shape(&mut self, shape: SUMOVehicleShape) {
        self.myParameter.shape = shape.clone();
        self.myParameter.parametersSet |= VTYPEPARS_SHAPE_SET;
    }

    /// C++: `void MSVehicleType::setBoardingDuration(SUMOTime, bool)`
    /// Set a new value for this type's boardingDuration
    // void
    // MSVehicleType::setBoardingDuration(SUMOTime duration, bool isPerson) {
    //     if (myOriginalType != nullptr && duration < 0) {
    //         myParameter.boardingDuration = myOriginalType->getBoardingDuration(isPerson);
    //     } else {
    //         if (isPerson) {
    //             myParameter.boardingDuration = duration;
    //         } else {
    //             myParameter.loadingDuration = duration;
    //         }
    //     }
    //     myParameter.parametersSet |= VTYPEPARS_BOARDING_DURATION;
    // }
    #[allow(unused_variables)]
    pub fn set_boarding_duration(&mut self, duration: SUMOTime, isPerson: bool) {
        if !self.myOriginalType.is_null() && duration < 0 {
            self.myParameter.boardingDuration = self.myOriginalType.get_boarding_duration(isPerson);
        } else if isPerson {
            self.myParameter.boardingDuration = duration;
        } else {
            self.myParameter.loadingDuration = duration;
        }
        self.myParameter.parametersSet |= VTYPEPARS_BOARDING_DURATION;
    }

    /// C++: `void MSVehicleType::setImpatience(const double)`
    /// Set a new value for this type's impatience
    // void
    // MSVehicleType::setImpatience(const double impatience) {
    //     myParameter.impatience = impatience;
    //     myParameter.parametersSet |= VTYPEPARS_IMPATIENCE_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_impatience(&mut self, impatience: f64) {
        self.myParameter.impatience = impatience;
        self.myParameter.parametersSet |= VTYPEPARS_IMPATIENCE_SET;
    }

    /// C++: `void MSVehicleType::setPreferredLateralAlignment(const LatAlignmentDefinition &, double)`
    /// Set vehicle's preferred lateral alignment
    // void
    // MSVehicleType::setPreferredLateralAlignment(const LatAlignmentDefinition& latAlignment, double latAlignmentOffset) {
    //     myParameter.latAlignmentProcedure = latAlignment;
    //     myParameter.latAlignmentOffset = latAlignmentOffset;
    //     myParameter.parametersSet |= VTYPEPARS_LATALIGNMENT_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_preferred_lateral_alignment(&mut self, latAlignment: &LatAlignmentDefinition, latAlignmentOffset: f64) {
        self.myParameter.latAlignmentProcedure = *latAlignment;
        self.myParameter.latAlignmentOffset = latAlignmentOffset;
        self.myParameter.parametersSet |= VTYPEPARS_LATALIGNMENT_SET;
    }

    /// C++: `void MSVehicleType::setScale(double)`
    /// Set traffic scaling factor
    // void
    // MSVehicleType::setScale(double value) {
    //     myParameter.scale = value;
    //     MSInsertionControl& insertControl = MSNet::getInstance()->getInsertionControl();
    //     insertControl.updateScale(getID());
    // }
    #[allow(unused_variables)]
    pub fn set_scale(&mut self, value: f64) {
        self.myParameter.scale = value;
        let vtypeid = self.get_id().to_string();
        MSNet::get_instance().m().get_insertion_control().update_scale(vtypeid);
    }

    /// C++: `void MSVehicleType::setLcContRight(const std::string &)`
    /// Set lcContRight (which is the only lc-attribute not used within the laneChange model)
    // void
    // MSVehicleType::setLcContRight(const std::string& value) {
    //     myParameter.lcParameter[SUMO_ATTR_LCA_CONTRIGHT] = value;
    // }
    #[allow(unused_variables)]
    pub fn set_lc_cont_right(&mut self, value: &str) {
        self.myParameter.lcParameter.insert(SUMO_ATTR_LCA_CONTRIGHT, value.to_string());
    }

    /// C++: `MSVehicleType * MSVehicleType::build(SUMOVTypeParameter &, const std::string &)` [static]
    /// Builds the microsim vehicle type described by the given parameter
    // MSVehicleType*
    // MSVehicleType::build(SUMOVTypeParameter& from, const std::string& fileName) {
    //     if (from.hasParameter("vehicleMass")) {
    //         if (from.wasSet(VTYPEPARS_MASS_SET)) {
    //             WRITE_WARNINGF(TL("The vType '%' has a 'mass' attribute and a 'vehicleMass' parameter. The 'mass' attribute will take precedence."), from.id);
    //         } else {
    //             WRITE_WARNINGF(TL("The vType '%' has a 'vehicleMass' parameter, which is deprecated. Please use the 'mass' attribute (for the empty mass) and the 'loading' parameter, if needed."), from.id);
    //             from.mass = from.getDouble("vehicleMass", from.mass);
    //             from.parametersSet |= VTYPEPARS_MASS_SET;
    //         }
    //     }
    //     // the unique_ptr ensures the type is deleted if an exception occurs
    //     std::unique_ptr<MSVehicleType> vtypeOwner(new MSVehicleType(from));
    //     MSVehicleType* vtype = vtypeOwner.get();
    //     const double decel = from.getCFParam(SUMO_ATTR_DECEL, SUMOVTypeParameter::getDefaultDecel(from.vehicleClass));
    //     const double emergencyDecel = from.getCFParam(SUMO_ATTR_EMERGENCYDECEL, SUMOVTypeParameter::getDefaultEmergencyDecel(from.vehicleClass, decel, MSGlobals::gDefaultEmergencyDecel));
    //     // by default decel and apparentDecel are identical
    //     const double apparentDecel = from.getCFParam(SUMO_ATTR_APPARENTDECEL, decel);
    //
    //     if (emergencyDecel < decel) {
    //         WRITE_WARNINGF(TL("Value of 'emergencyDecel' (%) should be higher than 'decel' (%) for vType '%'."), toString(emergencyDecel), toString(decel), from.id);
    //     }
    //     if (emergencyDecel < apparentDecel) {
    //         WRITE_WARNINGF(TL("Value of 'emergencyDecel' (%) is lower than 'apparentDecel' (%) for vType '%' may cause collisions."), toString(emergencyDecel), toString(apparentDecel), from.id);
    //     }
    //
    //     switch (from.cfModel) {
    //         case SUMO_TAG_CF_IDM:
    //             vtype->myCarFollowModel = new MSCFModel_IDM(vtype, false);
    //             break;
    //         case SUMO_TAG_CF_IDMM:
    //             vtype->myCarFollowModel = new MSCFModel_IDM(vtype, true);
    //             break;
    //         case SUMO_TAG_CF_BKERNER:
    //             vtype->myCarFollowModel = new MSCFModel_Kerner(vtype);
    //             break;
    //         case SUMO_TAG_CF_KRAUSS_ORIG1:
    //             vtype->myCarFollowModel = new MSCFModel_KraussOrig1(vtype);
    //             break;
    //         case SUMO_TAG_CF_KRAUSS_PLUS_SLOPE:
    //             vtype->myCarFollowModel = new MSCFModel_KraussPS(vtype);
    //             break;
    //         case SUMO_TAG_CF_KRAUSSX:
    //             vtype->myCarFollowModel = new MSCFModel_KraussX(vtype);
    //             break;
    //         case SUMO_TAG_CF_EIDM:
    //             vtype->myCarFollowModel = new MSCFModel_EIDM(vtype);
    //             break;
    //         case SUMO_TAG_CF_SMART_SK:
    //             vtype->myCarFollowModel = new MSCFModel_SmartSK(vtype);
    //             break;
    //         case SUMO_TAG_CF_DANIEL1:
    //             vtype->myCarFollowModel = new MSCFModel_Daniel1(vtype);
    //             break;
    //         case SUMO_TAG_CF_PWAGNER2009:
    //             vtype->myCarFollowModel = new MSCFModel_PWag2009(vtype);
    //             break;
    //         case SUMO_TAG_CF_WIEDEMANN:
    //             vtype->myCarFollowModel = new MSCFModel_Wiedemann(vtype);
    //             break;
    //         case SUMO_TAG_CF_W99:
    //             vtype->myCarFollowModel = new MSCFModel_W99(vtype);
    //             break;
    //         case SUMO_TAG_CF_RAIL:
    //             vtype->myCarFollowModel = new MSCFModel_Rail(vtype);
    //             break;
    //         case SUMO_TAG_CF_ACC:
    //             vtype->myCarFollowModel = new MSCFModel_ACC(vtype);
    //             break;
    //         case SUMO_TAG_CF_CACC:
    //             vtype->myCarFollowModel = new MSCFModel_CACC(vtype);
    //             break;
    //         case SUMO_TAG_CF_CC:
    //             vtype->myCarFollowModel = new MSCFModel_CC(vtype);
    //             break;
    //         case SUMO_TAG_CF_NASCH:
    //             vtype->myCarFollowModel = new MSCFModel_NaSch(vtype,
    //                     from.getCFParam(SUMO_ATTR_SIGMA, SUMOVTypeParameter::getDefaultImperfection(from.vehicleClass)));
    //             break;
    //         case SUMO_TAG_CF_KRAUSS:
    //         default:
    //             vtype->myCarFollowModel = new MSCFModel_Krauss(vtype);
    //             break;
    //     }
    //     // init Rail visualization parameters
    //     vtype->myParameter.initRailVisualizationParameters(fileName);
    //     return vtypeOwner.release();
    // }
    #[allow(unused_variables)]
    pub fn build(from: &mut SUMOVTypeParameter, fileName: &str) -> Ptr<MSVehicleType> {
        if from.has_parameter("vehicleMass") {
            if (from.parametersSet & VTYPEPARS_MASS_SET) != 0 {
                utils::write_warningf!("The vType '%' has a 'mass' attribute and a 'vehicleMass' parameter. The 'mass' attribute will take precedence.", from.id);
            } else {
                utils::write_warningf!("The vType '%' has a 'vehicleMass' parameter, which is deprecated. Please use the 'mass' attribute (for the empty mass) and the 'loading' parameter, if needed.", from.id);
                from.mass = from.get_double("vehicleMass", from.mass);
                from.parametersSet |= VTYPEPARS_MASS_SET;
            }
        }
        // the unique_ptr ensures the type is deleted if an exception occurs
        let vtype: Ptr<MSVehicleType> = Ptr::from_mut(Box::leak(Box::new(MSVehicleType::new(from))));
        let decel = from.get_cf_param(SUMO_ATTR_DECEL, SUMOVTypeParameter::get_default_decel(from.vehicleClass));
        let emergencyDecel = from.get_cf_param(SUMO_ATTR_EMERGENCYDECEL, SUMOVTypeParameter::get_default_emergency_decel(from.vehicleClass, decel, g_default_emergency_decel()));
        // by default decel and apparentDecel are identical
        let apparentDecel = from.get_cf_param(SUMO_ATTR_APPARENTDECEL, decel);

        if emergencyDecel < decel {
            utils::write_warningf!("Value of 'emergencyDecel' (%) should be higher than 'decel' (%) for vType '%'.", f64_to_string(emergencyDecel), f64_to_string(decel), from.id);
        }
        if emergencyDecel < apparentDecel {
            utils::write_warningf!("Value of 'emergencyDecel' (%) is lower than 'apparentDecel' (%) for vType '%' may cause collisions.", f64_to_string(emergencyDecel), f64_to_string(apparentDecel), from.id);
        }

        let cf: Ptr<MSCFModel> = match from.cfModel {
            SUMO_TAG_CF_IDM => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_IDM::new(vtype, false))).base),
            SUMO_TAG_CF_IDMM => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_IDM::new(vtype, true))).base),
            SUMO_TAG_CF_BKERNER => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_Kerner::new(vtype))).base),
            SUMO_TAG_CF_KRAUSS_ORIG1 => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_KraussOrig1::new(vtype))).base),
            SUMO_TAG_CF_KRAUSS_PLUS_SLOPE => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_KraussPS::new(vtype))).base),
            SUMO_TAG_CF_KRAUSSX => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_KraussX::new(vtype))).base),
            SUMO_TAG_CF_EIDM => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_EIDM::new(vtype))).base),
            SUMO_TAG_CF_SMART_SK => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_SmartSK::new(vtype))).base),
            SUMO_TAG_CF_DANIEL1 => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_Daniel1::new(vtype))).base),
            SUMO_TAG_CF_PWAGNER2009 => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_PWag2009::new(vtype))).base),
            SUMO_TAG_CF_WIEDEMANN => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_Wiedemann::new(vtype))).base),
            SUMO_TAG_CF_W99 => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_W99::new(vtype))).base),
            SUMO_TAG_CF_RAIL => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_Rail::new(vtype))).base),
            SUMO_TAG_CF_ACC => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_ACC::new(vtype))).base),
            SUMO_TAG_CF_CACC => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_CACC::new(vtype))).base),
            SUMO_TAG_CF_CC => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_CC::new(vtype))).base),
            SUMO_TAG_CF_NASCH => {
                let sigma = from.get_cf_param(SUMO_ATTR_SIGMA, SUMOVTypeParameter::get_default_imperfection(from.vehicleClass));
                Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_NaSch::new(vtype, sigma))).base)
            }
            // SUMO_TAG_CF_KRAUSS and default
            _ => Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_Krauss::new(vtype))).base),
        };
        vtype.m().myCarFollowModel = cf;
        // init Rail visualization parameters
        vtype.m().myParameter.init_rail_visualization_parameters(fileName.to_string());
        vtype
    }

    /// C++: `SUMOTime MSVehicleType::getEntryManoeuvreTime(const int) const` [const]
    /// Accessor function for parameter equivalent returning entry time for a specific manoeuver angle
    // SUMOTime
    // MSVehicleType::getEntryManoeuvreTime(const int angle) const {
    //     return (getParameter().getEntryManoeuvreTime(angle));
    // }
    #[allow(unused_variables)]
    pub fn get_entry_manoeuvre_time(&self, angle: i32) -> SUMOTime {
        self.myParameter.get_entry_manoeuvre_time(angle)
    }

    /// C++: `SUMOTime MSVehicleType::getExitManoeuvreTime(const int) const` [const]
    /// Accessor function for parameter equivalent returning exit time for a specific manoeuver angle
    // SUMOTime
    // MSVehicleType::getExitManoeuvreTime(const int angle) const {
    //     return (getParameter().getExitManoeuvreTime(angle));
    // }
    #[allow(unused_variables)]
    pub fn get_exit_manoeuvre_time(&self, angle: i32) -> SUMOTime {
        self.myParameter.get_exit_manoeuvre_time(angle)
    }

    /// C++: `MSVehicleType * MSVehicleType::buildSingularType(const std::string &) const` [const]
    /// Duplicates the microsim vehicle type giving the newly created type the given id, marking it as vehicle specific
    // MSVehicleType*
    // MSVehicleType::buildSingularType(const std::string& id) const {
    //     return duplicateType(id, false);
    // }
    #[allow(unused_variables)]
    pub fn build_singular_type(&self, id: &str) -> Ptr<MSVehicleType> {
        self.duplicate_type(id, false)
    }

    /// C++: `MSVehicleType * MSVehicleType::duplicateType(const std::string &, bool) const` [const]
    /// Duplicates the microsim vehicle type giving the newly created type the given id.
    // MSVehicleType*
    // MSVehicleType::duplicateType(const std::string& id, bool persistent) const {
    //     MSVehicleType* vtype = new MSVehicleType(myParameter);
    //     vtype->myParameter.id = id;
    //     vtype->myCarFollowModel = myCarFollowModel->duplicate(vtype);
    //     if (!persistent) {
    //         vtype->myOriginalType = this;
    //     }
    //     if (!MSNet::getInstance()->getVehicleControl().addVType(vtype)) {
    //         std::string singular = persistent ? "" : TL("singular ");
    //         throw ProcessError(TLF("could not add %type %", singular, vtype->getID()));
    //     }
    //     return vtype;
    // }
    #[allow(unused_variables)]
    pub fn duplicate_type(&self, id: &str, persistent: bool) -> Ptr<MSVehicleType> {
        let mut vtype: Ptr<MSVehicleType> = Ptr::from_mut(Box::leak(Box::new(MSVehicleType::new(&self.myParameter))));
        vtype.m().myParameter.id = id.to_string();
        let cf = self.myCarFollowModel.duplicate(vtype);
        vtype.m().myCarFollowModel = cf;
        if !persistent {
            vtype.m().myOriginalType = Ptr::new(self);
        }
        if !MSNet::get_instance().m().get_vehicle_control().add_v_type(vtype) {
            let singular = if persistent { "".to_string() } else { "singular ".to_string() };
            panic!("{}", ProcessError::new(tlf!("could not add %type %", singular, vtype.get_id())));
        }
        vtype
    }

    /// C++: `bool MSVehicleType::isVehicleSpecific() const` [const]
    /// Returns whether this type belongs to a single vehicle only (was modified)
    // bool isVehicleSpecific() const {
    //         return myOriginalType != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn is_vehicle_specific(&self) -> bool {
        !self.myOriginalType.is_null()
    }

    /// C++: `const std::string & MSVehicleType::getOriginalID() const` [const]
    /// Returns the id of the original vehicle type if this is a vehicle specific type, the id otherwise
    // const std::string& getOriginalID() const {
    //         return myOriginalType != nullptr ? myOriginalType->getID() : getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_original_id(&self) -> &str {
        if !self.myOriginalType.is_null() {
            self.myOriginalType.get_id()
        } else {
            self.get_id()
        }
    }

    /// C++: `const SUMOVTypeParameter & MSVehicleType::getParameter() const` [const]
    // const SUMOVTypeParameter& getParameter() const {
    //         return myParameter;
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVTypeParameter {
        &self.myParameter
    }

    /// C++: `void MSVehicleType::check()`
    /// Checks whether vehicle type parameters may be problematic (Currently, only the value for the action step length is compared with the value for the desired headway time.)
    // void
    // MSVehicleType::check() {
    //     if (!myWarnedActionStepLengthTauOnce
    //             && myParameter.actionStepLength != DELTA_T
    //             && STEPS2TIME(myParameter.actionStepLength) > getCarFollowModel().getHeadwayTime()) {
    //         myWarnedActionStepLengthTauOnce = true;
    //         std::stringstream s;
    //         s << "Given action step length " << STEPS2TIME(myParameter.actionStepLength) << " for vehicle type '" << getID()
    //           << "' is larger than its parameter tau (=" << getCarFollowModel().getHeadwayTime() << ")!"
    //           << " This may lead to collisions. (This warning is only issued once per vehicle type).";
    //         WRITE_WARNING(s.str());
    //     }
    //     if (!myWarnedActionStepLengthBallisticOnce
    //             && myParameter.actionStepLength != DELTA_T
    //             && MSGlobals::gSemiImplicitEulerUpdate) {
    //         myWarnedActionStepLengthBallisticOnce = true;
    //         std::string warning2;
    //         if (OptionsCont::getOptions().isDefault("step-method.ballistic")) {
    //             warning2 = " Setting it now to avoid collisions.";
    //             MSGlobals::gSemiImplicitEulerUpdate = false;
    //         } else {
    //             warning2 = " This may cause collisions.";
    //         }
    //         WRITE_WARNINGF("Action step length '%' is used for vehicle type '%' but step-method.ballistic was not set." + warning2
    //                        , STEPS2TIME(myParameter.actionStepLength), getID())
    //     }
    //     if (!myWarnedStepLengthTauOnce && TS > getCarFollowModel().getHeadwayTime()
    //             && !MSGlobals::gUseMesoSim) {
    //         myWarnedStepLengthTauOnce = true;
    //         WRITE_WARNINGF(TL("Value of tau=% in vehicle type '%' lower than simulation step size may cause collisions."),
    //                        getCarFollowModel().getHeadwayTime(), getID());
    //     }
    //     if (MSGlobals::gUseMesoSim && getVehicleClass() != SVC_PEDESTRIAN && !OptionsCont::getOptions().getBool("meso-lane-queue")) {
    //         SVCPermissions ignoreVClasses = parseVehicleClasses(OptionsCont::getOptions().getStringVector("meso-ignore-lanes-by-vclass"));
    //         if ((ignoreVClasses & getVehicleClass()) != 0) {
    //             WRITE_WARNINGF(TL("Vehicle class '%' of vType '%' is set as ignored by option --meso-ignore-lanes-by-vclass to ensure default vehicle capacity. Set option --meso-lane-queue for multi-modal meso simulation"),
    //                            toString(getVehicleClass()), getID());
    //         }
    //     }
    //     if (!myParameter.wasSet(VTYPEPARS_EMISSIONCLASS_SET) && !OptionsCont::getOptions().getBool("device.battery.track-fuel")
    //             && (OptionsCont::getOptions().getFloat("device.battery.probability") == 1.
    //                 || myParameter.getDouble("device.battery.probability", -1.) == 1.
    //                 || StringUtils::toBool(myParameter.getParameter("has.battery.device", "false")))) {
    //         myParameter.emissionClass = PollutantsInterface::getClassByName("Energy");
    //         myParameter.parametersSet |= VTYPEPARS_EMISSIONCLASS_SET;
    //         WRITE_MESSAGEF(TL("The battery device is active for vType '%' but no emission class is set. The emission class Energy/unknown will be used, please consider setting an explicit emission class!"),
    //                        getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check(&mut self) {
        if !self.myWarnedActionStepLengthTauOnce
            && self.myParameter.actionStepLength != delta_t()
            && steps2time(self.myParameter.actionStepLength) > self.get_car_follow_model().get_headway_time()
        {
            self.myWarnedActionStepLengthTauOnce = true;
            let s = format!("Given action step length {} for vehicle type '{}' is larger than its parameter tau (={})! This may lead to collisions. (This warning is only issued once per vehicle type).",
                f64_to_string_default(steps2time(self.myParameter.actionStepLength)),
                self.get_id(),
                f64_to_string_default(self.get_car_follow_model().get_headway_time()));
            write_warning(&s);
        }
        if !self.myWarnedActionStepLengthBallisticOnce
            && self.myParameter.actionStepLength != delta_t()
            && crate::microsim::cfmodels::mscf_model::g_semi_implicit_euler_update()
        {
            self.myWarnedActionStepLengthBallisticOnce = true;
            let warning2;
            if is_default("step-method.ballistic") {
                warning2 = " Setting it now to avoid collisions.";
                // shortcut: MSGlobals::gSemiImplicitEulerUpdate cannot be written (MSGlobals not ported), so the flag is not switched off here
            } else {
                warning2 = " This may cause collisions.";
            }
            utils::write_warningf!(&format!("Action step length '%' is used for vehicle type '%' but step-method.ballistic was not set.{}", warning2),
                steps2time(self.myParameter.actionStepLength), self.get_id());
        }
        if !self.myWarnedStepLengthTauOnce && ts() > self.get_car_follow_model().get_headway_time()
            && !g_use_meso_sim()
        {
            self.myWarnedStepLengthTauOnce = true;
            utils::write_warningf!("Value of tau=% in vehicle type '%' lower than simulation step size may cause collisions.",
                self.get_car_follow_model().get_headway_time(), self.get_id());
        }
        if g_use_meso_sim() && self.get_vehicle_class() != SVC_PEDESTRIAN && !get_bool("meso-lane-queue") {
            let ignoreVClasses = parse_vehicle_classes_vec(&get_string_vector("meso-ignore-lanes-by-vclass")).unwrap_or_else(|e| panic!("{}", e));
            if (ignoreVClasses & self.get_vehicle_class()) != 0 {
                utils::write_warningf!("Vehicle class '%' of vType '%' is set as ignored by option --meso-ignore-lanes-by-vclass to ensure default vehicle capacity. Set option --meso-lane-queue for multi-modal meso simulation",
                    get_vehicle_class_names(self.get_vehicle_class(), false), self.get_id());
            }
        }
        if (self.myParameter.parametersSet & VTYPEPARS_EMISSIONCLASS_SET) == 0 && !get_bool("device.battery.track-fuel")
            && (get_float("device.battery.probability") == 1.
                || self.myParameter.get_double("device.battery.probability", -1.) == 1.
                || to_bool(&self.myParameter.get_parameter("has.battery.device", "false")).unwrap_or_else(|e| panic!("{}", e)))
        {
            self.myParameter.emissionClass = PollutantsInterface::get_class_by_name("Energy", SVC_IGNORING);
            self.myParameter.parametersSet |= VTYPEPARS_EMISSIONCLASS_SET;
            utils::write_messagef!("The battery device is active for vType '%' but no emission class is set. The emission class Energy/unknown will be used, please consider setting an explicit emission class!",
                self.get_id());
        }
    }

    /// C++: `const EnergyParams * MSVehicleType::getEmissionParameters() const` [const]
    /// retrieve parameters for the energy consumption model
    // inline const EnergyParams* getEmissionParameters() const {
    //         return &myEnergyParams;
    //     }
    #[allow(unused_variables)]
    pub fn get_emission_parameters(&self) -> Ptr<EnergyParams> {
        Ptr::new(&self.myEnergyParams)
    }

    /// C++: `void MSVehicleType::MSVehicleType(const MSVehicleType &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSVehicleType) -> Self {
        panic!("MSVehicleType copy constructor is private and never defined in C++")
    }

    /// C++: `MSVehicleType & MSVehicleType::operator=(const MSVehicleType &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVehicleType) -> &mut MSVehicleType {
        panic!("MSVehicleType assignment operator is private and never defined in C++")
    }

}

