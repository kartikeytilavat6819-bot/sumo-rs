//! Port of microsim/MSVehicleType.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSVehicleType::MSVehicleType")
    }

    /// C++: `void MSVehicleType::~MSVehicleType()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSVehicleType::~MSVehicleType")
    }

    /// C++: `bool MSVehicleType::wasSet(long long) const` [const]
    /// Returns whether the given parameter was set
    // bool wasSet(long long int what) const {
    //         return (myParameter.parametersSet & what) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_set(&self, what: i64) -> bool {
        todo!("port MSVehicleType::wasSet")
    }

    /// C++: `const std::string & MSVehicleType::getID() const` [const]
    /// Returns the name of the vehicle type
    // const std::string& getID() const {
    //         return myParameter.id;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        todo!("port MSVehicleType::getID")
    }

    /// C++: `int MSVehicleType::getNumericalID() const` [const]
    /// Returns the running index of the vehicle type
    // int getNumericalID() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        todo!("port MSVehicleType::getNumericalID")
    }

    /// C++: `double MSVehicleType::getLength() const` [const]
    /// Get vehicle's length [m]
    // double getLength() const {
    //         return myParameter.length;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port MSVehicleType::getLength")
    }

    /// C++: `double MSVehicleType::getLengthWithGap() const` [const]
    /// Get vehicle's length including the minimum gap [m]
    // double getLengthWithGap() const {
    //         return myParameter.length + myParameter.minGap;
    //     }
    #[allow(unused_variables)]
    pub fn get_length_with_gap(&self) -> f64 {
        todo!("port MSVehicleType::getLengthWithGap")
    }

    /// C++: `double MSVehicleType::getMinGap() const` [const]
    /// Get the free space in front of vehicles of this class
    // double getMinGap() const {
    //         return myParameter.minGap;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_gap(&self) -> f64 {
        todo!("port MSVehicleType::getMinGap")
    }

    /// C++: `double MSVehicleType::getMinGapLat() const` [const]
    /// Get the minimum lateral gap that vehicles of this type maintain
    // double getMinGapLat() const {
    //         return myParameter.minGapLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_gap_lat(&self) -> f64 {
        todo!("port MSVehicleType::getMinGapLat")
    }

    /// C++: `const MSCFModel & MSVehicleType::getCarFollowModel() const` [const]
    /// Returns the vehicle type's car following model definition (const version)
    // inline const MSCFModel& getCarFollowModel() const {
    //         return *myCarFollowModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_follow_model(&self) -> &MSCFModel {
        todo!("port MSVehicleType::getCarFollowModel")
    }

    /// C++: `MSCFModel & MSVehicleType::getCarFollowModel()`
    /// Returns the vehicle type's car following model definition (non-const version)
    // inline MSCFModel& getCarFollowModel() {
    //         return *myCarFollowModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_follow_model_2(&mut self) -> &mut MSCFModel {
        todo!("port MSVehicleType::getCarFollowModel")
    }

    /// C++: `LaneChangeModel MSVehicleType::getLaneChangeModel() const` [const]
    // inline LaneChangeModel getLaneChangeModel() const {
    //         return myParameter.lcModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_change_model(&self) -> LaneChangeModel {
        todo!("port MSVehicleType::getLaneChangeModel")
    }

    /// C++: `double MSVehicleType::getMaxSpeed() const` [const]
    /// Get vehicle's (technical) maximum speed [m/s].
    // double getMaxSpeed() const {
    //         return myParameter.maxSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        todo!("port MSVehicleType::getMaxSpeed")
    }

    /// C++: `double MSVehicleType::getDesiredMaxSpeed() const` [const]
    /// Returns the vehicles's desired maximum speed
    // double getDesiredMaxSpeed() const {
    //         return myParameter.desiredMaxSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_desired_max_speed(&self) -> f64 {
        todo!("port MSVehicleType::getDesiredMaxSpeed")
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
        todo!("port MSVehicleType::computeChosenSpeedDeviation")
    }

    /// C++: `double MSVehicleType::getDefaultProbability() const` [const]
    /// Get the default probability of this vehicle type
    // double getDefaultProbability() const {
    //         return myParameter.defaultProbability;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_probability(&self) -> f64 {
        todo!("port MSVehicleType::getDefaultProbability")
    }

    /// C++: `SUMOVehicleClass MSVehicleType::getVehicleClass() const` [const]
    /// Get this vehicle type's vehicle class
    // SUMOVehicleClass getVehicleClass() const {
    //         return myParameter.vehicleClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_class(&self) -> SUMOVehicleClass {
        todo!("port MSVehicleType::getVehicleClass")
    }

    /// C++: `SUMOEmissionClass MSVehicleType::getEmissionClass() const` [const]
    /// Get this vehicle type's emission class
    // SUMOEmissionClass getEmissionClass() const {
    //         return myParameter.emissionClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_emission_class(&self) -> SUMOEmissionClass {
        todo!("port MSVehicleType::getEmissionClass")
    }

    /// C++: `double MSVehicleType::getMass() const` [const]
    /// Get this vehicle type's mass
    // inline double getMass() const {
    //         return myEnergyParams.getDouble(SUMO_ATTR_MASS);
    //     }
    #[allow(unused_variables)]
    pub fn get_mass(&self) -> f64 {
        todo!("port MSVehicleType::getMass")
    }

    /// C++: `const RGBColor & MSVehicleType::getColor() const` [const]
    /// Returns this type's color
    // const RGBColor& getColor() const {
    //         return myParameter.color;
    //     }
    #[allow(unused_variables)]
    pub fn get_color(&self) -> &RGBColor {
        todo!("port MSVehicleType::getColor")
    }

    /// C++: `const std::vector<std::string> & MSVehicleType::getParkingBadges() const` [const]
    /// Returns the parking access rights of this type
    // const std::vector<std::string>& getParkingBadges() const {
    //         return myParameter.parkingBadges;
    //     }
    #[allow(unused_variables)]
    pub fn get_parking_badges(&self) -> &Vec<String> {
        todo!("port MSVehicleType::getParkingBadges")
    }

    /// C++: `const Distribution_Parameterized & MSVehicleType::getSpeedFactor() const` [const]
    /// Returns this type's speed factor
    // const Distribution_Parameterized& getSpeedFactor() const {
    //         return myParameter.speedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_factor(&self) -> &Unmapped /* Distribution_Parameterized */ {
        todo!("port MSVehicleType::getSpeedFactor")
    }

    /// C++: `SUMOTime MSVehicleType::getActionStepLength() const` [const]
    /// Returns this type's default action step length
    // SUMOTime getActionStepLength() const {
    //         return myParameter.actionStepLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_action_step_length(&self) -> SUMOTime {
        todo!("port MSVehicleType::getActionStepLength")
    }

    /// C++: `double MSVehicleType::getActionStepLengthSecs() const` [const]
    /// Returns this type's default action step length in seconds
    // double getActionStepLengthSecs() const {
    //         return myCachedActionStepLengthSecs;
    //     }
    #[allow(unused_variables)]
    pub fn get_action_step_length_secs(&self) -> f64 {
        todo!("port MSVehicleType::getActionStepLengthSecs")
    }

    /// C++: `double MSVehicleType::getImpatience() const` [const]
    /// Returns this type's impatience
    // double getImpatience() const {
    //         return myParameter.impatience;
    //     }
    #[allow(unused_variables)]
    pub fn get_impatience(&self) -> f64 {
        todo!("port MSVehicleType::getImpatience")
    }

    /// C++: `double MSVehicleType::getWidth() const` [const]
    /// Get the width which vehicles of this class shall have when being drawn
    // double getWidth() const {
    //         return myParameter.width;
    //     }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        todo!("port MSVehicleType::getWidth")
    }

    /// C++: `double MSVehicleType::getSeatingWidth() const` [const]
    /// Get the width of the passenger compartment when being drawn
    // double getSeatingWidth() const {
    //         return myParameter.seatingWidth >= 0 ? myParameter.seatingWidth : myParameter.width;
    //     }
    #[allow(unused_variables)]
    pub fn get_seating_width(&self) -> f64 {
        todo!("port MSVehicleType::getSeatingWidth")
    }

    /// C++: `double MSVehicleType::getHeight() const` [const]
    /// Get the height which vehicles of this class shall have when being drawn
    // double getHeight() const {
    //         return myParameter.height;
    //     }
    #[allow(unused_variables)]
    pub fn get_height(&self) -> f64 {
        todo!("port MSVehicleType::getHeight")
    }

    /// C++: `SUMOVehicleShape MSVehicleType::getGuiShape() const` [const]
    /// Get this vehicle type's shape
    // SUMOVehicleShape getGuiShape() const {
    //         return myParameter.shape;
    //     }
    #[allow(unused_variables)]
    pub fn get_gui_shape(&self) -> SUMOVehicleShape {
        todo!("port MSVehicleType::getGuiShape")
    }

    /// C++: `std::string MSVehicleType::getOSGFile() const` [const]
    /// Get this vehicle type's 3D model file name
    // std::string getOSGFile() const {
    //         return myParameter.osgFile;
    //     }
    #[allow(unused_variables)]
    pub fn get_osg_file(&self) -> String {
        todo!("port MSVehicleType::getOSGFile")
    }

    /// C++: `std::string MSVehicleType::getImgFile() const` [const]
    /// Get this vehicle type's raster model file name
    // std::string getImgFile() const {
    //         return myParameter.imgFile;
    //     }
    #[allow(unused_variables)]
    pub fn get_img_file(&self) -> String {
        todo!("port MSVehicleType::getImgFile")
    }

    /// C++: `int MSVehicleType::getPersonCapacity() const` [const]
    /// Get this vehicle type's person capacity
    // int getPersonCapacity() const {
    //         return myParameter.personCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_person_capacity(&self) -> i32 {
        todo!("port MSVehicleType::getPersonCapacity")
    }

    /// C++: `int MSVehicleType::getContainerCapacity() const` [const]
    /// Get this vehicle type's container capacity
    // int getContainerCapacity() const {
    //         return myParameter.containerCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_container_capacity(&self) -> i32 {
        todo!("port MSVehicleType::getContainerCapacity")
    }

    /// C++: `SUMOTime MSVehicleType::getLoadingDuration(const bool) const` [const]
    /// Get this vehicle type's loading duration
    // SUMOTime getLoadingDuration(const bool isPerson) const {
    //         return isPerson ? myParameter.boardingDuration : myParameter.loadingDuration;
    //     }
    #[allow(unused_variables)]
    pub fn get_loading_duration(&self, isPerson: bool) -> SUMOTime {
        todo!("port MSVehicleType::getLoadingDuration")
    }

    /// C++: `SUMOTime MSVehicleType::getBoardingDuration(const bool) const` [const]
    /// Get this vehicle type's boarding duration
    // SUMOTime getBoardingDuration(const bool isPerson) const {
    //         return isPerson ? myParameter.boardingDuration : myParameter.loadingDuration;
    //     }
    #[allow(unused_variables)]
    pub fn get_boarding_duration(&self, isPerson: bool) -> SUMOTime {
        todo!("port MSVehicleType::getBoardingDuration")
    }

    /// C++: `double MSVehicleType::getBoardingFactor() const` [const]
    /// Get this person type's factor for loading/boarding duration
    // double getBoardingFactor() const {
    //         return myParameter.boardingFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_boarding_factor(&self) -> f64 {
        todo!("port MSVehicleType::getBoardingFactor")
    }

    /// C++: `double MSVehicleType::getMaxSpeedLat() const` [const]
    /// Get vehicle's maximum lateral speed [m/s].
    // double getMaxSpeedLat() const {
    //         return myParameter.maxSpeedLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed_lat(&self) -> f64 {
        todo!("port MSVehicleType::getMaxSpeedLat")
    }

    /// C++: `const LatAlignmentDefinition & MSVehicleType::getPreferredLateralAlignment() const` [const]
    /// Get vehicle's preferred lateral alignment procedure
    // const LatAlignmentDefinition& getPreferredLateralAlignment() const {
    //         return myParameter.latAlignmentProcedure;
    //     }
    #[allow(unused_variables)]
    pub fn get_preferred_lateral_alignment(&self) -> &LatAlignmentDefinition {
        todo!("port MSVehicleType::getPreferredLateralAlignment")
    }

    /// C++: `double MSVehicleType::getPreferredLateralAlignmentOffset() const` [const]
    /// Get vehicle's preferred lateral alignment offset (in m from center line)
    // double getPreferredLateralAlignmentOffset() const {
    //         return myParameter.latAlignmentOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_preferred_lateral_alignment_offset(&self) -> f64 {
        todo!("port MSVehicleType::getPreferredLateralAlignmentOffset")
    }

    /// C++: `double MSVehicleType::getFrontSeatPos() const` [const]
    /// Get offset of first seat from vehicle front
    // double getFrontSeatPos() const {
    //         return myParameter.frontSeatPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_front_seat_pos(&self) -> f64 {
        todo!("port MSVehicleType::getFrontSeatPos")
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
        todo!("port MSVehicleType::setAccel")
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
        todo!("port MSVehicleType::setDecel")
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
        todo!("port MSVehicleType::setEmergencyDecel")
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
        todo!("port MSVehicleType::setApparentDecel")
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
        todo!("port MSVehicleType::setMaxAccelProfile")
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
        todo!("port MSVehicleType::setDesAccelProfile")
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
        todo!("port MSVehicleType::setImperfection")
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
        todo!("port MSVehicleType::setTau")
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
        todo!("port MSVehicleType::setLength")
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
        todo!("port MSVehicleType::setHeight")
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
        todo!("port MSVehicleType::setMinGap")
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
        todo!("port MSVehicleType::setMinGapLat")
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
        todo!("port MSVehicleType::setMaxSpeed")
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
        todo!("port MSVehicleType::setMaxSpeedLat")
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
        todo!("port MSVehicleType::setVClass")
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
        todo!("port MSVehicleType::setGUIShape")
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
        todo!("port MSVehicleType::setDefaultProbability")
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
        todo!("port MSVehicleType::setSpeedFactor")
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
        todo!("port MSVehicleType::setSpeedDeviation")
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
        todo!("port MSVehicleType::setActionStepLength")
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
        todo!("port MSVehicleType::setEmissionClass")
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
        todo!("port MSVehicleType::setMass")
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
        todo!("port MSVehicleType::setColor")
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
        todo!("port MSVehicleType::setParkingBadges")
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
        todo!("port MSVehicleType::setWidth")
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
        todo!("port MSVehicleType::setShape")
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
        todo!("port MSVehicleType::setBoardingDuration")
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
        todo!("port MSVehicleType::setImpatience")
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
        todo!("port MSVehicleType::setPreferredLateralAlignment")
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
        todo!("port MSVehicleType::setScale")
    }

    /// C++: `void MSVehicleType::setLcContRight(const std::string &)`
    /// Set lcContRight (which is the only lc-attribute not used within the laneChange model)
    // void
    // MSVehicleType::setLcContRight(const std::string& value) {
    //     myParameter.lcParameter[SUMO_ATTR_LCA_CONTRIGHT] = value;
    // }
    #[allow(unused_variables)]
    pub fn set_lc_cont_right(&mut self, value: &str) {
        todo!("port MSVehicleType::setLcContRight")
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
        todo!("port MSVehicleType::build")
    }

    /// C++: `SUMOTime MSVehicleType::getEntryManoeuvreTime(const int) const` [const]
    /// Accessor function for parameter equivalent returning entry time for a specific manoeuver angle
    // SUMOTime
    // MSVehicleType::getEntryManoeuvreTime(const int angle) const {
    //     return (getParameter().getEntryManoeuvreTime(angle));
    // }
    #[allow(unused_variables)]
    pub fn get_entry_manoeuvre_time(&self, angle: i32) -> SUMOTime {
        todo!("port MSVehicleType::getEntryManoeuvreTime")
    }

    /// C++: `SUMOTime MSVehicleType::getExitManoeuvreTime(const int) const` [const]
    /// Accessor function for parameter equivalent returning exit time for a specific manoeuver angle
    // SUMOTime
    // MSVehicleType::getExitManoeuvreTime(const int angle) const {
    //     return (getParameter().getExitManoeuvreTime(angle));
    // }
    #[allow(unused_variables)]
    pub fn get_exit_manoeuvre_time(&self, angle: i32) -> SUMOTime {
        todo!("port MSVehicleType::getExitManoeuvreTime")
    }

    /// C++: `MSVehicleType * MSVehicleType::buildSingularType(const std::string &) const` [const]
    /// Duplicates the microsim vehicle type giving the newly created type the given id, marking it as vehicle specific
    // MSVehicleType*
    // MSVehicleType::buildSingularType(const std::string& id) const {
    //     return duplicateType(id, false);
    // }
    #[allow(unused_variables)]
    pub fn build_singular_type(&self, id: &str) -> Ptr<MSVehicleType> {
        todo!("port MSVehicleType::buildSingularType")
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
        todo!("port MSVehicleType::duplicateType")
    }

    /// C++: `bool MSVehicleType::isVehicleSpecific() const` [const]
    /// Returns whether this type belongs to a single vehicle only (was modified)
    // bool isVehicleSpecific() const {
    //         return myOriginalType != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn is_vehicle_specific(&self) -> bool {
        todo!("port MSVehicleType::isVehicleSpecific")
    }

    /// C++: `const std::string & MSVehicleType::getOriginalID() const` [const]
    /// Returns the id of the original vehicle type if this is a vehicle specific type, the id otherwise
    // const std::string& getOriginalID() const {
    //         return myOriginalType != nullptr ? myOriginalType->getID() : getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_original_id(&self) -> &str {
        todo!("port MSVehicleType::getOriginalID")
    }

    /// C++: `const SUMOVTypeParameter & MSVehicleType::getParameter() const` [const]
    // const SUMOVTypeParameter& getParameter() const {
    //         return myParameter;
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVTypeParameter {
        todo!("port MSVehicleType::getParameter")
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
        todo!("port MSVehicleType::check")
    }

    /// C++: `const EnergyParams * MSVehicleType::getEmissionParameters() const` [const]
    /// retrieve parameters for the energy consumption model
    // inline const EnergyParams* getEmissionParameters() const {
    //         return &myEnergyParams;
    //     }
    #[allow(unused_variables)]
    pub fn get_emission_parameters(&self) -> Ptr<EnergyParams> {
        todo!("port MSVehicleType::getEmissionParameters")
    }

    /// C++: `void MSVehicleType::MSVehicleType(const MSVehicleType &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSVehicleType) -> Self {
        todo!("port MSVehicleType::MSVehicleType")
    }

    /// C++: `MSVehicleType & MSVehicleType::operator=(const MSVehicleType &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVehicleType) -> &mut MSVehicleType {
        todo!("port MSVehicleType::operator=")
    }

}

