//! Port of microsim/devices/MSDevice_Example.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::to_double;
use utils::common::to_string::{f64_to_string_default, to_string};
use utils::options::options_cont::get_float;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Example` (microsim/devices/MSDevice_Example.h)
#[repr(C)]
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
        oc.add_option_sub_topic("Example Device");
        MSDevice::insert_default_assignment_options("example", "Example Device", oc, false);

        oc.do_register("device.example.parameter", SumoOption::float(0.0));
        oc.add_description("device.example.parameter", "Example Device", "An exemplary parameter which can be used by all instances of the example device");
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
        // shortcut: MSDevice::equippedByDefaultAssignmentOptions is declared in ms_device.rs without its (oc, "example", v, false) parameters, so the call cannot take the C++ arguments yet; upgrade when ms_device.rs gets the C++ signature (this call will then fail to compile and must be updated)
        if { panic!("shortcut: MSDevice::equippedByDefaultAssignmentOptions needs the holder vehicle parameters, which SUMOVehicle does not expose yet") } {
            // get custom vehicle parameter
            let mut customParameter2 = -1.0;
            if v.get_parameter().has_parameter("example") {
                let value = v.get_parameter().get_parameter("example", "-1");
                match to_double(&value) {
                    Ok(d) => customParameter2 = d,
                    Err(_) => utils::write_warningf!("Invalid value '%' for vehicle parameter 'example'", value),
                }
            } else {
                println!("vehicle '{}' does not supply vehicle parameter 'example'. Using default of {}", v.get_id(), f64_to_string_default(customParameter2));
            }
            // get custom vType parameter
            let mut customParameter3 = -1.0;
            if v.get_vehicle_type().get_parameter().has_parameter("example") {
                let value = v.get_vehicle_type().get_parameter().get_parameter("example", "-1");
                match to_double(&value) {
                    Ok(d) => customParameter3 = d,
                    Err(_) => utils::write_warningf!("Invalid value '%' for vType parameter 'example'", value),
                }
            } else {
                println!("vehicle '{}' does not supply vType parameter 'example'. Using default of {}", v.get_id(), f64_to_string_default(customParameter3));
            }
            let id = format!("example_{}", v.get_id());
            let customValue1 = get_float("device.example.parameter");
            // the device is owned by the vehicle's device list for the rest of the run, as in C++
            let device: &mut MSDevice_Example = Box::leak(Box::new(MSDevice_Example::new(v, &id, customValue1, customParameter2, customParameter3)));
            into.push(Ptr::from_mut(&mut device.base));
        }
    }

    /// C++: `void MSDevice_Example::cleanup()` [static]
    /// resets counters
    // void
    // MSDevice_Example::cleanup() {
    //     // cleaning up global state (if any)
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        // C++ cleanup has no global state to reset

    }

    /// C++: `void MSDevice_Example::~MSDevice_Example() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Example::~MSDevice_Example() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty

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
        println!("device '{}' notifyMove: newSpeed={}", self.base.base2.get_id(), f64_to_string_default(newSpeed));
        // shortcut: the C++ isVehicle() branch needs a static_cast to SUMOVehicle and MSDevice_Tripinfo lookup via getDevice(typeid(...)), which takes an Unmapped std::type_info, so only the speed line is printed; upgrade when SUMOTrafficObject can be downcast and getDevice takes a device name
        true // keep the device
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
        println!("device '{}' notifyEnter: reason={} currentEdge={}", self.base.base2.get_id(), reason as i32, veh.get_edge().get_id());
        true // keep the device
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
        println!("device '{}' notifyLeave: reason={} currentEdge={}", self.base.base2.get_id(), reason as i32, veh.get_edge().get_id());
        true // keep the device
    }

    /// C++: `const std::string MSDevice_Example::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "example";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "example".to_string()
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
        if key == "customValue1" {
            to_string(&self.myCustomValue1)
        } else if key == "customValue2" {
            to_string(&self.myCustomValue2)
        } else if key == "meaningOfLife" {
            "42".to_string()
        } else {
            // shortcut: C++ throws InvalidArgument; this virtual returns String, so the same message is raised as a panic
            panic!("{}", InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())).0)
        }
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
        // shortcut: C++ throws InvalidArgument; this virtual returns (), so the same messages are raised as panics
        let doubleValue = match to_double(value) {
            Ok(d) => d,
            Err(_) => panic!("{}", InvalidArgument(format!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name())).0),
        };
        if key == "customValue1" {
            self.myCustomValue1 = doubleValue;
        } else {
            panic!("{}", InvalidArgument(format!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name())).0);
        }
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
        if !tripinfoOut.is_null() {
            let out = tripinfoOut.m();
            out.open_tag("example_device");
            out.write_attr("customValue1", to_string(&self.myCustomValue1).as_str());
            out.write_attr("customValue2", to_string(&self.myCustomValue2).as_str());
            out.close_tag("");
        }
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
        println!("initialized device '{}' with myCustomValue1={}, myCustomValue2={}, myCustomValue3={}", id, f64_to_string_default(customValue1), f64_to_string_default(customValue2), f64_to_string_default(customValue3));
        MSDevice_Example {
            base: MSVehicleDevice::new(holder, id),
            myCustomValue1: customValue1,
            myCustomValue2: customValue2,
            myCustomValue3: customValue3,
        }
    }

    /// C++: `void MSDevice_Example::MSDevice_Example(const MSDevice_Example &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Example) -> Self {
        // the C++ copy constructor is private and has no definition, so it is never called
        unreachable!("MSDevice_Example copy constructor is private and undefined in C++")
    }

    /// C++: `MSDevice_Example & MSDevice_Example::operator=(const MSDevice_Example &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Example) -> &mut MSDevice_Example {
        // the C++ assignment operator is private and has no definition, so it is never called
        unreachable!("MSDevice_Example assignment operator is private and undefined in C++")
    }

}

