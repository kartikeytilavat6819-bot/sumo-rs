//! Port of microsim/devices/MSDevice.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// the option values of a device's default assignment (all callers pass the global options, which do not change during a run)
/// C++ class `MSDevice::DefaultAssignment` (microsim/devices/MSDevice.h)
pub struct MSDevice_DefaultAssignment {
    pub prefix: String,
    pub parameterKey: String,
    pub probabilityKey: String,
    pub deterministic: bool,
    pub numberGiven: bool,
    pub probability: f64,
    pub nameGiven: bool,
}

impl MSDevice_DefaultAssignment {
}

/// Abstract in-vehicle / in-person device
/// C++ class `MSDevice` (microsim/devices/MSDevice.h)
pub struct MSDevice {
    /// C++ base class `Named`
    pub base: Named,
}

impl std::ops::Deref for MSDevice {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MSDevice (port as module-level statics, see AGENTS.md):
//   static const std::string MSDevice::LOADSTATE_DEVICENAMES;
//   static int MSDevice::myExplicitIDs;
//   static int[2] MSDevice::myDefaultAssignments;
//   static SumoRNG MSDevice::myEquipmentRNG;

impl MSDevice {
    /// C++: `void MSDevice::insertOptions(OptionsCont &)` [static]
    /// Inserts options for building devices
    // void
    // MSDevice::insertOptions(OptionsCont& oc) {
    //     MSDevice_Routing::insertOptions(oc);
    //     MSDevice_Emissions::insertOptions(oc);
    //     MSVehicleDevice_BTreceiver::insertOptions(oc);
    //     MSVehicleDevice_BTsender::insertOptions(oc);
    //     MSDevice_Example::insertOptions(oc);
    //     MSDevice_StationFinder::insertOptions(oc);
    //     MSDevice_Battery::insertOptions(oc);
    //     MSDevice_SSM::insertOptions(oc);
    //     MSDevice_ToC::insertOptions(oc);
    //     MSDevice_DriverState::insertOptions(oc);
    //     MSDevice_Bluelight::insertOptions(oc);
    //     MSDevice_FCD::insertOptions(oc);
    //     MSDevice_ElecHybrid::insertOptions(oc);
    //     MSDevice_Taxi::insertOptions(oc);
    //     MSDevice_GLOSA::insertOptions(oc);
    //     MSDevice_Tripinfo::insertOptions(oc);
    //     MSDevice_Vehroutes::insertOptions(oc);
    //     MSDevice_Friction::insertOptions(oc);
    //     MSDevice_FCDReplay::insertOptions(oc);
    //
    //     MSTransportableDevice_Routing::insertOptions(oc);
    //     MSTransportableDevice_FCD::insertOptions(oc);
    //     MSTransportableDevice_BTsender::insertOptions(oc);
    //     MSTransportableDevice_BTreceiver::insertOptions(oc);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice::insertOptions")
    }

    /// C++: `bool MSDevice::checkOptions(OptionsCont &)` [static]
    /// check device-specific options
    // bool
    // MSDevice::checkOptions(OptionsCont& oc) {
    //     bool ok = true;
    //     ok &= MSDevice_Routing::checkOptions(oc);
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        todo!("port MSDevice::checkOptions")
    }

    /// C++: `void MSDevice::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut i32) {
        todo!("port MSDevice::buildVehicleDevices")
    }

    /// C++: `void MSDevice::buildTransportableDevices(MSTransportable &, int &)` [static]
    /// Build devices for the given person, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_transportable_devices(p: &mut MSTransportable, into: &mut i32) {
        todo!("port MSDevice::buildTransportableDevices")
    }

    /// C++: `std::string MSDevice::getDeviceName(const std::string &)` [static]
    /// extracts the deviceName from the id (which includes holder id) and is subject to special cases
    // std::string
    // MSDevice::getDeviceName(const std::string& id) {
    //     if (StringUtils::startsWith(id, "routing_")) {
    //         // special case: renamed in output
    //         return "rerouting";
    //     } else if (StringUtils::startsWith(id, "driverstate")) {
    //         // special case: no underscore
    //         return "driverstate";
    //     } else {
    //         return id.substr(0, id.find('_'));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_device_name(id: &str) -> String {
        todo!("port MSDevice::getDeviceName")
    }

    /// C++: `SumoRNG * MSDevice::getEquipmentRNG()` [static]
    // static SumoRNG* getEquipmentRNG() {
    //         return &myEquipmentRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_equipment_rng() -> Ptr<SumoRNG> {
        todo!("port MSDevice::getEquipmentRNG")
    }

    /// C++: `const std::string MSDevice::deviceName() const` [const, virtual, pure virtual]
    /// return the name for this type of device
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice::deviceName")
    }

    /// C++: `void MSDevice::cleanupAll()` [static]
    /// perform cleanup for all devices
    // void
    // MSDevice::cleanupAll() {
    //     myDefaultAssignments[0].clear();
    //     myDefaultAssignments[1].clear();
    //     MSRoutingEngine::cleanup();
    //     MSDevice_Tripinfo::cleanup();
    //     MSDevice_FCD::cleanup();
    //     MSDevice_Taxi::cleanup();
    // }
    #[allow(unused_variables)]
    pub fn cleanup_all() {
        todo!("port MSDevice::cleanupAll")
    }

    /// C++: `void MSDevice::MSDevice(const std::string &)`
    /// Constructor
    // MSDevice(const std::string& id) : Named(id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        todo!("port MSDevice::MSDevice")
    }

    /// C++: `void MSDevice::~MSDevice() noexcept` [virtual]
    /// Destructor
    // virtual ~MSDevice() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice::~MSDevice")
    }

    /// C++: `void MSDevice::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on vehicle deletion to extend tripinfo and other outputs
    // virtual void generateOutput(OutputDevice* /*tripinfoOut*/) const {
    //     }
    #[allow(unused_variables)]
    pub fn generate_output(&self, arg0: Ptr<OutputDevice>) {
        todo!("port MSDevice::generateOutput")
    }

    /// C++: `void MSDevice::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice::saveState(OutputDevice& /* out */) const {
    //     WRITE_WARNINGF(TL("Device '%' cannot save state"), getID());
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        todo!("port MSDevice::saveState")
    }

    /// C++: `void MSDevice::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice::loadState(const SUMOSAXAttributes& /* attrs */) {
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port MSDevice::loadState")
    }

    /// C++: `std::string MSDevice::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // virtual std::string getParameter(const std::string& key) const {
    //         throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice::getParameter")
    }

    /// C++: `void MSDevice::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // virtual void setParameter(const std::string& key, const std::string& value) {
    //         UNUSED_PARAMETER(value);
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice::setParameter")
    }

    /// C++: `void MSDevice::insertDefaultAssignmentOptions(const std::string &, const std::string &, OptionsCont &, const bool)` [static, protected]
    /// Adds common command options that allow to assign devices to vehicles
    // void
    // MSDevice::insertDefaultAssignmentOptions(const std::string& deviceName, const std::string& optionsTopic, OptionsCont& oc, const bool isPerson) {
    //     const std::string prefix = (isPerson ? "person-device." : "device.") + deviceName;
    //     const std::string object = isPerson ? "person" : "vehicle";
    //     oc.doRegister(prefix + ".probability", new Option_Float(-1.0));// (default: no need to call RNG)
    //     oc.addDescription(prefix + ".probability", optionsTopic, "The probability for a " + object + " to have a '" + deviceName + "' device");
    //
    //     oc.doRegister(prefix + ".explicit", new Option_StringVector());
    //     oc.addSynonyme(prefix + ".explicit", prefix + ".knownveh", true);
    //     oc.addDescription(prefix + ".explicit", optionsTopic, "Assign a '" + deviceName + "' device to named " + object + "s");
    //
    //     oc.doRegister(prefix + ".deterministic", new Option_Bool(false));
    //     oc.addDescription(prefix + ".deterministic", optionsTopic, "The '" + deviceName + "' devices are set deterministic using a fraction of 1000");
    // }
    #[allow(unused_variables)]
    pub fn insert_default_assignment_options(deviceName: &str, optionsTopic: &str, oc: &mut OptionsCont, isPerson: bool) {
        todo!("port MSDevice::insertDefaultAssignmentOptions")
    }

    /// C++: `bool MSDevice::equippedByDefaultAssignmentOptions(const OptionsCont &, const std::string &, DEVICEHOLDER &, bool, const bool)` [static, protected]
    /// Determines whether a vehicle should get a certain device
    // template<class DEVICEHOLDER> bool
    // MSDevice::equippedByDefaultAssignmentOptions(const OptionsCont& oc, const std::string& deviceName, DEVICEHOLDER& v, bool outputOptionSet, const bool isPerson) {
    //     const DefaultAssignment& defaults = getDefaultAssignment(oc, deviceName, isPerson);
    //     const std::string& prefix = defaults.prefix;
    //     // assignment by number
    //     bool numberGiven = defaults.numberGiven;
    //     double probability = defaults.probability;
    //     // assignment by name
    //     bool haveByName = false;
    //     const bool nameGiven = defaults.nameGiven;
    //     if (nameGiven) {
    //         if (myExplicitIDs.find(deviceName) == myExplicitIDs.end()) {
    //             myExplicitIDs[deviceName] = std::set<std::string>();
    //             const std::vector<std::string> idList = OptionsCont::getOptions().getStringVector(prefix + ".explicit");
    //             myExplicitIDs[deviceName].insert(idList.begin(), idList.end());
    //         }
    //         haveByName = myExplicitIDs[deviceName].count(v.getID()) > 0;
    //     }
    //     // assignment by abstract parameters
    //     bool haveByParameter = false;
    //     bool parameterGiven = false;
    //     const std::string& key = defaults.parameterKey;
    //     if (v.getParameter().hasParameter(key)) {
    //         parameterGiven = true;
    //         haveByParameter = StringUtils::toBool(v.getParameter().getParameter(key, "false"));
    //     } else if (v.getVehicleType().getParameter().hasParameter(key)) {
    //         parameterGiven = true;
    //         haveByParameter = StringUtils::toBool(v.getVehicleType().getParameter().getParameter(key, "false"));
    //     } else if (v.getVehicleType().getParameter().hasParameter(defaults.probabilityKey)) {
    //         // override global options
    //         numberGiven = true;
    //         probability = StringUtils::toDouble(v.getVehicleType().getParameter().getParameter(defaults.probabilityKey, "0"));
    //     }
    //     //std::cout << " deviceName=" << deviceName << " holder=" << v.getID()
    //     //    << " nameGiven=" << nameGiven << " haveByName=" << haveByName
    //     //    << " parameterGiven=" << parameterGiven << " haveByParameter=" << haveByParameter
    //     //    << " numberGiven=" << numberGiven << " haveByNumber=" << haveByNumber
    //     //    << " outputOptionSet=" << outputOptionSet << "\n";
    //     if (haveByName) {
    //         return true;
    //     } else if (parameterGiven) {
    //         return haveByParameter;
    //     } else if (numberGiven) {
    //         if (defaults.deterministic) {
    //             return MSNet::getInstance()->getVehicleControl().getQuota(probability) == 1;
    //         } else if (probability > 0) {
    //             if (v.getParameter().hasParameter(LOADSTATE_DEVICENAMES)) {
    //                 // replicate probabilistic assignment
    //                 const std::vector<std::string> lsdn = StringTokenizer(v.getParameter().getParameter(LOADSTATE_DEVICENAMES)).getVector();
    //                 return std::find(lsdn.begin(), lsdn.end(), deviceName) != lsdn.end();
    //             } else {
    //                 return RandHelper::rand(&myEquipmentRNG) < probability;
    //             }
    //         } else {
    //             return false;
    //         }
    //     } else {
    //         return !nameGiven && outputOptionSet;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn equipped_by_default_assignment_options() -> bool {
        todo!("port MSDevice::equippedByDefaultAssignmentOptions")
    }

    /// C++: `const DefaultAssignment & MSDevice::getDefaultAssignment(const OptionsCont &, const std::string &, const bool)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_default_assignment(oc: &OptionsCont, deviceName: &str, isPerson: bool) -> &'static MSDevice_DefaultAssignment {
        todo!("port MSDevice::getDefaultAssignment")
    }

    /// C++: `void MSDevice::MSDevice(const MSDevice &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice) -> Self {
        todo!("port MSDevice::MSDevice")
    }

    /// C++: `MSDevice & MSDevice::operator=(const MSDevice &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice) -> &mut MSDevice {
        todo!("port MSDevice::operator=")
    }

}

