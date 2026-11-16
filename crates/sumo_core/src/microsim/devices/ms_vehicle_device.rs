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
        a.myHolder.get_numerical_id() < b.myHolder.get_numerical_id()
    }

}

/// Abstract in-vehicle device
/// C++ class `MSVehicleDevice` (microsim/devices/MSVehicleDevice.h)
#[repr(C)]
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
    /// C++ virtual `MSDevice::generateOutput(OutputDevice *) const`, reached through a vehicle device
    // shortcut: no vtable for generateOutput; only the emissions device overrides it here (dispatch by deviceName), upgrade when the other devices' tripinfo outputs are ported
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        if self.base2.device_name() == "emissions" {
            let dev: &MSDevice_Emissions = unsafe { &*(self as *const MSVehicleDevice as *const MSDevice_Emissions) };
            dev.generate_output(tripinfoOut);
        } else {
            self.base2.generate_output(tripinfoOut);
        }
    }

    /// C++: `void MSVehicleDevice::MSVehicleDevice(SUMOVehicle &, const std::string &)`
    /// Constructor
    // MSVehicleDevice(SUMOVehicle& holder, const std::string& id) :
    //         MSMoveReminder(id), MSDevice(id), myHolder(holder) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        Self {
            base: MSMoveReminder::new(id, Ptr::default(), false),
            base2: MSDevice::new(id),
            myHolder: Ptr::from_mut(holder),
        }
    }

    /// C++: `void MSVehicleDevice::~MSVehicleDevice() noexcept` [virtual]
    /// Destructor
    // virtual ~MSVehicleDevice() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `SUMOVehicle & MSVehicleDevice::getHolder() const` [const]
    /// Returns the vehicle that holds this device
    // inline SUMOVehicle& getHolder() const {
    //         return myHolder;
    //     }
    #[allow(unused_variables)]
    pub fn get_holder(&self) -> &mut SUMOVehicle {
        self.myHolder.m()
    }

    /// C++: `SUMOVehicle::NumericalID MSVehicleDevice::getNumericalID() const` [const]
    // inline SUMOVehicle::NumericalID getNumericalID() const {
    //         return myHolder.getNumericalID();
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> SUMOTrafficObject_NumericalID {
        self.myHolder.get_numerical_id()
    }

    /// C++: `void MSVehicleDevice::MSVehicleDevice(const MSVehicleDevice &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSVehicleDevice) -> Self {
        panic!("MSVehicleDevice copy constructor is private and undefined in C++")
    }

    /// C++: `MSVehicleDevice & MSVehicleDevice::operator=(const MSVehicleDevice &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVehicleDevice) -> &mut MSVehicleDevice {
        panic!("MSVehicleDevice assignment operator is private and undefined in C++")
    }

}

