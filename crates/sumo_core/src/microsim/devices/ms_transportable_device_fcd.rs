//! Port of microsim/devices/MSTransportableDevice_FCD.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSTransportableDevice_FCD` (microsim/devices/MSTransportableDevice_FCD.h)
#[repr(C)]
pub struct MSTransportableDevice_FCD {
    /// C++ base class `MSTransportableDevice`
    pub base: MSTransportableDevice,
}

impl std::ops::Deref for MSTransportableDevice_FCD {
    type Target = MSTransportableDevice;
    fn deref(&self) -> &MSTransportableDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportableDevice_FCD {
    fn deref_mut(&mut self) -> &mut MSTransportableDevice {
        &mut self.base
    }
}

impl MSTransportableDevice_FCD {
    /// C++: `void MSTransportableDevice_FCD::insertOptions(OptionsCont &)` [static]
    /// Inserts MSTransportableDevice_FCD-options
    // void
    // MSTransportableDevice_FCD::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("fcd", "FCD Device", oc, true);
    //
    //     oc.doRegister("person-device.fcd.period", new Option_String("0"));
    //     oc.addDescription("person-device.fcd.period", "FCD Device", TL("Recording period for FCD-data"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSTransportableDevice_FCD::insertOptions")
    }

    /// C++: `void MSTransportableDevice_FCD::buildDevices(MSTransportable &, std::vector<MSTransportableDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSTransportableDevice_FCD::buildDevices(MSTransportable& t, std::vector<MSTransportableDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "fcd", t, oc.isSet("fcd-output"), true)) {
    //         MSTransportableDevice_FCD* device = new MSTransportableDevice_FCD(t, "fcd_" + t.getID());
    //         into.push_back(device);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_devices(t: &mut MSTransportable, into: &mut Vec<Ptr<MSTransportableDevice>>) {
        todo!("port MSTransportableDevice_FCD::buildDevices")
    }

    /// C++: `void MSTransportableDevice_FCD::~MSTransportableDevice_FCD() noexcept` [virtual]
    /// Destructor.
    // MSTransportableDevice_FCD::~MSTransportableDevice_FCD() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTransportableDevice_FCD::~MSTransportableDevice_FCD")
    }

    /// C++: `bool MSTransportableDevice_FCD::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& /*veh*/, MSMoveReminder::Notification /*reason*/, const MSLane* /*enteredLane*/) {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, arg0: &mut SUMOTrafficObject, arg1: MSMoveReminder_Notification, arg2: Ptr<MSLane>) -> bool {
        todo!("port MSTransportableDevice_FCD::notifyEnter")
    }

    /// C++: `void MSTransportableDevice_FCD::saveState(OutputDevice &) const` [const, virtual]
    // void saveState(OutputDevice& /* out */) const {
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&self, arg0: &mut OutputDevice) {
        todo!("port MSTransportableDevice_FCD::saveState")
    }

    /// C++: `const std::string MSTransportableDevice_FCD::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "fcd";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSTransportableDevice_FCD::deviceName")
    }

    /// C++: `void MSTransportableDevice_FCD::cleanup()` [static]
    /// resets the edge filter
    // void
    // MSTransportableDevice_FCD::cleanup() {
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSTransportableDevice_FCD::cleanup")
    }

    /// C++: `void MSTransportableDevice_FCD::MSTransportableDevice_FCD(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_FCD::MSTransportableDevice_FCD(MSTransportable& holder, const std::string& id) :
    //     MSTransportableDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        todo!("port MSTransportableDevice_FCD::MSTransportableDevice_FCD")
    }

    /// C++: `void MSTransportableDevice_FCD::MSTransportableDevice_FCD(const MSTransportableDevice_FCD &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportableDevice_FCD) -> Self {
        todo!("port MSTransportableDevice_FCD::MSTransportableDevice_FCD")
    }

    /// C++: `MSTransportableDevice_FCD & MSTransportableDevice_FCD::operator=(const MSTransportableDevice_FCD &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_FCD) -> &mut MSTransportableDevice_FCD {
        todo!("port MSTransportableDevice_FCD::operator=")
    }

}

