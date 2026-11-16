//! Port of microsim/devices/MSDevice.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::RefCell;

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
#[repr(C)]
pub struct MSDevice {
    /// C++ base class `Named`
    pub base: Named,
    /// the value of the virtual `deviceName()` of the most derived class
    pub name: String,
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

pub const LOADSTATE_DEVICENAMES: &str = "LOADSTATE_DEVICENAMES";

thread_local! {
    /// C++ `MSDevice::myExplicitIDs`
    static MY_EXPLICIT_IDS: RefCell<BTreeMap<String, BTreeSet<String>>> = RefCell::new(BTreeMap::new());
    /// C++ `MSDevice::myDefaultAssignments[2]`; entries are leaked so `get_default_assignment` can return `&'static`
    static MY_DEFAULT_ASSIGNMENTS: RefCell<[BTreeMap<String, &'static MSDevice_DefaultAssignment>; 2]> = RefCell::new([BTreeMap::new(), BTreeMap::new()]);
    /// C++ `MSDevice::myEquipmentRNG` (seeded with "deviceEquipment")
    static MY_EQUIPMENT_RNG: Ptr<SumoRNG> = Ptr::from_raw(Box::into_raw(Box::new(SumoRNG::new("deviceEquipment"))));
}

/// C++ throws here; the port panics with the same message (same convention as OptionsCont's free functions)
fn or_throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{}", e))
}

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
        MSDevice_Routing::insert_options(oc);
        MSDevice_Emissions::insert_options(oc);
        MSVehicleDevice_BTreceiver::insert_options(oc);
        MSVehicleDevice_BTsender::insert_options(oc);
        MSDevice_Example::insert_options(oc);
        MSDevice_StationFinder::insert_options(oc);
        MSDevice_Battery::insert_options(oc);
        MSDevice_SSM::insert_options(oc);
        MSDevice_ToC::insert_options(oc);
        MSDevice_DriverState::insert_options(oc);
        MSDevice_Bluelight::insert_options(oc);
        MSDevice_FCD::insert_options(oc);
        MSDevice_ElecHybrid::insert_options(oc);
        MSDevice_Taxi::insert_options(oc);
        MSDevice_GLOSA::insert_options(oc);
        MSDevice_Tripinfo::insert_options(oc);
        MSDevice_Vehroutes::insert_options(oc);
        MSDevice_Friction::insert_options(oc);
        MSDevice_FCDReplay::insert_options(oc);

        MSTransportableDevice_Routing::insert_options(oc);
        MSTransportableDevice_FCD::insert_options(oc);
        MSTransportableDevice_BTsender::insert_options(oc);
        MSTransportableDevice_BTreceiver::insert_options(oc);
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
        let mut ok = true;
        ok &= MSDevice_Routing::check_options(oc);
        ok
    }

    /// C++: `void MSDevice::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    // C++ (MSDevice.cpp, real signature `std::vector<MSVehicleDevice*>& into`):
    // void
    // MSDevice::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     MSDevice_Vehroutes::buildVehicleDevices(v, into);
    //     MSDevice_Tripinfo::buildVehicleDevices(v, into);
    //     MSDevice_Routing::buildVehicleDevices(v, into);
    //     MSDevice_Emissions::buildVehicleDevices(v, into);
    //     MSVehicleDevice_BTreceiver::buildVehicleDevices(v, into);
    //     MSVehicleDevice_BTsender::buildVehicleDevices(v, into);
    //     MSDevice_Example::buildVehicleDevices(v, into);
    //     const size_t numBefore = into.size();
    //     MSDevice_StationFinder::buildVehicleDevices(v, into);
    //     MSDevice_Battery::buildVehicleDevices(v, into, into.size() == numBefore ? nullptr : static_cast<MSDevice_StationFinder*>(into.back()));
    //     MSDevice_SSM::buildVehicleDevices(v, into);
    //     MSDevice_ToC::buildVehicleDevices(v, into);
    //     MSDevice_DriverState::buildVehicleDevices(v, into);
    //     MSDevice_Bluelight::buildVehicleDevices(v, into);
    //     MSDevice_FCD::buildVehicleDevices(v, into);
    //     MSDevice_ElecHybrid::buildVehicleDevices(v, into);
    //     MSDevice_Taxi::buildVehicleDevices(v, into);
    //     MSDevice_GLOSA::buildVehicleDevices(v, into);
    //     MSDevice_Friction::buildVehicleDevices(v, into);
    //     MSDevice_FCDReplay::buildVehicleDevices(v, into);
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        MSDevice_Vehroutes::build_vehicle_devices(v, into, i32::MAX);
        MSDevice_Tripinfo::build_vehicle_devices(v, into);
        MSDevice_Routing::build_vehicle_devices(v, into);
        MSDevice_Emissions::build_vehicle_devices(v, into);
        MSVehicleDevice_BTreceiver::build_vehicle_devices(v, into);
        MSVehicleDevice_BTsender::build_vehicle_devices(v, into);
        MSDevice_Example::build_vehicle_devices(v, into);
        let num_before = into.len();
        MSDevice_StationFinder::build_vehicle_devices(v, into);
        let sf: Ptr<MSDevice_StationFinder> = if into.len() == num_before {
            Ptr::default()
        } else {
            Ptr::from_raw(into.last().copied().unwrap().as_raw() as *mut MSDevice_StationFinder)
        };
        MSDevice_Battery::build_vehicle_devices(v, into, sf);
        MSDevice_SSM::build_vehicle_devices(v, into);
        MSDevice_ToC::build_vehicle_devices(v, into);
        MSDevice_DriverState::build_vehicle_devices(v, into);
        MSDevice_Bluelight::build_vehicle_devices(v, into);
        MSDevice_FCD::build_vehicle_devices(v, into);
        MSDevice_ElecHybrid::build_vehicle_devices(v, into);
        MSDevice_Taxi::build_vehicle_devices(v, into);
        MSDevice_GLOSA::build_vehicle_devices(v, into);
        MSDevice_Friction::build_vehicle_devices(v, into);
        MSDevice_FCDReplay::build_vehicle_devices(v, into);
    }

    /// C++: `void MSDevice::buildTransportableDevices(MSTransportable &, int &)` [static]
    /// Build devices for the given person, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    // C++ (MSDevice.cpp, real signature `std::vector<MSTransportableDevice*>& into`):
    // void
    // MSDevice::buildTransportableDevices(MSTransportable& p, std::vector<MSTransportableDevice*>& into) {
    //     MSTransportableDevice_Routing::buildDevices(p, into);
    //     MSTransportableDevice_FCD::buildDevices(p, into);
    //     MSTransportableDevice_BTsender::buildDevices(p, into);
    //     MSTransportableDevice_BTreceiver::buildDevices(p, into);
    //     MSTransportableDevice_FCDReplay::buildDevices(p, into);
    // }
    #[allow(unused_variables)]
    pub fn build_transportable_devices(p: &mut MSTransportable, into: &mut Vec<Ptr<MSTransportableDevice>>) {
        MSTransportableDevice_Routing::build_devices(p, into);
        MSTransportableDevice_FCD::build_devices(p, into);
        MSTransportableDevice_BTsender::build_devices(p, into);
        MSTransportableDevice_BTreceiver::build_devices(p, into);
        MSTransportableDevice_FCDReplay::build_devices(p, into);
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
        if utils::common::string_utils::starts_with(id, "routing_") {
            // special case: renamed in output
            "rerouting".to_string()
        } else if utils::common::string_utils::starts_with(id, "driverstate") {
            // special case: no underscore
            "driverstate".to_string()
        } else {
            // std::string::find returns npos when there is no '_', and substr(0, npos) keeps the whole id
            match id.find('_') {
                Some(pos) => id[..pos].to_string(),
                None => id.to_string(),
            }
        }
    }

    /// C++: `SumoRNG * MSDevice::getEquipmentRNG()` [static]
    // static SumoRNG* getEquipmentRNG() {
    //         return &myEquipmentRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_equipment_rng() -> Ptr<SumoRNG> {
        MY_EQUIPMENT_RNG.with(|r| *r)
    }

    /// C++: `const std::string MSDevice::deviceName() const` [const, virtual, pure virtual]
    /// return the name for this type of device
    // (pure virtual: no C++ body; subclasses implement it)
    // Port note: the name is stored in `name` at construction (no vtable), so base-class calls see the subclass name.
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        self.name.clone()
    }

    /// C++: `MSDevice * MSVehicle::getDevice(typeid(T))` [type-keyed lookup, emulated by device kind]
    /// returns the first device of the given kind (its deviceName), or null
    // shortcut: matches the device kind by deviceName instead of typeid, upgrade when devices carry a type tag
    pub fn get_device_by_name(devices: &[Ptr<MSVehicleDevice>], name: &str) -> Ptr<MSVehicleDevice> {
        for dev in devices.iter() {
            if dev.base2.device_name() == name {
                return *dev;
            }
        }
        Ptr::default()
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
        MY_DEFAULT_ASSIGNMENTS.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache[0].clear();
            cache[1].clear();
        });
        MSRoutingEngine::cleanup();
        MSDevice_Tripinfo::cleanup();
        MSDevice_FCD::cleanup();
        MSDevice_Taxi::cleanup();
    }

    /// C++: `void MSDevice::MSDevice(const std::string &)`
    /// Constructor
    // MSDevice(const std::string& id) : Named(id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        // default name from the id prefix; subclasses whose id prefix differs from deviceName() overwrite it
        MSDevice { base: Named::new(id), name: Self::get_device_name(id) }
    }

    /// C++: `void MSDevice::~MSDevice() noexcept` [virtual]
    /// Destructor
    // virtual ~MSDevice() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void MSDevice::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on vehicle deletion to extend tripinfo and other outputs
    // virtual void generateOutput(OutputDevice* /*tripinfoOut*/) const {
    //     }
    #[allow(unused_variables)]
    pub fn generate_output(&self, arg0: Ptr<OutputDevice>) {
        // C++ body is empty
    }

    /// C++: `void MSDevice::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice::saveState(OutputDevice& /* out */) const {
    //     WRITE_WARNINGF(TL("Device '%' cannot save state"), getID());
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        write_warningf!("Device '%' cannot save state", self.get_id());
    }

    /// C++: `void MSDevice::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice::loadState(const SUMOSAXAttributes& /* attrs */) {
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty
    }

    /// C++: `std::string MSDevice::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // virtual std::string getParameter(const std::string& key) const {
    //         throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // shortcut: C++ throws InvalidArgument; the port panics with the same text because the overrides return String, upgrade when device getParameter returns Result
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        panic!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())
    }

    /// C++: `void MSDevice::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // virtual void setParameter(const std::string& key, const std::string& value) {
    //         UNUSED_PARAMETER(value);
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // shortcut: C++ throws InvalidArgument; the port panics with the same text because the overrides return (), upgrade when device setParameter returns Result
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        panic!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name())
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
        let prefix = format!("{}{}", if isPerson { "person-device." } else { "device." }, deviceName);
        let object = if isPerson { "person" } else { "vehicle" };
        oc.do_register(&format!("{}.probability", prefix), SumoOption::float(-1.0)); // (default: no need to call RNG)
        oc.add_description(&format!("{}.probability", prefix), optionsTopic, &format!("The probability for a {} to have a '{}' device", object, deviceName));

        oc.do_register(&format!("{}.explicit", prefix), SumoOption::string_vector());
        oc.add_synonyme(&format!("{}.explicit", prefix), &format!("{}.knownveh", prefix), true);
        oc.add_description(&format!("{}.explicit", prefix), optionsTopic, &format!("Assign a '{}' device to named {}s", deviceName, object));

        oc.do_register(&format!("{}.deterministic", prefix), SumoOption::bool(false));
        oc.add_description(&format!("{}.deterministic", prefix), optionsTopic, &format!("The '{}' devices are set deterministic using a fraction of 1000", deviceName));
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
    // Port note: the template holder DEVICEHOLDER is replaced by its three inputs, because
    // utils SUMOVehicle has no getParameter()/getVehicleType() in Rust. Callers pass
    // (holder id, holder parameter, holder vehicle-type parameter).
    #[allow(unused_variables)]
    pub fn equipped_by_default_assignment_options(
        oc: &OptionsCont,
        deviceName: &str,
        holderID: &str,
        holderParameter: &utils::common::parameterised::Parameterised,
        vtypeParameter: &utils::common::parameterised::Parameterised,
        outputOptionSet: bool,
        isPerson: bool,
    ) -> bool {
        let defaults = Self::get_default_assignment(oc, deviceName, isPerson);
        let prefix = &defaults.prefix;
        // assignment by number
        let mut numberGiven = defaults.numberGiven;
        let mut probability = defaults.probability;
        // assignment by name
        let mut haveByName = false;
        let nameGiven = defaults.nameGiven;
        if nameGiven {
            if !MY_EXPLICIT_IDS.with(|m| m.borrow().contains_key(deviceName)) {
                // C++ reads the global options here
                let idList = utils::options::options_cont::get_string_vector(&format!("{}.explicit", prefix));
                let ids: BTreeSet<String> = idList.into_iter().collect();
                MY_EXPLICIT_IDS.with(|m| m.borrow_mut().insert(deviceName.to_string(), ids));
            }
            haveByName = MY_EXPLICIT_IDS.with(|m| m.borrow().get(deviceName).map_or(false, |s| s.contains(holderID)));
        }
        // assignment by abstract parameters
        let mut haveByParameter = false;
        let mut parameterGiven = false;
        let key = &defaults.parameterKey;
        if holderParameter.has_parameter(key) {
            parameterGiven = true;
            haveByParameter = or_throw(utils::common::string_utils::to_bool(&holderParameter.get_parameter(key, "false")));
        } else if vtypeParameter.has_parameter(key) {
            parameterGiven = true;
            haveByParameter = or_throw(utils::common::string_utils::to_bool(&vtypeParameter.get_parameter(key, "false")));
        } else if vtypeParameter.has_parameter(&defaults.probabilityKey) {
            // override global options
            numberGiven = true;
            probability = or_throw(utils::common::string_utils::to_double(&vtypeParameter.get_parameter(&defaults.probabilityKey, "0")));
        }
        if haveByName {
            return true;
        } else if parameterGiven {
            return haveByParameter;
        } else if numberGiven {
            if defaults.deterministic {
                return MSNet::get_instance().m().get_vehicle_control().get_quota(probability, -1) == 1;
            } else if probability > 0. {
                if holderParameter.has_parameter(LOADSTATE_DEVICENAMES) {
                    // replicate probabilistic assignment
                    let mut tokenizer = StringTokenizer::new(&holderParameter.get_parameter(LOADSTATE_DEVICENAMES, ""));
                    let lsdn = tokenizer.get_vector();
                    return lsdn.iter().any(|s| s.as_str() == deviceName);
                } else {
                    return MY_EQUIPMENT_RNG.with(|r| *r).m().rand() < probability;
                }
            } else {
                return false;
            }
        } else {
            return !nameGiven && outputOptionSet;
        }
    }

    /// C++: `const DefaultAssignment & MSDevice::getDefaultAssignment(const OptionsCont &, const std::string &, const bool)` [static, private]
    // const MSDevice::DefaultAssignment&
    // MSDevice::getDefaultAssignment(const OptionsCont& oc, const std::string& deviceName, const bool isPerson) {
    //     // the options do not change during a run, the cache is cleared in cleanupAll
    //     std::map<std::string, DefaultAssignment>& cache = myDefaultAssignments[isPerson ? 1 : 0];
    //     auto it = cache.find(deviceName);
    //     if (it == cache.end()) {
    //         DefaultAssignment d;
    //         d.prefix = (isPerson ? "person-device." : "device.") + deviceName;
    //         d.parameterKey = "has." + deviceName + ".device";
    //         d.probabilityKey = d.prefix + ".probability";
    //         d.deterministic = oc.exists(d.prefix + ".deterministic") && oc.getBool(d.prefix + ".deterministic");
    //         d.numberGiven = d.deterministic || (oc.exists(d.probabilityKey) && oc.getFloat(d.probabilityKey) >= 0.);
    //         d.probability = d.numberGiven ? oc.getFloat(d.probabilityKey) : 0;
    //         d.nameGiven = oc.exists(d.prefix + ".explicit") && oc.isSet(d.prefix + ".explicit");
    //         it = cache.emplace(deviceName, d).first;
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn get_default_assignment(oc: &OptionsCont, deviceName: &str, isPerson: bool) -> &'static MSDevice_DefaultAssignment {
        // the options do not change during a run, the cache is cleared in cleanupAll
        let slot = if isPerson { 1 } else { 0 };
        if let Some(d) = MY_DEFAULT_ASSIGNMENTS.with(|cache| cache.borrow()[slot].get(deviceName).copied()) {
            return d;
        }
        let prefix = format!("{}{}", if isPerson { "person-device." } else { "device." }, deviceName);
        let probabilityKey = format!("{}.probability", prefix);
        let deterministicKey = format!("{}.deterministic", prefix);
        let explicitKey = format!("{}.explicit", prefix);
        let deterministic = oc.exists(&deterministicKey) && or_throw(oc.get_bool(&deterministicKey));
        let numberGiven = deterministic || (oc.exists(&probabilityKey) && or_throw(oc.get_float(&probabilityKey)) >= 0.);
        let probability = if numberGiven { or_throw(oc.get_float(&probabilityKey)) } else { 0. };
        let nameGiven = oc.exists(&explicitKey) && or_throw(oc.is_set(&explicitKey, true));
        let d: &'static MSDevice_DefaultAssignment = Box::leak(Box::new(MSDevice_DefaultAssignment {
            prefix: prefix.clone(),
            parameterKey: format!("has.{}.device", deviceName),
            probabilityKey,
            deterministic,
            numberGiven,
            probability,
            nameGiven,
        }));
        MY_DEFAULT_ASSIGNMENTS.with(|cache| cache.borrow_mut()[slot].insert(deviceName.to_string(), d));
        d
    }

    /// C++: `void MSDevice::MSDevice(const MSDevice &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    // C++ declares it private and never defines it, so no caller can exist.
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice) -> Self {
        unreachable!("MSDevice copy constructor is private and undefined in C++")
    }

    /// C++: `MSDevice & MSDevice::operator=(const MSDevice &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    // C++ declares it private and never defines it, so no caller can exist.
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice) -> &mut MSDevice {
        unreachable!("MSDevice assignment operator is private and undefined in C++")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vtype_param_probability_equips_rerouting_device() {
        // global option left disabled (-1), so only the vType <param> can equip the device
        let mut oc = utils::options::options_cont::OptionsCont::new();
        oc.do_register("device.rerouting.probability", SumoOption::float(-1.));
        let holder = utils::common::parameterised::Parameterised::new();
        let mut vtype = utils::common::parameterised::Parameterised::new();
        assert!(!MSDevice::equipped_by_default_assignment_options(&oc, "rerouting", "v1", &holder, &vtype, false, false));
        vtype.set_parameter("device.rerouting.probability", "1");
        assert!(MSDevice::equipped_by_default_assignment_options(&oc, "rerouting", "v1", &holder, &vtype, false, false));
    }

    #[test]
    fn device_name_from_id_matches_cpp_get_device_name() {
        assert_eq!(MSDevice::new("routing_v1").device_name(), "rerouting");
        assert_eq!(MSDevice::new("driverstate7").device_name(), "driverstate");
        assert_eq!(MSDevice::new("battery_v1").device_name(), "battery");
    }
}

