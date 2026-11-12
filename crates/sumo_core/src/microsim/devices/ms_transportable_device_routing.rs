//! Port of microsim/devices/MSTransportableDevice_Routing.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSTransportableDevice_Routing::insertOptions")
    }

    /// C++: `bool MSTransportableDevice_Routing::checkOptions(OptionsCont &)` [static]
    /// checks MSTransportableDevice_Routing-options
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        todo!("port MSTransportableDevice_Routing::checkOptions")
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
        todo!("port MSTransportableDevice_Routing::buildDevices")
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
        todo!("port MSTransportableDevice_Routing::~MSTransportableDevice_Routing")
    }

    /// C++: `const std::string MSTransportableDevice_Routing::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "rerouting";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSTransportableDevice_Routing::deviceName")
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
        todo!("port MSTransportableDevice_Routing::saveState")
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
        todo!("port MSTransportableDevice_Routing::loadState")
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
        todo!("port MSTransportableDevice_Routing::reroute")
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
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSTransportableDevice_Routing::getParameter")
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
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSTransportableDevice_Routing::setParameter")
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
        todo!("port MSTransportableDevice_Routing::MSTransportableDevice_Routing")
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
        todo!("port MSTransportableDevice_Routing::wrappedRerouteCommandExecute")
    }

    /// C++: `void MSTransportableDevice_Routing::MSTransportableDevice_Routing(const MSTransportableDevice_Routing &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportableDevice_Routing) -> Self {
        todo!("port MSTransportableDevice_Routing::MSTransportableDevice_Routing")
    }

    /// C++: `MSTransportableDevice_Routing & MSTransportableDevice_Routing::operator=(const MSTransportableDevice_Routing &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_Routing) -> &mut MSTransportableDevice_Routing {
        todo!("port MSTransportableDevice_Routing::operator=")
    }

}

