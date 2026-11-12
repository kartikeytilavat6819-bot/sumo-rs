//! Port of microsim/devices/MSDevice_DriverState.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The ToC Device controls transition of control between automated and manual driving.
/// C++ class `MSDevice_DriverState` (microsim/devices/MSDevice_DriverState.h)
#[repr(C)]
pub struct MSDevice_DriverState {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// The holder vehicle casted to MSVehicle*
    pub myHolderMS: Ptr<MSVehicle>,
    pub myMinAwareness: f64,
    pub myInitialAwareness: f64,
    pub myErrorTimeScaleCoefficient: f64,
    pub myErrorNoiseIntensityCoefficient: f64,
    pub mySpeedDifferenceErrorCoefficient: f64,
    pub mySpeedDifferenceChangePerceptionThreshold: f64,
    pub myHeadwayChangePerceptionThreshold: f64,
    pub myHeadwayErrorCoefficient: f64,
    pub myFreeSpeedErrorCoefficient: f64,
    pub myMaximalReactionTime: f64,
    /// The driver state of the holder.
    pub myDriverState: Rc<MSSimpleDriverState>,
}

impl std::ops::Deref for MSDevice_DriverState {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_DriverState {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_DriverState {
    /// C++: `void MSDevice_DriverState::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_DriverState-options
    // void
    // MSDevice_DriverState::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("Driver State Device");
    //     insertDefaultAssignmentOptions("driverstate", "Driver State Device", oc);
    //     oc.doRegister("device.driverstate.initialAwareness", new Option_Float(DriverStateDefaults::initialAwareness));
    //     oc.addDescription("device.driverstate.initialAwareness", "Driver State Device", TL("Initial value assigned to the driver's awareness."));
    //     oc.doRegister("device.driverstate.errorTimeScaleCoefficient", new Option_Float(DriverStateDefaults::errorTimeScaleCoefficient));
    //     oc.addDescription("device.driverstate.errorTimeScaleCoefficient", "Driver State Device", TL("Time scale for the error process."));
    //     oc.doRegister("device.driverstate.errorNoiseIntensityCoefficient", new Option_Float(DriverStateDefaults::errorNoiseIntensityCoefficient));
    //     oc.addDescription("device.driverstate.errorNoiseIntensityCoefficient", "Driver State Device", TL("Noise intensity driving the error process."));
    //     oc.doRegister("device.driverstate.speedDifferenceErrorCoefficient", new Option_Float(DriverStateDefaults::speedDifferenceErrorCoefficient));
    //     oc.addDescription("device.driverstate.speedDifferenceErrorCoefficient", "Driver State Device", TL("General scaling coefficient for applying the error to the perceived speed difference (error also scales with distance)."));
    //     oc.doRegister("device.driverstate.headwayErrorCoefficient", new Option_Float(DriverStateDefaults::headwayErrorCoefficient));
    //     oc.addDescription("device.driverstate.headwayErrorCoefficient", "Driver State Device", TL("General scaling coefficient for applying the error to the perceived distance (error also scales with distance)."));
    //     oc.doRegister("device.driverstate.freeSpeedErrorCoefficient", new Option_Float(DriverStateDefaults::freeSpeedErrorCoefficient));
    //     oc.addDescription("device.driverstate.freeSpeedErrorCoefficient", "Driver State Device", TL("General scaling coefficient for applying the error to the vehicle's own speed when driving without a leader (error also scales with own speed)."));
    //     oc.doRegister("device.driverstate.speedDifferenceChangePerceptionThreshold", new Option_Float(DriverStateDefaults::speedDifferenceChangePerceptionThreshold));
    //     oc.addDescription("device.driverstate.speedDifferenceChangePerceptionThreshold", "Driver State Device", TL("Base threshold for recognizing changes in the speed difference (threshold also scales with distance)."));
    //     oc.doRegister("device.driverstate.headwayChangePerceptionThreshold", new Option_Float(DriverStateDefaults::headwayChangePerceptionThreshold));
    //     oc.addDescription("device.driverstate.headwayChangePerceptionThreshold", "Driver State Device", TL("Base threshold for recognizing changes in the headway (threshold also scales with distance)."));
    //     oc.doRegister("device.driverstate.minAwareness", new Option_Float(DriverStateDefaults::minAwareness));
    //     oc.addDescription("device.driverstate.minAwareness", "Driver State Device", TL("Minimal admissible value for the driver's awareness."));
    //     oc.doRegister("device.driverstate.maximalReactionTime", new Option_Float(-1.0));
    //     oc.addDescription("device.driverstate.maximalReactionTime", "Driver State Device", TL("Maximal reaction time (~action step length) induced by decreased awareness level (reached for awareness=minAwareness)."));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_DriverState::insertOptions")
    }

    /// C++: `void MSDevice_DriverState::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_DriverState::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     // ToC device implies driverstate
    //     if (equippedByDefaultAssignmentOptions(oc, "driverstate", v, false) || equippedByDefaultAssignmentOptions(oc, "toc", v, false)) {
    //         // build the device
    //         MSDevice_DriverState* device = new MSDevice_DriverState(v, "driverstate" + v.getID(),
    //                 v.getFloatParam("device.driverstate.minAwareness"),
    //                 v.getFloatParam("device.driverstate.initialAwareness"),
    //                 v.getFloatParam("device.driverstate.errorTimeScaleCoefficient"),
    //                 v.getFloatParam("device.driverstate.errorNoiseIntensityCoefficient"),
    //                 v.getFloatParam("device.driverstate.speedDifferenceErrorCoefficient"),
    //                 v.getFloatParam("device.driverstate.speedDifferenceChangePerceptionThreshold"),
    //                 v.getFloatParam("device.driverstate.headwayChangePerceptionThreshold"),
    //                 v.getFloatParam("device.driverstate.headwayErrorCoefficient"),
    //                 v.getFloatParam("device.driverstate.freeSpeedErrorCoefficient"),
    //                 v.getFloatParam("device.driverstate.maximalReactionTime"));
    //         into.push_back(device);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_DriverState::buildVehicleDevices")
    }

    /// C++: `void MSDevice_DriverState::update()`
    /// update internal state
    // void
    // MSDevice_DriverState::update() {
    //     myDriverState->update();
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self) {
        todo!("port MSDevice_DriverState::update")
    }

    /// C++: `std::shared_ptr<MSSimpleDriverState> MSDevice_DriverState::getDriverState() const` [const]
    /// return internal state
    // inline std::shared_ptr<MSSimpleDriverState> getDriverState() const {
    //         return myDriverState;
    //     }
    #[allow(unused_variables)]
    pub fn get_driver_state(&self) -> Rc<MSSimpleDriverState> {
        todo!("port MSDevice_DriverState::getDriverState")
    }

    /// C++: `void MSDevice_DriverState::~MSDevice_DriverState() noexcept` [virtual]
    /// Destructor.
    // ~MSDevice_DriverState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_DriverState::~MSDevice_DriverState")
    }

    /// C++: `const std::string MSDevice_DriverState::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "driverstate";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_DriverState::deviceName")
    }

    /// C++: `std::string MSDevice_DriverState::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_DriverState::getParameter(const std::string& key) const {
    // #ifdef DEBUG_DSDEVICE
    //     std::cout << "MSDevice_DriverState::getParameter(key=" << key << ")" << std::endl;
    // #endif
    //     if (key == "awareness") {
    //         return toString(myDriverState->getAwareness());
    //     } else if (key == "errorState") {
    //         return toString(myDriverState->getErrorState());
    //     } else if (key == "errorTimeScale") {
    //         return toString(myDriverState->getErrorTimeScale());
    //     } else if (key == "errorNoiseIntensity") {
    //         return toString(myDriverState->getErrorNoiseIntensity());
    //     } else if (key == "minAwareness") {
    //         return toString(myDriverState->getMinAwareness());
    //     } else if (key == "initialAwareness") {
    //         return toString(myDriverState->getInitialAwareness());
    //     } else if (key == "errorTimeScaleCoefficient") {
    //         return toString(myDriverState->getErrorTimeScaleCoefficient());
    //     } else if (key == "errorNoiseIntensityCoefficient") {
    //         return toString(myDriverState->getErrorNoiseIntensityCoefficient());
    //     } else if (key == "speedDifferenceErrorCoefficient") {
    //         return toString(myDriverState->getSpeedDifferenceErrorCoefficient());
    //     } else if (key == "headwayErrorCoefficient") {
    //         return toString(myDriverState->getHeadwayErrorCoefficient());
    //     } else if (key == "speedDifferenceChangePerceptionThreshold") {
    //         return toString(myDriverState->getSpeedDifferenceChangePerceptionThreshold());
    //     } else if (key == "headwayChangePerceptionThreshold") {
    //         return toString(myDriverState->getHeadwayChangePerceptionThreshold());
    //     } else if (key == "maximalReactionTime") {
    //         return toString(myDriverState->getMaximalReactionTime());
    //     } else if (key == "originalReactionTime") {
    //         return toString(myDriverState->getOriginalReactionTime());
    //     } else if (key == "actionStepLength") {
    //         return toString(myDriverState->getActionStepLength());
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_DriverState::getParameter")
    }

    /// C++: `void MSDevice_DriverState::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_DriverState::setParameter(const std::string& key, const std::string& value) {
    // #ifdef DEBUG_DSDEVICE
    //     std::cout << "MSDevice_DriverState::setParameter(key=" << key << ", value=" << value << ")" << std::endl;
    // #endif
    //     if (key == "awareness") {
    //         myDriverState->setAwareness(StringUtils::toDouble(value));
    //     } else if (key == "errorState") {
    //         myDriverState->setErrorState(StringUtils::toDouble(value));
    //     } else if (key == "errorTimeScale") {
    //         myDriverState->setErrorTimeScale(StringUtils::toDouble(value));
    //     } else if (key == "errorNoiseIntensity") {
    //         myDriverState->setErrorNoiseIntensity(StringUtils::toDouble(value));
    //     } else if (key == "minAwareness") {
    //         myDriverState->setMinAwareness(StringUtils::toDouble(value));
    //     } else if (key == "initialAwareness") {
    //         myDriverState->setInitialAwareness(StringUtils::toDouble(value));
    //     } else if (key == "errorTimeScaleCoefficient") {
    //         myDriverState->setErrorTimeScaleCoefficient(StringUtils::toDouble(value));
    //     } else if (key == "errorNoiseIntensityCoefficient") {
    //         myDriverState->setErrorNoiseIntensityCoefficient(StringUtils::toDouble(value));
    //     } else if (key == "speedDifferenceErrorCoefficient") {
    //         myDriverState->setSpeedDifferenceErrorCoefficient(StringUtils::toDouble(value));
    //     } else if (key == "headwayErrorCoefficient") {
    //         myDriverState->setHeadwayErrorCoefficient(StringUtils::toDouble(value));
    //     } else if (key == "freeSpeedErrorCoefficient") {
    //         myDriverState->setFreeSpeedErrorCoefficient(StringUtils::toDouble(value));
    //     } else if (key == "speedDifferenceChangePerceptionThreshold") {
    //         myDriverState->setSpeedDifferenceChangePerceptionThreshold(StringUtils::toDouble(value));
    //     } else if (key == "headwayChangePerceptionThreshold") {
    //         myDriverState->setHeadwayChangePerceptionThreshold(StringUtils::toDouble(value));
    //     } else if (key == "maximalReactionTime") {
    //         myDriverState->setMaximalReactionTime(StringUtils::toDouble(value));
    //     } else if (key == "originalReactionTime") {
    //         myDriverState->setOriginalReactionTime(StringUtils::toDouble(value));
    //     } else {
    //         throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice_DriverState::setParameter")
    }

    /// C++: `void MSDevice_DriverState::MSDevice_DriverState(SUMOVehicle &, const std::string &, double, double, double, double, double, double, double, double, double, double)` [private]
    /// Constructor
    // MSDevice_DriverState::MSDevice_DriverState(SUMOVehicle& holder, const std::string& id,
    //         double minAwareness,
    //         double initialAwareness,
    //         double errorTimeScaleCoefficient,
    //         double errorNoiseIntensityCoefficient,
    //         double speedDifferenceErrorCoefficient,
    //         double speedDifferenceChangePerceptionThreshold,
    //         double headwayChangePerceptionThreshold,
    //         double headwayErrorCoefficient,
    //         double freeSpeedErrorCoefficient,
    //         double maximalReactionTime) :
    //     MSVehicleDevice(holder, id),
    //     myMinAwareness(minAwareness),
    //     myInitialAwareness(initialAwareness),
    //     myErrorTimeScaleCoefficient(errorTimeScaleCoefficient),
    //     myErrorNoiseIntensityCoefficient(errorNoiseIntensityCoefficient),
    //     mySpeedDifferenceErrorCoefficient(speedDifferenceErrorCoefficient),
    //     mySpeedDifferenceChangePerceptionThreshold(speedDifferenceChangePerceptionThreshold),
    //     myHeadwayChangePerceptionThreshold(headwayChangePerceptionThreshold),
    //     myHeadwayErrorCoefficient(headwayErrorCoefficient),
    //     myFreeSpeedErrorCoefficient(freeSpeedErrorCoefficient),
    //     myMaximalReactionTime(maximalReactionTime) {
    //     // Take care! Holder is currently being constructed. Cast occurs before completion.
    //     myHolderMS = static_cast<MSVehicle*>(&holder);
    //     initDriverState();
    //
    //
    // #ifdef DEBUG_DSDEVICE
    //     std::cout << "initialized device '" << id << "' with "
    //               << "myMinAwareness=" << myMinAwareness << ", "
    //               << "myInitialAwareness=" << myInitialAwareness << ", "
    //               << "myErrorTimeScaleCoefficient=" << myErrorTimeScaleCoefficient << ", "
    //               << "myErrorNoiseIntensityCoefficient=" << myErrorNoiseIntensityCoefficient << ", "
    //               << "mySpeedDifferenceErrorCoefficient=" << mySpeedDifferenceErrorCoefficient << ", "
    //               << "mySpeedDifferenceChangePerceptionThreshold=" << mySpeedDifferenceChangePerceptionThreshold << ", "
    //               << "myHeadwayChangePerceptionThreshold=" << myHeadwayChangePerceptionThreshold << ", "
    //               << "myHeadwayErrorCoefficient=" << myHeadwayErrorCoefficient << std::endl;
    //             << "myFreeSpeedErrorCoefficient=" << myFreeSpeedErrorCoefficient << std::endl;
    // #endif
    //
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, minAwareness: f64, initialAwareness: f64, errorTimeScaleCoefficient: f64, errorNoiseIntensityCoefficient: f64, speedDifferenceErrorCoefficient: f64, speedDifferenceChangePerceptionThreshold: f64, headwayChangePerceptionThreshold: f64, headwayErrorCoefficient: f64, freeSpeedErrorCoefficient: f64, maximalReactionTime: f64) -> Self {
        todo!("port MSDevice_DriverState::MSDevice_DriverState")
    }

    /// C++: `void MSDevice_DriverState::initDriverState()` [private]
    /// Initialises the driver state parameters
    // void
    // MSDevice_DriverState::initDriverState() {
    //     myDriverState = std::make_shared<MSSimpleDriverState>(myHolderMS);
    //     myDriverState->setMinAwareness(myMinAwareness);
    //     myDriverState->setInitialAwareness(myInitialAwareness);
    //     myDriverState->setErrorTimeScaleCoefficient(myErrorTimeScaleCoefficient);
    //     myDriverState->setErrorNoiseIntensityCoefficient(myErrorNoiseIntensityCoefficient);
    //     myDriverState->setSpeedDifferenceErrorCoefficient(mySpeedDifferenceErrorCoefficient);
    //     myDriverState->setHeadwayErrorCoefficient(myHeadwayErrorCoefficient);
    //     myDriverState->setFreeSpeedErrorCoefficient(myFreeSpeedErrorCoefficient);
    //     myDriverState->setSpeedDifferenceChangePerceptionThreshold(mySpeedDifferenceChangePerceptionThreshold);
    //     myDriverState->setHeadwayChangePerceptionThreshold(myHeadwayChangePerceptionThreshold);
    //     myDriverState->setAwareness(myInitialAwareness);
    //     if (myMaximalReactionTime > 0) {
    //         myDriverState->setMaximalReactionTime(myMaximalReactionTime);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_driver_state(&mut self) {
        todo!("port MSDevice_DriverState::initDriverState")
    }

    /// C++: `void MSDevice_DriverState::MSDevice_DriverState(const MSDevice_DriverState &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_DriverState) -> Self {
        todo!("port MSDevice_DriverState::MSDevice_DriverState")
    }

    /// C++: `MSDevice_DriverState & MSDevice_DriverState::operator=(const MSDevice_DriverState &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_DriverState) -> &mut MSDevice_DriverState {
        todo!("port MSDevice_DriverState::operator=")
    }

}

