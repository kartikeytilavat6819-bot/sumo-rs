//! Port of microsim/output/MSAmitranTrajectories.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping the complete network state
/// C++ class `MSAmitranTrajectories` (microsim/output/MSAmitranTrajectories.h)
pub struct MSAmitranTrajectories {
}

// C++ static members of MSAmitranTrajectories (port as module-level statics, see AGENTS.md):
//   static int MSAmitranTrajectories::myWrittenTypes;
//   static int MSAmitranTrajectories::myWrittenVehicles;

impl MSAmitranTrajectories {
    /// C++: `void MSAmitranTrajectories::write(OutputDevice &, const SUMOTime)` [static]
    /// Writes the complete network state into the given device
    // void
    // MSAmitranTrajectories::write(OutputDevice& of, const SUMOTime timestep) {
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt v = vc.loadedVehBegin(); v != vc.loadedVehEnd(); ++v) {
    //         writeVehicle(of, *v->second, timestep);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime) {
        todo!("port MSAmitranTrajectories::write")
    }

    /// C++: `void MSAmitranTrajectories::writeVehicle(OutputDevice &, const SUMOVehicle &, const SUMOTime)` [static, private]
    /// Writes the dump of the given vehicle into the given device
    // void
    // MSAmitranTrajectories::writeVehicle(OutputDevice& of, const SUMOVehicle& veh, const SUMOTime timestep) {
    //     if (veh.isOnRoad()) {
    //         const std::string& type = veh.getVehicleType().getID();
    //         if (myWrittenTypes.count(type) == 0) {
    //             of.openTag(SUMO_TAG_ACTORCONFIG).writeAttr(SUMO_ATTR_ID, veh.getVehicleType().getNumericalID());
    //             const SUMOEmissionClass c = veh.getVehicleType().getEmissionClass();
    //             if (c != 0) {
    //                 of.writeAttr(SUMO_ATTR_VEHICLECLASS, PollutantsInterface::getAmitranVehicleClass(c));
    //                 of.writeAttr("fuel", PollutantsInterface::getFuel(c));
    //                 of.writeAttr(SUMO_ATTR_EMISSIONCLASS, "Euro" + toString(PollutantsInterface::getEuroClass(c)));
    //                 const double weight = PollutantsInterface::getWeight(c);
    //                 if (weight > 0.) {
    //                     of.writeAttr(SUMO_ATTR_WEIGHT, int(weight / 10. + 0.5));
    //                 }
    //             }
    //             of.writeAttr(SUMO_ATTR_REF, type).closeTag();
    //             myWrittenTypes.insert(type);
    //         }
    //         if (myWrittenVehicles.count(veh.getID()) == 0) {
    //             const int index = (int)myWrittenVehicles.size();
    //             of.openTag(SUMO_TAG_VEHICLE).writeAttr(SUMO_ATTR_ID, index)
    //             .writeAttr(SUMO_ATTR_ACTORCONFIG, veh.getVehicleType().getNumericalID())
    //             .writeAttr(SUMO_ATTR_STARTTIME, STEPS2MS(veh.getDeparture()));
    //             of.writeAttr(SUMO_ATTR_REF, veh.getID()).closeTag();
    //             myWrittenVehicles[veh.getID()] = index;
    //         }
    //         of.openTag(SUMO_TAG_MOTIONSTATE).writeAttr(SUMO_ATTR_VEHICLE, myWrittenVehicles[veh.getID()])
    //         .writeAttr(SUMO_ATTR_SPEED, int(100.*veh.getSpeed() + 0.5))
    //         .writeAttr(SUMO_ATTR_TIME, STEPS2MS(timestep))
    //         .writeAttr(SUMO_ATTR_ACCELERATION, int(1000.*veh.getAcceleration() + 0.5));
    //         of.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_vehicle(of: &mut OutputDevice, veh: &SUMOVehicle, timestep: SUMOTime) {
        todo!("port MSAmitranTrajectories::writeVehicle")
    }

    /// C++: `void MSAmitranTrajectories::MSAmitranTrajectories(const MSAmitranTrajectories &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSAmitranTrajectories) -> Self {
        todo!("port MSAmitranTrajectories::MSAmitranTrajectories")
    }

    /// C++: `MSAmitranTrajectories & MSAmitranTrajectories::operator=(const MSAmitranTrajectories &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSAmitranTrajectories) -> &mut MSAmitranTrajectories {
        todo!("port MSAmitranTrajectories::operator=")
    }

}

