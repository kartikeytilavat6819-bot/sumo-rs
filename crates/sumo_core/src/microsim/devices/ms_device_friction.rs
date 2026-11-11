//! Port of microsim/devices/MSDevice_Friction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on current friction Coefficient on the road
/// C++ class `MSDevice_Friction` (microsim/devices/MSDevice_Friction.h)
pub struct MSDevice_Friction {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// a value which is initialised based on a commandline/configuration option
    pub myMeasuredFrictionCoefficient: f64,
    /// realValue from Road (without measurement model)
    pub myRawFriction: f64,
    /// a value which is initialised based on a vehicle parameter
    pub myStdDeviation: f64,
    /// a value which is initialised based on a vType parameter
    pub myOffset: f64,
}

impl std::ops::Deref for MSDevice_Friction {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Friction {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_Friction {
    /// C++: `void MSDevice_Friction::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Friction-options
    // void
    // MSDevice_Friction::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("Friction Device");
    //     insertDefaultAssignmentOptions("friction", "Friction Device", oc);
    //     oc.doRegister("device.friction.stdDev", new Option_Float(.1)); //default .1
    //     oc.addDescription("device.friction.stdDev", "Friction Device", TL("The measurement noise parameter which can be applied to the friction device"));
    //     oc.doRegister("device.friction.offset", new Option_Float(0.)); //default no offset
    //     oc.addDescription("device.friction.offset", "Friction Device", TL("The measurement offset parameter which can be applied to the friction device -> e.g. to force false measurements"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_Friction::insertOptions")
    }

    /// C++: `void MSDevice_Friction::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_Friction::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "friction", v, false)) {
    //         // build the device
    //         MSDevice_Friction* device = new MSDevice_Friction(v, "friction_" + v.getID(),
    //                 v.getFloatParam("device.friction.stdDev"), // stdDev noise deviation
    //                 v.getFloatParam("device.friction.offset")); // static offset
    //         into.push_back(device);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_Friction::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Friction::~MSDevice_Friction() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Friction::~MSDevice_Friction() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Friction::~MSDevice_Friction")
    }

    /// C++: `bool MSDevice_Friction::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks for waiting steps when the vehicle moves
    // bool
    // MSDevice_Friction::notifyMove(SUMOTrafficObject& /* tObject */, double /* oldPos */,
    //                               double /* newPos */, double /* newSpeed */) {
    //     myRawFriction = myHolder.getLane()->getFrictionCoefficient();
    //     myMeasuredFrictionCoefficient = myOffset + RandHelper::randNorm(myRawFriction, myStdDeviation, myHolder.getRNG());
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_Friction::notifyMove")
    }

    /// C++: `const std::string MSDevice_Friction::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "friction";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Friction::deviceName")
    }

    /// C++: `std::string MSDevice_Friction::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Friction::getParameter(const std::string& key) const {
    //     if (key == "frictionCoefficient") {
    //         return toString(myMeasuredFrictionCoefficient);
    //     } else if (key == "stdDev") {
    //         return toString(myStdDeviation);
    //     } else if (key == "offset") {
    //         return toString(myOffset);
    //     } else if (key == "rawFriction") {
    //         return toString(myRawFriction);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_Friction::getParameter")
    }

    /// C++: `void MSDevice_Friction::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_Friction::setParameter(const std::string& key, const std::string& value) {
    //     try {
    //         const double doubleValue = StringUtils::toDouble(value);
    //         if (key == "frictionCoefficient") {
    //             myMeasuredFrictionCoefficient = doubleValue;
    //         } else if (key == "stdDev") {
    //             myStdDeviation = doubleValue;
    //         } else if (key == "offset") {
    //             myOffset = doubleValue;
    //         } else {
    //             throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //         }
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice_Friction::setParameter")
    }

    /// C++: `double MSDevice_Friction::getMeasuredFriction()`
    // inline double getMeasuredFriction() {
    //         return myMeasuredFrictionCoefficient;
    //     }
    #[allow(unused_variables)]
    pub fn get_measured_friction(&mut self) -> f64 {
        todo!("port MSDevice_Friction::getMeasuredFriction")
    }

    /// C++: `void MSDevice_Friction::MSDevice_Friction(SUMOVehicle &, const std::string &, double, double)` [private]
    /// Constructor
    // MSDevice_Friction::MSDevice_Friction(SUMOVehicle& holder, const std::string& id, double stdDev, double offset) :
    //     MSVehicleDevice(holder, id),
    //     myMeasuredFrictionCoefficient(1.),
    //     myRawFriction(1.),
    //     myStdDeviation(stdDev),
    //     myOffset(offset) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, stdDev: f64, offset: f64) -> Self {
        todo!("port MSDevice_Friction::MSDevice_Friction")
    }

    /// C++: `void MSDevice_Friction::MSDevice_Friction(const MSDevice_Friction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Friction) -> Self {
        todo!("port MSDevice_Friction::MSDevice_Friction")
    }

    /// C++: `MSDevice_Friction & MSDevice_Friction::operator=(const MSDevice_Friction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Friction) -> &mut MSDevice_Friction {
        todo!("port MSDevice_Friction::operator=")
    }

}

