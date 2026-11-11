//! Port of microsim/devices/MSDevice_Example.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Example` (microsim/devices/MSDevice_Example.h)
pub struct MSDevice_Example {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// a value which is initialised based on a commandline/configuration option
    pub myCustomValue1: f64,
    /// a value which is initialised based on a vehicle parameter
    pub myCustomValue2: f64,
    /// a value which is initialised based on a vType parameter
    pub myCustomValue3: f64,
}

impl std::ops::Deref for MSDevice_Example {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Example {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_Example {
    /// C++: `void MSDevice_Example::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Example-options
    // void
    // MSDevice_Example::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("Example Device");
    //     insertDefaultAssignmentOptions("example", "Example Device", oc);
    //
    //     oc.doRegister("device.example.parameter", new Option_Float(0.0));
    //     oc.addDescription("device.example.parameter", "Example Device", TL("An exemplary parameter which can be used by all instances of the example device"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_Example::insertOptions")
    }

    /// C++: `void MSDevice_Example::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_Example::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "example", v, false)) {
    //         // build the device
    //         // get custom vehicle parameter
    //         double customParameter2 = -1;
    //         if (v.getParameter().hasParameter("example")) {
    //             try {
    //                 customParameter2 = StringUtils::toDouble(v.getParameter().getParameter("example", "-1"));
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'example'"), v.getParameter().getParameter("example", "-1"));
    //             }
    //
    //         } else {
    //             std::cout << "vehicle '" << v.getID() << "' does not supply vehicle parameter 'example'. Using default of " << customParameter2 << "\n";
    //         }
    //         // get custom vType parameter
    //         double customParameter3 = -1;
    //         if (v.getVehicleType().getParameter().hasParameter("example")) {
    //             try {
    //                 customParameter3 = StringUtils::toDouble(v.getVehicleType().getParameter().getParameter("example", "-1"));
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'example'"), v.getVehicleType().getParameter().getParameter("example", "-1"));
    //             }
    //
    //         } else {
    //             std::cout << "vehicle '" << v.getID() << "' does not supply vType parameter 'example'. Using default of " << customParameter3 << "\n";
    //         }
    //         MSDevice_Example* device = new MSDevice_Example(v, "example_" + v.getID(),
    //                 oc.getFloat("device.example.parameter"),
    //                 customParameter2,
    //                 customParameter3);
    //         into.push_back(device);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_Example::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Example::cleanup()` [static]
    /// resets counters
    // void
    // MSDevice_Example::cleanup() {
    //     // cleaning up global state (if any)
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSDevice_Example::cleanup")
    }

    /// C++: `void MSDevice_Example::~MSDevice_Example() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Example::~MSDevice_Example() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Example::~MSDevice_Example")
    }

    /// C++: `bool MSDevice_Example::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks for waiting steps when the vehicle moves
    // bool
    // MSDevice_Example::notifyMove(SUMOTrafficObject& tObject, double /* oldPos */,
    //                              double /* newPos */, double newSpeed) {
    //     std::cout << "device '" << getID() << "' notifyMove: newSpeed=" << newSpeed << "\n";
    //     if (tObject.isVehicle()) {
    //         SUMOVehicle& veh = static_cast<SUMOVehicle&>(tObject);
    //         // check whether another device is present on the vehicle:
    //         MSDevice_Tripinfo* otherDevice = static_cast<MSDevice_Tripinfo*>(veh.getDevice(typeid(MSDevice_Tripinfo)));
    //         if (otherDevice != nullptr) {
    //             std::cout << "  veh '" << veh.getID() << " has device '" << otherDevice->getID() << "'\n";
    //         }
    //     }
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_Example::notifyMove")
    }

    /// C++: `bool MSDevice_Example::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Saves departure info on insertion
    // bool
    // MSDevice_Example::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     std::cout << "device '" << getID() << "' notifyEnter: reason=" << reason << " currentEdge=" << veh.getEdge()->getID() << "\n";
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Example::notifyEnter")
    }

    /// C++: `bool MSDevice_Example::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Saves arrival info
    // bool
    // MSDevice_Example::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     std::cout << "device '" << getID() << "' notifyLeave: reason=" << reason << " currentEdge=" << veh.getEdge()->getID() << "\n";
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Example::notifyLeave")
    }

    /// C++: `const std::string MSDevice_Example::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "example";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Example::deviceName")
    }

    /// C++: `std::string MSDevice_Example::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Example::getParameter(const std::string& key) const {
    //     if (key == "customValue1") {
    //         return toString(myCustomValue1);
    //     } else if (key == "customValue2") {
    //         return toString(myCustomValue2);
    //     } else if (key == "meaningOfLife") {
    //         return "42";
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_Example::getParameter")
    }

    /// C++: `void MSDevice_Example::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_Example::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    //     if (key == "customValue1") {
    //         myCustomValue1 = doubleValue;
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice_Example::setParameter")
    }

    /// C++: `void MSDevice_Example::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSDevice_Example::generateOutput(OutputDevice* tripinfoOut) const {
    //     if (tripinfoOut != nullptr) {
    //         tripinfoOut->openTag("example_device");
    //         tripinfoOut->writeAttr("customValue1", toString(myCustomValue1));
    //         tripinfoOut->writeAttr("customValue2", toString(myCustomValue2));
    //         tripinfoOut->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        todo!("port MSDevice_Example::generateOutput")
    }

    /// C++: `void MSDevice_Example::MSDevice_Example(SUMOVehicle &, const std::string &, double, double, double)` [private]
    /// Constructor
    // MSDevice_Example::MSDevice_Example(SUMOVehicle& holder, const std::string& id,
    //                                    double customValue1, double customValue2, double customValue3) :
    //     MSVehicleDevice(holder, id),
    //     myCustomValue1(customValue1),
    //     myCustomValue2(customValue2),
    //     myCustomValue3(customValue3) {
    //     std::cout << "initialized device '" << id << "' with myCustomValue1=" << myCustomValue1 << ", myCustomValue2=" << myCustomValue2 << ", myCustomValue3=" << myCustomValue3 << "\n";
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, customValue1: f64, customValue2: f64, customValue3: f64) -> Self {
        todo!("port MSDevice_Example::MSDevice_Example")
    }

    /// C++: `void MSDevice_Example::MSDevice_Example(const MSDevice_Example &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Example) -> Self {
        todo!("port MSDevice_Example::MSDevice_Example")
    }

    /// C++: `MSDevice_Example & MSDevice_Example::operator=(const MSDevice_Example &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Example) -> &mut MSDevice_Example {
        todo!("port MSDevice_Example::operator=")
    }

}

