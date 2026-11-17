//! Port of microsim/devices/MSTransportableDevice_FCD.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::options::options_cont::with_options;

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
        MSDevice::insert_default_assignment_options("fcd", "FCD Device", oc, true);

        oc.do_register("person-device.fcd.period", SumoOption::string_with("0"));
        oc.add_description("person-device.fcd.period", "FCD Device", "Recording period for FCD-data");
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
        let outputOptionSet = utils::options::options_cont::is_set("fcd-output");
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "fcd", t.get_id(), t.get_parameter(), t.get_vehicle_type().get_parameter(), outputOptionSet, true)
        });
        if equipped {
            let id = format!("fcd_{}", t.get_id());
            // shortcut: device is leaked; the person's device list will own it once person device ownership is ported, upgrade then
            let device: &mut MSTransportableDevice_FCD = Box::leak(Box::new(MSTransportableDevice_FCD::new(t, &id)));
            into.push(Ptr::from_mut(&mut device.base));
        }
    }

    /// C++: `void MSTransportableDevice_FCD::~MSTransportableDevice_FCD() noexcept` [virtual]
    /// Destructor.
    // MSTransportableDevice_FCD::~MSTransportableDevice_FCD() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool MSTransportableDevice_FCD::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& /*veh*/, MSMoveReminder::Notification /*reason*/, const MSLane* /*enteredLane*/) {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, arg0: &mut SUMOTrafficObject, arg1: MSMoveReminder_Notification, arg2: Ptr<MSLane>) -> bool {
        false
    }

    /// C++: `void MSTransportableDevice_FCD::saveState(OutputDevice &) const` [const, virtual]
    // void saveState(OutputDevice& /* out */) const {
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&self, arg0: &mut OutputDevice) {
        // C++ saveState body is empty
    }

    /// C++: `const std::string MSTransportableDevice_FCD::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "fcd";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "fcd".to_string()
    }

    /// C++: `void MSTransportableDevice_FCD::cleanup()` [static]
    /// resets the edge filter
    // void
    // MSTransportableDevice_FCD::cleanup() {
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        // C++ cleanup body is empty
    }

    /// C++: `void MSTransportableDevice_FCD::MSTransportableDevice_FCD(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_FCD::MSTransportableDevice_FCD(MSTransportable& holder, const std::string& id) :
    //     MSTransportableDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            false
        }
        static MS_MOVE_REMINDER_VT: crate::microsim::ms_move_reminder::MSMoveReminderVt = crate::microsim::ms_move_reminder::MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: crate::microsim::ms_move_reminder::default_notify_move,
            notify_leave: crate::microsim::ms_move_reminder::default_notify_leave,
        };
        let mut this = MSTransportableDevice_FCD {
            base: MSTransportableDevice::new(holder, id),
        };
        this.base.base2.vt = &MS_MOVE_REMINDER_VT;
        this
    }

    /// C++: `void MSTransportableDevice_FCD::MSTransportableDevice_FCD(const MSTransportableDevice_FCD &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportableDevice_FCD) -> Self {
        panic!("MSTransportableDevice_FCD copy constructor is invalidated in SUMO")
    }

    /// C++: `MSTransportableDevice_FCD & MSTransportableDevice_FCD::operator=(const MSTransportableDevice_FCD &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_FCD) -> &mut MSTransportableDevice_FCD {
        panic!("MSTransportableDevice_FCD assignment operator is invalidated in SUMO")
    }

}

