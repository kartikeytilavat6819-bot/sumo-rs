//! Port of microsim/devices/MSVehicleDevice.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Function-object for stable sorting of objects with numerical ids
/// C++ class `MSVehicleDevice::ComparatorNumericalVehicleIdLess` (microsim/devices/MSVehicleDevice.h)
pub struct MSVehicleDevice_ComparatorNumericalVehicleIdLess {
}

impl MSVehicleDevice_ComparatorNumericalVehicleIdLess {
    /// C++: `bool ComparatorNumericalVehicleIdLess::operator()(const MSVehicleDevice *const, const MSVehicleDevice *const) const` [const]
    // bool operator()(const MSVehicleDevice* const a, const MSVehicleDevice* const b) const {
    //             return a->myHolder.getNumericalID() < b->myHolder.getNumericalID();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, a: Ptr<MSVehicleDevice>, b: Ptr<MSVehicleDevice>) -> bool {
        todo!("port ComparatorNumericalVehicleIdLess::operator()")
    }

}

/// Abstract in-vehicle device
/// C++ class `MSVehicleDevice` (microsim/devices/MSVehicleDevice.h)
pub struct MSVehicleDevice {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// C++ base class `MSDevice`
    pub base2: MSDevice,
    /// The vehicle that stores the device
    pub myHolder: Ptr<SUMOVehicle>,
}

impl std::ops::Deref for MSVehicleDevice {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSVehicleDevice {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSVehicleDevice {
    /// C++: `void MSVehicleDevice::MSVehicleDevice(SUMOVehicle &, const std::string &)`
    /// Constructor
    // MSVehicleDevice(SUMOVehicle& holder, const std::string& id) :
    //         MSMoveReminder(id), MSDevice(id), myHolder(holder) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        todo!("port MSVehicleDevice::MSVehicleDevice")
    }

    /// C++: `void MSVehicleDevice::~MSVehicleDevice() noexcept` [virtual]
    /// Destructor
    // virtual ~MSVehicleDevice() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSVehicleDevice::~MSVehicleDevice")
    }

    /// C++: `SUMOVehicle & MSVehicleDevice::getHolder() const` [const]
    /// Returns the vehicle that holds this device
    // inline SUMOVehicle& getHolder() const {
    //         return myHolder;
    //     }
    #[allow(unused_variables)]
    pub fn get_holder(&self) -> &mut SUMOVehicle {
        todo!("port MSVehicleDevice::getHolder")
    }

    /// C++: `SUMOVehicle::NumericalID MSVehicleDevice::getNumericalID() const` [const]
    // inline SUMOVehicle::NumericalID getNumericalID() const {
    //         return myHolder.getNumericalID();
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> SUMOTrafficObject_NumericalID {
        todo!("port MSVehicleDevice::getNumericalID")
    }

    /// C++: `void MSVehicleDevice::MSVehicleDevice(const MSVehicleDevice &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSVehicleDevice) -> Self {
        todo!("port MSVehicleDevice::MSVehicleDevice")
    }

    /// C++: `MSVehicleDevice & MSVehicleDevice::operator=(const MSVehicleDevice &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVehicleDevice) -> &mut MSVehicleDevice {
        todo!("port MSVehicleDevice::operator=")
    }

}

