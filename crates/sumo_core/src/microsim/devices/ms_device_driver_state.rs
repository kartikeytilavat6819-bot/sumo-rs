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
        oc.add_option_sub_topic("Driver State Device");
        MSDevice::insert_default_assignment_options("driverstate", "Driver State Device", oc, false);
        oc.do_register("device.driverstate.initialAwareness", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_INITIAL_AWARENESS));
        oc.add_description("device.driverstate.initialAwareness", "Driver State Device", "Initial value assigned to the driver's awareness.");
        oc.do_register("device.driverstate.errorTimeScaleCoefficient", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_ERROR_TIME_SCALE_COEFFICIENT));
        oc.add_description("device.driverstate.errorTimeScaleCoefficient", "Driver State Device", "Time scale for the error process.");
        oc.do_register("device.driverstate.errorNoiseIntensityCoefficient", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_ERROR_NOISE_INTENSITY_COEFFICIENT));
        oc.add_description("device.driverstate.errorNoiseIntensityCoefficient", "Driver State Device", "Noise intensity driving the error process.");
        oc.do_register("device.driverstate.speedDifferenceErrorCoefficient", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_SPEED_DIFFERENCE_ERROR_COEFFICIENT));
        oc.add_description("device.driverstate.speedDifferenceErrorCoefficient", "Driver State Device", "General scaling coefficient for applying the error to the perceived speed difference (error also scales with distance).");
        oc.do_register("device.driverstate.headwayErrorCoefficient", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_HEADWAY_ERROR_COEFFICIENT));
        oc.add_description("device.driverstate.headwayErrorCoefficient", "Driver State Device", "General scaling coefficient for applying the error to the perceived distance (error also scales with distance).");
        oc.do_register("device.driverstate.freeSpeedErrorCoefficient", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_FREE_SPEED_ERROR_COEFFICIENT));
        oc.add_description("device.driverstate.freeSpeedErrorCoefficient", "Driver State Device", "General scaling coefficient for applying the error to the vehicle's own speed when driving without a leader (error also scales with own speed).");
        oc.do_register("device.driverstate.speedDifferenceChangePerceptionThreshold", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_SPEED_DIFFERENCE_CHANGE_PERCEPTION_THRESHOLD));
        oc.add_description("device.driverstate.speedDifferenceChangePerceptionThreshold", "Driver State Device", "Base threshold for recognizing changes in the speed difference (threshold also scales with distance).");
        oc.do_register("device.driverstate.headwayChangePerceptionThreshold", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_HEADWAY_CHANGE_PERCEPTION_THRESHOLD));
        oc.add_description("device.driverstate.headwayChangePerceptionThreshold", "Driver State Device", "Base threshold for recognizing changes in the headway (threshold also scales with distance).");
        oc.do_register("device.driverstate.minAwareness", SumoOption::float(crate::microsim::ms_driver_state::DRIVER_STATE_MIN_AWARENESS));
        oc.add_description("device.driverstate.minAwareness", "Driver State Device", "Minimal admissible value for the driver's awareness.");
        oc.do_register("device.driverstate.maximalReactionTime", SumoOption::float(-1.0));
        oc.add_description("device.driverstate.maximalReactionTime", "Driver State Device", "Maximal reaction time (~action step length) induced by decreased awareness level (reached for awareness=minAwareness).");
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
        // ToC device implies driverstate
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "driverstate", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), false, false)
                || MSDevice::equipped_by_default_assignment_options(oc, "toc", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), false, false)
        });
        if equipped {
            // build the device
            let id = format!("driverstate{}", v.get_id());
            let minAwareness = v.get_float_param("device.driverstate.minAwareness", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let initialAwareness = v.get_float_param("device.driverstate.initialAwareness", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let errorTimeScaleCoefficient = v.get_float_param("device.driverstate.errorTimeScaleCoefficient", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let errorNoiseIntensityCoefficient = v.get_float_param("device.driverstate.errorNoiseIntensityCoefficient", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let speedDifferenceErrorCoefficient = v.get_float_param("device.driverstate.speedDifferenceErrorCoefficient", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let speedDifferenceChangePerceptionThreshold = v.get_float_param("device.driverstate.speedDifferenceChangePerceptionThreshold", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let headwayChangePerceptionThreshold = v.get_float_param("device.driverstate.headwayChangePerceptionThreshold", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let headwayErrorCoefficient = v.get_float_param("device.driverstate.headwayErrorCoefficient", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let freeSpeedErrorCoefficient = v.get_float_param("device.driverstate.freeSpeedErrorCoefficient", false, utils::common::std_defs::INVALID_DOUBLE, true);
            let maximalReactionTime = v.get_float_param("device.driverstate.maximalReactionTime", false, utils::common::std_defs::INVALID_DOUBLE, true);
            // shortcut: the device is leaked like the C++ new, so the vehicle's device list keeps a stable pointer; upgrade when MSVehicle owns its devices as Box
            let device: &mut MSDevice_DriverState = Box::leak(Box::new(MSDevice_DriverState::new(v, &id,
                    minAwareness, initialAwareness, errorTimeScaleCoefficient, errorNoiseIntensityCoefficient,
                    speedDifferenceErrorCoefficient, speedDifferenceChangePerceptionThreshold,
                    headwayChangePerceptionThreshold, headwayErrorCoefficient, freeSpeedErrorCoefficient,
                    maximalReactionTime)));
            into.push(Ptr::from_mut(&mut device.base));
        }
    }

    /// C++: `void MSDevice_DriverState::update()`
    /// update internal state
    // void
    // MSDevice_DriverState::update() {
    //     myDriverState->update();
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self) {
        self.driver_state_mut().update();
    }

    /// Mutable access to the shared driver state (C++ shared_ptr semantics; the Rc is also handed out by getDriverState()).
    fn driver_state_mut(&self) -> &mut MSSimpleDriverState {
        unsafe { &mut *(Rc::as_ptr(&self.myDriverState) as *mut MSSimpleDriverState) }
    }

    /// C++: `std::shared_ptr<MSSimpleDriverState> MSDevice_DriverState::getDriverState() const` [const]
    /// return internal state
    // inline std::shared_ptr<MSSimpleDriverState> getDriverState() const {
    //         return myDriverState;
    //     }
    #[allow(unused_variables)]
    pub fn get_driver_state(&self) -> Rc<MSSimpleDriverState> {
        Rc::clone(&self.myDriverState)
    }

    /// C++: `void MSDevice_DriverState::~MSDevice_DriverState() noexcept` [virtual]
    /// Destructor.
    // ~MSDevice_DriverState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the Rc field is released by Rust drop glue
    }

    /// C++: `const std::string MSDevice_DriverState::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "driverstate";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "driverstate".to_string()
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
        let ds: &MSSimpleDriverState = &self.myDriverState;
        let value: f64 = match key {
            "awareness" => ds.get_awareness(),
            "errorState" => ds.get_error_state(),
            "errorTimeScale" => ds.get_error_time_scale(),
            "errorNoiseIntensity" => ds.get_error_noise_intensity(),
            "minAwareness" => ds.get_min_awareness(),
            "initialAwareness" => ds.get_initial_awareness(),
            "errorTimeScaleCoefficient" => ds.get_error_time_scale_coefficient(),
            "errorNoiseIntensityCoefficient" => ds.get_error_noise_intensity_coefficient(),
            "speedDifferenceErrorCoefficient" => ds.get_speed_difference_error_coefficient(),
            "headwayErrorCoefficient" => ds.get_headway_error_coefficient(),
            "speedDifferenceChangePerceptionThreshold" => ds.get_speed_difference_change_perception_threshold(),
            "headwayChangePerceptionThreshold" => ds.get_headway_change_perception_threshold(),
            "maximalReactionTime" => ds.get_maximal_reaction_time(),
            "originalReactionTime" => ds.get_original_reaction_time(),
            "actionStepLength" => ds.get_action_step_length(),
            // shortcut: C++ throws InvalidArgument; the port panics with the same text because overrides return String, upgrade when device getParameter returns Result
            _ => panic!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name()),
        };
        utils::common::to_string::to_string(&value)
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
        let ds = self.driver_state_mut();
        // shortcut: StringUtils::toDouble throws NumberFormatException in C++; the port panics with its message, upgrade when device setParameter returns Result
        let to_double = || utils::common::string_utils::to_double(value).unwrap_or_else(|e| panic!("{}", e));
        match key {
            "awareness" => ds.set_awareness(to_double()),
            "errorState" => ds.set_error_state(to_double()),
            "errorTimeScale" => ds.set_error_time_scale(to_double()),
            "errorNoiseIntensity" => ds.set_error_noise_intensity(to_double()),
            "minAwareness" => ds.set_min_awareness(to_double()),
            "initialAwareness" => ds.set_initial_awareness(to_double()),
            "errorTimeScaleCoefficient" => ds.set_error_time_scale_coefficient(to_double()),
            "errorNoiseIntensityCoefficient" => ds.set_error_noise_intensity_coefficient(to_double()),
            "speedDifferenceErrorCoefficient" => ds.set_speed_difference_error_coefficient(to_double()),
            "headwayErrorCoefficient" => ds.set_headway_error_coefficient(to_double()),
            "freeSpeedErrorCoefficient" => ds.set_free_speed_error_coefficient(to_double()),
            "speedDifferenceChangePerceptionThreshold" => ds.set_speed_difference_change_perception_threshold(to_double()),
            "headwayChangePerceptionThreshold" => ds.set_headway_change_perception_threshold(to_double()),
            "maximalReactionTime" => ds.set_maximal_reaction_time(to_double()),
            "originalReactionTime" => ds.set_original_reaction_time(to_double()),
            // shortcut: C++ throws InvalidArgument; the port panics with the same text, upgrade when device setParameter returns Result
            _ => panic!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name()),
        }
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
        let holderMS = Ptr::from_mut(holder).cast::<MSVehicle>();
        let mut b = MSDevice_DriverState {
            base: MSVehicleDevice::new(holder, id),
            myHolderMS: holderMS,
            myMinAwareness: minAwareness,
            myInitialAwareness: initialAwareness,
            myErrorTimeScaleCoefficient: errorTimeScaleCoefficient,
            myErrorNoiseIntensityCoefficient: errorNoiseIntensityCoefficient,
            mySpeedDifferenceErrorCoefficient: speedDifferenceErrorCoefficient,
            mySpeedDifferenceChangePerceptionThreshold: speedDifferenceChangePerceptionThreshold,
            myHeadwayChangePerceptionThreshold: headwayChangePerceptionThreshold,
            myHeadwayErrorCoefficient: headwayErrorCoefficient,
            myFreeSpeedErrorCoefficient: freeSpeedErrorCoefficient,
            myMaximalReactionTime: maximalReactionTime,
            myDriverState: Rc::new(MSSimpleDriverState::new(holderMS)),
        };
        b.init_driver_state();
        b
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
        // the state itself is created in new() (C++ make_shared); its parameters are set here in place
        let ds = Rc::get_mut(&mut self.myDriverState).unwrap();
        ds.set_min_awareness(self.myMinAwareness);
        ds.set_initial_awareness(self.myInitialAwareness);
        ds.set_error_time_scale_coefficient(self.myErrorTimeScaleCoefficient);
        ds.set_error_noise_intensity_coefficient(self.myErrorNoiseIntensityCoefficient);
        ds.set_speed_difference_error_coefficient(self.mySpeedDifferenceErrorCoefficient);
        ds.set_headway_error_coefficient(self.myHeadwayErrorCoefficient);
        ds.set_free_speed_error_coefficient(self.myFreeSpeedErrorCoefficient);
        ds.set_speed_difference_change_perception_threshold(self.mySpeedDifferenceChangePerceptionThreshold);
        ds.set_headway_change_perception_threshold(self.myHeadwayChangePerceptionThreshold);
        ds.set_awareness(self.myInitialAwareness);
        if self.myMaximalReactionTime > 0. {
            ds.set_maximal_reaction_time(self.myMaximalReactionTime);
        }
    }

    /// C++: `void MSDevice_DriverState::MSDevice_DriverState(const MSDevice_DriverState &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_DriverState) -> Self {
        panic!("MSDevice_DriverState copy constructor is private and invalid")
    }

    /// C++: `MSDevice_DriverState & MSDevice_DriverState::operator=(const MSDevice_DriverState &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_DriverState) -> &mut MSDevice_DriverState {
        panic!("MSDevice_DriverState assignment operator is private and invalid")
    }

}

