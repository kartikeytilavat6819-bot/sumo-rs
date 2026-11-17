//! Port of microsim/devices/MSTransportableDevice_Routing.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::to_double;
use utils::common::time::{time2steps, time2string, SUMOTIME_MIN};
use utils::options::options_cont::with_options;

/// A device that performs person rerouting based on current edge speeds
/// C++ class `MSTransportableDevice_Routing` (microsim/devices/MSTransportableDevice_Routing.h)
#[repr(C)]
pub struct MSTransportableDevice_Routing {
    /// C++ base class `MSTransportableDevice`
    pub base: MSTransportableDevice,
    /// The period with which a vehicle shall be rerouted
    pub myPeriod: SUMOTime,
    /// The last time a routing took place
    pub myLastRouting: SUMOTime,
    /// The (optional) command responsible for rerouting
    pub myRerouteCommand: Ptr<Unmapped /* WrappingCommand<MSTransportableDevice_Routing> */>,
    /// The range of the plan which shall be replaced
    pub myScope: String,
}

impl std::ops::Deref for MSTransportableDevice_Routing {
    type Target = MSTransportableDevice;
    fn deref(&self) -> &MSTransportableDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportableDevice_Routing {
    fn deref_mut(&mut self) -> &mut MSTransportableDevice {
        &mut self.base
    }
}

impl MSTransportableDevice_Routing {
    /// C++: `void MSTransportableDevice_Routing::insertOptions(OptionsCont &)` [static]
    /// Inserts MSTransportableDevice_Routing-options
    // void
    // MSTransportableDevice_Routing::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("rerouting", "Routing", oc, true);
    //     oc.doRegister("person-device.rerouting.period", new Option_String("0", "TIME"));
    //     oc.addSynonyme("person-device.rerouting.period", "person-device.routing.period", true);
    //     oc.addDescription("person-device.rerouting.period", "Routing", TL("The period with which the person shall be rerouted"));
    //
    //     oc.doRegister("person-device.rerouting.mode", new Option_String("0"));
    //     oc.addDescription("person-device.rerouting.mode", "Routing", TL("Set routing flags (8 ignores temporary blockages)"));
    //
    //     oc.doRegister("person-device.rerouting.scope", new Option_String("stage"));
    //     oc.addDescription("person-device.rerouting.scope", "Routing", TL("Which part of the person plan is to be replaced (stage, sequence, or trip)"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        MSDevice::insert_default_assignment_options("rerouting", "Routing", oc, true);

        oc.do_register("person-device.rerouting.period", SumoOption::string_typed("0", "TIME"));
        oc.add_synonyme("person-device.rerouting.period", "person-device.routing.period", true);
        oc.add_description("person-device.rerouting.period", "Routing", "The period with which the person shall be rerouted");

        oc.do_register("person-device.rerouting.mode", SumoOption::string_with("0"));
        oc.add_description("person-device.rerouting.mode", "Routing", "Set routing flags (8 ignores temporary blockages)");

        oc.do_register("person-device.rerouting.scope", SumoOption::string_with("stage"));
        oc.add_description("person-device.rerouting.scope", "Routing", "Which part of the person plan is to be replaced (stage, sequence, or trip)");
    }

    /// C++: `bool MSTransportableDevice_Routing::checkOptions(OptionsCont &)` [static]
    /// checks MSTransportableDevice_Routing-options
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        // C++ declares this static but never defines it (MSDevice::checkOptions only calls MSDevice_Routing's)
        panic!("MSTransportableDevice_Routing::checkOptions has no C++ definition")
    }

    /// C++: `void MSTransportableDevice_Routing::buildDevices(MSTransportable &, std::vector<MSTransportableDevice *> &)` [static]
    /// Build devices for the given person, if needed
    // void
    // MSTransportableDevice_Routing::buildDevices(MSTransportable& p, std::vector<MSTransportableDevice*>& into) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (p.getParameter().wasSet(VEHPARS_FORCE_REROUTE) || equippedByDefaultAssignmentOptions(oc, "rerouting", p, false, true)) {
    //         // route computation is enabled
    //         const SUMOTime period = p.getTimeParam("person-device.rerouting.period");
    //         if (period > 0) {
    //             MSRoutingEngine::initWeightUpdate();
    //             // build the device
    //             into.push_back(new MSTransportableDevice_Routing(p, "routing_" + p.getID(), period));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_devices(p: &mut MSTransportable, into: &mut Vec<Ptr<MSTransportableDevice>>) {
        let force = p.get_parameter().was_set(VEHPARS_FORCE_REROUTE);
        let equipped = force || with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "rerouting", p.get_id(), p.get_parameter(), p.get_vehicle_type().get_parameter(), false, true)
        });
        if equipped {
            // route computation is enabled
            let period = p.get_time_param("person-device.rerouting.period", false, SUMOTIME_MIN);
            if period > 0 {
                MSRoutingEngine::init_weight_update(-1);
                // build the device
                let id = format!("routing_{}", p.get_id());
                // shortcut: device is leaked; the person's device list will own it once person device ownership is ported, upgrade then
                let device: &mut MSTransportableDevice_Routing = Box::leak(Box::new(MSTransportableDevice_Routing::new(p, &id, period)));
                into.push(Ptr::from_mut(&mut device.base));
            }
        }
    }

    /// C++: `void MSTransportableDevice_Routing::~MSTransportableDevice_Routing() noexcept` [virtual]
    /// Destructor.
    // MSTransportableDevice_Routing::~MSTransportableDevice_Routing() {
    //     // make the rerouting command invalid if there is one
    //     if (myRerouteCommand != nullptr) {
    //         myRerouteCommand->deschedule();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myRerouteCommand.is_null() {
            // shortcut: Command/WrappingCommand is not ported, so there is no event to deschedule; upgrade when Command exists
        }
    }

    /// C++: `const std::string MSTransportableDevice_Routing::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "rerouting";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "rerouting".to_string()
    }

    /// C++: `void MSTransportableDevice_Routing::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSTransportableDevice_Routing::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     std::vector<std::string> internals;
    //     internals.push_back(toString(myPeriod));
    //     out.writeAttr(SUMO_ATTR_STATE, toString(internals));
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_DEVICE);
        out.write_attr(SUMO_ATTR_ID, self.get_id());
        let internals = vec![format!("{}", self.myPeriod)];
        out.write_attr(SUMO_ATTR_STATE, &internals.join(" "));
        out.close_tag("");
    }

    /// C++: `void MSTransportableDevice_Routing::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSTransportableDevice_Routing::loadState(const SUMOSAXAttributes& attrs) {
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> myPeriod;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        let state = attrs.get_string(SUMO_ATTR_STATE);
        // istringstream >> SUMOTime: whitespace-separated integer, 0 on failed extraction
        self.myPeriod = state.split_whitespace().next().and_then(|s| s.parse::<SUMOTime>().ok()).unwrap_or(0);
    }

    /// C++: `void MSTransportableDevice_Routing::reroute(const SUMOTime, const bool)`
    /// initiate the rerouting, create router / thread pool on first use
    // void
    // MSTransportableDevice_Routing::reroute(const SUMOTime currentTime, const bool onInit) {
    //     MSRoutingEngine::initEdgeWeights(SVC_PEDESTRIAN);
    //     //check whether the weights did change since the last reroute
    //     if (myLastRouting >= MSRoutingEngine::getLastAdaptation()) {
    //         return;
    //     }
    //     myLastRouting = currentTime;
    //     MSRoutingEngine::reroute(myHolder, currentTime, "person-device.rerouting", onInit);
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, currentTime: SUMOTime, onInit: bool) {
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        MSRoutingEngine::init_edge_weights(SVC_PEDESTRIAN, -1, -1);
        //check whether the weights did change since the last reroute
        if self.myLastRouting >= MSRoutingEngine::get_last_adaptation() {
            return;
        }
        self.myLastRouting = currentTime;
        // shortcut: MSRoutingEngine::reroute_2 panics until MSRouterProvider and MSTransportableRouter are ported
        MSRoutingEngine::reroute_2(holder, currentTime, "person-device.rerouting", onInit, false, &Default::default());
    }

    /// C++: `std::string MSTransportableDevice_Routing::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSTransportableDevice_Routing::getParameter(const std::string& key) const {
    //     if (key == "period") {
    //         return time2string(myPeriod);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> Result<String, ProcessError> {
        if key == "period" {
            return Ok(time2string(self.myPeriod));
        }
        Err(ProcessError::InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())))
    }

    /// C++: `void MSTransportableDevice_Routing::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSTransportableDevice_Routing::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    //     if (key == "period") {
    //         const SUMOTime oldPeriod = myPeriod;
    //         myPeriod = TIME2STEPS(doubleValue);
    //         if (myPeriod <= 0) {
    //             myRerouteCommand->deschedule();
    //         } else if (oldPeriod <= 0) {
    //             // re-schedule routing command
    //             MSNet::getInstance()->getInsertionEvents()->addEvent(new WrappingCommand<MSTransportableDevice_Routing>(this, &MSTransportableDevice_Routing::wrappedRerouteCommandExecute), SIMSTEP + myPeriod);
    //         }
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<(), ProcessError> {
        let doubleValue: f64 = match to_double(value) {
            Ok(v) => v,
            Err(_) => return Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name()))),
        };
        if key == "period" {
            let oldPeriod = self.myPeriod;
            self.myPeriod = time2steps(doubleValue);
            if self.myPeriod <= 0 {
                // shortcut: Command deschedule skipped (no Command type); upgrade when Command exists
            } else if oldPeriod <= 0 {
                // shortcut: re-scheduling the WrappingCommand insertion event is not ported; upgrade when Command and MSEventControl take commands
            }
        } else {
            return Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name())));
        }
        Ok(())
    }

    /// C++: `void MSTransportableDevice_Routing::MSTransportableDevice_Routing(MSTransportable &, const std::string &, SUMOTime)` [private]
    /// Constructor
    // MSTransportableDevice_Routing::MSTransportableDevice_Routing(MSTransportable& holder, const std::string& id, SUMOTime period)
    //     : MSTransportableDevice(holder, id), myPeriod(period), myLastRouting(-1), myRerouteCommand(0) {
    //     myScope = holder.getStringParam("person-device.rerouting.scope");
    //     // no need for initial routing here, personTrips trigger it themselves
    //     myRerouteCommand = new WrappingCommand<MSTransportableDevice_Routing>(this, &MSTransportableDevice_Routing::wrappedRerouteCommandExecute);
    //     MSNet::getInstance()->getInsertionEvents()->addEvent(myRerouteCommand, SIMSTEP + period);
    //     // the event will deschedule and destroy itself so it does not need to be stored
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str, period: SUMOTime) -> Self {
        // C++ reads the scope from the holder before the base device is built
        let myScope = holder.get_string_param("person-device.rerouting.scope", false, "");
        // shortcut: the WrappingCommand reroute event is not scheduled (no Command type); upgrade when Command and MSEventControl take commands
        MSTransportableDevice_Routing {
            base: MSTransportableDevice::new(holder, id),
            myPeriod: period,
            myLastRouting: -1,
            myRerouteCommand: Ptr::default(),
            myScope,
        }
    }

    /// C++: `SUMOTime MSTransportableDevice_Routing::wrappedRerouteCommandExecute(SUMOTime)` [private]
    /// Performs rerouting after a period
    // SUMOTime
    // MSTransportableDevice_Routing::wrappedRerouteCommandExecute(SUMOTime currentTime) {
    //     reroute(currentTime);
    //     return myPeriod;
    // }
    #[allow(unused_variables)]
    pub fn wrapped_reroute_command_execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        self.reroute(currentTime, false);
        self.myPeriod
    }

    /// C++: `void MSTransportableDevice_Routing::MSTransportableDevice_Routing(const MSTransportableDevice_Routing &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportableDevice_Routing) -> Self {
        panic!("MSTransportableDevice_Routing copy constructor is private and invalidated in C++")
    }

    /// C++: `MSTransportableDevice_Routing & MSTransportableDevice_Routing::operator=(const MSTransportableDevice_Routing &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_Routing) -> &mut MSTransportableDevice_Routing {
        panic!("MSTransportableDevice_Routing assignment operator is private and invalidated in C++")
    }

}

