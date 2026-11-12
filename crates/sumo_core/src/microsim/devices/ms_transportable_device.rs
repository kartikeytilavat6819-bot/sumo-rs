//! Port of microsim/devices/MSTransportableDevice.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Abstract in-person device
/// C++ class `MSTransportableDevice` (microsim/devices/MSTransportableDevice.h)
#[repr(C)]
pub struct MSTransportableDevice {
    /// C++ base class `MSDevice`
    pub base: MSDevice,
    /// C++ base class `MSMoveReminder`
    pub base2: MSMoveReminder,
    /// The person that stores the device
    pub myHolder: Ptr<MSTransportable>,
}

impl std::ops::Deref for MSTransportableDevice {
    type Target = MSDevice;
    fn deref(&self) -> &MSDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportableDevice {
    fn deref_mut(&mut self) -> &mut MSDevice {
        &mut self.base
    }
}

impl MSTransportableDevice {
    /// C++: `void MSTransportableDevice::MSTransportableDevice(MSTransportable &, const std::string &)`
    /// Constructor
    // MSTransportableDevice(MSTransportable& holder, const std::string& id) :
    //         MSDevice(id), MSMoveReminder(id), myHolder(holder) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        todo!("port MSTransportableDevice::MSTransportableDevice")
    }

    /// C++: `void MSTransportableDevice::~MSTransportableDevice() noexcept` [virtual]
    /// Destructor
    // virtual ~MSTransportableDevice() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTransportableDevice::~MSTransportableDevice")
    }

    /// C++: `MSTransportable & MSTransportableDevice::getHolder() const` [const]
    /// Returns the person that holds this device
    // MSTransportable& getHolder() const {
    //         return myHolder;
    //     }
    #[allow(unused_variables)]
    pub fn get_holder(&self) -> &mut MSTransportable {
        todo!("port MSTransportableDevice::getHolder")
    }

    /// C++: `void MSTransportableDevice::MSTransportableDevice(const MSTransportableDevice &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportableDevice) -> Self {
        todo!("port MSTransportableDevice::MSTransportableDevice")
    }

    /// C++: `MSTransportableDevice & MSTransportableDevice::operator=(const MSTransportableDevice &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice) -> &mut MSTransportableDevice {
        todo!("port MSTransportableDevice::operator=")
    }

}

