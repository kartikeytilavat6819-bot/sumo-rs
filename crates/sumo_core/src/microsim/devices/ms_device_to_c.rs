//! Port of microsim/devices/MSDevice_ToC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSDevice_ToC::ToCState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_ToC_ToCState {
    UNDEFINED = 0,
    MANUAL = 1,
    AUTOMATED = 2,
    PREPARING_TOC = 3,
    MRM = 4,
    RECOVERING = 5,
}

/// The ToC Device controls transition of control between automated and manual driving.
/// C++ class `MSDevice_ToC` (microsim/devices/MSDevice_ToC.h)
pub struct MSDevice_ToC {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// vehicle type ID for manual driving
    pub myManualTypeID: String,
    /// vehicle type ID for automated driving
    pub myAutomatedTypeID: String,
    /// Average response time needed by the driver to take back control
    pub myResponseTime: SUMOTime,
    /// Recovery rate for the driver's awareness after a ToC
    pub myRecoveryRate: f64,
    /// Level of the awareness below which no lane-changes are performed
    pub myLCAbstinence: f64,
    /// Average awareness the driver has initially after a ToC
    pub myInitialAwareness: f64,
    /// Deceleration rate applied during MRM
    pub myMRMDecel: f64,
    /// Current awareness-level of the driver in [0,1]
    pub myCurrentAwareness: f64,
    /// Coloring scheme,
    pub myColorScheme: i32,
    /// Whether a coloring scheme shall by applied to indicate the different toc stages,
    pub myUseColorScheme: bool,
    /// Current state of the device
    pub myState: MSDevice_ToC_ToCState,
    /// The holder vehicle casted to MSVehicle*
    pub myHolderMS: Ptr<MSVehicle>,
    pub myTriggerMRMCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myTriggerToCCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myRecoverAwarenessCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myExecuteMRMCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myPrepareToCCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    /// The file the devices output goes to
    pub myOutputFile: Ptr<OutputDevice>,
    /// Storage for events to be written to the output
    pub myEvents: i32,
    /// Storage for events to be written to the output
    pub myEventLanes: i32,
    /// Storage for events to be written to the output
    pub myEventXY: i32,
    /// LC mode overridden during MRM, stored for restoration
    pub myPreviousLCMode: i32,
    /// Parameters for the openGap mechanism applied during ToC preparation phase
    pub myOpenGapParams: MSDevice_ToC_OpenGapParams,
    /// Duration in s. for which the vehicle needs to be able to follow its route without a lane change to continue in automated mode (only has effect if dynamic ToCs are activated,
    pub myDynamicToCThreshold: f64,
    /// Probability of an MRM to occur after a dynamically triggered ToC
    pub myMRMProbability: f64,
    /// Switch for considering dynamic ToCs,
    pub myDynamicToCActive: bool,
    /// Flag to indicate that a dynamically triggered ToC is in preparation
    pub myIssuedDynamicToC: bool,
    /// Lane, on which the ongoing dynamic ToC was issued. It can only be aborted if the lane was changed.
    pub myDynamicToCLane: i32,
    /// Whether vehicle tries to change to the right during an MRM
    pub myMRMKeepRight: bool,
    /// stop vehicle tries to reach during MRM
    pub myMRMSafeSpot: String,
    /// duration at stop vehicle tries to reach during MRM
    pub myMRMSafeSpotDuration: SUMOTime,
    /// Maximal acceleration that may be applied during the ToC preparation phase TODO: Make effective
    pub myMaxPreparationAccel: f64,
    /// Storage for original maximal acceleration of vehicle.
    pub myOriginalMaxAccel: f64,
}

impl std::ops::Deref for MSDevice_ToC {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_ToC {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_ToC (port as module-level statics, see AGENTS.md):
//   static int MSDevice_ToC::myInstances;
//   static int MSDevice_ToC::createdOutputFiles;
//   static int MSDevice_ToC::LCModeMRM;
//   static int MSDevice_ToC::lookupResponseTimeMRMProbs;
//   static int MSDevice_ToC::lookupResponseTimeLeadTimes;
//   static int MSDevice_ToC::lookupResponseTimeVariances;
//   static SumoRNG MSDevice_ToC::myResponseTimeRNG;

impl MSDevice_ToC {
    /// C++: `void MSDevice_ToC::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_ToC-options
    // void
    // MSDevice_ToC::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("ToC Device");
    //     insertDefaultAssignmentOptions("toc", "ToC Device", oc);
    //
    //     oc.doRegister("device.toc.manualType", new Option_String());
    //     oc.addDescription("device.toc.manualType", "ToC Device", TL("Vehicle type for manual driving regime."));
    //     oc.doRegister("device.toc.automatedType", new Option_String());
    //     oc.addDescription("device.toc.automatedType", "ToC Device", TL("Vehicle type for automated driving regime."));
    //     oc.doRegister("device.toc.responseTime", new Option_Float(DEFAULT_RESPONSE_TIME));
    //     oc.addDescription("device.toc.responseTime", "ToC Device", TL("Average response time needed by a driver to take back control."));
    //     oc.doRegister("device.toc.recoveryRate", new Option_Float(DEFAULT_RECOVERY_RATE));
    //     oc.addDescription("device.toc.recoveryRate", "ToC Device", TL("Recovery rate for the driver's awareness after a ToC."));
    //     oc.doRegister("device.toc.lcAbstinence", new Option_Float(DEFAULT_LCABSTINENCE));
    //     oc.addDescription("device.toc.lcAbstinence", "ToC Device", TL("Attention level below which a driver restrains from performing lane changes (value in [0,1])."));
    //     oc.doRegister("device.toc.initialAwareness", new Option_Float(DEFAULT_INITIAL_AWARENESS));
    //     oc.addDescription("device.toc.initialAwareness", "ToC Device", TL("Average awareness a driver has initially after a ToC (value in [0,1])."));
    //     oc.doRegister("device.toc.mrmDecel", new Option_Float(DEFAULT_MRM_DECEL));
    //     oc.addDescription("device.toc.mrmDecel", "ToC Device", TL("Deceleration rate applied during a 'minimum risk maneuver'."));
    //     oc.doRegister("device.toc.dynamicToCThreshold", new Option_Float(DEFAULT_DYNAMIC_TOC_THRESHOLD));
    //     oc.addDescription("device.toc.dynamicToCThreshold", "ToC Device", TL("Time, which the vehicle requires to have ahead to continue in automated mode. The default value of 0 indicates no dynamic triggering of ToCs."));
    //     oc.doRegister("device.toc.dynamicMRMProbability", new Option_Float(DEFAULT_MRM_PROBABILITY));
    //     oc.addDescription("device.toc.dynamicMRMProbability", "ToC Device", TL("Probability that a dynamically triggered TOR is not answered in time."));
    //     oc.doRegister("device.toc.mrmKeepRight", new Option_Bool(false));
    //     oc.addDescription("device.toc.mrmKeepRight", "ToC Device", TL("If true, the vehicle tries to change to the right during an MRM."));
    //     oc.doRegister("device.toc.mrmSafeSpot", new Option_String());
    //     oc.addDescription("device.toc.mrmSafeSpot", "ToC Device", TL("If set, the vehicle tries to reach the given named stopping place during an MRM."));
    //     oc.doRegister("device.toc.mrmSafeSpotDuration", new Option_Float(60.));
    //     oc.addDescription("device.toc.mrmSafeSpotDuration", "ToC Device", TL("Duration the vehicle stays at the safe spot after an MRM."));
    //     oc.doRegister("device.toc.maxPreparationAccel", new Option_Float(0.0));
    //     oc.addDescription("device.toc.maxPreparationAccel", "ToC Device", TL("Maximal acceleration that may be applied during the ToC preparation phase."));
    //     oc.doRegister("device.toc.ogNewTimeHeadway", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogNewTimeHeadway", "ToC Device", TL("Timegap for ToC preparation phase."));
    //     oc.doRegister("device.toc.ogNewSpaceHeadway", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogNewSpaceHeadway", "ToC Device", TL("Additional spacing for ToC preparation phase."));
    //     oc.doRegister("device.toc.ogMaxDecel", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogMaxDecel", "ToC Device", TL("Maximal deceleration applied for establishing increased gap in ToC preparation phase."));
    //     oc.doRegister("device.toc.ogChangeRate", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogChangeRate", "ToC Device", TL("Rate of adaptation towards the increased headway during ToC preparation."));
    //     oc.doRegister("device.toc.useColorScheme", new Option_Bool(true));
    //     oc.addDescription("device.toc.useColorScheme", "ToC Device", TL("Whether a coloring scheme shall by applied to indicate the different ToC stages."));
    //     oc.doRegister("device.toc.file", new Option_String());
    //     oc.addDescription("device.toc.file", "ToC Device", TL("Switches on output by specifying an output filename."));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_ToC::insertOptions")
    }

    /// C++: `void MSDevice_ToC::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_ToC::buildVehicleDevices")
    }

    /// C++: `const int & MSDevice_ToC::getInstances()` [static]
    /// returns all currently existing ToC devices
    // static const std::set<MSDevice_ToC*, ComparatorNumericalIdLess>& getInstances() {
    //         return myInstances;
    //     }
    #[allow(unused_variables)]
    pub fn get_instances() -> &'static i32 {
        todo!("port MSDevice_ToC::getInstances")
    }

    /// C++: `void MSDevice_ToC::cleanup()` [static]
    /// Closes root tags of output files
    // void
    // MSDevice_ToC::cleanup() {
    //     // Close xml bodies for all existing files
    //     // TODO: Check if required
    //     for (auto& fn : createdOutputFiles) {
    //         OutputDevice* file = &OutputDevice::getDevice(fn);
    //         file->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSDevice_ToC::cleanup")
    }

    /// C++: `double MSDevice_ToC::getDynamicMRMProbability(const SUMOVehicle &)` [static, private]
    /// @{
    // double
    // MSDevice_ToC::getDynamicMRMProbability(const SUMOVehicle& v) {
    //     double pMRM = v.getFloatParam("device.toc.dynamicMRMProbability");
    //     if (pMRM < 0 || pMRM > 0.5) {
    //         const double pMRMTrunc = MAX2(0.0, MIN2(0.5, pMRM));
    //         WRITE_WARNINGF(TL("Given value for ToC device parameter 'dynamicMRMProbability' (=%) is not in the admissible range [0,0.5]. Truncated to %."), toString(pMRM), toString(pMRMTrunc));
    //         return pMRMTrunc;
    //     }
    //     return pMRM;
    // }
    #[allow(unused_variables)]
    pub fn get_dynamic_mrm_probability(v: &SUMOVehicle) -> f64 {
        todo!("port MSDevice_ToC::getDynamicMRMProbability")
    }

    /// C++: `OpenGapParams MSDevice_ToC::getOpenGapParams(const SUMOVehicle &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_open_gap_params(v: &SUMOVehicle) -> MSDevice_ToC_OpenGapParams {
        todo!("port MSDevice_ToC::getOpenGapParams")
    }

    /// C++: `ToCState MSDevice_ToC::_2ToCState(const std::string &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn n2_to_c_state(arg0: &str) -> MSDevice_ToC_ToCState {
        todo!("port MSDevice_ToC::_2ToCState")
    }

    /// C++: `std::string MSDevice_ToC::_2string(ToCState)` [static, private]
    // std::string
    // MSDevice_ToC::_2string(ToCState state) {
    //     if (state == UNDEFINED) {
    //         return "UNDEFINED";
    //     } else if (state == MANUAL) {
    //         return "MANUAL";
    //     } else if (state == AUTOMATED) {
    //         return "AUTOMATED";
    //     } else if (state == PREPARING_TOC) {
    //         return "PREPARING_TOC";
    //     } else if (state == MRM) {
    //         return "MRM";
    //     } else if (state == RECOVERING) {
    //         return "RECOVERING";
    //     } else {
    //         WRITE_WARNINGF(TL("Unknown ToCState '%'"), toString(state));
    //         return toString(state);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn n2string(state: MSDevice_ToC_ToCState) -> String {
        todo!("port MSDevice_ToC::_2string")
    }

    /// C++: `void MSDevice_ToC::~MSDevice_ToC()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_ToC::~MSDevice_ToC")
    }

    /// C++: `const std::string MSDevice_ToC::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const override {
    //         return "toc";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_ToC::deviceName")
    }

    /// C++: `bool MSDevice_ToC::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Return value indicates whether the device still wants to be notified about the vehicle movement
    // bool
    // MSDevice_ToC::notifyMove(SUMOTrafficObject& /*veh*/,
    //                          double /*oldPos*/,
    //                          double /*newPos*/,
    //                          double /*newSpeed*/) {
    //     if (myState == AUTOMATED && checkDynamicToC()) {
    //         // Initiate a ToC
    //         // Record event
    //         if (generatesOutput()) {
    //             myEvents.push(std::make_pair(SIMSTEP, "DYNTOR"));
    //             myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //             myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //         }
    //         // Leadtime for dynamic ToC is proportional to the time assumed for the dynamic ToC threshold
    //         const double leadTime = myDynamicToCThreshold * 1000 * DYNAMIC_TOC_LEADTIME_FACTOR;
    //         requestToC((SUMOTime) leadTime);
    //         // TODO: Alter the response time according to the given lead time. Consider re-sampling it at each call of requestToC(). (Conditional to whether a non-negative response time was given in the configuration)
    //         myIssuedDynamicToC = true;
    //         myDynamicToCLane = myHolderMS->getLane()->getNumericalID();
    //     } else if (myIssuedDynamicToC && myState == PREPARING_TOC && !checkDynamicToC()) {
    //         // Abort dynamic ToC, FIXME: This could abort an externally requested ToC in rare occasions... (needs test)
    //         // Record event
    //         if (generatesOutput()) {
    //             myEvents.push(std::make_pair(SIMSTEP, "DYNTOR"));
    //             myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //             myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //         }
    //         // NOTE: This should not occur if lane changing is prevented during ToC preparation...
    //         // TODO: Reset response time to the original value (unnecessary if re-sampling for each call to requestToC)
    //         triggerUpwardToC(0);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_ToC::notifyMove")
    }

    /// C++: `std::string MSDevice_ToC::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_ToC::getParameter(const std::string& key) const {
    //     if (key == "manualType") {
    //         return myManualTypeID;
    //     } else if (key == "automatedType") {
    //         return myAutomatedTypeID;
    //     } else if (key == "responseTime") {
    //         return toString(STEPS2TIME(myResponseTime));
    //     } else if (key == "recoveryRate") {
    //         return toString(myRecoveryRate);
    //     } else if (key == "initialAwareness") {
    //         return toString(myInitialAwareness);
    //     } else if (key == "mrmDecel") {
    //         return toString(myMRMDecel);
    //     } else if (key == "currentAwareness") {
    //         return toString(myCurrentAwareness);
    //     } else if (key == "lcAbstinence") {
    //         return toString(myLCAbstinence);
    //     } else if (key == "state") {
    //         return _2string(myState);
    //     } else if (key == "holder") {
    //         return myHolder.getID();
    //     } else if (key == "hasDynamicToC") {
    //         return toString(myDynamicToCActive);
    //     } else if (key == "dynamicToCThreshold") {
    //         return toString(myDynamicToCThreshold);
    //     } else if (key == "dynamicMRMProbability") {
    //         return toString(myMRMProbability);
    //     } else if (key == "mrmKeepRight") {
    //         return toString(myMRMKeepRight);
    //     } else if (key == "mrmSafeSpot") {
    //         return myMRMSafeSpot;
    //     } else if (key == "mrmSafeSpotDuration") {
    //         return toString(STEPS2TIME(myMRMSafeSpotDuration));
    //     } else if (key == "maxPreparationAccel") {
    //         return toString(myMaxPreparationAccel);
    //     } else if (key == "ogNewTimeHeadway") {
    //         return toString(myOpenGapParams.newTimeHeadway);
    //     } else if (key == "ogNewSpaceHeadway") {
    //         return toString(myOpenGapParams.newSpaceHeadway);
    //     } else if (key == "ogChangeRate") {
    //         return toString(myOpenGapParams.changeRate);
    //     } else if (key == "ogMaxDecel") {
    //         return toString(myOpenGapParams.maxDecel);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_ToC::getParameter")
    }

    /// C++: `void MSDevice_ToC::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_ToC::setParameter(const std::string& key, const std::string& value) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " MSDevice_ToC::setParameter(key=" << key << ", value=" << value << ")" << std::endl;
    // #endif
    //     if (key == "manualType") {
    //         myManualTypeID = value;
    //         myColorScheme[MANUAL] = MSNet::getInstance()->getVehicleControl().getVType(myManualTypeID)->getColor();
    //         if (myState == MANUAL) {
    //             switchHolderType(value);
    //         }
    //     } else if (key == "automatedType") {
    //         myAutomatedTypeID = value;
    //         myColorScheme[AUTOMATED] = MSNet::getInstance()->getVehicleControl().getVType(myAutomatedTypeID)->getColor();
    //         if (myState == AUTOMATED || myState == PREPARING_TOC || myState == MRM) {
    //             switchHolderType(value);
    //         }
    //     } else if (key == "responseTime") {
    //         myResponseTime = TIME2STEPS(StringUtils::toDouble(value));
    //     } else if (key == "recoveryRate") {
    //         myRecoveryRate = StringUtils::toDouble(value);
    //     } else if (key == "initialAwareness") {
    //         myInitialAwareness = StringUtils::toDouble(value);
    //     } else if (key == "lcAbstinence") {
    //         myLCAbstinence = StringUtils::toDouble(value);
    //         if (isManuallyDriven()) {
    //             setAwareness(myCurrentAwareness); // to eventually trigger LC-prevention
    //         }
    //     } else if (key == "currentAwareness") {
    //         if (isManuallyDriven()) {
    //             setAwareness(StringUtils::toDouble(value));
    //         } else {
    //             WRITE_WARNING(TL("Setting device.toc.currentAwareness during automated mode has no effect."))
    //         }
    //     } else if (key == "mrmDecel") {
    //         myMRMDecel = StringUtils::toDouble(value);
    //     } else if (key == "requestToC") {
    //         // setting this magic parameter gives the interface for inducing a ToC
    //         const SUMOTime timeTillMRM = TIME2STEPS(StringUtils::toDouble(value));
    //         requestToC(timeTillMRM, myResponseTime);
    //     } else if (key == "requestMRM") {
    //         // setting this magic parameter gives the interface for inducing an MRM
    //         requestMRM();
    //     } else if (key == "awareness") {
    //         // setting this magic parameter gives the interface for setting the driverstate's awareness
    //         setAwareness(StringUtils::toDouble(value));
    //     } else if (key == "dynamicToCThreshold") {
    //         const double newValue = StringUtils::toDouble(value);
    //         if (newValue < 0) {
    //             WRITE_WARNINGF(TL("Value of dynamicToCThreshold must be non-negative. (Given value % for vehicle % is ignored)"), value, myHolderMS->getID());
    //         } else if (newValue == 0) {
    //             myDynamicToCThreshold = newValue;
    //             myDynamicToCActive = false;
    //         } else {
    //             myDynamicToCThreshold = newValue;
    //             myDynamicToCActive = true;
    //         }
    //     } else if (key == "dynamicMRMProbability") {
    //         const double newValue = StringUtils::toDouble(value);
    //         if (newValue < 0) {
    //             WRITE_WARNINGF(TL("Value of dynamicMRMProbability must be non-negative. (Given value % for vehicle % is ignored)"), value, myHolderMS->getID());
    //         } else {
    //             myMRMProbability = newValue;
    //         }
    //     } else if (key == "mrmKeepRight")  {
    //         const bool newValue = StringUtils::toBool(value);
    //         myMRMKeepRight = newValue;
    //     } else if (key == "mrmSafeSpot") {
    //         myMRMSafeSpot = value;
    //     } else if (key == "mrmSafeSpotDuration") {
    //         myMRMSafeSpotDuration = TIME2STEPS(StringUtils::toDouble(value));
    //     } else if (key == "maxPreparationAccel") {
    //         const double newValue = StringUtils::toDouble(value);
    //         if (newValue < 0) {
    //             WRITE_WARNINGF(TL("Value of maxPreparationAccel must be non-negative. (Given value % for vehicle % is ignored)"), value, myHolderMS->getID());
    //         } else {
    //             myMaxPreparationAccel = newValue;
    //         }
    //     } else if (key == "ogNewTimeHeadway") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.newTimeHeadway = newValue;
    //         myOpenGapParams.active = true;
    //     } else if (key == "ogNewSpaceHeadway") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.newSpaceHeadway = newValue;
    //         myOpenGapParams.active = true;
    //     } else if (key == "ogChangeRate") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.changeRate = newValue;
    //         myOpenGapParams.active = true;
    //     } else if (key == "ogMaxDecel") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.maxDecel = newValue;
    //         myOpenGapParams.active = true;
    //     } else {
    //         throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice_ToC::setParameter")
    }

    /// C++: `SUMOTime MSDevice_ToC::triggerMRM(SUMOTime)`
    /// Trigger execution of an MRM
    // SUMOTime
    // MSDevice_ToC::triggerMRM(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " triggerMRM() for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     // Clear ongoing MRM
    //     descheduleMRM();
    //
    //     // Start MRM process
    //     if (myMRMSafeSpot != "") {
    //         SUMOVehicleParameter::Stop stop;
    //         MSStoppingPlace* s = MSNet::getInstance()->getStoppingPlace(myMRMSafeSpot, SUMO_TAG_PARKING_AREA);
    //         if (s == nullptr) {
    //             WRITE_WARNINGF(TL("Ignoring unknown safe spot '%' for vehicle '%'."), myMRMSafeSpot, myHolder.getID());
    //         } else {
    //             stop.parkingarea = myMRMSafeSpot;
    //             stop.parking = ParkingType::OFFROAD;
    //             stop.lane = s->getLane().getID();
    //             stop.endPos = s->getEndLanePosition();
    //             stop.startPos = s->getBeginLanePosition();
    //             stop.duration = myMRMSafeSpotDuration;
    //             myHolderMS->getSingularType().setDecel(myMRMDecel);
    //             std::string error;
    //             if (!myHolder.addStop(stop, error)) {
    //                 WRITE_WARNING("Could not set safe spot '" + myMRMSafeSpot + "' for vehicle '" + myHolder.getID() + "'. " + error);
    //             }
    //         }
    //     } else {
    //         myExecuteMRMCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::MRMExecutionStep);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myExecuteMRMCommand, SIMSTEP + DELTA_T);
    //     }
    //     if (myState == MANUAL || myState == RECOVERING) {
    //         switchHolderType(myAutomatedTypeID);
    //     }
    //     setState(MRM);
    //     setAwareness(1.);
    //
    //     // Record event
    //     if (generatesOutput()) {
    //         myEvents.push(std::make_pair(SIMSTEP, "MRM"));
    //         myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //         myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //     }
    //
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn trigger_mrm(&mut self, t: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_ToC::triggerMRM")
    }

    /// C++: `SUMOTime MSDevice_ToC::triggerUpwardToC(SUMOTime)`
    /// Trigger execution of a ToC X-->AUTOMATED ("upwards")
    // SUMOTime
    // MSDevice_ToC::triggerUpwardToC(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " triggerUpwardToC() for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     descheduleToC();
    //     // Eventually stop ToC preparation process
    //     descheduleToCPreparation();
    //     // Eventually abort MRM
    //     descheduleMRM();
    //     // Eventually abort awareness recovery process
    //     descheduleRecovery();
    //
    //     if (myState == MANUAL || myState == RECOVERING) {
    //         switchHolderType(myAutomatedTypeID);
    //     }
    //     setAwareness(1.);
    //     setState(AUTOMATED);
    //
    //     // Record event
    //     if (generatesOutput()) {
    //         myEvents.push(std::make_pair(SIMSTEP, "ToCup"));
    //         myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //         myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //     }
    //
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn trigger_upward_to_c(&mut self, t: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_ToC::triggerUpwardToC")
    }

    /// C++: `SUMOTime MSDevice_ToC::triggerDownwardToC(SUMOTime)`
    /// Trigger execution of a ToC X-->MANUAL ("downwards")
    // SUMOTime
    // MSDevice_ToC::triggerDownwardToC(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " triggerDownwardToC() for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     descheduleToC();
    //     // Eventually stop ToC preparation process
    //     descheduleToCPreparation();
    //     // Eventually abort MRM
    //     descheduleMRM();
    //
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " Initial awareness after ToC: " << myCurrentAwareness << std::endl;
    // #endif
    //
    //     // Start awareness recovery process
    //     myRecoverAwarenessCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::awarenessRecoveryStep);
    //     MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myRecoverAwarenessCommand, SIMSTEP + DELTA_T);
    //     setState(RECOVERING);
    //
    //     // @todo: Sample initial awareness
    //     double initialAwareness = myInitialAwareness;
    //     setAwareness(initialAwareness);
    //
    //     switchHolderType(myManualTypeID);
    //
    //     // Record event
    //     if (generatesOutput()) {
    //         myEvents.push(std::make_pair(SIMSTEP, "ToCdown"));
    //         myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //         myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn trigger_downward_to_c(&mut self, t: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_ToC::triggerDownwardToC")
    }

    /// C++: `SUMOTime MSDevice_ToC::ToCPreparationStep(SUMOTime)`
    /// Continue the ToC preparation for one time step
    // SUMOTime
    // MSDevice_ToC::ToCPreparationStep(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " ToC preparation step for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     if (myState == PREPARING_TOC) {
    //         return DELTA_T;
    //     } else {
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " Aborting ToC preparation for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //         descheduleToCPreparation();
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn to_c_preparation_step(&mut self, t: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_ToC::ToCPreparationStep")
    }

    /// C++: `SUMOTime MSDevice_ToC::MRMExecutionStep(SUMOTime)`
    /// Continue the MRM for one time step
    // SUMOTime
    // MSDevice_ToC::MRMExecutionStep(SUMOTime t) {
    //     deactivateDeliberateLCs();
    //     const double currentSpeed = myHolderMS->getSpeed();
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " MRM step for vehicle '" << myHolder.getID() << "', currentSpeed=" << currentSpeed << std::endl;
    // #endif
    //
    //     // Induce slowdown with MRMDecel
    //     if (currentSpeed > SPEED_EPS) {
    //         std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //         const double nextSpeed = MAX2(0., currentSpeed - ACCEL2SPEED(myMRMDecel));
    //         speedTimeLine.push_back(std::make_pair(t, currentSpeed));
    //         speedTimeLine.push_back(std::make_pair(t + DELTA_T, nextSpeed));
    //         myHolderMS->getInfluencer().setSpeedTimeLine(speedTimeLine);
    //     }
    //
    //     if (myMRMKeepRight) {
    //         // Try to change to the right
    //         std::vector<std::pair<SUMOTime, int> > laneTimeLine;
    //         laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), 0));
    //         laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + 1, 0));
    //         myHolderMS->getInfluencer().setLaneTimeLine(laneTimeLine);
    // //    	std::cout << "Keeping right..." << std::endl;
    //     }
    //
    //     if (myState == MRM) {
    //         return DELTA_T;
    //     } else {
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " Aborting MRM for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //         resetDeliberateLCs();
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn mrm_execution_step(&mut self, t: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_ToC::MRMExecutionStep")
    }

    /// C++: `SUMOTime MSDevice_ToC::awarenessRecoveryStep(SUMOTime)`
    /// Continue the awareness recovery for one time step
    // SUMOTime
    // MSDevice_ToC::awarenessRecoveryStep(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " Awareness recovery step for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     // Proceed with awareness recovery
    //     if (myCurrentAwareness < 1.0) {
    //         setAwareness(MIN2(1.0, myCurrentAwareness + TS * myRecoveryRate));
    //     }
    //
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " currentAwareness = " << myCurrentAwareness << std::endl;
    // #endif
    //
    //     const bool awarenessRecoveryCompleted = myCurrentAwareness == 1.0;
    //     if (awarenessRecoveryCompleted) {
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " Awareness recovery completed for veh '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //         myRecoverAwarenessCommand->deschedule();
    //         myRecoverAwarenessCommand = nullptr;
    //         setState(MANUAL);
    //         return 0;
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn awareness_recovery_step(&mut self, t: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_ToC::awarenessRecoveryStep")
    }

    /// C++: `void MSDevice_ToC::writeOutput()`
    /// Write output to file given by option device.toc.file
    // void
    // MSDevice_ToC::writeOutput() {
    //     if (!generatesOutput()) {
    //         assert(myEvents.empty());
    //         return;
    //     }
    //     while (!myEvents.empty()) {
    //         const std::pair<SUMOTime, std::string> e = myEvents.front(); // make a copy, it is used after pop
    //         const std::pair<std::string, double>& l = myEventLanes.front();
    //         const std::pair<double, double>& p = myEventXY.front();
    //         myOutputFile->openTag(e.second);
    //         myOutputFile->writeAttr("id", myHolder.getID()).writeAttr("t", STEPS2TIME(e.first));
    //         myOutputFile->writeAttr("lane", l.first).writeAttr("lanePos", l.second);
    //         myOutputFile->writeAttr("x", p.first).writeAttr("y", p.second);
    //         myOutputFile->closeTag();
    //         myEvents.pop();
    //         myEventLanes.pop();
    //         myEventXY.pop();
    //
    //         if (e.second.compare("DYNTOR") == 0 && !myEvents.empty()) { // skip "TOR" events if duplicate of "DYNTOR"
    //             std::pair<SUMOTime, std::string>& eNext = myEvents.front();
    //             if (eNext.second.compare("TOR") == 0 && eNext.first == e.first) {
    //                 myEvents.pop();
    //                 myEventLanes.pop();
    //                 myEventXY.pop();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_output(&mut self) {
        todo!("port MSDevice_ToC::writeOutput")
    }

    /// C++: `bool MSDevice_ToC::generatesOutput()`
    /// Whether this device requested to write output
    // bool generatesOutput() {
    //         return myOutputFile != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn generates_output(&mut self) -> bool {
        todo!("port MSDevice_ToC::generatesOutput")
    }

    /// C++: `SumoRNG * MSDevice_ToC::getResponseTimeRNG()` [static]
    // static SumoRNG* getResponseTimeRNG() {
    //         return &myResponseTimeRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_response_time_rng() -> Ptr<SumoRNG> {
        todo!("port MSDevice_ToC::getResponseTimeRNG")
    }

    /// C++: `void MSDevice_ToC::MSDevice_ToC(SUMOVehicle &, const std::string &, const std::string &, const std::string &, const std::string &, SUMOTime, double, double, double, double, double, double, double, bool, const std::string &, SUMOTime, bool, OpenGapParams)` [private]
    /// Constructor
    // MSDevice_ToC::MSDevice_ToC(SUMOVehicle& holder, const std::string& id, const std::string& outputFilename,
    //                            const std::string& manualType, const std::string& automatedType, SUMOTime responseTime, double recoveryRate,
    //                            double lcAbstinence, double initialAwareness, double mrmDecel,
    //                            double dynamicToCThreshold, double dynamicMRMProbability, double maxPreparationAccel,
    //                            bool mrmKeepRight, const std::string& mrmSafeSpot, SUMOTime mrmSafeSpotDuration, bool useColorScheme, OpenGapParams ogp) :
    //     MSVehicleDevice(holder, id),
    //     myManualTypeID(manualType),
    //     myAutomatedTypeID(automatedType),
    //     myResponseTime(responseTime),
    //     myRecoveryRate(recoveryRate),
    //     myLCAbstinence(lcAbstinence),
    //     myInitialAwareness(initialAwareness),
    //     myMRMDecel(mrmDecel),
    //     myCurrentAwareness(1.),
    //     myUseColorScheme(useColorScheme),
    //     myTriggerMRMCommand(nullptr),
    //     myTriggerToCCommand(nullptr),
    //     myRecoverAwarenessCommand(nullptr),
    //     myExecuteMRMCommand(nullptr),
    //     myPrepareToCCommand(nullptr),
    //     myOutputFile(nullptr),
    //     myEvents(),
    //     myEventLanes(),
    //     myEventXY(),
    //     myPreviousLCMode(-1),
    //     myOpenGapParams(ogp),
    //     myDynamicToCThreshold(dynamicToCThreshold),
    //     myMRMProbability(dynamicMRMProbability),
    //     myDynamicToCActive(dynamicToCThreshold > 0),
    //     myIssuedDynamicToC(false),
    //     myDynamicToCLane(-1),
    //     myMRMKeepRight(mrmKeepRight),
    //     myMRMSafeSpot(mrmSafeSpot),
    //     myMRMSafeSpotDuration(mrmSafeSpotDuration),
    //     myMaxPreparationAccel(maxPreparationAccel),
    //     myOriginalMaxAccel(-1) {
    //     // Take care! Holder is currently being constructed. Cast occurs before completion.
    //     myHolderMS = static_cast<MSVehicle*>(&holder);
    //
    //     if (outputFilename != "") {
    //         myOutputFile = &OutputDevice::getDevice(outputFilename);
    //         // TODO: make xsd, include header
    //         // myOutputFile.writeXMLHeader("ToCDeviceLog", "ToCDeviceLog.xsd");
    //         if (createdOutputFiles.count(outputFilename) == 0) {
    //             myOutputFile->writeXMLHeader("ToCDeviceLog", "");
    //             createdOutputFiles.insert(outputFilename);
    //         }
    //     }
    //
    //     // Check if the given vTypes for the ToC Device are vTypeDistributions
    //     MSVehicleControl& vehCtrl = MSNet::getInstance()->getVehicleControl();
    //     const bool automatedVTypeIsDist = vehCtrl.hasVTypeDistribution(myAutomatedTypeID);
    //     const bool manualVTypeIsDist = vehCtrl.hasVTypeDistribution(myManualTypeID);
    //
    //     // Check if the vType of the holder matches one of the given vTypes
    //     std::string holderVTypeID = holder.getVehicleType().getID();
    //     if (holderVTypeID == myManualTypeID) {
    //         myState = ToCState::MANUAL;
    //     } else if (holderVTypeID == myAutomatedTypeID) {
    //         myState = ToCState::AUTOMATED;
    //     } else if (manualVTypeIsDist && vehCtrl.getVTypeDistributionMembership(holderVTypeID).count(myManualTypeID) > 0) {
    //         // Holder type id is from the given manual type distribution.
    //         myState = ToCState::MANUAL;
    //         myManualTypeID = holderVTypeID;
    //     } else if (automatedVTypeIsDist && vehCtrl.getVTypeDistributionMembership(holderVTypeID).count(myAutomatedTypeID) > 0) {
    //         // Holder type id is from the given automated type distribution.
    //         myState = ToCState::AUTOMATED;
    //         myAutomatedTypeID = holderVTypeID;
    //     } else {
    //         throw ProcessError("Vehicle type of vehicle '" + holder.getID() + "' ('" +
    //                            holder.getVehicleType().getID() + "') must coincide with manualType ('" +
    //                            manualType + "') or automatedType ('" + automatedType +
    //                            "') specified for its ToC-device (or drawn from the specified vTypeDistributions).");
    //     }
    //     if (!vehCtrl.hasVType(myAutomatedTypeID)) {
    //         throw ProcessError("The automated vehicle type '" + myAutomatedTypeID +
    //                            "' of vehicle '" + holder.getID() + "' is not known.");
    //     }
    //     if (!vehCtrl.hasVType(myManualTypeID)) {
    //         throw ProcessError("The manual vehicle type '" + myManualTypeID +
    //                            "' of vehicle '" + holder.getID() + "' is not known.");
    //     }
    //
    //     // Eventually instantiate given vTypes from distributions
    //     if (myState == ToCState::MANUAL && automatedVTypeIsDist) {
    //         myAutomatedTypeID = vehCtrl.getVType(myAutomatedTypeID, MSRouteHandler::getParsingRNG())->getID();
    //     } else if (myState == ToCState::AUTOMATED && manualVTypeIsDist) {
    //         myManualTypeID = vehCtrl.getVType(myManualTypeID, MSRouteHandler::getParsingRNG())->getID();
    //     }
    //
    //     // register at static instance container
    //     myInstances.insert(this);
    //     initColorScheme();
    //
    // #ifdef DEBUG_TOC
    //     std::cout << "initialized device '" << id << "' with "
    //               << "outputFilename=" << outputFilename << ", "
    //               << "myManualType=" << myManualTypeID << ", "
    //               << "myAutomatedType=" << myAutomatedTypeID << ", "
    //               << "myResponseTime=" << myResponseTime << ", "
    //               << "myRecoveryRate=" << myRecoveryRate << ", "
    //               << "myInitialAwareness=" << myInitialAwareness << ", "
    //               << "myMRMDecel=" << myMRMDecel << ", "
    //               << "ogTimeHeadway=" << myOpenGapParams.newTimeHeadway << ", "
    //               << "ogSpaceHeadway=" << myOpenGapParams.newSpaceHeadway << ", "
    //               << "ogChangeRate=" << myOpenGapParams.changeRate << ", "
    //               << "ogMaxDecel=" << myOpenGapParams.maxDecel << ", "
    //               << "ogActive=" << myOpenGapParams.active << ", "
    //               << "myCurrentAwareness=" << myCurrentAwareness << ", "
    //               << "myState=" << _2string(myState) << std::endl;
    // #endif
    //
    //     assert(myInitialAwareness <= 1.0 && myInitialAwareness >= 0.0);
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, outputFilename: &str, manualType: &str, automatedType: &str, responseTime: SUMOTime, recoveryRate: f64, lcAbstinence: f64, initialAwareness: f64, mrmDecel: f64, dynamicToCThreshold: f64, dynamicMRMProbability: f64, maxPreparationAccel: f64, mrmKeepRight: bool, mrmSafeSpot: &str, mrmSafeSpotDuration: SUMOTime, useColorScheme: bool, ogp: MSDevice_ToC_OpenGapParams) -> Self {
        todo!("port MSDevice_ToC::MSDevice_ToC")
    }

    /// C++: `void MSDevice_ToC::initColorScheme()` [private]
    /// Initialize vehicle colors for different states
    // void
    // MSDevice_ToC::initColorScheme() {
    //     //RGBColor(red, green, blue)
    //     myColorScheme[MANUAL] = MSNet::getInstance()->getVehicleControl().getVType(myManualTypeID)->getColor();
    //     myColorScheme[AUTOMATED] = MSNet::getInstance()->getVehicleControl().getVType(myAutomatedTypeID)->getColor();
    //     myColorScheme[PREPARING_TOC] = RGBColor(200, 200, 250); // light blue
    //     myColorScheme[MRM] = RGBColor(250, 50, 50); // red
    //     myColorScheme[RECOVERING] = RGBColor(250, 210, 150); // light yellow
    //     myColorScheme[UNDEFINED] = RGBColor(150, 150, 150); // gray
    // }
    #[allow(unused_variables)]
    pub fn init_color_scheme(&mut self) {
        todo!("port MSDevice_ToC::initColorScheme")
    }

    /// C++: `void MSDevice_ToC::setAwareness(double)` [private]
    /// Set the awareness to the given value
    // void
    // MSDevice_ToC::setAwareness(double value) {
    //     if (value > 1.0 || value < 0.0) {
    //         std::stringstream ss;
    //         ss << "Truncating invalid value for awareness (" << value << ") to lie in [0,1].";
    //         WRITE_WARNING(ss.str());
    //         value = MAX2(0.0, MIN2(1.0, value));
    //     }
    //     if (myCurrentAwareness >= myLCAbstinence && value < myLCAbstinence) {
    //         // Awareness is now below LC abstinence level -> prevent deliberate LCs
    //         deactivateDeliberateLCs();
    //     } else if (myCurrentAwareness < myLCAbstinence && value >= myLCAbstinence) {
    //         // Awareness is now above LC abstinence level -> allow deliberate LCs
    //         resetDeliberateLCs();
    //     }
    //     myCurrentAwareness = value;
    //     myHolderMS->getDriverState()->setAwareness(value);
    // }
    #[allow(unused_variables)]
    pub fn set_awareness(&mut self, value: f64) {
        todo!("port MSDevice_ToC::setAwareness")
    }

    /// C++: `void MSDevice_ToC::setState(ToCState)` [private]
    /// Set the ToC device's state
    // void
    // MSDevice_ToC::setState(ToCState state) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " MSDevice_ToC::setState()" << std::endl;
    // #endif
    //     if (myState == state) {
    //         // No state change
    //         return;
    //     }
    //
    //     if (myState == MRM) {
    //         // reset the vehicle's maxAccel
    //         myHolderMS->getSingularType().getCarFollowModel().setMaxAccel(myOriginalMaxAccel);
    //         resetDeliberateLCs();
    //     } else if (myState == PREPARING_TOC) {
    //         if (myOpenGapParams.active) {
    //             // Deactivate gap control at preparation phase end
    //             myHolderMS->getInfluencer().deactivateGapController();
    //         }
    //         if (state != MRM) {
    //             // Aborting preparation
    //             resetDeliberateLCs();
    //             myHolderMS->getSingularType().getCarFollowModel().setMaxAccel(myOriginalMaxAccel);
    //         }
    //     } else if (state == PREPARING_TOC || state == MRM) {
    // #ifdef DEBUG_TOC
    //         std::cout << "  Entering ToC preparation... " << std::endl;
    // #endif
    //         // Prevent lane changing during takeover preparation
    //         deactivateDeliberateLCs();
    //         // Store original value of maxAccel for restoring it after preparation phase
    //         myOriginalMaxAccel = myHolderMS->getCarFollowModel().getMaxAccel();
    //         // Impose acceleration limit during preparation
    //         myHolderMS->getSingularType().getCarFollowModel().setMaxAccel(MIN2(myMaxPreparationAccel, myOriginalMaxAccel));
    //     }
    //
    //     if (myIssuedDynamicToC) {
    //         // Reset dynamic ToC flag
    //         myIssuedDynamicToC = false;
    //     }
    //
    //     myState = state;
    //     if (myUseColorScheme) {
    //         setVehicleColor();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_state(&mut self, state: MSDevice_ToC_ToCState) {
        todo!("port MSDevice_ToC::setState")
    }

    /// C++: `void MSDevice_ToC::setVehicleColor()` [private]
    // void
    // MSDevice_ToC::setVehicleColor() {
    //     const SUMOVehicleParameter& p = myHolder.getParameter();
    //     p.color = myColorScheme[myState];
    //     p.parametersSet |= VEHPARS_COLOR_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle_color(&mut self) {
        todo!("port MSDevice_ToC::setVehicleColor")
    }

    /// C++: `void MSDevice_ToC::requestToC(SUMOTime, SUMOTime)` [private]
    /// Request a ToC. If the device is in AUTOMATED or MRM state, a driver response time is sampled and the ToC is scheduled. If the response is larger than timeTillMRM, an MRM is scheduled as well. If the device is in MANUAL or UNDEFINED state, it switches to AUTOMATED. The request is ignored if the state is already PREPARING_TOC.
    // void
    // MSDevice_ToC::requestToC(SUMOTime timeTillMRM, SUMOTime responseTime) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " requestToC() for vehicle '" << myHolder.getID() << "', timeTillMRM=" << timeTillMRM << ", responseTime=" << responseTime << std::endl;
    // #endif
    //     if (myState == AUTOMATED) {
    //         // Initialize preparation phase
    //         if (responseTime == -1000) {
    //             // Sample response time from distribution
    //             const double sample = sampleResponseTime(STEPS2TIME(timeTillMRM));
    //             // this needs to be a separate line because TIME2STEPS may otherwise do two calls to sampleResponseTime
    //             responseTime = TIME2STEPS(sample);
    //         }
    //
    //         // Schedule ToC Event
    //         myTriggerToCCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::triggerDownwardToC);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myTriggerToCCommand, SIMSTEP + responseTime);
    //
    //         assert(myExecuteMRMCommand == nullptr);
    //         assert(myTriggerMRMCommand == nullptr);
    //         if (responseTime > timeTillMRM && myState != MRM) {
    //             // Schedule new MRM if driver response time is higher than permitted
    //             myTriggerMRMCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::triggerMRM);
    //             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myTriggerMRMCommand, SIMSTEP + timeTillMRM);
    //         }
    //
    //         // Start ToC preparation process
    //         myPrepareToCCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::ToCPreparationStep);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myPrepareToCCommand, SIMSTEP + DELTA_T);
    //         setState(PREPARING_TOC);
    //         if (myOpenGapParams.active) {
    //             // Start gap controller
    //             double originalTau = myHolderMS->getCarFollowModel().getHeadwayTime();
    //             myHolderMS->getInfluencer().activateGapController(originalTau,
    //                     myOpenGapParams.newTimeHeadway, myOpenGapParams.newSpaceHeadway, -1,
    //                     myOpenGapParams.changeRate, myOpenGapParams.maxDecel);
    //         }
    //         // Record event
    //         if (generatesOutput()) {
    //             myEvents.push(std::make_pair(SIMSTEP, "TOR"));
    //             myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //             myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //         }
    //     } else {
    //         // Switch to automated mode is performed immediately
    //         if (timeTillMRM > 0) {
    //             std::stringstream ss;
    //             ss << "[t=" << SIMTIME << "] Positive transition time (" << STEPS2TIME(timeTillMRM) << "s.) for upward ToC of vehicle '" << myHolder.getID() << "' is ignored.";
    //             WRITE_WARNING(ss.str());
    //         }
    //         triggerUpwardToC(SIMSTEP + DELTA_T);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn request_to_c(&mut self, timeTillMRM: SUMOTime, responseTime: SUMOTime) {
        todo!("port MSDevice_ToC::requestToC")
    }

    /// C++: `void MSDevice_ToC::requestMRM()` [private]
    /// Request an MRM to be initiated immediately. No downward ToC will be scheduled.
    // void
    // MSDevice_ToC::requestMRM() {
    //     // Remove any preparatory process
    //     descheduleToCPreparation();
    //     // .. and any recovery process
    //     descheduleRecovery();
    //     // ... and any pending ToC to manual
    //     descheduleToC();
    //     // Immediately trigger the MRM process
    //     triggerMRM(0);
    // }
    #[allow(unused_variables)]
    pub fn request_mrm(&mut self) {
        todo!("port MSDevice_ToC::requestMRM")
    }

    /// C++: `void MSDevice_ToC::switchHolderType(const std::string &)` [private]
    /// Switch the device holder's vehicle type
    // void
    // MSDevice_ToC::switchHolderType(const std::string& targetTypeID) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " Switching type of vehicle '" << myHolder.getID() << "' to '" << targetTypeID << "'" << std::endl;
    // #endif
    //     MSVehicleType* targetType = MSNet::getInstance()->getVehicleControl().getVType(targetTypeID);
    //     if (targetType == nullptr) {
    //         WRITE_ERRORF(TL("vType '%' for vehicle '%' is not known."), targetType->getID(), myHolder.getID());
    //         return;
    //     }
    //     myHolderMS->replaceVehicleType(targetType);
    // }
    #[allow(unused_variables)]
    pub fn switch_holder_type(&mut self, targetTypeID: &str) {
        todo!("port MSDevice_ToC::switchHolderType")
    }

    /// C++: `void MSDevice_ToC::descheduleMRM()` [private]
    /// Break MRM Process or remove MRM-Trigger command from the event-queue.
    // void
    // MSDevice_ToC::descheduleMRM() {
    //     // Eventually abort scheduled MRM
    //     if (myTriggerMRMCommand != nullptr) {
    //         myTriggerMRMCommand->deschedule();
    //         myTriggerMRMCommand = nullptr;
    //     }
    //     // Eventually abort ongoing MRM
    //     if (myExecuteMRMCommand != nullptr) {
    //         myExecuteMRMCommand->deschedule();
    //         resetDeliberateLCs();
    //         myExecuteMRMCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_mrm(&mut self) {
        todo!("port MSDevice_ToC::descheduleMRM")
    }

    /// C++: `void MSDevice_ToC::descheduleToC()` [private]
    /// Remove scheduled ToC-Trigger command from the event-queue.
    // void
    // MSDevice_ToC::descheduleToC() {
    //     if (myTriggerToCCommand != nullptr) {
    //         myTriggerToCCommand->deschedule();
    //         myTriggerToCCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_to_c(&mut self) {
        todo!("port MSDevice_ToC::descheduleToC")
    }

    /// C++: `void MSDevice_ToC::descheduleToCPreparation()` [private]
    /// Remove ongoing ToC-Preparation process from the event-queue.
    // void
    // MSDevice_ToC::descheduleToCPreparation() {
    //     // Eventually stop ToC preparation process
    //     if (myPrepareToCCommand != nullptr) {
    //         myPrepareToCCommand->deschedule();
    //         if (myState != MRM) {
    //             resetDeliberateLCs();
    //         }
    //         myPrepareToCCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_to_c_preparation(&mut self) {
        todo!("port MSDevice_ToC::descheduleToCPreparation")
    }

    /// C++: `void MSDevice_ToC::descheduleRecovery()` [private]
    /// Remove ongoing awareness recovery process from the event-queue.
    // void
    // MSDevice_ToC::descheduleRecovery() {
    //     // Eventually stop ToC preparation process
    //     if (myRecoverAwarenessCommand != nullptr) {
    //         myRecoverAwarenessCommand->deschedule();
    //         myRecoverAwarenessCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_recovery(&mut self) {
        todo!("port MSDevice_ToC::descheduleRecovery")
    }

    /// C++: `void MSDevice_ToC::resetDeliberateLCs()` [private]
    /// Resets the holder's LC mode to the last differing to LCModeMRM
    // void
    // MSDevice_ToC::resetDeliberateLCs() {
    //     if (myPreviousLCMode != -1) {
    //         myHolderMS->getInfluencer().setLaneChangeMode(myPreviousLCMode);
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " MSDevice_ToC::resetLCMode() restoring LC Mode of vehicle '" << myHolder.getID() << "' to " << myPreviousLCMode << std::endl;
    // #endif
    //     }
    //     myPreviousLCMode = -1;
    // }
    #[allow(unused_variables)]
    pub fn reset_deliberate_l_cs(&mut self) {
        todo!("port MSDevice_ToC::resetDeliberateLCs")
    }

    /// C++: `void MSDevice_ToC::deactivateDeliberateLCs()` [private]
    /// Resets the holder's LC mode to the operational LC-mode of the ToC Device (
    // void
    // MSDevice_ToC::deactivateDeliberateLCs() {
    //     const int lcModeHolder = myHolderMS->getInfluencer().getLaneChangeMode();
    //     if (lcModeHolder != LCModeMRM) {
    //         myPreviousLCMode = lcModeHolder;
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " MSDevice_ToC::setLCModeMRM() setting LC Mode of vehicle '" << myHolder.getID()
    //                   << "' from " << myPreviousLCMode << " to " << LCModeMRM << std::endl;
    // #endif
    //         myHolderMS->getInfluencer().setLaneChangeMode(LCModeMRM);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deactivate_deliberate_l_cs(&mut self) {
        todo!("port MSDevice_ToC::deactivateDeliberateLCs")
    }

    /// C++: `bool MSDevice_ToC::isManuallyDriven()` [private]
    /// Whether the current operation mode is manual
    // bool
    // MSDevice_ToC::isManuallyDriven() {
    //     return (myState == MANUAL || myState == RECOVERING);
    // }
    #[allow(unused_variables)]
    pub fn is_manually_driven(&mut self) -> bool {
        todo!("port MSDevice_ToC::isManuallyDriven")
    }

    /// C++: `bool MSDevice_ToC::isAutomated()` [private]
    /// Whether the current operation mode is automated
    // bool
    // MSDevice_ToC::isAutomated() {
    //     return (myState == AUTOMATED || myState == PREPARING_TOC || myState == MRM);
    // }
    #[allow(unused_variables)]
    pub fn is_automated(&mut self) -> bool {
        todo!("port MSDevice_ToC::isAutomated")
    }

    /// C++: `bool MSDevice_ToC::checkDynamicToC()` [private]
    /// Check if the vehicle should induce a ToC due to internal reasons. That is, if the route cannot be followed for more time than a given threshold,
    // bool
    // MSDevice_ToC::checkDynamicToC() {
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << SIMTIME << " # MSDevice_ToC::checkDynamicToC() for veh '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     if (!myDynamicToCActive) {
    //         return false;
    //     }
    //     // The vehicle's current lane
    //     const MSLane* currentLane = myHolderMS->getLane();
    //
    //     if (currentLane->isInternal()) {
    //         // Don't start or abort dynamic ToCs on internal lanes
    //         return myIssuedDynamicToC;
    //     }
    //
    //     if (myIssuedDynamicToC) {
    // #ifdef DEBUG_DYNAMIC_TOC
    //         std::cout << SIMTIME << " Dynamic ToC is ongoing." << std::endl;
    // #endif
    //         // Dynamic ToC in progress. Resist to aborting it if lane was not changed.
    //         if (myDynamicToCLane == currentLane->getNumericalID()) {
    //             return true;
    //         }
    //     }
    //     // Length for which the current route can be followed
    //     const std::vector<MSVehicle::LaneQ>& bestLanes = myHolderMS->getBestLanes();
    //     // Maximal distance for route continuation without LCs over the possible start lanes
    //     double maximalContinuationDistance = 0;
    //     // Distance for route continuation without LCs from the vehicle's current lane
    //     double continuationDistanceOnCurrent = 0;
    //     // Lane of the next stop
    //     const MSLane* nextStopLane = nullptr;
    //
    //     if (myHolderMS->hasStops()) {
    //         nextStopLane = myHolderMS->getNextStop().lane;
    //     }
    //     for (auto& i : bestLanes) {
    //         maximalContinuationDistance = MAX2(maximalContinuationDistance, i.length);
    //         if (currentLane == i.lane) {
    //             if (myHolderMS->hasStops()) {
    //                 // Check if the next stop lies on the route continuation from the current lane
    //                 for (MSLane* l : i.bestContinuations) {
    //                     if (l == nextStopLane) {
    // #ifdef DEBUG_DYNAMIC_TOC
    //                         std::cout << SIMTIME << " Stop found on the route continuation from the current lane. => No ToC" << std::endl;
    // #endif
    //                         // Stop found on the route continuation from the current lane => no ToC necessary
    //                         return false;
    //                     }
    //                 }
    //             }
    //             continuationDistanceOnCurrent = i.length;
    //         }
    //     }
    //     if (continuationDistanceOnCurrent == maximalContinuationDistance) {
    //         // There is no better lane than the current, hence no desire to change lanes,
    //         // which the driver could pursue better than the automation => no reason for ToC.
    //         return false;
    //     }
    //     const double distFromCurrent = continuationDistanceOnCurrent - myHolderMS->getPositionOnLane();
    //     const double MRMDist = 0.5 * myHolderMS->getSpeed() * myHolderMS->getSpeed() / MAX2(myMRMDecel, 0.0001);
    //     double distThreshold = myHolderMS->getSpeed() * myDynamicToCThreshold + MRMDist;
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "  speed=" << myHolderMS->getSpeed()
    //               << ", distFromCurrent=" << distFromCurrent
    //               << ", maximal dist=" << maximalContinuationDistance - myHolderMS->getPositionOnLane()
    //               << ", distThreshold=" << distThreshold
    //               << std::endl;
    // #endif
    //
    //     if (myIssuedDynamicToC) {
    //         // In case of an ongoing ToC, add an additional resistance to abort it.
    //         // (The lane-check above does not capture lanes subsequent to the dynamic ToC lane)
    //         distThreshold *= DYNAMIC_TOC_ABORT_RESISTANCE_FACTOR;
    //     }
    //
    //     if (distFromCurrent < distThreshold) {
    //         // TODO: Make this more sophisticated in dealing with low speeds/stops and route ends
    // #ifdef DEBUG_DYNAMIC_TOC
    //         std::cout << SIMTIME << "  * distAlongBest is below threshold! *" << std::endl;
    // #endif
    //         return true;
    //     }
    //
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_dynamic_to_c(&mut self) -> bool {
        todo!("port MSDevice_ToC::checkDynamicToC")
    }

    /// C++: `double MSDevice_ToC::responseTimeMean(double)` [static, private]
    /// Mean of the response time distribution. (Only depends on given lead time)
    // static double responseTimeMean(double leadTime) {
    //         return MIN2(2 * sqrt(leadTime), 0.7 * leadTime);
    //     }
    #[allow(unused_variables)]
    pub fn response_time_mean(leadTime: f64) -> f64 {
        todo!("port MSDevice_ToC::responseTimeMean")
    }

    /// C++: `double MSDevice_ToC::sampleResponseTime(double) const` [const, private]
    /// Samples a random driver response time from a truncated Gaussian with parameters according to the lookup tables
    // double
    // MSDevice_ToC::sampleResponseTime(double leadTime) const {
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "sampleResponseTime() leadTime=" << leadTime << std::endl;
    // #endif
    //     const double mean = responseTimeMean(leadTime);
    //     const double var = interpolateVariance(leadTime, myMRMProbability);
    //     double rt = RandHelper::randNorm(mean, var, &myResponseTimeRNG);
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "  mean=" << mean << ", variance=" << var << " => sampled responseTime=" << rt << std::endl;
    // #endif
    //     int it_count = 0;
    //     while (rt < 0 && it_count < MAX_RESPONSETIME_SAMPLE_TRIES) {
    //         rt = RandHelper::randNorm(mean, var, &myResponseTimeRNG);
    //         it_count++;
    //     }
    //     if (rt < 0) {
    //         // Didn't generate a positive random response time => use mean
    //         rt = mean;
    //     }
    //     return rt;
    // }
    #[allow(unused_variables)]
    pub fn sample_response_time(&self, leadTime: f64) -> f64 {
        todo!("port MSDevice_ToC::sampleResponseTime")
    }

    /// C++: `double MSDevice_ToC::interpolateVariance(double, double)` [static, private]
    /// Two-dimensional interpolation of variance from lookup table assumes pMRM >= 0, leadTime >= 0
    // double
    // MSDevice_ToC::interpolateVariance(double leadTime, double pMRM) {
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "interpolateVariance() leadTime=" << leadTime << ", pMRM=" << pMRM << std::endl;
    // #endif
    //     // Calculate indices for surrounding values in lookup tables
    //
    //     // Find largest p_{i-1} < pMRM < p_{i}
    //     const auto pi = std::lower_bound(lookupResponseTimeMRMProbs.begin(), lookupResponseTimeMRMProbs.end(), pMRM);
    //     if (pi == lookupResponseTimeMRMProbs.end()) {
    //         // requested probability lies outside lookup table.
    //         // => return maximal variance value
    //         return MAX_RESPONSETIME_VARIANCE;
    //     }
    //     const size_t pi1 = pi - lookupResponseTimeMRMProbs.begin();
    //     assert(pi1 > 0);
    //     const size_t pi0 = pi1 - 1;
    //     const double cp = (pMRM - * (pi - 1)) / (*pi - * (pi - 1));
    //
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << " p[=" << pi0 << "]=" << *(pi - 1) << ", p[=" << pi1 << "]=" << *pi << " => cp=" << cp << std::endl;
    // #endif
    //
    //     // Find largest p_{i-1} < pMRM < p_{i}
    //     auto li = std::lower_bound(lookupResponseTimeLeadTimes.begin(), lookupResponseTimeLeadTimes.end(), leadTime);
    //     if (li == lookupResponseTimeLeadTimes.begin()) {
    //         // Given lead time smaller than minimal lookup-value.
    //         // Use minimal value from lookup table instead
    //         leadTime = *li;
    //         li = lookupResponseTimeLeadTimes.begin() + 1;
    //     } else if (li == lookupResponseTimeLeadTimes.end()) {
    //         // Given leadTime exceeds values in lookup table
    //         // => induce extrapolation
    //         li--;
    //     }
    //     const size_t li1 = li - lookupResponseTimeLeadTimes.begin();
    //     const size_t li0 = li1 - 1;
    //     const double cl = (leadTime - * (li - 1)) / (*li - * (li - 1));
    //
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << " l[=" << li0 << "]=" << *(li - 1) << ", l[=" << li1 << "]=" << *li << " => cp=" << cl << std::endl;
    // #endif
    //
    //     // 2D interpolation for variance
    //     // First, interpolate (or extrapolate) variances along leadTimes
    //     const double var00 = lookupResponseTimeVariances[pi0][li0];
    //     const double var01 = lookupResponseTimeVariances[pi0][li1];
    //     const double var10 = lookupResponseTimeVariances[pi1][li0];
    //     const double var11 = lookupResponseTimeVariances[pi1][li1];
    //     const double var_0 = var00 + (var01 - var00) * cl;
    //     const double var_1 = var10 + (var11 - var10) * cl;
    //     // From these, interpolate along the pMRM-axis
    //     const double var = var_0 + (var_1 - var_0) * cp;
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << " var00=" << var00 << ", var01=" << var01 << " var10=" << var10 << ", var11=" << var11
    //               << " var_0=" << var_0 << ", var_1=" << var_1 << ", var=" << var << std::endl;
    // #endif
    //     return var;
    // }
    #[allow(unused_variables)]
    pub fn interpolate_variance(leadTime: f64, pMRM: f64) -> f64 {
        todo!("port MSDevice_ToC::interpolateVariance")
    }

    /// C++: `void MSDevice_ToC::MSDevice_ToC(const MSDevice_ToC &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_ToC) -> Self {
        todo!("port MSDevice_ToC::MSDevice_ToC")
    }

    /// C++: `MSDevice_ToC & MSDevice_ToC::operator=(const MSDevice_ToC &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_ToC) -> &mut MSDevice_ToC {
        todo!("port MSDevice_ToC::operator=")
    }

}

/// C++ class `MSDevice_ToC::OpenGapParams` (microsim/devices/MSDevice_ToC.h)
pub struct MSDevice_ToC_OpenGapParams {
    pub newTimeHeadway: f64,
    pub newSpaceHeadway: f64,
    pub changeRate: f64,
    pub maxDecel: f64,
    pub active: bool,
}

impl MSDevice_ToC_OpenGapParams {
    /// C++: `void OpenGapParams::OpenGapParams(double, double, double, double, bool)`
    // OpenGapParams(double timegap, double spacing, double changeRate, double maxDecel, bool active) :
    //             newTimeHeadway(timegap), newSpaceHeadway(spacing), changeRate(changeRate), maxDecel(maxDecel), active(active)
    //         {}
    #[allow(unused_variables)]
    pub fn new(timegap: f64, spacing: f64, changeRate: f64, maxDecel: f64, active: bool) -> Self {
        todo!("port OpenGapParams::OpenGapParams")
    }

}

