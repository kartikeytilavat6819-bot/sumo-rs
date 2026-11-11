//! Port of microsim/MSDynamicShapeUpdater.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Ensures that dynamic shapes tracking traffic objects are removed when the objects are removed.
/// C++ class `MSDynamicShapeUpdater` (microsim/MSDynamicShapeUpdater.h)
pub struct MSDynamicShapeUpdater {
    /// C++ base class `MSNet::VehicleStateListener`
    pub base: MSNet_VehicleStateListener,
    pub myShapeContainer: Ptr<ShapeContainer>,
}

impl std::ops::Deref for MSDynamicShapeUpdater {
    type Target = MSNet_VehicleStateListener;
    fn deref(&self) -> &MSNet_VehicleStateListener {
        &self.base
    }
}
impl std::ops::DerefMut for MSDynamicShapeUpdater {
    fn deref_mut(&mut self) -> &mut MSNet_VehicleStateListener {
        &mut self.base
    }
}

impl MSDynamicShapeUpdater {
    /// C++: `void MSDynamicShapeUpdater::MSDynamicShapeUpdater(ShapeContainer &)`
    /// Constructor
    // MSDynamicShapeUpdater(ShapeContainer& shapeContainer) : myShapeContainer(shapeContainer) {}
    #[allow(unused_variables)]
    pub fn new(shapeContainer: &mut ShapeContainer) -> Self {
        MSDynamicShapeUpdater {
            base: MSNet_VehicleStateListener {},
            myShapeContainer: Ptr::from_mut(shapeContainer),
        }
    }

    /// C++: `void MSDynamicShapeUpdater::vehicleStateChanged(const SUMOVehicle *const, MSNet::VehicleState, const std::string &)` [virtual]
    /// Constructor
    // void
    // MSDynamicShapeUpdater::vehicleStateChanged(const SUMOVehicle* const vehicle, MSNet::VehicleState to, const std::string& /* info */) {
    //
    //     /// @brief Definition of a vehicle state
    //     switch (to) {
    //         /// @brief The vehicle arrived at his destination (is deleted)
    //         case MSNet::VehicleState::ARRIVED: {
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //             std::cout << "MSDynamicShapeUpdater: Vehicle '" << vehicle->getID() << "' has arrived. Informing ShapeContainer." << std::endl;
    // #endif
    //             myShapeContainer.removeTrackers(vehicle->getID());
    //         }
    //         break;
    //         /// @brief The vehicle is involved in a collision
    //         case MSNet::VehicleState::COLLISION:
    //             // TODO: can this lead to removal without calling arrived?
    //             break;
    //         default:
    //         {}
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_state_changed(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        match to {
            MSNet_VehicleState::ARRIVED => {
                // #ifdef DEBUG_DYNAMIC_SHAPES output omitted
                self.myShapeContainer.m().remove_trackers(vehicle.get_id().to_string());
            }
            // TODO: can this lead to removal without calling arrived?
            MSNet_VehicleState::COLLISION => {}
            _ => {}
        }
    }

    /// C++: `MSDynamicShapeUpdater & MSDynamicShapeUpdater::operator=(const MSDynamicShapeUpdater &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSDynamicShapeUpdater) -> &mut MSDynamicShapeUpdater {
        panic!("MSDynamicShapeUpdater assignment operator is invalidated")
    }

}

